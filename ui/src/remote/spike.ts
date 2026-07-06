/**
 * Remote Guests — the TASK-R1 transport spike.
 *
 * PeerJS (vendored npm dep — no CDN) brokers a WebRTC session; the media
 * itself flows peer-to-peer. The HOST answers an incoming call receive-only,
 * adds a RemoteGuest source to the active scene, and pumps the remote video's
 * frames over the raw-payload IPC (`remote_guest_push_frame`) into the
 * compositor. The GUEST shares their webcam and calls the host's session id.
 *
 * Proof-of-concept: the efficient frame path, guest audio, and per-guest
 * status land with TASK-R4; invites/tokens with TASK-R2/R3.
 */
import Peer, { type DataConnection } from "peerjs";
import { invoke } from "@tauri-apps/api/core";

import { studioAddItem } from "../api/commands";
import type { SceneId } from "../api/types";
import { GATES_CLEAR, type GateState, guestToggleSelf } from "./mute";

/** Downscale bound for the spike's IPC frames (keeps a push ≤ ~0.9 MB). */
const MAX_FRAME_WIDTH = 640;

export type SpikeStatus = (message: string) => void;
export type GateListener = (gates: GateState) => void;

/** rVFC is not in every lib.dom yet — the narrow slice the pump needs. */
type VideoWithRvfc = HTMLVideoElement & {
  requestVideoFrameCallback: (callback: () => void) => number;
};

/** A control message on the host↔guest data channel. */
type GateMsg = { t: "host"; muted: boolean } | { t: "self"; muted: boolean };

type LiveSession = {
  peer: Peer;
  stops: Array<() => void>;
  role: "host" | "guest";
  /** Authoritative gate state for this guest (both sides converge on it). */
  gates: GateState;
  onGates?: GateListener;
  /** The control channel (present once the peers connect). */
  data?: DataConnection;
  /** The guest's own outbound mic track — self-mute flips `.enabled`. */
  micTrack?: MediaStreamTrack;
};
let live: LiveSession | null = null;

function emitGates(session: LiveSession): void {
  session.onGates?.(session.gates);
}

/** Tear down whichever spike session (host or guest) is running. */
export function spikeStop(): void {
  live?.stops.forEach((stop) => stop());
  live?.data?.close();
  live?.peer.destroy();
  live = null;
}

/** Host: wait for a guest call; when media arrives, add a RemoteGuest source
 * to the active scene and pump its frames into the compositor. */
export function spikeHost(
  sceneId: SceneId,
  onStatus: SpikeStatus,
  onPeerId: (id: string) => void,
  onGates?: GateListener,
): void {
  spikeStop();
  const peer = new Peer();
  const session: LiveSession = { peer, stops: [], role: "host", gates: GATES_CLEAR, onGates };
  live = session;
  onStatus("connecting to the signaling broker…");
  peer.on("open", (id) => {
    onPeerId(id);
    onStatus("hosting — share the session id; waiting for a guest…");
  });
  peer.on("error", (err) => onStatus(`peer error: ${err.type}`));
  // The guest opens a control channel (mute state) alongside the media call.
  peer.on("connection", (conn) => {
    session.data = conn;
    conn.on("data", (raw) => {
      const msg = raw as GateMsg;
      if (msg?.t === "self") {
        session.gates = { ...session.gates, selfGate: msg.muted };
        emitGates(session);
      }
    });
    conn.on("open", () => {
      conn.send({ t: "host", muted: session.gates.hostGate } satisfies GateMsg);
      emitGates(session); // let the host UI know a guest is connected
    });
  });
  peer.on("call", (call) => {
    onStatus("guest calling — answering (receive-only)…");
    call.answer();
    call.on("stream", (stream) => {
      onStatus("guest media arrived — adding the source…");
      addAndPump(stream, sceneId, session, onStatus).catch((err) =>
        onStatus(`bridge failed: ${err}`),
      );
    });
    call.on("close", () => onStatus("guest call closed"));
  });
}

