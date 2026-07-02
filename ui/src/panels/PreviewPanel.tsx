import { useEffect, useRef } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";

import { openPrivacySettings } from "../api/commands";
import type { PreviewStatus } from "../api/types";

type PreviewPanelProps = {
  status: PreviewStatus;
  /** `health().os` — drives the macOS permission deep-link button. */
  os?: string;
  /** The active source's kind ("webcam" picks the Camera privacy pane). */
  activeKind?: string;
};

/** Poll interval for the preview frame pipe (~30 fps). */
const FRAME_POLL_MS = 33;

/**
 * The program preview. Phase 1 draws one capture source directly (the
 * compositor takes over in 0.40.0): the Rust side parks the newest JPEG
 * behind the `preview://` scheme and this canvas polls it.
 */
export function PreviewPanel({ status, os, activeKind }: PreviewPanelProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const live = status.state === "live";

  useEffect(() => {
    if (!live) return;
    // jsdom (tests) has neither the scheme nor createImageBitmap.
    if (typeof createImageBitmap === "undefined") return;
    let stopped = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    let lastSeq = "";
    const url = convertFileSrc("frame", "preview");

    const tick = async () => {
      if (stopped) return;
      try {
        const response = await fetch(url, { cache: "no-store" });
        if (response.status === 200) {
          const seq = response.headers.get("x-frame-seq") ?? "";
          if (seq !== lastSeq) {
            lastSeq = seq;
            const blob = await response.blob();
            const bitmap = await createImageBitmap(blob);
            const canvas = canvasRef.current;
            if (canvas && !stopped) {
              if (canvas.width !== bitmap.width) canvas.width = bitmap.width;
              if (canvas.height !== bitmap.height) canvas.height = bitmap.height;
              canvas.getContext("2d")?.drawImage(bitmap, 0, 0);
            }
            bitmap.close();
          }
        }
      } catch {
        // The pump is restarting or the scheme isn't up yet — just retry.
      }
      if (!stopped) timer = setTimeout(tick, FRAME_POLL_MS);
    };
    void tick();
    return () => {
      stopped = true;
      if (timer !== undefined) clearTimeout(timer);
    };
  }, [live]);

  return (
    <section
      aria-label="Program preview"
      className="relative flex min-h-0 min-w-0 items-center justify-center overflow-hidden rounded-xl border border-white/10 bg-black/60 p-4"
    >
      {live ? (
        <>
          <canvas
            ref={canvasRef}
            role="img"
            aria-label={`Live preview of ${status.label ?? "the selected source"}`}
            className="max-h-full max-w-full object-contain"
          />
          <div className="pointer-events-none absolute bottom-3 left-3 flex items-center gap-2 rounded-md bg-black/70 px-2 py-1 text-[11px] text-havoc-muted">
            <span className="h-1.5 w-1.5 rounded-full bg-emerald-400" aria-hidden="true" />
            <span className="max-w-56 truncate text-havoc-text">{status.label}</span>
            {status.width ? (
              <span>
                {status.width}×{status.height}
              </span>
            ) : null}
            {status.fps !== undefined && <span>{status.fps} fps</span>}
            {status.dropped ? (
              <span className="text-amber-300">{status.dropped} dropped</span>
            ) : null}
          </div>
        </>
      ) : (
        <div className="flex aspect-video max-h-full w-full min-w-0 flex-col items-center justify-center gap-2 rounded-lg border border-dashed border-white/15 bg-havoc-panel/40 px-6">
          <span className="bg-gradient-to-r from-havoc-accent to-havoc-accent-2 bg-clip-text text-sm font-semibold tracking-widest text-transparent uppercase">
            Program
          </span>
          {status.state === "waiting" ? (
            <p className="m-0 text-center text-xs text-havoc-muted" role="status">
              {activeKind === "portal"
                ? "Choose what to share in the system dialog…"
                : `Starting ${status.label ?? "capture"}…`}
            </p>
          ) : status.state === "error" ? (
            <div className="flex flex-col items-center gap-2">
              <p className="m-0 text-center text-xs text-red-400" role="alert">
                {status.errorMessage ?? "Capture failed."}
              </p>
              {status.errorCode === "permission" && os === "macos" && (
                <button
                  type="button"
                  onClick={() =>
                    void openPrivacySettings(activeKind === "webcam" ? "camera" : "screenRecording")
                  }
                  className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
                >
                  Open {activeKind === "webcam" ? "Camera" : "Screen Recording"} settings
                </button>
              )}
            </div>
          ) : (
            <p className="m-0 text-center text-xs text-havoc-muted">
              No source selected — add a Display Capture, Window Capture, or Webcam in Sources.
            </p>
          )}
        </div>
      )}
    </section>
  );
}
