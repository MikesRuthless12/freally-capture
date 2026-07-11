import { useState } from "react";

import { settingsSet } from "../api/commands";
import type { HotkeySettings, Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

const FIELDS: Array<[keyof HotkeySettings, string, string]> = [
  ["record", "hotkeys-record", "hotkeys-record-placeholder"],
  ["goLive", "hotkeys-go-live", "hotkeys-go-live-placeholder"],
  ["transition", "hotkeys-transition", "hotkeys-transition-placeholder"],
  ["saveReplay", "hotkeys-save-replay", "hotkeys-save-replay-placeholder"],
  ["addMarker", "hotkeys-add-marker", "hotkeys-add-marker-placeholder"],
  ["still", "hotkeys-still", "hotkeys-still-placeholder"],
];

/**
 * Settings → Hotkeys (TASK-505): OS-global action keys. They work while
 * other apps are focused; on Linux/Wayland global hotkeys may be unavailable
 * — that's a compositor limit, said honestly (the buttons still work).
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
  const [error, setError] = useState<string | null>(null);

  if (!settings || !draft) return null;

  const save = () => {
    setError(null);
    const normalized: HotkeySettings = {
      record: draft.record?.trim() || null,
      goLive: draft.goLive?.trim() || null,
      transition: draft.transition?.trim() || null,
      saveReplay: draft.saveReplay?.trim() || null,
      addMarker: draft.addMarker?.trim() || null,
      still: draft.still?.trim() || null,
    };
    const next = { ...settings, hotkeys: normalized };
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
        {FIELDS.map(([key, label, placeholder]) => (
          <label key={key} className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t(label)}
            <input
              value={draft[key] ?? ""}
              onChange={(event) => setDraft({ ...draft, [key]: event.target.value })}
              placeholder={t(placeholder)}
              className={inputClass}
            />
          </label>
        ))}
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("hotkeys-note")}</p>
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
