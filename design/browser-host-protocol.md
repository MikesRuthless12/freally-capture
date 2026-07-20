# Browser host protocol — v1 (CAP-N77)

**Status: normative.** `crates/sources/src/browser.rs` implements the app side; the
`freally-browser-host` executable implements the host side. Version bumps change the magic.

## Why a host process

Per `design/browser-source-spike.md` (Option A, accepted): CEF offscreen rendering is the only
product-grade route to "arbitrary URL → RGBA frames with transparency at a chosen fps". CEF must
never run in-process with the Tauri app (message-loop ownership, subprocess model, WebView2
coexistence) and must never be bundled (the charter). So:

- `fcap_encode::cef` fetches + hash-verifies + extracts the **CEF runtime** on demand
  (shipped 0.90.0 — the ffmpeg component pattern; Spotify CDN index).
- The **browser host** is a small executable, built in CI against the **pinned** CEF version,
  published as a release asset, and installed by the same component flow into
  `<cache>/cef/host/freally-browser-host[.exe]` (a sibling of `current/<dist>/`).
- The app spawns one host per Browser source and reads frames off a pipe — the exact shape the
  Media source's ffmpeg decode pump already trusts (`read_exact_or_end`, latest-wins channel).

## Invocation

```
freally-browser-host --url <http(s) URL> --width <u32> --height <u32> --fps <u32>
                     [--transparent] --cef <extracted CEF dist dir>
```

- The host `dlopen`s/loads libcef from `--cef` (`Release/` + `Resources/`); it must refuse a
  version it was not built against (CEF has no ABI stability across majors) with exit code 3.
- `--transparent` renders on a fully transparent page background; otherwise white.
- The host runs windowless (OSR), GPU-accelerated where CEF allows, audio muted in v1
  (audio lands in protocol v2 as a second pipe — see Future).

## Stream (host stdout → app)

1. **Header, 16 bytes:** magic `FBH1` (4) + `u32le width` + `u32le height` + `u32le fps` — the
   host's ACTUAL values after its own clamping; the stream is the truth, the app trusts it
   (clamped app-side to 7680×4320 defensively).
2. **Frames:** fixed-size `width * height * 4` byte RGBA (straight alpha, top-down rows,
   stride = width*4), written whenever CEF paints, paced at most `fps`. No per-frame framing —
   fixed size makes torn reads impossible and the pump trivial.
3. **EOF** = the host exited; the app's standard auto-recover restarts the session.

## Exit codes (host)

| code | meaning |
|---|---|
| 0 | clean shutdown (stdin closed / SIGTERM) |
| 2 | bad arguments |
| 3 | CEF runtime missing/incompatible with this host build |
| 4 | CEF initialization failed |
| 5 | navigation hard-failed (DNS/refused) after retries |

One line on stderr explains any non-zero exit (the app logs it verbatim).

## Rules inherited from the charter

- **http/https only in v1** — `file://` UNC forms would stat network paths (the CAP-M16 NTLM
  rule); local files already play through Media/Image. Enforced BOTH app-side
  (`browser::validate_url`) and host-side.
- No telemetry, no accounts; the page's own network traffic is the user's choice, exactly like
  OBS's browser source.
- Chromium CVEs apply from the day this ships: the component panel surfaces the CEF version and
  the fetcher must be re-pointed at a pinned, host-matched build (NOT blindly `index.json`
  latest) once the host exists — tracked in the Phase 10 DoD.

## Host build (CI, follow-up)

`crates/browser-host` (workspace member, feature-gated): `--features cef` links the real backend
against the pinned CEF SDK in CI (per-OS runners, the same 3-OS matrix as release.yml); built
WITHOUT the feature it exits 3 with an honest message (so local workspace builds never need the
SDK). Release assets: `freally-browser-host-{win-x64,mac-arm64,mac-x64,linux-x64}` + sha256 —
consumed by the extended component installer.

## Future (v2)

- **Audio:** a `--audio-pipe` fd/named-pipe carrying `f32le` interleaved stereo @48k in fixed
  1024-frame blocks; joins the mixer as its own strip (then `Browser` gains `has_audio`).
- Input events (interact-with-the-page from Properties) and per-source CSS/JS injection.
