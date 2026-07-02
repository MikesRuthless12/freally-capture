import type { ReactNode } from "react";

type PanelProps = {
  title: string;
  /** Right-aligned header extras (badges, small buttons). */
  actions?: ReactNode;
  children: ReactNode;
  className?: string;
};

/** A glassy Havoc-dark dock panel with a slim uppercase header. */
export function Panel({ title, actions, children, className = "" }: PanelProps) {
  return (
    <section
      aria-label={title}
      className={`flex min-h-0 flex-col rounded-xl border border-white/10 bg-white/[0.03] shadow-[0_0_24px_rgba(74,158,255,0.06)] backdrop-blur ${className}`}
    >
      <header className="flex items-center justify-between gap-2 border-b border-white/5 px-3 py-2">
        <h2 className="m-0 text-[11px] font-semibold tracking-[0.14em] uppercase text-havoc-muted">
          {title}
        </h2>
        {actions}
      </header>
      <div className="min-h-0 flex-1 overflow-auto p-3">{children}</div>
    </section>
  );
}

/** Muted helper text for empty docks. */
export function EmptyHint({ children }: { children: ReactNode }) {
  return <p className="m-0 text-xs leading-relaxed text-havoc-muted">{children}</p>;
}
