import { useState } from "react";

import { studioSetItemTransform } from "../api/commands";
import type { ProgramStatus, SceneId, SceneItem, Transform } from "../api/types";
import { useT } from "../i18n/t";
import { clampScalesToCanvas, slideIntoCanvas, type Size } from "../lib/constrain";
import { copyTransform, useClipboard } from "../lib/clipboard";
import {
  ANCHORS,
  anchorPoint,
  anchorsEqual,
  CENTER_ANCHOR,
  displayedSize,
  moveAnchorTo,
  withSize,
  type Anchor,
} from "../lib/edit-transform";
import { contentSize, effectiveSourceSize } from "../lib/transform";
import { NumberField } from "./NumberField";
import { PickerShell } from "./PickerShell";

const fail = (what: string) => (err: unknown) => console.error(`${what} failed:`, err);

/**
 * The numeric Edit Transform panel (CAP-M05): typed X/Y (relative to a chosen
 * anchor), W/H, rotation, and crop, with nudge arrows, plus copy/paste of the
 * whole transform. Every edit runs through the same canvas constraints as an
 * on-canvas drag (never sized past the frame, never moved outside it) and lands
 * on the undo stack via `studio_set_item_transform`.
 */
export function EditTransformDialog({
  sceneId,
  item,
  sourceName,
  program,
  canvasW,
  canvasH,
  onClose,
}: {
  sceneId: SceneId;
  item: SceneItem;
  sourceName: string;
  program: ProgramStatus | null;
  canvasW: number;
  canvasH: number;
  onClose: () => void;
}) {
  const t = useT();
  const clipboard = useClipboard();
  const [anchor, setAnchor] = useState<Anchor>(CENTER_ANCHOR);

  const tf = item.transform;
  const canvas: Size = { w: canvasW, h: canvasH };
  const status = program?.sources[item.source];
  const source =
    status?.width && status?.height
      ? effectiveSourceSize(status.width, status.height, item.filters)
      : null;
  const content = source ? contentSize(source.w, source.h, tf.crop) : null;

  const apply = (next: Transform) =>
    studioSetItemTransform(sceneId, item.id, next).catch(fail("edit transform"));

  const position = content ? anchorPoint(tf, content, anchor) : { x: tf.x, y: tf.y };
  const size = content ? displayedSize(tf, content) : null;

  const setPosition = (x: number, y: number) => {
    if (content) {
      apply(slideIntoCanvas(moveAnchorTo(tf, content, anchor, x, y), content, canvas));
    } else {
      apply({ ...tf, x, y });
    }
  };

  const setSize = (w: number, h: number) => {
    if (!content) return;
    let next = withSize(tf, content, w, h);
    next = { ...next, ...clampScalesToCanvas(next, content, canvas, false) };
    apply(slideIntoCanvas(next, content, canvas));
  };

  const setRotation = (rotation: number) => {
    let next: Transform = { ...tf, rotation };
    if (content) {
      next = { ...next, ...clampScalesToCanvas(next, content, canvas, true) };
      next = slideIntoCanvas(next, content, canvas);
    }
    apply(next);
  };

  const setCrop = (edge: "left" | "top" | "right" | "bottom", value: number) => {
    if (!source) return;
    const crop = { ...tf.crop, [edge]: Math.max(0, Math.round(value)) };
    const next = contentSize(source.w, source.h, crop);
    if (!next) return; // would crop the item away entirely — ignore
    apply(slideIntoCanvas({ ...tf, crop }, next, canvas));
  };

  const paste = () => {
    const clip = clipboard.transform;
    if (!clip) return;
    const pastedContent = source ? contentSize(source.w, source.h, clip.crop) : null;
    apply(pastedContent ? slideIntoCanvas(clip, pastedContent, canvas) : clip);
  };

  return (
    <PickerShell title={t("transform-title", { name: sourceName })} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <section className="flex gap-3">
          <div className="flex flex-col gap-1">
            <span className="text-[10px] tracking-wide text-havoc-muted uppercase">
              {t("transform-anchor")}
            </span>
            <div className="grid grid-cols-3 gap-0.5">
              {ANCHORS.map((option, index) => {
                const on = anchorsEqual(option, anchor);
                return (
                  <button
                    key={index}
                    type="button"
                    aria-label={`${t("transform-anchor")} ${(index % 3) + 1},${
                      Math.floor(index / 3) + 1
                    }`}
                    aria-pressed={on}
                    onClick={() => setAnchor(option)}
                    className={`flex h-5 w-5 items-center justify-center rounded border ${
                      on ? "border-havoc-accent bg-havoc-accent/15" : "border-white/10"
                    }`}
                  >
                    <span
                      className={`h-1 w-1 rounded-full ${on ? "bg-havoc-accent" : "bg-havoc-muted"}`}
                    />
                  </button>
                );
              })}
            </div>
          </div>
          <div className="grid flex-1 grid-cols-2 gap-2">
            <NumberField
              label={t("transform-x")}
              value={Math.round(position.x)}
              min={-canvasW}
              max={2 * canvasW}
              step={1}
              onCommit={(value) => setPosition(value, position.y)}
            />
            <NumberField
              label={t("transform-y")}
              value={Math.round(position.y)}
              min={-canvasH}
              max={2 * canvasH}
              step={1}
              onCommit={(value) => setPosition(position.x, value)}
            />
          </div>
        </section>

        {size ? (
          <section className="grid grid-cols-2 gap-2">
            <NumberField
              label={t("transform-w")}
              value={Math.round(size.w)}
              min={1}
              max={canvasW}
              step={1}
              onCommit={(value) => setSize(value, size.h)}
            />
            <NumberField
              label={t("transform-h")}
              value={Math.round(size.h)}
              min={1}
              max={canvasH}
              step={1}
              onCommit={(value) => setSize(size.w, value)}
            />
          </section>
        ) : (
          <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("transform-no-size")}</p>
        )}

        <NumberField
          label={t("transform-rotation")}
          value={Math.round(tf.rotation * 10) / 10}
          min={-360}
          max={360}
          step={1}
          onCommit={setRotation}
        />

        {source && (
          <section className="flex flex-col gap-1">
            <span className="text-[10px] tracking-wide text-havoc-muted uppercase">
              {t("transform-crop")}
            </span>
            <div className="grid grid-cols-4 gap-2">
              <NumberField
                label={t("transform-crop-left")}
                value={tf.crop.left}
                min={0}
                max={source.w - tf.crop.right - 1}
                step={1}
                onCommit={(value) => setCrop("left", value)}
              />
              <NumberField
                label={t("transform-crop-top")}
                value={tf.crop.top}
                min={0}
                max={source.h - tf.crop.bottom - 1}
                step={1}
                onCommit={(value) => setCrop("top", value)}
              />
              <NumberField
                label={t("transform-crop-right")}
                value={tf.crop.right}
                min={0}
                max={source.w - tf.crop.left - 1}
                step={1}
                onCommit={(value) => setCrop("right", value)}
              />
              <NumberField
                label={t("transform-crop-bottom")}
                value={tf.crop.bottom}
                min={0}
                max={source.h - tf.crop.top - 1}
                step={1}
                onCommit={(value) => setCrop("bottom", value)}
              />
            </div>
          </section>
        )}

        <div className="flex items-center gap-2 border-t border-white/5 pt-3">
          <button
            type="button"
            onClick={() => copyTransform(tf)}
            className="rounded-md border border-white/10 px-2.5 py-1 text-xs text-havoc-muted hover:text-havoc-text"
          >
            {t("transform-copy")}
          </button>
          <button
            type="button"
            disabled={!clipboard.transform}
            onClick={paste}
            className="rounded-md border border-white/10 px-2.5 py-1 text-xs text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
          >
            {t("transform-paste")}
          </button>
          <button
            type="button"
            onClick={onClose}
            className="ml-auto rounded-md border border-white/10 px-3 py-1 text-xs text-havoc-muted hover:text-havoc-text"
          >
            {t("transform-close")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}
