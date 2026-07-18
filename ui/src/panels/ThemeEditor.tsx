import { open, save } from "@tauri-apps/plugin-dialog";

import { themeExport, themeImport } from "../api/commands";
import type { CustomPalette, ThemeSettings } from "../api/types";
import { DARK_PALETTE, contrastRatio } from "../theme/theme";
import { useT } from "../i18n/t";

const COLORS: Array<{ key: keyof CustomPalette; label: string }> = [
  { key: "bg", label: "theme-color-bg" },
  { key: "panel", label: "theme-color-panel" },
  { key: "text", label: "theme-color-text" },
  { key: "muted", label: "theme-color-muted" },
  { key: "accent", label: "theme-color-accent" },
  { key: "accent2", label: "theme-color-accent2" },
];

const FCTHEME_FILTER = { name: "Freally Capture theme", extensions: ["fctheme"] };

/**
 * CAP-N65: the visual theme editor — author the six palette colours with live
 * WCAG contrast checks and export/import as a `.fctheme` file. Applies live via
 * the appearance pane's existing preview (the parent updates `draft.theme`).
 */
export function ThemeEditor({
  theme,
  onChange,
}: {
  theme: ThemeSettings;
  onChange: (theme: ThemeSettings) => void;
}) {
  const t = useT();
  // A custom theme may carry only an accent (no full palette yet). Seed the
  // editor's accent swatch from that live accent so opening the editor — and then
  // tweaking any *other* colour — never silently reverts the user's accent to the
  // default blue.
  const palette: CustomPalette = theme.palette ?? {
    ...DARK_PALETTE,
    accent: theme.accent || DARK_PALETTE.accent,
  };
  const setColor = (key: keyof CustomPalette, value: string) =>
    onChange({ ...theme, palette: { ...palette, [key]: value } });

  const checks: Array<{ label: string; ratio: number; min: number }> = [
    { label: "theme-contrast-text-bg", ratio: contrastRatio(palette.text, palette.bg), min: 4.5 },
    {
      label: "theme-contrast-text-panel",
      ratio: contrastRatio(palette.text, palette.panel),
      min: 4.5,
    },
    { label: "theme-contrast-muted-bg", ratio: contrastRatio(palette.muted, palette.bg), min: 3 },
    { label: "theme-contrast-accent-bg", ratio: contrastRatio(palette.accent, palette.bg), min: 3 },
  ];

  const exportTheme = async () => {
    try {
      const dest = await save({ defaultPath: "custom.fctheme", filters: [FCTHEME_FILTER] });
      if (typeof dest === "string") await themeExport(dest, theme);
    } catch (err) {
      console.error("theme export failed:", err);
    }
  };
  const importTheme = async () => {
    try {
      const path = await open({ multiple: false, directory: false, filters: [FCTHEME_FILTER] });
      if (typeof path === "string") onChange(await themeImport(path));
    } catch (err) {
      console.error("theme import failed:", err);
    }
  };

  const smallButton =
    "shrink-0 rounded-md border border-white/10 px-2.5 py-1 text-[11px] text-havoc-text hover:border-havoc-accent/50";

  return (
    <div className="mt-2 flex flex-col gap-3 rounded-md border border-white/10 bg-white/[0.02] p-3">
      <p className="m-0 text-[11px] font-semibold tracking-wide text-havoc-muted uppercase">
        {t("theme-editor-title")}
      </p>
      <div className="grid grid-cols-2 gap-2">
        {COLORS.map(({ key, label }) => (
          // A plain <div>, not a <label>: the appearance pane already has an
          // "Accent" control, so the palette swatch's accessible name is scoped
          // (e.g. "Custom palette: Accent") to stay distinct from it — two
          // controls sharing the name "Accent" is an a11y (and test) ambiguity.
          <div key={key} className="flex items-center gap-2 text-[11px] text-havoc-text">
            <input
              type="color"
              value={palette[key]}
              onChange={(e) => setColor(key, e.target.value)}
              aria-label={`${t("theme-editor-title")}: ${t(label)}`}
              className="h-6 w-8 shrink-0 cursor-pointer rounded border border-white/10 bg-transparent p-0"
            />
            <span className="min-w-0 flex-1 truncate">{t(label)}</span>
            <span className="font-mono text-[10px] text-havoc-muted">{palette[key]}</span>
          </div>
        ))}
      </div>
      <div className="flex flex-col gap-1">
        {checks.map((check) => {
          const ok = check.ratio >= check.min;
          return (
            <p
              key={check.label}
              className={`m-0 text-[10px] ${ok ? "text-havoc-muted" : "text-amber-300"}`}
            >
              {t(check.label)}: {check.ratio.toFixed(1)}:1{" "}
              {ok ? "✓" : t("theme-contrast-low", { min: check.min })}
            </p>
          );
        })}
      </div>
      <div className="flex flex-wrap gap-2">
        <button
          type="button"
          onClick={() => onChange({ ...theme, palette: DARK_PALETTE })}
          className={smallButton}
        >
          {t("theme-reset")}
        </button>
        <button type="button" onClick={() => void exportTheme()} className={smallButton}>
          {t("theme-export")}
        </button>
        <button type="button" onClick={() => void importTheme()} className={smallButton}>
          {t("theme-import")}
        </button>
      </div>
    </div>
  );
}
