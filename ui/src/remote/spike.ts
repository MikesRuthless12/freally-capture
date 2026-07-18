/**
 * Remote Guests — the P2P session (multi-guest transport + invites + two-gate mute).
 *
 * PeerJS (vendored, no CDN) brokers a WebRTC session; media flows peer-to-peer.
 * A HOST admits up to {@link MAX_GUESTS} guests: each opens a control channel +
 * a media call, is bridged into the compositor (or held in the GREEN ROOM,
 * CAP-N54), and appears as its own tile with its own mute gates, QoS
 * (CAP-N56), and cue channel (CAP-N55). The GUEST shares its webcam + mic and
 * calls one host.
 *
 * Session state lives in a small **store** here — not in any one component — so
 * the status + per-guest controls persist on the main UI even when the setup
 * dialog is closed. Components subscribe via `spikeSubscribe`.
 */
import Peer, { type DataConnection, type MediaConnection } from "peerjs";
import { invoke } from "@tauri-apps/api/core";

import {
  settingsGet,
  studioAddItem,
  studioAutoGrid,
  studioRemoveItem,
  studioSetCenterView,
} from "../api/commands";
import type { ItemId, SceneId } from "../api/types";
import { micConstraints, routeToOutput } from "./devices";
import { inviteLink as buildInviteLink, joinTargetFromInput, mintInvite } from "./invite";
import { GATES_CLEAR, type GateState, guestToggleSelf } from "./mute";
import { type GuestQos, type QosPrev, sampleQos } from "./qos";

/** Downscale bound for the spike's IPC frames (keeps a push ≤ ~0.9 MB). */
const MAX_FRAME_WIDTH = 640;
/** The mixer's ring rate. WebRTC audio is 48 kHz; we tap it at that rate. */
const AUDIO_RATE = 48_000;
/** How many guests one host admits — host + 8 = up to 9 tiles (CAP-N59's 1–9).
 * Validated to the 4-guest DoD budget; higher counts depend on the host's
 * bandwidth/CPU (honest platform limit). */
export const MAX_GUESTS = 8;
/** Per-guest QoS poll interval (CAP-N56). */
const QOS_INTERVAL_MS = 1000;

/** rVFC is not in every lib.dom yet — the narrow slice the pump needs. */
type VideoWithRvfc = HTMLVideoElement & {
  requestVideoFrameCallback: (callback: () => void) => number;
};

/** A control message on the host↔guest data channel: the two mute gates, the
 * host's view-switching grant, a guest's center-view request, a private cue
 * (CAP-N55), the green-room hold flag (CAP-N54), and the deliberate-goodbye
 * marker (so a kicked/ended guest doesn't auto-rejoin). */
type ControlMsg =
  | { t: "host"; muted: boolean }
  | { t: "self"; muted: boolean }
  | { t: "allowCenter"; allowed: boolean }
  | { t: "centerReq"; view: "guestCam" | "guestScreen" | "hostView" }
  | { t: "cue"; text: string; seconds: number | null }
  | { t: "greenRoom"; staged: boolean }
  | { t: "bye"; reason: "removed" | "banned" | "ended" };

/** A bridged guest item: the scene item + the source its frames feed. */
export type GuestItemRef = { sceneId: SceneId; itemId: ItemId; sourceId: string };

/** A received cue on the guest side (CAP-N55): the text + optional countdown. */
export type GuestCue = { text: string; seconds: number | null; at: number };

/** The host's view of one connected guest (what the UI renders per tile). */
export type GuestView = {
  peerId: string;
  label: string;
  /** The two mute gates for this guest. */
  gates: GateState;
  /** Seated on program (bridged), or null while held in the green room. */
  itemId: ItemId | null;
  sceneId: SceneId | null;
  /** CAP-N54: held off-program in the green room (tech-check only). */
  greenRoom: boolean;
  /** Whether this guest may switch the center view. */
  allowCenter: boolean;
  /** CAP-N56: live connection quality, or null before the first sample. */
  qos: GuestQos | null;
  status: string;
  /** The guest's live cam stream — the green-room monitor renders it. */
  stream: MediaStream | null;
};

/** The observable session state (what the UI renders). */
export type SpikeState = {
  active: boolean;
  role: "host" | "guest" | null;
  status: string;
  /** The host's session id — the invite link is minted from it. */
  peerId: string | null;
  /** The minted invite link (host only; re-mints when the TTL changes). */
  invite: string | null;
  /** Host: every connected guest, in join order. */
  guests: GuestView[];
  /** CAP-N54: new guests land in the green room first when this is on (host). */
  greenRoomDefault: boolean;
  /** CAP-N59: keep the guest grid auto-arranged as guests join/leave (host). */
  autoGridArmed: boolean;
  /** Guest: this guest's own mute gates (null before connected). */
  gates: GateState | null;
  /** Guest: whether the host granted center-view switching. */
  allowCenter: boolean;
  /** Guest: a cue the host sent (CAP-N55), or null. */
  cue: GuestCue | null;
  /** Guest: held in the host's green room, not yet on program (CAP-N54). */
  greenRoom: boolean;
  /** Whether THIS side is sharing its screen into the session. */
  sharingScreen: boolean;
  /** Guest: the host's shared screen (rendered by the session bar). */
  hostShare: MediaStream | null;
  /** A freally:// invite that arrived via the OS deep link — held until the
   * user explicitly clicks Join (nothing auto-connects). */
  joinPrefill: string | null;
  /** The chosen microphone (null = system default). Survives sessions. */
  micId: string | null;
  /** The chosen output for the other side's audio (null = default). */
  speakerId: string | null;
};

const GREEN_ROOM_KEY = "remote.greenRoomDefault";

const IDLE: SpikeState = {
  active: false,
  role: null,
  status: "idle — nothing touches the network yet",
  peerId: null,
  invite: null,
  guests: [],
  greenRoomDefault: loadBool(GREEN_ROOM_KEY),
  autoGridArmed: false,
  gates: null,
  allowCenter: false,
  cue: null,
  greenRoom: false,
  sharingScreen: false,
  hostShare: null,
  joinPrefill: null,
  micId: null,
  speakerId: null,
};

