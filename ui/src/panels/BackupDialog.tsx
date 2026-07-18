import { useState } from "react";
import { open, save } from "@tauri-apps/plugin-dialog";

import {
  backupExport,
  backupInspect,
  backupRestore,
  settingsGet,
  type BackupManifest,
  type RestoreSelection,
} from "../api/commands";
import type { Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { formatBytes } from "../lib/format";
import { applyTheme } from "../theme/theme";
import { initLocale, useT } from "../i18n/t";

const BACKUP_FILTER = { name: "Freally Capture backup", extensions: ["fcapbackup"] };

/**
 * CAP-N64: one-file backup & restore — export the whole studio config to a
 * single `.fcapbackup` and restore it (selectively) here or on a new machine.
 * Secrets (stream keys, passwords) are never included; the operator re-enters
 * them after a restore. Strictly local.
 */
export function BackupDialog({
  onClose,
  onRestored,
  onSettingsSaved,
}: {
  onClose: () => void;
  /** Called after a restore lands so the parent can refresh. */
  onRestored: () => void;
  /** Push restored settings to the app (re-applies theme/accent/language live). */
  onSettingsSaved: (next: Settings) => void;
}) {
  const t = useT();
  const [busy, setBusy] = useState<false | "export" | "restore">(false);
  const [error, setError] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const [restore, setRestore] = useState<{ path: string; manifest: BackupManifest } | null>(null);
  const [selection, setSelection] = useState<RestoreSelection>({
    settings: true,
    collections: true,
    profiles: true,
  });

  const exportBackup = async () => {
    setError(null);
    setMessage(null);
    let dest: string | null = null;
    try {
      dest = await save({ defaultPath: "freally-capture.fcapbackup", filters: [BACKUP_FILTER] });
    } catch (err) {
      setError(String(err));
      return;
    }
    if (typeof dest !== "string") return;
    setBusy("export");
    backupExport(dest)
      .then((report) =>
        setMessage(
          t("backup-exported", {
            collections: report.collections,
            profiles: report.profiles,
            size: formatBytes(report.totalBytes),
          }),
        ),
      )
      .catch((err) => setError(String(err)))
      .finally(() => setBusy(false));
  };

  const pickRestore = async () => {
    setError(null);
    setMessage(null);
    let path: string | string[] | null = null;
    try {
      path = await open({ multiple: false, directory: false, filters: [BACKUP_FILTER] });
    } catch (err) {
      setError(String(err));
      return;
    }
    if (typeof path !== "string") return;
    const file = path;
    backupInspect(file)
      .then((manifest) => setRestore({ path: file, manifest }))
      .catch((err) => setError(String(err)));
  };

  const applyRestore = () => {
    if (!restore) return;
    setError(null);
    setBusy("restore");
    backupRestore(restore.path, selection)
      .then((report) => {
        setMessage(
          report.restartRecommended
            ? `${t("backup-restored")} ${t("backup-restart-note")}`
            : t("backup-restored"),
        );
        setRestore(null);
        onRestored();
        // The backend applied the restored settings (keeping this machine's
        // secrets) but `set` doesn't emit — re-fetch so the running app reflects
        // the restored theme/accent/language immediately, mirroring app startup.
        if (report.settings) {
          void settingsGet().then((next) => {
            onSettingsSaved(next);
            try {
              initLocale(next.language);
              applyTheme(next.theme);
            } catch (err) {
              console.error("could not apply restored language or theme:", err);
            }
          });
        }
      })
      .catch((err) => setError(String(err)))
      .finally(() => setBusy(false));
  };

  return (
    <PickerShell title={t("backup-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-havoc-muted">{t("backup-intro")}</p>
        <p className="m-0 text-[10px] leading-snug text-amber-300/90">{t("backup-secrets-note")}</p>

        {!restore && (
          <div className="flex flex-wrap items-center gap-2">
            <button
              type="button"
              disabled={busy !== false}
              onClick={() => void exportBackup()}
              className="shrink-0 rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2.5 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
            >
              {busy === "export" ? t("backup-exporting") : t("backup-export")}
            </button>
            <button
              type="button"
              disabled={busy !== false}
              onClick={() => void pickRestore()}
              className="shrink-0 rounded-md border border-white/10 px-2.5 py-1.5 text-xs text-havoc-text enabled:hover:border-havoc-accent/50 disabled:opacity-50"
            >
              {t("backup-restore")}
            </button>
          </div>
        )}

        {restore && (
          <div className="flex flex-col gap-2 rounded-md border border-white/10 bg-white/[0.02] p-3">
            <p className="m-0 text-[11px] text-havoc-muted">
              {t("backup-restore-from", {
                created: restore.manifest.created,
                version: restore.manifest.appVersion,
              })}
            </p>
            <label className="flex items-center gap-2">
              <input
                type="checkbox"
                checked={selection.settings}
                disabled={!restore.manifest.hasSettings}
                onChange={(event) =>
                  setSelection((s) => ({ ...s, settings: event.target.checked }))
                }
              />
              {t("backup-restore-settings")}
            </label>
            <label className="flex items-center gap-2">
              <input
                type="checkbox"
                checked={selection.collections}
                disabled={restore.manifest.collections.length === 0}
                onChange={(event) =>
                  setSelection((s) => ({ ...s, collections: event.target.checked }))
                }
              />
              {t("backup-restore-collections", { count: restore.manifest.collections.length })}
            </label>
            <label className="flex items-center gap-2">
              <input
                type="checkbox"
                checked={selection.profiles}
                disabled={restore.manifest.profiles.length === 0}
                onChange={(event) =>
                  setSelection((s) => ({ ...s, profiles: event.target.checked }))
                }
              />
              {t("backup-restore-profiles", { count: restore.manifest.profiles.length })}
            </label>
            <div className="flex items-center gap-2 pt-1">
              <button
                type="button"
                disabled={
                  busy !== false ||
                  (!selection.settings && !selection.collections && !selection.profiles)
                }
                onClick={applyRestore}
                className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
              >
                {busy === "restore" ? t("backup-restoring") : t("backup-restore-apply")}
              </button>
              <button
                type="button"
                onClick={() => setRestore(null)}
                className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
              >
                {t("backup-cancel")}
              </button>
            </div>
          </div>
        )}

        {message && <p className="m-0 text-[11px] text-havoc-text">{message}</p>}
        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}
      </div>
    </PickerShell>
  );
}
