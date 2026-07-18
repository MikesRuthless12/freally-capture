import { useEffect, useState } from "react";

import {
  snapshotCreate,
  snapshotDelete,
  snapshotList,
  snapshotRestore,
  type NamedList,
  type SnapshotList,
} from "../api/commands";
import { PickerShell } from "../components/PickerShell";
import { EmptyHint } from "../components/Panel";
import { CompareMergeDialog } from "./CompareMergeDialog";
import { formatBytes } from "../lib/format";
import { useT } from "../i18n/t";

/**
 * CAP-N62: named version snapshots of the active collection — a browsable
 * timeline of intentional checkpoints with restore and a diff view (reusing the
 * CAP-N61 compare dialog against a snapshot target). Long-lived insurance that
 * sits beside autosave + undo; strictly local.
 */
export function SnapshotsDialog({
  collections,
  onClose,
  onChanged,
}: {
  collections: NamedList | null;
  onClose: () => void;
  /** Called after a restore so the parent (and canvas) refresh. */
  onChanged: () => void;
}) {
  const t = useT();
  const [list, setList] = useState<SnapshotList | null>(null);
  const [draft, setDraft] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [compare, setCompare] = useState<{ id: string; name: string } | null>(null);

  const refresh = () => {
    snapshotList()
      .then(setList)
      .catch((err) => setError(String(err)));
  };
  useEffect(refresh, []);

  const run = (work: Promise<SnapshotList>, after?: () => void) => {
    setError(null);
    setBusy(true);
    work
      .then((next) => {
        setList(next);
        after?.();
      })
      .catch((err) => setError(String(err)))
      .finally(() => setBusy(false));
  };

  const create = () => {
    if (!draft.trim()) return;
    run(snapshotCreate(draft.trim()), () => setDraft(""));
  };

  const restore = (id: string) => {
    if (!window.confirm(t("snapshots-restore-confirm"))) return;
    run(snapshotRestore(id), onChanged);
  };

  const snapshots = list?.snapshots ?? [];

  return (
    <PickerShell title={t("snapshots-title")} onClose={onClose} onRefresh={refresh} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-havoc-muted">{t("snapshots-intro")}</p>

        <div className="flex gap-2">
          <input
            value={draft}
            onChange={(event) => setDraft(event.target.value)}
            onKeyDown={(event) => event.key === "Enter" && create()}
            placeholder={t("snapshots-name-placeholder")}
            maxLength={60}
            className="min-w-0 flex-1 rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text outline-none focus:border-havoc-accent/60"
          />
          <button
            type="button"
            disabled={busy || !draft.trim()}
            onClick={create}
            className="shrink-0 rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2.5 py-1 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
          >
            {t("snapshots-create")}
          </button>
        </div>

        {list && (
          <p className="m-0 text-[10px] text-havoc-muted">
            {t("snapshots-usage", {
              count: snapshots.length,
              cap: list.cap,
              size: formatBytes(list.totalBytes),
            })}
          </p>
        )}

        {snapshots.length === 0 ? (
          <EmptyHint>{t("snapshots-empty")}</EmptyHint>
        ) : (
          <ul className="m-0 flex max-h-[24rem] list-none flex-col gap-1.5 overflow-y-auto p-0">
            {snapshots.map((snapshot) => (
              <li
                key={snapshot.id}
                className="flex items-center gap-2 rounded-md border border-white/10 bg-white/[0.02] px-2 py-1.5"
              >
                <div className="min-w-0 flex-1">
                  <p className="m-0 truncate text-xs text-havoc-text">{snapshot.name}</p>
                  <p className="m-0 text-[10px] text-havoc-muted">
                    {snapshot.created} ·{" "}
                    {t("snapshots-scenes", {
                      scenes: snapshot.scenes,
                      sources: snapshot.sources,
                    })}{" "}
                    · {formatBytes(snapshot.bytes)}
                  </p>
                </div>
                <button
                  type="button"
                  disabled={busy}
                  onClick={() => setCompare({ id: snapshot.id, name: snapshot.name })}
                  className="shrink-0 rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-50"
                >
                  {t("snapshots-compare")}
                </button>
                <button
                  type="button"
                  disabled={busy}
                  onClick={() => restore(snapshot.id)}
                  className="shrink-0 rounded-md border border-havoc-accent/50 px-2 py-1 text-[11px] text-havoc-text enabled:hover:bg-havoc-accent/15 disabled:opacity-50"
                >
                  {t("snapshots-restore")}
                </button>
                <button
                  type="button"
                  disabled={busy}
                  onClick={() => run(snapshotDelete(snapshot.id))}
                  aria-label={t("snapshots-delete-aria", { name: snapshot.name })}
                  title={t("snapshots-delete")}
                  className="shrink-0 rounded px-1 text-xs text-havoc-muted enabled:hover:text-red-400 disabled:opacity-50"
                >
                  ×
                </button>
              </li>
            ))}
          </ul>
        )}

        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}
      </div>

      {compare && (
        <CompareMergeDialog
          collections={collections}
          initialTarget={{ kind: "snapshot", id: compare.id }}
          initialLabel={compare.name}
          onClose={() => setCompare(null)}
          onMerged={() => {
            refresh();
            onChanged();
          }}
        />
      )}
    </PickerShell>
  );
}