const MIC_KEY = "remote.micId";
const SPEAKER_KEY = "remote.speakerId";
/** TASK-R11: the host's peer id persists so a relaunched host reclaims it —
 * the invite links guests already hold keep working. */
const HOST_ID_KEY = "remote.hostPeerId";
/** A guest's peer id persists too, so a rejoin reuses it — the host reclaims
 * the same tile (no duplicate) and a ban's denylist stays effective. */
const GUEST_ID_KEY = "remote.guestPeerId";

function loadPref(key: string): string | null {
  try {
    return window.localStorage.getItem(key);
  } catch {
    return null;
  }
}

function loadBool(key: string): boolean {
  return loadPref(key) === "1";
}

function savePref(key: string, value: string | null): void {
  try {
    if (value) window.localStorage.setItem(key, value);
    else window.localStorage.removeItem(key);
  } catch {
    // Preference persistence is best-effort.
  }
}

let state: SpikeState = { ...IDLE, micId: loadPref(MIC_KEY), speakerId: loadPref(SPEAKER_KEY) };
const listeners = new Set<(s: SpikeState) => void>();

function setState(patch: Partial<SpikeState>): void {
  state = { ...state, ...patch };
  for (const listener of listeners) listener(state);
}

/** Register a session-state listener. `useSyncExternalStore`-shaped: it does
 * NOT fire immediately — read the current value via `spikeGetState`. */
export function spikeSubscribe(listener: (s: SpikeState) => void): () => void {
  listeners.add(listener);
  return () => {
    listeners.delete(listener);
  };
}

export function spikeGetState(): SpikeState {
  return state;
}

/** Per-guest connection resources (host side). */
type GuestConn = {
  peerId: string;
  label: string;
  data?: DataConnection;
  call?: MediaConnection;
  screenIn?: MediaConnection;
  /** The guest's live cam stream (for the green-room monitor + the bridge). */
  stream?: MediaStream;
  gates: GateState;
  guestItem: GuestItemRef | null;
  guestScreenItem: GuestItemRef | null;
  allowCenter: boolean;
  greenRoom: boolean;
  status: string;
  qos: GuestQos | null;
  /** The last QoS raw sample, for delta-based packet-loss (CAP-N56). */
  qosPrev?: QosPrev;
  qosTimer?: number;
  /** Per-guest teardown (bridge, tapped tracks, timers). */
  stops: Array<() => void>;
};

type LiveSession = {
  peer: Peer;
  stops: Array<() => void>;
  role: "host" | "guest";
  /** Host: the scene guests land in (needed to re-host after a ban). */
  sceneId?: SceneId;
  /** Host: the invite TTL the link is minted with (re-mints on change). */
  ttlMinutes?: number;
  /** Host: connected guests, keyed by peer id. */
  guests: Map<string, GuestConn>;
  /** Host: the shared talkback mic (answered into every guest call). */
  micTrack?: MediaStreamTrack;
  micStream?: MediaStream;
  // -- guest-role fields --
  data?: DataConnection;
  call?: MediaConnection;
  /** Guest: the host's peer id (screen shares call back to it). */
  hostPeer?: string;
  /** Guest: the host said goodbye deliberately — do NOT auto-rejoin. */
  byeReason?: "removed" | "banned" | "ended";
  /** Guest: an incoming shared-screen call (the host's view). */
  screenIn?: MediaConnection;
  /** This side's outgoing screen-share calls (host: one per guest; guest: one
   * to the host) + the shared stream. */
  screenOuts: MediaConnection[];
  screenStream?: MediaStream;
  /** Guest: the outbound mic track — self-mute / live device switch touch it. */
  guestMicTrack?: MediaStreamTrack;
  /** Guest: this guest's own mute gates. */
  gates?: GateState;
  /** Guest: the element the host's return audio plays through. */
  hostAudioEl?: HTMLAudioElement;
};
let live: LiveSession | null = null;

// TASK-R8: banned guest peer ids. Persisted so a ban survives a relaunch —
// honest limitation: a guest who mints a fresh peer id needs a valid invite
// link to rejoin (multi-guest keeps the shared link so the other guests stay).
const DENY_KEY = "remote.bannedPeers";

function loadDenylist(): Set<string> {
  try {
    const parsed: unknown = JSON.parse(window.localStorage.getItem(DENY_KEY) ?? "[]");
    return new Set(Array.isArray(parsed) ? parsed.filter((v) => typeof v === "string") : []);
  } catch {
    return new Set();
  }
}

const DENYLIST = loadDenylist();

function saveDenylist(): void {
  try {
    window.localStorage.setItem(DENY_KEY, JSON.stringify([...DENYLIST]));
  } catch {
    // Best-effort persistence.
  }
}

/** Publish the host's guest map into the observable state (one GuestView each). */
function publishGuests(): void {
  if (!live || live.role !== "host") return;
  const guests: GuestView[] = [...live.guests.values()].map((conn) => ({
    peerId: conn.peerId,
    label: conn.label,
    gates: conn.gates,
    itemId: conn.guestItem?.itemId ?? null,
    sceneId: conn.guestItem?.sceneId ?? null,
    greenRoom: conn.greenRoom,
    allowCenter: conn.allowCenter,
    qos: conn.qos,
    status: conn.status,
    stream: conn.stream ?? null,
  }));
  setState({ guests });
}

/** Tear down one guest's resources (call, channel, bridge, QoS) without
 * touching the store — the caller publishes. */
function teardownGuest(conn: GuestConn): void {
  if (conn.qosTimer) window.clearInterval(conn.qosTimer);
  conn.stops.forEach((stop) => stop());
  conn.stops = [];
  conn.call?.close();
  conn.data?.close();
  conn.screenIn?.close();
}

