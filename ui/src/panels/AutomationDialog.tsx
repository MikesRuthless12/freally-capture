import { useEffect, useState } from "react";

import {
  automationRunMacro,
  automationSetVariable,
  automationVariables,
  settingsSet,
} from "../api/commands";
import type {
  AutomationMacro,
  AutomationRule,
  AutomationTrigger,
  MacroStep,
  Settings,
} from "../api/types";
import { ALLOWED_COMMANDS } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/** The trigger kinds, in UI order, with their i18n keys. */
const TRIGGERS: Array<[AutomationTrigger["kind"], string]> = [
  ["sceneSwitched", "automation-trigger-scene"],
  ["streamState", "automation-trigger-stream"],
  ["recordingState", "automation-trigger-recording"],
  ["sourceError", "automation-trigger-source-error"],
  ["audioLevel", "automation-trigger-audio"],
  ["systemIdle", "automation-trigger-idle"],
  ["windowFocus", "automation-trigger-focus"],
  ["timeOfDay", "automation-trigger-time"],
  ["fileChanged", "automation-trigger-file"],
];

/** A fresh trigger of the chosen kind (with sane, honest defaults). */
function newTrigger(kind: AutomationTrigger["kind"]): AutomationTrigger {
  switch (kind) {
    case "sceneSwitched":
      return { kind, scene: "" };
    case "streamState":
      return { kind, live: true };
    case "recordingState":
      return { kind, recording: true };
    case "sourceError":
      return { kind, source: "" };
    case "audioLevel":
      return { kind, source: "", thresholdDb: -20, above: true };
    case "systemIdle":
      return { kind, seconds: 300 };
    case "windowFocus":
      return { kind, exe: "" };
    case "timeOfDay":
      return { kind, at: "20:00" };
    case "fileChanged":
      return { kind, path: "" };
  }
}

/**
 * Automation (CAP-N01 + CAP-N02): rules (trigger → actions) and macros
 * (named step sequences), plus the live studio variables.
 *
 * Actions are limited to the same fixed command allowlist the remote API
 * exposes — a rule can never name a file, run a process, or reach the
 * network. Every rule ships **disabled**; nothing runs until it is switched
 * on here.
 */
