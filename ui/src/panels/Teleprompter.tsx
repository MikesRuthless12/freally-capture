import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

import {
  auxWindowOpen,
  teleprompterControl,
  teleprompterSetCaesura,
  teleprompterSetCountdown,
  teleprompterSetFont,
  teleprompterSetMirror,
  teleprompterSetScript,
  teleprompterSetSpeed,
} from "../api/commands";
import type { TeleprompterState } from "../api/types";
import { CaesuraEditor } from "../components/CaesuraEditor";
import { PickerShell } from "../components/PickerShell";
import {
  PauseIcon,
  PlayIcon,
  RestartIcon,
  StepBackIcon,
  StepForwardIcon,
  StopIcon,
} from "../components/TeleprompterIcons";
import { getLocale, useT } from "../i18n/t";
import {
  CAESURA_DEFAULT_SECS,
  type Caesura,
  liveOffset,
  parseCaesuras,
  timeAtOffset,
  visibleChars,
} from "../lib/caesura";
import { useTeleprompter } from "../lib/useTeleprompter";
import { readAloud, stopReading } from "../lib/tts";

/** Format seconds as `M:SS` for the read-time and seek displays. */
function fmtTime(seconds: number): string {
  const s = Math.max(0, Math.round(seconds));
  return `${Math.floor(s / 60)}:${(s % 60).toString().padStart(2, "0")}`;
}

/** Line-height multiple used everywhere, so a line-based offset maps to pixels
 * the same way on every surface (bigger font → bigger pixels, same reading pace). */
const LINE_HEIGHT = 1.5;

/** The preview and the projector lay text out on a shared fixed-width virtual
 * "stage", then CSS-scale it to their own window — so wrapping is IDENTICAL on
 * both surfaces regardless of each window's monitor. A fixed constant (not
 * `window.screen.width`, which differs per monitor on a dual-screen rig) keeps
 * that invariant. */
const STAGE_WIDTH = 1920;

/** Render a script line as one `data-ch` span per character (no Markdown) so the
 * per-character pace cue aligns 1:1 with the Rust visible-char offset. */
function lineChars(line: string, keyBase: string): React.ReactNode[] {
  // EVERY character is its own data-ch span (no markdown, no word grouping) so
  // the char-based pace cue aligns with the Rust visible-char offset. Wrapping is
  // the container's job (`break-words` / overflow-wrap): normal text breaks at
  // spaces, and a long UNBROKEN string breaks wherever it would overflow.
  return Array.from(line).map((ch, ci) => (
    <span key={`${keyBase}-${ci}`} data-ch="">
      {ch}
    </span>
  ));
}

