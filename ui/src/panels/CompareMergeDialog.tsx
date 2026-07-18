import { useCallback, useEffect, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

import {
  collectionDiff,
  collectionMerge,
  type ChangeKind,
  type CollectionDiff,
  type DiffTarget,
  type NamedList,
  type SceneChange,
  type SourceChange,
} from "../api/commands";
import { PickerShell } from "../components/PickerShell";
import { EmptyHint } from "../components/Panel";
import { useT } from "../i18n/t";

/** Colour a change badge by kind. */
function badgeClass(kind: ChangeKind): string {
  switch (kind) {
    case "added":
      return "bg-emerald-500/15 text-emerald-300";
    case "removed":
      return "bg-red-500/15 text-red-300";
    case "modified":
      return "bg-amber-400/15 text-amber-300";
  }
}

/**
 * CAP-N61: compare the active collection against another (a named collection or
 * a `.fcappack`) and cherry-pick which scene/source changes to merge in. All
 * local — the merge rewrites only the active collection.
 */
export function CompareMergeDialog({
  collections,
  onClose,
  onMerged,
  initialTarget,
  initialLabel,
}: {
  collections: NamedList | null;
  onClose: () => void;
  /** Called after a merge lands so the parent can refresh. */
  onMerged: () => void;
  /** Pre-select a comparison target (e.g. a snapshot) and diff it on open. */
  initialTarget?: DiffTarget;
  initialLabel?: string;
}) {
  const t = useT();
  // Seed from a preset target (e.g. a snapshot) so the initial diff can fire
  // from an effect without any synchronous setState.
  const [target, setTarget] = useState<DiffTarget | null>(initialTarget ?? null);
  const [targetLabel, setTargetLabel] = useState(initialLabel ?? "");
  const [diff, setDiff] = useState<CollectionDiff | null>(null);
  const [busy, setBusy] = useState<false | "diff" | "merge">(initialTarget ? "diff" : false);
  const [error, setError] = useState<string | null>(null);
  const [pickedScenes, setPickedScenes] = useState<Set<string>>(new Set());
  const [pickedSources, setPickedSources] = useState<Set<string>>(new Set());

  const runDiff = useCallback((next: DiffTarget, label: string) => {
    setTarget(next);
    setTargetLabel(label);
    setError(null);
    setDiff(null);
    setPickedScenes(new Set());
    setPickedSources(new Set());
    setBusy("diff");
    collectionDiff(next)
      .then(setDiff)
      .catch((err) => setError(String(err)))
      .finally(() => setBusy(false));
  }, []);

  // Fire the preset target's diff once on open. Only async setState (in the
  // promise callbacks) runs here — the seeds above cover the synchronous state.
  useEffect(() => {
    if (!initialTarget) return;
    collectionDiff(initialTarget)
      .then(setDiff)
      .catch((err) => setError(String(err)))
      .finally(() => setBusy(false));
    // Runs once for the preset; changing the target later goes through runDiff.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const pickCollection = (name: string) => {
    if (!name) return;
    runDiff({ kind: "collection", name }, name);
  };

  const pickPack = async () => {
    try {
      const picked = await open({
        multiple: false,
        directory: false,
        filters: [{ name: "Freally Capture pack", extensions: ["fcappack"] }],
      });
      if (typeof picked === "string") {
        const base = picked.split(/[\\/]/).pop() ?? picked;
        runDiff({ kind: "pack", path: picked }, base);
      }
    } catch (err) {
      setError(String(err));
    }
  };

  const toggle = (set: Set<string>, id: string): Set<string> => {
    const next = new Set(set);
    if (next.has(id)) next.delete(id);
    else next.add(id);
    return next;
  };

  const selectedCount = pickedScenes.size + pickedSources.size;

  const selectAll = () => {
    if (!diff) return;
    setPickedScenes(new Set(diff.scenes.map((s) => s.scene)));
    setPickedSources(new Set(diff.sources.map((s) => s.source)));
  };
  const clearAll = () => {
    setPickedScenes(new Set());
    setPickedSources(new Set());
  };

  const applyMerge = () => {
    if (!target || selectedCount === 0) return;
    setError(null);
    setBusy("merge");
    collectionMerge(target, [...pickedScenes], [...pickedSources])
      .then((remaining) => {
        setDiff(remaining);
        setPickedScenes(new Set());
        setPickedSources(new Set());
        onMerged();
      })
      .catch((err) => setError(String(err)))
      .finally(() => setBusy(false));
  };

  const names = (collections?.names ?? []).filter((n) => n !== collections?.active);

  return (
    <PickerShell title={t("compare-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-havoc-muted">{t("compare-intro")}</p>

        <div className="flex flex-wrap items-center gap-2">
          <label className="text-[11px] text-havoc-muted">{t("compare-target-label")}</label>
          <select
            value={target?.kind === "collection" ? target.name : ""}
            onChange={(event) => pickCollection(event.target.value)}
            className="min-w-0 rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text outline-none focus:border-havoc-accent/60"
          >
            <option value="">{t("compare-target-none")}</option>
            {names.map((name) => (
              <option key={name} value={name}>
                {name}
              </option>
            ))}
          </select>
          <button
            type="button"
            onClick={() => void pickPack()}
            className="shrink-0 rounded-md border border-white/10 px-2.5 py-1 text-xs text-havoc-text hover:border-havoc-accent/50"
          >
            {t("compare-target-pack")}
          </button>
          {targetLabel && (
            <span className="truncate text-[11px] text-havoc-muted" title={targetLabel}>
              {t("compare-against", { name: targetLabel })}
            </span>
          )}
        </div>

        {busy === "diff" ? (
          <p className="m-0 text-havoc-muted">{t("compare-comparing")}</p>
        ) : diff && diff.total === 0 ? (
          <EmptyHint>{t("compare-no-diff")}</EmptyHint>
        ) : diff ? (
          <>
            <div className="flex items-center gap-2">
              <button
                type="button"
                onClick={selectAll}
                className="rounded-md border border-white/10 px-2 py-0.5 text-[11px] text-havoc-muted hover:text-havoc-text"
              >
                {t("compare-select-all")}
              </button>
              <button
                type="button"
                onClick={clearAll}
                className="rounded-md border border-white/10 px-2 py-0.5 text-[11px] text-havoc-muted hover:text-havoc-text"
              >
                {t("compare-clear")}
              </button>
              <p className="m-0 text-[10px] text-havoc-muted">{t("compare-merge-hint")}</p>
            </div>

            <div className="flex max-h-[22rem] flex-col gap-3 overflow-y-auto pr-1">
              {diff.sources.length > 0 && (
                <section className="flex flex-col gap-1">
                  <h3 className="m-0 text-[10px] font-semibold tracking-wide text-havoc-muted uppercase">
                    {t("compare-sources-title")}
                  </h3>
                  {diff.sources.map((change) => (
                    <SourceRow
                      key={change.source}
                      change={change}
                      checked={pickedSources.has(change.source)}
                      onToggle={() => setPickedSources((s) => toggle(s, change.source))}
                    />
                  ))}
                </section>
              )}
              {diff.scenes.length > 0 && (
                <section className="flex flex-col gap-1">
                  <h3 className="m-0 text-[10px] font-semibold tracking-wide text-havoc-muted uppercase">
                    {t("compare-scenes-title")}
                  </h3>
                  {diff.scenes.map((change) => (
                    <SceneRow
                      key={change.scene}
                      change={change}
                      checked={pickedScenes.has(change.scene)}
                      onToggle={() => setPickedScenes((s) => toggle(s, change.scene))}
                    />
                  ))}
                </section>
              )}
            </div>

            <div className="flex items-center gap-2 border-t border-white/10 pt-3">
              <button
                type="button"
                disabled={busy !== false || selectedCount === 0}
                onClick={applyMerge}
                className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
              >
                {busy === "merge"
                  ? t("compare-applying")
                  : selectedCount === 0
                    ? t("compare-apply-none")
                    : t("compare-apply", { count: selectedCount })}
              </button>
            </div>
          </>
        ) : null}

        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}
      </div>
    </PickerShell>
  );
}

function KindBadge({ kind }: { kind: ChangeKind }) {
  const t = useT();
  return (
    <span
      className={`shrink-0 rounded px-1.5 py-px text-[9px] font-semibold uppercase ${badgeClass(kind)}`}
    >
      {t(`compare-kind-${kind}`)}
    </span>
  );
}

function SourceRow({
  change,
  checked,
  onToggle,
}: {
  change: SourceChange;
  checked: boolean;
  onToggle: () => void;
}) {
  const t = useT();
  return (
    <label className="flex items-center gap-2 rounded-md border border-white/10 bg-white/[0.02] px-2 py-1">
      <input type="checkbox" checked={checked} onChange={onToggle} />
      <KindBadge kind={change.kind} />
      <span className="min-w-0 flex-1 truncate">{change.name}</span>
      {change.renamedFrom && (
        <span className="shrink-0 text-[10px] text-havoc-muted">
          {t("compare-renamed", { from: change.renamedFrom })}
        </span>
      )}
      {change.aspects.length > 0 && (
        <span className="shrink-0 text-[10px] text-havoc-muted">
          {change.aspects.map((a) => t(`compare-aspect-${a}`)).join(", ")}
        </span>
      )}
    </label>
  );
}

function SceneRow({
  change,
  checked,
  onToggle,
}: {
  change: SceneChange;
  checked: boolean;
  onToggle: () => void;
}) {
  const t = useT();
  return (
    <div className="rounded-md border border-white/10 bg-white/[0.02] px-2 py-1">
      <label className="flex items-center gap-2">
        <input type="checkbox" checked={checked} onChange={onToggle} />
        <KindBadge kind={change.kind} />
        <span className="min-w-0 flex-1 truncate">{change.name}</span>
        {change.renamedFrom && (
          <span className="shrink-0 text-[10px] text-havoc-muted">
            {t("compare-renamed", { from: change.renamedFrom })}
          </span>
        )}
        {change.reordered && (
          <span className="shrink-0 rounded bg-havoc-accent/10 px-1 text-[9px] text-havoc-accent">
            {t("compare-reordered")}
          </span>
        )}
      </label>
      {change.items.length > 0 && (
        <ul className="m-0 mt-1 ml-6 flex list-none flex-col gap-0.5 p-0">
          {change.items.map((item) => (
            <li key={item.item} className="flex items-center gap-1.5 text-[10px] text-havoc-muted">
              <KindBadge kind={item.kind} />
              <span className="min-w-0 truncate">{item.sourceName}</span>
              {item.aspects.length > 0 && (
                <span className="shrink-0">
                  ({item.aspects.map((a) => t(`compare-aspect-${a}`)).join(", ")})
                </span>
              )}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
