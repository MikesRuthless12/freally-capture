import type { Rgba, SocialPlatform, SocialRow } from "../api/types";
import { newSocialRow } from "../lib/sourceOptions";
import { useT } from "../i18n/t";
import { hexToRgba, rgbaToHex } from "../lib/color";
import { SOCIAL_PLATFORMS } from "../lib/sourceOptions";
import { NumberField } from "./NumberField";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/** V1-D: the editable social-bar fields, shared by the Add-Source form and the
 * Properties dialog so both stay in lockstep. `kind` lives on the callers. */
export type SocialBarValue = {
  header: string;
  rows: SocialRow[];
  fontFamily?: string | null;
  sizePx: number;
  color: Rgba;
  background: Rgba;
};

/** Swap a row with the one after it (the ↑/↓ reorder buttons). */
function swap(rows: SocialRow[], at: number): SocialRow[] {
  const next = [...rows];
  [next[at], next[at + 1]] = [next[at + 1], next[at]];
  return next;
}

/** Header + type size + colours + the add/remove/reorder account-rows editor.
 * `platform` picks a bundled brand (colour + name baked into the renderer) or
 * `custom` (the row supplies its own label + colour). */
export function SocialBarFields({
  value,
  onChange,
}: {
  value: SocialBarValue;
  onChange: (next: SocialBarValue) => void;
}) {
  const t = useT();
  const setRow = (index: number, next: SocialRow) =>
    onChange({ ...value, rows: value.rows.map((row, at) => (at === index ? next : row)) });

  return (
    <div className="flex flex-col gap-2">
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("sources-social-header")}
        <input
          value={value.header}
          onChange={(event) => onChange({ ...value, header: event.target.value })}
          placeholder={t("sources-social-header-placeholder")}
          className={inputClass}
        />
      </label>
      <div className="flex items-end gap-2">
        <NumberField
          label={t("sources-social-size")}
          value={value.sizePx}
          min={8}
          max={200}
          onCommit={(sizePx) => onChange({ ...value, sizePx })}
          className="w-24"
        />
        <label className="flex items-center gap-1 pb-1 text-[11px] text-havoc-muted">
          {t("sources-social-text-color")}
          <input
            type="color"
            value={rgbaToHex(value.color)}
            onChange={(event) =>
              onChange({ ...value, color: hexToRgba(event.target.value, value.color.a) })
            }
            aria-label={t("sources-social-text-color")}
            className="h-7 w-9 cursor-pointer rounded border border-white/10 bg-transparent"
          />
        </label>
        <label className="flex items-center gap-1 pb-1 text-[11px] text-havoc-muted">
          {t("sources-social-bg-color")}
          <input
            type="color"
            value={rgbaToHex(value.background)}
            onChange={(event) =>
              onChange({ ...value, background: hexToRgba(event.target.value, value.background.a) })
            }
            aria-label={t("sources-social-bg-color")}
            className="h-7 w-9 cursor-pointer rounded border border-white/10 bg-transparent"
          />
        </label>
        <label className="flex flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-social-opacity")}
          <input
            type="range"
            min={0}
            max={255}
            value={value.background.a}
            onChange={(event) =>
              onChange({
                ...value,
                background: { ...value.background, a: Number(event.target.value) },
              })
            }
            aria-label={t("sources-social-opacity")}
          />
        </label>
      </div>
      <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
        {t("sources-social-font")}
        <input
          value={value.fontFamily ?? ""}
          onChange={(event) =>
            onChange({ ...value, fontFamily: event.target.value.trim() || null })
          }
          placeholder={t("sources-social-font-placeholder")}
          className={inputClass}
        />
      </label>
      <span className="text-[11px] text-havoc-muted">{t("sources-social-accounts")}</span>
      <div className="flex max-h-72 flex-col gap-2 overflow-y-auto pr-1">
        {value.rows.length === 0 && (
          <p className="m-0 text-[10px] text-havoc-muted">{t("sources-social-empty")}</p>
        )}
        {value.rows.map((row, index) => (
          <div key={index} className="flex flex-col gap-1 rounded-md border border-white/10 p-2">
            <div className="flex items-center gap-1">
              <select
                value={row.platform}
                onChange={(event) =>
                  setRow(index, { ...row, platform: event.target.value as SocialPlatform })
                }
                aria-label={t("sources-social-platform")}
                className={`${inputClass} w-28`}
              >
                {SOCIAL_PLATFORMS.map(([platform, label]) => (
                  <option key={platform} value={platform}>
                    {platform === "custom" ? t("sources-social-custom") : label}
                  </option>
                ))}
              </select>
              <input
                value={row.handle}
                onChange={(event) => setRow(index, { ...row, handle: event.target.value })}
                placeholder={t("sources-social-handle-placeholder")}
                aria-label={t("sources-social-handle")}
                className={`${inputClass} min-w-0 flex-1`}
              />
              <button
                type="button"
                disabled={index === 0}
                onClick={() => onChange({ ...value, rows: swap(value.rows, index - 1) })}
                aria-label={t("sources-social-up")}
                className="rounded border border-white/10 px-1.5 py-1 text-[11px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
              >
                ↑
              </button>
              <button
                type="button"
                disabled={index === value.rows.length - 1}
                onClick={() => onChange({ ...value, rows: swap(value.rows, index) })}
                aria-label={t("sources-social-down")}
                className="rounded border border-white/10 px-1.5 py-1 text-[11px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
              >
                ↓
              </button>
              <button
                type="button"
                onClick={() =>
                  onChange({ ...value, rows: value.rows.filter((_, at) => at !== index) })
                }
                aria-label={t("sources-social-remove")}
                className="rounded border border-white/10 px-1.5 py-1 text-[11px] text-havoc-muted hover:text-red-400"
              >
                ×
              </button>
            </div>
            {row.platform === "custom" && (
              <div className="flex items-center gap-2">
                <input
                  value={row.label}
                  onChange={(event) => setRow(index, { ...row, label: event.target.value })}
                  placeholder={t("sources-social-custom-label")}
                  aria-label={t("sources-social-custom-label")}
                  className={`${inputClass} min-w-0 flex-1`}
                />
                <label className="flex items-center gap-1 text-[11px] text-havoc-muted">
                  {t("sources-social-custom-color")}
                  <input
                    type="color"
                    value={rgbaToHex(row.color)}
                    onChange={(event) =>
                      setRow(index, { ...row, color: hexToRgba(event.target.value, row.color.a) })
                    }
                    aria-label={t("sources-social-custom-color")}
                    className="h-7 w-9 cursor-pointer rounded border border-white/10 bg-transparent"
                  />
                </label>
              </div>
            )}
          </div>
        ))}
      </div>
      <button
        type="button"
        onClick={() => onChange({ ...value, rows: [...value.rows, newSocialRow()] })}
        className="self-start rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:text-havoc-text"
      >
        {t("sources-social-add-row")}
      </button>
      <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("sources-social-note")}</p>
    </div>
  );
}