function renderScript(script: string): React.ReactNode {
  return script.split("\n").map((line, index) => {
    const key = `l${index}`;
    const chars = lineChars(line, key);
    // A blank line keeps its height (so paragraph spacing survives).
    return <div key={key}>{chars.length ? chars : <span> </span>}</div>;
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
  onSeek,
  overrideOffset,
}: {
  state: TeleprompterState;
  fullscreen?: boolean;
  /** Click-to-start: seek to the clicked line + word (offset in lines). When
   * set, the surface is clickable. */
  onSeek?: (offset: number) => void;
  /** Preview-local highlight offset (read-aloud): when set, the surface shows
   * THIS visible-char offset instead of the shared scroll state, and does not
   * auto-animate — used so the modal can drive the highlight from speech without
   * touching the projector. */
  overrideOffset?: number;
}) {
  const t = useT();
  const trackRef = useRef<HTMLDivElement>(null);
  // Inline ` -- ` caesuras drive the same flat-crawl pauses the Rust state uses.
  const caesuras = useMemo(
    () => parseCaesuras(state.script, state.caesuraSecs),
    [state.script, state.caesuraSecs],
  );
  // The animation anchor: the last known offset + when we received it. `t` is
  // filled by the effect below (calling performance.now() during render is
  // impure); until then playing is false or the effect has already run.
  const anchor = useRef({
    offset: state.offset,
    t: 0,
    playing: state.playing,
    speed: state.speed,
    countdown: state.countdownRemaining,
  });
  useEffect(() => {
    anchor.current = {
      offset: state.offset,
      t: performance.now(),
      playing: state.playing,
      speed: state.speed,
      countdown: state.countdownRemaining,
    };
  }, [state.offset, state.playing, state.speed, state.countdownRemaining]);

  // Preview AND projector lay the script out on ONE fixed-width "stage" (the
  // screen width) at the real font size, then CSS-scale the whole stage to fit
  // their own window. Same stage width + same font => IDENTICAL wrapping, so the
  // preview is a faithful, smaller copy of the projector no matter the window
  // sizes. `dims` is the live viewport (measured below); `scale` fits the stage.
  const [dims, setDims] = useState({ w: 0, h: 0 });
  const stageW = STAGE_WIDTH;
  const scale = dims.w > 0 ? dims.w / stageW : 0.3;
  // The live scroll offset (visible chars) each frame, so click/wheel seeks
  // relative to it; and how many characters are currently lit (delta updates).
  const liveRef = useRef(state.offset);
  const litCountRef = useRef(0);
  // The pre-roll countdown overlay (big number), driven imperatively each frame.
  const countdownRef = useRef<HTMLDivElement>(null);
  // The beam-splitter mirror is only meaningful on the reading surface.
  const mirrored = fullscreen && state.mirror;

  useEffect(() => {
    const track = trackRef.current;
    if (!track) return;
    // Cache the character spans + content height for this script/font (the effect
    // re-runs when they change); start from a clean unlit slate.
    const chars = track.querySelectorAll<HTMLElement>("[data-ch]");
    const total = chars.length;
    for (let i = 0; i < total; i++) chars[i].style.color = "";
    litCountRef.current = 0;
    // Reading guide at 12% down the viewport, expressed in the stage's own
    // coordinate system (the stage transform then scales it to the window).
    const stageH = scale > 0 ? dims.h / scale : dims.h;
    const padTop = 0.12 * stageH;

    const write = () => {
      const a = anchor.current;
      const raw = a.playing ? (performance.now() - a.t) / 1000 : 0;
      // A start-countdown pre-roll holds the scroll (and shows a big number) for
      // its first `countdown` seconds; the shared clock ignores that lead-in, so
      // the preview and the projector count down and start together.
      const cd = Math.max(0, a.countdown - raw);
      const elapsed = Math.max(0, raw - a.countdown);
      if (countdownRef.current) {
        const show = overrideOffset === undefined && cd > 0;
        countdownRef.current.style.display = show ? "flex" : "none";
        if (show) countdownRef.current.textContent = String(Math.ceil(cd));
      }
      // Read-aloud drives the highlight locally via `overrideOffset`; otherwise
      // it's the shared, time-animated scroll offset.
      const live =
        overrideOffset !== undefined
          ? Math.max(0, Math.min(total, overrideOffset))
          : Math.max(0, Math.min(total, liveOffset(a.offset, elapsed, a.speed, caesuras)));
      liveRef.current = live;
      // Put the CURRENT character's row exactly at the reading guide (measured
      // from its span, interpolated across the row jump) so the highlighted word
      // is ALWAYS on the guide and never scrolls out of view when you seek/scroll
      // fast; at the end the last line sits at the guide, fully lit.
      const i0 = Math.max(0, Math.min(total - 1, Math.floor(live)));
      const y0 = total > 0 ? (chars[i0] as HTMLElement).offsetTop - padTop : 0;
      const y1 = i0 + 1 < total ? (chars[i0 + 1] as HTMLElement).offsetTop - padTop : y0;
      const y = y0 + (live - Math.floor(live)) * (y1 - y0);
      track.style.transform = `translateY(${-y}px)`;
      // Highlight the first round(live) characters GLOBALLY in reading order,
      // updating only the delta from the previous frame.
      const litCount = Math.max(0, Math.min(total, Math.round(live)));
      const from = litCountRef.current;
      if (litCount > from) {
        for (let i = from; i < litCount; i++) chars[i].style.color = "var(--color-havoc-accent)";
      } else if (litCount < from) {
        for (let i = litCount; i < from; i++) chars[i].style.color = "";
      }
      litCountRef.current = litCount;
    };
    // Override mode (read-aloud) writes once per offset change; only the shared,
    // playing scroll animates every frame.
    const animating = overrideOffset === undefined && state.playing;
    if (!animating) {
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
  }, [
    state.playing,
    state.offset,
    state.countdownRemaining,
    caesuras,
    dims.w,
    dims.h,
    scale,
    state.fontSize,
    state.script,
    overrideOffset,
  ]);

  // Re-parse the script only when its text changes (not on every speed/font tick).
  const body = useMemo(() => renderScript(state.script), [state.script]);

  // Click-to-start: map a click to `line + word` (offset) and seek there, so a
  // side note at the top can be skipped by clicking the line/word to begin on.
  const handleClick = (e: React.MouseEvent<HTMLDivElement>) => {
    if (!onSeek || !state.script.trim()) return;
    const track = trackRef.current;
    if (!track) return;
    const all = Array.from(track.querySelectorAll<HTMLElement>("[data-ch]"));
    // Clicking a character seeks right to it; a click in a gap estimates from row.
    const span = (e.target as HTMLElement).closest<HTMLElement>("[data-ch]");
    if (span) {
      const idx = all.indexOf(span);
      if (idx >= 0) onSeek(idx);
      return;
    }
    // Gap click (not on a character): estimate the char from the click's Y in
    // stage coordinates (getBoundingClientRect is post-transform, so undo scale).
    const rect = track.getBoundingClientRect();
    const stageY = (e.clientY - rect.top) / Math.max(scale, 0.0001);
    const h = Math.max(1, track.scrollHeight);
    onSeek(Math.max(0, Math.min(all.length, (stageY / h) * all.length)));
  };

  // Mousewheel scrubbing (preview + projector): a native non-passive listener so
  // preventDefault stops the modal scrolling and stopPropagation stops the
  // projector root's wheel handler double-firing. ~1 line per notch.
  const rootRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const el = rootRef.current;
    if (!el || !onSeek) return;
    const onWheel = (e: WheelEvent) => {
      e.preventDefault();
      e.stopPropagation();
      onSeek(Math.max(0, liveRef.current + e.deltaY / 8));
    };
    el.addEventListener("wheel", onWheel, { passive: false });
    return () => el.removeEventListener("wheel", onWheel);
  }, [onSeek]);

  // Measure the viewport (BOTH views) so the stage can scale to fit it.
  useEffect(() => {
    const el = rootRef.current;
    if (!el) return;
    const measure = () => setDims({ w: el.clientWidth, h: el.clientHeight });
    measure();
    const ro = new ResizeObserver(measure);
    ro.observe(el);
    return () => ro.disconnect();
  }, []);

  return (
    <div
      ref={rootRef}
      className="relative h-full w-full overflow-hidden bg-black text-white"
      style={{
        transform: mirrored ? "scaleX(-1)" : undefined,
        cursor: onSeek ? "pointer" : undefined,
      }}
      onClick={onSeek ? handleClick : undefined}
    >
      {state.script.trim() ? (
        <div
          className="absolute left-0 top-0 origin-top-left"
          style={{ width: stageW, transform: `scale(${scale})` }}
        >
          <div
            ref={trackRef}
            className="px-[8%] break-words will-change-transform"
            style={{
              fontSize: state.fontSize,
              lineHeight: LINE_HEIGHT,
              fontWeight: 500,
            }}
          >
            {body}
          </div>
        </div>
      ) : (
        <div className="flex h-full items-center justify-center p-6 text-center text-sm text-white/50">
          {t("teleprompter-empty")}
        </div>
      )}
      {/* Start-countdown pre-roll — a big number over the script until scrolling
          begins; updated imperatively by the frame loop above. */}
      <div
        ref={countdownRef}
        aria-hidden="true"
        className="pointer-events-none absolute inset-0 items-center justify-center font-bold text-havoc-accent"
        style={{
          display: "none",
          fontSize: Math.max(24, Math.min(dims.w || 1, dims.h || 1) * 0.4),
          textShadow: "0 2px 24px rgba(0,0,0,0.6)",
        }}
      />
      {/* Reading line — where the talent's eyes rest (projector only). */}
      {fullscreen && (
        <div
          className="pointer-events-none absolute inset-x-0 border-t border-havoc-accent/50"
          style={{ top: "12%" }}
          aria-hidden="true"
        />
      )}
    </div>
  );
}

