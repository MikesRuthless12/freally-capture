import { useEffect, useRef, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";

import { settingsSet, studioTransition } from "../api/commands";
import type { Settings, TransitionKind } from "../api/types";
import { TRANSITION_KINDS } from "../api/types";

/** How often the pane refetches its JPEG (~10 fps is plenty for a preview). */
const POLL_MS = 100;

/**
 * Studio Mode's PREVIEW pane (TASK-503): the preview-side scene as a polled
 * JPEG, with the transition controls — kind, duration, and the Transition
 * button that commits Preview → Program (the audience sees the blend).
 */
export function StudioPreviewPane({
  settings,
  onSettingsSaved,
  transitioning,
}: {
  settings: Settings | null;
  onSettingsSaved: (next: Settings) => void;
  transitioning: boolean;
}) {
  const imgRef = useRef<HTMLImageElement>(null);
  const [error, setError] = useState<string | null>(null);

  // Poll the preview-side JPEG; blob-swap so the <img> never flickers.
  useEffect(() => {
    if (typeof createImageBitmap === "undefined") return; // jsdom/tests
    let stopped = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    let lastSeq = "";
    let objectUrl: string | null = null;
    const url = convertFileSrc("studio-preview", "preview");
    const tick = async () => {
      if (stopped) return;
      try {
        const response = await fetch(url, { cache: "no-store" });
        if (response.status === 200) {
          const seq = response.headers.get("x-frame-seq") ?? "";
          if (seq !== lastSeq) {
            lastSeq = seq;
            const blob = await response.blob();
            const next = URL.createObjectURL(blob);
            if (imgRef.current && !stopped) {
              imgRef.current.src = next;
            }
            if (objectUrl) URL.revokeObjectURL(objectUrl);
            objectUrl = next;
          }
        }
      } catch {
        // The studio is restarting — retry.
      }
      if (!stopped) timer = setTimeout(tick, POLL_MS);
    };
    void tick();
    return () => {
      stopped = true;
      if (timer !== undefined) clearTimeout(timer);
      if (objectUrl) URL.revokeObjectURL(objectUrl);
    };
  }, []);

  const transition = settings?.transition ?? null;
  const saveTransition = (patch: Partial<Settings["transition"]>) => {
    if (!settings || !transition) return;
    const next = { ...settings, transition: { ...transition, ...patch } };
    onSettingsSaved(next);
    setError(null);
    settingsSet(next).catch((err) => setError(String(err)));
  };

  const commit = () => {
    setError(null);
    studioTransition().catch((err) => setError(String(err)));
  };

  return (
    <section
      aria-label="Studio Mode preview"
      className="flex min-h-0 w-[38%] min-w-0 flex-col gap-2 rounded-xl border border-emerald-400/30 bg-black/60 p-2"
    >
      <div className="flex items-center justify-between">
        <span className="text-[10px] font-semibold tracking-widest text-emerald-300 uppercase">
          Preview
        </span>
        <span className="text-[10px] text-havoc-muted">click a scene to load it here</span>
      </div>
      <div className="relative min-h-0 flex-1 overflow-hidden rounded-lg bg-black">
        <img
          ref={imgRef}
          alt="Preview scene"
          className="absolute inset-0 h-full w-full object-contain"
        />
      </div>
      <div className="flex shrink-0 items-center gap-2">
        <select
          value={transition?.kind ?? "fade"}
          onChange={(event) => saveTransition({ kind: event.target.value as TransitionKind })}
          aria-label="Transition"
          className="min-w-0 flex-1 rounded border border-white/10 bg-havoc-panel px-1.5 py-1 text-[11px] text-havoc-text"
        >
          {TRANSITION_KINDS.map(([value, label]) => (
            <option key={value} value={value}>
              {label}
            </option>
          ))}
        </select>
        <input
          type="number"
          min={50}
          max={5000}
          step={50}
          value={transition?.durationMs ?? 300}
          onChange={(event) => saveTransition({ durationMs: Number(event.target.value) })}
          aria-label="Transition duration (ms)"
          title="Transition duration (ms)"
          className="w-20 rounded border border-white/10 bg-havoc-panel px-1.5 py-1 text-[11px] text-havoc-text"
        />
        <button
          type="button"
          disabled={transitioning}
          onClick={commit}
          title="Commit Preview → Program through the transition (the audience sees it)"
          className="shrink-0 rounded-md border border-emerald-400/60 bg-emerald-500/15 px-3 py-1 text-xs font-semibold text-havoc-text enabled:hover:bg-emerald-500/25 disabled:opacity-50"
        >
          {transitioning ? "Transitioning…" : "Transition ⇄"}
        </button>
      </div>
      {error && (
        <p role="alert" className="m-0 text-[11px] text-red-300">
          {error}
        </p>
      )}
    </section>
  );
}
