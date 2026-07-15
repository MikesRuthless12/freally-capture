import { useState } from "react";

import {
  studioDownstreamAdd,
  studioDownstreamMove,
  studioDownstreamRemove,
  studioDownstreamSetEnabled,
  studioDownstreamSetOpacity,
  studioDownstreamSetTransform,
} from "../api/commands";
import type { Collection, DownstreamKeyer, SourceId } from "../api/types";
import { NumberField } from "../components/NumberField";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

/**
 * Downstream keyers (CAP-N24): a managed list of overlay layers composited on
 * the PROGRAM output — above every scene, surviving scene cuts (a station logo,
 * a LIVE badge, a persistent lower-third). Reorder, opacity, position/size, and
 * on/off per layer. Every change is a tracked command; the studio event re-feeds
 * this dialog with the updated collection.
 */
export function DownstreamKeyersDialog({
  collection,
  onClose,
}: {
  collection: Collection | null;
  onClose: () => void;
}) {
  const t = useT();
  const [addOpen, setAddOpen] = useState(false);
  const layers = collection?.downstream ?? [];
  const sources = collection?.sources ?? [];
  const nameOf = (id: SourceId) => sources.find((source) => source.id === id)?.name ?? id;
  const warn = (what: string) => (err: unknown) => console.error(`${what} failed:`, err);

  const setTransform = (dsk: DownstreamKeyer, patch: Partial<DownstreamKeyer["transform"]>) => {
    studioDownstreamSetTransform(dsk.id, { ...dsk.transform, ...patch }).catch(warn("dsk transform"));
  };

  return (
    <PickerShell title={t("dsk-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-2 text-xs text-havoc-text">
        <p className="m-0 text-[11px] leading-snug text-havoc-muted">{t("dsk-hint")}</p>

        {/* Layers are listed top-first (the on-top layer at the top of the list). */}
        {layers.length === 0 ? (
          <p className="m-0 text-havoc-muted">{t("dsk-empty")}</p>
        ) : (
          <ul className="m-0 flex list-none flex-col gap-2 p-0">
            {layers
              .map((dsk, index) => ({ dsk, index }))
              .reverse()
              .map(({ dsk, index }) => (
                <li key={dsk.id} className="flex flex-col gap-1.5 rounded-md border border-white/10 p-2">
                  <div className="flex items-center gap-2">
                    <input
                      type="checkbox"
                      checked={dsk.enabled}
                      onChange={(event) =>
                        studioDownstreamSetEnabled(dsk.id, event.target.checked).catch(
                          warn("dsk enable"),
                        )
                      }
                      aria-label={t("dsk-enable")}
                    />
                    <span className="flex-1 truncate">{nameOf(dsk.source)}</span>
                    <button
                      type="button"
                      disabled={index === layers.length - 1}
                      onClick={() => studioDownstreamMove(dsk.id, true).catch(warn("dsk move"))}
                      title={t("dsk-move-up")}
                      className="rounded px-1 text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
                    >
                      ▲
                    </button>
                    <button
                      type="button"
                      disabled={index === 0}
                      onClick={() => studioDownstreamMove(dsk.id, false).catch(warn("dsk move"))}
                      title={t("dsk-move-down")}
                      className="rounded px-1 text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
                    >
                      ▼
                    </button>
                    <button
                      type="button"
                      onClick={() => studioDownstreamRemove(dsk.id).catch(warn("dsk remove"))}
                      title={t("dsk-remove")}
                      className="rounded px-1 text-havoc-muted hover:text-red-400"
                    >
                      ×
                    </button>
                  </div>

                  <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
                    <span className="w-16 shrink-0">{t("dsk-opacity")}</span>
                    <input
                      type="range"
                      min={0}
                      max={1}
                      step={0.01}
                      value={dsk.opacity}
                      onChange={(event) =>
                        studioDownstreamSetOpacity(dsk.id, Number(event.target.value)).catch(
                          warn("dsk opacity"),
                        )
                      }
                      className="flex-1"
                    />
                    <span className="w-8 text-right font-mono">{Math.round(dsk.opacity * 100)}%</span>
                  </label>

                  <div className="grid grid-cols-3 gap-2">
                    <NumberField
                      label={t("dsk-x")}
                      value={Math.round(dsk.transform.x)}
                      min={-100000}
                      max={100000}
                      onCommit={(x) => setTransform(dsk, { x })}
                    />
                    <NumberField
                      label={t("dsk-y")}
                      value={Math.round(dsk.transform.y)}
                      min={-100000}
                      max={100000}
                      onCommit={(y) => setTransform(dsk, { y })}
                    />
                    <NumberField
                      label={t("dsk-scale")}
                      value={Math.round(dsk.transform.scaleX * 100) / 100}
                      step={0.05}
                      min={0.05}
                      max={100}
                      onCommit={(scale) => setTransform(dsk, { scaleX: scale, scaleY: scale })}
                    />
                  </div>
                </li>
              ))}
          </ul>
        )}

        <div className="relative">
          <button
            type="button"
            onClick={() => setAddOpen((open) => !open)}
            disabled={sources.length === 0}
            className="rounded-md border border-white/10 px-2 py-1.5 text-xs text-havoc-text hover:border-havoc-accent/50 disabled:opacity-40"
          >
            {t("dsk-add")}
          </button>
          {addOpen && (
            <div className="mt-1 flex max-h-48 flex-col gap-1 overflow-y-auto rounded-md border border-white/10 bg-havoc-panel p-1">
              {sources.map((source) => (
                <button
                  key={source.id}
                  type="button"
                  onClick={() => {
                    studioDownstreamAdd(source.id).catch(warn("dsk add"));
                    setAddOpen(false);
                  }}
                  className="truncate rounded px-2 py-1 text-left text-xs text-havoc-text hover:bg-white/5"
                >
                  {source.name}
                </button>
              ))}
            </div>
          )}
        </div>
      </div>
    </PickerShell>
  );
}