/** A projector control button (readable on black; symbol + accessible label). */
const projBtn =
  "flex h-11 w-11 items-center justify-center rounded-md bg-white/10 text-lg text-white transition-colors hover:bg-white/25 focus:outline-none focus-visible:ring-2 focus-visible:ring-white/60";

/** A YouTube-style seek bar for the projector: a scrubber with elapsed / total
 * read time (caesura pauses counted), hover-to-preview the words at any point,
 * and click / drag to jump there. `total` is in scroll lines; times come from
 * the shared caesura-aware timing so the numbers move with speed and seeks. */
function TeleprompterSeekBar({
  state,
  caesuras,
  onSeek,
  overrideOffset,
}: {
  state: TeleprompterState;
  caesuras: Caesura[];
  onSeek: (offset: number) => void;
  /** Read-aloud: track this offset instead of the shared scroll (so the scrubber
   * shows where the reading is, not where the shared scroll is). */
  overrideOffset?: number;
}) {
  const total = Math.max(1, visibleChars(state.script));
  const speed = state.speed > 0 ? state.speed : 1;
  const vis = useMemo(
    () => Array.from(state.script).filter((c) => c.charCodeAt(0) !== 10),
    [state.script],
  );
  const trackRef = useRef<HTMLDivElement>(null);
  const anchor = useRef({
    offset: state.offset,
    t: 0,
    playing: state.playing,
    speed: state.speed,
    countdown: state.countdownRemaining,
  });
  const [live, setLive] = useState(state.offset);
  const [hoverFrac, setHoverFrac] = useState<number | null>(null);
  const dragging = useRef(false);

  useEffect(() => {
    anchor.current = {
      offset: state.offset,
      t: performance.now(),
      playing: state.playing,
      speed: state.speed,
      countdown: state.countdownRemaining,
    };
  }, [state.offset, state.playing, state.speed, state.countdownRemaining]);

  // Advance the scrubber smoothly while playing; a single write while paused.
  // In read-aloud mode the scrubber just follows the reading offset (no anim).
  useEffect(() => {
    let raf = 0;
    if (overrideOffset !== undefined) {
      raf = requestAnimationFrame(() => setLive(overrideOffset));
      return () => cancelAnimationFrame(raf);
    }
    // Drive the scrubber from the animation frame (never a synchronous setState in
    // the effect body): one write when paused, a loop while playing.
    const run = () => {
      const a = anchor.current;
      // Honour the start-countdown pre-roll: the scrubber holds until it elapses.
      const elapsed = Math.max(0, (a.playing ? (performance.now() - a.t) / 1000 : 0) - a.countdown);
      // Clamp at the end (liveOffset itself is unbounded) so the elapsed read-time
      // never climbs past the total once the scroll has reached the last line.
      setLive(Math.min(total, liveOffset(a.offset, elapsed, a.speed, caesuras)));
      if (state.playing) raf = requestAnimationFrame(run);
    };
    raf = requestAnimationFrame(run);
    return () => cancelAnimationFrame(raf);
  }, [
    state.playing,
    state.offset,
    state.speed,
    state.countdownRemaining,
    caesuras,
    overrideOffset,
    total,
  ]);

  const fracFromX = (clientX: number) => {
    const rect = trackRef.current?.getBoundingClientRect();
    if (!rect) return 0;
    return Math.max(0, Math.min(1, (clientX - rect.left) / Math.max(1, rect.width)));
  };
  const seekFrac = (frac: number) => onSeek(frac * total);

  const progress = Math.max(0, Math.min(1, live / total));
  // At the hovered seek time, show ~80 characters around that point with the
  // highlight up to it (a "what would be lit here at this time" preview).
  const hoverChar = hoverFrac === null ? 0 : Math.floor(hoverFrac * total);
  const snipStart = Math.max(0, hoverChar - 40);
  const snippet = hoverFrac === null ? "" : vis.slice(snipStart, snipStart + 80).join("");
  const snipLit = Math.max(0, Math.min(snippet.length, hoverChar - snipStart));

  return (
    <div className="flex items-center gap-3 text-white">
      <span className="w-11 shrink-0 text-right font-mono text-xs tabular-nums text-white/70">
        {fmtTime(timeAtOffset(live, speed, caesuras))}
      </span>
      <div className="relative flex-1">
        {hoverFrac !== null && (
          <div
            className="pointer-events-none absolute bottom-full mb-3 w-64 max-w-[70vw] -translate-x-1/2 rounded-md border border-white/15 bg-black/90 p-2 text-left shadow-lg"
            style={{ left: `${Math.max(6, Math.min(94, hoverFrac * 100))}%` }}
          >
            <div className="mb-1 font-mono text-[11px] text-havoc-accent">
              {fmtTime(timeAtOffset(hoverFrac * total, speed, caesuras))}
            </div>
            <div className="text-[11px] leading-snug break-words text-white/80">
              <span style={{ color: "var(--color-havoc-accent)" }}>
                {snippet.slice(0, snipLit)}
              </span>
              <span>{snippet.slice(snipLit)}</span>
            </div>
          </div>
        )}
        <div
          ref={trackRef}
          className="relative h-2.5 cursor-pointer rounded-full bg-white/20"
          onPointerDown={(e) => {
            dragging.current = true;
            e.currentTarget.setPointerCapture(e.pointerId);
            seekFrac(fracFromX(e.clientX));
          }}
          onPointerMove={(e) => {
            const frac = fracFromX(e.clientX);
            setHoverFrac(frac);
            if (dragging.current) seekFrac(frac);
          }}
          onPointerUp={(e) => {
            dragging.current = false;
            e.currentTarget.releasePointerCapture(e.pointerId);
          }}
          onPointerLeave={() => {
            if (!dragging.current) setHoverFrac(null);
          }}
        >
          <div
            className="absolute inset-y-0 left-0 rounded-full bg-havoc-accent"
            style={{ width: `${progress * 100}%` }}
          />
          <div
            className="absolute top-1/2 h-4 w-4 -translate-x-1/2 -translate-y-1/2 rounded-full bg-havoc-accent shadow"
            style={{ left: `${progress * 100}%` }}
          />
        </div>
      </div>
      <span className="w-11 shrink-0 font-mono text-xs tabular-nums text-white/70">
        {fmtTime(timeAtOffset(total, speed, caesuras))}
      </span>
    </div>
  );
}

