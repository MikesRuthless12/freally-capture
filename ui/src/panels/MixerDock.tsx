import { useEffect, useState } from "react";

import { audioOutputDevices, settingsSet, studioSetAudioHotkeys } from "../api/commands";
import type {
  AudioDevice,
  AudioLevelsPayload,
  Collection,
  Scene,
  Settings,
  Source,
  SourceId,
} from "../api/types";
import { kindHasAudio } from "../api/types";
import { AdvancedAudioFields, ChannelStrip } from "../components/ChannelStrip";
import { EmptyHint, Panel } from "../components/Panel";
import { PickerShell } from "../components/PickerShell";

const fail = (what: string) => (err: unknown) => console.error(`${what} failed:`, err);

type MixerDockProps = {
  collection: Collection | null;
  scene: Scene | null;
  audio: AudioLevelsPayload | null;
  settings: Settings | null;
  onSettingsSaved: (settings: Settings) => void;
  onOpenAudioFilters: (source: SourceId) => void;
};

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
  const [advancedFor, setAdvancedFor] = useState<SourceId | null>(null);

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

  return (
    <Panel
      title="Audio Mixer"
      actions={
        <div className="flex items-center gap-2">
          {audio?.monitorError && (
            <span
              role="alert"
              title={audio.monitorError}
              className="max-w-48 truncate text-[10px] text-amber-400"
            >
              monitor: {audio.monitorError}
            </span>
          )}
          {strips.length > 0 && settings && (
            <MonitorDevicePicker settings={settings} onSaved={onSettingsSaved} />
          )}
        </div>
      }
    >
      {strips.length === 0 ? (
        <EmptyHint>
          No audio sources in this scene — add an <b>Audio Input Capture</b> (mic) or{" "}
          <b>Audio Output Capture</b> (desktop audio) with “+” in Sources. Strips get a VU meter,
          fader, mute, monitoring, filters, and track assignment.
        </EmptyHint>
      ) : (
        <div className="flex h-full min-h-0 gap-2">
          <ul className="m-0 flex min-h-0 min-w-0 flex-1 list-none flex-col gap-1.5 overflow-auto p-0">
            {strips.map((source) => (
              <li key={source.id}>
                <ChannelStrip
                  source={source}
                  audio={
                    source.audio ?? {
                      volumeDb: 0,
                      muted: false,
                      monitor: "off",
                      tracks: 1,
                      syncOffsetMs: 0,
                      filters: [],
                    }
                  }
                  levels={audio?.sources[source.id]}
                  onOpenFilters={() => onOpenAudioFilters(source.id)}
                  onOpenAdvanced={() => setAdvancedFor(source.id)}
                />
              </li>
            ))}
          </ul>
          <LufsStrip audio={audio} />
        </div>
      )}

      {advancedSource && (
        <PickerShell title={`Audio — ${advancedSource.name}`} onClose={() => setAdvancedFor(null)}>
          <AdvancedAudioFields
            source={advancedSource}
            audio={
              advancedSource.audio ?? {
                volumeDb: 0,
                muted: false,
                monitor: "off",
                tracks: 1,
                syncOffsetMs: 0,
                filters: [],
              }
            }
            onSetHotkeys={(pushToTalk, pushToMute) => {
              studioSetAudioHotkeys(advancedSource.id, pushToTalk, pushToMute)
                .then(() => setAdvancedFor(null))
                .catch(fail("hotkey update"));
            }}
          />
        </PickerShell>
      )}
    </Panel>
  );
}

/** The program-loudness readout: momentary + short-term LUFS. */
function LufsStrip({ audio }: { audio: AudioLevelsPayload | null }) {
  const momentary = audio?.lufs.momentary;
  const shortTerm = audio?.lufs.shortTerm;
  const show = (value?: number) => (value === undefined ? "–" : value.toFixed(1));
  return (
    <div
      aria-label="Program loudness (LUFS)"
      className="flex w-20 shrink-0 flex-col items-center justify-center gap-1 rounded-lg border border-white/10 bg-white/[0.02] px-2 py-1.5"
    >
      <span className="text-[9px] tracking-wider text-havoc-muted uppercase">LUFS</span>
      <span
        title="Momentary loudness (400 ms)"
        className="text-lg leading-none font-semibold tabular-nums text-havoc-text"
      >
        {show(momentary)}
      </span>
      <span title="Short-term loudness (3 s)" className="text-[10px] tabular-nums text-havoc-muted">
        S {show(shortTerm)}
      </span>
    </div>
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
      Monitor
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
        aria-label="Monitor output device"
        className="max-w-40 rounded-md border border-white/10 bg-havoc-panel px-1.5 py-0.5 text-[10px] text-havoc-text"
      >
        <option value="">Default output</option>
        {(devices ?? []).map((device) => (
          <option key={device.id} value={device.id}>
            {device.name}
          </option>
        ))}
      </select>
    </label>
  );
}
