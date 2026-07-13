import { useEffect, useState } from "react";

import { streamStart, streamStatus, streamStop } from "../api/commands";
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

  useEffect(() => {
    let alive = true;
    let unlisten: (() => void) | undefined;
    streamStatus()
      .then((current) => alive && setStatus(current))
      .catch(() => alive && setStatus(null));
    onStream((next) => setStatus(next))
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

  const start = async () => {
    setBusy(true);
    setError(null);
    try {
      await streamStart();
    } catch (raw) {
      const message = String(raw);
      setError(message);
      if (message.includes("ffmpeg component")) {
        onNeedsComponents();
      } else if (message.includes("stream key") || message.includes("ingest")) {
        onNeedsSettings();
      }
    } finally {
      setBusy(false);
    }
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
    try {
      await streamStop();
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
        title={live ? t("livebutton-title-live") : t("livebutton-title-offline")}
        className={`${buttonBase} ${
          live
            ? "border-red-500/60 bg-red-500/15 text-havoc-text hover:border-red-400/80"
            : "border-havoc-accent/40 bg-gradient-to-r from-havoc-accent/20 to-havoc-accent-2/20 text-havoc-text hover:border-havoc-accent/70"
        }`}
      >
        {live && status ? (
          <span className="flex items-center justify-between gap-2">
            <span>{t("livebutton-end-stream")}</span>
            <span className="inline-flex items-center gap-1.5" role="status">
              <span
                aria-label={
                  status.state === "reconnecting"
                    ? t("livebutton-aria-reconnecting")
                    : t("livebutton-aria-live")
                }
                className={`inline-block h-2 w-2 rounded-full ${
                  status.state === "reconnecting"
                    ? "animate-pulse bg-amber-400"
                    : "animate-pulse bg-red-500"
                }`}
              />
              <span className="text-[10px] font-bold tracking-widest text-red-300 uppercase">
                {status.state === "reconnecting"
                  ? t("livebutton-badge-retry", { n: status.reconnects + 1 })
                  : t("livebutton-badge-live")}
              </span>
              <span className="font-mono text-xs tabular-nums">{formatHms(status.elapsedSec)}</span>
            </span>
          </span>
        ) : (
          t("livebutton-go-live")
        )}
      </button>
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
