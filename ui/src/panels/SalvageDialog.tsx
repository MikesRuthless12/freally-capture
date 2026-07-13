import { useState } from "react";

import { salvageDismiss, salvageRepair } from "../api/commands";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

type RepairState =
  | { state: "idle" }
  | { state: "repairing" }
  | { state: "done"; path: string }
  | {
      state: "failed";
      error: string;
    };

/**
 * The next-launch salvage prompt (CAP-M11): the previous session ended
 * uncleanly with these recordings still being written. Repair runs a
 * tolerant stream-copy remux into a `(repaired)` sibling — the original is
 * never touched; dismissing keeps the files as they are.
 */
export function SalvageDialog({ paths, onClose }: { paths: string[]; onClose: () => void }) {
  const t = useT();
  const [repairs, setRepairs] = useState<Record<string, RepairState>>({});

  const setRepair = (path: string, state: RepairState) => {
    setRepairs((current) => ({ ...current, [path]: state }));
  };

  const repair = (path: string) => {
    setRepair(path, { state: "repairing" });
    salvageRepair(path)
      .then((repaired) => setRepair(path, { state: "done", path: repaired }))
      .catch((err) => setRepair(path, { state: "failed", error: String(err) }));
  };

  const dismiss = () => {
    salvageDismiss().catch(() => undefined);
    onClose();
  };

  const basename = (path: string) => path.split(/[\\/]/).pop() ?? path;

  return (
    <PickerShell title={t("salvage-title")} onClose={dismiss} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-havoc-muted">{t("salvage-body")}</p>
        <ul className="m-0 flex list-none flex-col gap-2 p-0">
          {paths.map((path) => {
            const status = repairs[path] ?? { state: "idle" };
            return (
              <li
                key={path}
                className="flex items-center justify-between gap-2 rounded-md border border-white/10 px-2.5 py-2"
              >
                <div className="min-w-0">
                  <span className="block truncate">{basename(path)}</span>
                  <span className="block truncate text-[10px] text-havoc-muted" title={path}>
                    {path}
                  </span>
                  {status.state === "done" && (
                    <span className="block truncate text-[10px] text-emerald-300">
                      {t("salvage-repaired", { name: basename(status.path) })}
                    </span>
                  )}
                  {status.state === "failed" && (
                    <span className="block truncate text-[10px] text-red-300" title={status.error}>
                      {t("salvage-failed", { error: status.error })}
                    </span>
                  )}
                </div>
                <button
                  type="button"
                  onClick={() => repair(path)}
                  disabled={status.state === "repairing" || status.state === "done"}
                  className="shrink-0 rounded-md border border-white/10 px-2.5 py-1 text-[11px] transition-colors hover:border-havoc-accent/60 disabled:cursor-not-allowed disabled:opacity-50"
                >
                  {status.state === "repairing"
                    ? t("salvage-repairing")
                    : status.state === "done"
                      ? t("salvage-done")
                      : t("salvage-repair")}
                </button>
              </li>
            );
          })}
        </ul>
        <div className="flex justify-end">
          <button
            type="button"
            onClick={dismiss}
            className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-text transition-colors hover:border-havoc-accent/60"
          >
            {t("salvage-dismiss")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}
