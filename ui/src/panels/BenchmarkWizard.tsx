import { useEffect, useState } from "react";

import { benchmarkCancel, benchmarkStart, benchmarkStatus, settingsSet } from "../api/commands";
import { onBenchmark } from "../api/events";
import type { BenchProgress, Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const PRESET_KEY: Record<string, string> = {
  quality: "output-preset-quality",
  balanced: "output-preset-balanced",
  performance: "output-preset-performance",
};

/**
 * CAP-N52: the encoder benchmark wizard — actually RUNS short encode
 * ladders (encoder × preset × resolution × fps) on this machine, shows the
 * measured fps + headroom per rung (failures documented, never hidden),
 * and recommends settings from measurements. Distinct from the first-run
 * wizard, which only probes capabilities and applies heuristics.
 */
export function BenchmarkWizard({
  settings,
  onSaved,
  onClose,
}: {
  settings: Settings;
  onSaved: (next: Settings) => void;
  onClose: () => void;
}) {
  const t = useT();
  const [progress, setProgress] = useState<BenchProgress | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [applied, setApplied] = useState(false);

  useEffect(() => {
    let alive = true;
    let unlisten: (() => void) | undefined;
    benchmarkStatus()
      .then((current) => alive && setProgress(current))
      .catch(() => undefined);
    onBenchmark((next) => setProgress(next))
      .then((fn) => {
        if (alive) unlisten = fn;
        else fn();
      })
      .catch(() => undefined);
    return () => {
      alive = false;
      unlisten?.();
    };
  }, []);

  const start = async () => {
    setError(null);
    setApplied(false);
    try {
      await benchmarkStart();
    } catch (raw) {
      setError(String(raw));
    }
  };

  const running = progress?.running === true;
  const done = !running && (progress?.results.length ?? 0) > 0;
  const recommendation = progress?.recommendation;

  const apply = () => {
    if (!recommendation) return;
    const next: Settings = {
      ...settings,
      recording: {
        ...settings.recording,
        encoderId: recommendation.encoderId,
        preset: recommendation.preset,
        fps: recommendation.fps,
      },
    };
    settingsSet(next)
      .then(() => {
        setApplied(true);
        onSaved(next);
      })
      .catch((raw) => setError(String(raw)));
  };

  return (
    <PickerShell title={t("bench-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 p-3 text-xs text-havoc-muted">
        <p className="m-0 leading-snug">{t("bench-intro")}</p>

        {!running && (
          <button
            type="button"
            onClick={() => void start()}
            className="self-start rounded-md border border-havoc-accent/40 bg-havoc-accent/15 px-3 py-1.5 text-xs font-medium text-havoc-text hover:border-havoc-accent/70"
          >
            {done ? t("bench-rerun") : t("bench-start")}
          </button>
        )}
        {running && progress && (
          <div className="flex items-center gap-3">
            <span role="status">
              {t("bench-running", {
                done: progress.results.length,
                total: progress.total,
              })}
            </span>
            <button
              type="button"
              onClick={() => void benchmarkCancel()}
              className="rounded border border-white/10 px-2 py-0.5 text-[11px] hover:text-havoc-text"
            >
              {t("bench-cancel")}
            </button>
          </div>
        )}
        {error && (
          <p role="alert" className="m-0 text-[11px] leading-snug break-words text-red-300">
            {error}
          </p>
        )}

        {(progress?.results.length ?? 0) > 0 && (
          <div className="max-h-64 overflow-y-auto rounded-lg border border-white/10">
            <table className="w-full border-collapse text-[11px]">
              <thead className="sticky top-0 bg-black/60 text-havoc-muted">
                <tr>
                  <th className="px-2 py-1 text-left">{t("bench-col-encoder")}</th>
                  <th className="px-2 py-1 text-left">{t("bench-col-preset")}</th>
                  <th className="px-2 py-1 text-right">{t("bench-col-rung")}</th>
                  <th className="px-2 py-1 text-right">{t("bench-col-achieved")}</th>
                  <th className="px-2 py-1 text-right">{t("bench-col-headroom")}</th>
                </tr>
              </thead>
              <tbody>
                {progress?.results.map((result, index) => (
                  <tr key={index} className="border-t border-white/5">
                    <td className="px-2 py-1">{result.encoderLabel}</td>
                    <td className="px-2 py-1">{t(PRESET_KEY[result.preset] ?? result.preset)}</td>
                    <td className="px-2 py-1 text-right tabular-nums">
                      {result.width}×{result.height}@{result.fps}
                    </td>
                    {result.error != null ? (
                      <td
                        colSpan={2}
                        className="px-2 py-1 text-right text-red-300"
                        title={result.error}
                      >
                        {t("bench-failed")}
                      </td>
                    ) : (
                      <>
                        <td className="px-2 py-1 text-right tabular-nums">
                          {(result.achievedFps ?? 0).toFixed(0)}
                        </td>
                        <td
                          className={`px-2 py-1 text-right tabular-nums ${
                            (result.headroom ?? 0) >= 1.5
                              ? "text-emerald-300"
                              : (result.headroom ?? 0) >= 1.1
                                ? "text-amber-300"
                                : "text-red-300"
                          }`}
                        >
                          {(result.headroom ?? 0).toFixed(2)}×
                        </td>
                      </>
                    )}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}

        {done && recommendation && (
          <div className="flex flex-col gap-1.5 rounded-lg border border-emerald-500/30 bg-emerald-500/5 p-2">
            <span className="text-[11px] font-semibold text-emerald-300">
              {t("bench-rec-title")}
            </span>
            <p className="m-0 leading-snug">
              {t("bench-rec-body", {
                encoder: recommendation.encoderLabel,
                preset: t(PRESET_KEY[recommendation.preset] ?? recommendation.preset),
                width: recommendation.width,
                height: recommendation.height,
                fps: recommendation.fps,
                bitrate: recommendation.bitrateKbps,
                headroom: recommendation.headroom.toFixed(1),
              })}
            </p>
            <button
              type="button"
              onClick={apply}
              disabled={applied}
              className="self-start rounded-md border border-emerald-500/40 bg-emerald-500/10 px-3 py-1.5 text-xs font-medium text-havoc-text enabled:hover:border-emerald-400/70 disabled:opacity-60"
            >
              {applied ? t("bench-applied") : t("bench-apply")}
            </button>
          </div>
        )}
        {done && !recommendation && (
          <p className="m-0 rounded-lg border border-amber-400/30 bg-amber-400/10 p-2 leading-snug text-amber-200">
            {t("bench-rec-none")}
          </p>
        )}
      </div>
    </PickerShell>
  );
}
