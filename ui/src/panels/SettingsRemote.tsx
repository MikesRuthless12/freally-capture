import { useState } from "react";

import { settingsSet } from "../api/commands";
import type { RemoteControlSettings, Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/**
 * Settings → Remote Control (TASK-701): the WebSocket remote API for Stream
 * Deck / Companion-style controllers. Off by default; a password is required;
 * binds 127.0.0.1 unless LAN is explicitly enabled. Disabled = the port is
 * closed. Auth is challenge–response — the password never crosses the wire.
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
  const [draft, setDraft] = useState<RemoteControlSettings | null>(settings?.remoteControl ?? null);
  const [showPassword, setShowPassword] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (!settings || !draft) return null;

  const save = () => {
    setError(null);
    if (draft.enabled && !draft.password.trim()) {
      setError("A password is required to enable the remote API.");
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
    <PickerShell title="Settings — Remote Control" onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <label className="flex items-center gap-2 text-[12px]">
          <input
            type="checkbox"
            checked={draft.enabled}
            onChange={(event) => setDraft({ ...draft, enabled: event.target.checked })}
          />
          Enable the WebSocket remote API
        </label>
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          Password (required — controllers authenticate with it)
          <div className="flex gap-2">
            <input
              type={showPassword ? "text" : "password"}
              value={draft.password}
              onChange={(event) => setDraft({ ...draft, password: event.target.value })}
              placeholder="a password for your controllers"
              className={`${inputClass} min-w-0 flex-1`}
            />
            <button
              type="button"
              onClick={() => setShowPassword((shown) => !shown)}
              className="shrink-0 rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
            >
              {showPassword ? "Hide" : "Show"}
            </button>
          </div>
        </label>
        <div className="flex items-end gap-3">
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            Port
            <input
              type="number"
              min={1024}
              max={65535}
              value={draft.port}
              onChange={(event) =>
                setDraft({ ...draft, port: Number(event.target.value) || draft.port })
              }
              className={`${inputClass} w-24`}
            />
          </label>
          <label className="flex items-center gap-2 pb-1.5 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={draft.lan}
              onChange={(event) => setDraft({ ...draft, lan: event.target.checked })}
            />
            Allow LAN connections (default is this machine only)
          </label>
        </div>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          Off = the port is closed. On = a password-protected WebSocket on 127.0.0.1 (or your LAN
          when opted in) that can switch scenes, run the transition, start/stop the stream and
          recording, save replays, and set mutes/volumes — the same actions as the UI, nothing more.
          It cannot read files. Treat the password like any credential; prefer this-machine-only
          unless you specifically control from another device.
        </p>
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
            Cancel
          </button>
          <button
            type="button"
            onClick={save}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            Save
          </button>
        </div>
      </div>
    </PickerShell>
  );
}
