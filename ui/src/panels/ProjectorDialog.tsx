import { useEffect, useState } from "react";

import { auxWindowOpen, listDisplays } from "../api/commands";
import type { Collection, DisplayInfo } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

/** What a projector can show. Scene/source (CAP-M07 extension) fullscreen a
 * specific scene or a single source; multiview opens the grid on a display. */
type Kind = "program" | "preview" | "scene" | "source" | "multiview";
const KINDS: Kind[] = ["program", "preview", "scene", "source", "multiview"];

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/**
 * Open a projector (CAP-M07): pick what to show (program / Studio-Mode preview /
 * a specific scene or source / the multiview grid) and where (a connected
 * display fullscreen, or a floating window on this screen). Opens a separate
 * always-clean output window.
 */
export function ProjectorDialog({
  collection,
  onClose,
}: {
  collection: Collection | null;
  onClose: () => void;
}) {
  const t = useT();
  const [displays, setDisplays] = useState<DisplayInfo[]>([]);
  const [kind, setKind] = useState<Kind>("program");
  // `null` = a floating window on the current screen; otherwise a display index.
  const [display, setDisplay] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);

  const scenes = collection?.scenes ?? [];
  const sources = collection?.sources ?? [];
  // `""` means "not chosen yet" → fall back to the first available (derived, so
  // no default-setting effect is needed).
  const [sceneId, setSceneId] = useState<string>("");
  const [sourceId, setSourceId] = useState<string>("");
  const effectiveSceneId = sceneId || (scenes[0]?.id ?? "");
  const effectiveSourceId = sourceId || (sources[0]?.id ?? "");

  useEffect(() => {
    listDisplays()
      .then(setDisplays)
      .catch((err) => setError(String(err)));
  }, []);

  const open = async () => {
    setError(null);
    let label: string;
    let title: string;
    if (kind === "multiview") {
      label = "multiview";
      title = t("multiview-title");
    } else if (kind === "scene") {
      if (!effectiveSceneId) {
        setError(t("projector-none"));
        return;
      }
      label = `projector-scene:${effectiveSceneId}`;
      title =
        scenes.find((scene) => scene.id === effectiveSceneId)?.name ?? t("projector-target-scene");
    } else if (kind === "source") {
      if (!effectiveSourceId) {
        setError(t("projector-none"));
        return;
      }
      label = `projector-source:${effectiveSourceId}`;
      title =
        sources.find((source) => source.id === effectiveSourceId)?.name ??
        t("projector-target-source");
    } else {
      label = `projector-${kind}`;
      title = t(`projector-target-${kind}`);
    }
    try {
      // Fullscreen on a chosen display; a floating window otherwise.
      await auxWindowOpen(label, title, display, display !== null);
      onClose();
    } catch (err) {
      setError(String(err));
    }
  };

  return (
    <PickerShell title={t("projector-title")} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <label className="flex items-center justify-between gap-3">
          <span className="text-havoc-muted">{t("projector-source")}</span>
          <select
            value={kind}
            onChange={(event) => setKind(event.target.value as Kind)}
            className={inputClass}
          >
            {KINDS.map((option) => (
              <option key={option} value={option}>
                {t(`projector-target-${option}`)}
              </option>
            ))}
          </select>
        </label>

        {kind === "scene" && (
          <label className="flex items-center justify-between gap-3">
            <span className="text-havoc-muted">{t("projector-which-scene")}</span>
            <select
              value={effectiveSceneId}
              onChange={(event) => setSceneId(event.target.value)}
              className={inputClass}
            >
              {scenes.map((scene) => (
                <option key={scene.id} value={scene.id}>
                  {scene.name}
                </option>
              ))}
            </select>
          </label>
        )}

        {kind === "source" && (
          <label className="flex items-center justify-between gap-3">
            <span className="text-havoc-muted">{t("projector-which-source")}</span>
            <select
              value={effectiveSourceId}
              onChange={(event) => setSourceId(event.target.value)}
              className={inputClass}
            >
              {sources.map((source) => (
                <option key={source.id} value={source.id}>
                  {source.name}
                </option>
              ))}
            </select>
          </label>
        )}

        <label className="flex items-center justify-between gap-3">
          <span className="text-havoc-muted">{t("projector-display")}</span>
          <select
            value={display ?? "float"}
            onChange={(event) =>
              setDisplay(event.target.value === "float" ? null : Number(event.target.value))
            }
            className={inputClass}
          >
            <option value="float">{t("projector-windowed")}</option>
            {displays.map((info) => (
              <option key={info.index} value={info.index}>
                {t("projector-display-option", {
                  n: info.index + 1,
                  w: info.width,
                  h: info.height,
                })}
                {info.primary ? ` ${t("projector-primary")}` : ""}
              </option>
            ))}
          </select>
        </label>

        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}

        <div className="flex justify-end gap-2">
          <button
            type="button"
            onClick={onClose}
            className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
          >
            {t("projector-cancel")}
          </button>
          <button
            type="button"
            onClick={() => void open()}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs text-havoc-text hover:border-havoc-accent"
          >
            {t("projector-open")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}
