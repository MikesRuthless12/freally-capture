import { useCallback, useEffect, useState } from "react";

import {
  audiorecSetPaused,
  audiorecStart,
  audiorecStatus,
  audiorecStop,
  recordingExport,
  recordingExportAlpha,
  recordingExportCancel,
  pipelineStatus,
  recordingNormalize,
  recordingRemux,
  recordingsList,
  recordingVerify,
  settingsGet,
  settingsSet,
} from "../api/commands";
import { onPipeline, onRecordingExport } from "../api/events";
import type {
  AudioRecFormat,
  AudioRecStatus,
  ExportStatus,
  PipelineJob,
  RecordingFile,
  Settings,
  VerifyReport,
} from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { TrimDialog } from "../components/TrimDialog";
import { useT } from "../i18n/t";
import { formatBytes } from "../lib/format";

function formatWhen(ms: number): string {
  return ms > 0 ? new Date(ms).toLocaleString() : "";
}

/**
 * The recordings list: what landed in the recordings folder, newest first.
 * `.mkv` files get a stream-copy Remux to MP4; owned `.frec` files get an
 * Export to MP4/MKV (decode + re-encode through the ffmpeg component) so they
 * play in any player, with a live percentage, a progress bar, and Cancel.
 */
export function RecordingsDialog({ onClose }: { onClose: () => void }) {
  const t = useT();
  const [files, setFiles] = useState<RecordingFile[] | null>(null);
  const [remuxing, setRemuxing] = useState<string | null>(null);
  const [normalizing, setNormalizing] = useState<string | null>(null);
  const [exportingPath, setExportingPath] = useState<string | null>(null);
  const [progress, setProgress] = useState<ExportStatus | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [trimFile, setTrimFile] = useState<RecordingFile | null>(null);
  const [verifying, setVerifying] = useState<string | null>(null);
  const [verifyResult, setVerifyResult] = useState<{ name: string; report: VerifyReport } | null>(
    null,
  );
  const [pipelineJobs, setPipelineJobs] = useState<PipelineJob[]>([]);

  // CAP-N45: the post-record pipeline queue (snapshot + live updates).
  useEffect(() => {
    let alive = true;
    let unlisten: (() => void) | undefined;
    pipelineStatus()
      .then((jobs) => alive && setPipelineJobs(jobs))
      .catch(() => undefined);
    onPipeline((jobs) => {
      if (alive) setPipelineJobs(jobs);
    })
      .then((fn) => {
        if (alive) unlisten = fn;
        else fn();
      })
      .catch(() => undefined);
    return () => {
      alive = false;
      unlisten?.();
    };
  }, []);

  const refresh = useCallback(() => {
    recordingsList()
      .then(setFiles)
      .catch((err) => {
        setFiles([]);
        setError(String(err));
      });
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  // Live export progress + terminal states arrive on the `recording-export`
  // event (the work runs on a worker thread; the command returns at once).
  useEffect(() => {
    let alive = true;
    let unlisten: (() => void) | undefined;
    onRecordingExport((status) => {
      if (!alive) return;
      setProgress(status);
      if (status.state === "done") {
        setExportingPath(null);
        setProgress(null);
        setNotice(t("recordings-exported-to", { path: status.path }));
        refresh();
      } else if (status.state === "error") {
        setExportingPath(null);
        setProgress(null);
        setError(status.message);
      } else if (status.state === "cancelled") {
        setExportingPath(null);
        setProgress(null);
        setNotice(t("recordings-export-cancelled"));
      }
    })
      .then((fn) => {
        if (alive) unlisten = fn;
        else fn();
      })
      .catch(() => undefined);
    return () => {
      alive = false;
      unlisten?.();
    };
  }, [refresh, t]);

  const remux = async (path: string) => {
    setRemuxing(path);
    setError(null);
    setNotice(null);
    try {
      const output = await recordingRemux(path);
      setNotice(t("recordings-remuxed-to", { path: output }));
      refresh();
    } catch (err) {
      setError(String(err));
    } finally {
      setRemuxing(null);
    }
  };

  const normalize = async (path: string) => {
    setNormalizing(path);
    setError(null);
    setNotice(null);
    try {
      const output = await recordingNormalize(path);
      setNotice(t("recordings-normalized-to", { path: output }));
      refresh();
    } catch (err) {
      setError(String(err));
    } finally {
      setNormalizing(null);
    }
  };

  const startExport = async (path: string, container: string) => {
    setError(null);
    setNotice(null);
    setExportingPath(path);
    setProgress({ state: "exporting", framesDone: 0, framesTotal: 0 });
    try {
      await recordingExport(path, container);
    } catch (err) {
      setExportingPath(null);
      setProgress(null);
      setError(String(err));
    }
  };

  // CAP-N46: on-demand integrity verification — deep for .frec (the owned
  // walk is fast), tail-scan for wire files.
  const verify = async (file: RecordingFile) => {
    setVerifying(file.path);
    setError(null);
    setNotice(null);
    setVerifyResult(null);
    try {
      const report = await recordingVerify(file.path, file.ext === "frec");
      setVerifyResult({ name: file.name, report });
    } catch (err) {
      setError(String(err));
    } finally {
      setVerifying(null);
    }
  };

  // CAP-N42: the alpha-preserving .mov master export (ProRes 4444 / QTRLE)
  // rides the same progress event/panel as the normal export.
  const startAlphaExport = async (path: string, codec: string) => {
    setError(null);
    setNotice(null);
    setExportingPath(path);
    setProgress({ state: "exporting", framesDone: 0, framesTotal: 0 });
    try {
      await recordingExportAlpha(path, codec);
    } catch (err) {
      setExportingPath(null);
      setProgress(null);
      setError(String(err));
    }
  };

  // CAP-N46: the Verify button appears in both the wire-file and .frec action
  // rows — one render helper so they can't drift.
  const verifyButton = (file: RecordingFile) => (
    <button
      type="button"
      disabled={verifying !== null}
      onClick={() => verify(file)}
      title={t("recordings-verify-title")}
      className="rounded-md border border-white/10 bg-white/[0.04] px-2 py-1 text-[11px] text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-50"
    >
      {verifying === file.path ? t("recordings-verifying") : t("recordings-verify")}
    </button>
  );

  const pct =
    progress?.state === "exporting" && progress.framesTotal > 0
      ? Math.min(100, (progress.framesDone / progress.framesTotal) * 100)
      : null;

  return (
    <PickerShell title={t("recordings-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-2 text-xs text-havoc-text">
        <AudioRecPanel onNotice={setNotice} onError={setError} />
        {files === null && <p className="m-0 text-havoc-muted">{t("recordings-loading")}</p>}
        {files?.length === 0 && <p className="m-0 text-havoc-muted">{t("recordings-empty")}</p>}
        {(files ?? []).map((file) => (
          <div
            key={file.path}
            className="flex items-center justify-between gap-2 rounded-lg border border-white/10 bg-white/[0.03] px-2.5 py-2"
          >
            <div className="min-w-0">
              <p className="m-0 truncate" title={file.path}>
                {file.name}
              </p>
              <p className="m-0 text-[10px] text-havoc-muted">
                {formatBytes(file.sizeBytes)} · {formatWhen(file.modifiedMs)}
                {file.ext === "frec" && ` · ${t("recordings-frec-label")}`}
                {file.frecAlpha && ` · ${t("recordings-alpha-label")}`}
              </p>
            </div>
            {["mkv", "mp4", "mov", "webm"].includes(file.ext) && (
              <div className="flex shrink-0 gap-1.5">
                {verifyButton(file)}
                <button
                  type="button"
                  disabled={remuxing !== null || normalizing !== null || exportingPath !== null}
                  onClick={() => setTrimFile(file)}
                  title={t("recordings-trim-title")}
                  className="rounded-md border border-white/10 bg-white/[0.04] px-2 py-1 text-[11px] text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-50"
                >
                  {t("recordings-trim")}
                </button>
                {file.ext === "mkv" && (
                  <button
                    type="button"
                    disabled={remuxing !== null || normalizing !== null || exportingPath !== null}
                    onClick={() => remux(file.path)}
                    title={t("recordings-remux-title")}
                    className="rounded-md border border-white/10 bg-white/[0.04] px-2 py-1 text-[11px] text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-50"
                  >
                    {remuxing === file.path
                      ? t("recordings-remuxing")
                      : t("recordings-remux-to-mp4")}
                  </button>
                )}
                <button
                  type="button"
                  disabled={normalizing !== null || remuxing !== null || exportingPath !== null}
                  onClick={() => normalize(file.path)}
                  title={t("recordings-normalize-title")}
                  className="rounded-md border border-white/10 bg-white/[0.04] px-2 py-1 text-[11px] text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-50"
                >
                  {normalizing === file.path
                    ? t("recordings-normalizing")
                    : t("recordings-normalize")}
                </button>
              </div>
            )}
            {file.ext === "frec" && (
              <div className="flex shrink-0 gap-1.5">
                {verifyButton(file)}
                <button
                  type="button"
                  disabled={exportingPath !== null || remuxing !== null || normalizing !== null}
                  onClick={() => startExport(file.path, "mp4")}
                  title={t("recordings-export-mp4-title")}
                  className="rounded-md border border-havoc-accent/40 bg-havoc-accent/10 px-2 py-1 text-[11px] text-havoc-text transition-colors enabled:hover:border-havoc-accent/70 disabled:opacity-50"
                >
                  {exportingPath === file.path
                    ? t("recordings-exporting")
                    : t("recordings-export-mp4")}
                </button>
                <button
                  type="button"
                  disabled={exportingPath !== null || remuxing !== null || normalizing !== null}
                  onClick={() => startExport(file.path, "mkv")}
                  title={t("recordings-export-mkv-title")}
                  className="rounded-md border border-white/10 bg-white/[0.04] px-2 py-1 text-[11px] text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-50"
                >
                  MKV
                </button>
                {file.frecAlpha && (
                  <>
                    <button
                      type="button"
                      disabled={exportingPath !== null || remuxing !== null || normalizing !== null}
                      onClick={() => startAlphaExport(file.path, "prores4444")}
                      title={t("recordings-prores-title")}
                      className="rounded-md border border-white/10 bg-white/[0.04] px-2 py-1 text-[11px] text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-50"
                    >
                      ProRes 4444
                    </button>
                    <button
                      type="button"
                      disabled={exportingPath !== null || remuxing !== null || normalizing !== null}
                      onClick={() => startAlphaExport(file.path, "qtrle")}
                      title={t("recordings-qtrle-title")}
                      className="rounded-md border border-white/10 bg-white/[0.04] px-2 py-1 text-[11px] text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-50"
                    >
                      QTRLE
                    </button>
                  </>
                )}
              </div>
            )}
          </div>
        ))}

        {exportingPath && (
          <div className="flex flex-col gap-1.5 rounded-lg border border-havoc-accent/30 bg-havoc-accent/[0.06] px-2.5 py-2">
            <div className="flex items-baseline justify-between">
              <span>{t("recordings-exporting")}</span>
              <span className="font-mono text-havoc-muted">
                {pct !== null ? `${pct.toFixed(2)}%` : t("recordings-starting")}
                {progress?.state === "exporting" && progress.framesTotal > 0
                  ? ` · ${t("recordings-frames", { done: progress.framesDone, total: progress.framesTotal })}`
                  : ""}
              </span>
            </div>
            <div className="h-1.5 overflow-hidden rounded-full bg-white/10">
              <div
                className="h-full rounded-full bg-gradient-to-r from-havoc-accent to-havoc-accent-2 transition-[width]"
                style={{ width: pct !== null ? `${pct.toFixed(2)}%` : "8%" }}
              />
            </div>
            <button
              type="button"
              onClick={() => recordingExportCancel().catch(() => undefined)}
              className="self-start rounded-md border border-white/10 bg-white/[0.04] px-2.5 py-1 text-[11px] text-havoc-muted transition-colors hover:text-havoc-text"
            >
              {t("recordings-cancel")}
            </button>
          </div>
        )}

        {pipelineJobs.length > 0 && (
          <div className="flex flex-col gap-1.5 rounded-lg border border-white/10 bg-white/[0.03] px-2.5 py-2">
            <p className="m-0 text-[11px] font-semibold tracking-wider text-havoc-muted uppercase">
              {t("pipeline-queue")}
            </p>
            {pipelineJobs.slice(0, 5).map((job) => (
              <div key={job.id} className="flex flex-col gap-0.5">
                <p className="m-0 truncate text-[11px]" title={job.file}>
                  {job.done ? "✓" : "⏳"} {job.file}
                </p>
                {job.steps.map((step, at) => (
                  <p
                    key={`${job.id}-${at}`}
                    className={`m-0 pl-4 text-[10px] leading-snug ${
                      step.status === "fail"
                        ? "text-red-300"
                        : step.status === "warn"
                          ? "text-amber-300"
                          : "text-havoc-muted"
                    }`}
                  >
                    {step.status === "ok"
                      ? "✓"
                      : step.status === "running"
                        ? "…"
                        : step.status === "pending"
                          ? "·"
                          : step.status === "skipped"
                            ? "—"
                            : "⚠"}{" "}
                    {t(`pipeline-${step.action}`)}
                    {step.detail ? `: ${step.detail}` : ""}
                  </p>
                ))}
              </div>
            ))}
          </div>
        )}
        {verifyResult && (
          <div
            className={`flex flex-col gap-1 rounded-lg border px-2.5 py-2 ${
              verifyResult.report.verdict === "pass"
                ? "border-emerald-400/30 bg-emerald-400/[0.06]"
                : verifyResult.report.verdict === "warn"
                  ? "border-amber-400/30 bg-amber-400/[0.06]"
                  : "border-red-400/30 bg-red-400/[0.06]"
            }`}
          >
            <div className="flex items-baseline justify-between gap-2">
              <p className="m-0 text-[11px] font-semibold">
                {t(`verify-verdict-${verifyResult.report.verdict}`, {
                  name: verifyResult.name,
                })}
              </p>
              <button
                type="button"
                onClick={() => setVerifyResult(null)}
                className="rounded-md border border-white/10 bg-white/[0.04] px-1.5 py-0.5 text-[10px] text-havoc-muted transition-colors hover:text-havoc-text"
              >
                {t("verify-dismiss")}
              </button>
            </div>
            {verifyResult.report.checks.map((check) => (
              <p key={check.id} className="m-0 text-[10px] leading-snug text-havoc-muted">
                {check.status === "pass" ? "✓" : check.status === "skipped" ? "—" : "⚠"}{" "}
                {t(`verify-${check.id}`)}: {check.detail}
              </p>
            ))}
          </div>
        )}
        {notice && <p className="m-0 text-[11px] break-all text-emerald-300">{notice}</p>}
        {error && (
          <p role="alert" className="m-0 text-[11px] break-words text-red-300">
            {error}
          </p>
        )}
      </div>
      {trimFile && (
        <TrimDialog
          file={trimFile}
          onClose={() => setTrimFile(null)}
          onStarted={() => {
            // The trim export rides the same `recording-export` event this
            // dialog already renders — show the progress panel right away.
            setError(null);
            setNotice(null);
            setExportingPath(trimFile.path);
            setProgress({ state: "exporting", framesDone: 0, framesTotal: 0 });
          }}
        />
      )}
    </PickerShell>
  );
}

const AUDIO_FORMATS: AudioRecFormat[] = ["wav", "flac", "opus"];

/** CAP-N38 audio-only recorder: format picker + start/stop + live duration. */
function AudioRecPanel({
  onNotice,
  onError,
}: {
  onNotice: (message: string) => void;
  onError: (message: string) => void;
}) {
  const t = useT();
  const [settings, setSettings] = useState<Settings | null>(null);
  const [status, setStatus] = useState<AudioRecStatus>({ state: "idle" });
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    settingsGet()
      .then(setSettings)
      .catch(() => undefined);
  }, []);

  useEffect(() => {
    let alive = true;
    const poll = () =>
      audiorecStatus()
        .then((s) => alive && setStatus(s))
        .catch(() => undefined);
    poll();
    const id = setInterval(poll, 1000);
    return () => {
      alive = false;
      clearInterval(id);
    };
  }, []);

  const recording = status.state === "recording";
  const format = settings?.recording.audioFormat ?? "wav";

  const setFormat = (next: AudioRecFormat) => {
    if (!settings) return;
    const nextSettings: Settings = {
      ...settings,
      recording: { ...settings.recording, audioFormat: next },
    };
    setSettings(nextSettings);
    settingsSet(nextSettings).catch((err) => onError(String(err)));
  };

  const start = async () => {
    setBusy(true);
    try {
      await audiorecStart();
    } catch (err) {
      onError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const stop = async () => {
    setBusy(true);
    try {
      const outputs = await audiorecStop();
      onNotice(t("audiorec-saved", { count: outputs.length }));
    } catch (err) {
      onError(String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="flex flex-wrap items-center gap-2 rounded-lg border border-havoc-accent/30 bg-havoc-accent/[0.06] px-2.5 py-2">
      <span className="font-medium text-havoc-text">{t("audiorec-title")}</span>
      <select
        value={format}
        disabled={recording}
        onChange={(e) => setFormat(e.target.value as AudioRecFormat)}
        aria-label={t("audiorec-format")}
        className="rounded-md border border-white/10 bg-havoc-panel px-1.5 py-0.5 text-[11px] text-havoc-text disabled:opacity-50"
      >
        {AUDIO_FORMATS.map((f) => (
          <option key={f} value={f}>
            {t(`audiorec-format-${f}`)}
          </option>
        ))}
      </select>
      {recording ? (
        <>
          <span className="font-mono text-havoc-accent">
            {t("audiorec-recording", { sec: Math.floor(status.durationSec) })}
          </span>
          <button
            type="button"
            disabled={busy}
            onClick={() => audiorecSetPaused(!status.paused).catch((err) => onError(String(err)))}
            className="rounded-md border border-white/10 px-2 py-0.5 text-[11px] text-havoc-muted enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-50"
          >
            {status.paused ? t("audiorec-resume") : t("audiorec-pause")}
          </button>
          <button
            type="button"
            disabled={busy}
            onClick={stop}
            className="rounded-md border border-red-500/40 bg-red-500/10 px-2 py-0.5 text-[11px] text-red-300 enabled:hover:border-red-500/70 disabled:opacity-50"
          >
            {t("audiorec-stop")}
          </button>
        </>
      ) : (
        <button
          type="button"
          disabled={busy || !settings}
          onClick={start}
          className="ml-auto rounded-md border border-havoc-accent/40 bg-havoc-accent/10 px-2 py-0.5 text-[11px] text-havoc-text enabled:hover:border-havoc-accent/70 disabled:opacity-50"
        >
          {t("audiorec-start")}
        </button>
      )}
    </div>
  );
}
