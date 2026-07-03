# Third-Party Notices

Freally Capture is proprietary software (© 2026 Mike Weaver — All Rights Reserved; see
[`LICENSE`](LICENSE)). It incorporates, drives, or downloads the third-party components listed below,
each of which remains licensed under its own terms. This file provides the attribution those licenses
require. Trademarks belong to their respective owners; listing here does not imply endorsement.

> Third-party components are kept **behind interfaces** so an owned implementation can replace them
> later. This list grows as later phases add dependencies. Verify the full set with `cargo about` /
> `cargo deny` before any release.

## Owned (NOT third-party)

`freally-video` (the owned lossless recording codec, `.frec`, shared with **Freally Snipper**), the
**`wgpu`-based GPU compositor**, the **scene/source/filter data model**, the **per-OS capture
pipeline**, the **audio engine** (the mixing graph, the owned radix-2 FFT + spectral denoiser, the
noise gate / compressor / limiter / EQ / gain / ducker filter set, BS.1770 LUFS metering, and the
resampler — `cpal` supplies only device I/O), and the **stream orchestration / muxers** are original
works © Mike Weaver, covered by [`LICENSE`](LICENSE) — they are not third-party components.

## Bundled / linked (compiled into the app)

| Component | Role | License |
|-----------|------|---------|
| [Tauri](https://tauri.app) (v2) | desktop shell + command/event bridge | MIT OR Apache-2.0 |
| [React](https://react.dev) / [Vite](https://vitejs.dev) / [Tailwind CSS](https://tailwindcss.com) | control UI + build + styling | MIT |
| [`wgpu`](https://github.com/gfx-rs/wgpu) | GPU compositor backend (owned compositor crate) | MIT OR Apache-2.0 |
| [`windows`](https://crates.io/crates/windows) (windows-rs) *(Windows only)* | DXGI + Windows.Graphics.Capture (screen/window capture) | MIT OR Apache-2.0 |
| [`objc2`](https://crates.io/crates/objc2) + ScreenCaptureKit bindings *(macOS only)* | screen/window + system-audio capture | MIT OR Apache-2.0 |
| [`ashpd`](https://crates.io/crates/ashpd) *(Linux only)* | ScreenCast portal negotiation (Wayland-safe capture) | MIT |
| [`pipewire`](https://crates.io/crates/pipewire) (pipewire-rs) *(Linux only)* | consume the portal's video stream (links system libpipewire) | MIT |
| [`x11rb`](https://crates.io/crates/x11rb) *(Linux only)* | direct X11 screen/window capture | MIT OR Apache-2.0 |
| [`nokhwa`](https://crates.io/crates/nokhwa) | webcam / capture-card input | Apache-2.0 |
| [`mozjpeg`](https://crates.io/crates/mozjpeg) (via `nokhwa`) | MJPEG webcam-frame decode | IJG AND Zlib AND BSD-3-Clause |
| [`jpeg-encoder`](https://crates.io/crates/jpeg-encoder) | preview-frame JPEG encoding (the in-app preview pipe) | (MIT OR Apache-2.0) AND IJG |
| [`cpal`](https://crates.io/crates/cpal) | audio device I/O only — capture + monitor output (the whole DSP engine: mixer, FFT, spectral denoise, gate/comp/limiter/EQ, LUFS, resampler, is **owned** — no ML) | Apache-2.0 |
| [`pollster`](https://crates.io/crates/pollster) | blocking bridge for `wgpu`'s async adapter/readback calls | MIT OR Apache-2.0 |
| [`image`](https://crates.io/crates/image) | Image-source decode + Image-Mask filter files (PNG/JPEG/BMP/GIF/WebP…) | MIT OR Apache-2.0 |
| [`rustybuzz`](https://crates.io/crates/rustybuzz) | text-source shaping (Arabic joining, ligatures, kerning; RTL) | MIT |
| [`unicode-bidi`](https://crates.io/crates/unicode-bidi) | UAX #9 bidirectional line ordering for the Text source | MIT OR Apache-2.0 |
| [`tiny-skia`](https://crates.io/crates/tiny-skia) | anti-aliased glyph rasterization for the Text source | BSD-3-Clause |
| [`fontdb`](https://crates.io/crates/fontdb) | font discovery (bundled Noto first, then system fonts) | MIT |
| [Noto Sans](https://notofonts.github.io) (complete variable family: upright + Italic + Arabic + Hebrew) | the Text source's bundled default fonts — identical rendering on every machine (`crates/sources/fonts/`, provenance + hashes in its README) | SIL OFL 1.1 (vendored as `crates/sources/fonts/OFL.txt`) |
| [`tungstenite`](https://crates.io/crates/tungstenite) | WebSocket remote-control API | MIT OR Apache-2.0 |
| [`mlua`](https://crates.io/crates/mlua) (Lua) | scripting (later phase) | MIT |
| [`global-hotkey`](https://crates.io/crates/global-hotkey) | system-wide hotkeys | Apache-2.0 OR MIT |
| [`tauri-plugin-global-shortcut`](https://crates.io/crates/tauri-plugin-global-shortcut) | audio push-to-talk / push-to-mute global shortcuts (the full hotkey map lands in Phase 5) | MIT OR Apache-2.0 |
| [`directories`](https://crates.io/crates/directories) | OS config/data paths | MIT OR Apache-2.0 |
| [`serde`](https://serde.rs) / [`serde_json`](https://crates.io/crates/serde_json) | scene/profile (de)serialization | MIT OR Apache-2.0 |
| [`fluent`](https://crates.io/crates/fluent) / `fluent-bundle` | i18n catalogs (18 locales) | Apache-2.0 OR MIT |

Transitive Rust dependencies are MIT / Apache-2.0 / BSD / Zlib / MPL / IJG.

> **IJG acknowledgment** (required by the libjpeg-lineage license of `mozjpeg` and `jpeg-encoder`):
> this software is based in part on the work of the **Independent JPEG Group**.

> **Linux note:** the Tauri webview links **WebKitGTK**; capture links **PipeWire**, and the build links
> `libwayland`, `libxcb`, `libgtk-3`, `libasound2`, `libv4l`, and related system libraries (see
> `README.md` for the full `apt` list). The **virtual camera** on Linux requires the user-installed
> **`v4l2loopback`** kernel module.

## Hardware video encoders (driven via system APIs — not vendored)

| Component | Role | License / Terms |
|-----------|------|-----------------|
| NVIDIA **NVENC** | hardware H.264/HEVC/AV1 encode (NVIDIA GPUs) | NVIDIA SDK / driver terms |
| Intel **Quick Sync** | hardware encode (Intel GPUs) | Intel driver terms |
| AMD **AMF** | hardware encode (AMD GPUs) | AMD driver terms |
| **VAAPI** *(Linux)* | hardware encode (Intel/AMD on Linux) | MIT (libva) + driver terms |
| Apple **VideoToolbox** *(macOS)* | hardware encode (Apple platforms) | Apple SDK terms |

Freally Capture **drives** these encoders through the operating system / vendor drivers; it does not
bundle or redistribute them. The CPU fallback uses **x264** (see below).

## Downloaded on demand (NOT bundled, NOT linked)

| Component | Role | License | Notes |
|-----------|------|---------|-------|
| [ffmpeg](https://ffmpeg.org) (via [`ffmpeg-sidecar`](https://crates.io/crates/ffmpeg-sidecar) / [`ffmpeg-next`](https://crates.io/crates/ffmpeg-next)) | the patent-encumbered **wire codecs** (H.264/AVC, AAC, HEVC, AV1) required to **stream** to platforms and to **export** certain formats | **LGPL / GPL** (the binary's own license) | **fetched on demand** to a per-user cache, **hash-verified** before use; the owned `freally-video` is the default for local lossless recording |

ffmpeg is the **only** on-demand component — Freally Capture ships **no AI/ML features and downloads
no models**. Downloads are over **TLS** from fixed, hardcoded hosts. The **ffmpeg binary is verified
against a pinned hash before it is executed**. See [`SECURITY.md`](SECURITY.md) for the full
download-integrity posture.

## x264 (CPU encoder fallback)

| Component | Role | License |
|-----------|------|---------|
| [x264](https://www.videolan.org/developers/x264.html) | CPU H.264 encode fallback when no hardware encoder is available | GPL (the encoder's own license) |

x264 is used as a fallback encoder for H.264. Its GPL stays with that component; where bundled, it is
kept behind the `crates/encode` interface and its license is honored. Where the project chooses to ship
it, the relevant source-availability obligations are met; otherwise the encoder is fetched/driven like
the other wire-codec tooling. (Default recording uses hardware encoders or the owned `freally-video`.)

## Optional (later phases — listed now for licensing clarity)

| Component | Role | License | Notes |
|-----------|------|---------|-------|
| **NDI** SDK | optional networked source/output | NewTek/NDI SDK terms | optional; driven, not vendored; clearly labeled |
| **VST2/VST3** plugins (host) | optional audio plugins | Steinberg VST SDK terms | the user's own plugins; the host integration is behind an interface |

## Codec / patent note

H.264 (AVC), AAC, HEVC, and AV1 are **patent-encumbered** (AVC/AAC/HEVC via patent pools; AV1 is
royalty-free per the Alliance for Open Media but the broader landscape is still maturing). The streaming
platforms (Twitch, YouTube, Kick, etc.) **require** these "wire" codecs for ingest — the owned
`freally-video` codec is **not accepted by those servers**. Freally Capture therefore:

- uses the owned **`freally-video`** codec (expired-patent / public-domain techniques only) as the
  **default for local lossless recording**, which needs **no external tool**; and
- provides the patent-encumbered **wire codecs only via ffmpeg**, which is **not bundled** — it is
  **fetched on demand**, **hash-verified**, and run behind the `crates/encode` interface — exactly the
  posture Freally Snipper uses for MP4/WebM export.

(A from-scratch H.264 encoder would **not** avoid these patents — they cover the format's techniques,
not the code — so an owned wire encoder is revisited only once the relevant patents fully expire.)
Keep this file current with every bundled/downloaded/driven component before any release.
