# Changelog

All notable changes to Freally Capture will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project aims to adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> **Status: in development.** Phase 4 (recording, 0.55.0) is complete. Early
> development builds are downloadable from each release; the **studio MVP — the first build meant for
> everyday use — arrives at 0.70.0**. The release ladder below tracks the plan to 1.0.0.

## [Unreleased]

> The next rung is **0.70.0 (studio MVP — first public)**: single-target streaming, Studio Mode +
> transitions, the virtual camera, and the stats dock.

### Added — native GPU preview ("OBS feel"), Windows (in progress)
- **A zero-copy native GPU preview surface** that replaces the read-back → JPEG → canvas preview with
  the compositor's own GPU output painted directly on screen — no encode round-trip, no lag. On
  **Windows** this is a **DirectComposition** surface (wgpu `SurfaceTargetUnsafe::CompositionVisual` on
  DX12) composited *above* WebView2, **verified real-time on hardware**; the interactive **selection box
  + transform handles are drawn into the GPU frame itself** (preview-only — never recorded or streamed).
  Under the hood the GPU stack was **upgraded wgpu 0.20 → 27**. **macOS (CAMetalLayer) and Linux (Vulkan)
  land next**; until they do, those platforms keep the existing JPEG preview, which also stays as the
  universal fallback everywhere. *(Windows-only so far — not yet in a tagged release.)*

## [0.55.0] — 2026-07-03 (Audio mixer + recording)

> The 0.55.0 rung is **Phase 3 (audio mixer + filters)** plus **Phase 4 (recording)** — the studio can
> now mix and **record** its program feed to disk, multi-track, with the best available hardware encoder
> or the owned lossless codec.

### Added — recording (Phase 4)
- **The owned `freally-video` (`.frec`) lossless codec** (`fcap-encode`): the default local-recording
  format, authored here and **owned outright** — temporal frame deltas + PNG-style left-pixel
  prediction + an owned byte-aligned **FLZ** (LZ77) stage, every technique decades-expired or
  public-domain, **zero dependencies, nothing fetched**, `#![forbid(unsafe_code)]`. The container carries
  up to **6 interleaved stereo PCM tracks** with absolute sample positions (A/V sync + gapless pause by
  construction), intra frames on a ~2 s cadence, a seek index, and a **truncation-tolerant reader** (a
  crashed recording plays back to its last complete chunk; corrupt input errors, never panics, and is
  allocation-capped against hostile files). Real-time verified: a synthetic 1080p60 encode holds its
  budget.
- **Encoder detection + the encoder catalog**: a `wgpu` GPU probe (vendor ids + name heuristics,
  software rasterizers skipped) drives per-OS offer rules — **NVENC / Quick Sync / AMF** on Windows,
  **NVENC / VAAPI** (render-node gated) on Linux, **VideoToolbox** on macOS — always alongside the
  universal **x264 / x265 / AV1** CPU fallbacks. Hardware encoders are offered as honest candidates and
  **confirmed by a real 3-frame smoke encode** on first use (support varies by GPU + driver); every wire
  encoder labels its ffmpeg dependency.
- **Multi-track muxing, containers, and file splitting**: record to **mp4 / mkv / mov / webm** (wire
  codecs) or the owned **`.frec`** (lossless), with **up to 6 audio tracks**, and **automatic file
  splitting** into standalone playable segments. The recording engine runs a strict-CFR clock (frame
  count locked to recorded time), so A/V stays in sync through stalls and **pause/resume is gapless
  within one playable file**. The main recording is architected to continue regardless of stream state
  (streaming lands in 0.70.0), and a **separate-track local copy** is persisted for it.
