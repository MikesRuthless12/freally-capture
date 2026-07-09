import { useCallback, useEffect, useState } from "react";

import {
  recordingExport,
  recordingExportCancel,
  recordingRemux,
  recordingsList,
} from "../api/commands";
import { onRecordingExport } from "../api/events";
import type { ExportStatus, RecordingFile } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

function formatSize(bytes: number): string {
  if (bytes >= 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

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
  const [exportingPath, setExportingPath] = useState<string | null>(null);
  const [progress, setProgress] = useState<ExportStatus | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

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

  const pct =
    progress?.state === "exporting" && progress.framesTotal > 0
      ? Math.min(100, (progress.framesDone / progress.framesTotal) * 100)
      : null;

  return (
    <PickerShell title={t("recordings-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-2 text-xs text-havoc-text">
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
                {formatSize(file.sizeBytes)} · {formatWhen(file.modifiedMs)}
                {file.ext === "frec" && ` · ${t("recordings-frec-label")}`}
              </p>
            </div>
            {file.ext === "mkv" && (
              <button
                type="button"
                disabled={remuxing !== null || exportingPath !== null}
                onClick={() => remux(file.path)}
                title={t("recordings-remux-title")}
                className="shrink-0 rounded-md border border-white/10 bg-white/[0.04] px-2 py-1 text-[11px] text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-50"
              >
                {remuxing === file.path ? t("recordings-remuxing") : t("recordings-remux-to-mp4")}
              </button>
            )}
            {file.ext === "frec" && (
              <div className="flex shrink-0 gap-1.5">
                <button
                  type="button"
                  disabled={exportingPath !== null || remuxing !== null}
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
                  disabled={exportingPath !== null || remuxing !== null}
                  onClick={() => startExport(file.path, "mkv")}
                  title={t("recordings-export-mkv-title")}
                  className="rounded-md border border-white/10 bg-white/[0.04] px-2 py-1 text-[11px] text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-50"
                >
                  MKV
                </button>
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

        {notice && <p className="m-0 text-[11px] break-all text-emerald-300">{notice}</p>}
        {error && (
          <p role="alert" className="m-0 text-[11px] break-words text-red-300">
            {error}
          </p>
        )}
      </div>
    </PickerShell>
  );
}
