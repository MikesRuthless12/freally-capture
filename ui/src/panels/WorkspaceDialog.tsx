import { useEffect, useState } from "react";
import { open, save } from "@tauri-apps/plugin-dialog";

import {
  collectionCreate,
  collectionImportObs,
  collectionSwitch,
  collectionsList,
  dockPresetApply,
  dockPresetDelete,
  dockPresetSave,
  dockPresetsList,
  packExport,
  packImport,
  profileCreate,
  profileSwitch,
  profilesList,
  type DockPreset,
  type ImportReport,
  type NamedList,
} from "../api/commands";
import type { Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { CompareMergeDialog } from "./CompareMergeDialog";
import { SnapshotsDialog } from "./SnapshotsDialog";
import { BackupDialog } from "./BackupDialog";
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

const PACK_FILTER = { name: "Freally Capture pack", extensions: ["fcappack"] };

/** Native "save as" dialog for a `.fcappack`, seeded with the collection name. */
async function pickPackSave(defaultName: string): Promise<string | null> {
  try {
    const picked = await save({
      defaultPath: `${defaultName}.fcappack`,
      filters: [PACK_FILTER],
    });
    return typeof picked === "string" ? picked : null;
  } catch (err) {
    console.error("save dialog failed:", err);
    return null;
  }
}

/** Native "open" dialog scoped to `.fcappack` files. */
async function pickPackFile(): Promise<string | null> {
  try {
    const picked = await open({
      multiple: false,
      directory: false,
      filters: [PACK_FILTER],
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
  const [packBusy, setPackBusy] = useState<false | "export" | "import">(false);
  const [packMsg, setPackMsg] = useState<string | null>(null);
  const [showCompare, setShowCompare] = useState(false);
  const [showSnapshots, setShowSnapshots] = useState(false);
  const [showBackup, setShowBackup] = useState(false);
  const [dockPresets, setDockPresets] = useState<DockPreset[]>([]);
  const [presetDraft, setPresetDraft] = useState("");

  const refresh = () => {
    profilesList()
      .then(setProfiles)
      .catch(() => {});
    collectionsList()
      .then(setCollections)
      .catch(() => {});
    dockPresetsList()
      .then(setDockPresets)
      .catch(() => {});
  };
  useEffect(refresh, []);

  const run = (work: Promise<unknown>) => {
    setError(null);
    work.then(refresh).catch((err) => setError(String(err)));
  };

  const applyPreset = (name: string) => {
    setError(null);
    dockPresetApply(name)
      .then((settings) => onSettingsSaved(settings))
      .catch((err) => setError(String(err)));
  };
  const savePreset = () => {
    if (!presetDraft.trim()) return;
    setError(null);
    dockPresetSave(presetDraft.trim())
      .then((presets) => {
        setDockPresets(presets);
        setPresetDraft("");
      })
      .catch((err) => setError(String(err)));
  };
  const deletePreset = (name: string) =>
    dockPresetDelete(name)
      .then(setDockPresets)
      .catch((err) => setError(String(err)));

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

  const exportPack = () => {
    setError(null);
    setPackMsg(null);
    pickPackSave(collections?.active ?? "collection").then((dest) => {
      if (!dest) return;
      setPackBusy("export");
      packExport(dest)
        .then((r) =>
          setPackMsg(
            t("workspace-pack-exported", {
              name: r.collectionName,
              bundled: r.bundled,
              external: r.external,
            }),
          ),
        )
        .catch((err) => setError(String(err)))
        .finally(() => setPackBusy(false));
    });
  };

  const importPack = () => {
    setError(null);
    setPackMsg(null);
    pickPackFile().then((path) => {
      if (!path) return;
      setPackBusy("import");
      packImport(path)
        .then((r) => {
          setPackMsg(
            t("workspace-pack-imported", {
              name: r.collectionName,
              scenes: r.sceneCount,
              relinked: r.relinked,
              external: r.external,
            }),
          );
          refresh();
        })
        .catch((err) => setError(String(err)))
        .finally(() => setPackBusy(false));
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

      <div className="mt-3 flex flex-col gap-2 border-t border-white/10 pt-3">
        <p className="m-0 text-[10px] font-semibold tracking-wide text-havoc-muted uppercase">
          {t("workspace-pack-title")}
        </p>
        <div className="flex flex-wrap items-center gap-2">
          <button
            type="button"
            disabled={packBusy !== false || !collections}
            onClick={exportPack}
            className="shrink-0 rounded-md border border-white/10 px-2.5 py-1.5 text-xs text-havoc-text hover:border-havoc-accent/50 disabled:opacity-50"
          >
            {packBusy === "export" ? t("workspace-pack-exporting") : t("workspace-pack-export")}
          </button>
          <button
            type="button"
            disabled={packBusy !== false}
            onClick={importPack}
            className="shrink-0 rounded-md border border-white/10 px-2.5 py-1.5 text-xs text-havoc-text hover:border-havoc-accent/50 disabled:opacity-50"
          >
            {packBusy === "import" ? t("workspace-pack-importing") : t("workspace-pack-import")}
          </button>
          <button
            type="button"
            disabled={importing}
            onClick={importObs}
            title={t("workspace-import-obs-hint")}
            className="shrink-0 rounded-md border border-white/10 px-2.5 py-1.5 text-xs text-havoc-text hover:border-havoc-accent/50 disabled:opacity-50"
          >
            {importing ? t("workspace-import-busy") : t("workspace-import-obs")}
          </button>
          <button
            type="button"
            onClick={() => setShowCompare(true)}
            className="shrink-0 rounded-md border border-white/10 px-2.5 py-1.5 text-xs text-havoc-text hover:border-havoc-accent/50"
          >
            {t("workspace-compare")}
          </button>
          <button
            type="button"
            onClick={() => setShowSnapshots(true)}
            className="shrink-0 rounded-md border border-white/10 px-2.5 py-1.5 text-xs text-havoc-text hover:border-havoc-accent/50"
          >
            {t("workspace-snapshots")}
          </button>
          <button
            type="button"
            onClick={() => setShowBackup(true)}
            className="shrink-0 rounded-md border border-white/10 px-2.5 py-1.5 text-xs text-havoc-text hover:border-havoc-accent/50"
          >
            {t("workspace-backup")}
          </button>
        </div>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("workspace-pack-hint")}</p>
        {packMsg && <p className="m-0 text-[11px] text-havoc-text">{packMsg}</p>}
      </div>

      <div className="mt-3 flex flex-col gap-2 border-t border-white/10 pt-3">
        <p className="m-0 text-[10px] font-semibold tracking-wide text-havoc-muted uppercase">
          {t("workspace-dock-presets")}
        </p>
        {dockPresets.length > 0 && (
          <ul className="m-0 flex list-none flex-wrap gap-1.5 p-0">
            {dockPresets.map((preset) => (
              <li
                key={preset.name}
                className="flex items-center gap-1 rounded-md border border-white/10 bg-white/[0.02] pl-2"
              >
                <button
                  type="button"
                  onClick={() => applyPreset(preset.name)}
                  title={t("workspace-dock-preset-apply", { name: preset.name })}
                  className="py-1 text-xs text-havoc-text hover:text-havoc-accent"
                >
                  {preset.name}
                </button>
                <button
                  type="button"
                  onClick={() => deletePreset(preset.name)}
                  aria-label={t("workspace-dock-preset-delete", { name: preset.name })}
                  className="rounded px-1 text-xs text-havoc-muted hover:text-red-400"
                >
                  ×
                </button>
              </li>
            ))}
          </ul>
        )}
        <div className="flex gap-2">
          <input
            value={presetDraft}
            onChange={(event) => setPresetDraft(event.target.value)}
            onKeyDown={(event) => event.key === "Enter" && savePreset()}
            placeholder={t("workspace-dock-preset-placeholder")}
            aria-label={t("workspace-dock-preset-placeholder")}
            className={inputClass}
          />
          <button
            type="button"
            disabled={!presetDraft.trim()}
            onClick={savePreset}
            className="shrink-0 rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2.5 py-1 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
          >
            {t("workspace-dock-preset-save")}
          </button>
        </div>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("workspace-dock-presets-hint")}
        </p>
      </div>

      {report && <ImportReportCard report={report} onDismiss={() => setReport(null)} />}

      {error && (
        <p role="alert" className="mt-2 mb-0 text-[11px] text-red-300">
          {error}
        </p>
      )}

      {showCompare && (
        <CompareMergeDialog
          collections={collections}
          onClose={() => setShowCompare(false)}
          onMerged={refresh}
        />
      )}

      {showSnapshots && (
        <SnapshotsDialog
          collections={collections}
          onClose={() => setShowSnapshots(false)}
          onChanged={refresh}
        />
      )}

      {showBackup && (
        <BackupDialog
          onClose={() => setShowBackup(false)}
          onRestored={refresh}
          onSettingsSaved={onSettingsSaved}
        />
      )}
    </PickerShell>
  );
}