/** The fullscreen teleprompter projector (CAP-N58): the talent's reading
 * surface — big, mirror-able, scrolled by the shared state. A control bar
 * (play/pause/stop, step, speed) fades in on activity so the talent can drive
 * the scroll from the projector itself; Esc (or the ✕) closes it. */
export function TeleprompterProjector() {
  const t = useT();
  const state = useTeleprompter();
  // The chrome (controls + hints) fades away while reading and returns on any
  // mouse/keyboard activity, so it never sits over the script during a take.
  const [chromeVisible, setChromeVisible] = useState(true);
  const hideTimer = useRef<number | null>(null);
  // Inline caesuras + a live anchor so the mousewheel and the seek bar can jump
  // to an absolute scroll position.
  const caesuras = useMemo(
    () => parseCaesuras(state?.script ?? "", state?.caesuraSecs ?? CAESURA_DEFAULT_SECS),
    [state?.script, state?.caesuraSecs],
  );
  const wheelAnchor = useRef({ offset: 0, t: 0, playing: false, speed: 2, countdown: 0 });
  useEffect(() => {
    if (!state) return;
    wheelAnchor.current = {
      offset: state.offset,
      t: performance.now(),
      playing: state.playing,
      speed: state.speed,
      countdown: state.countdownRemaining,
    };
  }, [state?.offset, state?.playing, state?.speed, state?.countdownRemaining]);

  const control = (action: string, value?: number) =>
    void teleprompterControl(action, value).catch(() => undefined);
  const seek = (offset: number) => control("seek", Math.max(0, offset));
  // Stop = halt the scroll AND rewind to the top (mirrors the operator panel).
  const stop = () => {
    if (state?.playing) control("toggle");
    control("top");
  };

  const reveal = useCallback(() => {
    setChromeVisible(true);
    if (hideTimer.current) window.clearTimeout(hideTimer.current);
    hideTimer.current = window.setTimeout(() => setChromeVisible(false), 2800);
  }, []);

  // Hold a step button to keep rewinding / fast-forwarding, accelerating the
  // longer it's held (the highlight follows each step). Release stops it; a quick
  // tap does one step (the immediate control call).
  const holdTimer = useRef<number | null>(null);
  const stopHold = () => {
    if (holdTimer.current) {
      window.clearInterval(holdTimer.current);
      holdTimer.current = null;
    }
  };
  const startHold = (action: "stepBack" | "stepForward") => {
    reveal();
    control(action);
    let step = 0.5;
    holdTimer.current = window.setInterval(() => {
      control(action, step);
      step = Math.min(step * 1.25, 6);
    }, 80);
  };

  // Mousewheel scrubbing: ~1 line per notch, forward (wheel down) advances.
  const onWheel = (e: React.WheelEvent) => {
    reveal();
    const a = wheelAnchor.current;
    const elapsed = Math.max(0, (a.playing ? (performance.now() - a.t) / 1000 : 0) - a.countdown);
    seek(liveOffset(a.offset, elapsed, a.speed, caesuras) + e.deltaY / 8);
  };

  useEffect(() => {
    // Defer the first reveal to a frame so we don't setState synchronously in the
    // effect body; the chrome starts visible anyway and this arms the auto-hide.
    const raf = requestAnimationFrame(reveal);
    const onKey = (event: KeyboardEvent) => {
      reveal();
      const ctrl = (action: string) => void teleprompterControl(action).catch(() => undefined);
      switch (event.key) {
        case "Escape":
          void getCurrentWindow().close();
          break;
        case " ": // Space toggles play/pause without scrolling the page.
          event.preventDefault();
          ctrl("toggle");
          break;
        case "ArrowLeft":
          ctrl("stepBack");
          break;
        case "ArrowRight":
          ctrl("stepForward");
          break;
        case "ArrowUp":
          ctrl("faster");
          break;
        case "ArrowDown":
          ctrl("slower");
          break;
        case "Home":
          ctrl("top");
          break;
        default:
          break;
      }
    };
    window.addEventListener("keydown", onKey);
    window.addEventListener("mousemove", reveal);
    return () => {
      cancelAnimationFrame(raf);
      window.removeEventListener("keydown", onKey);
      window.removeEventListener("mousemove", reveal);
      if (hideTimer.current) window.clearTimeout(hideTimer.current);
      if (holdTimer.current) window.clearInterval(holdTimer.current);
    };
  }, [reveal]);

  return (
    <div
      className="fixed inset-0 bg-black"
      style={{ cursor: chromeVisible ? "default" : "none" }}
      onWheel={onWheel}
    >
      {state && <TeleprompterScroller state={state} fullscreen onSeek={seek} />}
      {/* Chrome (seek bar + controls) — fades while reading, back on activity. */}
      <div
        className={`absolute inset-x-0 bottom-0 flex flex-col gap-3 bg-gradient-to-t from-black/90 to-transparent px-6 pb-5 pt-16 transition-opacity duration-300 ${
          chromeVisible ? "opacity-100" : "pointer-events-none opacity-0"
        }`}
      >
        {state && <TeleprompterSeekBar state={state} caesuras={caesuras} onSeek={seek} />}
        <div className="flex items-center justify-center gap-2">
          <button
            type="button"
            className={projBtn}
            onClick={() => control("top")}
            title={t("teleprompter-top")}
            aria-label={t("teleprompter-top")}
          >
            <RestartIcon className="h-5 w-5" />
          </button>
          <button
            type="button"
            className={projBtn}
            onPointerDown={() => startHold("stepBack")}
            onPointerUp={stopHold}
            onPointerLeave={stopHold}
            title={t("teleprompter-step-back")}
            aria-label={t("teleprompter-step-back")}
          >
            <StepBackIcon className="h-5 w-5" />
          </button>
          <button
            type="button"
            className={projBtn}
            onClick={() => control("slower")}
            title={t("teleprompter-slower")}
            aria-label={t("teleprompter-slower")}
          >
            –
          </button>
          <button
            type="button"
            className={`${projBtn} ${state?.playing ? "bg-white/25" : ""}`}
            onClick={() => control("toggle")}
            title={state?.playing ? t("teleprompter-pause") : t("teleprompter-play")}
            aria-label={state?.playing ? t("teleprompter-pause") : t("teleprompter-play")}
          >
            {state?.playing ? <PauseIcon className="h-5 w-5" /> : <PlayIcon className="h-5 w-5" />}
          </button>
          <button
            type="button"
            className={projBtn}
            onClick={stop}
            title={t("teleprompter-stop")}
            aria-label={t("teleprompter-stop")}
          >
            <StopIcon className="h-5 w-5" />
          </button>
          <button
            type="button"
            className={projBtn}
            onClick={() => control("faster")}
            title={t("teleprompter-faster")}
            aria-label={t("teleprompter-faster")}
          >
            +
          </button>
          <button
            type="button"
            className={projBtn}
            onPointerDown={() => startHold("stepForward")}
            onPointerUp={stopHold}
            onPointerLeave={stopHold}
            title={t("teleprompter-step-forward")}
            aria-label={t("teleprompter-step-forward")}
          >
            <StepForwardIcon className="h-5 w-5" />
          </button>
          {state && (
            <span
              className="ml-2 w-14 text-center font-mono text-xs text-white/60"
              aria-hidden="true"
            >
              {state.speed.toFixed(1)}×
            </span>
          )}
        </div>
      </div>
      {chromeVisible && (
        <div className="pointer-events-none absolute right-3 top-3 rounded bg-black/60 px-2 py-1 text-xs text-white/60">
          {t("projector-exit-hint")}
        </div>
      )}
    </div>
  );
}

