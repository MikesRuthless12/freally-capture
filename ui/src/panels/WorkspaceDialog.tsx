import { useEffect, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

import {
  collectionCreate,
  collectionImportObs,
  collectionSwitch,
  collectionsList,
  profileCreate,
  profileSwitch,
  profilesList,
  type ImportReport,
  type NamedList,
} from "../api/commands";
import type { Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const inputClass =
  "min-w-0 flex-1 rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/** One switchable-names column (profiles / scene collections). */
function NamedColumn({
  title,
  hint,
  list,
  onSwitch,
  onCreate,
}: {
  title: string;
  hint: string;
  list: NamedList | null;
  onSwitch: (name: string) => void;
  onCreate: (name: string) => void;
}) {
  const t = useT();
  const [draft, setDraft] = useState("");
  return (
    <div className="flex min-w-0 flex-1 flex-col gap-2">
      <p className="m-0 text-[11px] font-semibold tracking-wide text-havoc-muted uppercase">
        {title}
      </p>
      <ul className="m-0 flex max-h-48 list-none flex-col gap-1 overflow-auto p-0">
        {(list?.names ?? []).map((name) => {
          const active = name === list?.active;
          return (
            <li key={name}>
              <button
                type="button"
                onClick={() => !active && onSwitch(name)}
                title={active ? t("workspace-active") : t("workspace-switch-to", { name })}
                className={`w-full truncate rounded-md border px-2 py-1 text-left text-xs ${
                  active
                    ? "border-havoc-accent/50 bg-havoc-accent/10 text-havoc-text"
                    : "border-white/10 bg-white/[0.02] text-havoc-muted hover:text-havoc-text"
                }`}
              >
                {name}
                {active && (
                  <span className="ml-1.5 text-[10px] text-havoc-accent">
                    {t("workspace-active-marker")}
                  </span>
                )}
              </button>
            </li>
          );
        })}
      </ul>
      <div className="flex gap-2">
        <input
          value={draft}
          onChange={(event) => setDraft(event.target.value)}
          placeholder={t("workspace-new-name-placeholder")}
          aria-label={t("workspace-new-name-label", { title: title.toLowerCase() })}
          className={inputClass}
        />
        <button
          type="button"
          disabled={!draft.trim()}
          onClick={() => {
            onCreate(draft.trim());
            setDraft("");
          }}
          className="shrink-0 rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2.5 py-1 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          {t("workspace-create")}
        </button>
      </div>
      <p className="m-0 text-[10px] leading-snug text-havoc-muted">{hint}</p>
    </div>
  );
}

/** The honest per-source account of an OBS import (CAP-M02). */
function ImportReportCard({ report, onDismiss }: { report: ImportReport; onDismiss: () => void }) {
  const t = useT();
  const clean = report.notes.length === 0 && report.skipped.length === 0;
  return (
    <div className="mt-3 flex flex-col gap-2 rounded-md border border-white/10 bg-white/[0.02] p-3">
      <div className="flex items-start justify-between gap-2">
        <div className="min-w-0">
          <p className="m-0 truncate text-xs font-semibold text-havoc-text">
            {t("workspace-import-title", { name: report.name })}
          </p>
          <p className="m-0 text-[11px] text-havoc-muted">
            {t("workspace-import-summary", {
              scenes: report.sceneCount,
              sources: report.sourceCount,
              items: report.itemCount,
            })}
          </p>
        </div>
        <button
          type="button"
          onClick={onDismiss}
          className="shrink-0 rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:text-havoc-text"
        >
          {t("workspace-import-dismiss")}
        </button>
      </div>

      {clean ? (
        <p className="m-0 text-[11px] text-havoc-text">{t("workspace-import-clean")}</p>
      ) : (
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("workspace-import-geometry-caveat")}
        </p>
      )}

      {report.notes.length > 0 && (
        <div className="flex flex-col gap-1">
          <p className="m-0 text-[10px] font-semibold tracking-wide text-havoc-muted uppercase">
            {t("workspace-import-notes-title")}
          </p>
          <ul className="m-0 flex max-h-40 list-none flex-col gap-1 overflow-auto p-0">
            {report.notes.map((source) => (
              <li
                key={source.name}
                className="rounded border border-white/10 bg-white/[0.02] px-2 py-1 text-[11px]"
              >
                <span className="text-havoc-text">{source.name}</span>{" "}
                <span className="text-havoc-muted">({source.obsKind})</span>
                <span className="mt-0.5 flex flex-wrap gap-1">
                  {source.notes.map((note) => (
                    <span
                      key={note}
                      className="rounded bg-havoc-accent/10 px-1.5 py-0.5 text-[10px] text-havoc-text"
                    >
                      {t(`import-note-${note}`)}
                    </span>
                  ))}
                </span>
              </li>
            ))}
          </ul>
        </div>
      )}

      {report.skipped.length > 0 && (
        <div className="flex flex-col gap-1">
          <p className="m-0 text-[10px] font-semibold tracking-wide text-havoc-muted uppercase">
            {t("workspace-import-skipped-title")}
          </p>
          <ul className="m-0 flex max-h-32 list-none flex-col gap-1 overflow-auto p-0">
            {report.skipped.map((source) => (
              <li
                key={source.name}
                className="rounded border border-white/10 bg-white/[0.02] px-2 py-1 text-[11px]"
              >
                <span className="text-havoc-text">{source.name}</span>{" "}
                <span className="text-havoc-muted">({source.obsKind})</span>
                <span className="ml-1 text-havoc-muted">— {t(`import-skip-${source.reason}`)}</span>
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}

/** Native file dialog scoped to OBS scene-collection `.json` files. */
async function pickObsFile(): Promise<string | null> {
  try {
    const picked = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "OBS scene collection", extensions: ["json"] }],
    });
    return typeof picked === "string" ? picked : null;
  } catch (err) {
    console.error("file dialog failed:", err);
    return null;
  }
}

/**
 * Profiles + scene collections (TASK-506). Switching saves the outgoing one
 * first — nothing is ever lost. A profile carries the settings; a collection
 * carries the scenes. Collections can also be imported from OBS (CAP-M02).
 */
export function WorkspaceDialog({
  onClose,
  onSettingsSaved,
}: {
  onClose: () => void;
  /** A profile switch replaces the live settings — the app re-renders them. */
  onSettingsSaved: (next: Settings) => void;
}) {
  const t = useT();
  const [profiles, setProfiles] = useState<NamedList | null>(null);
  const [collections, setCollections] = useState<NamedList | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [report, setReport] = useState<ImportReport | null>(null);
  const [importing, setImporting] = useState(false);

  const refresh = () => {
    profilesList()
      .then(setProfiles)
      .catch(() => {});
    collectionsList()
      .then(setCollections)
      .catch(() => {});
  };
  useEffect(refresh, []);

  const run = (work: Promise<unknown>) => {
    setError(null);
    work.then(refresh).catch((err) => setError(String(err)));
  };

  const importObs = () => {
    setError(null);
    pickObsFile().then((path) => {
      if (!path) return;
      setImporting(true);
      collectionImportObs(path)
        .then((result) => {
          setReport(result);
          refresh();
        })
        .catch((err) => setError(String(err)))
        .finally(() => setImporting(false));
    });
  };

  return (
    <PickerShell title={t("workspace-title")} onClose={onClose} wide>
      <div className="flex gap-4">
        <NamedColumn
          title={t("workspace-profiles")}
          hint={t("workspace-profiles-hint")}
          list={profiles}
          onSwitch={(name) =>
            run(profileSwitch(name).then((settings) => onSettingsSaved(settings)))
          }
          onCreate={(name) => run(profileCreate(name))}
        />
        <NamedColumn
          title={t("workspace-collections")}
          hint={t("workspace-collections-hint")}
          list={collections}
          onSwitch={(name) => run(collectionSwitch(name))}
          onCreate={(name) => run(collectionCreate(name))}
        />
      </div>

      <div className="mt-3 flex items-center gap-2 border-t border-white/10 pt-3">
        <button
          type="button"
          disabled={importing}
          onClick={importObs}
          className="shrink-0 rounded-md border border-white/10 px-2.5 py-1.5 text-xs text-havoc-text hover:border-havoc-accent/50 disabled:opacity-50"
        >
          {importing ? t("workspace-import-busy") : t("workspace-import-obs")}
        </button>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("workspace-import-obs-hint")}
        </p>
      </div>

      {report && <ImportReportCard report={report} onDismiss={() => setReport(null)} />}

      {error && (
        <p role="alert" className="mt-2 mb-0 text-[11px] text-red-300">
          {error}
        </p>
      )}
    </PickerShell>
  );
}
