import { useEffect, useMemo, useRef, useState } from "react";

import {
  auxWindowOpen,
  calibrationFinish,
  calibrationStart,
  calibrationStatus,
  calibrationStop,
  listDisplays,
  studioAddItem,
  studioRemoveItem,
  studioSetAudioMonitor,
  studioSetAudioSolo,
  studioSetAudioSyncOffset,
} from "../api/commands";
import type {
  AudioLevelsPayload,
  CalibrationResult,
  CalibrationStatus,
  DisplayInfo,
  ProgramStatus,
  SourceId,
  StudioDto,
} from "../api/types";
import { MAX_SYNC_OFFSET_MS } from "../api/types";
import { NumberField } from "../components/NumberField";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const selectClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text";

/** How long one measuring run records (≈6 flash+beep cycles). */
const MEASURE_MS = 12_000;

/** Kinds that render nothing — they can't be the camera side. */
const AUDIO_ONLY_KINDS = new Set(["audioInput", "audioOutput", "appAudio", "testTone"]);

type Step = "setup" | "measuring" | "done";

/**
 * CAP-M20 — the guided A/V sync calibration workbench. Plays the built-in
 * flash+beep pattern (a temporary source on the current scene, beep on the
 * monitor device), watches the picked camera + mic capture it back, and
 * offers to apply the measured offset to the mic's sync-offset — with the
 * manual field as the fallback. Honest note: the loop runs through the
 * user's display + speakers, so their own small latencies are included.
 */
