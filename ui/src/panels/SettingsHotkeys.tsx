import { useState } from "react";

import { settingsSet } from "../api/commands";
import type { HotkeySettings, PanicSlateSettings, Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";
import { normalizeHotkeys } from "../lib/settingsDraft";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

const FIELDS: Array<[keyof HotkeySettings, string]> = [
  ["record", "hotkeys-record"],
  ["goLive", "hotkeys-go-live"],
  ["transition", "hotkeys-transition"],
  ["saveReplay", "hotkeys-save-replay"],
  ["addMarker", "hotkeys-add-marker"],
  ["still", "hotkeys-still"],
  ["panic", "hotkeys-panic"],
  ["timerToggle", "hotkeys-timer-toggle"],
  ["timerReset", "hotkeys-timer-reset"],
  ["zoom100", "hotkeys-zoom-100"],
  ["zoom150", "hotkeys-zoom-150"],
  ["zoom200", "hotkeys-zoom-200"],
  ["splitTimerSplit", "hotkeys-split-split"],
  ["splitTimerUndo", "hotkeys-split-undo"],
  ["splitTimerSkip", "hotkeys-split-skip"],
  ["splitTimerReset", "hotkeys-split-reset"],
  ["playlistNext", "hotkeys-playlist-next"],
  ["playlistPrevious", "hotkeys-playlist-previous"],
  ["replayRoll", "hotkeys-replay-roll"],
];

/**
 * The curated accelerator pool (obs-chrome). Bindings are PICKED, never
 * typed — free text let "Ctrl+asekfj…" reach the store (Rust's
 * `validate_accelerator` now refuses that class too; this is the entry-side
 * half). The strings are EXACTLY the format `HotkeySettings` stores and
 * hotkeys.rs parses: "Ctrl+Shift+R", "Ctrl+Alt+Right", "F13", "Numpad1".
 * ~146 options for 19 rows, grouped for the <optgroup>s.
 */
const POOL_KEYS = [..."ABCDEFGHIJKLMNOPQRSTUVWXYZ", ..."0123456789"];
const POOL_GROUPS: Array<[string, string[]]> = [
  ["settings-hotkey-group-ctrl", POOL_KEYS.map((key) => `Ctrl+${key}`)],
  ["settings-hotkey-group-ctrl-shift", POOL_KEYS.map((key) => `Ctrl+Shift+${key}`)],
  [
    "settings-hotkey-group-ctrl-alt",
    [
      ...POOL_KEYS.map((key) => `Ctrl+Alt+${key}`),
      ...["Up", "Down", "Left", "Right"].map((key) => `Ctrl+Alt+${key}`),
    ],
  ],
  ["settings-hotkey-group-function", Array.from({ length: 24 }, (_, at) => `F${at + 1}`)],
  ["settings-hotkey-group-numpad", [..."0123456789"].map((digit) => `Numpad${digit}`)],
];
const POOL = new Set(POOL_GROUPS.flatMap(([, options]) => options));

/**
 * The Hotkeys editor body (TASK-505): OS-global action keys, with an
 * OBS-style filter box over the binding list, plus the panic slate (CAP-M22)
 * configured beside its hotkey. Pure draft editing — the caller saves.
 *
 * Each row is a combobox over ONE shared pool with exclusive allocation: a
 * combo held by any row disappears from every other row's options, so two
 * actions can never claim the same key by construction. "None" unbinds and
 * returns the combo to the pool. A saved binding from OUTSIDE the pool (a
 * hand-edited settings.json, or a chord like "Ctrl+K, 3" from CAP-N05) is
 * rendered as an extra option on ITS row — shown honestly, never silently
 * clobbered — and it participates in exclusion like everything else. The
 * conflict audit (CAP-M14) still covers whatever imports bring in.
 */
export function HotkeySettingsBody({
  hotkeys,
  onChangeHotkeys,
  slate,
  onChangeSlate,
}: {
  hotkeys: HotkeySettings;
  onChangeHotkeys: (next: HotkeySettings) => void;
  slate: PanicSlateSettings;
  onChangeSlate: (next: PanicSlateSettings) => void;
}) {
  const t = useT();
  const [filter, setFilter] = useState("");

  // Filter on what the user sees: the translated action label. A binding
  // whose value matches counts too ("F13" finds where F13 went).
  const query = filter.trim().toLowerCase();
  const shown = FIELDS.filter(
    ([key, label]) =>
      !query ||
      t(label).toLowerCase().includes(query) ||
      (hotkeys[key] ?? "").toLowerCase().includes(query),
  );

  // Every bound combo, across ALL rows — the exclusion set.
  const taken = new Set(
    FIELDS.map(([key]) => hotkeys[key]).filter((value): value is string => Boolean(value)),
  );

  return (
    <div className="flex flex-col gap-3 text-xs text-havoc-text">
      <input
        value={filter}
        onChange={(event) => setFilter(event.target.value)}
        placeholder={t("settings-hotkeys-filter-placeholder")}
        aria-label={t("settings-hotkeys-filter")}
        className={inputClass}
      />
      {shown.length === 0 && (
        <p className="m-0 text-[11px] text-havoc-muted">
          {t("settings-hotkeys-no-match", { query: filter.trim() })}
        </p>
      )}
      {shown.map(([key, label]) => {
        const current = hotkeys[key] ?? "";
        return (
          <label key={key} className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t(label)}
            <select
              value={current}
              onChange={(event) =>
                onChangeHotkeys({ ...hotkeys, [key]: event.target.value || null })
              }
              className={inputClass}
            >
              <option value="">{t("settings-hotkey-none")}</option>
              {current && !POOL.has(current) && <option value={current}>{current}</option>}
              {POOL_GROUPS.map(([groupLabel, options]) => (
                <optgroup key={groupLabel} label={t(groupLabel)}>
                  {options
                    .filter((option) => option === current || !taken.has(option))
                    .map((option) => (
                      <option key={option} value={option}>
                        {option}
                      </option>
                    ))}
                </optgroup>
              ))}
            </select>
          </label>
        );
      })}
      <h4 className="m-0 mt-1 text-[10px] tracking-wide text-havoc-muted uppercase">
        {t("settings-panic-section")}
      </h4>
      <div className="grid grid-cols-2 gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("panic-slate-color")}
          <input
            value={slate.color}
            onChange={(event) => onChangeSlate({ ...slate, color: event.target.value })}
            placeholder="#10141a"
            className={inputClass}
          />
        </label>
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("panic-slate-image")}
          <input
            value={slate.image}
            onChange={(event) => onChangeSlate({ ...slate, image: event.target.value })}
            placeholder={t("panic-slate-image-placeholder")}
            className={inputClass}
          />
        </label>
      </div>
      <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("hotkeys-note")}</p>
    </div>
  );
}

