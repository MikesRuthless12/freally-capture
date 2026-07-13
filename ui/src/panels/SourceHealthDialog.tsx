import { studioRetrySource } from "../api/commands";
import type { ProgramStatus, SourceId, SourceRuntime, StudioDto } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

/**
 * The source health dashboard (CAP-M13): one row per collection source with
 * its live pipeline state — fps, last-frame age, dropped frames, restart
 * history — and per-source actions (restart capture, open properties). Pure
 * presentation over the `program` event; sources the engine isn't running
 * (not in the active scene, or audio-only) honestly read "inactive".
 */
export function SourceHealthDialog({
  studio,
  program,
  onOpenProperties,
  onClose,
}: {
  studio: StudioDto | null;
  program: ProgramStatus | null;
  onOpenProperties: (source: SourceId) => void;
  onClose: () => void;
}) {
  const t = useT();
  const sources = studio?.collection.sources ?? [];

  const restart = (id: SourceId) => {
    studioRetrySource(id).catch((err) => console.error("source restart failed:", err));
  };

  const stateChip = (state: string) => {
    const palette: Record<string, string> = {
      live: "bg-emerald-500/15 text-emerald-300 border-emerald-500/40",
      waiting: "bg-amber-500/15 text-amber-300 border-amber-500/40",
      error: "bg-red-500/15 text-red-300 border-red-500/40",
      inactive: "bg-transparent text-havoc-muted border-white/10",
    };
    return (
      <span
        className={`inline-block rounded border px-1.5 py-0.5 text-[10px] font-semibold ${palette[state] ?? palette.inactive}`}
      >
        {t(`health-state-${state}`)}
      </span>
    );
  };

  const age = (runtime: SourceRuntime | undefined) => {
    if (runtime?.lastFrameMs == null) return "—";
    return t("health-seconds", { value: (runtime.lastFrameMs / 1000).toFixed(1) });
  };

  return (
    <PickerShell title={t("health-title")} onClose={onClose} wide>
      {sources.length === 0 ? (
        <p className="m-0 text-xs text-havoc-muted">{t("health-empty")}</p>
      ) : (
        <div className="overflow-x-auto">
          <table className="w-full border-collapse text-left text-[11px] text-havoc-text">
            <thead>
              <tr className="text-[10px] uppercase tracking-wide text-havoc-muted">
                <th className="px-2 py-1 font-medium">{t("health-col-source")}</th>
                <th className="px-2 py-1 font-medium">{t("health-col-state")}</th>
                <th className="px-2 py-1 font-medium">{t("health-col-resolution")}</th>
                <th className="px-2 py-1 font-medium">{t("health-col-fps")}</th>
                <th className="px-2 py-1 font-medium">{t("health-col-last-frame")}</th>
                <th className="px-2 py-1 font-medium">{t("health-col-dropped")}</th>
                <th className="px-2 py-1 font-medium">{t("health-col-retries")}</th>
                <th className="px-2 py-1 font-medium">{t("health-col-actions")}</th>
              </tr>
            </thead>
            <tbody>
              {sources.map((source) => {
                const runtime = program?.sources?.[source.id];
                const state = runtime?.state ?? "inactive";
                return (
                  <tr key={source.id} className="border-t border-white/5">
                    <td className="max-w-48 px-2 py-1.5">
                      <span className="block truncate">{source.name}</span>
                      {runtime?.errorMessage && (
                        <span className="block truncate text-[10px] text-red-300">
                          {runtime.errorMessage}
                        </span>
                      )}
                    </td>
                    <td className="px-2 py-1.5">{stateChip(state)}</td>
                    <td className="px-2 py-1.5 tabular-nums">
                      {runtime?.width != null && runtime?.height != null
                        ? `${runtime.width}×${runtime.height}`
                        : "—"}
                    </td>
                    <td className="px-2 py-1.5 tabular-nums">{runtime?.fps ?? "—"}</td>
                    <td className="px-2 py-1.5 tabular-nums">{age(runtime)}</td>
                    <td className="px-2 py-1.5 tabular-nums">{runtime?.dropped ?? "—"}</td>
                    <td className="px-2 py-1.5 tabular-nums">{runtime?.retries ?? 0}</td>
                    <td className="px-2 py-1.5">
                      <div className="flex gap-1">
                        <button
                          type="button"
                          onClick={() => restart(source.id)}
                          disabled={state === "inactive"}
                          aria-label={t("sources-retry-item", { name: source.name })}
                          className="rounded border border-white/10 px-1.5 py-0.5 text-[10px] transition-colors hover:border-havoc-accent/60 disabled:cursor-not-allowed disabled:opacity-40"
                        >
                          {t("health-restart")}
                        </button>
                        <button
                          type="button"
                          onClick={() => onOpenProperties(source.id)}
                          aria-label={t("sources-properties-item", { name: source.name })}
                          className="rounded border border-white/10 px-1.5 py-0.5 text-[10px] transition-colors hover:border-havoc-accent/60"
                        >
                          {t("health-properties")}
                        </button>
                      </div>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}
    </PickerShell>
  );
}
