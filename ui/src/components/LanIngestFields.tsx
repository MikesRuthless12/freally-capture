import type { IngestProtocol } from "../api/types";
import { useT } from "../i18n/t";
import { LAN_DEFAULT_PORTS, lanIngestUrl, lanPassphraseUsable } from "../lib/lanIngest";
import { useLocalLanIp } from "../lib/useLocalLanIp";
import { NumberField } from "./NumberField";
import { QrSvg } from "./QrSvg";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

export type LanIngestValue = {
  protocol: IngestProtocol;
  port: number;
  passphrase: string;
};

/** CAP-N11: the LAN ingest listener fields + the live connect URL/QR — shared
 * by the add-source form and the properties editor; state stays in the
 * caller. Rendered as siblings of the caller's column layout. */
export function LanIngestFields({
  value,
  onChange,
}: {
  value: LanIngestValue;
  onChange: (next: LanIngestValue) => void;
}) {
  const t = useT();
  const host = useLocalLanIp();
  const passUsable = lanPassphraseUsable(value.protocol, value.passphrase);
  const url = lanIngestUrl(
    value.protocol,
    host ?? "…",
    value.port,
    value.protocol === "srt" ? value.passphrase : "",
  );
  return (
    <>
      <div className="flex items-end gap-2">
        <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-lan-protocol-label")}
          <select
            value={value.protocol}
            onChange={(event) => {
              const protocol = event.target.value as IngestProtocol;
              // Follow the protocol's default port unless the user set one.
              const port =
                value.port === LAN_DEFAULT_PORTS[value.protocol]
                  ? LAN_DEFAULT_PORTS[protocol]
                  : value.port;
              onChange({ ...value, protocol, port });
            }}
            className={inputClass}
          >
            <option value="srt">{t("sources-lan-protocol-srt")}</option>
            <option value="rtmp">{t("sources-lan-protocol-rtmp")}</option>
          </select>
        </label>
        <NumberField
          label={t("sources-lan-port-label")}
          value={value.port}
          min={1024}
          max={65535}
          onCommit={(port) => onChange({ ...value, port })}
          className="flex-1"
        />
      </div>
      {value.protocol === "srt" && (
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-lan-passphrase-label")}
          <input
            value={value.passphrase}
            onChange={(event) => onChange({ ...value, passphrase: event.target.value })}
            className={`${inputClass} font-mono`}
          />
          <span className={passUsable ? "" : "text-amber-300"}>
            {t("sources-lan-passphrase-hint")}
          </span>
        </label>
      )}
      {value.protocol === "rtmp" ? (
        <p className="m-0 text-[10px] leading-snug text-amber-300">
          {t("sources-lan-rtmp-warning")}
        </p>
      ) : (
        value.passphrase === "" && (
          <p className="m-0 text-[10px] leading-snug text-amber-300">
            {t("sources-lan-open-warning")}
          </p>
        )
      )}
      <div className="flex items-start gap-3">
        <div className="min-w-0 flex-1">
          <p className="m-0 text-[11px] text-havoc-muted">{t("sources-lan-url-label")}</p>
          <p className="m-0 break-all font-mono text-xs text-havoc-text">{url}</p>
        </div>
        {host && <QrSvg link={url} ariaKey="sources-lan-qr-aria" />}
      </div>
    </>
  );
}
