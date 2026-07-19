# Freally Capture

A **local-first**, **cross-platform** live-streaming & recording **studio** (OBS-class) for **Windows,
macOS, and Linux** — by [Havoc Software](https://github.com/mikesruthless12). Build a scene from sources,
compose it on the **GPU**, **record** multi-track, and **go live** — to **several platforms at once** —
all from **one clean app**. Scenes · sources · a real-time **wgpu compositor** · filters & transitions ·
**Studio Mode** · multi-track recording (hardware encoders + the **owned `freally-video`** lossless
codec) · broadcasting & **multistream** over RTMP/RTMPS/SRT/WHIP · a **virtual camera** · a **replay
buffer** · a **WebSocket remote API**. **No accounts, no telemetry, no cloud** — the only thing that
leaves your machine is the stream you chose to send.

> **Tagline:** *Record and stream like a studio — scenes, sources, multistream, one clean app.*

> **Status: in development — 0.99.0 closed all 26 must-haves, and the post-stable phases have landed
> through 0.900.0 (Studio Management & Workflow). 0.910.0 ships the Teleprompter Rework: a caesura
> pause-chip editor, an optional start countdown, chars/sec-or-BPM pacing, per-OS read-aloud, a
> stream-safe fullscreen projector, and 18-language ghost-text autocomplete.** The engine is
> complete end to end. Real per-OS **capture** (Windows DXGI + Windows.Graphics.Capture, macOS
> ScreenCaptureKit, Linux ScreenCast portal + X11), webcams/capture cards, the **owned wgpu GPU
> compositor** (60 fps @ 1080p verified on hardware), the **owned scene/source model**, the **on-GPU
> filter chain**, and the **owned audio engine** (up to 6 tracks; **spectral denoise — no ML** — plus
> gate, compressor, limiter, EQ, gain, ducking; monitoring, push-to-talk, LUFS) are all in.
> **Recording** (0.55.0) ships the **owned `freally-video` (`.frec`) lossless codec**, per-OS
> **hardware encoders** (NVENC/Quick Sync/AMF/VAAPI/VideoToolbox) + x264/x265/AV1 fallback,
> multi-track mp4/mkv/mov/webm with splitting and pause/resume — the patent-encumbered wire codecs
> run through the **clearly-labeled, on-demand, hash-verified ffmpeg bridge** (never bundled). The
> preview **feels like OBS** (0.56.0): a **native GPU preview** with no read-back round-trip
> (DirectComposition / CAMetalLayer / X11-Vulkan; the JPEG path stays as the universal fallback, and
> covers Wayland). **Streaming** is in — single-target (0.70.0) and **simultaneous multistream**
> over RTMP/RTMPS/SRT/WHIP with a vertical 9:16 canvas, nested scenes, a replay buffer, and a
> **no-key** chat overlay (0.85.0) — plus the **extensibility surface** (0.90.0: WebSocket remote
> API, browser docks, sandboxed Lua, a plugin SDK) and **distribution** (0.95.0: a signed,
> self-hosted auto-updater, per-app audio, an NDI seam, the EULA gate). The interface speaks **18
> languages**, is keyboard-operable and screen-reader-audible, and themes light/dark/custom
> (0.96.0). The **26 CAP-M must-haves** then landed as three batches: **scene authoring &
> monitoring** (0.97.0 — undo/redo, an OBS importer, a missing-file doctor, multi-select +
> guides, a keying workbench, multiview, projectors), **broadcast safety & reliability** (0.98.0 —
> a go-live pre-flight, always-on alarms, a source-health dashboard, mid-session encoder failover,
> crash-safe recording with next-launch repair, a quit guard, a panic button, filename templates, a
> redacted diagnostics bundle), and **sources, devices & calibration** (0.99.0 — test signals, an
> A/V sync calibration workbench, timer/clock sources, text bound to a watched file, a hotkey map
> with conflict detection, mixer pan/solo/mono, deinterlacing, and camera controls with per-device
> profiles). **0.200.0** then adds two themed phases at once — **Automation** (a rules engine, macros
> with studio variables, MIDI and OSC control surfaces, hotkey chords/layers, a LAN touch panel, a
> tally-light service, PTZ camera control, and a timed show rundown — every surface sharing one fixed,
> off-by-default command allowlist) and **Capture & Device Depth** (a low-latency passthrough monitor,
> pixel-perfect scaling, punch-in zoom, auto black-bar crop, window↔app-audio auto-linking, and
> HDR→SDR tone-mapping), alongside a scene backdrop/wallpaper and true reverse media playback.
> **1.0.0 ships once the remaining feature set lands** — it is deliberately gated on the
> *complete* product, not just the must-haves. A true browser source, the virtual camera, and
> game-capture GPU hooking follow as their own milestones. The detailed planning + design set
> (product vision, PRD, roadmap, build-prompts guide, and go-to-market plan) is **maintained
> privately** and is not published here.
> **Installers for every release are on the
> [releases page](https://github.com/MikesRuthless12/freally-capture/releases) — 0.910.0 is the
> latest.**

> **🔒 Local-first, no account, no cloud.** Composition, recording, and streaming all run **on your
> machine**. There is **no account** (a streaming tool should never become "connect your channel"), **no
> telemetry**, and **no cloud restreaming we run** — your stream goes **direct to the platform**. The
> only network actions are the **stream targets you configure**, the opt-in **remote guests** session
> you start (invite-only P2P WebRTC — camera/mic/screen flow **directly between peers**, encrypted in
> transit; only the tiny connection handshake crosses a signaling broker, and joining is always an
> explicit click), an optional **live chat overlay** you
> point at a channel (public chat reads only — **never an API key, account, or sign-in**), an
> optional **update check**, and the on-demand download of one clearly-labeled, non-bundled component:
> **ffmpeg** (for the patent-encumbered wire codecs the platforms require). There are **no AI/ML
> features and no model downloads**. See [`PRIVACY.md`](PRIVACY.md) and
> [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md).

## What it does

1. **Build a scene** — add sources (display, window, game, webcam, capture card, browser, media, image, **text — including text bound to a watched `.txt`/CSV/JSON file**, **timers & clocks**, **test signals**, color, audio) and arrange them on a GPU canvas (move/crop/scale/rotate), with per-source **filters** (chroma key, color correction, LUT, blur — Gaussian/directional/radial/zoom, pixelate, mask, sharpen, **freeze-frame**), a **3D perspective tilt**, scene **transitions** (cut/fade/slide/stinger/luma-wipe), and **downstream keyers** (persistent overlays on the program output).
2. **Compose** — the owned real-time **wgpu** compositor composes every source into the program frame on the GPU, with **Studio Mode** (preview/program) so you stage changes before they go live.
3. **Record** — multi-track, with your GPU's **hardware encoder** (NVENC/Quick Sync/AMF/VAAPI/VideoToolbox) + an **x264** fallback, to **mp4/mkv/mov/webm** — or in the **owned `freally-video`** codec for fully-lossless local capture, on up to **6 audio tracks**, with file splitting and a **separate-track local copy while streaming**.
4. **Go live** — broadcast over **RTMP/RTMPS/SRT/WHIP** to **Twitch / YouTube / Kick / Facebook / Trovo / custom**, with auto-reconnect and a configurable stream delay — including **multistream** to several platforms **at once**, **direct from your machine** (no restream server).
5. **Extras** — a rolling **replay buffer** with a save hotkey, a **vertical/multi-canvas** second output (recordable + streamable independently), a **live chat overlay** (YouTube/Twitch/Kick — **no key or sign-in, ever**) and **floating reactions** baked into the program, **nested scenes / source groups / per-scene audio**, stinger + luma transition packs, **recording chapter markers**, **global hotkeys** for everything (with a searchable **hotkey map** that flags conflicts), **profiles + scene collections**, a live **stats dock** (fps/dropped frames/CPU/GPU/bitrate), and a **teleprompter** — an operator dock, a stream-safe **fullscreen projector**, and LAN control — with caesura **pause chips**, a **start countdown**, **chars/sec or BPM** pacing, per-OS **read-aloud**, and **18-language ghost-text autocomplete**.
6. **Keep the show safe** — a **go-live pre-flight checklist**, always-on **safety alarms** (silent audio, clipping, a black/frozen picture, a low-disk forecast), a **source-health dashboard**, **mid-session encoder failover**, **crash-safe recording** with a next-launch repair, a **quit guard**, and a **panic button** that cuts to a privacy slate and hard-mutes everything.
7. **Calibrate the chain** — built-in **test signals** (SMPTE bars, a calibration grid, a motion sweep, a 1 kHz lineup tone, an A/V flash+beep) and a guided **A/V sync calibration workbench** that measures your camera/mic offset and applies it; plus **deinterlacing** and cross-platform **camera controls** (exposure, white balance, focus, zoom) with per-device profiles that survive hotplug.
8. **Extend it** — a password-protected **WebSocket remote-control API** (Stream Deck / Companion-style; **off by default**, loopback unless you opt into LAN), **browser docks** (chat popouts / alerts / web buttons as their own window), **sandboxed Lua scripting** (react to go-live/scene/recording events, drive the studio — no file or OS access), and a **plugin SDK** (add a source or filter without touching core). A **virtual camera** and a true **browser source** follow as their own signed-driver / on-demand-component milestones.

It is **OBS-class power in one clean app** — an owned GPU compositor and an owned lossless codec, fully
local and account-free, the same on all three desktop OSes, and integrated with the Freally suite.

## Relationship to Freally Snipper

Freally Capture is the **sibling** of **Freally Snipper** in the Freally suite — they share owned
technology but do different jobs:

- **[Freally Snipper](https://github.com/MikesRuthless12/freally-snipper)** = screenshots + quick screen recording + image/video **editing** (the capture-and-edit tool).
- **Freally Capture** = the **live production / streaming studio** (OBS-class): a real-time scene compositor, live broadcasting/multistream, a virtual camera, a replay buffer, and multi-track recording (the go-live tool).

They share the owned **`freally-video`** codec (Capture: lossless local recording; Snipper: default
`.fvid` record format) and overlay-object/font concepts, plus the Havoc theme and the privacy invariants
— but they are **deliberately separate, focused apps**. Capture is the dedicated, full-depth home for
live production.

## License (important)

Freally Capture is **proprietary, source-available** software — **© 2026 Mike Weaver. All Rights
Reserved.** The source is **public so you can read it and build/run it for your own personal
evaluation**, but it is **not open source**: you may not copy, modify, redistribute, or reuse it. See
[`LICENSE`](LICENSE). Bundled third-party components keep their own licenses — see
[`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md). The pre-built application is **completely free to
download and run — every feature, for everyone. No paid tier, no payments, no license keys, no ads.**

## Security & privacy

**Local-first and account-free** — no accounts, no cloud, no telemetry; composition/recording/streaming
run on your machine and your stream goes direct to the platform. To report a vulnerability, see
[`SECURITY.md`](SECURITY.md) (please report **privately**, not via a public issue). The intended terms of
use are in [`EULA.md`](EULA.md) and the privacy policy in [`PRIVACY.md`](PRIVACY.md) (both DRAFT pending
legal review).

## Stack

> Every engine piece below is **built and shipping**: the Tauri v2 + React shell, the workspace, CI
> and packaging; per-OS capture; the wgpu compositor + GPU filters; the owned audio engine and
> mixer; multi-track recording (owned codec + hardware encoders); streaming and multistream; the
> remote API, docks, scripting and plugin SDK; and the signed self-hosted auto-updater. What
> remains before 1.0.0 is polish, not engine work.

**Tauri v2** shell + **React + TypeScript (Vite)** control UI (Havoc dark) · a **Rust** Cargo workspace
(the `src-tauri` app crate + owned crates `capture`, `compositor`, `encode`, `stream`, `audio`,
`sources`, `scene`) · the GPU compositor on **`wgpu`** · per-OS capture (**Windows** DXGI +
Windows.Graphics.Capture, **macOS** ScreenCaptureKit, **Linux** PipeWire portal + X11) · webcam via
**`nokhwa`** · audio via **`cpal`** (+ owned classic-DSP filters incl. denoise — no ML) · the **owned `freally-video`**
lossless codec · hardware encoders (**NVENC/Quick Sync/AMF/VAAPI/VideoToolbox**) + **x264** fallback · a
clearly-labeled, on-demand **ffmpeg** bridge for the patent-encumbered wire codecs · the **WebSocket
remote API** via **`tungstenite`** · the **Tauri bundler** per-OS installers.

## Free for everyone

**Everything is free.** There is no Pro tier, no payments, no license keys, no ads, and no account —
the full studio for everyone: scenes/sources/compositor, all video + audio filters, multi-track
recording (hardware encoders + `freally-video` lossless), streaming **including multistream to many
targets** and **SRT/WHIP**, the **virtual camera**, Studio Mode + all transitions (including the
stinger/luma packs), the **replay buffer with presets**, **vertical/multi-canvas output**, scripting/
plugins, and **remote-control automation**.

## Requirements

- [Rust](https://rustup.rs) (stable; pinned via `rust-toolchain.toml`) + Node (for the Vite UI).
- A GPU with up-to-date drivers (the `wgpu` compositor uses the GPU; x264 CPU fallback exists).
- **Linux only** — system libraries for the GUI/webview, PipeWire capture, audio, and webcam:
  ```sh
  sudo apt-get install -y \
    pkg-config libgtk-3-dev libwebkit2gtk-4.1-dev librsvg2-dev \
    libxcb1-dev libxrandr-dev libxkbcommon-dev libxkbcommon-x11-dev \
    libpipewire-0.3-dev libwayland-dev libegl-dev \
    libasound2-dev libpulse-dev libv4l-dev libdbus-1-dev libclang-dev \
    libudev-dev
  ```
  (`libv4l-dev` is for the webcam; `libudev-dev` is for gamepad detection (the input overlay);
  `v4l2loopback` is needed for the **virtual camera** on Linux.)

## Build & run

```sh
# UI + Tauri app
npm install                      # install the React/Vite UI deps
npm run tauri dev                # run the studio (dev build; keeps a console for logs)
npm run tauri build              # optimized per-OS installer (Windows .exe has NO console window)
```

```sh
# Rust workspace checks (mirror CI)
cargo fmt --all -- --check                   # format check
cargo clippy --all-targets -- -D warnings    # lint (warnings = errors)
cargo test                                   # tests (incl. compositor golden frames)
```

```sh
# One command that runs the WHOLE CI suite locally, before you push
npm run ci:local                 # rustfmt · clippy · test · deny/audit · UI (prettier/eslint/i18n/
                                 # theme/vitest/build) · Playwright e2e · Tauri debug compile
npm run ci:local -- --no-e2e --no-tauri-build   # fast inner-loop subset
```

These mirror exactly what CI runs (`.github/workflows/ci.yml`) on Windows, macOS, and Linux — `npm run
ci:local` runs every job in one pass and prints a pass/fail summary, so a green run means CI will be green.

> **Windows release = GUI app, no console window.** `src-tauri/src/main.rs` keeps
> `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` and the release job builds
> `--release`, so the shipped `.exe` never pops a terminal (only *debug* builds keep the console, for
> logs).

## Packaging & releases

The Tauri bundler produces a per-OS installable artifact. Pushing a version tag triggers
[`.github/workflows/release.yml`](.github/workflows/release.yml), which builds the app on all three OSes,
packages each, and opens a **draft GitHub Release** with the downloadable installers (reviewed, then the
**agent publishes for Mike** — Mike never clicks Publish):

| OS | Produces | Notes |
|----|----------|-------|
| Windows | `.msi` / `.exe` (NSIS) | GUI app, no console window |
| macOS | `.app` / `.dmg` | **notarized**; ships the **CoreMediaIO virtual-camera plugin** |
| Linux | AppImage / `.deb` / `.rpm` / Flatpak | virtual camera needs `v4l2loopback` |

Signed/notarized installers and the **signed, self-hosted auto-updater** shipped in **0.95.0** — the
updater refuses any package it can't verify against a public key baked into the binary.

A **Releases & Updates** web page lives in [`docs/`](docs/) (a static site). Publish it via
**Settings → Pages → Deploy from a branch → `main` / `docs`** to serve it at
`https://mikesruthless12.github.io/freally-capture/`.

## Workspace layout

```
.
├── README.md                # this file
├── CHANGELOG.md             # Keep a Changelog
├── SECURITY.md              # security policy
├── PRIVACY.md               # privacy policy (DRAFT)
├── EULA.md                  # end-user license agreement (DRAFT)
├── THIRD-PARTY-NOTICES.md   # bundled / downloaded / driven components
├── THIRD-PARTY.md           # data + platform components (dictionaries, OS speech)
├── LICENSE                  # proprietary, source-available — All Rights Reserved
├── Cargo.toml               # Rust workspace (src-tauri + crates/*)
├── src-tauri/               # `freally-capture` app crate (Tauri v2)
├── crates/                  # owned: capture, compositor (wgpu), encode, stream, audio, sources, scene
├── ui/                      # React + TypeScript control UI (Vite)
└── docs/                    # GitHub Pages site (Releases & Updates + Documentation)
```

> The planning, spec, roadmap, build-prompts, and go-to-market documents are kept **private** and are deliberately **not** part of this public repository.

## Roadmap

The detailed build plan is maintained privately. Public release ladder:
**0.10.0** (foundation — **done**) → **0.25** (capture core — **done**) → **0.40** (compositor +
scenes/sources — **done**) → **0.55** (audio + recording — **done**) → **0.70** (studio MVP — **done**)
→ **0.85** (multistream/SRT/WHIP — **done**) → **0.95/0.96** (distribution + launch polish — **done**)
→ **0.99** (all 26 must-haves — **done**) → **0.100.0–0.900.0** (post-stable CAP-N phases — **done**)
→ **0.910.0** (teleprompter rework — **current**) → **1.0.0**. Progress is published on the [project site](https://mikesruthless12.github.io/freally-capture/).

---

© 2026 Havoc Software · Mike Weaver &lt;mythodikalone@gmail.com&gt; · All Rights Reserved · _Project started: June 30th, 2026 · Early development builds on the [releases page](https://github.com/MikesRuthless12/freally-capture/releases); studio MVP at 0.70.0._
