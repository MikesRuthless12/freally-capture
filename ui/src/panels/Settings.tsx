import { useEffect, useMemo, useRef, useState } from "react";

import { portableStatus, settingsGet, settingsSet, type PortableStatus } from "../api/commands";
import type {
  AccessibilitySettings,
  AlignmentSettings,
  MeterPreset,
  Settings,
  ThemeMode,
  WebPanelSettings,
} from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { AUTO_LOCALE, LOCALES, isLocaleCode } from "../i18n/locales";
import { initLocale, setLocale, useT } from "../i18n/t";
import { DEFAULT_METER_COLORS, meterGradient, resolveMeterColors } from "../lib/meters";
import { DEFAULT_LINK, DEFAULT_OSC, normalizeHotkeys } from "../lib/settingsDraft";
import { applyTheme } from "../theme/theme";
import { HotkeySettingsBody } from "./SettingsHotkeys";
import { OutputSettingsBody } from "./SettingsOutput";
import { LanServicesSections } from "./SettingsPanel";
import { RemoteApiSection } from "./SettingsRemote";
import { ReplaySettingsBody } from "./SettingsReplay";
import { StreamSettingsBody } from "./SettingsStream";
import { ThemeEditor } from "./ThemeEditor";
import { categoryMatches } from "./settingsSearch";

/** The sidebar, top to bottom. Every entry is a pane of REAL settings —
 * nothing decorative, nothing that doesn't persist. */
const CATEGORIES = [
  "general",
  "appearance",
  "streaming",
  "output",
  "replay",
  "hotkeys",
  "network",
  "accessibility",
  "about",
] as const;
export type CategoryId = (typeof CATEGORIES)[number];

const CATEGORY_LABELS: Record<CategoryId, string> = {
  general: "settings-cat-general",
  appearance: "settings-cat-appearance",
  streaming: "settings-cat-streaming",
  output: "settings-cat-output",
  replay: "settings-cat-replay",
  hotkeys: "settings-cat-hotkeys",
  network: "settings-cat-network",
  accessibility: "settings-cat-accessibility",
  about: "settings-cat-about",
};

/** Which Settings slices back each category, for the "recently changed" marker
 * (draft vs applied). The main fields per pane. */
const CATEGORY_FIELDS: Record<CategoryId, Array<keyof Settings>> = {
  general: ["language", "showStatsDock", "alignment"],
  appearance: ["theme"],
  streaming: ["stream"],
  output: ["recording"],
  replay: ["replay"],
  hotkeys: ["hotkeys"],
  network: ["webPanel", "osc", "midi", "remoteControl", "remote", "ptz", "link", "rundown"],
  accessibility: ["accessibility"],
  about: [],
};

/** Mirrors `AccessibilitySettings::default()` in settings.rs. */
const DEFAULT_ACCESSIBILITY: AccessibilitySettings = {
  meterPreset: "default",
  meterLow: DEFAULT_METER_COLORS.low,
  meterMid: DEFAULT_METER_COLORS.mid,
  meterHigh: DEFAULT_METER_COLORS.high,
};

/** Mirrors `WebPanelSettings::default()` (DEFAULT_PANEL_PORT) in webpanel.rs. */
const DEFAULT_WEB_PANEL: WebPanelSettings = {
  enabled: false,
  port: 4457,
  lan: false,
  password: "",
};

/** A draft every pane can edit without null-checks: the optional slices a
 * pre-existing settings file may lack get their Rust-side defaults. */
function withDefaults(settings: Settings): Settings {
  return {
    ...settings,
    accessibility: settings.accessibility ?? DEFAULT_ACCESSIBILITY,
    webPanel: settings.webPanel ?? DEFAULT_WEB_PANEL,
    osc: settings.osc ?? DEFAULT_OSC,
    link: settings.link ?? DEFAULT_LINK,
  };
}

type SettingsDialogProps = {
  settings: Settings;
  onSettingsSaved: (next: Settings) => void;
  onClose: () => void;
  /** Escape hatches to dialogs that live outside this modal: the full About
   * panel and the component (ffmpeg) installer. Opening one replaces this
   * dialog, so the draft is discarded first (same as Cancel). */
  onOpen: (dialog: "about" | "components") => void;
};

