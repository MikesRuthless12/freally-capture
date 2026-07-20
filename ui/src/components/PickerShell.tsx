import { useEffect, useRef } from "react";
import { createPortal } from "react-dom";

import { useFocusTrap } from "../lib/useFocusTrap";
import { pushModal } from "../lib/modal";
import { useT } from "../i18n/t";

/**
 * A centered modal shell shared by the add-source pickers and dialogs.
 *
 * Rendered through a portal into `document.body`. Every caller lives inside a
 * `Panel`, whose `backdrop-blur` makes it the containing block for `position:
 * fixed` descendants — so without the portal this overlay centres itself inside
 * that dock's box (the Controls dock is ~312×176) and the dialog's buttons fall
 * off the bottom of the window, unreachable. Any ancestor `transform`, `filter`
 * or `contain` would do the same, so the portal is the durable fix, not deleting
 * one blur.
 *
 * Focus is trapped inside while open and restored on close (TASK-901).
 * `aria-modal` tells assistive tech that everything behind the dialog is inert;
 * without a trap, Tab walks the user straight into controls their screen reader
 * has been told to ignore. The two belong together or not at all.
 */
export function PickerShell({
  title,
  onClose,
  onRefresh,
  children,
  wide = false,
  large = false,
  sidebar = false,
  companion = false,
}: {
  title: string;
  onClose: () => void;
  /** When set, a refresh button appears to the left of the close button. */
  onRefresh?: () => void;
  children: React.ReactNode;
  wide?: boolean;
  /** A big editing shell (e.g. the teleprompter): tall + wide with a
   * height-filling body, so long content has room to breathe. Overrides `wide`. */
  large?: boolean;
  /** OBS-style two-pane layout (Settings): a fixed-height shell whose body
   * fills edge-to-edge with no padding or scroll of its own — the children
   * own the sidebar/pane split and each pane's scrolling. Overrides `wide`. */
  sidebar?: boolean;
  /** A companion editing surface (the teleprompter) that sits alongside live
   * production rather than blocking it: it opts OUT of the background blur so
   * the operator's read of what's behind stays crisp. It stays a real modal in
   * every other respect (scrim, focus trap, and `pushModal` — which hides the
   * native GPU preview overlay that would otherwise paint over this centered
   * dialog). "Float over the live program" is the projector/dock's job — those
   * are separate OS windows, immune to this blur entirely. */
  companion?: boolean;
}) {
  const t = useT();
  const dialogRef = useRef<HTMLDivElement>(null);
  useFocusTrap(true, dialogRef);

  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      // A layered modal above this shell (e.g. the Central panel's changelog
      // viewer) claims Escape and marks it consumed — only the topmost closes.
      if (event.key === "Escape" && !event.defaultPrevented) onClose();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [onClose]);

  // Hide the native GPU preview overlay while open — it's a native child
  // window layered over the webview and would otherwise paint over this
  // centered dialog (dark backdrop shows, dialog invisible behind it).
  useEffect(() => pushModal(), []);

  return createPortal(
    <div
      className={`fixed inset-0 z-30 flex items-center justify-center bg-black/60 p-6 ${
        companion ? "" : "modal-scrim-blur"
      }`}
    >
      <div
        ref={dialogRef}
        role="dialog"
        aria-modal="true"
        aria-label={title}
        className={`flex ${
          sidebar
            ? "h-[40rem] max-h-[85vh] w-[56rem]"
            : large
              ? "h-[46rem] max-h-[90vh] w-[64rem]"
              : `max-h-[80vh] ${wide ? "w-[34rem]" : "w-[26rem]"}`
        } max-w-full flex-col rounded-xl border border-white/10 bg-havoc-panel shadow-2xl`}
      >
        <header className="flex items-center justify-between border-b border-white/5 px-4 py-2.5">
          <h3 className="m-0 text-xs font-semibold tracking-wider text-havoc-muted uppercase">
            {title}
          </h3>
          <div className="flex items-center gap-1">
            {onRefresh && (
              <button
                type="button"
                onClick={onRefresh}
                aria-label={t("pickershell-refresh-aria")}
                title={t("pickershell-refresh-title")}
                className="rounded px-1.5 text-sm text-havoc-muted hover:text-havoc-text"
              >
                ↻
              </button>
            )}
            <button
              type="button"
              onClick={onClose}
              aria-label={t("pickershell-close")}
              className="rounded px-1.5 text-sm text-havoc-muted hover:text-havoc-text"
            >
              ×
            </button>
          </div>
        </header>
        <div className={sidebar ? "flex min-h-0 flex-1" : "min-h-0 flex-1 overflow-auto p-3"}>
          {children}
        </div>
      </div>
    </div>,
    document.body,
  );
}
