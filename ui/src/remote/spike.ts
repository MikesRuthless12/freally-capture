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
import Peer from "peerjs";
import { invoke } from "@tauri-apps/api/core";

import { studioAddItem } from "../api/commands";
import type { SceneId } from "../api/types";

/** Downscale bound for the spike's IPC frames (keeps a push ≤ ~0.9 MB). */
const MAX_FRAME_WIDTH = 640;

export type SpikeStatus = (message: string) => void;

/** rVFC is not in every lib.dom yet — the narrow slice the pump needs. */
type VideoWithRvfc = HTMLVideoElement & {
  requestVideoFrameCallback: (callback: () => void) => number;
};

type LiveSession = { peer: Peer; stops: Array<() => void> };
let live: LiveSession | null = null;

/** Tear down whichever spike session (host or guest) is running. */
export function spikeStop(): void {
  live?.stops.forEach((stop) => stop());
  live?.peer.destroy();
  live = null;
}

/** Host: wait for a guest call; when media arrives, add a RemoteGuest source
 * to the active scene and pump its frames into the compositor. */
export function spikeHost(
  sceneId: SceneId,
  onStatus: SpikeStatus,
  onPeerId: (id: string) => void,
): void {
  spikeStop();
  const peer = new Peer();
  const session: LiveSession = { peer, stops: [] };
  live = session;
  onStatus("connecting to the signaling broker…");
  peer.on("open", (id) => {
    onPeerId(id);
    onStatus("hosting — share the session id; waiting for a guest…");
  });
  peer.on("error", (err) => onStatus(`peer error: ${err.type}`));
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

/** Guest: share the webcam and call the host's session id. */
export async function spikeJoin(hostId: string, onStatus: SpikeStatus): Promise<void> {
  spikeStop();
  onStatus("requesting the webcam…");
  const stream = await navigator.mediaDevices.getUserMedia({ video: true, audio: false });
  const peer = new Peer();
  const session: LiveSession = {
    peer,
    stops: [() => stream.getTracks().forEach((track) => track.stop())],
  };
  live = session;
  peer.on("open", () => {
    onStatus("calling the host…");
    const call = peer.call(hostId.trim(), stream);
    call.on("close", () => onStatus("call closed"));
    onStatus("sharing the webcam with the host — keep this app open.");
  });
  peer.on("error", (err) => onStatus(`peer error: ${err.type}`));
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