/** Remove one guest's bridged scene items (seated cam + shared screen). */
async function removeGuestItems(conn: GuestConn): Promise<void> {
  for (const item of [conn.guestItem, conn.guestScreenItem]) {
    if (item) await studioRemoveItem(item.sceneId, item.itemId).catch(() => undefined);
  }
  conn.guestItem = null;
  conn.guestScreenItem = null;
}

/** Stop the live session's resources without touching the store. */
function teardownLive(): void {
  if (live?.role === "host") {
    for (const conn of live.guests.values()) teardownGuest(conn);
    live.guests.clear();
  }
  live?.stops.forEach((stop) => stop());
  live?.data?.close();
  live?.peer.destroy();
  live = null;
}

// TASK-R11: a dropped guest auto-redials the same host with backoff while
// their invite token stays valid; a user Leave/Stop cancels the cycle.
const REJOIN_MAX_ATTEMPTS = 8;
let rejoin: { timer: number; attempt: number } | null = null;

function cancelRejoin(): void {
  if (rejoin) {
    window.clearTimeout(rejoin.timer);
    rejoin = null;
  }
}

function scheduleRejoin(hostPeerId: string): void {
  const attempt = (rejoin?.attempt ?? 0) + 1;
  if (attempt > REJOIN_MAX_ATTEMPTS) {
    rejoin = null;
    spikeStop("couldn't reconnect — rejoin with your invite link while it's still valid.");
    return;
  }
  const delaySec = Math.min(2 ** attempt, 30);
  setState({
    status: `connection lost — reconnecting in ${delaySec}s (attempt ${attempt}/${REJOIN_MAX_ATTEMPTS})…`,
  });
  const timer = window.setTimeout(() => {
    void spikeJoin(hostPeerId, { rejoining: true });
  }, delaySec * 1000);
  rejoin = { timer, attempt };
}

/** Tear down whichever session is running and reset the store. An optional
 * final status stays readable in the setup dialog (e.g. why it ended). */
export function spikeStop(finalStatus?: string): void {
  cancelRejoin();
  if (live?.role === "host") {
    // A courtesy goodbye so each guest's app ends cleanly instead of
    // auto-rejoining a dead session (best-effort — teardown proceeds).
    for (const conn of live.guests.values()) {
      try {
        conn.data?.send({ t: "bye", reason: "ended" } satisfies ControlMsg);
      } catch {
        // The channel may already be gone.
      }
      // A deliberate end removes the bridged items — nothing stays in the scene.
      void removeGuestItems(conn);
    }
  }
  teardownLive();
  // Device preferences + a pending deep-link invite survive the reset.
  state = {
    ...IDLE,
    greenRoomDefault: state.greenRoomDefault,
    status: finalStatus ?? IDLE.status,
    joinPrefill: state.joinPrefill,
    micId: state.micId,
    speakerId: state.speakerId,
  };
  for (const listener of listeners) listener(state);
}

/** TASK-R2: a freally:// link arrived via the OS deep link. It is held for
 * an explicit user Join — consent stays a click, nothing auto-connects. */
export function spikeSetJoinPrefill(link: string | null): void {
  setState({ joinPrefill: link });
}

/** Join from a held deep-link invite (the user clicked Join). */
export function spikeJoinFromLink(raw: string): void {
  const target = joinTargetFromInput(raw, Date.now());
  if ("error" in target) {
    setState({ joinPrefill: null, status: target.error });
    return;
  }
  setState({ joinPrefill: null });
  void spikeJoin(target.peerId).catch((err) => setState({ status: `join failed: ${err}` }));
}

/** CAP-N54: toggle whether new guests land in the green room first (host). */
export function spikeSetGreenRoomDefault(on: boolean): void {
  savePref(GREEN_ROOM_KEY, on ? "1" : null);
  setState({ greenRoomDefault: on });
}

/**
 * A Peer configured from settings: with the user's own opt-in TURN relay
 * (TASK-R5) appended to the default STUN when one is configured; otherwise
 * the PeerJS defaults (direct P2P, STUN-only — the free path). A host passes
 * its persisted `requestId` to reclaim its session id across relaunches.
 */
async function buildPeer(requestId?: string | null): Promise<Peer> {
  let options: ConstructorParameters<typeof Peer>[1] | undefined;
  try {
    const settings = await settingsGet();
    const { turnUrl, turnUsername, turnCredential } = settings.remote;
    // A turn:/turns: server REQUIRES credentials at the RTCPeerConnection
    // level — an incomplete config would throw on every session, so the
    // relay only engages once all three fields are set.
    if (turnUrl && turnUsername && turnCredential) {
      options = {
        config: {
          iceServers: [
            { urls: "stun:stun.l.google.com:19302" },
            { urls: turnUrl, username: turnUsername, credential: turnCredential },
          ],
        },
      };
    }
  } catch {
    // Browser mode / settings unavailable — the defaults still work.
  }
  if (requestId) return options ? new Peer(requestId, options) : new Peer(requestId);
  return options ? new Peer(options) : new Peer();
}

/** Honest connectivity status on a media call (STUN-first, TURN optional). */
function watchIce(call: MediaConnection): void {
  call.on("iceStateChanged", (ice) => {
    if (ice === "failed") {
      setState({
        status:
          "direct P2P failed — both sides are behind strict NATs. An opt-in TURN relay (Remote dialog → network) would carry this.",
      });
    } else if (ice === "disconnected") {
      setState({ status: "connection unstable — trying to recover…" });
    }
  });
}

// ===========================================================================
// HOST
// ===========================================================================

/** Host: wait for guests; when media arrives, bridge each in (or hold it in
 * the green room). The session id persists across relaunches (TASK-R11): a
 * restarted host reclaims it so already-shared invite links keep working; a
 * ban denylists the guest's id (the shared link stays live for the others). */
