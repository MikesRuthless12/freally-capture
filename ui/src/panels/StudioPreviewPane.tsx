import { useEffect, useRef, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

import { settingsSet, studioTransition } from "../api/commands";
import type { Settings, StingerMatte, TransitionKind } from "../api/types";
import { STINGER_MATTES, TRANSITION_KINDS } from "../api/types";
import { useT } from "../i18n/t";

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
  const t = useT();
  const imgRef = useRef<HTMLImageElement>(null);
  const [error, setError] = useState<string | null>(null);
  const [hasFrame, setHasFrame] = useState(false);
  const [mirrors, setMirrors] = useState(false);

  // Poll the preview-side JPEG; blob-swap so the <img> never flickers. When the
  // preview scene *is* the program scene (its own slot is empty — e.g. the moment
  // Studio Mode turns on, or with a single scene), fall back to the program frame
  // so the pane shows the program instead of a broken image.
  useEffect(() => {
    if (typeof createImageBitmap === "undefined") return; // jsdom/tests
    let stopped = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    let lastSeq = "";
    let objectUrl: string | null = null;
    const previewUrl = convertFileSrc("studio-preview", "preview");
    const programUrl = convertFileSrc("frame", "preview");
    const tick = async () => {
      if (stopped) return;
      try {
        let response = await fetch(previewUrl, { cache: "no-store" });
        let mirroring = false;
        if (response.status !== 200) {
          response = await fetch(programUrl, { cache: "no-store" });
          mirroring = true;
        }
        if (response.status === 200) {
          const seq = (mirroring ? "p:" : "s:") + (response.headers.get("x-frame-seq") ?? "");
          if (seq !== lastSeq) {
            lastSeq = seq;
            const blob = await response.blob();
            const next = URL.createObjectURL(blob);
            if (imgRef.current && !stopped) imgRef.current.src = next;
            if (objectUrl) URL.revokeObjectURL(objectUrl);
            objectUrl = next;
            if (!stopped) {
              setHasFrame(true);
              setMirrors(mirroring);
            }
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
      aria-label={t("studio-preview-label")}
      className="flex min-h-0 w-[38%] min-w-0 flex-col gap-2 rounded-xl border border-emerald-400/30 bg-black/60 p-2"
    >
      <div className="flex items-center justify-between">
        <span className="text-[10px] font-semibold tracking-widest text-emerald-300 uppercase">
          {t("studio-preview-heading")}
        </span>
        <span className="text-[10px] text-havoc-muted">{t("studio-preview-hint")}</span>
      </div>
      <div className="relative min-h-0 flex-1 overflow-hidden rounded-lg bg-black">
        <img
          ref={imgRef}
          alt=""
          className={`absolute inset-0 h-full w-full object-contain ${hasFrame ? "" : "hidden"}`}
        />
        {!hasFrame && (
          <div className="absolute inset-0 flex items-center justify-center text-[11px] text-havoc-muted">
            {t("studio-preview-empty")}
          </div>
        )}
        {hasFrame && mirrors && (
          <span className="absolute top-1.5 left-1.5 rounded bg-black/50 px-1.5 py-0.5 text-[9px] tracking-wide text-havoc-muted uppercase">
            {t("studio-preview-mirrors")}
          </span>
        )}
      </div>
      <div className="flex shrink-0 items-center gap-2">
        <select
          value={transition?.kind ?? "fade"}
          onChange={(event) => saveTransition({ kind: event.target.value as TransitionKind })}
          aria-label={t("studio-preview-transition-select")}
          className="min-w-0 flex-1 rounded border border-white/10 bg-havoc-panel px-1.5 py-1 text-[11px] text-havoc-text"
        >
          {TRANSITION_KINDS.map(([value, labelKey]) => (
            <option key={value} value={value}>
              {t(labelKey)}
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
          aria-label={t("studio-preview-duration")}
          title={t("studio-preview-duration")}
          className="w-20 rounded border border-white/10 bg-havoc-panel px-1.5 py-1 text-[11px] text-havoc-text"
        />
        <button
          type="button"
          disabled={transitioning}
          onClick={commit}
          title={t("studio-preview-commit-title")}
          className="shrink-0 rounded-md border border-emerald-400/60 bg-emerald-500/15 px-3 py-1 text-xs font-semibold text-havoc-text enabled:hover:bg-emerald-500/25 disabled:opacity-50"
        >
          {transitioning
            ? t("studio-preview-transitioning")
            : t("studio-preview-transition-button")}
        </button>
      </div>
      {transition?.kind === "lumaImage" && (
        <div className="flex shrink-0 items-center gap-2">
          <input
            value={transition.lumaImage}
            onChange={(event) => saveTransition({ lumaImage: event.target.value })}
            placeholder={t("studio-preview-luma-placeholder")}
            aria-label={t("studio-preview-luma-label")}
            className="min-w-0 flex-1 rounded border border-white/10 bg-havoc-panel px-1.5 py-1 font-mono text-[11px] text-havoc-text"
          />
          <button
            type="button"
            onClick={() => {
              void open({
                multiple: false,
                filters: [
                  {
                    name: t("studio-preview-filter-images"),
                    extensions: ["png", "jpg", "jpeg", "bmp", "webp"],
                  },
                ],
              }).then((picked) => {
                if (typeof picked === "string") saveTransition({ lumaImage: picked });
              });
            }}
            className="shrink-0 rounded border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:text-havoc-text"
          >
            {t("studio-preview-browse")}
          </button>
        </div>
      )}
      {transition?.kind === "stinger" && (
        <div className="flex shrink-0 items-center gap-2">
          <input
            value={transition.stingerPath}
            onChange={(event) => saveTransition({ stingerPath: event.target.value })}
            placeholder={t("studio-preview-stinger-placeholder")}
            aria-label={t("studio-preview-stinger-label")}
            className="min-w-0 flex-1 rounded border border-white/10 bg-havoc-panel px-1.5 py-1 font-mono text-[11px] text-havoc-text"
          />
          <button
            type="button"
            onClick={() => {
              void open({
                multiple: false,
                filters: [
                  {
                    name: t("studio-preview-filter-video"),
                    extensions: ["mov", "mp4", "mkv", "webm", "avi"],
                  },
                ],
              }).then((picked) => {
                if (typeof picked === "string") saveTransition({ stingerPath: picked });
              });
            }}
            className="shrink-0 rounded border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:text-havoc-text"
          >
            {t("studio-preview-browse")}
          </button>
          <input
            type="number"
            min={0}
            max={5000}
            step={50}
            value={transition.stingerCutMs}
            onChange={(event) => saveTransition({ stingerCutMs: Number(event.target.value) })}
            aria-label={t("studio-preview-stinger-cut-label")}
            title={t("studio-preview-stinger-cut-title")}
            className="w-20 rounded border border-white/10 bg-havoc-panel px-1.5 py-1 text-[11px] text-havoc-text"
          />
        </div>
      )}
      {transition?.kind === "stinger" && (
        <div className="flex shrink-0 flex-wrap items-center gap-2">
          <label className="text-[11px] text-havoc-muted">
            {t("studio-preview-stinger-matte-label")}
          </label>
          <select
            value={transition.stingerMatte ?? "none"}
            onChange={(event) =>
              saveTransition({ stingerMatte: event.target.value as StingerMatte })
            }
            aria-label={t("studio-preview-stinger-matte-label")}
            title={t("studio-preview-stinger-matte-title")}
            className="rounded border border-white/10 bg-havoc-panel px-1.5 py-1 text-[11px] text-havoc-text"
          >
            {STINGER_MATTES.map(([value, key]) => (
              <option key={value} value={value}>
                {t(key)}
              </option>
            ))}
          </select>
          <label className="ml-2 text-[11px] text-havoc-muted">
            {t("studio-preview-stinger-duck-label")}
          </label>
          <input
            type="number"
            min={0}
            max={60}
            step={1}
            value={transition.stingerDuckDb ?? 0}
            onChange={(event) => saveTransition({ stingerDuckDb: Number(event.target.value) })}
            aria-label={t("studio-preview-stinger-duck-label")}
            title={t("studio-preview-stinger-duck-title")}
            className="w-16 rounded border border-white/10 bg-havoc-panel px-1.5 py-1 text-[11px] text-havoc-text"
          />
          <span className="text-[11px] text-havoc-muted">{t("studio-preview-stinger-duck-unit")}</span>
        </div>
      )}
      {error && (
        <p role="alert" className="m-0 text-[11px] text-red-300">
          {error}
        </p>
      )}
    </section>
  );
}
