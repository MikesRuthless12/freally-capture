/**
 * Remote Guests — the P2P session (transport spike + invites + two-gate mute).
 *
 * PeerJS (vendored, no CDN) brokers a WebRTC session; media flows peer-to-peer.
 * The HOST answers a guest's call receive-only, adds a RemoteGuest source, and
 * pumps the remote video frames into the compositor and the remote mic into the
 * mixer. The GUEST shares their webcam + mic and calls the host.
 *
 * Session state lives in a small **store** here — not in any one component — so
 * the status + mute controls persist on the main UI even when the setup dialog
 * is closed. Components subscribe via `spikeSubscribe`.
 */
import Peer, { type DataConnection, type MediaConnection } from "peerjs";
import { invoke } from "@tauri-apps/api/core";

import { settingsGet, studioAddItem, studioRemoveItem, studioSetCenterView } from "../api/commands";
import type { ItemId, SceneId } from "../api/types";
import { micConstraints, routeToOutput } from "./devices";
import { inviteLink as buildInviteLink, joinTargetFromInput, mintInvite } from "./invite";
import { GATES_CLEAR, type GateState, guestToggleSelf } from "./mute";

/** Downscale bound for the spike's IPC frames (keeps a push ≤ ~0.9 MB). */
const MAX_FRAME_WIDTH = 640;
/** The mixer's ring rate. WebRTC audio is 48 kHz; we tap it at that rate. */
const AUDIO_RATE = 48_000;

/** rVFC is not in every lib.dom yet — the narrow slice the pump needs. */
type VideoWithRvfc = HTMLVideoElement & {
  requestVideoFrameCallback: (callback: () => void) => number;
};

/** A control message on the host↔guest data channel: the two mute gates,
 * the host's view-switching grant, a guest's center-view request, and the
 * deliberate-goodbye marker (so a kicked/ended guest doesn't auto-rejoin). */
type ControlMsg =
  | { t: "host"; muted: boolean }
  | { t: "self"; muted: boolean }
  | { t: "allowCenter"; allowed: boolean }
  | { t: "centerReq"; view: "guestCam" | "guestScreen" | "hostView" }
  | { t: "bye"; reason: "removed" | "banned" | "ended" };

/** A bridged guest item: the scene item + the source its frames feed. */
export type GuestItemRef = { sceneId: SceneId; itemId: ItemId; sourceId: string };