export async function spikeHost(
  sceneId: SceneId,
  ttlMinutes: number,
  opts: { freshId?: boolean; idAttempt?: number } = {},
): Promise<void> {
  spikeStop();
  setState({
    active: true,
    role: "host",
    status: "connecting to the signaling broker…",
    peerId: null,
    invite: null,
    guests: [],
  });
  if (opts.freshId) savePref(HOST_ID_KEY, null);
  const requestedId = loadPref(HOST_ID_KEY);
  const peer = await buildPeer(requestedId);
  const session: LiveSession = {
    peer,
    stops: [],
    role: "host",
    guests: new Map(),
    screenOuts: [],
    sceneId,
    ttlMinutes,
  };
  live = session;
  peer.on("open", (id) => {
    savePref(HOST_ID_KEY, id);
    setState({
      peerId: id,
      invite: buildInviteLink(mintInvite(id, session.ttlMinutes ?? ttlMinutes, Date.now())),
      status: "hosting — share the invite link; waiting for guests…",
    });
  });
  peer.on("error", (err) => {
    if (err.type === "unavailable-id" && requestedId) {
      const attempt = opts.idAttempt ?? 0;
      if (attempt === 0) {
        setState({ status: "your previous session id is still held — retrying it…" });
        window.setTimeout(() => {
          if (live === session) void spikeHost(sceneId, ttlMinutes, { idAttempt: 1 });
        }, 3000);
      } else {
        setState({ status: "couldn't reclaim the old session id — starting a fresh link." });
        window.setTimeout(() => {
          if (live === session)
            void spikeHost(sceneId, ttlMinutes, { freshId: true, idAttempt: 2 });
        }, 0);
      }
      return;
    }
    setState({ status: `peer error: ${err.type}` });
  });
  peer.on("disconnected", () => {
    setState({ status: "signaling broker lost — reconnecting (media keeps flowing)…" });
    peer.reconnect();
  });

  // A guest opens a control channel (mute state) alongside the media call.
  peer.on("connection", (conn) => {
    if (DENYLIST.has(conn.peer)) {
      conn.close();
      setState({ status: "a banned guest tried to rejoin — refused." });
      return;
    }
    let guest = session.guests.get(conn.peer);
    if (!guest && session.guests.size >= MAX_GUESTS) {
      conn.close();
      setState({ status: `the room is full (${MAX_GUESTS} guests) — refused another join.` });
      return;
    }
    if (!guest) {
      guest = newGuest(session, conn.peer);
      session.guests.set(conn.peer, guest);
    }
    guest.data = conn;
    guest.status = "guest connected.";
    const sendHostGate = () =>
      conn.send({ t: "host", muted: guest.gates.hostGate } satisfies ControlMsg);
    if (conn.open) sendHostGate();
    else conn.on("open", sendHostGate);
    // Tell the guest it landed in the green room (its state was fixed in newGuest).
    if (guest.greenRoom) {
      const sendGreen = () => conn.send({ t: "greenRoom", staged: true } satisfies ControlMsg);
      if (conn.open) sendGreen();
      else conn.on("open", sendGreen);
    }
    conn.on("data", (raw) => {
      const msg = raw as ControlMsg;
      if (msg?.t === "self") {
        guest.gates = { ...guest.gates, selfGate: msg.muted };
        publishGuests();
      } else if (msg?.t === "centerReq") {
        applyGuestCenterRequest(guest, msg.view);
      }
    });
    conn.on("close", () => {
      // The control channel is the liveness signal. Only drop the guest if THIS
      // connection is still its live one — a rejoin (same persisted id) replaces
      // guest.data, and this stale close must not evict the reclaimed guest.
      if (guest.data === conn) void dropGuest(session, conn.peer, "guest disconnected");
    });
    publishGuests();
    setState({ status: "guest connected." });
  });

  peer.on("call", (call) => {
    if (DENYLIST.has(call.peer)) {
      call.close();
      setState({ status: "a banned guest tried to call — refused." });
      return;
    }
    let guest = session.guests.get(call.peer);
    if (!guest && session.guests.size >= MAX_GUESTS) {
      call.close();
      return;
    }
    if (!guest) {
      guest = newGuest(session, call.peer);
      session.guests.set(call.peer, guest);
    }
    const kind = (call.metadata as { kind?: string } | null)?.kind ?? "cam";
    if (kind === "screen") {
      guest.screenIn = call;
      call.answer();
      watchIce(call);
      let stopScreen: (() => void) | undefined;
      call.on("stream", (stream) => {
        addAndPump(stream, sceneId, guest, {
          label: `${guest.label} Screen`,
          assign: "guestScreenItem",
        })
          .then((stop) => {
            stopScreen = stop;
          })
          .catch((err) => setState({ status: `screen bridge failed: ${err}` }));
      });
      call.on("close", () => {
        stopScreen?.();
        if (guest.guestScreenItem) {
          void studioRemoveItem(guest.guestScreenItem.sceneId, guest.guestScreenItem.itemId).catch(
            () => undefined,
          );
          guest.guestScreenItem = null;
          publishGuests();
        }
      });
      return;
    }
    guest.call = call;
    guest.status = "guest calling — answering…";
    watchIce(call);
    void answerWithTalkback(call, session);
    startQos(guest);
    call.on("stream", (stream) => {
      guest.stream = stream;
      if (guest.greenRoom) {
        // CAP-N54: hold the stream in the green room; the host seats them.
        guest.status = "in the green room — tech-check, then seat on air.";
        publishGuests();
      } else {
        seatGuestStream(session, guest, stream);
      }
    });
    call.on("error", (err) => setState({ status: `guest call error: ${err.type}` }));
    publishGuests();
  });
}

/** A fresh, unbridged guest record. Green-room state is fixed HERE so it holds
 * whether the media `call` or the data `connection` event creates the guest
 * first (CAP-N54 — otherwise a call-first guest would seat straight to air). */
function newGuest(session: LiveSession, peerId: string): GuestConn {
  const label = `Guest ${session.guests.size + 1}`;
  return {
    peerId,
    label,
    gates: GATES_CLEAR,
    guestItem: null,
    guestScreenItem: null,
    allowCenter: false,
    greenRoom: state.greenRoomDefault,
    status: "connecting…",
    qos: null,
    stops: [],
  };
}

