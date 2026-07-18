import { useEffect, useMemo, useRef, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

import {
  auxWindowOpen,
  teleprompterControl,
  teleprompterSetFont,
  teleprompterSetMirror,
  teleprompterSetScript,
  teleprompterSetSpeed,
} from "../api/commands";
import type { TeleprompterState } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";
import { useTeleprompter } from "../lib/useTeleprompter";

/** Line-height multiple used everywhere, so a line-based offset maps to pixels
 * the same way on every surface (bigger font → bigger pixels, same reading pace). */
const LINE_HEIGHT = 1.5;
/** The dock preview renders at a fixed small size; the projector uses the
 * configured font. Because the offset is in LINES, both show the same text. */
const PREVIEW_FONT = 20;

/** Render a script line with light Markdown: `#`/`##` headings, `**bold**`,
 * `*italic*`. No HTML injection — everything is React elements. */
function inlineMarkup(text: string, keyBase: string): React.ReactNode[] {
  const nodes: React.ReactNode[] = [];
  const re = /\*\*(.+?)\*\*|\*(.+?)\*|_(.+?)_/g;
  let last = 0;
  let match: RegExpExecArray | null;
  let i = 0;
  while ((match = re.exec(text)) !== null) {
    if (match.index > last) nodes.push(text.slice(last, match.index));
    if (match[1] !== undefined) {
      nodes.push(<strong key={`${keyBase}-b${i}`}>{match[1]}</strong>);
    } else {
      nodes.push(<em key={`${keyBase}-i${i}`}>{match[2] ?? match[3]}</em>);
    }
    last = re.lastIndex;
    i += 1;
  }
  if (last < text.length) nodes.push(text.slice(last));
  return nodes;
}

function renderScript(script: string): React.ReactNode {
  return script.split("\n").map((line, index) => {
    const key = `l${index}`;
    if (line.startsWith("## ")) {
      return (
        <div key={key} className="font-bold" style={{ fontSize: "1.3em" }}>
          {inlineMarkup(line.slice(3), key)}
        </div>
      );
    }
    if (line.startsWith("# ")) {
      return (
        <div key={key} className="font-bold" style={{ fontSize: "1.6em" }}>
          {inlineMarkup(line.slice(2), key)}
        </div>
      );
    }
    // A blank line keeps its height (so paragraph spacing survives).
    return <div key={key}>{line ? inlineMarkup(line, key) : " "}</div>;
  });
}

/**
 * The scrolling script surface (CAP-N58). Reads the shared state and animates
 * the scroll LOCALLY between control changes (rAF), resyncing to `offset`
 * whenever the state updates — so the dock preview, the projector, and the LAN
 * panel stay in step without high-frequency polling.
 */
export function TeleprompterScroller({
  state,
  fullscreen = false,
}: {
  state: TeleprompterState;
  fullscreen?: boolean;
}) {
  const t = useT();
  const trackRef = useRef<HTMLDivElement>(null);
  // The animation anchor: the last known offset + when we received it. `t` is
  // filled by the effect below (calling performance.now() during render is
  // impure); until then playing is false or the effect has already run.
  const anchor = useRef({
    offset: state.offset,
    t: 0,
    playing: state.playing,
    speed: state.speed,
  });
  useEffect(() => {
    anchor.current = {
      offset: state.offset,
      t: performance.now(),
      playing: state.playing,
      speed: state.speed,
    };
  }, [state.offset, state.playing, state.speed]);

  const px = fullscreen ? state.fontSize : PREVIEW_FONT;
  const lineHeight = px * LINE_HEIGHT;

  useEffect(() => {
    const write = () => {
      const a = anchor.current;
      const live = a.offset + (a.playing ? (a.speed * (performance.now() - a.t)) / 1000 : 0);
      if (trackRef.current) {
        trackRef.current.style.transform = `translateY(${-live * lineHeight}px)`;
      }
    };
    // While playing, animate every frame; while paused the offset is constant,
    // so write it once and stop scheduling (no idle 60 fps style churn).
    if (!state.playing) {
      write();
      return;
    }
    let raf = 0;
    const step = () => {
      write();
      raf = requestAnimationFrame(step);
    };
    raf = requestAnimationFrame(step);
    return () => cancelAnimationFrame(raf);
  }, [lineHeight, state.playing, state.offset]);

  // The beam-splitter mirror is only meaningful on the reading surface.
  const mirrored = fullscreen && state.mirror;
  // Re-parse the script only when its text changes (not on every speed/font tick).
  const body = useMemo(() => renderScript(state.script), [state.script]);

  return (
    <div
      className="relative h-full w-full overflow-hidden bg-black text-white"
      style={{ transform: mirrored ? "scaleX(-1)" : undefined }}
    >
      {state.script.trim() ? (
        <div
          ref={trackRef}
          className="px-[8%] will-change-transform"
          style={{
            fontSize: px,
            lineHeight: LINE_HEIGHT,
            paddingTop: fullscreen ? "42vh" : "0.5em",
            fontWeight: 500,
          }}
        >
          {body}
        </div>
      ) : (
        <div className="flex h-full items-center justify-center p-6 text-center text-sm text-white/50">
          {t("teleprompter-empty")}
        </div>
      )}
      {/* Reading line — where the talent's eyes rest (projector only). */}
      {fullscreen && (
        <div
          className="pointer-events-none absolute inset-x-0 border-t border-havoc-accent/50"
          style={{ top: "42vh" }}
          aria-hidden="true"
        />
      )}
    </div>
  );
}

/** The fullscreen teleprompter projector (CAP-N58): the talent's reading
 * surface — big, mirror-able, scrolled by the shared state. Esc closes it. */
export function TeleprompterProjector() {
  const t = useT();
  const state = useTeleprompter();
  const [hint, setHint] = useState(true);
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
    <div className="fixed inset-0 bg-black">
      {state && <TeleprompterScroller state={state} fullscreen />}
      {hint && (
        <div className="pointer-events-none absolute bottom-4 left-1/2 -translate-x-1/2 rounded bg-black/60 px-2 py-1 text-xs text-white/70">
          {t("projector-exit-hint")}
        </div>
      )}
    </div>
  );
}

