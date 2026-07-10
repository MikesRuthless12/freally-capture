import { useEffect, useRef, useState } from "react";

import { replayArm, replayDisarm, replaySave, replayStatus } from "../api/commands";
import { onReplay, onReplaySaved } from "../api/events";
import type { ReplayStatus } from "../api/types";
import { useT } from "../i18n/t";

const buttonBase =
  "w-full rounded-lg border px-3 py-2 text-left text-xs font-medium transition-colors disabled:cursor-not-allowed disabled:opacity-50";

/**
 * The replay buffer's controls (TASK-603): arm/disarm the rolling buffer and
 * save its last N seconds — the save also fires from the global hotkey, and
 * either path confirms with a transient toast + the `replay_saved` event.
 */
export function ReplayControls({
  disabled,
  onNeedsComponents,
}: {
  disabled: boolean;
  /** The honest ffmpeg gate: route the user to the labeled component panel. */
  onNeedsComponents: () => void;
}) {
  const t = useT();
  const [status, setStatus] = useState<ReplayStatus | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [toast, setToast] = useState<string | null>(null);
  const toastTimer = useRef<number | undefined>(undefined);

  useEffect(() => {
    let alive = true;
    const cleanups: Array<() => void> = [];
    replayStatus()
      .then((current) => alive && setStatus(current))
      .catch(() => alive && setStatus(null));
    onReplay((next) => setStatus(next))
      .then((fn) => {
        if (alive) cleanups.push(fn);
        else fn();
      })
      .catch(() => undefined);
    onReplaySaved(({ path }) => {
      const name = path.split(/[\\/]/).pop() ?? path;
      setToast(t("replaycontrols-saved", { name }));
      window.clearTimeout(toastTimer.current);
      toastTimer.current = window.setTimeout(() => setToast(null), 5000);
    })
      .then((fn) => {
        if (alive) cleanups.push(fn);
        else fn();
      })
      .catch(() => undefined);
    return () => {
      alive = false;
      window.clearTimeout(toastTimer.current);
      cleanups.forEach((fn) => fn());
    };
  }, [t]);

  const armed = status?.armed ?? false;

  const toggle = async () => {
    setBusy(true);
    setError(null);
    try {
      if (armed) {
        await replayDisarm();
      } else {
        await replayArm();
      }
    } catch (raw) {
      const message = String(raw);
      setError(message);
      if (message.includes("ffmpeg component")) {
        onNeedsComponents();
      }
    } finally {
      setBusy(false);
    }
  };

  const save = async () => {
    setBusy(true);
    setError(null);
    try {
      await replaySave();
    } catch (raw) {
      setError(String(raw));
    } finally {
      setBusy(false);
    }
  };

  const failure =
    status?.state === "failed" ? (status.error ?? t("replaycontrols-failure-stopped")) : null;
  const shownError = error ?? failure;

  return (
    <>
      <div className="flex gap-2">
        <button
          type="button"
          disabled={disabled || busy}
          onClick={toggle}
          title={armed ? t("replaycontrols-title-disarm") : t("replaycontrols-title-arm")}
          className={`${buttonBase} min-w-0 flex-1 ${
            armed
              ? "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text hover:border-havoc-accent/80"
              : "border-white/10 bg-white/[0.04] text-havoc-text hover:border-havoc-accent/50"
          }`}
        >
          {armed && status ? (
            <span className="flex items-center justify-between gap-2">
              <span>{t("replaycontrols-replay-seconds", { seconds: status.seconds })}</span>
              <span
                aria-label={status.state}
                className={`inline-block h-2 w-2 rounded-full ${
                  status.state === "buffering"
                    ? "bg-emerald-400"
                    : status.state === "recovering"
                      ? "animate-pulse bg-amber-400"
                      : "bg-red-500"
                }`}
              />
            </span>
          ) : (
            t("replaycontrols-arm")
          )}
        </button>
        {armed && (
          <button
            type="button"
            disabled={busy}
            onClick={save}
            title={t("replaycontrols-save-title")}
            className={`${buttonBase} w-auto shrink-0 border-white/10 bg-white/[0.04] text-havoc-text hover:border-havoc-accent/50`}
          >
            {t("replaycontrols-save")}
          </button>
        )}
      </div>
      {toast && (
        <p
          role="status"
          className="m-0 rounded-md border border-emerald-400/40 bg-emerald-400/10 px-2 py-1 text-[11px] leading-snug break-words text-emerald-200"
        >
          {toast}
        </p>
      )}
      {shownError && (
        <p role="alert" className="m-0 text-[11px] leading-snug break-words text-red-300">
          {shownError}
        </p>
      )}
    </>
  );
}