/**
 * Settings → Hotkeys as a standalone dialog — the Controls dock's "Keys…"
 * button. The unified Settings modal renders `HotkeySettingsBody` instead.
 * On Linux/Wayland global hotkeys may be unavailable — that's a compositor
 * limit, said honestly (the buttons still work).
 */
export function SettingsHotkeys({
  settings,
  onSaved,
  onClose,
}: {
  settings: Settings | null;
  onSaved: (next: Settings) => void;
  onClose: () => void;
}) {
  const t = useT();
  const [draft, setDraft] = useState<HotkeySettings | null>(settings?.hotkeys ?? null);
  // The panic slate (CAP-M22) is configured beside its hotkey.
  const [slate, setSlate] = useState<PanicSlateSettings | null>(settings?.panicSlate ?? null);
  const [error, setError] = useState<string | null>(null);

  if (!settings || !draft || !slate) return null;

  const save = () => {
    setError(null);
    const next = {
      ...settings,
      hotkeys: normalizeHotkeys(draft),
      panicSlate: { color: slate.color.trim(), image: slate.image.trim() },
    };
    settingsSet(next)
      .then(() => {
        onSaved(next);
        onClose();
      })
      .catch((err) => setError(String(err)));
  };

  return (
    <PickerShell title={t("hotkeys-title")} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <HotkeySettingsBody
          hotkeys={draft}
          onChangeHotkeys={setDraft}
          slate={slate}
          onChangeSlate={setSlate}
        />
        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}
        <div className="flex justify-end gap-2">
          <button
            type="button"
            onClick={onClose}
            className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
          >
            {t("hotkeys-cancel")}
          </button>
          <button
            type="button"
            onClick={save}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            {t("hotkeys-save")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}