/**
 * The Settings modal, OBS-style (obs-chrome): a category sidebar, a grouped
 * scrollable pane, and an **OK / Cancel / Apply** footer.
 *
 * Everything edits ONE draft `Settings`, seeded from a fresh `settingsGet()`
 * at open (not the render prop — a stale snapshot would resurrect settings
 * another dialog saved meanwhile; server-owned fields like camera profiles
 * are re-applied by the store on save regardless). Nothing persists until
 * Apply/OK calls `settingsSet` with the whole draft — the same validation the
 * per-panel Save buttons hit, so an out-of-range value keeps the draft and
 * shows the error instead of saving.
 *
 * Two deliberate exceptions to "nothing happens until Apply":
 *   - Appearance previews live (a theme picker that doesn't show the theme is
 *     useless); Cancel/Escape puts the last APPLIED theme back.
 *   - Language applies only after a successful save — it re-renders the whole
 *     app, which is not a reversible "preview".
 */
export function SettingsDialog({
  settings,
  onSettingsSaved,
  onClose,
  onOpen,
}: SettingsDialogProps) {
  const t = useT();
  const [active, setActive] = useState<CategoryId>("general");
  const [search, setSearch] = useState("");
  const [draft, setDraft] = useState<Settings | null>(null);
  /** The last state known to be on disk — Apply's baseline, Cancel's target. */
  const [applied, setApplied] = useState<Settings | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [categoryErrors, setCategoryErrors] = useState<Partial<Record<CategoryId, string>>>({});
  const tabRefs = useRef<Partial<Record<CategoryId, HTMLButtonElement | null>>>({});
  // The prop as it was at open — the seed fallback if the fresh read fails.
  // A ref, NOT a dep: our own Apply updates the parent's `settings`, and
  // re-seeding from it would clobber whatever the user edited since.
  const fallback = useRef(settings);
  // `applied`, but readable from the unmount cleanup below. Updated
  // SYNCHRONOUSLY beside setApplied — an effect-mirrored ref can miss the
  // final value when OK's state update and the parent's close batch into one
  // commit, and the cleanup would then revert a theme the user just saved.
  const appliedRef = useRef<Settings | null>(null);

  useEffect(() => {
    let alive = true;
    const seed = (base: Settings) => {
      if (!alive) return;
      const seeded = withDefaults(base);
      setDraft(seeded);
      setApplied(seeded);
      appliedRef.current = seeded;
    };
    settingsGet()
      .then(seed)
      .catch(() => seed(fallback.current));
    return () => {
      alive = false;
    };
  }, []);

  // Appearance previews live. `applyTheme` ignores a malformed accent, so a
  // half-typed value can never reach a CSS declaration.
  useEffect(() => {
    if (draft) applyTheme(draft.theme);
  }, [draft?.theme.mode, draft?.theme.accent]); // eslint-disable-line react-hooks/exhaustive-deps -- keyed to the two theme fields, not the whole draft

  // Airtight revert: HOWEVER this modal leaves the tree — Cancel, Escape,
  // the ×, or an ancestor swapping dialogs — a theme preview that was never
  // applied must not survive it. (Re-applying the applied theme is a no-op.)
  useEffect(() => () => applyTheme((appliedRef.current ?? fallback.current).theme), []);

  const dirty = useMemo(
    () => draft !== null && applied !== null && JSON.stringify(draft) !== JSON.stringify(applied),
    [draft, applied],
  );

  /** Discard the draft: un-preview the theme, then leave. */
  const cancel = () => {
    applyTheme((appliedRef.current ?? fallback.current).theme);
    onClose();
  };

  /** Leaving for a sibling dialog discards the draft, same as Cancel. */
  const openSibling = (dialog: "about" | "components") => {
    applyTheme((appliedRef.current ?? fallback.current).theme);
    onOpen(dialog);
  };

  /** Save the whole draft. Returns whether it stuck (OK closes only then). */
  const apply = async (): Promise<boolean> => {
    if (!draft || !applied) return false;
    // A clean draft writes nothing — Apply is disabled then, but the guard
    // holds for any programmatic caller too.
    if (!dirty) return true;
    setError(null);
    // The same normalization the per-panel Saves always did.
    const next: Settings = {
      ...draft,
      hotkeys: normalizeHotkeys(draft.hotkeys),
      panicSlate: { color: draft.panicSlate.color.trim(), image: draft.panicSlate.image.trim() },
    };
    // The same client-side gate the old Remote dialog enforced. Anything else
    // is Rust's validate() — its rejection keeps the draft and shows below.
    const problems: Partial<Record<CategoryId, string>> = {};
    if (next.remoteControl.enabled && !next.remoteControl.password.trim()) {
      problems.network = t("remote-password-required");
    }
    setCategoryErrors(problems);
    const failing = CATEGORIES.find((category) => problems[category]);
    if (failing) {
      setActive(failing);
      return false;
    }
    try {
      await settingsSet(next);
    } catch (err) {
      setError(String(err));
      return false;
    }
    // Language lands only with a successful save (see the component docs).
    if (next.language !== applied.language) {
      if (next.language === AUTO_LOCALE) initLocale(AUTO_LOCALE);
      else if (isLocaleCode(next.language)) setLocale(next.language);
    }
    onSettingsSaved(next);
    // The baseline advances: Apply greys out again, and a LATER Cancel (or
    // the unmount revert) targets this applied state, not the open-time one.
    setApplied(next);
    appliedRef.current = next;
    setDraft(next);
    return true;
  };

  const confirm = () => {
    // A clean draft has nothing to save — OK is just a close.
    if (!dirty) {
      onClose();
      return;
    }
    void apply().then((ok) => {
      if (ok) onClose();
    });
  };

  // The tab list is filtered by the search box; keyboard nav and the roving
  // tabindex must stay within what's actually rendered.
  const visibleCategories = CATEGORIES.filter((category) =>
    categoryMatches(category, t(CATEGORY_LABELS[category]), search),
  );

  /** Roving-tabindex arrow navigation over the *visible* vertical tablist. */
  const onTabKeyDown = (event: React.KeyboardEvent) => {
    if (visibleCategories.length === 0) return;
    // If the active category was filtered out, start stepping from the top of the
    // visible list rather than jumping to a hidden (unfocusable) tab.
    const at = visibleCategories.indexOf(active);
    const from = at === -1 ? 0 : at;
    const count = visibleCategories.length;
    let next: CategoryId | undefined;
    if (event.key === "ArrowDown") next = visibleCategories[(from + 1) % count];
    else if (event.key === "ArrowUp") next = visibleCategories[(from + count - 1) % count];
    else if (event.key === "Home") next = visibleCategories[0];
    else if (event.key === "End") next = visibleCategories[count - 1];
    if (!next) return;
    event.preventDefault();
    setActive(next);
    tabRefs.current[next]?.focus();
  };

  const footerButton =
    "rounded-md border px-4 py-1.5 text-xs transition-colors disabled:cursor-not-allowed disabled:opacity-40";

  const changedByCategory = useMemo<Record<CategoryId, boolean>>(() => {
    const result = {} as Record<CategoryId, boolean>;
    for (const category of CATEGORIES) {
      result[category] =
        draft !== null &&
        applied !== null &&
        CATEGORY_FIELDS[category].some(
          (field) => JSON.stringify(draft[field]) !== JSON.stringify(applied[field]),
        );
    }
    return result;
  }, [draft, applied]);

  return (
    <PickerShell title={t("settings-title")} onClose={cancel} sidebar>
      <div className="flex min-h-0 min-w-0 flex-1 flex-col text-xs text-havoc-text">
        <div className="flex min-h-0 flex-1">
          <nav
            role="tablist"
            aria-orientation="vertical"
            aria-label={t("settings-categories")}
            className="flex w-44 shrink-0 flex-col gap-1 overflow-y-auto border-r border-white/5 p-2"
          >
            <input
              type="search"
              value={search}
              onChange={(event) => setSearch(event.target.value)}
              placeholder={t("settings-search-placeholder")}
              aria-label={t("settings-search-placeholder")}
              className="mb-1 w-full rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text outline-none focus:border-havoc-accent/60"
            />
            {visibleCategories.length === 0 ? (
              <p className="m-0 px-2 py-1 text-[11px] text-havoc-muted">
                {t("settings-search-none")}
              </p>
            ) : (
              visibleCategories.map((category) => (
                <button
                  key={category}
                  ref={(el) => {
                    tabRefs.current[category] = el;
                  }}
                  type="button"
                  role="tab"
                  id={`settings-tab-${category}`}
                  aria-selected={active === category}
                  aria-controls="settings-active-pane"
                  tabIndex={active === category ? 0 : -1}
                  onClick={() => setActive(category)}
                  onKeyDown={onTabKeyDown}
                  className={`flex items-center justify-between gap-2 rounded-md border px-3 py-2 text-left text-xs transition-colors ${
                    active === category
                      ? "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text"
                      : "border-transparent text-havoc-muted hover:bg-white/5 hover:text-havoc-text"
                  }`}
                >
                  <span className="flex items-center gap-1.5">
                    {t(CATEGORY_LABELS[category])}
                    {changedByCategory[category] && (
                      <span
                        title={t("settings-changed")}
                        aria-label={t("settings-changed")}
                        className="h-1.5 w-1.5 shrink-0 rounded-full bg-havoc-accent"
                      />
                    )}
                  </span>
                  {categoryErrors[category] && (
                    <span aria-hidden className="h-1.5 w-1.5 shrink-0 rounded-full bg-red-400" />
                  )}
                </button>
              ))
            )}
          </nav>

          <div
            role="tabpanel"
            id="settings-active-pane"
            aria-labelledby={`settings-tab-${active}`}
            tabIndex={0}
            className="min-h-0 min-w-0 flex-1 overflow-y-auto p-4"
          >
            {draft === null ? (
              <p className="m-0 text-havoc-muted">{t("settings-loading")}</p>
            ) : (
              <>
                {categoryErrors[active] && (
                  <p role="alert" className="m-0 mb-3 text-[11px] text-red-300">
                    {categoryErrors[active]}
                  </p>
                )}
                <CategoryPane
                  category={active}
                  draft={draft}
                  onChange={setDraft}
                  onOpenSibling={openSibling}
                />
              </>
            )}
          </div>
        </div>

        <footer className="flex items-center gap-3 border-t border-white/5 px-4 py-2.5">
          {error && (
            <p
              role="alert"
              title={error}
              className="m-0 min-w-0 flex-1 truncate text-[11px] text-red-300"
            >
              {error}
            </p>
          )}
          <div className="ml-auto flex shrink-0 gap-2">
            <button
              type="button"
              onClick={confirm}
              disabled={draft === null}
              className={`${footerButton} border-havoc-accent/60 bg-havoc-accent/15 font-semibold text-havoc-text hover:bg-havoc-accent/25`}
            >
              {t("settings-ok")}
            </button>
            <button
              type="button"
              onClick={cancel}
              className={`${footerButton} border-white/10 text-havoc-muted hover:text-havoc-text`}
            >
              {t("settings-cancel")}
            </button>
            <button
              type="button"
              onClick={() => void apply()}
              disabled={!dirty}
              className={`${footerButton} border-white/10 text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text`}
            >
              {t("settings-apply")}
            </button>
          </div>
        </footer>
      </div>
    </PickerShell>
  );
}

