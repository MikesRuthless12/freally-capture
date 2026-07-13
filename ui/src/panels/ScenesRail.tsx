import { useEffect, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

import {
  studioAddScene,
  studioMediaPaused,
  studioMediaSeek,
  studioMediaSetPaused,
  studioMediaTransport,
  studioRemoveScene,
  studioRenameScene,
  studioReorderScene,
  studioSelectScene,
  studioSetBackdropSplit,
  studioSetBackdropSync,
  studioSetPreviewScene,
  studioSetSceneBackdrop,
  studioUpdateSourceSettings,
} from "../api/commands";
import type { BackdropSplit, Collection, Scene, SceneId } from "../api/types";
import { EmptyHint, Panel } from "../components/Panel";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

type ScenesRailProps = {
  collection: Collection | null;
  /** Studio Mode: clicks target the preview pane; this scene shows green. */
  previewScene?: SceneId | null;
};

const BACKDROP_IMAGE_EXTS = ["png", "jpg", "jpeg", "bmp", "webp", "tif", "tiff"];
const BACKDROP_MEDIA_EXTS = ["gif", "mp4", "mkv", "webm", "mov", "frec"];
const BACKDROP_SPLITS: BackdropSplit[] = ["full", "left", "right", "top", "bottom"];

const fail = (what: string) => (err: unknown) => console.error(`${what} failed:`, err);

/** The Scenes rail: create/rename/remove/reorder scenes; click = program
 * (or, in Studio Mode, the preview pane). */
export function ScenesRail({ collection, previewScene }: ScenesRailProps) {
  const t = useT();
  const [renaming, setRenaming] = useState<{ id: string; draft: string } | null>(null);
  const [backdropScene, setBackdropScene] = useState<SceneId | null>(null);
  const scenes = collection?.scenes ?? [];

  const commitRename = () => {
    if (!renaming) return;
    const { id, draft } = renaming;
    setRenaming(null);
    if (draft.trim()) {
      studioRenameScene(id, draft.trim()).catch(fail("scene rename"));
    }
  };

  const backdropFor = backdropScene ? scenes.find((scene) => scene.id === backdropScene) : null;

  return (
    <Panel
      title={t("scenes-title")}
      actions={
        <button
          type="button"
          disabled={!collection}
          onClick={() => studioAddScene(t("scenes-new-scene-name")).catch(fail("scene add"))}
          title={t("scenes-add")}
          aria-label={t("scenes-add")}
          className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-60"
        >
          +
        </button>
      }
    >
      {scenes.length === 0 ? (
        <EmptyHint>{t("scenes-empty")}</EmptyHint>
      ) : (
        <ul className="m-0 flex list-none flex-col gap-1.5 p-0">
          {scenes.map((scene, index) => {
            const isActive = scene.id === collection?.activeScene;
            const isPreview = previewScene != null && scene.id === previewScene;
            const isRenaming = renaming?.id === scene.id;
            const hasBackdrop = scene.items.some((item) => item.backdrop);
            return (
              <li key={scene.id}>
                <div
                  className={`group flex items-center gap-1 rounded-lg border px-2 py-1.5 ${
                    isActive
                      ? "border-havoc-accent/50 bg-havoc-accent/10"
                      : isPreview
                        ? "border-emerald-400/50 bg-emerald-500/10"
                        : "border-white/10 bg-white/[0.02]"
                  }`}
                >
                  {isRenaming ? (
                    <input
                      autoFocus
                      value={renaming.draft}
                      onChange={(event) => setRenaming({ id: scene.id, draft: event.target.value })}
                      onBlur={commitRename}
                      onKeyDown={(event) => {
                        if (event.key === "Enter") commitRename();
                        if (event.key === "Escape") setRenaming(null);
                      }}
                      aria-label={t("scenes-rename", { name: scene.name })}
                      className="min-w-0 flex-1 rounded border border-havoc-accent/50 bg-transparent px-1 text-xs text-havoc-text outline-none"
                    />
                  ) : (
                    <button
                      type="button"
                      onClick={() =>
                        (previewScene != null
                          ? studioSetPreviewScene(scene.id)
                          : studioSelectScene(scene.id)
                        ).catch(fail("scene select"))
                      }
                      onDoubleClick={() => setRenaming({ id: scene.id, draft: scene.name })}
                      title={
                        isActive
                          ? t("scenes-on-program")
                          : previewScene != null
                            ? t("scenes-preview", { name: scene.name })
                            : t("scenes-switch-to", { name: scene.name })
                      }
                      className="min-w-0 flex-1 truncate text-left text-xs text-havoc-text"
                    >
                      {scene.name}
                      <span className="ml-1.5 text-[10px] text-havoc-muted">
                        {scene.items.length > 0 ? scene.items.length : ""}
                      </span>
                    </button>
                  )}
                  <span
                    className={`shrink-0 items-center gap-0.5 ${hasBackdrop ? "flex" : "hidden group-hover:flex"}`}
                  >
                    <button
                      type="button"
                      onClick={() => setBackdropScene(scene.id)}
                      title={t("scenes-backdrop")}
                      aria-label={t("scenes-backdrop-aria", { name: scene.name })}
                      className={`rounded px-1 text-[10px] ${
                        hasBackdrop
                          ? "text-havoc-accent"
                          : "text-havoc-muted enabled:hover:text-havoc-text"
                      }`}
                    >
                      🖼
                    </button>
                  </span>
                  <span className="hidden shrink-0 items-center gap-0.5 group-hover:flex">
                    <button
                      type="button"
                      disabled={index === 0}
                      onClick={() =>
                        studioReorderScene(scene.id, index - 1).catch(fail("scene reorder"))
                      }
                      title={t("scenes-move-up")}
                      aria-label={t("scenes-move-up-aria", { name: scene.name })}
                      className="rounded px-1 text-[10px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
                    >
                      ▲
                    </button>
                    <button
                      type="button"
                      disabled={index === scenes.length - 1}
                      onClick={() =>
                        studioReorderScene(scene.id, index + 1).catch(fail("scene reorder"))
                      }
                      title={t("scenes-move-down")}
                      aria-label={t("scenes-move-down-aria", { name: scene.name })}
                      className="rounded px-1 text-[10px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
                    >
                      ▼
                    </button>
                    <button
                      type="button"
                      disabled={scenes.length === 1}
                      onClick={() => studioRemoveScene(scene.id).catch(fail("scene remove"))}
                      title={scenes.length === 1 ? t("scenes-last-stays") : t("scenes-remove")}
                      aria-label={t("scenes-remove-aria", { name: scene.name })}
                      className="rounded px-1 text-xs text-havoc-muted enabled:hover:text-red-400 disabled:opacity-40"
                    >
                      ×
                    </button>
                  </span>
                </div>
              </li>
            );
          })}
        </ul>
      )}
      {collection && backdropFor && (
        <BackdropDialog
          collection={collection}
          scene={backdropFor}
          onClose={() => setBackdropScene(null)}
        />
      )}
    </Panel>
  );
}

/** `123.4s` → `m:ss` for the transport readout. */
function formatTime(seconds: number): string {
  const whole = Math.max(0, Math.floor(seconds));
  const minutes = Math.floor(whole / 60);
  const secs = whole % 60;
  return `${minutes}:${secs.toString().padStart(2, "0")}`;
}

/** The per-scene Backdrop dialog: pick a wallpaper (image / GIF / looping
 * video) that sits pinned under everything, position it (full canvas or a
 * half, the capture seated into the other half), and for videos a full
 * transport: play/pause, a scrubber, loop on/off, true reverse, and the
 * "start playback with recording" hold. */
function BackdropDialog({
  collection,
  scene,
  onClose,
}: {
  collection: Collection;
  scene: Scene;
  onClose: () => void;
}) {
  const t = useT();
  const item = scene.items.find((entry) => entry.backdrop) ?? null;
  const source = item ? (collection.sources.find((s) => s.id === item.source) ?? null) : null;
  const media = source?.kind === "media" ? source : null;
  const isMedia = media !== null;
  const syncOn = media?.startWithRecording ?? false;
  const split: BackdropSplit = item?.backdrop ?? "full";
  const [paused, setPaused] = useState(false);
  const [transport, setTransport] = useState({ position: 0, duration: 0 });
  // While the user drags the scrubber, the poll must not fight the thumb.
  const [scrubbing, setScrubbing] = useState<number | null>(null);

  useEffect(() => {
    if (item && isMedia) {
      studioMediaPaused(item.source).then(setPaused).catch(fail("backdrop pause state"));
    }
  }, [item, item?.source, isMedia]);

  // Poll the playhead while a media backdrop is shown (2 Hz is plenty for a
  // readout; the canvas itself is the real preview).
  useEffect(() => {
    if (!item || !isMedia) return;
    const sourceId = item.source;
    const tick = () => {
      studioMediaTransport(sourceId).then(setTransport).catch(fail("backdrop transport"));
    };
    tick();
    const timer = setInterval(tick, 500);
    return () => clearInterval(timer);
  }, [item, item?.source, isMedia]);

  /** Write one field of the media source's settings (loop / reverse). */
  const updateMedia = (change: { loop?: boolean; reverse?: boolean }) => {
    if (!media) return;
    studioUpdateSourceSettings(media.id, {
      kind: "media",
      path: media.path,
      loop: change.loop ?? media.loop,
      hwDecode: media.hwDecode,
      startWithRecording: media.startWithRecording ?? false,
      reverse: change.reverse ?? media.reverse ?? false,
    }).catch(fail("backdrop media settings"));
  };

  const choose = async () => {
    let picked: string | string[] | null = null;
    try {
      picked = await open({
        multiple: false,
        directory: false,
        filters: [
          {
            name: t("backdrop-filter-all"),
            extensions: [...BACKDROP_IMAGE_EXTS, ...BACKDROP_MEDIA_EXTS],
          },
          { name: t("backdrop-filter-images"), extensions: BACKDROP_IMAGE_EXTS },
          { name: t("backdrop-filter-media"), extensions: BACKDROP_MEDIA_EXTS },
        ],
      });
    } catch (err) {
      console.error("file dialog failed:", err);
      return;
    }
    if (typeof picked === "string") {
      studioSetSceneBackdrop(scene.id, picked).catch(fail("backdrop set"));
    }
  };

  return (
    <PickerShell title={t("backdrop-title", { name: scene.name })} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-havoc-muted">{t("backdrop-hint")}</p>
        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={() => void choose()}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 font-semibold enabled:hover:bg-havoc-accent/25"
          >
            {t("backdrop-choose")}
          </button>
          {item && (
            <button
              type="button"
              onClick={() => studioSetSceneBackdrop(scene.id, null).catch(fail("backdrop clear"))}
              className="rounded-md border border-white/10 px-3 py-1.5 text-havoc-muted hover:border-red-400/50 hover:text-red-400"
            >
              {t("backdrop-remove")}
            </button>
          )}
        </div>
        {source ? (
          <p className="m-0 truncate text-havoc-muted" title={"path" in source ? source.path : ""}>
            {"path" in source ? source.path : source.name}
          </p>
        ) : (
          <p className="m-0 text-havoc-muted">{t("backdrop-none")}</p>
        )}
        <fieldset className="m-0 border-0 p-0" disabled={!item}>
          <legend className="mb-1 p-0 text-havoc-muted">{t("backdrop-position")}</legend>
          <div className="flex flex-wrap gap-1.5">
            {BACKDROP_SPLITS.map((candidate) => (
              <button
                key={candidate}
                type="button"
                onClick={() =>
                  studioSetBackdropSplit(scene.id, candidate).catch(fail("backdrop split"))
                }
                aria-pressed={split === candidate}
                className={`rounded-md border px-2 py-1 ${
                  split === candidate && item
                    ? "border-havoc-accent/60 bg-havoc-accent/15"
                    : "border-white/10 text-havoc-muted enabled:hover:text-havoc-text"
                } disabled:opacity-50`}
              >
                {t(`backdrop-split-${candidate}`)}
              </button>
            ))}
          </div>
        </fieldset>
        {item && media && (
          <div className="flex flex-col gap-2 border-t border-white/5 pt-2">
            <div className="flex items-center gap-2">
              <button
                type="button"
                onClick={() => {
                  studioMediaSetPaused(item.source, !paused).catch(fail("backdrop preview"));
                  setPaused(!paused);
                }}
                aria-label={paused ? t("backdrop-preview-play") : t("backdrop-preview-pause")}
                title={paused ? t("backdrop-preview-play") : t("backdrop-preview-pause")}
                className="shrink-0 rounded-md border border-white/10 px-2.5 py-1 text-sm text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
              >
                {paused ? "▶" : "⏸"}
              </button>
              <input
                type="range"
                min={0}
                max={Math.max(transport.duration, 0.1)}
                step={0.1}
                disabled={transport.duration <= 0}
                value={scrubbing ?? Math.min(transport.position, transport.duration)}
                aria-label={t("backdrop-scrub")}
                onChange={(event) => {
                  const target = Number(event.target.value);
                  setScrubbing(target);
                  studioMediaSeek(item.source, target).catch(fail("backdrop seek"));
                }}
                onPointerUp={() => setScrubbing(null)}
                onKeyUp={() => setScrubbing(null)}
                className="min-w-0 flex-1 accent-havoc-accent"
              />
              <span className="shrink-0 tabular-nums text-havoc-muted">
                {formatTime(scrubbing ?? transport.position)}
                {transport.duration > 0 ? ` / ${formatTime(transport.duration)}` : ""}
              </span>
            </div>
            <div className="flex flex-wrap items-center gap-4">
              <label className="flex items-center gap-2">
                <input
                  type="checkbox"
                  checked={media.loop}
                  onChange={(event) => updateMedia({ loop: event.target.checked })}
                />
                {t("backdrop-loop")}
              </label>
              <label className="flex items-center gap-2">
                <input
                  type="checkbox"
                  checked={media.reverse ?? false}
                  onChange={(event) => updateMedia({ reverse: event.target.checked })}
                />
                {t("backdrop-reverse")}
              </label>
            </div>
            <p className="m-0 text-havoc-muted">{t("backdrop-reverse-hint")}</p>
            <label className="flex items-center gap-2">
              <input
                type="checkbox"
                checked={syncOn}
                onChange={(event) =>
                  studioSetBackdropSync(scene.id, event.target.checked).catch(fail("backdrop sync"))
                }
              />
              {t("backdrop-sync")}
            </label>
            <p className="m-0 text-havoc-muted">{t("backdrop-sync-hint")}</p>
          </div>
        )}
      </div>
    </PickerShell>
  );
}