const sliderClass = "w-full accent-havoc-accent";
const ctrlButton =
  "flex h-8 items-center justify-center rounded-md border border-havoc-border px-2 text-xs font-medium text-havoc-text hover:border-havoc-accent";

/** The teleprompter operator panel (CAP-N58): edit the script, control the
 * scroll, adjust speed/font/mirror, and open the fullscreen projector. */
export function TeleprompterDialog({ onClose }: { onClose: () => void }) {
  const t = useT();
  const state = useTeleprompter();

  const control = (action: string, value?: number) =>
    void teleprompterControl(action, value).catch(() => undefined);

  // Stop = halt the scroll AND rewind to the top, so the operator can re-read
  // and re-edit the script. (Pause alone leaves a short script scrolled off the
  // top, which looks blank and is why "there's no way to re-edit" — the text is
  // still there, just scrolled past.)
  const stop = () => {
    if (state?.playing) control("toggle");
    control("top");
  };

  return (
    <PickerShell title={t("teleprompter-title")} onClose={onClose} wide>
      {!state ? (
        <p className="p-4 text-sm text-havoc-muted">{t("teleprompter-loading")}</p>
      ) : (
        <div className="grid gap-4 p-4 md:grid-cols-2">
          <div className="flex flex-col gap-3">
            <label className="text-xs font-medium text-havoc-muted" htmlFor="tp-script">
              {t("teleprompter-script")}
            </label>
            <textarea
              id="tp-script"
              value={state.script}
              onChange={(e) => void teleprompterSetScript(e.target.value).catch(() => undefined)}
              placeholder={t("teleprompter-script-placeholder")}
              spellCheck={false}
              className="h-56 w-full resize-none rounded-md border border-havoc-border bg-havoc-panel p-2 font-mono text-xs text-havoc-text outline-none focus:border-havoc-accent/60"
            />
            <div className="grid grid-cols-5 gap-1.5">
              <button type="button" className={ctrlButton} onClick={() => control("top")}>
                ⟲ {t("teleprompter-top")}
              </button>
              <button type="button" className={ctrlButton} onClick={() => control("slower")}>
                – {t("teleprompter-slower")}
              </button>
              <button
                type="button"
                className={`${ctrlButton} ${state.playing ? "border-havoc-accent text-havoc-accent" : ""}`}
                onClick={() => control("toggle")}
              >
                {state.playing ? `⏸ ${t("teleprompter-pause")}` : `▶ ${t("teleprompter-play")}`}
              </button>
              <button type="button" className={ctrlButton} onClick={stop}>
                ■ {t("teleprompter-stop")}
              </button>
              <button type="button" className={ctrlButton} onClick={() => control("faster")}>
                + {t("teleprompter-faster")}
              </button>
            </div>
            <div className="flex flex-col gap-2">
              <label className="flex items-center justify-between text-xs text-havoc-muted">
                <span>{t("teleprompter-speed")}</span>
                <span className="font-mono">{state.speed.toFixed(1)}</span>
              </label>
              <input
                type="range"
                min={0.2}
                max={20}
                step={0.1}
                value={state.speed}
                onChange={(e) =>
                  void teleprompterSetSpeed(Number(e.target.value)).catch(() => undefined)
                }
                className={sliderClass}
              />
              <label className="flex items-center justify-between text-xs text-havoc-muted">
                <span>{t("teleprompter-font")}</span>
                <span className="font-mono">{Math.round(state.fontSize)}px</span>
              </label>
              <input
                type="range"
                min={24}
                max={160}
                step={2}
                value={state.fontSize}
                onChange={(e) =>
                  void teleprompterSetFont(Number(e.target.value)).catch(() => undefined)
                }
                className={sliderClass}
              />
              <label className="flex items-center gap-2 text-xs text-havoc-text">
                <input
                  type="checkbox"
                  checked={state.mirror}
                  onChange={(e) =>
                    void teleprompterSetMirror(e.target.checked).catch(() => undefined)
                  }
                  className="accent-havoc-accent"
                />
                {t("teleprompter-mirror")}
              </label>
              <button
                type="button"
                className={ctrlButton}
                onClick={() =>
                  // A normal decorated, always-on-top window (not a borderless
                  // "fullscreen" one): the borderless no-display window had no
                  // titlebar X, no way to close, and could render blank. The
                  // talent can maximize/move it to a second screen; Esc closes
                  // it too (the projector's own key handler).
                  void auxWindowOpen(
                    "projector-teleprompter",
                    t("teleprompter-title"),
                    null,
                    false,
                  ).catch(() => undefined)
                }
              >
                ⧉ {t("teleprompter-open-projector")}
              </button>
            </div>
          </div>
          <div className="flex flex-col gap-2">
            <span className="text-xs font-medium text-havoc-muted">
              {t("teleprompter-preview")}
            </span>
            <div className="h-72 overflow-hidden rounded-md border border-havoc-border">
              <TeleprompterScroller state={state} />
            </div>
            <p className="text-[11px] text-havoc-muted">{t("teleprompter-remote-hint")}</p>
          </div>
        </div>
      )}
    </PickerShell>
  );
}