/** Bridge a guest's cam stream into the compositor (seat them on program). */
function seatGuestStream(session: LiveSession, guest: GuestConn, stream: MediaStream): void {
  const sceneId = session.sceneId;
  if (!sceneId) return;
  guest.greenRoom = false;
  addAndPump(stream, sceneId, guest, { label: guest.label, assign: "guestItem" })
    .then(() => {
      guest.status = `${guest.label} is live in the scene ✔`;
      publishGuests();
      if (state.autoGridArmed) void spikeAutoGrid();
    })
    .catch((err) => setState({ status: `bridge failed: ${err}` }));
}

/** CAP-N54: seat a green-roomed guest on air (one-click). */
export function spikeSeatGuest(peerId: string): void {
  const session = live;
  if (!session || session.role !== "host") return;
  const guest = session.guests.get(peerId);
  if (!guest || !guest.greenRoom || !guest.stream) return;
  guest.data?.send({ t: "greenRoom", staged: false } satisfies ControlMsg);
  seatGuestStream(session, guest, guest.stream);
}

/** Remove one guest from the room (drop / moderation), freeing its tile. */
async function dropGuest(session: LiveSession, peerId: string, why: string): Promise<void> {
  const guest = session.guests.get(peerId);
  if (!guest) return;
  session.guests.delete(peerId);
  teardownGuest(guest);
  await removeGuestItems(guest);
  publishGuests();
  setState({ status: `${guest.label}: ${why}.` });
  if (state.autoGridArmed) void spikeAutoGrid();
}

/** Host: answer a guest's call WITH this machine's mic (return audio, so the
 * guest hears the host). One shared mic is answered into every guest call;
 * falls back to a receive-only answer when the mic is unavailable. */
async function answerWithTalkback(call: MediaConnection, session: LiveSession): Promise<void> {
  try {
    if (!session.micStream) {
      session.micStream = await navigator.mediaDevices.getUserMedia({
        audio: micConstraints(state.micId),
      });
      session.micTrack = session.micStream.getAudioTracks()[0];
      const stream = session.micStream;
      session.stops.push(() => stream.getTracks().forEach((track) => track.stop()));
    }
    call.answer(session.micStream);
  } catch {
    setState({ status: "mic unavailable — answering without talkback…" });
    call.answer();
  }
}

// -- QoS (CAP-N56) ----------------------------------------------------------

/** Start the per-guest QoS poll on a connected call. */
function startQos(guest: GuestConn): void {
  if (guest.qosTimer) window.clearInterval(guest.qosTimer);
  guest.qosTimer = window.setInterval(() => {
    const pc = guest.call?.peerConnection;
    if (!pc) return;
    void pc
      .getStats()
      .then((stats) => {
        const sample = sampleQos(stats, guest.qosPrev);
        guest.qosPrev = sample.prev;
        guest.qos = sample.qos;
        publishGuests();
      })
      .catch(() => undefined);
  }, QOS_INTERVAL_MS);
}

// ===========================================================================
// GUEST
// ===========================================================================

/** Guest: share the webcam + mic and call the host's peer id. A dropped
 * session auto-rejoins with backoff (`rejoining`) as a fresh guest host-side. */
export async function spikeJoin(
  hostPeerId: string,
  opts: { rejoining?: boolean } = {},
): Promise<void> {
  const carried = opts.rejoining ? rejoin : null;
  spikeStop();
  rejoin = carried;
  setState({
    active: true,
    role: "guest",
    status: "requesting the webcam + mic…",
    peerId: null,
    gates: null,
    cue: null,
    greenRoom: false,
  });
  let stream: MediaStream;
  try {
    stream = await navigator.mediaDevices.getUserMedia({
      video: true,
      audio: micConstraints(state.micId),
    });
  } catch (err) {
    if (rejoin) {
      scheduleRejoin(hostPeerId);
      return;
    }
    setState({ status: `camera/mic blocked: ${err}` });
    return;
  }
  // Persist the guest's own peer id (like the host's) so a rejoin reuses it:
  // the host then reclaims the same tile/source instead of stacking a duplicate,
  // and a ban's denylist actually catches a reconnection (TASK-R11 / R8).
  const peer = await buildPeer(loadPref(GUEST_ID_KEY));
  const session: LiveSession = {
    peer,
    stops: [() => stream.getTracks().forEach((track) => track.stop())],
    role: "guest",
    guests: new Map(),
    screenOuts: [],
    gates: GATES_CLEAR,
    guestMicTrack: stream.getAudioTracks()[0],
  };
  live = session;
  session.hostPeer = hostPeerId;
  // The host may share their view back — accept media calls only from the
  // host we joined; anything else is refused.
  peer.on("call", (call) => {
    if (call.peer !== hostPeerId) {
      call.close();
      return;
    }
    session.screenIn = call;
    call.answer();
    call.on("stream", (hostStream) =>
      setState({ hostShare: hostStream, status: "the host is sharing their view." }),
    );
    call.on("close", () =>
      setState({ hostShare: null, status: "the host stopped sharing their view." }),
    );
  });
  peer.on("open", (id) => {
    savePref(GUEST_ID_KEY, id);
    setState({ status: "calling the host…" });
    const conn = peer.connect(hostPeerId);
    session.data = conn;
    conn.on("open", () => {
      rejoin = null; // connected — the backoff cycle is over
      setState({ gates: session.gates ?? GATES_CLEAR });
    });
    conn.on("data", (raw) => {
      const msg = raw as ControlMsg;
      if (msg?.t === "host") {
        session.gates = { ...(session.gates ?? GATES_CLEAR), hostGate: msg.muted };
        setState({ gates: session.gates });
      } else if (msg?.t === "allowCenter") {
        setState({ allowCenter: msg.allowed });
      } else if (msg?.t === "cue") {
        setState({ cue: { text: msg.text, seconds: msg.seconds, at: Date.now() } });
      } else if (msg?.t === "greenRoom") {
        setState({ greenRoom: msg.staged });
      } else if (msg?.t === "bye") {
        session.byeReason = msg.reason;
      }
    });
    conn.on("close", () => {
      if (live !== session) return; // an intentional Leave already handled it
      if (session.byeReason) {
        const message =
          session.byeReason === "removed"
            ? "the host removed you — the link still works if you're invited back."
            : session.byeReason === "banned"
              ? "the host removed you from this session."
              : "the host ended the session.";
        spikeStop(message);
        return;
      }
      teardownLive();
      scheduleRejoin(hostPeerId);
    });
    const call = peer.call(hostPeerId, stream);
    session.call = call;
    watchIce(call);
    call.on("stream", (hostStream) => playHostAudio(hostStream, session));
    call.on("error", (err) => setState({ status: `call error: ${err.type}` }));
    setState({ status: "connected — sharing your webcam + mic. You can leave this open." });
  });
  peer.on("disconnected", () => {
    setState({ status: "signaling broker lost — reconnecting (media keeps flowing)…" });
    peer.reconnect();
  });
  peer.on("error", (err) => {
    // The broker still holds our persisted id (e.g. after a crash) — drop it
    // and rejoin with a fresh identity so a stale id can't wedge the guest.
    if (err.type === "unavailable-id") {
      savePref(GUEST_ID_KEY, null);
      if (live === session) teardownLive();
      window.setTimeout(() => void spikeJoin(hostPeerId, { rejoining: !!carried }), 0);
      return;
    }
    if (rejoin && (err.type === "peer-unavailable" || err.type === "network")) {
      if (live === session) teardownLive();
      scheduleRejoin(hostPeerId);
      return;
    }
    setState({ status: `peer error: ${err.type}` });
  });
}

