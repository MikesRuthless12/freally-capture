import { useEffect, useState } from "react";

import { openFrecExport } from "../api/commands";
import { onRecordingExport } from "../api/events";
import type { ExportStatus } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

/**
 * Shown when Freally Capture is opened with a `.frec` (OS double-click).
 * Capture **records** `.frec`; playback is Freally Player's job (coming soon).
 * So this offers to **export** the opened file to MP4/MKV — which plays in any
 * player today — with a live percentage + bar (via the `recording-export`
 * event, same as the Recordings dialog).
 */
export function OpenedFrecDialog({ path, onClose }: { path: string; onClose: () => void }) {
  const t = useT();
  const name = path.split(/[/\\]/).pop() ?? path;
  const [progress, setProgress] = useState<ExportStatus | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [done, setDone] = useState<string | null>(null);

  useEffect(() => {
    let alive = true;
    let unlisten: (() => void) | undefined;
    onRecordingExport((status) => {
      if (!alive) return;
      setProgress(status.state === "exporting" ? status : null);
      if (status.state === "done") setDone(status.path);
      else if (status.state === "error") setError(status.message);
    })
      .then((fn) => (alive ? (unlisten = fn) : fn()))
      .catch(() => undefined);
    return () => {
      alive = false;
      unlisten?.();
    };
  }, []);

  const doExport = (container: string) => {
    setError(null);
    setDone(null);
    setProgress({ state: "exporting", framesDone: 0, framesTotal: 0 });
    openFrecExport(path, container).catch((err) => {
      setProgress(null);
      setError(String(err));
    });
  };

  const pct =
    progress?.state === "exporting" && progress.framesTotal > 0
      ? Math.min(100, (progress.framesDone / progress.framesTotal) * 100)
      : null;

  return (
    <PickerShell title={t("openfrec-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0">
          <span className="font-mono text-[11px] text-havoc-text">{name}</span>
        </p>
        <p className="m-0 text-[11px] leading-snug text-havoc-muted">{t("openfrec-desc")}</p>

        {done ? (
          <p className="m-0 text-[11px] break-all text-emerald-300">
            {t("openfrec-exported-to", { path: done })}
          </p>
        ) : progress ? (
          <div className="flex flex-col gap-1.5">
            <div className="flex items-baseline justify-between">
              <span>{t("openfrec-exporting")}</span>
              <span className="font-mono text-havoc-muted">
                {pct !== null ? `${pct.toFixed(2)}%` : t("openfrec-starting")}
              </span>
            </div>
            <div className="h-1.5 overflow-hidden rounded-full bg-white/10">
              <div
                className="h-full rounded-full bg-gradient-to-r from-havoc-accent to-havoc-accent-2 transition-[width]"
                style={{ width: pct !== null ? `${pct.toFixed(2)}%` : "8%" }}
              />
            </div>
          </div>
        ) : (
          <div className="flex flex-wrap gap-2">
            <button
              type="button"
              onClick={() => doExport("mp4")}
              className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
            >
              {t("openfrec-export-mp4")}
            </button>
            <button
              type="button"
              onClick={() => doExport("mkv")}
              className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
            >
              {t("openfrec-export-mkv")}
            </button>
          </div>
        )}

        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}
      </div>
    </PickerShell>
  );
}
