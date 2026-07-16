import { useEffect, useState } from "react";

import { audioOutputDevices, settingsSet, studioSetAudioHotkeys } from "../api/commands";
import type {
  AudioDevice,
  AudioLevelsPayload,
  Collection,
  MixerLayout,
  Scene,
  Settings,
  Source,
  SourceId,
} from "../api/types";
import { kindHasAudio } from "../api/types";
import { AdvancedAudioFields, ChannelStrip } from "../components/ChannelStrip";
import { resolveMeterColors } from "../lib/meters";
import { EmptyHint, Panel } from "../components/Panel";
import { PickerShell } from "../components/PickerShell";
import { RoutingMatrixDialog } from "../components/RoutingMatrixDialog";
import { DuckingMatrixDialog } from "../components/DuckingMatrixDialog";
import { SoundboardDialog } from "../components/SoundboardDialog";
import { PluginsDialog } from "../components/PluginsDialog";
import { useT } from "../i18n/t";

const fail = (what: string) => (err: unknown) => console.error(`${what} failed:`, err);

type MixerDockProps = {
  collection: Collection | null;
  scene: Scene | null;
  audio: AudioLevelsPayload | null;
  settings: Settings | null;
  onSettingsSaved: (settings: Settings) => void;
  onOpenAudioFilters: (source: SourceId) => void;
};

/** A dialog-opening toolbar button in the mixer header (uniform styling). */
function ToolButton({
  onClick,
  label,
  title,
}: {
  onClick: () => void;
  label: string;
  title: string;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      title={title}
      aria-label={title}
      className="rounded-md border border-white/10 px-1.5 py-0.5 text-xs text-havoc-muted transition-colors hover:border-havoc-accent/50 hover:text-havoc-text"
    >
      {label}
    </button>
  );
}

/**
 * The Audio Mixer dock: one channel strip per audio source in the active
 * scene (visible items), the program LUFS meter, and the monitor-device
 * picker. Live levels ride the ~20 Hz `audio` event.
 */
