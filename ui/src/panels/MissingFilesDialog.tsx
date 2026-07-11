import { useEffect, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

import {
  collectionMissingFiles,
  collectionRelink,
  collectionRelinkFolder,
  type MissingFile,
} from "../api/commands";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

/** The trailing file name of a path, for a compact primary label. */
function baseName(path: string): string {
  const parts = path.split(/[\\/]/);
  return parts[parts.length - 1] || path;
}

/**
 * The missing-file doctor (CAP-M03): every referenced media/image/font/LUT/mask
 * that no longer resolves on disk, each relinkable in place. Relinking is by
 * path — fix one broken file and every scene that used it is repaired at once.
 */
export function MissingFilesDialog({ onClose }: { onClose: () => void }) {
  const t = useT();
  const [missing, setMissing] = useState<MissingFile[] | null>(null);
  const [busy, setBusy] = useState(false);
  const [note, setNote] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const scan = () => {
    collectionMissingFiles()
      .then(setMissing)
      .catch((err) => {
        setError(String(err));
        setMissing([]);
      });
  };
  useEffect(scan, []);

  const relinkOne = (file: MissingFile) => {
    setError(null);
    setNote(null);
    open({ multiple: false, directory: false })
      .then((picked) => {
        if (typeof picked !== "string") return;
        setBusy(true);
        return collectionRelink(file.path, picked)
          .then((count) => setNote(t("doctor-relinked", { count })))
          .then(scan);
      })
      .catch((err) => setError(String(err)))
      .finally(() => setBusy(false));
  };

  const relinkFolder = () => {
    setError(null);
    setNote(null);
    open({ multiple: false, directory: true })
      .then((folder) => {
        if (typeof folder !== "string") return;
        setBusy(true);
        return collectionRelinkFolder(folder)
          .then((count) => setNote(t("doctor-relinked", { count })))
          .then(scan);
      })
      .catch((err) => setError(String(err)))
      .finally(() => setBusy(false));
  };

  const count = missing?.length ?? 0;

  return (
    <PickerShell title={t("doctor-title")} onClose={onClose} wide>
      {missing === null ? (
        <p className="m-0 text-xs text-havoc-muted">{t("doctor-scanning")}</p>
      ) : count === 0 ? (
        <p className="m-0 text-xs text-havoc-text">{t("doctor-all-good")}</p>
      ) : (
        <div className="flex flex-col gap-2">
          <p className="m-0 text-[11px] text-havoc-muted">{t("doctor-intro", { count })}</p>
          <ul className="m-0 flex max-h-72 list-none flex-col gap-1 overflow-auto p-0">
            {missing.map((file) => (
              <li
                key={file.path}
                className="flex items-center gap-2 rounded-md border border-white/10 bg-white/[0.02] px-2 py-1.5"
              >
                <div className="min-w-0 flex-1">
                  <p className="m-0 truncate text-xs text-havoc-text" title={file.path}>
                    {baseName(file.path)}
                    <span className="ml-1.5 text-[10px] text-havoc-muted">
                      {t(`doctor-kind-${file.kind}`)}
                      {file.uses > 1 ? ` · ${t("doctor-uses", { count: file.uses })}` : ""}
                    </span>
                  </p>
                  <p className="m-0 truncate text-[10px] text-havoc-muted" title={file.path}>
                    {file.path}
                  </p>
                </div>
                <button
                  type="button"
                  disabled={busy}
                  onClick={() => relinkOne(file)}
                  className="shrink-0 rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2.5 py-1 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
                >
                  {t("doctor-locate")}
                </button>
              </li>
            ))}
          </ul>
          <div className="flex items-center gap-2 border-t border-white/10 pt-2">
            <button
              type="button"
              disabled={busy}
              onClick={relinkFolder}
              className="shrink-0 rounded-md border border-white/10 px-2.5 py-1.5 text-xs text-havoc-text hover:border-havoc-accent/50 disabled:opacity-50"
            >
              {t("doctor-locate-folder")}
            </button>
            <p className="m-0 text-[10px] leading-snug text-havoc-muted">
              {t("doctor-locate-folder-hint")}
            </p>
          </div>
        </div>
      )}

      {note && <p className="mt-2 mb-0 text-[11px] text-havoc-accent">{note}</p>}
      {error && (
        <p role="alert" className="mt-2 mb-0 text-[11px] text-red-300">
          {error}
        </p>
      )}
    </PickerShell>
  );
}
