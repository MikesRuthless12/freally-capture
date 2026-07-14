import { useEffect, useState } from "react";
import qrcode from "qrcode-generator";

import { panelUrl, settingsSet } from "../api/commands";
import type { OscSettings, Settings, WebPanelSettings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/** A QR for `text`, as a data URI (drawn locally — nothing is fetched). */
function qrDataUri(text: string): string {
  const qr = qrcode(0, "M");
  qr.addData(text);
  qr.make();
  return qr.createDataURL(5, 8);
}

/**
 * Settings → LAN panel & tally (CAP-N06 / CAP-N07).
 *
 * The app serves a control page and a full-screen tally page to phones on the
 * operator's own network. Off by default; a password is required; it binds
 * 127.0.0.1 unless LAN is explicitly enabled; every command it accepts is on
 * the same fixed allowlist the desktop UI uses. Nothing is fetched from the
 * internet — the page is embedded in the app.
 */
export function SettingsPanel({
  settings,
  onSaved,
  onClose,
}: {
  settings: Settings | null;
  onSaved: (next: Settings) => void;
  onClose: () => void;
}) {
  const t = useT();
  const [draft, setDraft] = useState<WebPanelSettings | null>(settings?.webPanel ?? null);
  const [showPassword, setShowPassword] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [url, setUrl] = useState<string | null>(null);
  const [osc, setOsc] = useState<OscSettings>(
    settings?.osc ?? { enabled: false, port: 9000, lan: false },
  );

  useEffect(() => {
    let alive = true;
    const load = () => {
      panelUrl()
        .then((next) => alive && setUrl(next ?? null))
        .catch(() => undefined);
    };
    load();
    const timer = window.setInterval(load, 2000);
    return () => {
      alive = false;
      window.clearInterval(timer);
    };
  }, []);

  if (!settings || !draft) return null;

  const save = () => {
    const next = { ...settings, webPanel: draft, osc };
    setError(null);
    settingsSet(next)
      .then(() => onSaved(next))
      .catch((err) => setError(String(err)));
  };

  // The served links carry the key, so a scanned QR just works.
  const keyed = (path: string) =>
    url ? `${url.replace(/\/$/, "")}${path}?k=${encodeURIComponent(draft.password)}` : "";
  const panelLink = keyed("/");
  const tallyLink = keyed("/tally");

  return (
    <PickerShell title={t("panel-title")} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-havoc-muted">{t("panel-about")}</p>
        {error && <p className="m-0 text-red-400">{error}</p>}

        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={draft.enabled}
            onChange={(event) => setDraft({ ...draft, enabled: event.target.checked })}
          />
          {t("panel-enable")}
        </label>

        <label className="flex items-center justify-between gap-3">
          <span className="text-havoc-muted">{t("panel-port")}</span>
          <input
            type="number"
            min={1024}
            max={65535}
            value={draft.port}
            onChange={(event) =>
              setDraft({ ...draft, port: Number(event.target.value) || draft.port })
            }
            className={`${inputClass} w-28`}
          />
        </label>

        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={draft.lan}
            onChange={(event) => setDraft({ ...draft, lan: event.target.checked })}
          />
          {t("panel-lan")}
        </label>
        {draft.lan && <p className="m-0 text-amber-400">{t("panel-lan-warning")}</p>}

        <label className="flex items-center justify-between gap-3">
          <span className="text-havoc-muted">{t("panel-password")}</span>
          <span className="flex items-center gap-1">
            <input
              type={showPassword ? "text" : "password"}
              value={draft.password}
              onChange={(event) => setDraft({ ...draft, password: event.target.value })}
              className={`${inputClass} w-40`}
            />
            <button
              type="button"
              onClick={() => setShowPassword((shown) => !shown)}
              className="rounded px-1.5 text-havoc-muted hover:text-havoc-text"
            >
              {showPassword ? t("panel-hide") : t("panel-show")}
            </button>
          </span>
        </label>

        {url && draft.enabled && draft.password ? (
          <div className="flex flex-col items-center gap-2 rounded-lg border border-white/10 p-3">
            <img src={qrDataUri(panelLink)} alt={t("panel-qr-alt")} className="h-40 w-40" />
            <code className="break-all text-center text-[10px] text-havoc-muted">{panelLink}</code>
            <p className="m-0 text-center text-havoc-muted">{t("panel-tally-hint")}</p>
            <code className="break-all text-center text-[10px] text-havoc-muted">{tallyLink}</code>
          </div>
        ) : (
          <p className="m-0 text-havoc-muted">{t("panel-off-hint")}</p>
        )}

        <section className="flex flex-col gap-2 border-t border-white/5 pt-3">
          <span className="text-[11px] font-semibold tracking-wider text-havoc-muted uppercase">
            {t("osc-title")}
          </span>
          <p className="m-0 text-havoc-muted">{t("osc-about")}</p>
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={osc.enabled}
              onChange={(event) => setOsc({ ...osc, enabled: event.target.checked })}
            />
            {t("osc-enable")}
          </label>
          <label className="flex items-center justify-between gap-3">
            <span className="text-havoc-muted">{t("panel-port")}</span>
            <input
              type="number"
              min={1024}
              max={65535}
              value={osc.port}
              onChange={(event) => setOsc({ ...osc, port: Number(event.target.value) || osc.port })}
              className={`${inputClass} w-28`}
            />
          </label>
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={osc.lan}
              onChange={(event) => setOsc({ ...osc, lan: event.target.checked })}
            />
            {t("panel-lan")}
          </label>
          {osc.lan && <p className="m-0 text-amber-400">{t("osc-lan-warning")}</p>}
          <p className="m-0 font-mono text-[10px] text-havoc-muted">{t("osc-addresses")}</p>
        </section>

        <button
          type="button"
          onClick={save}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 font-semibold"
        >
          {t("panel-save")}
        </button>
      </div>
    </PickerShell>
  );
}
