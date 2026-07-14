import { useEffect, useState } from "react";
import qrcode from "qrcode-generator";

import { linkUrl, panelUrl, settingsSet } from "../api/commands";
import type { LinkSettings, OscSettings, Settings, WebPanelSettings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";
import { DEFAULT_LINK, DEFAULT_OSC } from "../lib/settingsDraft";

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
 * The LAN services editor body (CAP-N06/N07 + CAP-N04 + CAP-N12): the phone
 * control panel + tally pages, the OSC control surface, and the Freally Link
 * output — grouped sections over one draft each. Everything is off by
 * default; passwords/keys gate what is on; loopback unless LAN is explicit.
 * Nothing is fetched from the internet — the pages are embedded in the app.
 * Pure draft editing — the caller saves.
 *
 * The QR/URL block shows the *running* service's address (polled), keyed with
 * the DRAFT password so a freshly typed one scans correctly after Apply.
 */
export function LanServicesSections({
  webPanel,
  onChangeWebPanel,
  osc,
  onChangeOsc,
  link,
  onChangeLink,
}: {
  webPanel: WebPanelSettings;
  onChangeWebPanel: (next: WebPanelSettings) => void;
  osc: OscSettings;
  onChangeOsc: (next: OscSettings) => void;
  link: LinkSettings;
  onChangeLink: (next: LinkSettings) => void;
}) {
  const t = useT();
  const [showPassword, setShowPassword] = useState(false);
  const [url, setUrl] = useState<string | null>(null);
  const [showLinkKey, setShowLinkKey] = useState(false);
  const [linkAddress, setLinkAddress] = useState<string | null>(null);

  useEffect(() => {
    let alive = true;
    const load = () => {
      panelUrl()
        .then((next) => alive && setUrl(next ?? null))
        .catch(() => undefined);
      linkUrl()
        .then((next) => alive && setLinkAddress(next ?? null))
        .catch(() => undefined);
    };
    load();
    const timer = window.setInterval(load, 2000);
    return () => {
      alive = false;
      window.clearInterval(timer);
    };
  }, []);

  // The served links carry the key, so a scanned QR just works.
  const keyed = (path: string) =>
    url ? `${url.replace(/\/$/, "")}${path}?k=${encodeURIComponent(webPanel.password)}` : "";
  const panelLink = keyed("/");
  const tallyLink = keyed("/tally");

  return (
    <div className="flex flex-col gap-3 text-xs text-havoc-text">
      <p className="m-0 text-havoc-muted">{t("panel-about")}</p>

      <label className="flex items-center gap-2">
        <input
          type="checkbox"
          checked={webPanel.enabled}
          onChange={(event) => onChangeWebPanel({ ...webPanel, enabled: event.target.checked })}
        />
        {t("panel-enable")}
      </label>

      <label className="flex items-center justify-between gap-3">
        <span className="text-havoc-muted">{t("panel-port")}</span>
        <input
          type="number"
          min={1024}
          max={65535}
          value={webPanel.port}
          onChange={(event) =>
            onChangeWebPanel({ ...webPanel, port: Number(event.target.value) || webPanel.port })
          }
          className={`${inputClass} w-28`}
        />
      </label>

      <label className="flex items-center gap-2">
        <input
          type="checkbox"
          checked={webPanel.lan}
          onChange={(event) => onChangeWebPanel({ ...webPanel, lan: event.target.checked })}
        />
        {t("panel-lan")}
      </label>
      {webPanel.lan && <p className="m-0 text-amber-400">{t("panel-lan-warning")}</p>}

      <label className="flex items-center justify-between gap-3">
        <span className="text-havoc-muted">{t("panel-password")}</span>
        <span className="flex items-center gap-1">
          <input
            type={showPassword ? "text" : "password"}
            value={webPanel.password}
            onChange={(event) => onChangeWebPanel({ ...webPanel, password: event.target.value })}
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

      {url && webPanel.enabled && webPanel.password ? (
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
            onChange={(event) => onChangeOsc({ ...osc, enabled: event.target.checked })}
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
            onChange={(event) =>
              onChangeOsc({ ...osc, port: Number(event.target.value) || osc.port })
            }
            className={`${inputClass} w-28`}
          />
        </label>
        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={osc.lan}
            onChange={(event) => onChangeOsc({ ...osc, lan: event.target.checked })}
          />
          {t("panel-lan")}
        </label>
        {osc.lan && <p className="m-0 text-amber-400">{t("osc-lan-warning")}</p>}
        <p className="m-0 font-mono text-[10px] text-havoc-muted">{t("osc-addresses")}</p>
      </section>

      <section className="flex flex-col gap-2 border-t border-white/5 pt-3">
        <span className="text-[11px] font-semibold tracking-wider text-havoc-muted uppercase">
          {t("link-title")}
        </span>
        <p className="m-0 text-havoc-muted">{t("link-about")}</p>
        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={link.enabled}
            onChange={(event) => onChangeLink({ ...link, enabled: event.target.checked })}
          />
          {t("link-enable")}
        </label>
        {link.enabled && <p className="m-0 text-amber-400">{t("link-lan-warning")}</p>}
        <label className="flex items-center justify-between gap-3">
          <span className="text-havoc-muted">{t("panel-port")}</span>
          <input
            type="number"
            min={1024}
            max={65535}
            value={link.port}
            onChange={(event) =>
              onChangeLink({ ...link, port: Number(event.target.value) || link.port })
            }
            className={`${inputClass} w-28`}
          />
        </label>
        <label className="flex items-center justify-between gap-3">
          <span className="text-havoc-muted">{t("link-name")}</span>
          <input
            value={link.name}
            onChange={(event) => onChangeLink({ ...link, name: event.target.value })}
            placeholder="Freally Capture"
            className={`${inputClass} w-40`}
          />
        </label>
        <label className="flex items-center justify-between gap-3">
          <span className="text-havoc-muted">{t("link-key")}</span>
          <span className="flex items-center gap-1">
            <input
              type={showLinkKey ? "text" : "password"}
              value={link.key}
              onChange={(event) => onChangeLink({ ...link, key: event.target.value })}
              className={`${inputClass} w-40`}
            />
            <button
              type="button"
              onClick={() => setShowLinkKey((shown) => !shown)}
              className="rounded px-1.5 text-havoc-muted hover:text-havoc-text"
            >
              {showLinkKey ? t("panel-hide") : t("panel-show")}
            </button>
          </span>
        </label>
        <p className="m-0 text-havoc-muted">{t("link-key-hint")}</p>
        {link.enabled && linkAddress ? (
          <p className="m-0 text-havoc-muted">
            {t("link-serving")}{" "}
            <code className="font-mono text-[10px] text-havoc-text">{linkAddress}</code>
          </p>
        ) : (
          <p className="m-0 text-havoc-muted">{t("link-off-hint")}</p>
        )}
      </section>
    </div>
  );
}

/**
 * Settings → LAN panel & tally as a standalone dialog — the Controls dock's
 * "Panel…" button. The unified Settings modal renders `LanServicesSections`
 * inside its Network category instead.
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
  const [error, setError] = useState<string | null>(null);
  const [osc, setOsc] = useState<OscSettings>(settings?.osc ?? DEFAULT_OSC);
  // Freally Link output (CAP-N12) — off by default; one receiver at a time.
  const [link, setLink] = useState<LinkSettings>(settings?.link ?? DEFAULT_LINK);

  if (!settings || !draft) return null;

  const save = () => {
    const next = { ...settings, webPanel: draft, osc, link };
    setError(null);
    settingsSet(next)
      .then(() => onSaved(next))
      .catch((err) => setError(String(err)));
  };

  return (
    <PickerShell title={t("panel-title")} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        {error && <p className="m-0 text-red-400">{error}</p>}
        <LanServicesSections
          webPanel={draft}
          onChangeWebPanel={setDraft}
          osc={osc}
          onChangeOsc={setOsc}
          link={link}
          onChangeLink={setLink}
        />
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
