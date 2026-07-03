# Security Policy

Freally Capture is proprietary software (© 2026 Mike Weaver — All Rights Reserved; see
[`LICENSE`](LICENSE)). Protecting your data is a core design goal: the app is **local-first and
account-free** — composition, recording, and streaming all run **on your machine**, with **no accounts,
no cloud restreaming we run, and no telemetry**. Your broadcast goes **direct** to the platforms you
configure.

## Supported versions

Freally Capture is pre-1.0 and under active development. Security fixes target the **latest** commit on
the default branch; older snapshots are not maintained.

| Version | Supported |
|---------|-----------|
| latest (`main`) | ✅ |
| older | ❌ |

## Reporting a vulnerability

Please report security issues **privately — do not open a public issue or PR**.

- **Email:** mythodikalone@gmail.com (subject: `Freally Capture security`), **or**
- **GitHub:** use **Security ▸ Report a vulnerability** (private vulnerability reporting) on this repo.

Include the affected version/commit, your OS, reproduction steps, impact, and any proof-of-concept.
You'll get an acknowledgement and status updates through to the fix. Please allow reasonable time to
remediate before any public disclosure.

## Scope & notes

- **Local-first:** the core never transmits your captures or recordings. The outbound network actions
  are *limited and explicit* — the **stream targets you configure**, the optional **ffmpeg / model
  downloads** and the optional **update check**.
- **No account / no cloud video path:** there is no login and no server-side video. Streams are muxed
  on-device and sent **directly** to each platform you configured; there is no restream relay we run.
- **Capture surface:** screen / window / game / webcam frames stay **in-process** and go only to the
  recording file (the folder you choose), the streams you configured, and the virtual camera. The
  live preview is served over an **app-private custom URI scheme** (`preview://`) inside the webview —
  no localhost socket, no temp files. OS capture permissions (macOS Screen Recording / Camera) are
  requested honestly and a denial is surfaced, never worked around. The unavoidable OS-capture
  `unsafe` (DXGI/WGC/ScreenCaptureKit) is isolated behind small, audited modules in `crates/capture`
  (the Linux path is entirely safe Rust); the rest of the core is `#![forbid(unsafe_code)]`.
- **Audio surface:** captured audio (microphone / line-in and desktop / system audio) stays **on this
  machine** and goes **only** to the mixer, the **monitor** output device you pick, and the **recording
  file** — it is never transmitted anywhere. The whole DSP engine (mixing, FFT/spectral denoise,
  gate/comp/limiter/EQ, LUFS, resampler) is **owned** and `#![forbid(unsafe_code)]`; `cpal` supplies only
  device I/O. Desktop-audio capture is **honest per OS** and never silently installs a system component
  (Windows WASAPI loopback; Linux monitor devices; macOS a user-installed virtual device). The
  **monitor-device name** persisted in settings is length/shape-validated, and **push-to-talk /
  push-to-mute hotkeys** are parsed as accelerators and **validated before** they enter the model or are
  registered — a global shortcut only mutes/unmutes a source (`tauri-plugin-global-shortcut`), never runs
  code or reads files, and registration failures (e.g. Wayland) are surfaced honestly, never fatal.
- **Stream keys / service credentials:** stored **locally** in your profile (the OS config dir), masked
  in the UI, and sent **only** to the streaming service you are broadcasting to. They are never
  transmitted anywhere else and never logged.
- **WebSocket remote-control API:** **off by default**. When enabled it binds to **`127.0.0.1`** by
  default (LAN exposure is an explicit opt-in), is **password-authenticated**, validates every command,
  and **cannot read arbitrary files**. Disabled means the port is closed. Treat the password like any
  credential; prefer loopback-only unless you specifically need LAN control.
- **ffmpeg download (on demand, not bundled):** the patent-encumbered wire codecs are provided by
  **ffmpeg**, **fetched on demand** over **TLS** (rustls) from a **per-OS pinned URL** — a hardcoded
  literal, no path-traversal input — to a per-user cache; the download is streamed to a temp path.
  **Integrity:** each pinned build carries a **SHA-256 baked into the source** (cross-checked against the
  publisher's own published checksum at pin time), and the archive is **verified against that hash before
  anything is extracted or executed** — a mismatch aborts the install and removes the partial file; an
  unverified binary is never run. Extraction pulls out the **single `ffmpeg` member only**, always to an
  app-chosen path — **archive-supplied paths are never used for writing** (zip-slip / tar-traversal
  inert). The fetch **never starts on its own** (an explicit click in the labeled Components panel). The
  owned **`freally-video`** lossless recording path needs none of this. (There are **no model downloads**
  — the product ships no AI/ML features.)
- **Decode/parse hardening:** any file read from an untrusted source (a `.frec` recording, a media file,
  a LUT, a mask) has its allocations **bounded by validated header fields** so a malformed or hostile
  file fails cleanly (an error, not a panic) instead of exhausting memory; the owned codec + FLZ stage
  are `#![forbid(unsafe_code)]`.
- **Subprocess execution:** where ffmpeg is driven as a subprocess, arguments are passed as an **argv
  vector (no shell)** — user-controlled values (media paths, encoder ids, output paths) are separate
  argv entries, and encoder ids are **shape-validated** (`[A-Za-z0-9_-]`) before reaching `-c:v`. Child
  processes run with **hard timeouts + drained pipes** and, on Windows, **no console window**. Recording
  audio tracks ride **loopback-only (`127.0.0.1`) sockets** that accept **exactly one** connection then
  close. The **remux** and **recordings-list** actions are **confined to the recordings folder** (a
  canonicalized parent-directory check) so the webview command surface cannot reach arbitrary files.
- **Third-party components** (see [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md)) carry their own
  advisories; `cargo audit` and `cargo deny` run in CI on every push, and documented-ignore entries in
  `deny.toml` are limited to unmaintained-class advisories with no reachable vulnerability.
- **No secrets** are bundled or logged; `.env` and config files are treated as sensitive.

Thank you for helping keep Freally Capture and its users safe.
