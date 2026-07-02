# Changelog

All notable changes to Freally Capture will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project aims to adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> **Status: in development.** Phase 0 (foundation, 0.10.0) is complete; the app is not yet published —
> **downloads will be available in future releases.** The release ladder below tracks the plan to 1.0.0.

## [Unreleased]

_Nothing yet — 0.25.0 (capture core) is next._

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
