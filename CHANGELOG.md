# Changelog

All notable changes to Freally Capture will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project aims to adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> **Status: in development.** Phase 2 (compositor + scenes/sources, 0.40.0) is complete. Early
> development builds are downloadable from each release; the **studio MVP — the first build meant for
> everyday use — arrives at 0.70.0**. The release ladder below tracks the plan to 1.0.0.

## [Unreleased]

_Nothing yet — 0.55.0 (audio + recording) is next._

## [0.40.0] — 2026-07-02 (Compositor + scenes/sources)

### Added
- **The owned wgpu GPU compositor** (`fcap-compositor`): every visible scene item composes
  back-to-front into the program frame with per-item **move/scale/rotate/crop** (one authoritative,
  unit-tested transform), all seven **blend modes** (normal/additive/subtract/screen/multiply/
  lighten/darken as fixed-function blend states), stride-aware BGRA/RGBA uploads with no CPU
  swizzle, and a headless readback path. Golden tests assert real GPU pixels and skip loudly on
  adapterless machines; the hardware perf gate holds **60 fps at 1080p with 4 sources**
  (5.2 ms/frame measured on an RTX 4070).
- **The owned scene/source/filter model** (`fcap-scene`): scenes hold ordered items (index =
  z-order); sources live in a shared pool and are referenced across scenes; items carry
  transform/blend/visibility/lock and an ordered filter chain. Serde round-trip tested,
  unknown-key tolerant, self-repairing on load — this is the scene-collection project format,
  autosaved (atomic, debounced) to `scene-collection.json` in the OS config dir.
- **The on-GPU video filter chain**, per item and live-parameterized: **Chroma Key** (with spill
  suppression), **Color Correction** (gamma/brightness/contrast/saturation/hue/opacity),
  **LUT** (.cube → 3D lattice), **Blur** (separable gaussian), **Image Mask** (alpha/luma, invert),
  **Sharpen**, **Scroll** (wrapping ticker), and **Crop** — ordered, toggleable, each verified
  against rendered pixels. Filters whose file has not loaded are skipped, never rendered black.
- **New sources**: **Image** (PNG/JPEG/BMP/GIF/WebP…), **Color** (solid block), and **Text** —
  real shaping via rustybuzz (Arabic joining, ligatures, kerning), UAX #9 bidi RTL, word wrap,
  alignment, line spacing, anti-aliased tiny-skia rasterization. The **complete Noto Sans family
  is bundled** (variable fonts: every weight/width, upright + Italic + Arabic + Hebrew; OFL-1.1
  vendored, provenance pinned + hashed) with **per-run script fallback**, so text renders
  identically on every machine; system families stay selectable and a font file overrides.
- **The studio runtime**: capture sessions and static sources reconcile against the active scene
  every tick; newly added items auto-fit on their first frame; the composed program frame feeds
  the same in-process `preview://` pipe at ~30 fps while composing at ~60; per-source
  live/waiting/error status + compose fps flow on the `program` event. A GPU-less machine gets an
  honest "no GPU" report instead of a frozen canvas.
- **The studio UI**: working **Scenes** rail (add/select/rename/reorder/remove), a **Sources**
  rail with visibility/lock/z-order/status per item and an add menu covering every implemented
  kind (plus cross-scene source sharing), **pixel-accurate on-canvas transform handles** (drag,
  corner/edge scale, rotate with 15° snap, Alt-drag cropping), a **Filters** dialog (blend mode +
  the live-editable chain), and per-kind **Properties** dialogs.

### Honest scope notes
- **Media** (video files) is **folded into the recording phase** (decided 2026-07-02): it rides
  the wire-codec / hardware-decode architecture (on-demand ffmpeg) — no pure-Rust general video
  decoder exists, and stubs are against the charter.
- **Browser source** moves to the streaming-depth phase behind an **offscreen-webview design
  spike** (Tauri v2 cannot render a webview to a texture cross-platform).
- **CJK text** uses system fonts for now — Noto CJK (~100 MB) is not bundled; the bundled set
  covers Latin/Greek/Cyrillic + Arabic + Hebrew.

