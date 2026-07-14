import { useEffect, useState } from "react";

import {
  rundownAdvance,
  rundownStart,
  rundownStatus,
  rundownStop,
  settingsSet,
} from "../api/commands";
import type { RundownStatus, RundownStep, Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/** `95` → `1:35`. */
function mmss(seconds: number): string {
  const whole = Math.max(0, Math.floor(seconds));
  return `${Math.floor(whole / 60)}:${(whole % 60).toString().padStart(2, "0")}`;
}

/**
 * The show rundown (CAP-N09): an ordered playlist of steps — scene + hold —
 * with manual or automatic advance, and a live "next up + remaining time".
 *
 * Running a rundown switches scenes through the ordinary (undoable) command
 * path; the rundown never edits the scene collection itself. Auto-advance is
 * off by default: a show should never run away from its operator unasked.
 */
export function RundownDialog({
  settings,
  sceneNames,
  onSaved,
  onClose,
}: {
  settings: Settings | null;
  /** The collection's scene names — what a step cuts to. */
  sceneNames: string[];
  onSaved: (next: Settings) => void;
  onClose: () => void;
}) {
  const t = useT();
  const [error, setError] = useState<string | null>(null);
  const [status, setStatus] = useState<RundownStatus>({ running: false });

  useEffect(() => {
    let alive = true;
    const tick = () => {
      rundownStatus()
        .then((next) => alive && next && setStatus(next))
        .catch(() => undefined);
    };
    tick();
    const timer = window.setInterval(tick, 500);
    return () => {
      alive = false;
      window.clearInterval(timer);
    };
  }, []);

  if (!settings) return null;
  const rundown = settings.rundown ?? { steps: [], autoAdvance: false };

  const persist = (next: { steps: RundownStep[]; autoAdvance: boolean }) => {
    const nextSettings = { ...settings, rundown: next };
    setError(null);
    settingsSet(nextSettings)
      .then(() => onSaved(nextSettings))
      .catch((err) => setError(String(err)));
  };

  const updateStep = (index: number, patch: Partial<RundownStep>) =>
    persist({
      ...rundown,
      steps: rundown.steps.map((step, at) => (at === index ? { ...step, ...patch } : step)),
    });

  const move = (index: number, delta: number) => {
    const to = index + delta;
    if (to < 0 || to >= rundown.steps.length) return;
    const steps = [...rundown.steps];
    const [step] = steps.splice(index, 1);
    steps.splice(to, 0, step);
    persist({ ...rundown, steps });
  };

  const fail = (err: unknown) => setError(String(err));

  return (
    <PickerShell title={t("rundown-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-havoc-muted">{t("rundown-about")}</p>
        {error && <p className="m-0 text-red-400">{error}</p>}

        {/* Transport */}
        <div className="flex flex-wrap items-center gap-2 rounded-lg border border-white/10 p-2">
          <button
            type="button"
            disabled={rundown.steps.length === 0}
            onClick={() => rundownStart(0).catch(fail)}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 font-semibold disabled:opacity-50"
          >
            {t("rundown-start")}
          </button>
          <button
            type="button"
            disabled={!status.running}
            onClick={() => rundownAdvance().catch(fail)}
            className="rounded-md border border-white/10 px-3 py-1.5 disabled:opacity-50"
          >
            {t("rundown-next")}
          </button>
          <button
            type="button"
            disabled={!status.running}
            onClick={() => rundownStop().catch(fail)}
            className="rounded-md border border-white/10 px-3 py-1.5 disabled:opacity-50"
          >
            {t("rundown-stop")}
          </button>
          <span className="ml-auto flex items-center gap-3 tabular-nums">
            {status.running && status.remainingSecs != null && (
              <span className="font-mono text-emerald-300">{mmss(status.remainingSecs)}</span>
            )}
            <span className="text-havoc-muted">
              {status.running
                ? status.nextUp
                  ? t("rundown-next-up", { name: status.nextUp })
                  : t("rundown-last-step")
                : t("rundown-idle")}
            </span>
          </span>
        </div>

        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={rundown.autoAdvance}
            onChange={(event) => persist({ ...rundown, autoAdvance: event.target.checked })}
          />
          {t("rundown-auto-advance")}
        </label>

        {/* Steps */}
        {rundown.steps.length === 0 && <p className="m-0 text-havoc-muted">{t("rundown-empty")}</p>}
        <ul className="m-0 flex list-none flex-col gap-1.5 p-0">
          {rundown.steps.map((step, index) => (
            <li
              key={index}
              className={`flex flex-wrap items-center gap-2 rounded-lg border p-2 ${
                status.at === index && status.running
                  ? "border-havoc-accent/60 bg-havoc-accent/10"
                  : "border-white/10"
              }`}
            >
              <span className="w-5 text-center text-havoc-muted">{index + 1}</span>
              <input
                value={step.name}
                onChange={(event) => updateStep(index, { name: event.target.value })}
                placeholder={t("rundown-step-name")}
                aria-label={t("rundown-step-name")}
                className={`${inputClass} min-w-0 flex-1`}
              />
              <select
                value={step.scene}
                onChange={(event) => updateStep(index, { scene: event.target.value })}
                aria-label={t("rundown-step-scene")}
                className={inputClass}
              >
                <option value="">{t("rundown-stay")}</option>
                {sceneNames.map((name) => (
                  <option key={name} value={name}>
                    {name}
                  </option>
                ))}
              </select>
              <label className="flex items-center gap-1 text-havoc-muted">
                <input
                  type="number"
                  min={0}
                  max={7200}
                  value={step.holdSecs}
                  onChange={(event) =>
                    updateStep(index, { holdSecs: Math.max(0, Number(event.target.value) || 0) })
                  }
                  aria-label={t("rundown-hold")}
                  className={`${inputClass} w-20`}
                />
                {t("rundown-seconds")}
              </label>
              <button
                type="button"
                onClick={() => rundownStart(index).catch(fail)}
                title={t("rundown-jump")}
                aria-label={t("rundown-jump")}
                className="rounded px-1.5 text-havoc-muted hover:text-havoc-text"
              >
                ▶
              </button>
              <button
                type="button"
                disabled={index === 0}
                onClick={() => move(index, -1)}
                aria-label={t("rundown-move-up")}
                className="rounded px-1 text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
              >
                ▲
              </button>
              <button
                type="button"
                disabled={index === rundown.steps.length - 1}
                onClick={() => move(index, 1)}
                aria-label={t("rundown-move-down")}
                className="rounded px-1 text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
              >
                ▼
              </button>
              <button
                type="button"
                onClick={() =>
                  persist({ ...rundown, steps: rundown.steps.filter((_, at) => at !== index) })
                }
                aria-label={t("rundown-remove")}
                className="rounded px-1.5 text-havoc-muted hover:text-red-400"
              >
                ×
              </button>
            </li>
          ))}
        </ul>

        <button
          type="button"
          onClick={() =>
            persist({
              ...rundown,
              steps: [
                ...rundown.steps,
                { name: t("rundown-new-step"), scene: "", holdSecs: 0, actions: [] },
              ],
            })
          }
          className="self-start rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 font-semibold"
        >
          {t("rundown-add-step")}
        </button>
      </div>
    </PickerShell>
  );
}