/** The active category's pane — every field edits the shared draft. */
function CategoryPane({
  category,
  draft,
  onChange,
  onOpenSibling,
}: {
  category: CategoryId;
  draft: Settings;
  onChange: (next: Settings) => void;
  onOpenSibling: (dialog: "about" | "components") => void;
}) {
  const t = useT();

  switch (category) {
    case "general":
      return (
        <div className="flex flex-col gap-4">
          <Section title={t("settings-language-section")}>
            <label className="flex items-center justify-between gap-3">
              <span className="text-havoc-muted">{t("settings-language")}</span>
              <select
                value={draft.language}
                onChange={(event) => onChange({ ...draft, language: event.target.value })}
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

          <Section title={t("settings-general-section")}>
            <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
              <input
                type="checkbox"
                checked={draft.showStatsDock}
                onChange={(event) => onChange({ ...draft, showStatsDock: event.target.checked })}
              />
              {t("settings-show-stats-dock")}
            </label>
          </Section>

          <Section title={t("settings-alignment-section")}>
            {(
              [
                ["smartGuides", "settings-smart-guides"],
                ["safeAreas", "settings-safe-areas"],
                ["rulers", "settings-rulers"],
              ] as Array<[keyof AlignmentSettings, string]>
            ).map(([key, label]) => (
              <label key={key} className="flex items-center gap-2 text-[11px] text-havoc-muted">
                <input
                  type="checkbox"
                  checked={draft.alignment[key]}
                  onChange={(event) =>
                    onChange({
                      ...draft,
                      alignment: { ...draft.alignment, [key]: event.target.checked },
                    })
                  }
                />
                {t(label)}
              </label>
            ))}
          </Section>
        </div>
      );

    case "appearance":
      return (
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
                aria-checked={draft.theme.mode === mode}
                onClick={() => onChange({ ...draft, theme: { ...draft.theme, mode } })}
                className={`rounded-md border px-3 py-1.5 text-xs transition-colors ${
                  draft.theme.mode === mode
                    ? "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text"
                    : "border-white/10 text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
                }`}
              >
                {t(`settings-theme-${mode}`)}
              </button>
            ))}
            {/* Only Custom spends the accent, so only Custom may set it. */}
            <label className="ml-auto flex items-center gap-2 text-[11px] text-havoc-muted">
              {t("settings-accent")}
              <input
                type="color"
                value={draft.theme.accent}
                onChange={(event) =>
                  onChange({ ...draft, theme: { ...draft.theme, accent: event.target.value } })
                }
                disabled={draft.theme.mode !== "custom"}
                aria-label={t("settings-accent")}
                className="h-6 w-10 cursor-pointer rounded border border-white/10 bg-transparent p-0 disabled:cursor-not-allowed disabled:opacity-40"
              />
            </label>
          </div>
          {draft.theme.mode === "custom" && (
            <ThemeEditor theme={draft.theme} onChange={(theme) => onChange({ ...draft, theme })} />
          )}
        </Section>
      );

    case "streaming":
      return (
        <StreamSettingsBody
          stream={draft.stream}
          onChange={(stream) => onChange({ ...draft, stream })}
          onOpenComponents={() => onOpenSibling("components")}
        />
      );

    case "output":
      return (
        <OutputSettingsBody
          recording={draft.recording}
          onPatch={(patch) => onChange({ ...draft, recording: { ...draft.recording, ...patch } })}
          onOpenComponents={() => onOpenSibling("components")}
        />
      );

    case "replay":
      return (
        <ReplaySettingsBody
          replay={draft.replay}
          onChange={(replay) => onChange({ ...draft, replay })}
        />
      );

    case "hotkeys":
      return (
        <HotkeySettingsBody
          hotkeys={draft.hotkeys}
          onChangeHotkeys={(hotkeys) => onChange({ ...draft, hotkeys })}
          slate={draft.panicSlate}
          onChangeSlate={(panicSlate) => onChange({ ...draft, panicSlate })}
        />
      );

    case "network":
      return (
        <div className="flex flex-col gap-4">
          <Section title={t("panel-title")}>
            <LanServicesSections
              webPanel={draft.webPanel ?? DEFAULT_WEB_PANEL}
              onChangeWebPanel={(webPanel) => onChange({ ...draft, webPanel })}
              osc={draft.osc ?? DEFAULT_OSC}
              onChangeOsc={(osc) => onChange({ ...draft, osc })}
              link={draft.link ?? DEFAULT_LINK}
              onChangeLink={(link) => onChange({ ...draft, link })}
            />
          </Section>
          <Section title={t("remote-title")}>
            <RemoteApiSection
              remoteControl={draft.remoteControl}
              onChange={(remoteControl) => onChange({ ...draft, remoteControl })}
            />
          </Section>
        </div>
      );

    case "accessibility":
      return (
        <AccessibilityPane
          accessibility={draft.accessibility ?? DEFAULT_ACCESSIBILITY}
          onChange={(accessibility) => onChange({ ...draft, accessibility })}
        />
      );

    case "about":
      return (
        <div className="flex flex-col gap-3">
          <p className="m-0 text-havoc-muted">{t("about-tagline")}</p>
          <p className="m-0 text-[11px] leading-snug text-havoc-muted">{t("about-local-first")}</p>
          <PortableInfo />
          <button
            type="button"
            onClick={() => onOpenSibling("about")}
            className="self-start rounded-md border border-white/10 bg-white/[0.04] px-3 py-1.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
          >
            {t("settings-open-about")}
          </button>
        </div>
      );
  }
}

