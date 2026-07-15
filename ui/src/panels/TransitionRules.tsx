import { useState } from "react";

import { studioTransitionOverrideRemove, studioTransitionOverrideSet } from "../api/commands";
import type { Collection, SceneId, TransitionKind } from "../api/types";
import { TRANSITION_KINDS } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

/**
 * Transition rules (CAP-N21): a per-scene-pair transition matrix. Scene A→B
 * uses a chosen kind and duration instead of the default transition (the
 * stinger/luma FILE still comes from the global transition settings). Every
 * change is a tracked command; the studio event re-feeds this dialog.
 */
export function TransitionRulesDialog({
  collection,
  onClose,
}: {
  collection: Collection | null;
  onClose: () => void;
}) {
  const t = useT();
  const scenes = collection?.scenes ?? [];
  const rules = collection?.transitionOverrides ?? [];
  const nameOf = (id: SceneId) => scenes.find((scene) => scene.id === id)?.name ?? id;
  const warn = (err: unknown) => console.error("transition rule failed:", err);

  const [from, setFrom] = useState<SceneId>("");
  const [to, setTo] = useState<SceneId>("");
  const [kind, setKind] = useState<TransitionKind>("fade");
  const [durationMs, setDurationMs] = useState(300);

  const canAdd = from !== "" && to !== "" && from !== to;
  const add = () => {
    if (!canAdd) return;
    studioTransitionOverrideSet(from, to, kind, durationMs).catch(warn);
  };

  return (
    <PickerShell title={t("transition-rules-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-2 text-xs text-havoc-text">
        <p className="m-0 text-[11px] leading-snug text-havoc-muted">{t("transition-rules-hint")}</p>

        {rules.length === 0 ? (
          <p className="m-0 text-havoc-muted">{t("transition-rules-empty")}</p>
        ) : (
          <ul className="m-0 flex list-none flex-col gap-1.5 p-0">
            {rules.map((rule) => (
              <li
                key={`${rule.from}->${rule.to}`}
                className="flex items-center gap-2 rounded-md border border-white/10 p-2"
              >
                <span className="flex-1 truncate">
                  {nameOf(rule.from)} <span className="text-havoc-muted">→</span> {nameOf(rule.to)}
                </span>
                <span className="truncate text-havoc-muted">
                  {t(
                    TRANSITION_KINDS.find(([value]) => value === rule.kind)?.[1] ??
                      "transition-kind-fade",
                  )}{" "}
                  · {rule.durationMs} ms
                </span>
                <button
                  type="button"
                  onClick={() =>
                    studioTransitionOverrideRemove(rule.from, rule.to).catch(warn)
                  }
                  title={t("transition-rules-remove")}
                  className="rounded px-1 text-havoc-muted hover:text-red-400"
                >
                  ×
                </button>
              </li>
            ))}
          </ul>
        )}

        {/* Add / replace a rule. */}
        <div className="flex flex-wrap items-end gap-2 rounded-md border border-white/10 p-2">
          <label className="flex flex-col gap-0.5 text-[11px] text-havoc-muted">
            {t("transition-rules-from")}
            <select
              value={from}
              onChange={(event) => setFrom(event.target.value)}
              className="rounded border border-white/10 bg-havoc-panel px-1.5 py-1 text-[11px] text-havoc-text"
            >
              <option value="">—</option>
              {scenes.map((scene) => (
                <option key={scene.id} value={scene.id}>
                  {scene.name}
                </option>
              ))}
            </select>
          </label>
          <label className="flex flex-col gap-0.5 text-[11px] text-havoc-muted">
            {t("transition-rules-to")}
            <select
              value={to}
              onChange={(event) => setTo(event.target.value)}
              className="rounded border border-white/10 bg-havoc-panel px-1.5 py-1 text-[11px] text-havoc-text"
            >
              <option value="">—</option>
              {scenes.map((scene) => (
                <option key={scene.id} value={scene.id}>
                  {scene.name}
                </option>
              ))}
            </select>
          </label>
          <label className="flex flex-col gap-0.5 text-[11px] text-havoc-muted">
            {t("transition-rules-kind")}
            <select
              value={kind}
              onChange={(event) => setKind(event.target.value as TransitionKind)}
              className="rounded border border-white/10 bg-havoc-panel px-1.5 py-1 text-[11px] text-havoc-text"
            >
              {TRANSITION_KINDS.filter(([value]) => value !== "cut").map(([value, key]) => (
                <option key={value} value={value}>
                  {t(key)}
                </option>
              ))}
            </select>
          </label>
          <label className="flex flex-col gap-0.5 text-[11px] text-havoc-muted">
            {t("transition-rules-duration")}
            <input
              type="number"
              min={50}
              max={5000}
              step={50}
              value={durationMs}
              onChange={(event) => setDurationMs(Number(event.target.value))}
              className="w-20 rounded border border-white/10 bg-havoc-panel px-1.5 py-1 text-[11px] text-havoc-text"
            />
          </label>
          <button
            type="button"
            onClick={add}
            disabled={!canAdd}
            className="rounded-md border border-white/10 px-2 py-1.5 text-xs text-havoc-text hover:border-havoc-accent/50 disabled:opacity-40"
          >
            {t("transition-rules-add")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}
