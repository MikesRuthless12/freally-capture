import { useEffect, useRef, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

import { useT } from "../i18n/t";

/** The preview slot each projector target reads. */
const SLOT: Record<string, string> = {
  program: "frame",
  preview: "studio-preview",
};

const POLL_MS = 40;

/**
 * A projector window (CAP-M07): a clean, chrome-free feed of the program, the
 * Studio-Mode preview, or (CAP-M07 extension) a specific scene or source,
 * fullscreen on its display. What it shows is encoded in the window label
 * (`projector-program` / `projector-preview` / `projector-scene:<id>` /
 * `projector-source:<id>`); it only fetches the `preview://` slot — no IPC.
 * Esc closes it.
 */
export function Projector({ label }: { label: string }) {
  const t = useT();
  const target = label.replace(/^projector-/, "");
  const slot = target.startsWith("scene:")
    ? `projector-scene/${target.slice("scene:".length)}`
    : target.startsWith("source:")
      ? `projector-source/${target.slice("source:".length)}`
      : (SLOT[target] ?? "frame");
  const imgRef = useRef<HTMLImageElement>(null);
  const [hint, setHint] = useState(true);

  useEffect(() => {
    if (typeof createImageBitmap === "undefined") return;
    let stopped = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    let lastSeq = "";
    let objectUrl: string | null = null;
    const url = convertFileSrc(slot, "preview");
    const tick = async () => {
      if (stopped) return;
      try {
        const response = await fetch(url, { cache: "no-store" });
        if (response.status === 200) {
          const seq = response.headers.get("x-frame-seq") ?? "";
          if (seq !== lastSeq) {
            lastSeq = seq;
            const next = URL.createObjectURL(await response.blob());
            if (imgRef.current && !stopped) imgRef.current.src = next;
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
      if (timer) clearTimeout(timer);
      if (objectUrl) URL.revokeObjectURL(objectUrl);
    };
  }, [slot]);

  // Esc closes the projector; the exit hint fades after a moment.
  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape") void getCurrentWindow().close();
    };
    window.addEventListener("keydown", onKey);
    const fade = setTimeout(() => setHint(false), 3000);
    return () => {
      window.removeEventListener("keydown", onKey);
      clearTimeout(fade);
    };
  }, []);

  return (
    <div className="fixed inset-0 flex items-center justify-center bg-black">
      <img ref={imgRef} alt="" className="max-h-full max-w-full object-contain" />
      {hint && (
        <div className="pointer-events-none absolute bottom-4 rounded bg-black/60 px-2 py-1 text-xs text-white/70">
          {t("projector-exit-hint")}
        </div>
      )}
    </div>
  );
}