/** Guest: share the webcam + mic and call the host's session id. */
export async function spikeJoin(
  hostId: string,
  onStatus: SpikeStatus,
  onGates?: GateListener,
): Promise<void> {
  spikeStop();
  onStatus("requesting the webcam + mic…");
  const stream = await navigator.mediaDevices.getUserMedia({ video: true, audio: true });
  const peer = new Peer();
  const session: LiveSession = {
    peer,
    stops: [() => stream.getTracks().forEach((track) => track.stop())],
    role: "guest",
    gates: GATES_CLEAR,
    onGates,
    micTrack: stream.getAudioTracks()[0],
  };
  live = session;
  const id = hostId.trim();
  peer.on("open", () => {
    onStatus("calling the host…");
    // Control channel first (mute state), then the media call.
    const conn = peer.connect(id);
    session.data = conn;
    conn.on("open", () => emitGates(session)); // reveal the guest's mute button
    conn.on("data", (raw) => {
      const msg = raw as GateMsg;
      if (msg?.t === "host") {
        session.gates = { ...session.gates, hostGate: msg.muted };
        emitGates(session);
      }
    });
    const call = peer.call(id, stream);
    call.on("close", () => onStatus("call closed"));
    onStatus("sharing the webcam + mic with the host — keep this app open.");
  });
  peer.on("error", (err) => onStatus(`peer error: ${err.type}`));
}

/** Host action: set/clear the host gate for the connected guest. */
export function spikeSetHostGate(muted: boolean): void {
  const session = live;
  if (!session || session.role !== "host") return;
  session.gates = { ...session.gates, hostGate: muted };
  session.data?.send({ t: "host", muted } satisfies GateMsg);
  emitGates(session);
}

/** Guest action: toggle the self gate (no-op while the host gate is set), mute
 * the mic at the source, and tell the host. */
export function spikeToggleSelfMute(): void {
  const session = live;
  if (!session || session.role !== "guest") return;
  const next = guestToggleSelf(session.gates);
  if (next === session.gates) return; // locked by the host gate
  session.gates = next;
  if (session.micTrack) session.micTrack.enabled = !next.selfGate;
  session.data?.send({ t: "self", muted: next.selfGate } satisfies GateMsg);
  emitGates(session);
}

/** The webview → compositor bridge: rVFC → canvas → RGBA → raw-payload IPC. */
async function addAndPump(
  stream: MediaStream,
  sceneId: SceneId,
  session: LiveSession,
  onStatus: SpikeStatus,
): Promise<void> {
  const added = await studioAddItem(
    sceneId,
    { kind: "remoteGuest", label: "Remote Guest" },
    "Remote Guest",
  );

  const video = document.createElement("video") as VideoWithRvfc;
  video.muted = true;
  video.playsInline = true;
  video.srcObject = stream;
  await video.play();

  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d", { willReadFrequently: true });
  if (!ctx) {
    onStatus("no 2d canvas available — cannot bridge frames");
    return;
  }

  let alive = true;
  session.stops.push(() => {
    alive = false;
    video.srcObject = null;
  });

  pumpGuestAudio(stream, added.sourceId, session);

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
          "x-fcap-source": added.sourceId,
          "x-fcap-width": String(width),
          "x-fcap-height": String(height),
        },
      }).catch(() => {
        // Dropped frame (e.g. the source is still starting) — latest-wins.
      });
      pushed += 1;
      if (pushed === 1) {
        onStatus("first frame pushed — the guest is live in the scene ✔");
      }
    }
    video.requestVideoFrameCallback(pump);
  };
  video.requestVideoFrameCallback(pump);
}

/** The mixer's ring rate. WebRTC audio is 48 kHz; we tap it at that rate so
 * no resampling is needed on either side. */
const AUDIO_RATE = 48_000;

/**
 * Tap the guest's WebRTC mic and push interleaved-stereo 48 kHz f32 to the
 * mixer over IPC — the guest becomes a mixer strip like any audio source. The
 * ScriptProcessor's output is left silent (we never write it), so this taps
 * the audio without playing it out the host speakers.
 */
function pumpGuestAudio(stream: MediaStream, sourceId: string, session: LiveSession): void {
  if (stream.getAudioTracks().length === 0) return;
  const AudioCtx: typeof AudioContext =
    window.AudioContext ??
    (window as unknown as { webkitAudioContext: typeof AudioContext }).webkitAudioContext;
  const ctx = new AudioCtx({ sampleRate: AUDIO_RATE });
  const source = ctx.createMediaStreamSource(stream);
  const processor = ctx.createScriptProcessor(4096, 2, 2);
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
  session.stops.push(() => {
    processor.disconnect();
    source.disconnect();
    void ctx.close();
  });
}
