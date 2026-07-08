import { useEffect, useState } from "react";

import {
  recordingAddMarker,
  studioSendReaction,
  recordingPause,
  recordingResume,
  recordingStart,
  recordingStatus,
  recordingStop,
} from "../api/commands";
import { onRecording } from "../api/events";
import type { RecordingStatus, Settings } from "../api/types";
import { LiveButton } from "../components/LiveButton";
import { Panel } from "../components/Panel";
import { RecDot } from "../components/RecDot";
import { ReplayControls } from "../components/ReplayControls";
import { BrowserDockDialog } from "./BrowserDock";
import { ModelsDialog } from "./Models";
import { RecordingsDialog } from "./Recordings";
import { ScriptsDialog } from "./ScriptsDialog";
import { SettingsHotkeys } from "./SettingsHotkeys";
import { SettingsRemote } from "./SettingsRemote";
import { SettingsOutput } from "./SettingsOutput";
import { SettingsReplay } from "./SettingsReplay";
import { SettingsStream } from "./SettingsStream";
import { WorkspaceDialog } from "./WorkspaceDialog";

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
  const [dialog, setDialog] = useState<
    | "components"
    | "output"
    | "stream"
    | "hotkeys"
    | "workspace"
    | "recordings"
    | "replay"
    | "remote"
    | "docks"
    | "scripts"
    | null
  >(null);

  useEffect(() => {
    let alive = true;
    let unlisten: (() => void) | undefined;
    recordingStatus()
      .then((status) => alive && setRec(status))
      .catch(() => alive && setRec(null));
    onRecording((status) => setRec(status))
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
            disabled={busy || rec?.state !== "recording"}
            onClick={() => {
              recordingAddMarker().catch((err) => setActionError(String(err)));
            }}
            title="Drop a chapter marker at this moment — it lands in the RECORDING (mkv chapters, or a sidecar file). Platform-side stream markers need platform accounts, which this app never asks for."
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-text hover:border-havoc-accent/50`}
          >
            ◈ Marker
            {rec?.state === "recording" && rec.markers > 0 ? ` (${rec.markers})` : ""}
          </button>
        )}
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
        <LiveButton
          disabled={!settings}
          onNeedsComponents={() => setDialog("components")}
          onNeedsSettings={() => setDialog("stream")}
        />
        <ReplayControls disabled={!settings} onNeedsComponents={() => setDialog("components")} />
        <div
          className="flex items-center gap-1"
          role="group"
          aria-label="Reactions (baked into the program)"
          title="Float a reaction over the program — recorded AND streamed, so the replay shows the exact moment. Viewers in chat trigger these too (their reaction emoji float automatically); a flood only caps what's on screen."
        >
          {["❤", "🔥", "💯", "👏", "😂", "🎉"].map((emoji) => (
            <button
              key={emoji}
              type="button"
              onClick={() => {
                studioSendReaction(emoji).catch((err) => console.error("reaction failed:", err));
              }}
              aria-label={`React ${emoji}`}
              className="flex-1 rounded-md border border-white/10 bg-white/[0.04] px-1 py-1 text-sm hover:border-havoc-accent/50"
            >
              {emoji}
            </button>
          ))}
        </div>
        <button
          type="button"
          disabled
          title="The virtual camera needs its own signed driver component per OS (Win11 MFCreateVirtualCamera / Win10 DirectShow / macOS CoreMediaIO extension / Linux v4l2loopback) — it ships as its own milestone. The feed model is ready for it: program, vertical canvas, or a single source, with a paired virtual mic on Windows/Linux (macOS has no virtual-mic API — said honestly)."
          className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-text`}
        >
          ⌁ Start Virtual Camera
        </button>
        <div className="grid grid-cols-3 gap-2">
          <button
            type="button"
            onClick={() => setDialog("recordings")}
            title="Finished recordings + the remux-to-mp4 action"
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            ▤ Files…
          </button>
          <button
            type="button"
            onClick={() => setDialog("output")}
            title="Recording format, encoder, folder, tracks, and splitting"
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            ⚙ Output…
          </button>
          <button
            type="button"
            onClick={() => setDialog("stream")}
            title="Go Live target: service, stream key, encoder, bitrate"
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            ⦿ Stream…
          </button>
          <button
            type="button"
            onClick={() => setDialog("components")}
            title="The on-demand ffmpeg wire-codec component (clearly labeled, never bundled)"
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            ⬡ Codecs…
          </button>
          <button
            type="button"
            onClick={() => setDialog("replay")}
            title="Replay buffer length + quality presets"
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            ⟲ Replay…
          </button>
          <button
            type="button"
            onClick={() => setDialog("hotkeys")}
            title="Global hotkeys: record, Go Live, transition, save replay"
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            ⌨ Keys…
          </button>
          <button
            type="button"
            onClick={() => setDialog("scripts")}
            title="Sandboxed Lua scripts: react to go-live/scene/recording events, drive the studio"
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            ⚡ Scripts…
          </button>
          <button
            type="button"
            onClick={() => setDialog("docks")}
            title="Browser docks: open a chat popout, alerts page, or Companion buttons as a window beside the studio"
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            ⧉ Docks…
          </button>
          <button
            type="button"
            onClick={() => setDialog("remote")}
            title="WebSocket remote API for Stream Deck / Companion controllers (off by default)"
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            ⌁ Remote…
          </button>
          <button
            type="button"
            onClick={() => setDialog("workspace")}
            title="Profiles (settings) + scene collections — switchable snapshots"
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            ▣ Profiles…
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
      {dialog === "recordings" && <RecordingsDialog onClose={() => setDialog(null)} />}
      {dialog === "output" && (
        <SettingsOutput
          settings={settings}
          onSaved={onSettingsSaved}
          onClose={() => setDialog(null)}
          onOpenComponents={() => setDialog("components")}
        />
      )}
      {dialog === "stream" && (
        <SettingsStream
          settings={settings}
          onSaved={onSettingsSaved}
          onClose={() => setDialog(null)}
          onOpenComponents={() => setDialog("components")}
        />
      )}
      {dialog === "hotkeys" && (
        <SettingsHotkeys
          settings={settings}
          onSaved={onSettingsSaved}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog === "replay" && (
        <SettingsReplay
          settings={settings}
          onSaved={onSettingsSaved}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog === "remote" && (
        <SettingsRemote
          settings={settings}
          onSaved={onSettingsSaved}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog === "docks" && (
        <BrowserDockDialog
          settings={settings}
          onSaved={onSettingsSaved}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog === "scripts" && (
        <ScriptsDialog
          settings={settings}
          onSaved={onSettingsSaved}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog === "workspace" && (
        <WorkspaceDialog onClose={() => setDialog(null)} onSettingsSaved={onSettingsSaved} />
      )}
    </Panel>
  );
}