/** Guest: play the host's return audio through the selected output device. */
function playHostAudio(hostStream: MediaStream, session: LiveSession): void {
  if (hostStream.getAudioTracks().length === 0) return;
  let element = session.hostAudioEl;
  if (!element) {
    element = document.createElement("audio");
    element.autoplay = true;
    session.hostAudioEl = element;
    session.stops.push(() => {
      session.hostAudioEl = undefined;
      if (element) element.srcObject = null;
    });
  }
  element.srcObject = hostStream;
  void applySink(element, state.speakerId);
  void element.play().catch(() => {});
}

/** Route an element to an output device; honest status when it can't. */
async function applySink(element: HTMLAudioElement, speakerId: string | null): Promise<void> {
  const sink = await routeToOutput(element, speakerId);
  if (sink === "unsupported") {
    setState({ status: "output-device selection isn't supported here — using the default" });
  } else if (sink === "failed") {
    setState({ status: "couldn't switch the output device — using the default" });
  }
}

// ===========================================================================
// Host moderation + per-guest controls
// ===========================================================================

/**
 * Host moderation (TASK-R8): remove one guest. The call + control channel close
 * (the guest's app sees an honest session end), the bridge stops, and the
 * guest's tile frees. With `ban`, the peer id is denylisted (persisted) so a
 * reconnect is refused. If the banned guest was the ONLY one, the host session
 * id also rotates (a fresh invite link — the old one dies), exactly as the
 * single-guest ban did; with other guests still connected the shared link must
 * stay live for them, so a banned guest who clears their id and reuses the link
 * could rejoin as a new guest (an honest limit of a shared invite).
 */
export async function spikeRemoveGuest(peerId: string, ban: boolean): Promise<void> {
  const session = live;
  if (!session || session.role !== "host") return;
  const guest = session.guests.get(peerId);
  if (!guest) return;
  if (ban) {
    DENYLIST.add(peerId);
    saveDenylist();
  }
  // Deliberate goodbye first — the guest's app must end instead of treating
  // the close as a drop and auto-rejoining.
  try {
    guest.data?.send({ t: "bye", reason: ban ? "banned" : "removed" } satisfies ControlMsg);
  } catch {
    // The channel may already be dead — the drop below still lands.
  }
  await dropGuest(session, peerId, ban ? "banned" : "removed");
  // Kill the invite link on a ban when no one else is on it — a fresh host id
  // invalidates every link a lone banned guest still holds.
  if (ban && session.guests.size === 0 && session.sceneId) {
    const ttl = session.ttlMinutes ?? 30;
    void spikeHost(session.sceneId, ttl, { freshId: true });
    setState({ status: "guest banned — minting a fresh invite link (the old one is dead)…" });
  }
}

/** Host action: change the invite TTL — re-mints the link for the live id. */
export function spikeSetInviteTtl(ttlMinutes: number): void {
  const session = live;
  if (!session || session.role !== "host") return;
  session.ttlMinutes = ttlMinutes;
  if (state.peerId) {
    setState({ invite: buildInviteLink(mintInvite(state.peerId, ttlMinutes, Date.now())) });
  }
}

/** Host action: set/clear the host gate for one guest. */
export function spikeSetHostGate(peerId: string, muted: boolean): void {
  const session = live;
  if (!session || session.role !== "host") return;
  const guest = session.guests.get(peerId);
  if (!guest) return;
  guest.gates = { ...guest.gates, hostGate: muted };
  guest.data?.send({ t: "host", muted } satisfies ControlMsg);
  publishGuests();
}

/** Guest action: toggle the self gate (no-op under a host mute), mute the mic
 * at the source, and tell the host. */
export function spikeToggleSelfMute(): void {
  const session = live;
  if (!session || session.role !== "guest") return;
  const gates = session.gates ?? GATES_CLEAR;
  const next = guestToggleSelf(gates);
  if (next === gates) return; // locked by the host gate
  session.gates = next;
  if (session.guestMicTrack) session.guestMicTrack.enabled = !next.selfGate;
  session.data?.send({ t: "self", muted: next.selfGate } satisfies ControlMsg);
  setState({ gates: session.gates });
}

