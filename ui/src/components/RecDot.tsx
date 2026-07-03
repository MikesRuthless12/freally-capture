/** mm:ss (or h:mm:ss past an hour) for the REC readout. */
function formatDuration(totalSeconds: number): string {
  const whole = Math.max(0, Math.floor(totalSeconds));
  const hours = Math.floor(whole / 3600);
  const minutes = Math.floor((whole % 3600) / 60);
  const seconds = whole % 60;
  const mmss = `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
  return hours > 0 ? `${hours}:${mmss}` : mmss;
}

/**
 * The REC indicator: a pulsing red dot while recording, steady amber while
 * paused, with the recorded duration (pauses excluded) and the track count.
 */
export function RecDot({
  paused,
  durationSec,
  tracks,
}: {
  paused: boolean;
  durationSec: number;
  tracks: number;
}) {
  return (
    <span className="inline-flex items-center gap-1.5" role="status">
      <span
        aria-label={paused ? "Recording paused" : "Recording"}
        className={`inline-block h-2 w-2 rounded-full ${
          paused ? "bg-amber-400" : "animate-pulse bg-red-500"
        }`}
      />
      <span className="font-mono text-xs tabular-nums">{formatDuration(durationSec)}</span>
      <span
        title={`${tracks} audio track${tracks === 1 ? "" : "s"} recording`}
        className="rounded bg-white/10 px-1 text-[10px] text-havoc-muted"
      >
        {tracks}⏵
      </span>
      {paused && <span className="text-[10px] tracking-wide text-amber-400 uppercase">paused</span>}
    </span>
  );
}
