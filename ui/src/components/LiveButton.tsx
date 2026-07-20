import { useEffect, useRef, useState } from "react";

import {
  recordingStart,
  recordingStatus,
  recordingStop,
  streamStart,
  streamStartRehearsal,
  streamStatus,
  streamStop,
} from "../api/commands";
import { onStream } from "../api/events";
import type { Settings, StreamStatus } from "../api/types";
import { PreflightDialog } from "../panels/PreflightDialog";
import { useT } from "../i18n/t";
import { formatHms } from "../lib/time";

const buttonBase =
  "w-full rounded-lg border px-3 py-2 text-left text-xs font-medium transition-colors disabled:cursor-not-allowed disabled:opacity-50";

/**
 * Go Live → End Stream (TASK-506): the pulsing LIVE state with the elapsed
 * HH:MM:SS clock ticking from Go Live to End Stream, honest reconnect and
 * failure states, all driven by the ~1 Hz `stream` events.
 */
export function LiveButton({
  disabled,
  onNeedsComponents,
  onNeedsSettings,
  onOpenSourceHealth,
  onSettingsSaved,
}: {
  disabled: boolean;
  /** The honest ffmpeg gate: route the user to the labeled component panel. */
  onNeedsComponents: () => void;
  /** A missing key/ingest routes to the Stream settings. */
  onNeedsSettings: () => void;
  /** The pre-flight's "sources" fix opens the health dashboard (CAP-M13). */
  onOpenSourceHealth: () => void;
  /** The pre-flight's hold toggle saves settings — keep App's copy fresh. */
  onSettingsSaved: (next: Settings) => void;
}) {
  const t = useT();
  const [status, setStatus] = useState<StreamStatus | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  // The go-live pre-flight checklist (CAP-M09).
  const [preflight, setPreflight] = useState(false);
  // Task #10: did WE auto-start a recording for the current rehearsal? (So we
  // stop only our own recording when the rehearsal ends — never one the user
  // started by hand.)
  const autoRecording = useRef(false);

  useEffect(() => {
    let alive = true;
    let unlisten: (() => void) | undefined;
    streamStatus()
      .then((current) => alive && setStatus(current))
      .catch(() => alive && setStatus(null));
    onStream((next) => {
      setStatus(next);
      // A session that ends by ANY path (failure after spent retries, panic,
      // automation) invalidates the auto-recording claim — without this, a
      // later End Stream would stop a recording the user started by hand.
      if (next.state !== "live" && next.state !== "reconnecting") {
        autoRecording.current = false;
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
  }, []);

  const live = status?.state === "live" || status?.state === "reconnecting";
  // CAP-N49: a running dry run — every LIVE surface restyles so a rehearsal
  // can never read as a real broadcast.
  const rehearsing = live && status?.rehearsal === true;

  // Go Live and Rehearse share one launch path; only the command and which
  // error substrings route to Stream settings differ.
  const launch = async (invoke: () => Promise<void>, settingsMarkers: string[]) => {
    setBusy(true);
    setError(null);
    try {
      await invoke();
    } catch (raw) {
      const message = String(raw);
      setError(message);
      if (message.includes("ffmpeg component")) {
        onNeedsComponents();
      } else if (settingsMarkers.some((marker) => message.includes(marker))) {
        onNeedsSettings();
      }
    } finally {
      setBusy(false);
    }
  };

  const start = () => launch(streamStart, ["stream key", "ingest"]);
  // CAP-N49: the rehearsal is itself the check, so it skips the pre-flight
  // dialog (and never reads keys) — a keyless pre-show dry run must work.
  // Task #10: offer to ALSO record the dry run to a file (a keeper you can
  // edit/upload yourself) — only when nothing is recording already, and we
  // stop that recording ourselves when the rehearsal ends.
  const rehearse = async () => {
    let alsoRecord = false;
    try {
      const rec = await recordingStatus();
      if (rec.state === "idle") {
        alsoRecord = window.confirm(t("livebutton-rehearse-record-confirm"));
      }
    } catch {
      // No recording status — skip the offer rather than block the rehearsal.
    }
    // The shared launch path (busy/error + component/settings routing), with
    // the optional recording chained after the rehearsal starts.
    await launch(async () => {
      await streamStartRehearsal();
      if (alsoRecord) {
        await recordingStart();
        autoRecording.current = true;
      }
    }, ["stream target"]);
  };

  const toggle = async () => {
    if (!live) {
      // CAP-M09: Go Live runs through the pre-flight checklist first.
      setError(null);
      setPreflight(true);
      return;
    }
    setBusy(true);
    setError(null);
    // Read the claim BEFORE stopping: the stream event that lands during the
    // await clears the ref (any non-live state does), which would otherwise
    // leave our own rehearsal recording running.
    const wasAutoRecording = autoRecording.current;
    autoRecording.current = false;
    try {
      await streamStop();
      if (wasAutoRecording) {
        await recordingStop();
      }
    } catch (raw) {
      setError(String(raw));
    } finally {
      setBusy(false);
    }
  };

  // A spent-retries failure surfaces until the next action clears it.
  const failure =
    status?.state === "failed" ? (status.error ?? t("livebutton-failure-ended")) : null;
  const shownError = error ?? failure;

  return (
    <>
      <button
        type="button"
        disabled={disabled || busy}
        onClick={toggle}
        title={
          rehearsing
            ? t("livebutton-title-rehearsing")
            : live
              ? t("livebutton-title-live")
              : t("livebutton-title-offline")
        }
        className={`${buttonBase} ${
          rehearsing
            ? "border-violet-500/60 bg-violet-500/15 text-havoc-text hover:border-violet-400/80"
            : live
              ? "border-red-500/60 bg-red-500/15 text-havoc-text hover:border-red-400/80"
              : "border-havoc-accent/40 bg-gradient-to-r from-havoc-accent/20 to-havoc-accent-2/20 text-havoc-text hover:border-havoc-accent/70"
        }`}
      >
        {live && status ? (
          <span className="flex items-center justify-between gap-2">
            <span>{rehearsing ? t("livebutton-end-rehearsal") : t("livebutton-end-stream")}</span>
            <span className="inline-flex items-center gap-1.5" role="status">
              <span
                aria-label={
                  status.state === "reconnecting"
                    ? t("livebutton-aria-reconnecting")
                    : rehearsing
                      ? t("livebutton-aria-rehearsal")
                      : t("livebutton-aria-live")
                }
                className={`inline-block h-2 w-2 rounded-full ${
                  status.state === "reconnecting"
                    ? "animate-pulse bg-amber-400"
                    : rehearsing
                      ? "animate-pulse bg-violet-400"
                      : "animate-pulse bg-red-500"
                }`}
              />
              <span
                className={`text-[10px] font-bold tracking-widest uppercase ${
                  rehearsing ? "text-violet-300" : "text-red-300"
                }`}
              >
                {status.state === "reconnecting"
                  ? t("livebutton-badge-retry", { n: status.reconnects + 1 })
                  : rehearsing
                    ? t("livebutton-badge-rehearsal")
                    : t("livebutton-badge-live")}
              </span>
              <span className="font-mono text-xs tabular-nums">{formatHms(status.elapsedSec)}</span>
            </span>
          </span>
        ) : (
          t("livebutton-go-live")
        )}
      </button>
      {!live && (
        <button
          type="button"
          disabled={disabled || busy}
          onClick={() => void rehearse()}
          title={t("livebutton-rehearse-title")}
          className={`${buttonBase} border-violet-500/40 bg-violet-500/10 text-havoc-text hover:border-violet-400/70`}
        >
          {t("livebutton-rehearse")}
        </button>
      )}
      {rehearsing && (
        <p
          role="status"
          className="m-0 rounded border border-violet-500/40 bg-violet-500/10 px-2 py-1 text-[11px] leading-snug text-violet-300"
        >
          {t("livebutton-rehearsal-banner")}
          {/* CAP-N48: name the armed network drill, so a capped/flapping
              rehearsal is never mistaken for real trouble. */}
          {status?.simulator === "hotelWifi" && ` · ${t("stream-simulator-hotel-wifi")}`}
          {status?.simulator === "mobileHotspot" && ` · ${t("stream-simulator-mobile-hotspot")}`}
          {status?.simulator === "custom" && ` · ${t("stream-simulator-custom")}`}
        </p>
      )}
      {shownError && (
        <p role="alert" className="m-0 text-[11px] leading-snug break-words text-red-300">
          {shownError}
        </p>
      )}
      {preflight && (
        <PreflightDialog
          onSettingsSaved={onSettingsSaved}
          onClose={() => setPreflight(false)}
          onProceed={() => {
            setPreflight(false);
            void start();
          }}
          onOpenStreamSettings={() => {
            setPreflight(false);
            onNeedsSettings();
          }}
          onOpenComponents={() => {
            setPreflight(false);
            onNeedsComponents();
          }}
          onOpenSourceHealth={() => {
            setPreflight(false);
            onOpenSourceHealth();
          }}
        />
      )}
    </>
  );
}
