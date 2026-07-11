import { useEffect, useRef } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";

import { studioMultiviewSet, studioSelectScene, studioSetPreviewScene } from "../api/commands";
import type { Scene, SceneId, StudioDto } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

/** Thumbnail poll (matches the backend multiview cadence, ~150ms). */
const POLL_MS = 180;

/**
 * The multiview monitor (CAP-M06): a grid of live thumbnails of every scene,
 * with red (program) / green (preview) tally borders. Clicking a cell cuts to
 * that scene, or — in Studio Mode — stages it in preview. While open it asks the
 * render loop to keep every scene's sources live and publish per-scene
 * thumbnails.
 */
export function MultiviewDialog({
  studio,
  onClose,
}: {
  studio: StudioDto | null;
  onClose: () => void;
}) {
  const t = useT();
  const scenes = studio?.collection.scenes ?? [];
  const activeScene = studio?.collection.activeScene ?? null;
  const previewScene = studio?.studioMode?.previewScene ?? null;
  const studioMode = studio?.studioMode != null;

  // Drive the backend thumbnail rendering only while the monitor is open.
  useEffect(() => {
    studioMultiviewSet(true).catch(() => undefined);
    return () => {
      studioMultiviewSet(false).catch(() => undefined);
    };
  }, []);

  // Square-ish grid, 1×1 up to 5×5.
  const cols = Math.min(5, Math.max(1, Math.ceil(Math.sqrt(scenes.length))));

  const select = (id: SceneId) => {
    (studioMode ? studioSetPreviewScene(id) : studioSelectScene(id)).catch((err) =>
      console.error("multiview select failed:", err),
    );
  };

  return (
    <PickerShell title={t("multiview-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-2">
        <div
          className="grid gap-2"
          style={{ gridTemplateColumns: `repeat(${cols}, minmax(0, 1fr))` }}
        >
          {scenes.map((scene) => (
            <MultiviewCell
              key={scene.id}
              scene={scene}
              isProgram={scene.id === activeScene}
              isPreview={scene.id === previewScene}
              onSelect={() => select(scene.id)}
              programLabel={t("multiview-program")}
              previewLabel={t("multiview-preview")}
            />
          ))}
        </div>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {studioMode ? t("multiview-hint-stage") : t("multiview-hint-cut")}
        </p>
      </div>
    </PickerShell>
  );
}

function MultiviewCell({
  scene,
  isProgram,
  isPreview,
  onSelect,
  programLabel,
  previewLabel,
}: {
  scene: Scene;
  isProgram: boolean;
  isPreview: boolean;
  onSelect: () => void;
  programLabel: string;
  previewLabel: string;
}) {
  const imgRef = useRef<HTMLImageElement>(null);

  useEffect(() => {
    if (typeof createImageBitmap === "undefined") return; // jsdom in tests
    let stopped = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    let lastSeq = "";
    let objectUrl: string | null = null;
    const url = convertFileSrc(`multiview/${scene.id}`, "preview");
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
        // The studio is restarting or the thumbnail isn't rendered yet — retry.
      }
      if (!stopped) timer = setTimeout(tick, POLL_MS);
    };
    void tick();
    return () => {
      stopped = true;
      if (timer) clearTimeout(timer);
      if (objectUrl) URL.revokeObjectURL(objectUrl);
    };
  }, [scene.id]);

  const border = isProgram
    ? "border-red-500"
    : isPreview
      ? "border-emerald-500"
      : "border-white/10";

  return (
    <button
      type="button"
      onClick={onSelect}
      className={`relative aspect-video overflow-hidden rounded border-2 ${border} bg-black/60`}
    >
      <img ref={imgRef} alt={scene.name} className="h-full w-full object-contain" />
      <span className="absolute inset-x-0 bottom-0 truncate bg-black/60 px-1 py-0.5 text-left text-[10px] text-havoc-text">
        {scene.name}
      </span>
      {isProgram && (
        <span className="absolute top-1 left-1 rounded bg-red-600 px-1 text-[9px] font-bold text-white">
          {programLabel}
        </span>
      )}
      {isPreview && !isProgram && (
        <span className="absolute top-1 left-1 rounded bg-emerald-600 px-1 text-[9px] font-bold text-white">
          {previewLabel}
        </span>
      )}
    </button>
  );
}
