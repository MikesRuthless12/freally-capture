import { useCallback, useEffect, useState } from "react";

import { relaunch } from "@tauri-apps/plugin-process";
import { check, type Update } from "@tauri-apps/plugin-updater";

import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

type Phase =
  | { kind: "checking" }
  | { kind: "uptodate" }
  | { kind: "available"; update: Update }
  | { kind: "downloading"; version: string; pct: number | null }
  | { kind: "installed" }
  | { kind: "error"; message: string };

/**
 * Self-hosted auto-updater (TASK-803). Checks the signed `latest.json` on the
 * GitHub releases endpoint (see `tauri.conf.json` → plugins.updater); every
 * download is verified against the bundled minisign public key before it is
 * applied — an unsigned or tampered package is refused by the plugin, never by
 * this UI. Nothing is downloaded without an explicit click.
 */
export function UpdatesDialog({ onClose }: { onClose: () => void }) {
  const t = useT();
  const [phase, setPhase] = useState<Phase>({ kind: "checking" });

  const runCheck = useCallback(async () => {
    setPhase({ kind: "checking" });
    try {
      const update = await check();
      setPhase(update ? { kind: "available", update } : { kind: "uptodate" });
    } catch (err) {
      setPhase({ kind: "error", message: String(err) });
    }
  }, []);

  useEffect(() => {
    // Rely on `alive` (not a ref guard): under React StrictMode the effect runs
    // setup→cleanup→setup, and a ref guard would swallow the second setup's
    // check while the first's result was already discarded — leaving the dialog
    // stuck on "checking". Re-issuing the check on the second setup is harmless.
    let alive = true;
    // Initial state is already "checking" — set state only in the async callback
    // (never synchronously inside the effect).
    check()
      .then((update) => {
        if (alive) setPhase(update ? { kind: "available", update } : { kind: "uptodate" });
      })
      .catch((err) => {
        if (alive) setPhase({ kind: "error", message: String(err) });
      });
    return () => {
      alive = false;
    };
  }, []);

  // On Windows this never returns: after verifying the package the plugin
  // `ShellExecuteW`s the NSIS installer and calls `std::process::exit(0)` so the
  // old binary is not locked while it is replaced (`/P /R` = passive install,
  // then relaunch). The "installed" phase and its `relaunch()` below are
  // therefore reachable only on macOS and Linux, where the bundle is swapped in
  // place and the app must restart itself.
  const install = useCallback(async (update: Update) => {
    let downloaded = 0;
    let total = 0;
    setPhase({ kind: "downloading", version: update.version, pct: null });
    try {
      await update.downloadAndInstall((event) => {
        if (event.event === "Started") {
          total = event.data.contentLength ?? 0;
        } else if (event.event === "Progress") {
          downloaded += event.data.chunkLength;
          const pct = total > 0 ? Math.min(100, (downloaded / total) * 100) : null;
          setPhase({ kind: "downloading", version: update.version, pct });
        }
      });
      setPhase({ kind: "installed" });
    } catch (err) {
      setPhase({ kind: "error", message: String(err) });
    }
  }, []);

  return (
    <PickerShell title={t("updates-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        {phase.kind === "checking" && (
          <p className="m-0 text-havoc-muted">{t("updates-checking")}</p>
        )}

        {phase.kind === "uptodate" && (
          <>
            <p className="m-0 text-emerald-300">{t("updates-uptodate")}</p>
            <div className="flex gap-2">
              <button type="button" onClick={runCheck} className={secondaryBtn}>
                {t("updates-check-again")}
              </button>
            </div>
          </>
        )}

        {phase.kind === "available" && (
          <>
            <p className="m-0">
              {t("updates-available", { version: phase.update.version })}
              {phase.update.currentVersion ? (
                <span className="text-havoc-muted">
                  {" "}
                  {t("updates-current-version", { current: phase.update.currentVersion })}
                </span>
              ) : null}
              .
            </p>
            {phase.update.body ? (
              <div className="flex flex-col gap-1">
                <label
                  htmlFor="release-notes"
                  className="text-[10px] tracking-wide text-havoc-muted uppercase"
                >
                  {t("updates-release-notes-label", { version: phase.update.version })}
                </label>
                <textarea
                  id="release-notes"
                  readOnly
                  value={phase.update.body}
                  rows={10}
                  className="m-0 resize-none rounded-md border border-white/10 bg-black/30 px-2 py-1.5 font-mono text-[10px] leading-snug text-havoc-muted outline-none focus:border-havoc-accent/60"
                />
              </div>
            ) : null}
            <p className="m-0 text-[11px] leading-snug text-havoc-muted">{t("updates-confirm")}</p>
            <div className="flex flex-wrap gap-2">
              <button type="button" onClick={() => install(phase.update)} className={primaryBtn}>
                {t("updates-yes-update-now")}
              </button>
              <button type="button" onClick={onClose} className={secondaryBtn}>
                {t("updates-no-not-now")}
              </button>
            </div>
          </>
        )}

        {phase.kind === "downloading" && (
          <div className="flex flex-col gap-1.5">
            <div className="flex items-baseline justify-between">
              <span>{t("updates-downloading", { version: phase.version })}</span>
              <span className="font-mono text-havoc-muted">
                {phase.pct !== null ? `${phase.pct.toFixed(2)}%` : t("updates-starting")}
              </span>
            </div>
            <div className="h-1.5 overflow-hidden rounded-full bg-white/10">
              <div
                className="h-full rounded-full bg-gradient-to-r from-havoc-accent to-havoc-accent-2 transition-[width]"
                style={{ width: phase.pct !== null ? `${phase.pct.toFixed(2)}%` : "8%" }}
              />
            </div>
          </div>
        )}

        {phase.kind === "installed" && (
          <>
            <p className="m-0 text-emerald-300">{t("updates-installed")}</p>
            <div className="flex gap-2">
              <button
                type="button"
                onClick={() => {
                  void relaunch();
                }}
                className={primaryBtn}
              >
                {t("updates-restart-now")}
              </button>
              <button type="button" onClick={onClose} className={secondaryBtn}>
                {t("updates-restart-later")}
              </button>
            </div>
          </>
        )}

        {phase.kind === "error" && (
          <>
            <p role="alert" className="m-0 text-[11px] break-words text-red-300">
              {phase.message}
            </p>
            <div className="flex gap-2">
              <button type="button" onClick={runCheck} className={secondaryBtn}>
                {t("updates-try-again")}
              </button>
            </div>
          </>
        )}
      </div>
    </PickerShell>
  );
}

const primaryBtn =
  "rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25";
const secondaryBtn =
  "rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text";
