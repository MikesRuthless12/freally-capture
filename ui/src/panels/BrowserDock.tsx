import { useState } from "react";

import { browserDockOpen, settingsSet } from "../api/commands";
import type { BrowserDockSettings, Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/**
 * Browser docks (TASK-702): named URLs — a chat popout, an alerts panel,
 * Companion web buttons — opened as their own dock windows beside the studio.
 * The dock window has no IPC surface (it's outside the app's capability set):
 * the page renders, nothing more. The list persists in settings; nothing
 * opens without a click.
 */
export function BrowserDockDialog({
  settings,
  onSaved,
  onClose,
}: {
  settings: Settings | null;
  onSaved: (next: Settings) => void;
  onClose: () => void;
}) {
  const t = useT();
  const [docks, setDocks] = useState<BrowserDockSettings[]>(settings?.browserDocks ?? []);
  const [name, setName] = useState("");
  const [url, setUrl] = useState("");
  const [error, setError] = useState<string | null>(null);

  if (!settings) return null;

  const persist = (next: BrowserDockSettings[]) => {
    setDocks(next);
    const nextSettings = { ...settings, browserDocks: next };
    settingsSet(nextSettings)
      .then(() => onSaved(nextSettings))
      .catch((err) => setError(String(err)));
  };

  const add = () => {
    setError(null);
    const trimmedName = name.trim();
    const trimmedUrl = url.trim();
    if (!trimmedName) {
      setError(t("browser-dock-error-name"));
      return;
    }
    if (!/^https?:\/\//.test(trimmedUrl)) {
      setError(t("browser-dock-error-url"));
      return;
    }
    persist([...docks, { name: trimmedName, url: trimmedUrl }]);
    setName("");
    setUrl("");
  };

  const open = (dock: BrowserDockSettings) => {
    setError(null);
    browserDockOpen(dock.name, dock.url).catch((err) => setError(String(err)));
  };

  return (
    <PickerShell title={t("browser-dock-title")} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        {docks.length === 0 && (
          <p className="m-0 text-[11px] text-havoc-muted">{t("browser-dock-empty")}</p>
        )}
        {docks.map((dock, index) => (
          <div key={`${dock.name}-${index}`} className="flex items-center gap-2">
            <div className="min-w-0 flex-1">
              <p className="m-0 truncate text-[12px] text-havoc-text">{dock.name}</p>
              <p className="m-0 truncate font-mono text-[10px] text-havoc-muted">{dock.url}</p>
            </div>
            <button
              type="button"
              onClick={() => open(dock)}
              className="shrink-0 rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2.5 py-1 text-[11px] font-semibold text-havoc-text hover:bg-havoc-accent/25"
            >
              {t("browser-dock-open")}
            </button>
            <button
              type="button"
              onClick={() => persist(docks.filter((_, i) => i !== index))}
              aria-label={t("browser-dock-remove", { name: dock.name })}
              className="shrink-0 rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-red-400/50 hover:text-red-300"
            >
              ✕
            </button>
          </div>
        ))}
        <div className="flex flex-col gap-2 border-t border-white/5 pt-2">
          <div className="flex gap-2">
            <input
              value={name}
              onChange={(event) => setName(event.target.value)}
              placeholder={t("browser-dock-name-placeholder")}
              aria-label={t("browser-dock-name-label")}
              className={`${inputClass} w-36 shrink-0`}
            />
            <input
              value={url}
              onChange={(event) => setUrl(event.target.value)}
              placeholder="https://…"
              aria-label={t("browser-dock-url-label")}
              className={`${inputClass} min-w-0 flex-1`}
            />
            <button
              type="button"
              onClick={add}
              className="shrink-0 rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
            >
              Add
            </button>
          </div>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("browser-dock-note")}</p>
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
