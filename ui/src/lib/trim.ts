// CAP-N41: the trim window's frame math — a tested TS mirror of the
// decisions `crates/encode/src/trim.rs` makes, so the dialog's honesty badge
// ("exports without re-encode") always matches what the export will do.

/**
 * Whether an in-point lands on a keyframe (within half a frame) — the
 * stream-copy condition. Mirrors `lands_on_keyframe` in trim.rs.
 */
export function landsOnKeyframe(inSecs: number, keyframesSecs: number[], fps: number): boolean {
  const tolerance = 0.5 / Math.max(fps, 1);
  for (const kf of keyframesSecs) {
    if (Math.abs(kf - inSecs) <= tolerance) return true;
    if (kf > inSecs + tolerance) break; // ascending — nothing closer follows
  }
  return false;
}

/** The nearest keyframe to `t`, or null when the list is empty. */
export function nearestKeyframe(t: number, keyframesSecs: number[]): number | null {
  let best: number | null = null;
  let bestDist = Infinity;
  for (const kf of keyframesSecs) {
    const dist = Math.abs(kf - t);
    if (dist < bestDist) {
      best = kf;
      bestDist = dist;
    } else if (kf > t) {
      break; // ascending — the distance only grows from here
    }
  }
  return best;
}

/** Step `t` by `frames` (negative = back), clamped to `[0, durationSecs]`. */
export function stepFrames(t: number, frames: number, fps: number, durationSecs: number): number {
  const step = 1 / Math.max(fps, 1);
  return Math.min(Math.max(t + frames * step, 0), Math.max(durationSecs, 0));
}

/** `HH:MM:SS.ff` (ff = frame number within the second) for display. */
export function formatTimecode(t: number, fps: number): string {
  const clamped = Math.max(t, 0);
  const whole = Math.floor(clamped);
  const hours = Math.floor(whole / 3600);
  const minutes = Math.floor((whole % 3600) / 60);
  const seconds = whole % 60;
  const frame = Math.min(
    Math.round((clamped - whole) * Math.max(fps, 1)),
    Math.max(Math.ceil(fps) - 1, 0),
  );
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${pad(hours)}:${pad(minutes)}:${pad(seconds)}.${pad(frame)}`;
}
