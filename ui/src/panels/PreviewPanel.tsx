/** The program preview — the composed program frame renders here from Phase 2. */
export function PreviewPanel() {
  return (
    <section
      aria-label="Program preview"
      className="flex min-h-0 min-w-0 items-center justify-center rounded-xl border border-white/10 bg-black/60 p-4"
    >
      <div className="flex aspect-video max-h-full w-full min-w-0 flex-col items-center justify-center gap-2 rounded-lg border border-dashed border-white/15 bg-havoc-panel/40 px-6">
        <span className="bg-gradient-to-r from-havoc-accent to-havoc-accent-2 bg-clip-text text-sm font-semibold tracking-widest text-transparent uppercase">
          Program
        </span>
        <p className="m-0 text-center text-xs text-havoc-muted">
          No sources yet — Display Capture and Webcam arrive with capture (0.25.0).
        </p>
      </div>
    </section>
  );
}
