import { useState } from "react";

import { settingsSet } from "../api/commands";
import type { RemoteControlSettings, Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/**
 * The Remote Control editor body (TASK-701): the WebSocket remote API for
 * Stream Deck / Companion-style controllers. Off by default; a password is
 * required; binds 127.0.0.1 unless LAN is explicitly enabled. Disabled = the
 * port is closed. Auth is challenge–response — the password never crosses the
 * wire. Pure draft editing — the caller saves and enforces the
 * password-required rule (`remote-password-required`).
 */
export function RemoteApiSection({
  remoteControl,
  onChange,
}: {
  remoteControl: RemoteControlSettings;
  onChange: (next: RemoteControlSettings) => void;
}) {
  const t = useT();
  const [showPassword, setShowPassword] = useState(false);

  return (
    <div className="flex flex-col gap-3 text-xs text-havoc-text">
      <label className="flex items-center gap-2 text-[12px]">
        <input
          type="checkbox"
          checked={remoteControl.enabled}
          onChange={(event) => onChange({ ...remoteControl, enabled: event.target.checked })}
        />
        {t("remote-enable")}
      </label>
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("remote-password")}
        <div className="flex gap-2">
          <input
            type={showPassword ? "text" : "password"}
            value={remoteControl.password}
            onChange={(event) => onChange({ ...remoteControl, password: event.target.value })}
            placeholder={t("remote-password-placeholder")}
            className={`${inputClass} min-w-0 flex-1`}
          />
          <button
            type="button"
            onClick={() => setShowPassword((shown) => !shown)}
            className="shrink-0 rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
          >
            {showPassword ? t("remote-password-hide") : t("remote-password-show")}
          </button>
        </div>
      </label>
      <div className="flex items-end gap-3">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("remote-port")}
          <input
            type="number"
            min={1024}
            max={65535}
            value={remoteControl.port}
            onChange={(event) =>
              onChange({
                ...remoteControl,
                port: Number(event.target.value) || remoteControl.port,
              })
            }
            className={`${inputClass} w-24`}
          />
        </label>
        <label className="flex items-center gap-2 pb-1.5 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={remoteControl.lan}
            onChange={(event) => onChange({ ...remoteControl, lan: event.target.checked })}
          />
          {t("remote-allow-lan")}
        </label>
      </div>
      <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("remote-note")}</p>
    </div>
  );
}

/**
 * Settings → Remote Control as a standalone dialog — the Controls dock's
 * "Remote…" button. The unified Settings modal renders `RemoteApiSection`
 * inside its Network category instead.
 */
export function SettingsRemote({
  settings,
  onSaved,
  onClose,
}: {
  settings: Settings | null;
  onSaved: (next: Settings) => void;
  onClose: () => void;
}) {
  const t = useT();
  const [draft, setDraft] = useState<RemoteControlSettings | null>(settings?.remoteControl ?? null);
  const [error, setError] = useState<string | null>(null);

  if (!settings || !draft) return null;

  const save = () => {
    setError(null);
    if (draft.enabled && !draft.password.trim()) {
      setError(t("remote-password-required"));
      return;
    }
    const next = { ...settings, remoteControl: draft };
    settingsSet(next)
      .then(() => {
        onSaved(next);
        onClose();
      })
      .catch((err) => setError(String(err)));
  };

  return (
    <PickerShell title={t("remote-title")} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <RemoteApiSection remoteControl={draft} onChange={setDraft} />
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
            {t("remote-cancel")}
          </button>
          <button
            type="button"
            onClick={save}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            {t("remote-save")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}