export function AvSyncDialog({
  studio,
  program,
  audio,
  onClose,
}: {
  studio: StudioDto | null;
  program: ProgramStatus | null;
  audio: AudioLevelsPayload | null;
  onClose: () => void;
}) {
  const t = useT();
  const [step, setStep] = useState<Step>("setup");
  const [videoId, setVideoId] = useState<SourceId | null>(null);
  const [audioId, setAudioId] = useState<SourceId | null>(null);
  const [displays, setDisplays] = useState<DisplayInfo[]>([]);
  const [display, setDisplay] = useState(0);
  const [status, setStatus] = useState<CalibrationStatus | null>(null);
  const [result, setResult] = useState<CalibrationResult | null>(null);
  const [applied, setApplied] = useState<number | null>(null);
  const [fail, setFail] = useState<string | null>(null);
  // The temporary pattern item, removed when the run ends or the dialog closes.
  const tempRef = useRef<{ sceneId: string; itemId: string } | null>(null);
  // Still mounted? A close between Start and the add resolving must not leave
  // a strobing pattern source orphaned in the scene.
  const mountedRef = useRef(true);

  const sources = useMemo(() => studio?.collection.sources ?? [], [studio]);
  // Only sources the engine is ACTUALLY running can be measured: the video
  // probe samples drained capture frames, the audio tap samples engine
  // blocks. A camera in an off-program scene has no session and a mic outside
  // the program mix has no strip — offering them would guarantee a failed run.
  const videoCandidates = useMemo(
    () =>
      sources.filter(
        (s) =>
          !AUDIO_ONLY_KINDS.has(s.kind) &&
          !s.kind.startsWith("test") &&
          program?.sources?.[s.id]?.state === "live",
      ),
    [sources, program],
  );
  const audioCandidates = useMemo(
    () =>
      sources.filter(
        (s) =>
          s.audio &&
          s.kind !== "testTone" &&
          s.kind !== "testFlashBeep" &&
          audio?.sources?.[s.id] !== undefined,
      ),
    [sources, audio],
  );
  const currentOffset = sources.find((s) => s.id === audioId)?.audio?.syncOffsetMs ?? 0;

  useEffect(() => {
    let alive = true;
    listDisplays()
      .then((all) => {
        if (alive) setDisplays(all);
      })
      .catch(() => {});
    return () => {
      alive = false;
    };
  }, []);

  const removeTemp = () => {
    const temp = tempRef.current;
    tempRef.current = null;
    if (temp) studioRemoveItem(temp.sceneId, temp.itemId).catch(() => {});
  };

  // Disarm + clean up whenever the dialog unmounts, however it closes.
  useEffect(
    () => () => {
      mountedRef.current = false;
      calibrationStop().catch(() => {});
      removeTemp();
    },
    [],
  );

  const start = async () => {
    if (!studio || !videoId || !audioId) return;
    setFail(null);
    setApplied(null);
    setResult(null);
    setStatus(null);
    try {
      const sceneId = studio.collection.activeScene;
      const added = await studioAddItem(sceneId, {
        kind: "testFlashBeep",
        width: studio.collection.canvasWidth,
        height: studio.collection.canvasHeight,
      });
      tempRef.current = { sceneId, itemId: added.itemId };
      // Closed while the add was in flight: the unmount cleanup already ran
      // (with nothing to remove), so tear the orphan down here instead.
      if (!mountedRef.current) {
        calibrationStop().catch(() => {});
        removeTemp();
        return;
      }
      // Beep on the monitor device only — never into the program mix. Solo it
      // too: a PFL solo left on ANY strip (CAP-M19) evicts every non-soloed
      // strip from the monitor bus, which would silence the beep and make the
      // measurement fail with a misleading "the mic never heard it".
      await studioSetAudioMonitor(added.sourceId, "monitorOnly");
      await studioSetAudioSolo(added.sourceId, true);
      await calibrationStart(videoId, audioId);
      setStep("measuring");
    } catch (err) {
      setFail(String(err));
      removeTemp();
    }
  };

  useEffect(() => {
    if (step !== "measuring") return;
    let alive = true;
    const startedAt = Date.now();
    const tick = window.setInterval(() => {
      calibrationStatus()
        .then((s) => {
          if (alive) setStatus(s);
        })
        .catch(() => {});
      if (Date.now() - startedAt < MEASURE_MS) return;
      window.clearInterval(tick);
      calibrationFinish()
        .then((verdict) => {
          if (!alive) return;
          setResult(verdict);
          setStep("done");
        })
        .catch((err) => {
          if (!alive) return;
          setFail(String(err));
          setStep("setup");
        })
        .finally(removeTemp);
    }, 600);
    return () => {
      alive = false;
      window.clearInterval(tick);
    };
  }, [step]);

  const cancel = () => {
    calibrationStop().catch(() => {});
    removeTemp();
    setStep("setup");
  };

  const apply = () => {
    const measurement = result?.measurement;
    if (!measurement || !audioId) return;
    const value = Math.round(measurement.offsetMs);
    studioSetAudioSyncOffset(audioId, value)
      .then(() => setApplied(value))
      .catch((err) => setFail(String(err)));
  };

  const measurement = result?.measurement ?? null;
  const offsetRounded = measurement ? Math.round(measurement.offsetMs) : 0;

  return (
    <PickerShell title={t("avsync-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        {step === "setup" && (
          <>
            <p className="m-0 leading-relaxed text-havoc-muted">{t("avsync-intro")}</p>
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("avsync-video-label")}
              <select
                value={videoId ?? ""}
                onChange={(event) => setVideoId((event.target.value || null) as SourceId | null)}
                className={selectClass}
              >
                <option value="">{t("avsync-pick")}</option>
                {videoCandidates.map((source) => (
                  <option key={source.id} value={source.id}>
                    {source.name}
                  </option>
                ))}
              </select>
              {videoCandidates.length === 0 && (
                <span className="text-amber-300">{t("avsync-no-video")}</span>
              )}
            </label>
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("avsync-audio-label")}
              <select
                value={audioId ?? ""}
                onChange={(event) => setAudioId((event.target.value || null) as SourceId | null)}
                className={selectClass}
              >
                <option value="">{t("avsync-pick")}</option>
                {audioCandidates.map((source) => (
                  <option key={source.id} value={source.id}>
                    {source.name}
                  </option>
                ))}
              </select>
              {audioCandidates.length === 0 && (
                <span className="text-amber-300">{t("avsync-no-audio")}</span>
              )}
            </label>
            {displays.length > 0 && (
              <div className="flex items-end gap-2">
                <label className="flex min-w-0 flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
                  {t("avsync-projector")}
                  <select
                    value={display}
                    onChange={(event) => setDisplay(Number(event.target.value))}
                    className={selectClass}
                  >
                    {displays.map((info) => (
                      <option key={info.index} value={info.index}>
                        {info.name} ({info.width}×{info.height})
                      </option>
                    ))}
                  </select>
                </label>
                <button
                  type="button"
                  onClick={() =>
                    auxWindowOpen(
                      "projector-program",
                      t("avsync-projector-window-title"),
                      display,
                      true,
                    ).catch((err) => setFail(String(err)))
                  }
                  className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
                >
                  {t("avsync-projector-open")}
                </button>
              </div>
            )}
            <p className="m-0 text-[10px] leading-snug text-havoc-muted">
              {t("avsync-start-note")}
            </p>
            {audioId && (
              <NumberField
                label={t("avsync-manual")}
                value={currentOffset}
                min={0}
                max={MAX_SYNC_OFFSET_MS}
                step={5}
                onCommit={(value) =>
                  studioSetAudioSyncOffset(audioId, Math.round(value)).catch((err) =>
                    setFail(String(err)),
                  )
                }
                className="max-w-48"
              />
            )}
            <button
              type="button"
              disabled={!videoId || !audioId}
              onClick={() => void start()}
              className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
            >
              {t("avsync-start")}
            </button>
          </>
        )}

        {step === "measuring" && (
          <>
            <p className="m-0 leading-relaxed text-havoc-muted">{t("avsync-measuring")}</p>
            <p className={`m-0 ${status?.flashSeen ? "text-emerald-300" : "text-havoc-muted"}`}>
              {status?.flashSeen ? `✓ ${t("avsync-flash-seen")}` : t("avsync-flash-waiting")}
            </p>
            <p className={`m-0 ${status?.beepHeard ? "text-emerald-300" : "text-havoc-muted"}`}>
              {status?.beepHeard ? `✓ ${t("avsync-beep-heard")}` : t("avsync-beep-waiting")}
            </p>
            <button
              type="button"
              onClick={cancel}
              className="self-end rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
            >
              {t("avsync-cancel")}
            </button>
          </>
        )}

        {step === "done" && (
          <>
            {measurement ? (
              <>
                <p className="m-0 text-sm font-semibold">
                  {t("avsync-result-offset", { offset: offsetRounded })}
                </p>
                <p className="m-0 text-havoc-muted">
                  {t("avsync-result-detail", {
                    cycles: measurement.cycles,
                    jitter: Math.round(measurement.jitterMs),
                  })}
                </p>
                {offsetRounded < 0 ? (
                  <p className="m-0 leading-relaxed text-amber-300">{t("avsync-negative")}</p>
                ) : offsetRounded > MAX_SYNC_OFFSET_MS ? (
                  <p className="m-0 leading-relaxed text-amber-300">
                    {t("avsync-over-cap", { max: MAX_SYNC_OFFSET_MS })}
                  </p>
                ) : applied !== null ? (
                  <p className="m-0 text-emerald-300">{t("avsync-applied", { offset: applied })}</p>
                ) : (
                  <button
                    type="button"
                    onClick={apply}
                    className="self-start rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
                  >
                    {t("avsync-apply", { offset: offsetRounded })}
                  </button>
                )}
              </>
            ) : (
              <p className="m-0 leading-relaxed text-amber-300">
                {t(`avsync-error-${result?.error?.kind ?? "noFlash"}`)}
              </p>
            )}
            {audioId && (
              <NumberField
                label={t("avsync-manual")}
                value={currentOffset}
                min={0}
                max={MAX_SYNC_OFFSET_MS}
                step={5}
                onCommit={(value) =>
                  studioSetAudioSyncOffset(audioId, Math.round(value)).catch((err) =>
                    setFail(String(err)),
                  )
                }
                className="max-w-48"
              />
            )}
            <div className="flex justify-end gap-2">
              <button
                type="button"
                onClick={() => void start()}
                className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
              >
                {t("avsync-again")}
              </button>
              <button
                type="button"
                onClick={onClose}
                className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
              >
                {t("avsync-close")}
              </button>
            </div>
          </>
        )}

        {fail && (
          <p role="alert" className="m-0 text-xs text-red-400">
            {fail}
          </p>
        )}
      </div>
    </PickerShell>
  );
}
