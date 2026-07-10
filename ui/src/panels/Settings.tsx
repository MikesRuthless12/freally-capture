import { useState } from "react";

import { settingsSet } from "../api/commands";
import type { Settings, ThemeMode } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { AUTO_LOCALE, LOCALES, isLocaleCode } from "../i18n/locales";
import { initLocale, setLocale, useT } from "../i18n/t";
import { applyTheme, isHexColor } from "../theme/theme";

type SettingsDialogProps = {
  settings: Settings;
  onSettingsSaved: (next: Settings) => void;
  onClose: () => void;
  /** The unified modal is a hub: these open the panels that already exist. */
  onOpen: (dialog: "output" | "stream" | "hotkeys" | "replay" | "remote" | "about") => void;
};

/**
 * The unified Settings modal (TASK-906): the app-wide preferences that were
 * scattered across seven dialogs or missing entirely — **Language**,
 * **Appearance**, and the general UI knobs — plus one place to reach the
 * per-feature panels that stay where they are.
 *
 * Language and theme apply **live**, before the save round-trips. If the save
 * fails we roll both back, so the UI never shows a state the disk doesn't have.
 */
export function SettingsDialog({
  settings,
  onSettingsSaved,
  onClose,
  onOpen,
}: SettingsDialogProps) {
  const t = useT();
  const [error, setError] = useState<string | null>(null);

  const save = (next: Settings, rollback: () => void) => {
    setError(null);
    onSettingsSaved(next);
    settingsSet(next).catch((err) => {
      rollback();
      onSettingsSaved(settings);
      setError(String(err));
    });
  };

  const changeLanguage = (value: string) => {
    const previous = settings.language;
    // Apply first: a language picker that lags its own click feels broken.
    if (value === AUTO_LOCALE) initLocale(AUTO_LOCALE);
    else if (isLocaleCode(value)) setLocale(value);
    save({ ...settings, language: value }, () => initLocale(previous));
  };

  const changeTheme = (mode: ThemeMode) => {
    const previous = settings.theme;
    const theme = { ...settings.theme, mode };
    applyTheme(theme);
    save({ ...settings, theme }, () => applyTheme(previous));
  };

  const changeAccent = (accent: string) => {
    if (!isHexColor(accent)) return;
    const previous = settings.theme;
    // Choosing a colour implies wanting it — otherwise the picker does nothing
    // visible until you also flip the mode, which reads as a broken control.
    const theme = { mode: "custom" as ThemeMode, accent };
    applyTheme(theme);
    save({ ...settings, theme }, () => applyTheme(previous));
  };

  const changeStatsDock = (show: boolean) =>
    save({ ...settings, showStatsDock: show }, () => undefined);

  return (
    <PickerShell title={t("settings-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-4 text-xs text-havoc-text">
        <Section title={t("settings-language-section")}>
          <label className="flex items-center justify-between gap-3">
            <span className="text-havoc-muted">{t("settings-language")}</span>
            <select
              value={settings.language}
              onChange={(event) => changeLanguage(event.target.value)}
              className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text"
            >
              <option value={AUTO_LOCALE}>{t("settings-language-system")}</option>
              {LOCALES.map((locale) => (
                <option key={locale.code} value={locale.code}>
                  {locale.native}
                </option>
              ))}
            </select>
          </label>
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">
            {t("settings-language-note")}
          </p>
        </Section>

        <Section title={t("settings-appearance-section")}>
          <div
            role="radiogroup"
            aria-label={t("settings-theme")}
            className="flex flex-wrap items-center gap-2"
          >
            {(["dark", "light", "custom"] as ThemeMode[]).map((mode) => (
              <button
                key={mode}
                type="button"
                role="radio"
                aria-checked={settings.theme.mode === mode}
                onClick={() => changeTheme(mode)}
                className={`rounded-md border px-3 py-1.5 text-xs transition-colors ${
                  settings.theme.mode === mode
                    ? "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text"
                    : "border-white/10 text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
                }`}
              >
                {t(`settings-theme-${mode}`)}
              </button>
            ))}
            <label className="ml-auto flex items-center gap-2 text-[11px] text-havoc-muted">
              {t("settings-accent")}
              <input
                type="color"
                value={settings.theme.accent}
                onChange={(event) => changeAccent(event.target.value)}
                aria-label={t("settings-accent")}
                className="h-6 w-10 cursor-pointer rounded border border-white/10 bg-transparent p-0"
              />
            </label>
          </div>
        </Section>

        <Section title={t("settings-general-section")}>
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={settings.showStatsDock}
              onChange={(event) => changeStatsDock(event.target.checked)}
            />
            {t("settings-show-stats-dock")}
          </label>
        </Section>

        <Section title={t("settings-more-section")}>
          <div className="grid grid-cols-3 gap-2">
            <HubButton onClick={() => onOpen("output")}>{t("settings-open-output")}</HubButton>
            <HubButton onClick={() => onOpen("stream")}>{t("settings-open-stream")}</HubButton>
            <HubButton onClick={() => onOpen("replay")}>{t("settings-open-replay")}</HubButton>
            <HubButton onClick={() => onOpen("hotkeys")}>{t("settings-open-hotkeys")}</HubButton>
            <HubButton onClick={() => onOpen("remote")}>{t("settings-open-remote")}</HubButton>
            <HubButton onClick={() => onOpen("about")}>{t("settings-open-about")}</HubButton>
          </div>
        </Section>

        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}
      </div>
    </PickerShell>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <section className="flex flex-col gap-2">
      <h3 className="m-0 text-[10px] tracking-wide text-havoc-muted uppercase">{title}</h3>
      {children}
    </section>
  );
}

function HubButton({ onClick, children }: { onClick: () => void; children: React.ReactNode }) {
  return (
    <button
      type="button"
      onClick={onClick}
      className="rounded-md border border-white/10 bg-white/[0.04] px-2 py-1.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
    >
      {children}
    </button>
  );
}