export function MixerDock({
  collection,
  scene,
  audio,
  settings,
  onSettingsSaved,
  onOpenAudioFilters,
}: MixerDockProps) {
  const t = useT();
  const [advancedFor, setAdvancedFor] = useState<SourceId | null>(null);
  const [routingOpen, setRoutingOpen] = useState(false);
  const [duckingOpen, setDuckingOpen] = useState(false);
  const [soundboardOpen, setSoundboardOpen] = useState(false);
  const [pluginsOpen, setPluginsOpen] = useState(false);

  // Audio sources shown: the active scene's visible items, deduped by
  // source (a source shared into the scene twice is still one strip).
  const strips: Source[] = [];
  if (collection && scene) {
    const seen = new Set<SourceId>();
    for (const item of scene.items) {
      if (!item.visible || seen.has(item.source)) continue;
      const source = collection.sources.find((candidate) => candidate.id === item.source);
      if (!source || !kindHasAudio(source.kind)) continue;
      seen.add(item.source);
      strips.push(source);
    }
  }

  const advancedSource =
    advancedFor === null ? null : (strips.find((source) => source.id === advancedFor) ?? null);

  const layout: MixerLayout = settings?.mixerLayout ?? "horizontal";
  const vertical = layout === "vertical";
  // Settings → Accessibility: the meter palette (default / colorblind / custom).
  const meterColors = resolveMeterColors(settings?.accessibility);
  const defaultStrip = {
    volumeDb: 0,
    muted: false,
    monitor: "off" as const,
    tracks: 1,
    pan: 0,
    solo: false,
    mono: false,
    syncOffsetMs: 0,
    filters: [],
  };

  const toggleLayout = () => {
    if (!settings) return;
    const next: Settings = { ...settings, mixerLayout: vertical ? "horizontal" : "vertical" };
    settingsSet(next)
      .then(() => onSettingsSaved(next))
      .catch(fail("mixer layout save"));
  };

  return (
    <Panel
      title={t("mixer-title")}
      actions={
        <div className="flex items-center gap-2">
          {audio?.monitorError && (
            <span
              role="alert"
              title={audio.monitorError}
              className="max-w-48 truncate text-[10px] text-amber-400"
            >
              {t("mixer-monitor-error", { error: audio.monitorError })}
            </span>
          )}
          {settings && (
            <button
              type="button"
              onClick={toggleLayout}
              title={vertical ? t("mixer-switch-to-horizontal") : t("mixer-switch-to-vertical")}
              aria-label={
                vertical ? t("mixer-layout-aria-vertical") : t("mixer-layout-aria-horizontal")
              }
              className="rounded-md border border-white/10 px-1.5 py-0.5 text-xs text-havoc-muted transition-colors hover:border-havoc-accent/50 hover:text-havoc-text"
            >
              {vertical ? "▭" : "▯"}
            </button>
          )}
          {settings && (
            <ToolButton
              onClick={() => setRoutingOpen(true)}
              title={t("mixer-routing-title")}
              label={t("mixer-routing")}
            />
          )}
          {settings && (
            <ToolButton
              onClick={() => setSoundboardOpen(true)}
              title={t("mixer-soundboard-title")}
              label={t("mixer-soundboard")}
            />
          )}
          <ToolButton
            onClick={() => setPluginsOpen(true)}
            title={t("mixer-plugins-title")}
            label={t("mixer-plugins")}
          />
          {strips.length > 0 && (
            <ToolButton
              onClick={() => setDuckingOpen(true)}
              title={t("mixer-ducking-title")}
              label={t("mixer-ducking")}
            />
          )}
          {strips.length > 0 && settings && (
            <MonitorDevicePicker settings={settings} onSaved={onSettingsSaved} />
          )}
        </div>
      }
    >
      {strips.length === 0 ? (
        <EmptyHint>{t("mixer-empty")}</EmptyHint>
      ) : vertical ? (
        <div className="flex h-full min-h-0 gap-2">
          <ul className="m-0 flex min-h-0 min-w-0 flex-1 list-none flex-row gap-1.5 overflow-x-auto p-0">
            {strips.map((source) => (
              <li key={source.id} className="h-full">
                <ChannelStrip
                  source={source}
                  audio={source.audio ?? defaultStrip}
                  levels={audio?.sources[source.id]}
                  orientation="vertical"
                  meterColors={meterColors}
                  sceneId={scene?.id ?? null}
                  sceneOverride={
                    scene?.audioOverrides?.find((entry) => entry.source === source.id) ?? null
                  }
                  onOpenFilters={() => onOpenAudioFilters(source.id)}
                  onOpenAdvanced={() => setAdvancedFor(source.id)}
                />
              </li>
            ))}
          </ul>
          <LufsStrip audio={audio} settings={settings} onSettingsSaved={onSettingsSaved} />
        </div>
      ) : (
        <div className="flex h-full min-h-0 gap-2">
          <ul className="m-0 flex min-h-0 min-w-0 flex-1 list-none flex-col gap-1.5 overflow-auto p-0">
            {strips.map((source) => (
              <li key={source.id}>
                <ChannelStrip
                  source={source}
                  audio={source.audio ?? defaultStrip}
                  levels={audio?.sources[source.id]}
                  meterColors={meterColors}
                  sceneId={scene?.id ?? null}
                  sceneOverride={
                    scene?.audioOverrides?.find((entry) => entry.source === source.id) ?? null
                  }
                  onOpenFilters={() => onOpenAudioFilters(source.id)}
                  onOpenAdvanced={() => setAdvancedFor(source.id)}
                />
              </li>
            ))}
          </ul>
          <LufsStrip audio={audio} settings={settings} onSettingsSaved={onSettingsSaved} />
        </div>
      )}

      {advancedSource && (
        <PickerShell
          title={t("mixer-advanced-title", { name: advancedSource.name })}
          onClose={() => setAdvancedFor(null)}
        >
          <AdvancedAudioFields
            source={advancedSource}
            audio={advancedSource.audio ?? defaultStrip}
            onSetHotkeys={(pushToTalk, pushToMute) => {
              studioSetAudioHotkeys(advancedSource.id, pushToTalk, pushToMute)
                .then(() => setAdvancedFor(null))
                .catch(fail("hotkey update"));
            }}
          />
        </PickerShell>
      )}

      {routingOpen && settings && (
        <RoutingMatrixDialog
          strips={strips}
          settings={settings}
          audio={audio}
          onSettingsSaved={onSettingsSaved}
          onClose={() => setRoutingOpen(false)}
        />
      )}

      {duckingOpen && <DuckingMatrixDialog strips={strips} onClose={() => setDuckingOpen(false)} />}

      {soundboardOpen && settings && (
        <SoundboardDialog
          settings={settings}
          onSettingsSaved={onSettingsSaved}
          onClose={() => setSoundboardOpen(false)}
        />
      )}

      {pluginsOpen && <PluginsDialog onClose={() => setPluginsOpen(false)} />}
    </Panel>
  );
}

/** The program-loudness readout: momentary + short-term LUFS + the CAP-N34
 * normalization control. */
