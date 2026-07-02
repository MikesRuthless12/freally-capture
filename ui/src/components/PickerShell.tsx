import { useEffect } from "react";

/** A centered modal shell shared by the add-source pickers and dialogs. */
export function PickerShell({
  title,
  onClose,
  children,
  wide = false,
}: {
  title: string;
  onClose: () => void;
  children: React.ReactNode;
  wide?: boolean;
}) {
  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape") onClose();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [onClose]);

  return (
    <div className="fixed inset-0 z-30 flex items-center justify-center bg-black/60 p-6">
      <div
        role="dialog"
        aria-label={title}
        className={`flex max-h-[80vh] ${wide ? "w-[34rem]" : "w-[26rem]"} max-w-full flex-col rounded-xl border border-white/10 bg-havoc-panel shadow-2xl`}
      >
        <header className="flex items-center justify-between border-b border-white/5 px-4 py-2.5">
          <h3 className="m-0 text-xs font-semibold tracking-wider text-havoc-muted uppercase">
            {title}
          </h3>
          <button
            type="button"
            onClick={onClose}
            aria-label="Close"
            className="rounded px-1.5 text-sm text-havoc-muted hover:text-havoc-text"
          >
            ×
          </button>
        </header>
        <div className="min-h-0 flex-1 overflow-auto p-3">{children}</div>
      </div>
    </div>
  );
}
