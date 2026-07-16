import { useEffect, useState } from "react";

import { audioOutputDevices, settingsSet, studioSetAudioTracks } from "../api/commands";
import type {
  AudioDevice,
  AudioLevelsPayload,
  AudioOutputRoute,
  OutputBus,
  Settings,
  Source,
} from "../api/types";
import { PickerShell } from "./PickerShell";
import { useT } from "../i18n/t";

const fail = (what: string) => (err: unknown) => console.error(`${what} failed:`, err);

/** The six track buses the mixer sums (bit 0 = Track 1). */
const TRACKS = 6;

/** A `<select>` value distinct from "" (a real route on the OS default) and
 * from any device id (which can never contain a NUL). */
const OFF = "\0off";

/** A stable identity string for a bus — two buses match iff their keys match. */
const busKey = (bus: OutputBus): string =>
  bus.bus === "master" ? "master" : `track${bus.index + 1}`;

/**
 * CAP-N30 routing matrix: which strips feed which track buses (the existing
 * `tracks` bitmask, edited as a grid), and where each program bus — master and
 * the six track buses — is sent physically. The monitor bus keeps its own
 * device (the mixer's monitor picker); routes here are additive, so with none
 * added the mix behaves exactly as before.
 */
export function RoutingMatrixDialog({
  strips,
  settings,
  audio,
  onSettingsSaved,
  onClose,
}: {
  strips: Source[];
  settings: Settings;
  audio: AudioLevelsPayload | null;
  onSettingsSaved: (settings: Settings) => void;
  onClose: () => void;
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

  const routes = settings.audioOutputs ?? [];
  const routableBuses: OutputBus[] = [
    { bus: "master" },
    ...Array.from({ length: TRACKS }, (_, index) => ({ bus: "track", index }) as OutputBus),
  ];

  const busLabel = (bus: OutputBus): string =>
    bus.bus === "master" ? t("routing-master") : t("routing-track", { n: bus.index + 1 });

  const toggleTrack = (source: Source, index: number) => {
    const current = source.audio?.tracks ?? 0;
    studioSetAudioTracks(source.id, current ^ (1 << index)).catch(fail("track assignment"));
  };

  const saveRoutes = (nextRoutes: AudioOutputRoute[]) => {
    const next: Settings = { ...settings, audioOutputs: nextRoutes };
    settingsSet(next)
      .then(() => onSettingsSaved(next))
      .catch(fail("output routing save"));
  };

  const setDevice = (bus: OutputBus, value: string) => {
    const key = busKey(bus);
    const others = routes.filter((route) => busKey(route) !== key);
    if (value === OFF) {
      saveRoutes(others);
      return;
    }
    const existing = routes.find((route) => busKey(route) === key);
    const route = { ...bus, deviceId: value, gainDb: existing?.gainDb ?? 0 } as AudioOutputRoute;
    saveRoutes([...others, route]);
  };

  const setTrim = (bus: OutputBus, gainDb: number) => {
    const key = busKey(bus);
    const existing = routes.find((route) => busKey(route) === key);
    if (!existing) return;
    const others = routes.filter((route) => busKey(route) !== key);
    saveRoutes([...others, { ...existing, gainDb }]);
  };

  return (
    <PickerShell title={t("routing-title")} onClose={onClose} wide>
      <div className="flex max-h-[70vh] flex-col gap-4 overflow-y-auto">
        <p className="text-xs text-havoc-muted">{t("routing-intro")}</p>

        {/* Strip → track-bus assignment grid. */}
        <section>
          <h3 className="mb-1 text-[11px] font-semibold tracking-wide text-havoc-text uppercase">
            {t("routing-sends-title")}
          </h3>
          {strips.length === 0 ? (
            <p className="text-xs text-havoc-muted">{t("routing-no-strips")}</p>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full border-collapse text-xs">
                <thead>
                  <tr className="text-havoc-muted">
                    <th className="px-2 py-1 text-left font-medium">{t("routing-source")}</th>
                    {Array.from({ length: TRACKS }, (_, index) => (
                      <th key={index} className="px-2 py-1 text-center font-medium">
                        {t("routing-track", { n: index + 1 })}
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {strips.map((source) => (
                    <tr key={source.id} className="border-t border-white/10">
                      <td
                        className="max-w-40 truncate px-2 py-1 text-havoc-text"
                        title={source.name}
                      >
                        {source.name}
                      </td>
                      {Array.from({ length: TRACKS }, (_, index) => {
                        const on = ((source.audio?.tracks ?? 0) & (1 << index)) !== 0;
                        return (
                          <td key={index} className="px-2 py-1 text-center">
                            <input
                              type="checkbox"
                              checked={on}
                              onChange={() => toggleTrack(source, index)}
                              aria-label={t("routing-send-aria", {
                                source: source.name,
                                n: index + 1,
                              })}
                            />
                          </td>
                        );
                      })}
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </section>

        {/* Program bus → physical output. */}
        <section>
          <h3 className="mb-1 text-[11px] font-semibold tracking-wide text-havoc-text uppercase">
            {t("routing-outputs-title")}
          </h3>
          <ul className="m-0 flex list-none flex-col gap-1.5 p-0">
            {routableBuses.map((bus) => {
              const route = routes.find((candidate) => busKey(candidate) === busKey(bus)) ?? null;
              const error = audio?.outputErrors?.find((entry) => entry.bus === busKey(bus));
              return (
                <li
                  key={busKey(bus)}
                  className="flex flex-wrap items-center gap-2 rounded-md border border-white/10 bg-white/[0.02] px-2 py-1.5"
                >
                  <span className="w-16 shrink-0 text-xs text-havoc-text">{busLabel(bus)}</span>
                  <select
                    value={route ? route.deviceId : OFF}
                    onChange={(event) => setDevice(bus, event.target.value)}
                    aria-label={t("routing-device-aria", { bus: busLabel(bus) })}
                    className="min-w-40 flex-1 rounded-md border border-white/10 bg-havoc-panel px-1.5 py-0.5 text-[11px] text-havoc-text"
                  >
                    <option value={OFF}>{t("routing-off")}</option>
                    <option value="">{t("routing-default-output")}</option>
                    {(devices ?? []).map((device) => (
                      <option key={device.id} value={device.id}>
                        {device.name}
                      </option>
                    ))}
                  </select>
                  {route && (
                    <label className="flex items-center gap-1 text-[10px] text-havoc-muted">
                      <input
                        type="range"
                        min={-60}
                        max={6}
                        step={0.5}
                        value={route.gainDb}
                        onChange={(event) => setTrim(bus, Number(event.target.value))}
                        aria-label={t("routing-trim-aria", { bus: busLabel(bus) })}
                        className="w-24"
                      />
                      <span className="w-12 text-right tabular-nums">
                        {route.gainDb <= -60
                          ? t("routing-muted")
                          : t("routing-trim-db", { db: route.gainDb.toFixed(1) })}
                      </span>
                    </label>
                  )}
                  {error && (
                    <span role="alert" title={error.message} className="text-[10px] text-amber-400">
                      {t("routing-device-error")}
                    </span>
                  )}
                </li>
              );
            })}
          </ul>
        </section>
      </div>
    </PickerShell>
  );
}
