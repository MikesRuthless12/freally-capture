import { useState } from "react";

import {
  studioAddScene,
  studioRemoveScene,
  studioRenameScene,
  studioReorderScene,
  studioSelectScene,
  studioSetPreviewScene,
} from "../api/commands";
import type { Collection, SceneId } from "../api/types";
import { EmptyHint, Panel } from "../components/Panel";
import { useT } from "../i18n/t";

type ScenesRailProps = {
  collection: Collection | null;
  /** Studio Mode: clicks target the preview pane; this scene shows green. */
  previewScene?: SceneId | null;
};

/** The Scenes rail: create/rename/remove/reorder scenes; click = program
 * (or, in Studio Mode, the preview pane). */
export function ScenesRail({ collection, previewScene }: ScenesRailProps) {
  const t = useT();
  const [renaming, setRenaming] = useState<{ id: string; draft: string } | null>(null);
  const scenes = collection?.scenes ?? [];

  const fail = (what: string) => (err: unknown) => console.error(`${what} failed:`, err);

  const commitRename = () => {
    if (!renaming) return;
    const { id, draft } = renaming;
    setRenaming(null);
    if (draft.trim()) {
      studioRenameScene(id, draft.trim()).catch(fail("scene rename"));
    }
  };

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
    </Panel>
  );
}