const sliderClass = "w-full accent-havoc-accent";
const ctrlButton =
  "flex h-8 items-center justify-center rounded-md border border-havoc-border px-2 text-xs font-medium text-havoc-text hover:border-havoc-accent";

/** BPM ↔ chars/sec. The scroll speed stays chars/sec-authoritative (shared with
 * the projector/LAN); BPM is just an alternate display/entry mode for musical
 * pacing (any singing style). One beat ≈ one short word or syllable — a sane
 * default that keeps the 50–250 BPM range inside the 1–60 chars/sec speed range. */
const CHARS_PER_BEAT = 3.5;
const BPM_MIN = 50;
const BPM_MAX = 250;
const bpmFromSpeed = (charsPerSec: number) => Math.round((charsPerSec * 60) / CHARS_PER_BEAT);
const speedFromBpm = (bpm: number) => (bpm * CHARS_PER_BEAT) / 60;

/** The teleprompter operator panel (CAP-N58): edit the script, control the
 * scroll, adjust speed/font/mirror, and open the fullscreen projector. */
export function TeleprompterDialog({ onClose }: { onClose: () => void }) {
  const t = useT();
  const state = useTeleprompter();
  const caesuras = useMemo(
    () => parseCaesuras(state?.script ?? "", state?.caesuraSecs ?? CAESURA_DEFAULT_SECS),
    [state?.script, state?.caesuraSecs],
  );
  // Estimated time to read the whole script at the current pace, caesura pauses
  // counted — moves live as the speed slider or the script changes.
  const estSecs = state
    ? timeAtOffset(
        Math.max(1, visibleChars(state.script)),
        state.speed > 0 ? state.speed : 1,
        caesuras,
      )
    : 0;

  // Speed shows/enters as chars/sec or BPM (musical pacing for rap / R&B) — an
  // operator-local display toggle over the same authoritative chars/sec speed.
  const [bpmMode, setBpmMode] = useState(false);
  const [bpmDraft, setBpmDraft] = useState<string | null>(null);
  const displayBpm = state
    ? Math.max(BPM_MIN, Math.min(BPM_MAX, bpmFromSpeed(state.speed)))
    : BPM_MIN;
  // Typed BPM entry: clamp to [50, 250] on commit so no out-of-range value lands.
  const commitBpm = (raw: string) => {
    const n = Number.parseInt(raw, 10);
    if (Number.isFinite(n)) {
      const clamped = Math.max(BPM_MIN, Math.min(BPM_MAX, n));
      void teleprompterSetSpeed(speedFromBpm(clamped)).catch(() => undefined);
    }
    setBpmDraft(null);
  };

  // "Read aloud with per-OS speech synthesis" — a preview-only MODE (checkbox):
  // never the projector, never the shared scroll state. When on, the transport
  // (play/pause/stop) and the seek drive the speech, and the highlight follows
  // the spoken word via `raOffset`. `engaged` (play pressed, until stop/end)
  // disables the checkbox so it can't be flipped mid-speech.
  const [readAloudMode, setReadAloudMode] = useState(false);
  const [engaged, setEngaged] = useState(false);
  const [speaking, setSpeaking] = useState(false);
  const [raOffset, setRaOffsetState] = useState(0);
  const raOffsetRef = useRef(0);
  const setRaOffset = (o: number) => {
    raOffsetRef.current = o;
    setRaOffsetState(o);
  };
  const [seekNonce, setSeekNonce] = useState(0);

  // Start / restart speech when the read engages, the user seeks (nonce), or the
  // pace/script changes — debounced so scrubbing doesn't stutter. Progress drives
  // the local highlight; onEnd (finished or native-estimate) frees the checkbox.
  useEffect(() => {
    if (!readAloudMode || !speaking || !state) return;
    const id = window.setTimeout(() => {
      void readAloud(
        state.script,
        state.speed,
        () => {
          setSpeaking(false);
          setEngaged(false);
        },
        raOffsetRef.current,
        (off) => setRaOffset(off),
        caesuras,
      );
    }, 100);
    return () => window.clearTimeout(id);
  }, [
    readAloudMode,
    speaking,
    seekNonce,
    state?.speed,
    state?.script,
    state?.caesuraSecs,
    caesuras,
  ]);

  // Stop speech when the mode turns off or the dialog closes.
  useEffect(() => {
    if (readAloudMode) return;
    stopReading();
    // Defer the flag reset out of the effect body (avoids a synchronous setState).
    const raf = requestAnimationFrame(() => {
      setSpeaking(false);
      setEngaged(false);
    });
    return () => cancelAnimationFrame(raf);
  }, [readAloudMode]);
  useEffect(() => () => stopReading(), []);

  const raPlayPause = () => {
    // Parked at the very end (finished, or seeked there)? Play restarts from the
    // top automatically — no need to hit Stop/⟲ first.
    const atEnd = raOffsetRef.current >= (state ? visibleChars(state.script) : 0) - 0.5;
    if (!engaged) {
      if (atEnd) setRaOffset(0);
      setEngaged(true);
      setSpeaking(true);
    } else if (speaking) {
      stopReading();
      setSpeaking(false); // pause — offset stays; resume re-speaks from here
    } else {
      if (atEnd) setRaOffset(0); // resume from the top when parked at the end
      setSpeaking(true);
    }
  };
  const raStop = () => {
    stopReading();
    setSpeaking(false);
    setEngaged(false);
    setRaOffset(0);
  };
  // A click/seek/drag while reading: move the highlight and jump the speech there
  // (nonce forces a restart even to the same offset).
  const raSeek = (o: number) => {
    setRaOffset(o);
    setSeekNonce((n) => n + 1);
  };

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
    <PickerShell title={t("teleprompter-title")} onClose={onClose} large>
      {!state ? (
        <p className="p-4 text-sm text-havoc-muted">{t("teleprompter-loading")}</p>
      ) : (
        <div className="grid h-full gap-4 p-4 md:grid-cols-2">
          <div className="flex min-h-0 flex-col gap-3">
            <label id="tp-script-label" className="text-xs font-medium text-havoc-muted">
              {t("teleprompter-script")}
            </label>
            <CaesuraEditor
              value={state.script}
              onChange={(next) => void teleprompterSetScript(next).catch(() => undefined)}
              caesuraSecs={state.caesuraSecs}
              placeholder={t("teleprompter-script-placeholder")}
              ariaLabelledBy="tp-script-label"
              lang={getLocale()}
              className="h-full w-full overflow-y-auto rounded-md border border-havoc-border bg-havoc-panel p-2 font-mono text-sm text-havoc-text focus:border-havoc-accent/60"
            />
            <div className="flex items-center justify-between text-[11px] text-havoc-muted">
              <span>{t("teleprompter-caesura-hint")}</span>
              <span className="font-mono">
                {t("teleprompter-est-time")} {fmtTime(estSecs)}
              </span>
            </div>
            <div className="grid grid-cols-5 gap-1.5">
              <button
                type="button"
                className={`${ctrlButton} gap-1`}
                onClick={() => (readAloudMode ? raSeek(0) : control("top"))}
              >
                <RestartIcon className="h-3.5 w-3.5" /> {t("teleprompter-top")}
              </button>
              <button type="button" className={ctrlButton} onClick={() => control("slower")}>
                – {t("teleprompter-slower")}
              </button>
              <button
                type="button"
                className={`${ctrlButton} gap-1 ${(readAloudMode ? speaking : state.playing) ? "border-havoc-accent text-havoc-accent" : ""}`}
                onClick={() => (readAloudMode ? raPlayPause() : control("toggle"))}
              >
                {(readAloudMode ? speaking : state.playing) ? (
                  <>
                    <PauseIcon className="h-3.5 w-3.5" /> {t("teleprompter-pause")}
                  </>
                ) : (
                  <>
                    <PlayIcon className="h-3.5 w-3.5" /> {t("teleprompter-play")}
                  </>
                )}
              </button>
              <button
                type="button"
                className={`${ctrlButton} gap-1`}
                onClick={() => (readAloudMode ? raStop() : stop())}
              >
                <StopIcon className="h-3.5 w-3.5" /> {t("teleprompter-stop")}
              </button>
              <button type="button" className={ctrlButton} onClick={() => control("faster")}>
                + {t("teleprompter-faster")}
              </button>
            </div>
            <div className="flex flex-col gap-2">
              <label className="flex items-center justify-between text-xs text-havoc-muted">
                <span>{bpmMode ? t("teleprompter-speed-bpm") : t("teleprompter-speed")}</span>
                {bpmMode ? (
                  <input
                    type="number"
                    min={BPM_MIN}
                    max={BPM_MAX}
                    step={1}
                    value={bpmDraft ?? displayBpm}
                    onChange={(e) => {
                      const raw = e.target.value;
                      setBpmDraft(raw);
                      // Commit live when in range so the up/down arrows (and a
                      // valid typed value) take effect at once; anything typed
                      // out of range is clamped to [50, 250] on blur/Enter.
                      const n = Number(raw);
                      if (raw !== "" && Number.isFinite(n) && n >= BPM_MIN && n <= BPM_MAX) {
                        void teleprompterSetSpeed(speedFromBpm(n)).catch(() => undefined);
                      }
                    }}
                    onBlur={(e) => {
                      // Only commit an actual edit — a bare focus/blur must not
                      // down-convert a high chars/sec speed to the clamped BPM view.
                      if (bpmDraft !== null) commitBpm(e.target.value);
                    }}
                    onKeyDown={(e) => {
                      if (e.key === "Enter") {
                        commitBpm(e.currentTarget.value);
                        e.currentTarget.blur();
                      }
                    }}
                    aria-label={t("teleprompter-speed-bpm")}
                    className="w-20 rounded-md border border-havoc-border bg-havoc-panel px-2 py-0.5 text-center font-mono text-havoc-text outline-none focus:border-havoc-accent/60"
                  />
                ) : (
                  <span className="font-mono">{state.speed.toFixed(1)}</span>
                )}
              </label>
              {!bpmMode && (
                <input
                  type="range"
                  min={1}
                  max={60}
                  step={1}
                  value={state.speed}
                  onChange={(e) =>
                    void teleprompterSetSpeed(Number(e.target.value)).catch(() => undefined)
                  }
                  className={sliderClass}
                />
              )}
              <label className="flex items-center gap-2 text-xs text-havoc-text">
                <input
                  type="checkbox"
                  checked={bpmMode}
                  onChange={(e) => setBpmMode(e.target.checked)}
                  className="accent-havoc-accent"
                />
                {t("teleprompter-bpm-mode")}
              </label>
              <label className="flex items-center gap-2 text-xs text-havoc-text">
                <input
                  type="checkbox"
                  checked={readAloudMode}
                  disabled={engaged}
                  onChange={(e) => setReadAloudMode(e.target.checked)}
                  className="accent-havoc-accent disabled:opacity-40"
                />
                <span className={engaged ? "opacity-40" : undefined}>
                  🔊 {t("teleprompter-read-aloud")}
                </span>
              </label>
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
              <label className="flex items-center justify-between text-xs text-havoc-muted">
                <span>{t("teleprompter-caesura-pause")}</span>
                <span className="font-mono">{state.caesuraSecs.toFixed(2)}s</span>
              </label>
              <input
                type="range"
                min={0.75}
                max={2}
                step={0.05}
                value={state.caesuraSecs}
                onChange={(e) =>
                  void teleprompterSetCaesura(Number(e.target.value)).catch(() => undefined)
                }
                className={sliderClass}
              />
              <label className="flex items-center gap-2 text-xs text-havoc-text">
                <input
                  type="checkbox"
                  checked={state.countdownSecs > 0}
                  onChange={(e) =>
                    void teleprompterSetCountdown(e.target.checked ? 3 : 0).catch(() => undefined)
                  }
                  className="accent-havoc-accent"
                />
                {t("teleprompter-countdown")}
              </label>
              {state.countdownSecs > 0 && (
                <label className="flex items-center justify-between text-xs text-havoc-muted">
                  <span>{t("teleprompter-countdown-secs")}</span>
                  <input
                    type="number"
                    min={1}
                    max={10}
                    step={1}
                    value={Math.round(state.countdownSecs)}
                    onChange={(e) =>
                      void teleprompterSetCountdown(Number(e.target.value)).catch(() => undefined)
                    }
                    className="w-16 rounded-md border border-havoc-border bg-havoc-panel px-2 py-0.5 text-center font-mono text-havoc-text outline-none focus:border-havoc-accent/60"
                  />
                </label>
              )}
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
                  //
                  // Then close THIS (blocking) operator modal so the main window
                  // is free to run the livestream while the projector scrolls
                  // independently off the shared state — the talent reads from the
                  // projector mid-stream instead of being blocked by the preview.
                  void auxWindowOpen("projector-teleprompter", t("teleprompter-title"), null, false)
                    .then(() => {
                      // With a start countdown armed, roll the take on the
                      // projector automatically. PAUSE first so a scroll that was
                      // already playing (even parked at the very end) is stopped;
                      // then rewind to clear the highlight; then play from the top
                      // so resume() actually arms the countdown (it is a no-op while
                      // already playing). A short delay lets the projector webview
                      // mount so it shows the full countdown.
                      if (state.countdownSecs > 0) {
                        window.setTimeout(() => {
                          // Await in order: play's resume() only arms the countdown
                          // when base_offset<=0, which top must set first.
                          void (async () => {
                            await teleprompterControl("pause").catch(() => undefined);
                            await teleprompterControl("top").catch(() => undefined);
                            await teleprompterControl("play").catch(() => undefined);
                          })();
                        }, 600);
                      }
                      onClose();
                    })
                    .catch(() => undefined)
                }
              >
                ⧉ {t("teleprompter-open-projector")}
              </button>
            </div>
          </div>
          <div className="flex min-h-0 flex-col gap-2">
            <span className="text-xs font-medium text-havoc-muted">
              {t("teleprompter-preview")}
            </span>
            <div className="min-h-0 flex-1 overflow-hidden rounded-md border border-havoc-border">
              <TeleprompterScroller
                state={state}
                onSeek={(offset) => (readAloudMode ? raSeek(offset) : control("seek", offset))}
                overrideOffset={readAloudMode ? raOffset : undefined}
              />
            </div>
            <TeleprompterSeekBar
              state={state}
              caesuras={caesuras}
              onSeek={(offset) => (readAloudMode ? raSeek(offset) : control("seek", offset))}
              overrideOffset={readAloudMode ? raOffset : undefined}
            />
            <p className="text-[11px] text-havoc-muted">{t("teleprompter-remote-hint")}</p>
          </div>
        </div>
      )}
    </PickerShell>
  );
}
