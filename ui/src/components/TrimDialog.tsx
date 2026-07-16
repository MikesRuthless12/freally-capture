import { useEffect, useRef, useState } from "react";

import { recordingTrim, recordingTrimInfo, recordingTrimPreview } from "../api/commands";
import type { RecordingFile, TrimInfo } from "../api/types";
import { useT } from "../i18n/t";
import { formatTimecode, landsOnKeyframe, nearestKeyframe, stepFrames } from "../lib/trim";
import { PickerShell } from "./PickerShell";

const stepButton =
  "rounded-md border border-white/10 bg-white/[0.04] px-2 py-1 text-[11px] text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-50";

/**
 * CAP-N41: the trim window. Scrub a saved recording with frame-step
 * precision, set in/out points, and export the range — the badge says
 * honestly whether the export will stream-copy (in-point on a keyframe)
 * or re-encode. Export progress rides the parent recordings dialog's
 * existing `recording-export` display; this dialog closes on start.
 */
export function TrimDialog({
  file,
  onClose,
  onStarted,
}: {
  file: RecordingFile;
  onClose: () => void;
  /** The export was kicked off — the parent takes over progress display. */
  onStarted: () => void;
}) {
  const t = useT();
  const [info, setInfo] = useState<TrimInfo | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [position, setPosition] = useState(0);
  const [preview, setPreview] = useState<string | null>(null);
  const [inPoint, setInPoint] = useState(0);
  const [outPoint, setOutPoint] = useState(0);
  const [starting, setStarting] = useState(false);
  const previewSeq = useRef(0);

  useEffect(() => {
    let alive = true;
    recordingTrimInfo(file.path)
      .then((found) => {
        if (!alive) return;
        setInfo(found);
        setOutPoint(found.durationSecs);
      })
      .catch((err) => alive && setError(String(err)));
    return () => {
      alive = false;
    };
  }, [file.path]);

  // Debounced preview of the scrub position (latest request wins).
  useEffect(() => {
    if (!info) return;
    const seq = ++previewSeq.current;
    const timer = setTimeout(() => {
      recordingTrimPreview(file.path, position)
        .then((url) => {
          if (previewSeq.current === seq) setPreview(url);
        })
        .catch(() => undefined);
    }, 150);
    return () => clearTimeout(timer);
  }, [file.path, info, position]);

  const fps = info?.fps ?? 30;
  const duration = info?.durationSecs ?? 0;
  const validRange = outPoint > inPoint;
  const losslessCut = info ? landsOnKeyframe(inPoint, info.keyframesSecs, fps) : false;

  const step = (frames: number) => {
    setPosition((current) => stepFrames(current, frames, fps, duration));
  };

  const snapToKeyframe = () => {
    if (!info) return;
    const kf = nearestKeyframe(position, info.keyframesSecs);
    if (kf !== null) setPosition(kf);
  };

  const startExport = async (reframe916: boolean) => {
    setStarting(true);
    setError(null);
    try {
      await recordingTrim(file.path, inPoint, outPoint, reframe916);
      onStarted();
      onClose();
    } catch (err) {
      setError(String(err));
      setStarting(false);
    }
  };

  return (
    <PickerShell title={t("trim-title", { name: file.name })} onClose={onClose} wide>
      <div className="flex flex-col gap-2 text-xs text-havoc-text">
        {!info && !error && <p className="m-0 text-havoc-muted">{t("trim-loading")}</p>}
        {info && (
          <>
            <div className="flex min-h-40 items-center justify-center overflow-hidden rounded-lg border border-white/10 bg-black/40">
              {preview ? (
                <img
                  src={preview}
                  alt={t("trim-preview-alt")}
                  className="max-h-64 max-w-full object-contain"
                />
              ) : (
                <span className="text-[11px] text-havoc-muted">{t("trim-loading")}</span>
              )}
            </div>

            <input
              type="range"
              min={0}
              max={duration}
              step={1 / Math.max(fps, 1)}
              value={position}
              onChange={(event) => setPosition(Number(event.target.value))}
              aria-label={t("trim-position")}
              className="w-full accent-havoc-accent"
            />
            <div className="flex items-center justify-between gap-2">
              <div className="flex gap-1">
                <button
                  type="button"
                  onClick={() => step(-Math.round(fps))}
                  title={t("trim-step-second-back")}
                  className={stepButton}
                >
                  −1s
                </button>
                <button
                  type="button"
                  onClick={() => step(-1)}
                  title={t("trim-step-frame-back")}
                  className={stepButton}
                >
                  −1f
                </button>
                <button
                  type="button"
                  onClick={() => step(1)}
                  title={t("trim-step-frame-forward")}
                  className={stepButton}
                >
                  +1f
                </button>
                <button
                  type="button"
                  onClick={() => step(Math.round(fps))}
                  title={t("trim-step-second-forward")}
                  className={stepButton}
                >
                  +1s
                </button>
                <button
                  type="button"
                  onClick={snapToKeyframe}
                  title={t("trim-snap-title")}
                  className={stepButton}
                >
                  {t("trim-snap")}
                </button>
              </div>
              <span className="font-mono text-[11px] text-havoc-muted">
                {formatTimecode(position, fps)} / {formatTimecode(duration, fps)}
              </span>
            </div>

            <div className="flex items-center gap-2">
              <button type="button" onClick={() => setInPoint(position)} className={stepButton}>
                {t("trim-set-in")}
              </button>
              <span className="font-mono text-[11px]">{formatTimecode(inPoint, fps)}</span>
              <span className="text-havoc-muted">→</span>
              <button type="button" onClick={() => setOutPoint(position)} className={stepButton}>
                {t("trim-set-out")}
              </button>
              <span className="font-mono text-[11px]">{formatTimecode(outPoint, fps)}</span>
            </div>
            {!validRange && (
              <p className="m-0 text-[11px] text-amber-300">{t("trim-range-invalid")}</p>
            )}

            <p
              className={`m-0 text-[11px] ${losslessCut ? "text-emerald-300" : "text-havoc-muted"}`}
            >
              {losslessCut ? t("trim-copy-badge") : t("trim-reencode-badge")}
            </p>

            <div className="flex items-center gap-1.5">
              <button
                type="button"
                disabled={!validRange || starting}
                onClick={() => startExport(false)}
                className="rounded-md border border-havoc-accent/40 bg-havoc-accent/10 px-2.5 py-1 text-[11px] text-havoc-text transition-colors enabled:hover:border-havoc-accent/70 disabled:opacity-50"
              >
                {t("trim-export")}
              </button>
              <button
                type="button"
                disabled={!validRange || starting}
                onClick={() => startExport(true)}
                title={t("trim-export-916-title")}
                className={stepButton}
              >
                {t("trim-export-916")}
              </button>
            </div>
          </>
        )}
        {error && (
          <p role="alert" className="m-0 text-[11px] break-words text-red-300">
            {error}
          </p>
        )}
      </div>
    </PickerShell>
  );
}