### Security / privacy
- The posture is unchanged: composed program frames stay **in-process** behind the CORS-pinned
  `preview://` scheme; the only file the studio writes is the local scene collection. New
  third-party dependencies (`wgpu` runtime use, `image`, `tiny-skia`, `fontdb`, `unicode-bidi`,
  `pollster`) are permissively licensed and recorded in `THIRD-PARTY-NOTICES.md`.

## [0.25.0] — 2026-07-01 (Capture core)

### Added
- **Per-OS screen & window capture** behind the owned `fcap-capture` interface, yielding timestamped,
  GPU-uploadable frames through a latest-wins channel with an honest dropped-frame count:
  - **Windows** — DXGI Desktop Duplication for displays (with mouse-pointer blending; duplication
    excludes the cursor) and **Windows.Graphics.Capture** for individual windows (Windows 10 1903+),
    surviving display-mode switches and window resizes.
  - **macOS** — **ScreenCaptureKit** (displays + windows, macOS 12.3+); the Screen-Recording
    permission is requested up front, and a denial shows an **Open Screen Recording settings**
    deep-link instead of a silent black frame. (Bundle minimum raised 12.0 → 12.3 —
    ScreenCaptureKit does not exist below 12.3.)
  - **Linux** — the **ScreenCast portal** (`ashpd` → PipeWire) on Wayland, where the *system dialog*
    picks the screen or window (the only capture Wayland allows — the picker says so plainly), plus
    a direct **X11** path (screens + `_NET_CLIENT_LIST` windows) on X sessions.
- **Webcam / capture-card** sources via `nokhwa` (Media Foundation / AVFoundation / V4L2): device
  enumeration, per-device format listing (resolution / fps / wire format), live RGBA frames, and the
  macOS camera-permission flow (`NSCameraUsageDescription` bundled).
- **Live program preview**: the Sources rail's “+” now works — Display Capture, Window Capture, and
  Video Capture Device pickers; the selected source renders live in the program preview with a
  label / resolution / measured-fps / dropped-frames overlay. The pipe is in-process end to end
  (capture → JPEG → the app-private `preview://` scheme → canvas) — frames never touch a socket or
  disk. Direct draw proves the pipe; the wgpu compositor takes over at 0.40.0.
- **Live-hardware smoke tests** (kept `--ignored` on headless CI): display, window, and webcam
  capture verified with real frames on real hardware.

### Security / privacy
- Preview frames stay **in-process** (a custom URI scheme — no localhost socket, no temp files); the
  posture is unchanged: zero outbound network calls, no accounts, no telemetry.
- OS permission denials (macOS Screen Recording / Camera) surface honestly with a settings deep-link.
- New third-party notices: `pipewire` + `x11rb` (Linux capture) and `mozjpeg` (webcam MJPEG decode) +
  `jpeg-encoder` (preview encode) — the latter two carry the **IJG** license, now acknowledged in
  `THIRD-PARTY-NOTICES.md` and allowed in `deny.toml`.

## [0.10.0] — 2026-07-01 (Foundation & repo)

### Added
- **Tauri v2 + React 19 / TypeScript / Vite 7 / Tailwind 4** control UI (Havoc dark) over a Rust Cargo workspace: the `freally-capture` app crate + the owned `fcap-*` engine crates (`capture`, `sources`, `compositor`, `scene`, `audio`, `encode`, `stream`), all `#![forbid(unsafe_code)]`.
- The **studio dock layout** (large program preview, Scenes/Sources rails, Audio Mixer, Controls, Stats) with honest not-yet-implemented states on every control.
- The **typed UI ↔ core bridge**: `health()` (app + linked core-crate versions), `settings_get`/`settings_set`, and a ~2 Hz `stats` push event rendered live in the stats dock (placeholder data, labeled as such until real sampling in 0.70.0).
- The **settings store**: JSON in the OS config dir, atomic temp-file+rename writes, corrupt/missing-file fallbacks; the stats-dock toggle persists across runs.
- **CI matrix** (Windows/macOS/Linux): rustfmt, clippy `-D warnings`, tests, cargo-deny + cargo-audit, UI prettier/eslint/tsc/vitest/build, and per-OS Tauri debug builds; a tag-triggered **release workflow** (per-OS installers → draft GitHub Release).
- The proprietary `LICENSE` + governance docs and the seed `docs/` GitHub Pages site with the true-alpha badge icon.
- **Windows release verified**: GUI app with **no console window** (checked on the built binary); MSI + NSIS installers build.