/** Host: grant/revoke one guest's ability to switch the center view. */
export function spikeSetAllowCenter(peerId: string, allowed: boolean): void {
  const session = live;
  if (!session || session.role !== "host") return;
  const guest = session.guests.get(peerId);
  if (!guest) return;
  guest.allowCenter = allowed;
  guest.data?.send({ t: "allowCenter", allowed } satisfies ControlMsg);
  publishGuests();
}

/** Guest: ask the host to switch the center view. */
export function spikeRequestCenter(view: "guestCam" | "guestScreen" | "hostView"): void {
  const session = live;
  if (!session || session.role !== "guest") return;
  session.data?.send({ t: "centerReq", view } satisfies ControlMsg);
}

/** Host: apply an allowed guest request — ignored unless that guest's grant is on. */
function applyGuestCenterRequest(
  guest: GuestConn,
  view: "guestCam" | "guestScreen" | "hostView",
): void {
  if (!guest.allowCenter) return;
  const fail = (err: unknown) => setState({ status: `center switch failed: ${err}` });
  if (view === "guestCam" && guest.guestItem) {
    void studioSetCenterView(guest.guestItem.sceneId, guest.guestItem.itemId).catch(fail);
  } else if (view === "guestScreen" && guest.guestScreenItem) {
    void studioSetCenterView(guest.guestScreenItem.sceneId, guest.guestScreenItem.itemId).catch(
      fail,
    );
  } else if (view === "hostView" && guest.guestItem) {
    void studioSetCenterView(guest.guestItem.sceneId, null).catch(fail);
  }
}

// -- Cue & talk (CAP-N55) ---------------------------------------------------

/** Send a private text cue to one guest (or every guest when `peerId` is
 * null). Rides the existing P2P data channel — no new network surface. */
export function spikeSendCue(peerId: string | null, text: string, seconds: number | null): void {
  const session = live;
  if (!session || session.role !== "host") return;
  const msg: ControlMsg = { t: "cue", text, seconds };
  const targets = peerId
    ? [session.guests.get(peerId)].filter(Boolean)
    : [...session.guests.values()];
  for (const guest of targets) {
    try {
      guest?.data?.send(msg);
    } catch {
      // The channel may be momentarily gone — the cue is best-effort.
    }
  }
}

// -- Auto-grid (CAP-N59) ----------------------------------------------------

/** Host: whether the grid re-arranges automatically on join/leave. */
export function spikeSetAutoGrid(on: boolean): void {
  setState({ autoGridArmed: on });
  if (on) void spikeAutoGrid();
}

/** Host: arrange every seated guest into a reflowing grid (CAP-N59). The
 * geometry + nameplates are engine-side; this just names the participants. */
export async function spikeAutoGrid(): Promise<void> {
  const session = live;
  if (!session || session.role !== "host" || !session.sceneId) return;
  const participants = [...session.guests.values()]
    .filter((guest) => guest.guestItem && !guest.greenRoom)
    .map((guest) => ({ itemId: guest.guestItem!.itemId, name: guest.label }));
  if (participants.length === 0) return;
  await studioAutoGrid(session.sceneId, participants).catch((err) =>
    setState({ status: `auto-grid failed: ${err}` }),
  );
}

// ===========================================================================
// Screen share + device selection
// ===========================================================================

/**
 * Share this side's screen into the session (TASK-R6). Guest → the host gets a
 * "Guest Screen" source; host → every guest's session bar shows the host's
 * view. Passing `false` stops the share.
 */
export async function spikeShareScreen(share: boolean): Promise<void> {
  const session = live;
  if (!session) return;
  if (!share) {
    session.screenStream?.getTracks().forEach((track) => track.stop());
    session.screenStream = undefined;
    // Close EVERY outgoing screen call (the host has one per guest) so each
    // peer's `close` fires and the frozen last frame clears.
    session.screenOuts.forEach((call) => call.close());
    session.screenOuts = [];
    setState({ sharingScreen: false });
    return;
  }
  const targets =
    session.role === "host"
      ? [...session.guests.keys()]
      : session.hostPeer
        ? [session.hostPeer]
        : [];
  if (targets.length === 0) {
    setState({ status: "no connected peer to share the screen with yet." });
    return;
  }
  let stream: MediaStream;
  try {
    stream = await navigator.mediaDevices.getDisplayMedia({ video: true });
  } catch (err) {
    setState({ status: `screen share blocked: ${err}` });
    return;
  }
  session.screenStream = stream;
  session.stops.push(() => stream.getTracks().forEach((track) => track.stop()));
  stream.getVideoTracks()[0]?.addEventListener("ended", () => void spikeShareScreen(false));
  // The host calls every guest; a guest calls the one host. Track every call
  // so a later Stop closes all of them.
  const metadata = { kind: session.role === "host" ? "hostScreen" : "screen" };
  session.screenOuts = targets.map((targetPeer) =>
    session.peer.call(targetPeer, stream, { metadata }),
  );
  setState({ sharingScreen: true, status: "sharing your screen into the session." });
}

/** Pick the microphone this machine sends into the session. Applies LIVE to
 * every active call (RTCRtpSender.replaceTrack — no renegotiation) and to
 * every later session. */
