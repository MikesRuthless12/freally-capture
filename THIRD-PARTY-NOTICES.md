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
pipeline**, and the **stream orchestration / muxers** are original works © Mike Weaver, covered by
[`LICENSE`](LICENSE) — they are not third-party components.

## Bundled / linked (compiled into the app)

| Component | Role | License |
|-----------|------|---------|
| [Tauri](https://tauri.app) (v2) | desktop shell + command/event bridge | MIT OR Apache-2.0 |
| [React](https://react.dev) / [Vite](https://vitejs.dev) / [Tailwind CSS](https://tailwindcss.com) | control UI + build + styling | MIT |
| [`wgpu`](https://github.com/gfx-rs/wgpu) | GPU compositor backend (owned compositor crate) | MIT OR Apache-2.0 |
| [`windows`](https://crates.io/crates/windows) (windows-rs) *(Windows only)* | DXGI + Windows.Graphics.Capture (screen/window capture) | MIT OR Apache-2.0 |
| [`objc2`](https://crates.io/crates/objc2) + ScreenCaptureKit bindings *(macOS only)* | screen/window + system-audio capture | MIT OR Apache-2.0 |
| [`ashpd`](https://crates.io/crates/ashpd) (+ PipeWire) *(Linux only)* | portal screen capture | MIT |
| [`nokhwa`](https://crates.io/crates/nokhwa) | webcam / capture-card input | Apache-2.0 OR MIT |
| [`cpal`](https://crates.io/crates/cpal) | audio capture + output graph | Apache-2.0 |
| [`nnnoiseless`](https://crates.io/crates/nnnoiseless) (RNNoise) | audio denoise | BSD-3-Clause |
| [`ort`](https://crates.io/crates/ort) / [`tract`](https://crates.io/crates/tract) | ONNX inference (DeepFilterNet denoise + selfie segmentation) | MIT / Apache-2.0 |
| [`rustybuzz`](https://crates.io/crates/rustybuzz) + [Noto fonts](https://fonts.google.com/noto) | text-source shaping (incl. RTL) + bundled fonts (reused from Freally Snipper) | MIT / SIL OFL 1.1 |
| [`tungstenite`](https://crates.io/crates/tungstenite) | WebSocket remote-control API | MIT OR Apache-2.0 |
| [`mlua`](https://crates.io/crates/mlua) (Lua) | scripting (later phase) | MIT |
| [`ed25519-dalek`](https://crates.io/crates/ed25519-dalek) | offline license verification | BSD-3-Clause |
| [`global-hotkey`](https://crates.io/crates/global-hotkey) | system-wide hotkeys | Apache-2.0 OR MIT |
| [`directories`](https://crates.io/crates/directories) | OS config/data paths | MIT OR Apache-2.0 |
| [`serde`](https://serde.rs) / [`serde_json`](https://crates.io/crates/serde_json) | scene/profile (de)serialization | MIT OR Apache-2.0 |
| [`fluent`](https://crates.io/crates/fluent) / `fluent-bundle` | i18n catalogs (18 locales) | Apache-2.0 OR MIT |

Transitive Rust dependencies are MIT / Apache-2.0 / BSD / Zlib / MPL.

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
| [MediaPipe Selfie Segmentation](https://developers.google.com/mediapipe) model | webcam **virtual background** (blur/replace) | **Apache-2.0** | optional; local; fetched on demand only if you enable virtual background; run via `ort`/`tract` |
| [DeepFilterNet](https://github.com/Rikorose/DeepFilterNet) model | optional high-quality audio **denoise** | MIT / Apache-2.0 | optional; local; fetched on demand |

Downloads are over **TLS** from fixed, hardcoded hosts. The **ffmpeg binary is verified against a pinned
hash before it is executed**. See [`SECURITY.md`](SECURITY.md) for the full download-integrity posture.

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
