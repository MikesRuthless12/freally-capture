import { useT } from "../i18n/t";
import { formatHms } from "../lib/time";

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
  const t = useT();
  return (
    <span className="inline-flex items-center gap-1.5" role="status">
      <span
        aria-label={paused ? t("recdot-paused-aria") : t("recdot-recording-aria")}
        className={`inline-block h-2 w-2 rounded-full ${
          paused ? "bg-amber-400" : "animate-pulse bg-red-500"
        }`}
      />
      <span className="font-mono text-xs tabular-nums">{formatHms(durationSec)}</span>
      <span
        title={
          tracks === 1
            ? t("recdot-tracks-one", { count: tracks })
            : t("recdot-tracks-other", { count: tracks })
        }
        className="rounded bg-white/10 px-1 text-[10px] text-havoc-muted"
      >
        {tracks}⏵
      </span>
      {paused && (
        <span className="text-[10px] tracking-wide text-amber-400 uppercase">
          {t("recdot-paused")}
        </span>
      )}
    </span>
  );
}
