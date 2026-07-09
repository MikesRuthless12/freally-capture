import { useEffect, useRef, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";

import { studioSetVertical } from "../api/commands";
import type { StudioDto } from "../api/types";
import { NumberField } from "../components/NumberField";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/** How often the pane refetches its JPEG (~10 fps is plenty for a preview). */
const POLL_MS = 100;

/**
 * The second (vertical) output canvas (TASK-604): enable it, pick the scene
 * it composes and its dimensions, and watch it live. Item positions are
 * canvas-pixel-true — select the chosen scene in the rail to arrange it
 * while this preview shows the vertical result.
 */
export function VerticalCanvasDialog({
  studio,
  onClose,
}: {
  studio: StudioDto | null;
  onClose: () => void;
}) {
  const t = useT();
  const current = studio?.collection.vertical ?? null;
  const scenes = studio?.collection.scenes ?? [];
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const imgRef = useRef<HTMLImageElement>(null);

  // Poll the vertical canvas's JPEG while enabled; blob-swap, no flicker.
  useEffect(() => {
    if (!current || typeof createImageBitmap === "undefined") return;
    let stopped = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    let lastSeq = "";
    let objectUrl: string | null = null;
    const url = convertFileSrc("vertical-preview", "preview");
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
      if (timer) clearTimeout(timer);
      if (objectUrl) URL.revokeObjectURL(objectUrl);
    };
  }, [current]);

  const apply = async (next: { width: number; height: number; scene: string } | null) => {
    setBusy(true);
    setError(null);
    try {
      await studioSetVertical(next);
    } catch (raw) {
      setError(String(raw));
    } finally {
      setBusy(false);
    }
  };

  return (
    <PickerShell title={t("vertical-title")} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={current !== null}
            disabled={busy || scenes.length === 0}
            onChange={(event) =>
              void apply(
                event.target.checked && scenes.length > 0
                  ? {
                      width: 1080,
                      height: 1920,
                      scene: studio?.collection.activeScene ?? scenes[0].id,
                    }
                  : null,
              )
            }
          />
          {t("vertical-enable")}
        </label>

        {current && (
          <>
            <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
              {t("vertical-scene-label")}
              <select
                value={current.scene}
                disabled={busy}
                onChange={(event) => void apply({ ...current, scene: event.target.value })}
                className={inputClass}
              >
                {scenes.map((scene) => (
                  <option key={scene.id} value={scene.id}>
                    {scene.name}
                  </option>
                ))}
              </select>
            </label>

            <div className="grid grid-cols-2 gap-2">
              <NumberField
                label={t("vertical-width")}
                value={current.width}
                min={16}
                max={16384}
                step={2}
                onCommit={(value) => void apply({ ...current, width: Math.round(value) })}
              />
              <NumberField
                label={t("vertical-height")}
                value={current.height}
                min={16}
                max={16384}
                step={2}
                onCommit={(value) => void apply({ ...current, height: Math.round(value) })}
              />
            </div>

            <div className="flex justify-center rounded-lg border border-white/10 bg-black/40 p-2">
              <img
                ref={imgRef}
                alt={t("vertical-preview-alt")}
                className="max-h-64 max-w-full object-contain"
              />
            </div>

            <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("vertical-note")}</p>
          </>
        )}

        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}
        <div className="flex justify-end">
          <button
            type="button"
            onClick={onClose}
            className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
          >
            {t("vertical-close")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}