export async function spikeSetMic(deviceId: string | null): Promise<void> {
  savePref(MIC_KEY, deviceId);
  setState({ micId: deviceId });
  const session = live;
  if (!session) return;
  // The active calls whose audio sender should swap.
  const calls: MediaConnection[] =
    session.role === "host"
      ? [...session.guests.values()]
          .map((guest) => guest.call)
          .filter((c): c is MediaConnection => !!c)
      : session.call
        ? [session.call]
        : [];
  if (calls.length === 0) return;
  try {
    const mic = await navigator.mediaDevices.getUserMedia({ audio: micConstraints(deviceId) });
    const track = mic.getAudioTracks()[0];
    if (!track) return;
    const keepEnabled =
      session.role === "guest"
        ? (session.guestMicTrack?.enabled ?? true)
        : (session.micTrack?.enabled ?? true);
    track.enabled = keepEnabled;
    for (const call of calls) {
      const sender = call.peerConnection?.getSenders().find((s) => s.track?.kind === "audio");
      if (sender) await sender.replaceTrack(track);
    }
    if (session.role === "guest") {
      session.guestMicTrack?.stop();
      session.guestMicTrack = track;
    } else {
      session.micTrack?.stop();
      session.micTrack = track;
      // Refresh the cached talkback stream too, so a guest that joins AFTER
      // the device switch is answered with the live mic, not the stopped track.
      session.micStream = mic;
    }
    session.stops.push(() => track.stop());
  } catch (err) {
    setState({ status: `couldn't switch the mic: ${err}` });
  }
}

/** Pick the output device the other side's audio plays through (live). */
export async function spikeSetSpeaker(deviceId: string | null): Promise<void> {
  savePref(SPEAKER_KEY, deviceId);
  setState({ speakerId: deviceId });
  const element = live?.hostAudioEl;
  if (element) await applySink(element, deviceId);
}

// ===========================================================================
// The webview → compositor bridge
// ===========================================================================

/** rVFC → canvas → RGBA → raw-payload IPC. Returns a stop() so one bridge (e.g.
 * an unshared screen) can end without tearing the session down. */
async function addAndPump(
  stream: MediaStream,
  sceneId: SceneId,
  guest: GuestConn,
  target: { label: string; assign: "guestItem" | "guestScreenItem" },
): Promise<() => void> {
  const existing = target.assign === "guestItem" ? guest.guestItem : guest.guestScreenItem;
  let ref: GuestItemRef;
  if (existing && existing.sceneId === sceneId) {
    ref = existing;
  } else {
    const added = await studioAddItem(
      sceneId,
      { kind: "remoteGuest", label: target.label },
      target.label,
    );
    ref = { sceneId, itemId: added.itemId, sourceId: added.sourceId };
  }
  if (target.assign === "guestItem") guest.guestItem = ref;
  else guest.guestScreenItem = ref;
  publishGuests();

  const video = document.createElement("video") as VideoWithRvfc;
  video.muted = true;
  video.playsInline = true;
  video.srcObject = stream;
  await video.play();

  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d", { willReadFrequently: true });
  if (!ctx) {
    setState({ status: "no 2d canvas — cannot bridge frames" });
    return () => {};
  }

  let alive = true;
  const stopAudio = pumpGuestAudio(stream, ref.sourceId, guest);
  const stop = () => {
    alive = false;
    video.srcObject = null;
    stopAudio();
  };
  // Owned by the guest only — teardownGuest runs these; a stale copy in
  // session.stops would double-fire stop()/close on full teardown.
  guest.stops.push(stop);

  const pump = () => {
    if (!alive) return;
    const videoWidth = video.videoWidth;
    const videoHeight = video.videoHeight;
    if (videoWidth && videoHeight) {
      const scale = Math.min(1, MAX_FRAME_WIDTH / videoWidth);
      const width = Math.max(2, Math.round(videoWidth * scale) & ~1);
      const height = Math.max(2, Math.round(videoHeight * scale) & ~1);
      if (canvas.width !== width) canvas.width = width;
      if (canvas.height !== height) canvas.height = height;
      ctx.drawImage(video, 0, 0, width, height);
      const rgba = ctx.getImageData(0, 0, width, height).data;
      invoke("remote_guest_push_frame", new Uint8Array(rgba.buffer), {
        headers: {
          "x-fcap-source": ref.sourceId,
          "x-fcap-width": String(width),
          "x-fcap-height": String(height),
        },
      }).catch(() => {
        // Dropped frame (e.g. the source is still starting) — latest-wins.
      });
    }
    video.requestVideoFrameCallback(pump);
  };
  video.requestVideoFrameCallback(pump);
  return stop;
}

/**
 * Tap the guest's WebRTC mic and push interleaved-stereo 48 kHz f32 to the
 * mixer over IPC — the guest becomes a mixer strip. The host gate is per-guest.
 *
 * Chromium quirk: a *remote* WebRTC audio track is silent through Web Audio
 * unless the stream is also attached to a media element (kept muted) to prime
 * the pipeline, and the AudioContext is resumed.
 */
function pumpGuestAudio(stream: MediaStream, sourceId: string, guest: GuestConn): () => void {
  if (stream.getAudioTracks().length === 0) return () => {};

  const primer = document.createElement("audio");
  primer.muted = true;
  primer.srcObject = stream;
  void primer.play().catch(() => {});

  const AudioCtx: typeof AudioContext =
    window.AudioContext ??
    (window as unknown as { webkitAudioContext: typeof AudioContext }).webkitAudioContext;
  const ctx = new AudioCtx({ sampleRate: AUDIO_RATE });
  void ctx.resume();
  const source = ctx.createMediaStreamSource(stream);
  const processor = ctx.createScriptProcessor(4096, 1, 1);
  processor.onaudioprocess = (event) => {
    // Host gate: drop this guest's audio while the host has muted them.
    if (guest.gates.hostGate) return;
    const input = event.inputBuffer;
    const left = input.getChannelData(0);
    const right = input.numberOfChannels > 1 ? input.getChannelData(1) : left;
    const interleaved = new Float32Array(left.length * 2);
    for (let i = 0; i < left.length; i += 1) {
      interleaved[2 * i] = left[i];
      interleaved[2 * i + 1] = right[i];
    }
    invoke("remote_guest_push_audio", new Uint8Array(interleaved.buffer), {
      headers: { "x-fcap-source": sourceId },
    }).catch(() => {
      // The source is still starting / already stopped — drop the block.
    });
  };
  source.connect(processor);
  processor.connect(ctx.destination); // required to pump; output stays silent
  return () => {
    processor.disconnect();
    source.disconnect();
    void ctx.close();
    primer.srcObject = null;
  };
}
