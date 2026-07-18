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
import { WhatsNewDialog } from "./WhatsNew";
import { AboutDialog } from "./About";
import { SettingsDialog } from "./Settings";
import { ModelsDialog } from "./Models";
import { MoreAppsDialog } from "./MoreApps";
import { OpenedFrecDialog } from "./OpenedFrec";
import { RecordingsDialog } from "./Recordings";
import { AutomationDialog } from "./AutomationDialog";
import { MidiDialog } from "./MidiDialog";
import { PtzDialog } from "./PtzDialog";
import { RundownDialog } from "./RundownDialog";
import { ScriptsDialog } from "./ScriptsDialog";
import { TeleprompterDialog } from "./Teleprompter";
import { SettingsHotkeys } from "./SettingsHotkeys";
import { SettingsPanel } from "./SettingsPanel";
import { SettingsRemote } from "./SettingsRemote";
import { SettingsOutput } from "./SettingsOutput";
import { SettingsReplay } from "./SettingsReplay";
import { SettingsStream } from "./SettingsStream";
import { WorkspaceDialog } from "./WorkspaceDialog";

const buttonBase =
  "w-full rounded-lg border px-3 py-2 text-left text-xs font-medium transition-colors disabled:cursor-not-allowed disabled:opacity-50";

/** Every dialog the dock can open — the menu bar's launchers name these. */
export type ControlsDialogKind =
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
  | "automation"
  | "rundown"
  | "panel"
  | "ptz"
  | "teleprompter"
  | "midi"
  | "bug"
  | "updates"
  | "whatsnew"
  | "settings"
  | "about"
  | "moreapps";

/** The Controls dock: recording (P4); Go Live / Virtual Camera land in 0.70. */
export function ControlsDock({
  settings,
  sceneNames,
  onSettingsSaved,
  onOpenSourceHealth,
  menuOpenRef,
}: {
  settings: Settings | null;
  /** The collection's scene names — the rundown's step targets (CAP-N09). */
  sceneNames: string[];
  onSettingsSaved: (next: Settings) => void;
  /** The pre-flight's "sources" fix (CAP-M09) → the CAP-M13 dashboard. */
  onOpenSourceHealth: () => void;
  /** The menu-bar seam: while mounted, the dock parks its dialog opener here
   * so the in-app menus can launch the same dialogs without owning their
   * state. A ref (not a consumed prop) because a synchronous setState in a
   * consuming effect trips `react-hooks/set-state-in-effect`. */
  menuOpenRef?: React.RefObject<((kind: ControlsDialogKind) => void) | null>;
}) {
  const t = useT();
  const [rec, setRec] = useState<RecordingStatus | null>(null);
  const [busy, setBusy] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);
  const [dialog, setDialog] = useState<ControlsDialogKind | null>(null);

  const [openedFrec, setOpenedFrec] = useState<string | null>(null);

  useEffect(() => {
    if (!menuOpenRef) return;
    menuOpenRef.current = setDialog;
    return () => {
      menuOpenRef.current = null;
    };
  }, [menuOpenRef]);

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
        {rec?.state === "recording" && rec.isoLanes > 0 && (
          <p className="m-0 text-[10px] text-havoc-muted">
            {t("controls-iso-lanes", { count: rec.isoLanes })}
          </p>
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
        {/* The dialog LAUNCHERS moved to the menu bar (File/Tools/Help/…);
            only the live-operation controls stay down here. The dialogs
            themselves still render below — the menus open them through
            `menuOpenRef`, and the crash-report / update-available auto-surface
            keeps working. The removed buttons' i18n keys are left in place. */}
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
      {dialog === "automation" && (
        <AutomationDialog
          settings={settings}
          onSaved={onSettingsSaved}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog === "rundown" && (
        <RundownDialog
          settings={settings}
          sceneNames={sceneNames}
          onSaved={onSettingsSaved}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog === "panel" && (
        <SettingsPanel
          settings={settings}
          onSaved={onSettingsSaved}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog === "ptz" && (
        <PtzDialog
          settings={settings}
          sceneNames={sceneNames}
          onSaved={onSettingsSaved}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog === "midi" && (
        <MidiDialog
          settings={settings}
          sceneNames={sceneNames}
          onSaved={onSettingsSaved}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog === "teleprompter" && <TeleprompterDialog onClose={() => setDialog(null)} />}
      {dialog === "bug" && <BugReportDialog onClose={() => setDialog(null)} />}
      {dialog === "updates" && <UpdatesDialog onClose={() => setDialog(null)} />}
      {dialog === "whatsnew" && <WhatsNewDialog onClose={() => setDialog(null)} />}
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
      {dialog === "moreapps" && <MoreAppsDialog onClose={() => setDialog(null)} />}
      {openedFrec && <OpenedFrecDialog path={openedFrec} onClose={() => setOpenedFrec(null)} />}
      {dialog === "workspace" && (
        <WorkspaceDialog onClose={() => setDialog(null)} onSettingsSaved={onSettingsSaved} />
      )}
    </Panel>
  );
}
