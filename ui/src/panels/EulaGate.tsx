import { useEffect, useRef, useState } from "react";

import { exit } from "@tauri-apps/plugin-process";

import { eulaAccept } from "../api/commands";
import type { EulaStatus } from "../api/types";

/**
 * First-run EULA acceptance gate (Phase 8). Rendered by `App` instead of the
 * studio until the current EULA version is accepted. The user must scroll to
 * the end and click **I Agree** before the app is usable; **Decline** quits.
 * Acceptance is persisted, so it appears once (and again only if the EULA
 * version changes).
 */
export function EulaGate({ status, onAccepted }: { status: EulaStatus; onAccepted: () => void }) {
  const [scrolledToEnd, setScrolledToEnd] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const scrollRef = useRef<HTMLDivElement | null>(null);

  // If the agreement is short enough not to scroll, enable Agree immediately.
  // Measured after paint (rAF) so we never setState synchronously in the effect.
  useEffect(() => {
    const el = scrollRef.current;
    if (!el) return;
    const id = requestAnimationFrame(() => {
      if (el.scrollHeight <= el.clientHeight + 4) setScrolledToEnd(true);
    });
    return () => cancelAnimationFrame(id);
  }, []);

  const onScroll = () => {
    const el = scrollRef.current;
    if (el && el.scrollTop + el.clientHeight >= el.scrollHeight - 24) setScrolledToEnd(true);
  };

  const agree = () => {
    setBusy(true);
    setError(null);
    eulaAccept()
      .then(onAccepted)
      .catch((err) => {
        setBusy(false);
        setError(String(err));
      });
  };

  const decline = () => {
    void exit(0);
  };

  return (
    <div className="flex h-full w-full items-center justify-center bg-havoc-bg p-4 text-havoc-text">
      <div className="flex max-h-full w-full max-w-3xl flex-col gap-3 rounded-xl border border-white/10 bg-white/[0.03] p-5">
        <div className="flex items-baseline justify-between gap-2">
          <h1 className="m-0 bg-gradient-to-r from-havoc-accent to-havoc-accent-2 bg-clip-text text-lg font-bold tracking-wide text-transparent">
            Freally Capture — License Agreement
          </h1>
          <span className="shrink-0 font-mono text-[10px] text-havoc-muted">v{status.version}</span>
        </div>
        <p className="m-0 text-xs leading-relaxed text-havoc-muted">
          Please read and accept this agreement to use Freally Capture. In short: it&apos;s a
          neutral tool, and <strong className="text-havoc-text">you are solely responsible</strong>{" "}
          for what you capture, record, and broadcast — and for having the rights to it.
        </p>
        <div
          ref={scrollRef}
          onScroll={onScroll}
          className="min-h-0 flex-1 overflow-auto rounded-lg border border-white/10 bg-black/30 p-3 text-[11px] leading-relaxed whitespace-pre-wrap text-havoc-muted"
        >
          {status.text}
        </div>
        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}
        <div className="flex flex-wrap items-center justify-between gap-2">
          <span className="text-[10px] text-havoc-muted">
            {scrolledToEnd ? "Thanks for reading." : "Scroll to the end to continue."}
          </span>
          <div className="flex gap-2">
            <button
              type="button"
              onClick={decline}
              className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:border-red-400/50 hover:text-red-200"
            >
              Decline &amp; Quit
            </button>
            <button
              type="button"
              onClick={agree}
              disabled={!scrolledToEnd || busy}
              className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25 disabled:opacity-40"
            >
              I Agree
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
