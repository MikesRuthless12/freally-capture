# Browser Source — design spike (TASK-612, Phase 6)

**Status: spike only — Mike decides before anything is built.** The
roadmap moved the Browser source out of Phase 2 because Tauri v2 cannot
render a webview to a texture cross-platform; this document weighs the
three viable routes and recommends one.

## What a Browser source must do

Render an arbitrary URL offscreen at a set resolution/fps, hand the
compositor RGBA frames (with transparency), keep audio available to the
mixer, and never violate the charter (no accounts, no telemetry, nothing
bundled that can be fetched on demand).

## Option A — CEF-class embed (obs-browser's approach)

Chromium Embedded Framework with offscreen rendering (OSR): CEF paints
BGRA buffers straight into our capture pipeline; audio is interceptable
per-browser; transparency works; this is the battle-proven path every
OBS-family app ships.

- **For:** full web-platform correctness (WebGL, video, fonts, CSS
  animations), true offscreen at any resolution/fps, per-source audio,
  transparency.
- **Against:** ~150–200 MB per OS, its own release cadence (security
  updates are mandatory — Chromium CVEs apply to us the day they land),
  Rust bindings (`cef-rs`) are workable but the build is the heaviest
  thing in the workspace by far.
- **Charter fit:** distribute like ffmpeg — a clearly-labeled, on-demand,
  hash-verified **component download**, never bundled. That pattern
  already exists and users already understand it.

## Option B — per-OS hidden webview + window capture

Create a real (offscreen-positioned) WebView2/WKWebView/WebKitGTK window
and capture it with our existing window-capture pipeline.

- **For:** zero new dependencies; reuses two subsystems we own.
- **Against:** WebView2 has **no readback API** — capturing the window
  needs it composited (occlusion/minimize fights, DWM throttling of
  hidden windows), **no transparency**, fps at the mercy of the
  compositor, macOS/Linux each behave differently, and a visible-ish
  ghost window is a support nightmare. This is a demo, not a product
  feature.

## Option C — upstream wry/WebView2 offscreen work

WebView2's `CoreWebView2CompositionController` renders into a
DirectComposition visual, but Microsoft exposes **no frame readback**;
WKWebView can snapshot (`takeSnapshot`) at ~seconds cadence, not video
rate; WebKitGTK can render to a GtkOffscreenWindow with similar caveats.
Upstreaming a cross-platform offscreen API into wry is a multi-quarter
external-dependency bet we do not control.

## Recommendation

**Option A (CEF), shipped as an on-demand component in its own milestone
after 0.85.0** — with the same honesty pattern as ffmpeg: fetched on
first use, hash-verified, clearly labeled, never bundled. Until then the
picker keeps its honest note, and the documented workaround is capturing
a real browser window (Window Capture + chroma/color key — both ship).

**Decision needed from Mike:**
1. Approve CEF-as-component (size + Chromium-update cadence accepted)?
2. Or keep Browser source out of scope until a lighter path exists?

*Written 2026-07-07 during the Phase 6 build; no code was built for this
task by design ("decide with Mike before building").*