### Security / privacy
- **Zero outbound network calls** in the app; a strict CSP; the minimal `core:default` capability only; stream keys/secrets are not stored (none exist yet); **no AI/ML anywhere** (charter).

### Planned — 0.25.0 (Capture core)
- Per-OS screen/window capture (Windows DXGI + Windows.Graphics.Capture; macOS ScreenCaptureKit; Linux PipeWire portal + X11) behind a `Capture` interface.
- Webcam / capture-card input via `nokhwa`; a live program preview.

### Planned — 0.40.0 (Compositor + scenes/sources)
- The owned `wgpu` GPU compositor: per-source transform/crop/scale/rotate, blend modes, and the video filter chain (chroma key, color correction, LUT, blur, mask, sharpen, scroll).
- The owned scene/source/filter data model; the full source set (image, text, color, media, browser); the Scenes + Sources rails with on-canvas transform handles.

### Planned — 0.55.0 (Audio mixer + recording)
- The `cpal` audio graph: per-source volume/mute/monitor, up to 6 tracks, sync offset, ducking, push-to-talk, a LUFS meter, and filters (owned classic-DSP denoise — no ML — plus gate, compressor, limiter, EQ, gain).
- Multi-track recording via hardware encoders (NVENC/Quick Sync/AMF/VAAPI/VideoToolbox) + x264 fallback, plus the owned `freally-video` lossless codec; containers mp4/mkv/mov/webm; file splitting; separate-track local copy. The patent-encumbered wire codecs via the on-demand, hash-verified ffmpeg bridge.

### Planned — 0.70.0 (Studio MVP — first public)
- Single-target live streaming (RTMP/RTMPS) to Twitch/YouTube/Kick/Facebook/Trovo/custom, with auto-reconnect and stream delay; the main recording continues regardless of stream state.
- Studio Mode (preview/program) + transitions (cut/fade/slide/swipe/stinger/luma-wipe); the virtual camera (per-OS); global hotkeys; profiles + scene collections; the stats dock. **First public build — completely free, like every Freally app.**

### Planned — 0.85.0 (Multistream / SRT / WHIP + scene/source/encoder depth)
- Simultaneous multistream to several targets, direct to each platform; SRT and WHIP protocols; vertical/multi-canvas output.
- The rolling replay buffer with presets; nested scenes, source groups + per-scene audio; an image-slideshow source + capture-card presets; color/luma-key + render-delay filters; high-FPS/4K output, color-space handling + output downscale; recording-side stream markers; virtual-camera depth (single-source + audio); premium stinger + luma packs; advanced filters.

### Planned — 1.0.0 (Remote API, scripting, game capture & launch)
- The WebSocket remote-control API (Stream Deck / Companion-style) + browser docks; Lua/JS scripting + a plugin SDK.
- Game capture (DX/GL/Vulkan GPU-API hooking, flagged honestly); optional NDI + VST.
- Signed/notarized installers (Win MSI/NSIS, macOS .app/.dmg + the virtual-camera plugin, Linux AppImage/.deb/.rpm/Flatpak) + self-hosted auto-update.
- Accessibility (keyboard-first, screen-reader-labeled, high-contrast) and UI localization into 18 languages (`ar de en es fr hi id it ja ko nl pl pt-BR ru tr uk vi zh-CN`, English first, RTL Arabic); onboarding + templates.

[Unreleased]: https://github.com/MikesRuthless12/freally-capture/commits/main
[0.25.0]: https://github.com/MikesRuthless12/freally-capture/releases/tag/v0.25.0
[0.10.0]: https://github.com/MikesRuthless12/freally-capture/releases/tag/v0.10.0
