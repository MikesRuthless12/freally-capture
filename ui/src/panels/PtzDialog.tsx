import { useState } from "react";

import { ptzMove, ptzPresetRecall, ptzPresetStore, ptzZoom, settingsSet } from "../api/commands";
import type { PtzCamera, PtzMoveDirection, Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

const padClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-2 text-sm hover:border-havoc-accent/50 active:bg-havoc-accent/20";

/** The 3×3 pad, in grid order. `null` = the centre (stop). */
const PAD: Array<PtzMoveDirection | null> = [
  "upLeft",
  "up",
  "upRight",
  "left",
  null,
  "right",
  "downLeft",
  "down",
  "downRight",
];

const ARROWS: Record<PtzMoveDirection, string> = {
  up: "▲",
  down: "▼",
  left: "◀",
  right: "▶",
  upLeft: "◤",
  upRight: "◥",
  downLeft: "◣",
  downRight: "◢",
  stop: "■",
};

/**
 * PTZ camera control (CAP-N08): VISCA-over-IP pan/tilt/zoom, named presets,
 * and per-scene auto-recall.
 *
 * LAN-only and explicit: the app talks to a camera only because its address
 * was typed here. Press-and-hold drives the head; releasing stops it (VISCA
 * heads keep moving until told to stop, so the release matters).
 */
export function PtzDialog({
  settings,
  sceneNames,
  onSaved,
  onClose,
}: {
  settings: Settings | null;
  sceneNames: string[];
  onSaved: (next: Settings) => void;
  onClose: () => void;
}) {
  const t = useT();
  const [error, setError] = useState<string | null>(null);
  const [speed, setSpeed] = useState(8);
  const [selected, setSelected] = useState(0);

  if (!settings) return null;
  const ptz = settings.ptz ?? { cameras: [] };
  const camera: PtzCamera | undefined = ptz.cameras[selected];

  const persist = (cameras: PtzCamera[]) => {
    const next = { ...settings, ptz: { cameras } };
    setError(null);
    settingsSet(next)
      .then(() => onSaved(next))
      .catch((err) => setError(String(err)));
  };

  const update = (patch: Partial<PtzCamera>) =>
    persist(ptz.cameras.map((entry, at) => (at === selected ? { ...entry, ...patch } : entry)));

  const fail = (err: unknown) => setError(String(err));
  const drive = (direction: PtzMoveDirection) => {
    if (!camera) return;
    ptzMove(camera.name, direction, speed, speed).catch(fail);
  };

  return (
    <PickerShell title={t("ptz-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-havoc-muted">{t("ptz-about")}</p>
        {error && <p className="m-0 text-red-400">{error}</p>}

        <div className="flex flex-wrap items-center gap-2">
          <select
            value={selected}
            onChange={(event) => setSelected(Number(event.target.value))}
            aria-label={t("ptz-camera")}
            className={inputClass}
          >
            {ptz.cameras.map((entry, index) => (
              <option key={index} value={index}>
                {entry.name}
              </option>
            ))}
            {ptz.cameras.length === 0 && <option value={0}>{t("ptz-none")}</option>}
          </select>
          <button
            type="button"
            onClick={() =>
              persist([
                ...ptz.cameras,
                {
                  name: t("ptz-new-camera"),
                  host: "192.168.1.50",
                  port: 52381,
                  presets: [],
                  sceneRecalls: [],
                },
              ])
            }
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2 py-1 font-semibold"
          >
            {t("ptz-add-camera")}
          </button>
          {camera && (
            <button
              type="button"
              onClick={() => {
                persist(ptz.cameras.filter((_, at) => at !== selected));
                setSelected(0);
              }}
              className="rounded-md border border-white/10 px-2 py-1 text-havoc-muted hover:border-red-400/50 hover:text-red-400"
            >
              {t("ptz-remove-camera")}
            </button>
          )}
        </div>

        {camera && (
          <>
            <div className="flex flex-wrap items-center gap-2">
              <input
                value={camera.name}
                onChange={(event) => update({ name: event.target.value })}
                aria-label={t("ptz-camera-name")}
                className={`${inputClass} w-32`}
              />
              <input
                value={camera.host}
                onChange={(event) => update({ host: event.target.value })}
                placeholder="192.168.1.50"
                aria-label={t("ptz-host")}
                className={`${inputClass} w-40`}
              />
              <input
                type="number"
                min={1024}
                max={65535}
                value={camera.port}
                onChange={(event) => update({ port: Number(event.target.value) || camera.port })}
                aria-label={t("ptz-port")}
                className={`${inputClass} w-24`}
              />
            </div>

            <div className="flex flex-wrap items-start gap-4">
              {/* The pad: press-and-hold drives, release stops. */}
              <div className="grid grid-cols-3 gap-1">
                {PAD.map((direction, index) =>
                  direction ? (
                    <button
                      key={index}
                      type="button"
                      className={padClass}
                      aria-label={t(`ptz-move-${direction}`)}
                      onPointerDown={() => drive(direction)}
                      onPointerUp={() => drive("stop")}
                      onPointerLeave={() => drive("stop")}
                    >
                      {ARROWS[direction]}
                    </button>
                  ) : (
                    <button
                      key={index}
                      type="button"
                      className={padClass}
                      aria-label={t("ptz-move-stop")}
                      onClick={() => drive("stop")}
                    >
                      {ARROWS.stop}
                    </button>
                  ),
                )}
              </div>

              <div className="flex flex-col gap-2">
                <label className="flex items-center gap-2 text-havoc-muted">
                  {t("ptz-speed")}
                  <input
                    type="range"
                    min={1}
                    max={24}
                    value={speed}
                    onChange={(event) => setSpeed(Number(event.target.value))}
                    className="accent-havoc-accent"
                  />
                  <span className="tabular-nums">{speed}</span>
                </label>
                <div className="flex items-center gap-1">
                  <span className="text-havoc-muted">{t("ptz-zoom")}</span>
                  <button
                    type="button"
                    className={padClass}
                    aria-label={t("ptz-zoom-out")}
                    onPointerDown={() => ptzZoom(camera.name, -5).catch(fail)}
                    onPointerUp={() => ptzZoom(camera.name, 0).catch(fail)}
                    onPointerLeave={() => ptzZoom(camera.name, 0).catch(fail)}
                  >
                    −
                  </button>
                  <button
                    type="button"
                    className={padClass}
                    aria-label={t("ptz-zoom-in")}
                    onPointerDown={() => ptzZoom(camera.name, 5).catch(fail)}
                    onPointerUp={() => ptzZoom(camera.name, 0).catch(fail)}
                    onPointerLeave={() => ptzZoom(camera.name, 0).catch(fail)}
                  >
                    +
                  </button>
                </div>
              </div>
            </div>

            {/* Presets */}
            <section className="flex flex-col gap-1.5 border-t border-white/5 pt-2">
              <span className="text-[11px] font-semibold tracking-wider text-havoc-muted uppercase">
                {t("ptz-presets")}
              </span>
              {camera.presets.map((preset, index) => (
                <div key={index} className="flex flex-wrap items-center gap-2">
                  <input
                    value={preset.name}
                    onChange={(event) =>
                      update({
                        presets: camera.presets.map((entry, at) =>
                          at === index ? { ...entry, name: event.target.value } : entry,
                        ),
                      })
                    }
                    aria-label={t("ptz-preset-name")}
                    className={`${inputClass} min-w-0 flex-1`}
                  />
                  <input
                    type="number"
                    min={0}
                    max={254}
                    value={preset.slot}
                    onChange={(event) =>
                      update({
                        presets: camera.presets.map((entry, at) =>
                          at === index
                            ? { ...entry, slot: Math.min(254, Number(event.target.value) || 0) }
                            : entry,
                        ),
                      })
                    }
                    aria-label={t("ptz-slot")}
                    className={`${inputClass} w-16`}
                  />
                  <button
                    type="button"
                    onClick={() => ptzPresetRecall(camera.name, preset.slot).catch(fail)}
                    className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2 py-1"
                  >
                    {t("ptz-recall")}
                  </button>
                  <button
                    type="button"
                    onClick={() => ptzPresetStore(camera.name, preset.slot).catch(fail)}
                    className="rounded-md border border-white/10 px-2 py-1 text-havoc-muted hover:text-havoc-text"
                  >
                    {t("ptz-store")}
                  </button>
                  <button
                    type="button"
                    onClick={() =>
                      update({ presets: camera.presets.filter((_, at) => at !== index) })
                    }
                    aria-label={t("ptz-remove-preset")}
                    className="rounded px-1.5 text-havoc-muted hover:text-red-400"
                  >
                    ×
                  </button>
                </div>
              ))}
              <button
                type="button"
                onClick={() =>
                  update({
                    presets: [
                      ...camera.presets,
                      { name: t("ptz-new-preset"), slot: camera.presets.length },
                    ],
                  })
                }
                className="self-start rounded-md border border-white/10 px-2 py-1 text-havoc-muted hover:text-havoc-text"
              >
                {t("ptz-add-preset")}
              </button>
            </section>

            {/* Per-scene auto-recall */}
            <section className="flex flex-col gap-1.5 border-t border-white/5 pt-2">
              <span className="text-[11px] font-semibold tracking-wider text-havoc-muted uppercase">
                {t("ptz-scene-recalls")}
              </span>
              <p className="m-0 text-havoc-muted">{t("ptz-scene-recalls-about")}</p>
              {camera.sceneRecalls.map((recall, index) => (
                <div key={index} className="flex flex-wrap items-center gap-2">
                  <select
                    value={recall.scene}
                    onChange={(event) =>
                      update({
                        sceneRecalls: camera.sceneRecalls.map((entry, at) =>
                          at === index ? { ...entry, scene: event.target.value } : entry,
                        ),
                      })
                    }
                    aria-label={t("ptz-scene")}
                    className={inputClass}
                  >
                    {sceneNames.map((name) => (
                      <option key={name} value={name}>
                        {name}
                      </option>
                    ))}
                  </select>
                  <select
                    value={recall.slot}
                    onChange={(event) =>
                      update({
                        sceneRecalls: camera.sceneRecalls.map((entry, at) =>
                          at === index ? { ...entry, slot: Number(event.target.value) } : entry,
                        ),
                      })
                    }
                    aria-label={t("ptz-slot")}
                    className={inputClass}
                  >
                    {camera.presets.map((preset) => (
                      <option key={preset.slot} value={preset.slot}>
                        {preset.name}
                      </option>
                    ))}
                  </select>
                  <button
                    type="button"
                    onClick={() =>
                      update({
                        sceneRecalls: camera.sceneRecalls.filter((_, at) => at !== index),
                      })
                    }
                    aria-label={t("ptz-remove-recall")}
                    className="rounded px-1.5 text-havoc-muted hover:text-red-400"
                  >
                    ×
                  </button>
                </div>
              ))}
              <button
                type="button"
                disabled={camera.presets.length === 0 || sceneNames.length === 0}
                onClick={() =>
                  update({
                    sceneRecalls: [
                      ...camera.sceneRecalls,
                      { scene: sceneNames[0], slot: camera.presets[0].slot },
                    ],
                  })
                }
                className="self-start rounded-md border border-white/10 px-2 py-1 text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-50"
              >
                {t("ptz-add-recall")}
              </button>
            </section>
          </>
        )}
      </div>
    </PickerShell>
  );
}
