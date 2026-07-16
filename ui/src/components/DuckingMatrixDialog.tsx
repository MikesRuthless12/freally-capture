import { useState } from "react";

import {
  studioAddAudioFilter,
  studioRemoveAudioFilter,
  studioUpdateAudioFilter,
} from "../api/commands";
import type { Source, SourceId } from "../api/types";
import { DUCKER_DEFAULT, duckKind, findDuck, type Ducker } from "../lib/ducking";
import { PickerShell } from "./PickerShell";
import { useT } from "../i18n/t";

const fail = (what: string) => (err: unknown) => console.error(`${what} failed:`, err);

/**
 * CAP-N31 ducking matrix: an N×M grid of who ducks whom. A cell (trigger row →
 * target column) is one Ducker filter on the target strip keyed to the trigger
 * — so any source can duck any set, each pair with its own depth/attack/release.
 * The engine already stacks multiple Duckers per strip; this is the view that
 * makes the whole topology editable at once instead of buried per strip.
 */
export function DuckingMatrixDialog({
  strips,
  onClose,
}: {
  strips: Source[];
  onClose: () => void;
}) {
  const t = useT();
  const [selected, setSelected] = useState<{ target: SourceId; trigger: SourceId } | null>(null);

  const stripById = (id: SourceId) => strips.find((source) => source.id === id) ?? null;

  const addDuck = (target: Source, triggerId: SourceId) => {
    studioAddAudioFilter(target.id, { ...DUCKER_DEFAULT, trigger: triggerId }).catch(
      fail("add duck"),
    );
    setSelected({ target: target.id, trigger: triggerId });
  };

  const removeDuck = (target: Source, triggerId: SourceId) => {
    const duck = findDuck(target, triggerId);
    if (duck) studioRemoveAudioFilter(target.id, duck.id).catch(fail("remove duck"));
    setSelected(null);
  };

  const patchDuck = (target: Source, duck: Ducker, patch: Partial<Ducker>) => {
    studioUpdateAudioFilter(target.id, duck.id, duckKind({ ...duck, ...patch })).catch(
      fail("edit duck"),
    );
  };

  const selectedTarget = selected ? stripById(selected.target) : null;
  const selectedTrigger = selected ? stripById(selected.trigger) : null;
  const selectedDuck =
    selectedTarget && selected ? findDuck(selectedTarget, selected.trigger) : null;

  return (
    <PickerShell title={t("ducking-title")} onClose={onClose} wide>
      <div className="flex max-h-[70vh] flex-col gap-4 overflow-y-auto">
        <p className="text-xs text-havoc-muted">{t("ducking-intro")}</p>

        {strips.length < 2 ? (
          <p className="text-xs text-havoc-muted">{t("ducking-need-two")}</p>
        ) : (
          <div className="overflow-x-auto">
            <table className="border-collapse text-xs">
              <thead>
                <tr className="text-havoc-muted">
                  <th className="px-2 py-1 text-left font-medium">{t("ducking-trigger-target")}</th>
                  {strips.map((target) => (
                    <th
                      key={target.id}
                      className="max-w-24 truncate px-2 py-1 text-center font-medium"
                      title={target.name}
                    >
                      {target.name}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {strips.map((trigger) => (
                  <tr key={trigger.id} className="border-t border-white/10">
                    <td
                      className="max-w-32 truncate px-2 py-1 text-havoc-text"
                      title={trigger.name}
                    >
                      {trigger.name}
                    </td>
                    {strips.map((target) => {
                      if (target.id === trigger.id) {
                        return (
                          <td key={target.id} className="px-2 py-1 text-center text-havoc-muted/40">
                            —
                          </td>
                        );
                      }
                      const duck = findDuck(target, trigger.id);
                      const isSelected =
                        selected?.target === target.id && selected?.trigger === trigger.id;
                      return (
                        <td key={target.id} className="px-1 py-1 text-center">
                          <button
                            type="button"
                            onClick={() =>
                              duck
                                ? setSelected({ target: target.id, trigger: trigger.id })
                                : addDuck(target, trigger.id)
                            }
                            aria-label={t("ducking-cell-aria", {
                              trigger: trigger.name,
                              target: target.name,
                            })}
                            className={`h-6 w-10 rounded border text-[10px] tabular-nums transition-colors ${
                              duck
                                ? isSelected
                                  ? "border-havoc-accent bg-havoc-accent/30 text-havoc-text"
                                  : "border-havoc-accent/50 bg-havoc-accent/15 text-havoc-text"
                                : "border-white/10 text-havoc-muted hover:border-havoc-accent/50"
                            }`}
                          >
                            {duck ? `−${duck.amountDb.toFixed(0)}` : "+"}
                          </button>
                        </td>
                      );
                    })}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}

        {selectedTarget && selectedTrigger && selectedDuck && (
          <section className="rounded-md border border-white/10 bg-white/[0.02] p-3">
            <div className="mb-2 flex items-center justify-between gap-2">
              <span className="text-xs text-havoc-text">
                {t("ducking-pair", {
                  trigger: selectedTrigger.name,
                  target: selectedTarget.name,
                })}
              </span>
              <button
                type="button"
                onClick={() => removeDuck(selectedTarget, selectedTrigger.id)}
                className="rounded border border-white/10 px-2 py-0.5 text-[10px] text-havoc-muted transition-colors hover:border-red-500/60 hover:text-red-400"
              >
                {t("ducking-remove")}
              </button>
            </div>
            <div className="flex flex-col gap-1.5">
              <DuckSlider
                label={t("ducking-amount")}
                min={0}
                max={60}
                step={0.5}
                value={selectedDuck.amountDb}
                suffix={t("ducking-unit-db")}
                onChange={(v) => patchDuck(selectedTarget, selectedDuck, { amountDb: v })}
              />
              <DuckSlider
                label={t("ducking-threshold")}
                min={-96}
                max={0}
                step={1}
                value={selectedDuck.thresholdDb}
                suffix={t("ducking-unit-db")}
                onChange={(v) => patchDuck(selectedTarget, selectedDuck, { thresholdDb: v })}
              />
              <DuckSlider
                label={t("ducking-attack")}
                min={1}
                max={1000}
                step={1}
                value={selectedDuck.attackMs}
                suffix={t("ducking-unit-ms")}
                onChange={(v) => patchDuck(selectedTarget, selectedDuck, { attackMs: v })}
              />
              <DuckSlider
                label={t("ducking-release")}
                min={1}
                max={5000}
                step={10}
                value={selectedDuck.releaseMs}
                suffix={t("ducking-unit-ms")}
                onChange={(v) => patchDuck(selectedTarget, selectedDuck, { releaseMs: v })}
              />
            </div>
          </section>
        )}
      </div>
    </PickerShell>
  );
}

function DuckSlider({
  label,
  min,
  max,
  step,
  value,
  suffix,
  onChange,
}: {
  label: string;
  min: number;
  max: number;
  step: number;
  value: number;
  suffix: string;
  onChange: (value: number) => void;
}) {
  return (
    <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
      <span className="w-20 shrink-0">{label}</span>
      <input
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={(event) => onChange(Number(event.target.value))}
        className="flex-1"
      />
      <span className="w-16 text-right tabular-nums text-havoc-text">
        {value.toFixed(step < 1 ? 1 : 0)} {suffix}
      </span>
    </label>
  );
}
