import { useT } from "../i18n/t";
import { browserUrlValid } from "../lib/browserUrl";
import { NumberField } from "./NumberField";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

export type BrowserValue = {
  url: string;
  width: number;
  height: number;
  fps: number;
  transparent: boolean;
};

/** CAP-N77: the Browser source fields — shared by the add-source form and the
 * properties editor; state stays in the caller. Rendered as siblings of the
 * caller's column layout. */
export function BrowserFields({
  value,
  onChange,
}: {
  value: BrowserValue;
  onChange: (next: BrowserValue) => void;
}) {
  const t = useT();
  const urlOk = browserUrlValid(value.url);
  return (
    <>
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("sources-browser-url")}
        <input
          value={value.url}
          onChange={(event) => onChange({ ...value, url: event.target.value })}
          placeholder="https://example.com/overlay"
          className={`${inputClass} font-mono`}
        />
        {value.url.trim() !== "" && !urlOk && (
          <span className="text-amber-300">{t("sources-browser-url-invalid")}</span>
        )}
      </label>
      <div className="flex items-end gap-2">
        <NumberField
          label={t("sources-browser-width")}
          value={value.width}
          min={64}
          max={3840}
          onCommit={(width) => onChange({ ...value, width })}
          className="flex-1"
        />
        <NumberField
          label={t("sources-browser-height")}
          value={value.height}
          min={64}
          max={2160}
          onCommit={(height) => onChange({ ...value, height })}
          className="flex-1"
        />
        <NumberField
          label={t("sources-browser-fps")}
          value={value.fps}
          min={1}
          max={60}
          onCommit={(fps) => onChange({ ...value, fps })}
          className="flex-1"
        />
      </div>
      <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
        <input
          type="checkbox"
          checked={value.transparent}
          onChange={(event) => onChange({ ...value, transparent: event.target.checked })}
        />
        {t("sources-browser-transparent")}
      </label>
      <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("sources-browser-note")}</p>
    </>
  );
}
