import { useState } from "react";

import { settingsSet } from "../api/commands";
import type { ScriptSettings, Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/**
 * Scripts (TASK-703): sandboxed Lua files that react to studio events
 * (go-live, scene change, recording state) and call the same command surface
 * as the remote API. The sandbox has no file or OS access; a script error
 * never touches the stream or recording. `scripts/sample.lua` in the repo
 * shows the API.
 */
export function ScriptsDialog({
  settings,
  onSaved,
  onClose,
}: {
  settings: Settings | null;
  onSaved: (next: Settings) => void;
  onClose: () => void;
}) {
  const t = useT();
  const [scripts, setScripts] = useState<ScriptSettings[]>(settings?.scripts ?? []);
  const [path, setPath] = useState("");
  const [error, setError] = useState<string | null>(null);

  if (!settings) return null;

  const persist = (next: ScriptSettings[]) => {
    setScripts(next);
    const nextSettings = { ...settings, scripts: next };
    settingsSet(nextSettings)
      .then(() => onSaved(nextSettings))
      .catch((err) => setError(String(err)));
  };

  const add = () => {
    setError(null);
    const trimmed = path.trim();
    if (!trimmed.toLowerCase().endsWith(".lua")) {
      setError(t("scripts-error-not-lua"));
      return;
    }
    persist([...scripts, { path: trimmed, enabled: true }]);
    setPath("");
  };

  return (
    <PickerShell title={t("scripts-title")} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        {scripts.length === 0 && (
          <p className="m-0 text-[11px] text-havoc-muted">{t("scripts-empty")}</p>
        )}
        {scripts.map((script, index) => (
          <div key={`${script.path}-${index}`} className="flex items-center gap-2">
            <label className="flex min-w-0 flex-1 items-center gap-2">
              <input
                type="checkbox"
                checked={script.enabled}
                onChange={(event) =>
                  persist(
                    scripts.map((entry, i) =>
                      i === index ? { ...entry, enabled: event.target.checked } : entry,
                    ),
                  )
                }
                aria-label={t("scripts-enable", { path: script.path })}
              />
              <span className="min-w-0 flex-1 truncate font-mono text-[10px] text-havoc-muted">
                {script.path}
              </span>
            </label>
            <button
              type="button"
              onClick={() => persist(scripts.filter((_, i) => i !== index))}
              aria-label={t("scripts-remove", { path: script.path })}
              className="shrink-0 rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-red-400/50 hover:text-red-300"
            >
              ✕
            </button>
          </div>
        ))}
        <div className="flex flex-col gap-2 border-t border-white/5 pt-2">
          <div className="flex gap-2">
            <input
              value={path}
              onChange={(event) => setPath(event.target.value)}
              placeholder="C:\\path\\to\\script.lua"
              aria-label={t("scripts-path-label")}
              className={`${inputClass} min-w-0 flex-1`}
            />
            <button
              type="button"
              onClick={add}
              className="shrink-0 rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
            >
              {t("scripts-add")}
            </button>
          </div>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("scripts-note")}</p>
        </div>
        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}
      </div>
    </PickerShell>
  );
}