function LufsStrip({
  audio,
  settings,
  onSettingsSaved,
}: {
  audio: AudioLevelsPayload | null;
  settings: Settings | null;
  onSettingsSaved: (settings: Settings) => void;
}) {
  const t = useT();
  const [loudnessOpen, setLoudnessOpen] = useState(false);
  const momentary = audio?.lufs.momentary;
  const shortTerm = audio?.lufs.shortTerm;
  const show = (value?: number) => (value === undefined ? "–" : value.toFixed(1));
  const riding = settings?.loudness?.enabled ?? false;
  return (
    <div
      aria-label={t("mixer-loudness-label")}
      className="flex w-20 shrink-0 flex-col items-center justify-center gap-1 rounded-lg border border-white/10 bg-white/[0.02] px-2 py-1.5"
    >
      <span className="text-[9px] tracking-wider text-havoc-muted uppercase">
        {t("mixer-lufs")}
      </span>
      <span
        title={t("mixer-momentary-title")}
        className="text-lg leading-none font-semibold tabular-nums text-havoc-text"
      >
        {show(momentary)}
      </span>
      <span
        title={t("mixer-short-term-title")}
        className="text-[10px] tabular-nums text-havoc-muted"
      >
        {t("mixer-lufs-short", { value: show(shortTerm) })}
      </span>
      {settings && (
        <button
          type="button"
          onClick={() => setLoudnessOpen(true)}
          title={t("loudness-title")}
          className={`rounded px-1 text-[9px] tracking-wide uppercase transition-colors ${
            riding ? "text-havoc-accent" : "text-havoc-muted hover:text-havoc-text"
          }`}
        >
          {riding
            ? t("loudness-on", { target: settings.loudness?.targetLufs ?? -16 })
            : t("loudness-off")}
        </button>
      )}
      {loudnessOpen && settings && (
        <LoudnessDialog
          settings={settings}
          onSettingsSaved={onSettingsSaved}
          onClose={() => setLoudnessOpen(false)}
        />
      )}
    </div>
  );
}

const LOUDNESS_TARGETS = [-14, -16, -23];

/** CAP-N34: the live loudness-rider control (enable, target, ceiling). */
function LoudnessDialog({
  settings,
  onSettingsSaved,
  onClose,
}: {
  settings: Settings;
  onSettingsSaved: (settings: Settings) => void;
  onClose: () => void;
}) {
  const t = useT();
  const loudness = settings.loudness ?? { enabled: false, targetLufs: -16, ceilingDb: -1 };
  const save = (patch: Partial<NonNullable<Settings["loudness"]>>) => {
    const next: Settings = { ...settings, loudness: { ...loudness, ...patch } };
    settingsSet(next)
      .then(() => onSettingsSaved(next))
      .catch(fail("loudness save"));
  };
  return (
    <PickerShell title={t("loudness-title")} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-havoc-muted">{t("loudness-intro")}</p>
        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={loudness.enabled}
            onChange={(event) => save({ enabled: event.target.checked })}
          />
          {t("loudness-enable")}
        </label>
        <label className="flex items-center justify-between gap-2">
          {t("loudness-target")}
          <select
            value={loudness.targetLufs}
            onChange={(event) => save({ targetLufs: Number(event.target.value) })}
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text"
          >
            {LOUDNESS_TARGETS.map((target) => (
              <option key={target} value={target}>
                {t("loudness-target-option", { target })}
              </option>
            ))}
          </select>
        </label>
        <label className="flex items-center justify-between gap-2">
          {t("loudness-ceiling")}
          <input
            type="number"
            min={-9}
            max={0}
            step={0.5}
            value={loudness.ceilingDb}
            onChange={(event) =>
              save({ ceilingDb: Math.min(0, Math.max(-9, Number(event.target.value))) })
            }
            className="w-20 rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text"
          />
        </label>
        <p className="m-0 text-[10px] text-havoc-muted">{t("loudness-note")}</p>
      </div>
    </PickerShell>
  );
}

/** The monitor output picker (persists in settings; "" = the OS default). */
function MonitorDevicePicker({
  settings,
  onSaved,
}: {
  settings: Settings;
  onSaved: (settings: Settings) => void;
}) {
  const t = useT();
  const [devices, setDevices] = useState<AudioDevice[] | null>(null);

  useEffect(() => {
    let cancelled = false;
    audioOutputDevices()
      .then((list) => {
        if (!cancelled) setDevices(list);
      })
      .catch(() => {
        if (!cancelled) setDevices([]);
      });
    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <label className="flex items-center gap-1 text-[10px] text-havoc-muted">
      {t("mixer-monitor-label")}
      <select
        value={settings.monitorDevice ?? ""}
        onChange={(event) => {
          const next: Settings = {
            ...settings,
            monitorDevice: event.target.value || null,
          };
          settingsSet(next)
            .then(() => onSaved(next))
            .catch(fail("monitor device save"));
        }}
        aria-label={t("mixer-monitor-device-aria")}
        className="max-w-40 rounded-md border border-white/10 bg-havoc-panel px-1.5 py-0.5 text-[10px] text-havoc-text"
      >
        <option value="">{t("mixer-default-output")}</option>
        {(devices ?? []).map((device) => (
          <option key={device.id} value={device.id}>
            {device.name}
          </option>
        ))}
      </select>
    </label>
  );
}