/** Settings → About: portable-mode status + where app data is stored (CAP-N63). */
function PortableInfo() {
  const t = useT();
  const [status, setStatus] = useState<PortableStatus | null>(null);
  useEffect(() => {
    portableStatus()
      .then(setStatus)
      .catch(() => {});
  }, []);
  if (!status) return null;
  return (
    <p className="m-0 text-[11px] leading-snug text-havoc-muted">
      {status.portable
        ? t("about-portable-on")
        : t("about-portable-off", { marker: status.marker })}
      <br />
      <span className="break-all" title={status.dataDir}>
        {t("about-portable-data", { path: status.configDir })}
      </span>
    </p>
  );
}

/** Settings → Accessibility: the mixer VU meter palette (real, wired to the
 * mixer's meters through `resolveMeterColors` — see ChannelStrip.tsx). */
function AccessibilityPane({
  accessibility,
  onChange,
}: {
  accessibility: AccessibilitySettings;
  onChange: (next: AccessibilitySettings) => void;
}) {
  const t = useT();
  const colors = resolveMeterColors(accessibility);

  return (
    <Section title={t("settings-meter-section")}>
      <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("settings-meter-note")}</p>
      <div
        role="radiogroup"
        aria-label={t("settings-meter-preset")}
        className="flex flex-wrap items-center gap-2"
      >
        {(["default", "colorblind", "custom"] as MeterPreset[]).map((preset) => (
          <button
            key={preset}
            type="button"
            role="radio"
            aria-checked={accessibility.meterPreset === preset}
            onClick={() => onChange({ ...accessibility, meterPreset: preset })}
            className={`rounded-md border px-3 py-1.5 text-xs transition-colors ${
              accessibility.meterPreset === preset
                ? "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text"
                : "border-white/10 text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
            }`}
          >
            {t(`settings-meter-preset-${preset}`)}
          </button>
        ))}
      </div>
      {/* Only Custom reads these, so only Custom may set them (the accent-swatch rule). */}
      <div className="flex flex-wrap items-center gap-4">
        {(
          [
            ["meterLow", "settings-meter-low"],
            ["meterMid", "settings-meter-mid"],
            ["meterHigh", "settings-meter-high"],
          ] as Array<["meterLow" | "meterMid" | "meterHigh", string]>
        ).map(([key, label]) => (
          <label key={key} className="flex items-center gap-2 text-[11px] text-havoc-muted">
            {t(label)}
            <input
              type="color"
              value={accessibility[key]}
              onChange={(event) => onChange({ ...accessibility, [key]: event.target.value })}
              disabled={accessibility.meterPreset !== "custom"}
              aria-label={t(label)}
              className="h-6 w-10 cursor-pointer rounded border border-white/10 bg-transparent p-0 disabled:cursor-not-allowed disabled:opacity-40"
            />
          </label>
        ))}
      </div>
      <div className="flex flex-col gap-1">
        <span className="text-[10px] text-havoc-muted">{t("settings-meter-preview")}</span>
        <div
          aria-hidden
          className="h-2 w-full max-w-72 rounded-sm"
          style={{ background: meterGradient("to right", colors) }}
        />
      </div>
    </Section>
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
