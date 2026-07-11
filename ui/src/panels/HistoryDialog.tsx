import { studioRedo, studioUndo } from "../api/commands";
import type { HistoryState, StudioDto } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const EMPTY: HistoryState = {
  canUndo: false,
  canRedo: false,
  undoLabel: null,
  redoLabel: null,
  undo: [],
  redo: [],
};

const buttonClass =
  "rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-text enabled:hover:border-havoc-accent/60 disabled:opacity-40";

/**
 * The viewable edit history (CAP-M01): the undo stack (next undo at the top of
 * the lower list), a current-state marker, and the redo stack above it, plus
 * Undo/Redo buttons. Entry labels are stable keys the backend supplies — each
 * renders through `history-<label>`, mirroring the Rust action tag 1:1.
 */
export function HistoryDialog({
  studio,
  onClose,
}: {
  studio: StudioDto | null;
  onClose: () => void;
}) {
  const t = useT();
  const history = studio?.history ?? EMPTY;
  const label = (key: string) => t(`history-${key}`);

  // Redo: next-to-redo first (newest undone on top). Undo: next-to-undo first.
  const redo = [...history.redo].reverse();
  const undo = [...history.undo].reverse();

  return (
    <PickerShell title={t("history-title")} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <div className="flex gap-2">
          <button
            type="button"
            disabled={!history.canUndo}
            onClick={() => void studioUndo().catch(() => undefined)}
            className={buttonClass}
          >
            {t("palette-undo")}
          </button>
          <button
            type="button"
            disabled={!history.canRedo}
            onClick={() => void studioRedo().catch(() => undefined)}
            className={buttonClass}
          >
            {t("palette-redo")}
          </button>
        </div>

        {undo.length === 0 && redo.length === 0 ? (
          <p className="m-0 text-[11px] text-havoc-muted">{t("history-empty")}</p>
        ) : (
          <ol className="m-0 flex max-h-72 list-none flex-col gap-0.5 overflow-y-auto p-0">
            {redo.map((key, index) => (
              <li
                key={`redo-${redo.length - index}`}
                className="rounded px-2 py-1 text-[11px] text-havoc-muted/60"
              >
                {label(key)}
              </li>
            ))}
            <li className="px-2 py-1 text-[10px] uppercase tracking-wide text-havoc-accent">
              {t("history-current")}
            </li>
            {undo.map((key, index) => (
              <li
                key={`undo-${undo.length - index}`}
                className={
                  index === 0
                    ? "rounded px-2 py-1 text-[11px] font-medium text-havoc-accent"
                    : "rounded px-2 py-1 text-[11px] text-havoc-muted"
                }
              >
                {label(key)}
              </li>
            ))}
          </ol>
        )}

        <div className="flex justify-end">
          <button
            type="button"
            onClick={onClose}
            className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
          >
            {t("history-close")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}
