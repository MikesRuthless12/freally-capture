import { useCallback, useEffect, useState } from "react";

import { recordingRemux, recordingsList } from "../api/commands";
import type { RecordingFile } from "../api/types";
import { PickerShell } from "../components/PickerShell";

function formatSize(bytes: number): string {
  if (bytes >= 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function formatWhen(ms: number): string {
  return ms > 0 ? new Date(ms).toLocaleString() : "";
}

/**
 * The recordings list: what landed in the recordings folder, newest first,
 * with the post-record remux action (mkv → mp4, stream copy — no
 * re-encode) on mkv files.
 */
export function RecordingsDialog({ onClose }: { onClose: () => void }) {
  const [files, setFiles] = useState<RecordingFile[] | null>(null);
  const [remuxing, setRemuxing] = useState<string | null>(null);
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

  const remux = async (path: string) => {
    setRemuxing(path);
    setError(null);
    setNotice(null);
    try {
      const output = await recordingRemux(path);
      setNotice(`Remuxed to ${output}`);
      refresh();
    } catch (err) {
      setError(String(err));
    } finally {
      setRemuxing(null);
    }
  };

  return (
    <PickerShell title="Recordings" onClose={onClose} wide>
      <div className="flex flex-col gap-2 text-xs text-havoc-text">
        {files === null && <p className="m-0 text-havoc-muted">Reading the folder…</p>}
        {files?.length === 0 && (
          <p className="m-0 text-havoc-muted">
            No recordings yet — Start Recording writes into the folder set in Output.
          </p>
        )}
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
                {file.ext === "frec" && " · owned lossless (freally-video)"}
              </p>
            </div>
            {file.ext === "mkv" && (
              <button
                type="button"
                disabled={remuxing !== null}
                onClick={() => remux(file.path)}
                title="Rewrap as mp4 — stream copy, no re-encode, no quality change (needs the FFmpeg component)"
                className="shrink-0 rounded-md border border-white/10 bg-white/[0.04] px-2 py-1 text-[11px] text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-50"
              >
                {remuxing === file.path ? "Remuxing…" : "Remux to MP4"}
              </button>
            )}
          </div>
        ))}
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