export function AutomationDialog({
  settings,
  onSaved,
  onClose,
}: {
  settings: Settings | null;
  onSaved: (next: Settings) => void;
  onClose: () => void;
}) {
  const t = useT();
  const [error, setError] = useState<string | null>(null);
  const [variables, setVariables] = useState<Record<string, string>>({});
  const [newVar, setNewVar] = useState({ name: "", value: "" });

  useEffect(() => {
    let alive = true;
    const load = () => {
      automationVariables()
        .then((vars) => alive && vars && setVariables(vars))
        .catch(() => undefined);
    };
    load();
    const timer = window.setInterval(load, 1000);
    return () => {
      alive = false;
      window.clearInterval(timer);
    };
  }, []);

  if (!settings) return null;
  const automation = settings.automation ?? { rules: [], macros: [] };

  const persist = (next: { rules: AutomationRule[]; macros: AutomationMacro[] }) => {
    const nextSettings = { ...settings, automation: next };
    setError(null);
    settingsSet(nextSettings)
      .then(() => onSaved(nextSettings))
      .catch((err) => setError(String(err)));
  };

  const addRule = () =>
    persist({
      ...automation,
      rules: [
        ...automation.rules,
        {
          name: t("automation-new-rule"),
          enabled: false,
          trigger: newTrigger("sceneSwitched"),
          conditions: [],
          actions: [],
          macroName: "",
        },
      ],
    });

  const updateRule = (index: number, patch: Partial<AutomationRule>) =>
    persist({
      ...automation,
      rules: automation.rules.map((rule, at) => (at === index ? { ...rule, ...patch } : rule)),
    });

  const removeRule = (index: number) =>
    persist({ ...automation, rules: automation.rules.filter((_, at) => at !== index) });

  const addMacro = () =>
    persist({
      ...automation,
      macros: [...automation.macros, { name: t("automation-new-macro"), steps: [], repeat: 1 }],
    });

  const updateMacro = (index: number, patch: Partial<AutomationMacro>) =>
    persist({
      ...automation,
      macros: automation.macros.map((entry, at) => (at === index ? { ...entry, ...patch } : entry)),
    });

  const removeMacro = (index: number) =>
    persist({ ...automation, macros: automation.macros.filter((_, at) => at !== index) });

  const addStep = (index: number, step: MacroStep) => {
    const entry = automation.macros[index];
    updateMacro(index, { steps: [...entry.steps, step] });
  };

  return (
    <PickerShell title={t("automation-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-4 text-xs text-havoc-text">
        <p className="m-0 text-havoc-muted">{t("automation-about")}</p>
        {error && <p className="m-0 text-red-400">{error}</p>}

        {/* Rules */}
        <section className="flex flex-col gap-2">
          <div className="flex items-center justify-between">
            <span className="text-[11px] font-semibold tracking-wider text-havoc-muted uppercase">
              {t("automation-rules")}
            </span>
            <button
              type="button"
              onClick={addRule}
              className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2 py-1 font-semibold"
            >
              {t("automation-add-rule")}
            </button>
          </div>
          {automation.rules.length === 0 && (
            <p className="m-0 text-havoc-muted">{t("automation-no-rules")}</p>
          )}
          {automation.rules.map((rule, index) => (
            <div key={index} className="flex flex-col gap-2 rounded-lg border border-white/10 p-2">
              <div className="flex flex-wrap items-center gap-2">
                <label className="flex items-center gap-1">
                  <input
                    type="checkbox"
                    checked={rule.enabled}
                    onChange={(event) => updateRule(index, { enabled: event.target.checked })}
                  />
                  {t("automation-enabled")}
                </label>
                <input
                  value={rule.name}
                  onChange={(event) => updateRule(index, { name: event.target.value })}
                  aria-label={t("automation-rule-name")}
                  className={`${inputClass} min-w-0 flex-1`}
                />
                <button
                  type="button"
                  onClick={() => removeRule(index)}
                  aria-label={t("automation-remove")}
                  className="rounded px-1.5 text-havoc-muted hover:text-red-400"
                >
                  ×
                </button>
              </div>
              <div className="flex flex-wrap items-center gap-2">
                <span className="text-havoc-muted">{t("automation-when")}</span>
                <select
                  value={rule.trigger.kind}
                  onChange={(event) =>
                    updateRule(index, {
                      trigger: newTrigger(event.target.value as AutomationTrigger["kind"]),
                    })
                  }
                  className={inputClass}
                >
                  {TRIGGERS.map(([kind, key]) => (
                    <option key={kind} value={kind}>
                      {t(key)}
                    </option>
                  ))}
                </select>
                <TriggerFields
                  trigger={rule.trigger}
                  onChange={(trigger) => updateRule(index, { trigger })}
                />
              </div>
              <div className="flex flex-wrap items-center gap-2">
                <span className="text-havoc-muted">{t("automation-then-run")}</span>
                <select
                  value={rule.macroName}
                  onChange={(event) => updateRule(index, { macroName: event.target.value })}
                  className={inputClass}
                >
                  <option value="">{t("automation-no-macro")}</option>
                  {automation.macros.map((entry) => (
                    <option key={entry.name} value={entry.name}>
                      {entry.name}
                    </option>
                  ))}
                </select>
              </div>
            </div>
          ))}
        </section>

        {/* Macros */}
        <section className="flex flex-col gap-2 border-t border-white/5 pt-3">
          <div className="flex items-center justify-between">
            <span className="text-[11px] font-semibold tracking-wider text-havoc-muted uppercase">
              {t("automation-macros")}
            </span>
            <button
              type="button"
              onClick={addMacro}
              className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2 py-1 font-semibold"
            >
              {t("automation-add-macro")}
            </button>
          </div>
          {automation.macros.length === 0 && (
            <p className="m-0 text-havoc-muted">{t("automation-no-macros")}</p>
          )}
          {automation.macros.map((entry, index) => (
            <div key={index} className="flex flex-col gap-2 rounded-lg border border-white/10 p-2">
              <div className="flex flex-wrap items-center gap-2">
                <input
                  value={entry.name}
                  onChange={(event) => updateMacro(index, { name: event.target.value })}
                  aria-label={t("automation-macro-name")}
                  className={`${inputClass} min-w-0 flex-1`}
                />
                <input
                  value={entry.hotkey ?? ""}
                  onChange={(event) =>
                    updateMacro(index, { hotkey: event.target.value.trim() || undefined })
                  }
                  placeholder={t("automation-hotkey-placeholder")}
                  title={t("automation-chord-hint")}
                  aria-label={t("automation-hotkey")}
                  className={`${inputClass} w-36`}
                />
                <label className="flex items-center gap-1 text-havoc-muted">
                  {t("automation-layer")}
                  <input
                    type="number"
                    min={0}
                    max={9}
                    value={entry.layer ?? ""}
                    placeholder="—"
                    onChange={(event) => {
                      const raw = event.target.value.trim();
                      updateMacro(index, {
                        layer: raw === "" ? undefined : Math.min(9, Math.max(0, Number(raw) || 0)),
                      });
                    }}
                    title={t("automation-layer-hint")}
                    aria-label={t("automation-layer")}
                    className={`${inputClass} w-14`}
                  />
                </label>
                <button
                  type="button"
                  onClick={() =>
                    automationRunMacro(entry.name).catch((err) => setError(String(err)))
                  }
                  className="rounded-md border border-white/10 px-2 py-1 hover:border-havoc-accent/50"
                >
                  {t("automation-run")}
                </button>
                <button
                  type="button"
                  onClick={() => removeMacro(index)}
                  aria-label={t("automation-remove")}
                  className="rounded px-1.5 text-havoc-muted hover:text-red-400"
                >
                  ×
                </button>
              </div>
              <ul className="m-0 flex list-none flex-col gap-1 p-0">
                {entry.steps.map((step, stepAt) => (
                  <li key={stepAt} className="flex items-center gap-2 text-havoc-muted">
                    <span className="flex-1 truncate">
                      {step.kind === "action"
                        ? `▶ ${step.command}`
                        : step.kind === "wait"
                          ? `⏱ ${step.ms} ms`
                          : `= ${step.name} → ${step.value}`}
                    </span>
                    <button
                      type="button"
                      onClick={() =>
                        updateMacro(index, {
                          steps: entry.steps.filter((_, at) => at !== stepAt),
                        })
                      }
                      aria-label={t("automation-remove")}
                      className="rounded px-1 hover:text-red-400"
                    >
                      ×
                    </button>
                  </li>
                ))}
              </ul>
              <div className="flex flex-wrap items-center gap-2">
                <select
                  defaultValue=""
                  onChange={(event) => {
                    const command = event.target.value;
                    if (command) addStep(index, { kind: "action", command, params: {} });
                    event.target.value = "";
                  }}
                  aria-label={t("automation-add-action")}
                  className={inputClass}
                >
                  <option value="">{t("automation-add-action")}</option>
                  {ALLOWED_COMMANDS.map((command) => (
                    <option key={command} value={command}>
                      {command}
                    </option>
                  ))}
                </select>
                <button
                  type="button"
                  onClick={() => addStep(index, { kind: "wait", ms: 1000 })}
                  className="rounded-md border border-white/10 px-2 py-1 hover:border-havoc-accent/50"
                >
                  {t("automation-add-wait")}
                </button>
                <label className="flex items-center gap-1 text-havoc-muted">
                  {t("automation-repeat")}
                  <input
                    type="number"
                    min={1}
                    max={100}
                    value={entry.repeat}
                    onChange={(event) =>
                      updateMacro(index, { repeat: Math.max(1, Number(event.target.value) || 1) })
                    }
                    className={`${inputClass} w-16`}
                  />
                </label>
              </div>
            </div>
          ))}
        </section>

        {/* Variables */}
        <section className="flex flex-col gap-2 border-t border-white/5 pt-3">
          <span className="text-[11px] font-semibold tracking-wider text-havoc-muted uppercase">
            {t("automation-variables")}
          </span>
          <p className="m-0 text-havoc-muted">{t("automation-variables-about")}</p>
          <ul className="m-0 flex list-none flex-col gap-1 p-0">
            {Object.entries(variables).map(([name, value]) => (
              <li key={name} className="flex items-center gap-2">
                <span className="font-mono text-havoc-muted">{`{{${name}}}`}</span>
                <span className="flex-1 truncate">{value}</span>
              </li>
            ))}
          </ul>
          <div className="flex flex-wrap items-center gap-2">
            <input
              value={newVar.name}
              onChange={(event) => setNewVar({ ...newVar, name: event.target.value })}
              placeholder={t("automation-var-name")}
              aria-label={t("automation-var-name")}
              className={`${inputClass} w-32`}
            />
            <input
              value={newVar.value}
              onChange={(event) => setNewVar({ ...newVar, value: event.target.value })}
              placeholder={t("automation-var-value")}
              aria-label={t("automation-var-value")}
              className={`${inputClass} min-w-0 flex-1`}
            />
            <button
              type="button"
              disabled={!newVar.name.trim()}
              onClick={() => {
                automationSetVariable(newVar.name.trim(), newVar.value)
                  .then(() => setNewVar({ name: "", value: "" }))
                  .catch((err) => setError(String(err)));
              }}
              className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2 py-1 font-semibold disabled:opacity-50"
            >
              {t("automation-set-var")}
            </button>
          </div>
        </section>
      </div>
    </PickerShell>
  );
}

/** The per-kind fields a trigger needs (kept flat and boring on purpose). */
function TriggerFields({
  trigger,
  onChange,
}: {
  trigger: AutomationTrigger;
  onChange: (next: AutomationTrigger) => void;
}) {
  const t = useT();
  switch (trigger.kind) {
    case "sceneSwitched":
      return (
        <input
          value={trigger.scene}
          onChange={(event) => onChange({ ...trigger, scene: event.target.value })}
          placeholder={t("automation-scene-name")}
          aria-label={t("automation-scene-name")}
          className={inputClass}
        />
      );
    case "streamState":
      return (
        <select
          value={trigger.live ? "on" : "off"}
          onChange={(event) => onChange({ ...trigger, live: event.target.value === "on" })}
          aria-label={t("automation-trigger-stream")}
          className={inputClass}
        >
          <option value="on">{t("automation-starts")}</option>
          <option value="off">{t("automation-stops")}</option>
        </select>
      );
    case "recordingState":
      return (
        <select
          value={trigger.recording ? "on" : "off"}
          onChange={(event) => onChange({ ...trigger, recording: event.target.value === "on" })}
          aria-label={t("automation-trigger-recording")}
          className={inputClass}
        >
          <option value="on">{t("automation-starts")}</option>
          <option value="off">{t("automation-stops")}</option>
        </select>
      );
    case "sourceError":
      return (
        <input
          value={trigger.source}
          onChange={(event) => onChange({ ...trigger, source: event.target.value })}
          placeholder={t("automation-any-source")}
          aria-label={t("automation-source-name")}
          className={inputClass}
        />
      );
    case "audioLevel":
      return (
        <>
          <input
            value={trigger.source}
            onChange={(event) => onChange({ ...trigger, source: event.target.value })}
            placeholder={t("automation-source-name")}
            aria-label={t("automation-source-name")}
            className={`${inputClass} w-28`}
          />
          <select
            value={trigger.above ? "above" : "below"}
            onChange={(event) => onChange({ ...trigger, above: event.target.value === "above" })}
            aria-label={t("automation-trigger-audio")}
            className={inputClass}
          >
            <option value="above">{t("automation-rises-above")}</option>
            <option value="below">{t("automation-falls-below")}</option>
          </select>
          <input
            type="number"
            value={trigger.thresholdDb}
            onChange={(event) =>
              onChange({ ...trigger, thresholdDb: Number(event.target.value) || 0 })
            }
            aria-label={t("automation-threshold")}
            className={`${inputClass} w-20`}
          />
          <span className="text-havoc-muted">dB</span>
        </>
      );
    case "systemIdle":
      return (
        <>
          <input
            type="number"
            min={1}
            value={trigger.seconds}
            onChange={(event) =>
              onChange({ ...trigger, seconds: Math.max(1, Number(event.target.value) || 1) })
            }
            aria-label={t("automation-idle-seconds")}
            className={`${inputClass} w-20`}
          />
          <span className="text-havoc-muted">{t("automation-seconds-windows")}</span>
        </>
      );
    case "windowFocus":
      return (
        <>
          <input
            value={trigger.exe}
            onChange={(event) => onChange({ ...trigger, exe: event.target.value })}
            placeholder="game.exe"
            aria-label={t("automation-exe")}
            className={inputClass}
          />
          <span className="text-havoc-muted">{t("automation-windows-only")}</span>
        </>
      );
    case "timeOfDay":
      return (
        <input
          value={trigger.at}
          onChange={(event) => onChange({ ...trigger, at: event.target.value })}
          placeholder="20:00"
          aria-label={t("automation-time")}
          className={`${inputClass} w-24`}
        />
      );
    case "fileChanged":
      return (
        <input
          value={trigger.path}
          onChange={(event) => onChange({ ...trigger, path: event.target.value })}
          placeholder="C:/scores/score.txt"
          aria-label={t("automation-file")}
          className={`${inputClass} min-w-0 flex-1`}
        />
      );
  }
}