/** The observable session state (what the UI renders). */
export type SpikeState = {
  active: boolean;
  role: "host" | "guest" | null;
  status: string;
  /** The host's session id — the invite link is minted from it. */
  peerId: string | null;
  /** The minted invite link (host only; re-mints when the TTL changes). */
  invite: string | null;
  /** Non-null once a peer connects (drives the mute controls). */
  gates: GateState | null;
  /** Host: the guest's scene item — drives the position presets, and lets a
   * rejoining guest reclaim the same seat + source (TASK-R11). */
  guestItem: GuestItemRef | null;
  /** Host: the guest's shared-screen item (present while they share). */
  guestScreenItem: GuestItemRef | null;
  /** Host: whether the guest may switch the center view. Guest: granted? */
  allowCenter: boolean;
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

const IDLE: SpikeState = {
  active: false,
  role: null,
  status: "idle — nothing touches the network yet",
  peerId: null,
  invite: null,
  gates: null,
  guestItem: null,
  guestScreenItem: null,
  allowCenter: false,
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

function loadPref(key: string): string | null {
  try {
    return window.localStorage.getItem(key);
  } catch {
    return null;
  }
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

type LiveSession = {
  peer: Peer;
  stops: Array<() => void>;
  role: "host" | "guest";
  gates: GateState;
  /** Host: the scene guests land in (needed to re-host after a ban). */
  sceneId?: SceneId;
  /** Host: the invite TTL the link is minted with (re-mints on change). */
  ttlMinutes?: number;
  /** Host: the connected guest's peer id (the ban denylist key). */
  guestPeer?: string;
  data?: DataConnection;
  /** The media call (guest: outbound; host: the answered call). */
  call?: MediaConnection;
  /** Guest: the host's peer id (screen shares call back to it). */
  hostPeer?: string;
  /** Guest: the host said goodbye deliberately — do NOT auto-rejoin. */
  byeReason?: "removed" | "banned" | "ended";
  /** An incoming shared-screen call (guest's screen / host's view). */
  screenIn?: MediaConnection;
  /** This side's outgoing screen-share call + stream. */
  screenOut?: MediaConnection;
  screenStream?: MediaStream;
  /** The outbound mic track — self-mute / live device switch touch it. */
  micTrack?: MediaStreamTrack;
  /** Guest: the element the host's return audio plays through. */
  hostAudioEl?: HTMLAudioElement;
};
let live: LiveSession | null = null;

// TASK-R8: banned guest peer ids. Persisted so a ban survives a relaunch —
// honest limitation: a guest who mints a fresh peer id needs a valid invite
// link to rejoin, and a ban also rotates the host session id, so old links
// die with it.
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

/** Stop the live session's resources without touching the store. */
function teardownLive(): void {
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
    // A courtesy goodbye so the guest's app ends cleanly instead of
    // auto-rejoining a dead session (best-effort — teardown proceeds).
    try {
      live.data?.send({ t: "bye", reason: "ended" } satisfies ControlMsg);
    } catch {
      // The channel may already be gone.
    }
    // A deliberate end removes the bridged items — nothing stays frozen in
    // the scene (a mid-session DROP keeps them for the rejoin reclaim).
    for (const item of [state.guestItem, state.guestScreenItem]) {
      if (item) void studioRemoveItem(item.sceneId, item.itemId).catch(() => undefined);
    }
  }
  teardownLive();
  // Device preferences + a pending deep-link invite survive the reset.
  state = {
    ...IDLE,
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

/** Host: wait for a guest; when media arrives, bridge video + audio in.
 * The session id persists across relaunches (TASK-R11): a restarted host
 * reclaims it so already-shared invite links keep working; a ban rotates it
 * (`freshId`) so the old links die. */
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
    gates: null,
  });
  if (opts.freshId) savePref(HOST_ID_KEY, null);
  const requestedId = loadPref(HOST_ID_KEY);
  const peer = await buildPeer(requestedId);
  const session: LiveSession = {
    peer,
    stops: [],
    role: "host",
    gates: GATES_CLEAR,
    sceneId,
    ttlMinutes,
  };
  live = session;
  peer.on("open", (id) => {
    savePref(HOST_ID_KEY, id);
    setState({
      peerId: id,
      invite: buildInviteLink(mintInvite(id, session.ttlMinutes ?? ttlMinutes, Date.now())),
      status: "hosting — share the invite link; waiting for a guest…",
    });
  });
  peer.on("error", (err) => {
    if (err.type === "unavailable-id" && requestedId) {
      // The broker still holds the previous session (e.g. after a crash).
      // Retry the same id once — old invite links stay alive that way —
      // then give up and mint a fresh identity.
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
  // Honest signaling status: a broker drop doesn't kill the P2P media, but
  // new guests can't join until it comes back.
  peer.on("disconnected", () => {
    setState({ status: "signaling broker lost — reconnecting (media keeps flowing)…" });
    peer.reconnect();
  });
  // The guest opens a control channel (mute state) alongside the media call.
  peer.on("connection", (conn) => {
    if (DENYLIST.has(conn.peer)) {
      conn.close();
      setState({ status: "a banned guest tried to rejoin — refused." });
      return;
    }
    // One guest at a time (the PoC transport): a second peer while the seat
    // is taken is refused instead of silently hijacking the session state.
    if (session.data && conn.peer !== session.guestPeer) {
      conn.close();
      setState({ status: "another guest tried to join — sessions are 1:1 for now." });
      return;
    }
    session.data = conn;
    session.guestPeer = conn.peer;
    // Reveal the host mute control as soon as the guest connects — don't rely
    // only on "open", which PeerJS can fire before this handler on the receiver.
    setState({ gates: session.gates, status: "guest connected." });
    const sendHostGate = () =>
      conn.send({ t: "host", muted: session.gates.hostGate } satisfies ControlMsg);
    if (conn.open) sendHostGate();
    else conn.on("open", sendHostGate);
    conn.on("data", (raw) => {
      const msg = raw as ControlMsg;
      if (msg?.t === "self") {
        session.gates = { ...session.gates, selfGate: msg.muted };
        setState({ gates: session.gates });
      } else if (msg?.t === "centerReq") {
        applyGuestCenterRequest(msg.view);
      }
    });
    conn.on("close", () => {
      // The control channel is the liveness signal — honest drop reporting.
      // Free the single guest seat (a rejoining guest arrives with a FRESH
      // peer id) and revoke the per-guest view-switching grant.
      session.data = undefined;
      session.call = undefined;
      session.guestPeer = undefined;
      setState({
        gates: null,
        allowCenter: false,
        status: "guest disconnected — they can rejoin with the same link while it's valid.",
      });
    });
  });
  peer.on("call", (call) => {
    if (DENYLIST.has(call.peer)) {
      call.close();
      setState({ status: "a banned guest tried to call — refused." });
      return;
    }
    const kind = (call.metadata as { kind?: string } | null)?.kind ?? "cam";
    if (kind === "screen") {
      // The guest's shared screen — only from the CONNECTED guest.
      if (call.peer !== session.guestPeer) {
        call.close();
        return;
      }
      session.screenIn = call;
      call.answer();
      watchIce(call);
      let stopScreen: (() => void) | undefined;
      call.on("stream", (stream) => {
        setState({ status: "guest screen arrived — bridging in…" });
        addAndPump(stream, sceneId, session, { label: "Guest Screen", assign: "guestScreenItem" })
          .then((stop) => {
            stopScreen = stop;
          })
          .catch((err) => setState({ status: `screen bridge failed: ${err}` }));
      });
      call.on("close", () => {
        stopScreen?.();
        const item = state.guestScreenItem;
        if (item) void studioRemoveItem(item.sceneId, item.itemId).catch(() => undefined);
        setState({ guestScreenItem: null, status: "the guest stopped sharing their screen." });
      });
      return;
    }
    if (session.call && call.peer !== session.guestPeer) {
      call.close();
      setState({ status: "another guest tried to call — sessions are 1:1 for now." });
      return;
    }
    session.call = call;
    session.guestPeer = call.peer;
    setState({ status: "guest calling — answering…" });
    watchIce(call);
    void answerWithTalkback(call, session);
    call.on("stream", (stream) => {
      setState({ status: "guest media arrived — bridging in…" });
      addAndPump(stream, sceneId, session).catch((err) =>
        setState({ status: `bridge failed: ${err}` }),
      );
    });
    call.on("error", (err) => setState({ status: `guest call error: ${err.type}` }));
    call.on("close", () =>
      setState({
        gates: null,
        status: "guest call closed — they can rejoin with the same link while it's valid.",
      }),
    );
  });
}

/** Host: answer the guest's call WITH this machine's mic (return audio, so
 * the guest hears the host). Falls back to a receive-only answer when the mic
 * is unavailable — the session still works, just without talkback. */
async function answerWithTalkback(call: MediaConnection, session: LiveSession): Promise<void> {
  try {
    const mic = await navigator.mediaDevices.getUserMedia({ audio: micConstraints(state.micId) });
    session.micTrack = mic.getAudioTracks()[0];
    session.stops.push(() => mic.getTracks().forEach((track) => track.stop()));
    call.answer(mic);
  } catch {
    setState({ status: "mic unavailable — answering without talkback…" });
    call.answer();
  }
}

/** Guest: share the webcam + mic and call the host's peer id. A dropped
 * session auto-rejoins with backoff (`rejoining`); the seat is reclaimed
 * host-side because the guest's item + source are reused. */
export async function spikeJoin(
  hostPeerId: string,
  opts: { rejoining?: boolean } = {},
): Promise<void> {
  // A rejoin attempt must keep its backoff count across the internal reset.
  const carried = opts.rejoining ? rejoin : null;
  spikeStop();
  rejoin = carried;
  setState({
    active: true,
    role: "guest",
    status: "requesting the webcam + mic…",
    peerId: null,
    gates: null,
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
  const peer = await buildPeer();
  const session: LiveSession = {
    peer,
    stops: [() => stream.getTracks().forEach((track) => track.stop())],
    role: "guest",
    gates: GATES_CLEAR,
    micTrack: stream.getAudioTracks()[0],
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
  peer.on("open", () => {
    setState({ status: "calling the host…" });
    const conn = peer.connect(hostPeerId);
    session.data = conn;
    conn.on("open", () => {
      rejoin = null; // connected — the backoff cycle is over
      setState({ gates: session.gates }); // reveal the guest mute button
    });
    conn.on("data", (raw) => {
      const msg = raw as ControlMsg;
      if (msg?.t === "host") {
        session.gates = { ...session.gates, hostGate: msg.muted };
        setState({ gates: session.gates });
      } else if (msg?.t === "allowCenter") {
        setState({ allowCenter: msg.allowed });
      } else if (msg?.t === "bye") {
        session.byeReason = msg.reason;
      }
    });
    conn.on("close", () => {
      if (live !== session) return; // an intentional Leave already handled it
      // A deliberate goodbye (remove/ban/end) ends the session cleanly; only
      // an unexplained drop auto-rejoins with backoff (TASK-R11).
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
    // The host answers with their mic — play it through the chosen output.
    call.on("stream", (hostStream) => playHostAudio(hostStream, session));
    call.on("error", (err) => setState({ status: `call error: ${err.type}` }));
    setState({ status: "connected — sharing your webcam + mic. You can leave this open." });
  });
  peer.on("disconnected", () => {
    setState({ status: "signaling broker lost — reconnecting (media keeps flowing)…" });
    peer.reconnect();
  });
  peer.on("error", (err) => {
    // Mid-rejoin, "the host isn't there yet" just means try again later.
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

/**
 * Host moderation (TASK-R8): remove the connected guest. The call + control
 * channel close (the guest's app sees an honest session end), the bridge
 * resources stop, and the guest's scene item frees its seat. With `ban`, the
 * guest's peer id is denylisted (persisted) AND the session id rotates — a
 * fresh Peer mints a fresh invite link, so the banned guest's old link is
 * dead. A plain remove keeps hosting on the same link.
 */
export async function spikeRemoveGuest(ban: boolean): Promise<void> {
  const session = live;
  if (!session || session.role !== "host") return;
  if (ban && session.guestPeer) {
    DENYLIST.add(session.guestPeer);
    saveDenylist();
  }
  // Deliberate goodbye first — the guest's app must end instead of treating
  // the close as a drop and auto-rejoining two seconds later.
  try {
    session.data?.send({ t: "bye", reason: ban ? "banned" : "removed" } satisfies ControlMsg);
  } catch {
    // The channel may already be dead — the close below still lands.
  }
  // Per-guest teardown; the hosting Peer itself stays up (unless banning).
  session.call?.close();
  session.data?.close();
  session.screenIn?.close();
  session.call = undefined;
  session.data = undefined;
  session.screenIn = undefined;
  session.guestPeer = undefined;
  session.gates = GATES_CLEAR;
  session.stops.forEach((stop) => stop());
  session.stops = [];
  for (const item of [state.guestItem, state.guestScreenItem]) {
    if (item) await studioRemoveItem(item.sceneId, item.itemId).catch(() => undefined);
  }
  if (ban) {
    const sceneId = session.sceneId;
    const ttl = session.ttlMinutes ?? 30;
    if (sceneId) {
      // freshId: the id rotation IS the invalidation — the old link must die.
      void spikeHost(sceneId, ttl, { freshId: true });
      setState({ status: "guest banned — minting a fresh invite link (the old one is dead)…" });
    } else {
      spikeStop("guest banned — start hosting again for a fresh link.");
    }
  } else {
    setState({
      gates: null,
      guestItem: null,
      guestScreenItem: null,
      allowCenter: false,
      status: "guest removed — the invite link still works if you want them back.",
    });
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

/** Host action: set/clear the host gate for the connected guest. */
export function spikeSetHostGate(muted: boolean): void {
  const session = live;
  if (!session || session.role !== "host") return;
  session.gates = { ...session.gates, hostGate: muted };
  session.data?.send({ t: "host", muted } satisfies ControlMsg);
  setState({ gates: session.gates });
}

/** Guest action: toggle the self gate (no-op under a host mute), mute the mic
 * at the source, and tell the host. */
export function spikeToggleSelfMute(): void {
  const session = live;
  if (!session || session.role !== "guest") return;
  const next = guestToggleSelf(session.gates);
  if (next === session.gates) return; // locked by the host gate
  session.gates = next;
  if (session.micTrack) session.micTrack.enabled = !next.selfGate;
  session.data?.send({ t: "self", muted: next.selfGate } satisfies ControlMsg);
  setState({ gates: session.gates });
}

/** Host: grant/revoke the guest's ability to switch the center view. The
 * authority stays host-side — requests are applied only while granted. */
export function spikeSetAllowCenter(allowed: boolean): void {
  const session = live;
  if (!session || session.role !== "host") return;
  setState({ allowCenter: allowed });
  session.data?.send({ t: "allowCenter", allowed } satisfies ControlMsg);
}

/** Guest: ask the host to switch the center view. */
export function spikeRequestCenter(view: "guestCam" | "guestScreen" | "hostView"): void {
  const session = live;
  if (!session || session.role !== "guest") return;
  session.data?.send({ t: "centerReq", view } satisfies ControlMsg);
}

/** Host: apply an allowed guest request — ignored unless the grant is on. */
function applyGuestCenterRequest(view: "guestCam" | "guestScreen" | "hostView"): void {
  if (!state.allowCenter) return;
  const fail = (err: unknown) => setState({ status: `center switch failed: ${err}` });
  if (view === "guestCam" && state.guestItem) {
    void studioSetCenterView(state.guestItem.sceneId, state.guestItem.itemId).catch(fail);
  } else if (view === "guestScreen" && state.guestScreenItem) {
    void studioSetCenterView(state.guestScreenItem.sceneId, state.guestScreenItem.itemId).catch(
      fail,
    );
  } else if (view === "hostView" && state.guestItem) {
    void studioSetCenterView(state.guestItem.sceneId, null).catch(fail);
  }
}

/**
 * Share this side's screen into the session (TASK-R6). Guest → the host gets
 * a "Guest Screen" source it can route into the center; host → the guest's
 * session bar shows the host's view. Passing `false` stops the share.
 */
export async function spikeShareScreen(share: boolean): Promise<void> {
  const session = live;
  if (!session) return;
  if (!share) {
    session.screenStream?.getTracks().forEach((track) => track.stop());
    session.screenStream = undefined;
    session.screenOut?.close();
    session.screenOut = undefined;
    setState({ sharingScreen: false });
    return;
  }
  const targetPeer = session.role === "host" ? session.guestPeer : session.hostPeer;
  if (!targetPeer) {
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
  // The browser's own "stop sharing" bar must end the share honestly too.
  stream.getVideoTracks()[0]?.addEventListener("ended", () => void spikeShareScreen(false));
  const call = session.peer.call(targetPeer, stream, {
    metadata: { kind: session.role === "host" ? "hostScreen" : "screen" },
  });
  session.screenOut = call;
  setState({ sharingScreen: true, status: "sharing your screen into the session." });
}

/** Pick the microphone this machine sends into the session. Applies LIVE
 * when a call is up (RTCRtpSender.replaceTrack — no renegotiation), and to
 * every later session; the guest's mute state carries over to the new mic. */
export async function spikeSetMic(deviceId: string | null): Promise<void> {
  savePref(MIC_KEY, deviceId);
  setState({ micId: deviceId });
  const session = live;
  if (!session) return;
  const senders = session.call?.peerConnection?.getSenders() ?? [];
  const sender = senders.find((candidate) => candidate.track?.kind === "audio");
  if (!sender) {
    if (session.call) setState({ status: "mic saved — it applies when the next call connects" });
    return;
  }
  try {
    const mic = await navigator.mediaDevices.getUserMedia({ audio: micConstraints(deviceId) });
    const track = mic.getAudioTracks()[0];
    if (!track) return;
    track.enabled = session.micTrack?.enabled ?? true; // keep the mute state
    await sender.replaceTrack(track);
    session.micTrack?.stop();
    session.micTrack = track;
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

/** The webview → compositor bridge: rVFC → canvas → RGBA → raw-payload IPC.
 * Returns a stop() so one bridge (e.g. an unshared screen) can end without
 * tearing the session down. */
async function addAndPump(
  stream: MediaStream,
  sceneId: SceneId,
  session: LiveSession,
  target: { label: string; assign: "guestItem" | "guestScreenItem" } = {
    label: "Remote Guest",
    assign: "guestItem",
  },
): Promise<() => void> {
  // A returning guest reclaims their existing item — same seat, same source,
  // same mixer strip (TASK-R11) — instead of stacking a duplicate.
  const existing = target.assign === "guestItem" ? state.guestItem : state.guestScreenItem;
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
  setState({ [target.assign]: ref });

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
  const stopAudio = pumpGuestAudio(stream, ref.sourceId, session);
  const stop = () => {
    alive = false;
    video.srcObject = null;
    stopAudio();
  };
  session.stops.push(stop);

  let pushed = 0;
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
      pushed += 1;
      if (pushed === 1) setState({ status: `${target.label} is live in the scene ✔` });
    }
    video.requestVideoFrameCallback(pump);
  };
  video.requestVideoFrameCallback(pump);
  return stop;
}

/**
 * Tap the guest's WebRTC mic and push interleaved-stereo 48 kHz f32 to the
 * mixer over IPC — the guest becomes a mixer strip.
 *
 * Chromium quirk: a *remote* WebRTC audio track is silent through Web Audio
 * unless the stream is also attached to a media element (kept muted) to prime
 * the pipeline, and the AudioContext is resumed. The ScriptProcessor's output
 * is left silent, so this taps the audio without echoing out the host speakers.
 */
function pumpGuestAudio(stream: MediaStream, sourceId: string, session: LiveSession): () => void {
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
    // Host gate: drop the guest's audio when the host has muted them (the
    // guest's own self-mute is already silenced at their mic via track.enabled).
    if (session.gates.hostGate) return;
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