- **The clearly-labeled, on-demand ffmpeg bridge** (`fcap-encode::ffmpeg`): the patent-encumbered wire
  codecs (H.264/AAC/HEVC/AV1) run through ffmpeg, which is **never bundled**. On first use it is fetched
  from a **per-OS pinned build** (URL + **SHA-256 baked into source**, cross-checked against the
  publisher's own checksum), **hash-verified before anything runs** (a mismatch aborts the install),
  extracted (a single member — archive paths are never used for writing), and driven as a **separate
  process** (which also keeps its LGPL/GPL license isolated from the owned app). A clearly-labeled
  **Components** panel shows what it is, why it exists, the pinned source, and live %/of-total/MB/s
  progress; the owned `.frec` path needs none of it.
- **HEVC / AV1 recording + post-record remux**: HEVC and AV1 record through the labeled ffmpeg path
  (hardware where the GPU supports it); a **Recordings** list (Controls → Files…) shows finished files
  newest-first and offers **Remux to MP4** (mkv → mp4, `-c copy`, no re-encode, faststart, `hvc1` tag on
  HEVC). The remux command validates the path lives inside the recordings folder — the webview can never
  point it at arbitrary files.
- **Encoder settings + presets** (Settings → Output): rate control **CBR / VBR / CQP** + bitrate +
  keyframe interval, per-encoder **Quality / Balanced / Performance** presets (mapped onto each family's
  own knob), per-track audio bitrate, and one-click **Lossless / High-quality / Balanced** record
  presets. Recording controls in the dock: **Start / Stop / Pause / Resume**, a pulsing **REC**
  indicator with duration (pauses excluded) + track count, and the last session's files or its honest
  error.
- **The Media source** (`fcap-sources::media`) — folded in from Phase 2: a video/image file composed on
  the canvas **with its audio in the mixer**. Still images decode once (like the Image source); **`.frec`
  plays through the owned codec** with nothing fetched; the wire formats decode through the labeled
  ffmpeg component (`-hwaccel auto` hardware decode, software fallback, loop/restart), and a stop
  watchdog means a wedged decoder can never wedge the studio. Media audio flows through a new
  **media-audio hub** so a media clip gets a full mixer strip (fader, filters, tracks, sync offset) with
  no special cases.

### Added — audio mixer (Phase 3)
- **The owned audio engine** (`fcap-audio`): a `cpal` capture graph running everything internally as
  **stereo f32 at 48 kHz** in 10 ms blocks. Per-source **microphone / line-in** (Audio Input) and
  **desktop / system audio** (Audio Output) capture, each format-converted and resampled into the
  engine clock through an **owned streaming linear resampler**. Desktop audio is told honestly per OS:
  **Windows** captures any output device via **WASAPI loopback**; **Linux** uses a PipeWire/PulseAudio
  **monitor** device; **macOS** needs a virtual loopback device (e.g. BlackHole) until a
  ScreenCaptureKit audio path lands — the pickers say so and filter for known virtual devices.
- **The mixing graph**: per-source **sync-offset delay → filter chain → push-to-talk / push-to-mute /
  mute / fader**, routed into **up to 6 track buses** (per-source assignment), the **program (master)**
  mix, and a **monitor** mix. Click-free ~8 ms gain smoothing on every mute/PTT/fader move. The whole
  core is pure and **device-free — every routing rule is unit-tested without hardware**.
- **The owned classic-DSP filter set — no ML anywhere, per charter**: **Denoise** (STFT spectral
  suppression with an owned radix-2 FFT — 512-sample √Hann frames, 50% overlap, per-bin noise-floor
  tracking + spectral subtraction; steady noise drops while speech passes), **Noise Gate** (hysteresis
  + hold), **Compressor** (peak-sensing, make-up gain), **Limiter** (instant-attack with a hard
  ceiling), **3-Band EQ** (RBJ shelf/peak biquads), **Gain**, and sidechain **Ducking** (dip one source
  under another). Every processor is clamped defensively so a hand-edited scene can't build a runaway.
- **Monitoring, ducking, PTT/PTM, and LUFS**: a **monitor output** on any device (bounded, underrun-safe);
  sidechain ducking driven by the trigger source's live envelope; **push-to-talk / push-to-mute** via
  global hotkeys (registered from the model, honest about Wayland's lack of global hotkeys); a
  **BS.1770 K-weighted LUFS** meter (momentary + short-term) on the program mix; per-source peak/RMS
  metering.
- **The Audio Mixer panel**: one **channel strip** per audio source in the active scene — name +
  status, a **VU meter** (green→yellow→red, peak tick), a **fader** (−60…+6 dB), **mute**, a **monitor**
  cycle, an **audio-filters** dialog (all 7 filters, with a trigger picker for ducking), **track dots
  1–6**, and an advanced popover (sync offset + PTT/PTM hotkeys). A **LUFS** readout and a
  **monitor-device** picker (persisted in settings). Audio sources appear in the Sources rail with the
  same live status dots as video, and in the **Add Source** menu with per-OS-honest device pickers.
- The scene/source model (`fcap-scene`) gains **Audio Input / Audio Output** source kinds and a
  per-source **`AudioSettings`** strip (fader, mute, monitor mode, track bitmask, sync offset, PTT/PTM
  hotkeys, the ordered audio-filter chain) — serde round-trip tested, camelCase wire-checked,
  range-clamped, and self-repairing on load (audio state exists exactly on audio-capable sources).

### Security / privacy
- The posture is unchanged and re-verified: captured audio + composed frames flow **only** to the mixer,
  the monitor device the user picks, the **recording file**, and the virtual camera — **nothing leaves
  the machine**, no accounts, no telemetry. The monitor-device name persisted in settings is
  length/shape-validated; hotkey accelerators are parsed/validated before they enter the model.
- **The one network action added this rung is the explicit, user-clicked ffmpeg download** — over TLS
  from a **hardcoded, per-OS pinned URL**, **SHA-256-verified before the binary is ever executed**, and
  driven as a separate process. It never starts on its own; a checksum mismatch aborts the install. The
  owned `.frec` recording path makes no network calls at all.
- The recording engine's ffmpeg audio tracks ride **loopback-only (`127.0.0.1`) sockets** that accept
  exactly one connection then close; recording settings (encoder id, container, bitrate, folder,
  filename) are **range/shape-validated** before use, and **remux/recordings actions are confined to the
  recordings folder** (canonicalized parent check) so the webview cannot reach arbitrary files. The
  `.frec`/media parsers treat file bytes as **untrusted** — allocation-capped, erroring rather than
  panicking on corrupt input.
- New third-party dependencies, all recorded in `THIRD-PARTY-NOTICES.md`: **`tauri-plugin-global-shortcut`**
  (push-to-talk/mute), and the ffmpeg-bridge fetch/verify/unpack set (**`ureq`** + rustls, **`sha2`**,
  **`zip`**, **`tar`** + **`lzma-rs`**) plus **`chrono`** (local-time filenames). The audio DSP and the
  **`freally-video`** codec are **entirely owned**; ffmpeg is **driven, never bundled**.
- **Live-hardware audio smoke tests** (kept `--ignored` on headless CI) plus a release-mode `.frec`
  1080p60 encode guardrail; the full in-app ffmpeg download → verify → record → play-back flow is the
  Phase 4 release smoke test.

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
[0.55.0]: https://github.com/MikesRuthless12/freally-capture/releases/tag/v0.55.0
[0.40.0]: https://github.com/MikesRuthless12/freally-capture/releases/tag/v0.40.0
[0.25.0]: https://github.com/MikesRuthless12/freally-capture/releases/tag/v0.25.0
[0.10.0]: https://github.com/MikesRuthless12/freally-capture/releases/tag/v0.10.0
