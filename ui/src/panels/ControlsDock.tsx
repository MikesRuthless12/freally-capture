import { useEffect, useState } from "react";

import {
  recordingPause,
  recordingResume,
  recordingStart,
  recordingStatus,
  recordingStop,
} from "../api/commands";
import { onRecording } from "../api/events";
import type { RecordingStatus, Settings } from "../api/types";
import { Panel } from "../components/Panel";
import { RecDot } from "../components/RecDot";
import { ModelsDialog } from "./Models";
import { SettingsOutput } from "./SettingsOutput";

const buttonBase =
  "w-full rounded-lg border px-3 py-2 text-left text-xs font-medium transition-colors disabled:cursor-not-allowed disabled:opacity-50";

/** The Controls dock: recording (P4); Go Live / Virtual Camera land in 0.70. */
export function ControlsDock({
  settings,
  onSettingsSaved,
}: {
  settings: Settings | null;
  onSettingsSaved: (next: Settings) => void;
}) {
  const [rec, setRec] = useState<RecordingStatus | null>(null);
  const [busy, setBusy] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);
  const [dialog, setDialog] = useState<"components" | "output" | null>(null);

  useEffect(() => {
    let alive = true;
    let unlisten: (() => void) | undefined;
    recordingStatus()
      .then((status) => alive && setRec(status))
      .catch(() => alive && setRec(null));
    onRecording((status) => setRec(status)).then((fn) => {
      if (alive) unlisten = fn;
      else fn();
    });
    return () => {
      alive = false;
      unlisten?.();
    };
  }, []);

  const active = rec?.state === "recording" || rec?.state === "paused";
  const finalizing = rec?.state === "finalizing";

  const startStop = async () => {
    setBusy(true);
    setActionError(null);
    try {
      if (active) {
        await recordingStop();
      } else {
        await recordingStart();
      }
    } catch (error) {
      const message = String(error);
      setActionError(message);
      // The honest ffmpeg gate: recording a wire container without the
      // component lands the user in the labeled panel, one click away.
      if (message.includes("ffmpeg component")) {
        setDialog("components");
      }
    } finally {
      setBusy(false);
    }
  };

  const pauseResume = async () => {
    setBusy(true);
    setActionError(null);
    try {
      if (rec?.state === "paused") {
        await recordingResume();
      } else {
        await recordingPause();
      }
    } catch (error) {
      setActionError(String(error));
    } finally {
      setBusy(false);
    }
  };

  // The last session's error surfaces until the next action clears it.
  const sessionError = rec?.state === "idle" ? rec.error : null;
  const shownError = actionError ?? sessionError;

  return (
    <Panel title="Controls">
      <div className="flex flex-col gap-2">
        <button
          type="button"
          disabled={busy || finalizing || !settings}
          onClick={startStop}
          title={
            active
              ? "Stop and finalize the recording"
              : "Record the program feed with the Settings → Output configuration"
          }
          className={`${buttonBase} ${
            active
              ? "border-red-500/50 bg-red-500/15 text-havoc-text hover:border-red-400/70"
              : "border-white/10 bg-white/[0.04] text-havoc-text hover:border-havoc-accent/50"
          }`}
        >
          {finalizing ? (
            "◌ Finalizing…"
          ) : active && rec ? (
            <span className="flex items-center justify-between gap-2">
              <span>■ Stop Recording</span>
              <RecDot
                paused={rec.state === "paused"}
                durationSec={
                  rec.state === "recording" || rec.state === "paused" ? rec.durationSec : 0
                }
                tracks={rec.state === "recording" || rec.state === "paused" ? rec.tracks : 0}
              />
            </span>
          ) : (
            "● Start Recording"
          )}
        </button>
        {active && (
          <button
            type="button"
            disabled={busy}
            onClick={pauseResume}
            title={
              rec?.state === "paused"
                ? "Resume — the file continues as one contiguous timeline"
                : "Pause — no frames are written; resuming continues the same playable file"
            }
            className={`${buttonBase} ${
              rec?.state === "paused"
                ? "border-amber-400/50 bg-amber-400/15 text-havoc-text hover:border-amber-300/70"
                : "border-white/10 bg-white/[0.04] text-havoc-text hover:border-amber-400/50"
            }`}
          >
            {rec?.state === "paused" ? "▶ Resume Recording" : "⏸ Pause Recording"}
          </button>
        )}
        <button
          type="button"
          disabled
          title="Streaming arrives with the studio MVP (0.70.0)"
          className={`${buttonBase} border-havoc-accent/40 bg-gradient-to-r from-havoc-accent/20 to-havoc-accent-2/20 text-havoc-text`}
        >
          ⦿ Go Live
        </button>
        <button
          type="button"
          disabled
          title="The virtual camera arrives with the studio MVP (0.70.0)"
          className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-text`}
        >
          ⌁ Start Virtual Camera
        </button>
        <div className="grid grid-cols-2 gap-2">
          <button
            type="button"
            onClick={() => setDialog("output")}
            title="Recording format, folder, tracks, and splitting"
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            ⚙ Output…
          </button>
          <button
            type="button"
            onClick={() => setDialog("components")}
            title="The on-demand ffmpeg wire-codec component (clearly labeled, never bundled)"
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            ⬡ Components…
          </button>
        </div>
        {shownError && (
          <p role="alert" className="m-0 text-[11px] leading-snug break-words text-red-300">
            {shownError}
          </p>
        )}
        {rec?.state === "idle" && rec.lastPaths.length > 0 && !shownError && (
          <p title={rec.lastPaths.join("\n")} className="m-0 truncate text-[10px] text-havoc-muted">
            Saved: {rec.lastPaths[rec.lastPaths.length - 1]}
          </p>
        )}
      </div>
      {dialog === "components" && <ModelsDialog onClose={() => setDialog(null)} />}
      {dialog === "output" && (
        <SettingsOutput
          settings={settings}
          onSaved={onSettingsSaved}
          onClose={() => setDialog(null)}
          onOpenComponents={() => setDialog("components")}
        />
      )}
    </Panel>
  );
}
