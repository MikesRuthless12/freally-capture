import { useEffect } from "react";
import { createPortal } from "react-dom";

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
 */
export function PickerShell({
  title,
  onClose,
  onRefresh,
  children,
  wide = false,
}: {
  title: string;
  onClose: () => void;
  /** When set, a refresh button appears to the left of the close button. */
  onRefresh?: () => void;
  children: React.ReactNode;
  wide?: boolean;
}) {
  const t = useT();
  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape") onClose();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [onClose]);

  // Hide the native GPU preview overlay while open — it's a native child
  // window layered over the webview and would otherwise paint over this
  // centered dialog (dark backdrop shows, dialog invisible behind it).
  useEffect(() => pushModal(), []);

  return createPortal(
    <div className="fixed inset-0 z-30 flex items-center justify-center bg-black/60 p-6">
      <div
        role="dialog"
        aria-modal="true"
        aria-label={title}
        className={`flex max-h-[80vh] ${wide ? "w-[34rem]" : "w-[26rem]"} max-w-full flex-col rounded-xl border border-white/10 bg-havoc-panel shadow-2xl`}
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
        <div className="min-h-0 flex-1 overflow-auto p-3">{children}</div>
      </div>
    </div>,
    document.body,
  );
}
