/**
 * Remote Guests — audio device choices: which microphone this machine sends
 * into the session, and which output device plays the other side's audio.
 *
 * Enumeration is permission-aware: until the user grants mic access the
 * browser hides device labels, so options fall back to honest generated names
 * ("Microphone 2") and refresh to real labels once a session unlocks them.
 */

export type AudioDeviceOption = { deviceId: string; label: string };

export type RemoteAudioDevices = {
  inputs: AudioDeviceOption[];
  outputs: AudioDeviceOption[];
};

/** An option with a fallback name for label-less (pre-permission) devices. */
export function deviceOption(
  kind: "audioinput" | "audiooutput",
  device: { deviceId: string; label: string },
  index: number,
): AudioDeviceOption {
  const fallback = kind === "audioinput" ? `Microphone ${index + 1}` : `Output ${index + 1}`;
  return { deviceId: device.deviceId, label: device.label || fallback };
}

/** getUserMedia audio constraints for a chosen mic — echo-safe defaults, and
 * `ideal` (not `exact`) so an unplugged device degrades instead of failing. */
export function micConstraints(deviceId: string | null): MediaTrackConstraints {
  const base: MediaTrackConstraints = {
    echoCancellation: true,
    noiseSuppression: true,
    autoGainControl: true,
  };
  return deviceId ? { ...base, deviceId: { ideal: deviceId } } : base;
}

/**
 * List the selectable mics + outputs with their REAL device names.
 *
 * The browser hides device labels until a media permission is granted (a
 * privacy rule), so a fresh enumeration returns blank labels and we'd fall
 * back to "Microphone 2". When labels are hidden this does a one-shot mic
 * probe (a stream requested then immediately released) to unlock them for the
 * session, then re-enumerates. If the probe is denied, the honest generated
 * names remain. Empty in environments without media.
 */
export async function listRemoteAudioDevices(): Promise<RemoteAudioDevices> {
  if (!navigator.mediaDevices?.enumerateDevices) return { inputs: [], outputs: [] };
  let all = await navigator.mediaDevices.enumerateDevices();
  const audio = all.filter(
    (device) => device.kind === "audioinput" || device.kind === "audiooutput",
  );
  const labelsHidden = audio.length > 0 && audio.some((device) => !device.label);
  if (labelsHidden && navigator.mediaDevices.getUserMedia) {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      stream.getTracks().forEach((track) => track.stop());
      all = await navigator.mediaDevices.enumerateDevices();
    } catch {
      // Denied / unavailable — keep the generated fallback names.
    }
  }
  return {
    inputs: all
      .filter((device) => device.kind === "audioinput")
      .map((device, index) => deviceOption("audioinput", device, index)),
    outputs: all
      .filter((device) => device.kind === "audiooutput")
      .map((device, index) => deviceOption("audiooutput", device, index)),
  };
}

/** Re-run enumeration when devices are (un)plugged; returns an unlisten. */
export function onDeviceChange(callback: () => void): () => void {
  const target = navigator.mediaDevices;
  if (!target?.addEventListener) return () => {};
  target.addEventListener("devicechange", callback);
  return () => target.removeEventListener("devicechange", callback);
}

/** setSinkId is not in every lib.dom/webview — the narrow slice we need. */
type Sinkable = HTMLAudioElement & { setSinkId?: (id: string) => Promise<void> };

export type SinkResult = "ok" | "unsupported" | "failed";

/** Route an element's playback to an output device ("" / null = default). */
export async function routeToOutput(
  element: HTMLAudioElement,
  deviceId: string | null,
): Promise<SinkResult> {
  const sinkable = element as Sinkable;
  if (!sinkable.setSinkId) return deviceId ? "unsupported" : "ok";
  try {
    await sinkable.setSinkId(deviceId ?? "");
    return "ok";
  } catch {
    return "failed";
  }
}

/** A mic self-test: capture the chosen mic and play it straight back through
 * the chosen output, so you can talk and hear yourself. Returns a stop(). */
export async function startMicTest(
  micId: string | null,
  speakerId: string | null,
): Promise<{ stop: () => void; sink: SinkResult }> {
  const stream = await navigator.mediaDevices.getUserMedia({ audio: micConstraints(micId) });
  const element = document.createElement("audio");
  element.autoplay = true;
  element.srcObject = stream;
  const sink = await routeToOutput(element, speakerId);
  void element.play().catch(() => {});
  return {
    stop: () => {
      stream.getTracks().forEach((track) => track.stop());
      element.srcObject = null;
    },
    sink,
  };
}
