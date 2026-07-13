import { useEffect, useState } from "react";

import { check } from "@tauri-apps/plugin-updater";

import {
  bugReportContext,
  openFrecPending,
  recordingAddMarker,
  studioSendReaction,
  recordingPause,
  recordingResume,
  recordingStart,
  recordingStatus,
  recordingStop,
} from "../api/commands";
import { onOpenFrec, onRecording } from "../api/events";
import type { RecordingStatus, Settings } from "../api/types";
import { LiveButton } from "../components/LiveButton";
import { Panel } from "../components/Panel";
import { RecDot } from "../components/RecDot";
import { ReplayControls } from "../components/ReplayControls";
import { useT } from "../i18n/t";
import { BrowserDockDialog } from "./BrowserDock";
import { BugReportDialog } from "./BugReport";
import { UpdatesDialog } from "./Updates";
import { AboutDialog } from "./About";
import { SettingsDialog } from "./Settings";
import { ModelsDialog } from "./Models";
import { OpenedFrecDialog } from "./OpenedFrec";
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
  onOpenSourceHealth,
}: {
  settings: Settings | null;
  onSettingsSaved: (next: Settings) => void;
  /** The pre-flight's "sources" fix (CAP-M09) → the CAP-M13 dashboard. */
  onOpenSourceHealth: () => void;
}) {
  const t = useT();
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
    | "bug"
    | "updates"
    | "settings"
    | "about"
    | null
  >(null);

  const [openedFrec, setOpenedFrec] = useState<string | null>(null);

  // Auto-surface the bug-report dialog on startup when the app crashed on a
  // previous run — this is the "relaunch → report" half of the loop. If there is
  // no crash to report, check once for a new version instead and surface it, so
  // a user learns about an update without going looking for it. A crash always
  // wins the one dialog slot; the update keeps until next launch.
  //
  // The check is a single GET of the signed `latest.json`; nothing downloads
  // without the user answering. This runs after the EULA gate, because the whole
  // studio (and therefore this dock) is blocked behind it.
  useEffect(() => {
    let alive = true;
    bugReportContext()
      .then(async (ctx) => {
        if (!alive) return;
        if (ctx.pendingCrash) {
          setDialog("bug");
          return;
        }
        // Offline, rate-limited, or no release yet: stay silent, never nag.
        const update = await check().catch(() => null);
        if (alive && update) setDialog("updates");
      })
      .catch(() => undefined);
    return () => {
      alive = false;
    };
  }, []);

  // A .frec opened via the OS (cold start on load, or a second launch while
  // running) → offer to export it (Capture records .frec; Player plays it).
  useEffect(() => {
    let alive = true;
    let unlisten: (() => void) | undefined;
    openFrecPending()
      .then((path) => {
        if (alive && path) setOpenedFrec(path);
      })
      .catch(() => undefined);
    onOpenFrec((path) => setOpenedFrec(path))
      .then((fn) => (alive ? (unlisten = fn) : fn()))
      .catch(() => undefined);
    return () => {
      alive = false;
      unlisten?.();
    };
  }, []);

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
    <Panel title={t("controls-title")}>
      <div className="flex flex-col gap-2">
        <button
          type="button"
          disabled={busy || finalizing || !settings}
          onClick={startStop}
          title={
            active ? t("controls-start-stop-title-stop") : t("controls-start-stop-title-start")
          }
          className={`${buttonBase} ${
            active
              ? "border-red-500/50 bg-red-500/15 text-havoc-text hover:border-red-400/70"
              : "border-white/10 bg-white/[0.04] text-havoc-text hover:border-havoc-accent/50"
          }`}
        >
          {finalizing ? (
            t("controls-finalizing")
          ) : active && rec ? (
            <span className="flex items-center justify-between gap-2">
              <span>{t("controls-stop-recording")}</span>
              <RecDot
                paused={rec.state === "paused"}
                durationSec={
                  rec.state === "recording" || rec.state === "paused" ? rec.durationSec : 0
                }
                tracks={rec.state === "recording" || rec.state === "paused" ? rec.tracks : 0}
              />
            </span>
          ) : (
            t("controls-start-recording")
          )}
        </button>
        {active && (
          <button
            type="button"
            disabled={busy || rec?.state !== "recording"}
            onClick={() => {
              recordingAddMarker().catch((err) => setActionError(String(err)));
            }}
            title={t("controls-marker-title")}
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-text hover:border-havoc-accent/50`}
          >
            {t("controls-marker")}
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
                ? t("controls-pause-title-resume")
                : t("controls-pause-title-pause")
            }
            className={`${buttonBase} ${
              rec?.state === "paused"
                ? "border-amber-400/50 bg-amber-400/15 text-havoc-text hover:border-amber-300/70"
                : "border-white/10 bg-white/[0.04] text-havoc-text hover:border-amber-400/50"
            }`}
          >
            {rec?.state === "paused"
              ? t("controls-resume-recording")
              : t("controls-pause-recording")}
          </button>
        )}
        <LiveButton
          disabled={!settings}
          onNeedsComponents={() => setDialog("components")}
          onNeedsSettings={() => setDialog("stream")}
          onOpenSourceHealth={onOpenSourceHealth}
          onSettingsSaved={onSettingsSaved}
        />
        <ReplayControls disabled={!settings} onNeedsComponents={() => setDialog("components")} />
        <div
          className="flex items-center gap-1"
          role="group"
          aria-label={t("controls-reactions-label")}
          title={t("controls-reactions-title")}
        >
          {["❤", "🔥", "💯", "👏", "😂", "🎉"].map((emoji) => (
            <button
              key={emoji}
              type="button"
              onClick={() => {
                studioSendReaction(emoji).catch((err) => console.error("reaction failed:", err));
              }}
              aria-label={t("controls-react", { emoji })}
              className="flex-1 rounded-md border border-white/10 bg-white/[0.04] px-1 py-1 text-sm hover:border-havoc-accent/50"
            >
              {emoji}
            </button>
          ))}
        </div>
        <button
          type="button"
          disabled
          title={t("controls-virtual-camera-title")}
          className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-text`}
        >
          {t("controls-virtual-camera")}
        </button>
        <div className="grid grid-cols-3 gap-2">
          <button
            type="button"
            onClick={() => setDialog("recordings")}
            title={t("controls-files-title")}
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            {t("controls-files")}
          </button>
          <button
            type="button"
            onClick={() => setDialog("output")}
            title={t("controls-output-title")}
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            {t("controls-output")}
          </button>
          <button
            type="button"
            onClick={() => setDialog("stream")}
            title={t("controls-stream-title")}
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            {t("controls-stream")}
          </button>
          <button
            type="button"
            onClick={() => setDialog("components")}
            title={t("controls-codecs-title")}
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            {t("controls-codecs")}
          </button>
          <button
            type="button"
            onClick={() => setDialog("replay")}
            title={t("controls-replay-title")}
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            {t("controls-replay")}
          </button>
          <button
            type="button"
            onClick={() => setDialog("hotkeys")}
            title={t("controls-keys-title")}
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            {t("controls-keys")}
          </button>
          <button
            type="button"
            onClick={() => setDialog("scripts")}
            title={t("controls-scripts-title")}
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            {t("controls-scripts")}
          </button>
          <button
            type="button"
            onClick={() => setDialog("docks")}
            title={t("controls-docks-title")}
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            {t("controls-docks")}
          </button>
          <button
            type="button"
            onClick={() => setDialog("remote")}
            title={t("controls-remote-title")}
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            {t("controls-remote")}
          </button>
          <button
            type="button"
            onClick={() => setDialog("workspace")}
            title={t("controls-profiles-title")}
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            {t("controls-profiles")}
          </button>
          <button
            type="button"
            onClick={() => setDialog("settings")}
            title={t("controls-settings-title")}
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            {t("controls-settings")}
          </button>
          <button
            type="button"
            onClick={() => setDialog("bug")}
            title={t("controls-bug-title")}
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            {t("controls-bug")}
          </button>
          <button
            type="button"
            onClick={() => setDialog("updates")}
            title={t("controls-updates-title")}
            className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-muted hover:text-havoc-text`}
          >
            {t("controls-updates")}
          </button>
        </div>
        {shownError && (
          <p role="alert" className="m-0 text-[11px] leading-snug break-words text-red-300">
            {shownError}
          </p>
        )}
        {rec?.state === "idle" && rec.lastPaths.length > 0 && !shownError && (
          <p title={rec.lastPaths.join("\n")} className="m-0 truncate text-[10px] text-havoc-muted">
            {t("controls-saved", { path: rec.lastPaths[rec.lastPaths.length - 1] })}
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
      {dialog === "bug" && <BugReportDialog onClose={() => setDialog(null)} />}
      {dialog === "updates" && <UpdatesDialog onClose={() => setDialog(null)} />}
      {dialog === "settings" && settings && (
        <SettingsDialog
          settings={settings}
          onSettingsSaved={onSettingsSaved}
          onClose={() => setDialog(null)}
          onOpen={(next) => setDialog(next)}
        />
      )}
      {dialog === "about" && (
        <AboutDialog onClose={() => setDialog(null)} onCheckUpdates={() => setDialog("updates")} />
      )}
      {openedFrec && <OpenedFrecDialog path={openedFrec} onClose={() => setOpenedFrec(null)} />}
      {dialog === "workspace" && (
        <WorkspaceDialog onClose={() => setDialog(null)} onSettingsSaved={onSettingsSaved} />
      )}
    </Panel>
  );
}
