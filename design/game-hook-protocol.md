# Game-capture hook protocol — v1 (CAP-N78, Windows)

**Status: normative.** `crates/game-hook` is the injected producer; `fcap_capture::win::hook`
is the app-side consumer. Version bumps change the magic.

## Why a hook at all

Windows.Graphics.Capture (`win/wgc.rs`) already captures borderless/windowed games with **no
injection and no anti-cheat exposure**, and stays the recommended path. The hook exists only for
what WGC cannot reach: **exclusive-fullscreen** titles, and capture at the game's native present
rate without a desktop-composition round trip.

Because injection is genuinely risky, the rules below are part of the protocol, not policy notes:

- **Never inject silently.** One explicit, per-title opt-in, stored per executable, after the
  blunt consent text in `fcap_capture::game::risk_warning()` has been shown and acknowledged.
- **Never inject into a process the user did not name.** No scanning-and-attaching, no
  "all games" mode, no persistence across reinstalls of the title.
- **Failure degrades, never crashes.** A title that refuses the hook (anti-cheat, a protected
  process, a mismatched D3D version) falls back to WGC window capture with an honest message.

## Shape

The hook DLL is loaded into the game, patches the **COM vtable** entry for `Present`, and on
each present copies the back buffer into a **named shared texture**. The app
opens that texture by name and reads it. No handle duplication, no code in the app's own render
path, and the game keeps presenting normally whether or not anyone is listening.

Vtable patching (not an inline trampoline) is deliberate: it is a single pointer write inside
DXGI's own vtable, it is trivially reversible on unload, and it needs no executable-page
allocation — the pattern most likely to be read as ordinary interop rather than as evasion.

## Control block (named shared memory)

`Local\freally-game-hook-<pid>`, a fixed 64-byte C-layout block, little-endian:

| offset | field | meaning |
|---|---|---|
| 0 | `magic: [u8;4]` | `FGH1` |
| 4 | `version: u32` | `1` |
| 8 | `width: u32` | back-buffer width, as presented |
| 12 | `height: u32` | back-buffer height |
| 16 | `format: u32` | the `DXGI_FORMAT` of the shared texture |
| 20 | `flags: u32` | bit 0 = producer alive, bit 1 = geometry changed |
| 24 | `frame_index: u64` | incremented after each successful publish |
| 32 | `present_api: u32` | 11 = D3D11, 12 = D3D12 |
| 36 | `_reserved: [u32;7]` | zero |

`frame_index` is the only liveness signal the consumer needs: it advances iff the game presented
**and** the copy succeeded. A stalled index means the game is not presenting (alt-tabbed, paused,
loading) — not an error.

## Shared texture

`Local\freally-game-hook-tex-<pid>`, created by the producer with
`D3D11_RESOURCE_MISC_SHARED_NTHANDLE | D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX` and published via
`IDXGIResource1::CreateSharedHandle(..., Some(name))`. The consumer opens it with
`ID3D11Device1::OpenSharedResourceByName`.

**Keyed mutex:** producer acquires key `0`, writes, releases key `1`; consumer acquires `1`,
copies to its own staging texture, releases `0`. A consumer that dies holding the key is why the
producer's acquire uses a short timeout and simply skips the frame — the game must never stall
because the capture app went away.

## Lifecycle

1. App shows the risk text, records a per-executable opt-in, and injects the DLL.
2. `DllMain` does **no** D3D work (loader lock); it spawns the install thread.
3. The install thread creates a hidden dummy swapchain, reads its vtable, and patches `Present`
   (index 8), saving the original. `Present1` (index 22) is **not** hooked in this build: titles
   that present exclusively through it fall back to Window Capture.
4. First present: create the shared texture sized to the back buffer, publish the control block.
5. Each present: copy back buffer → shared texture, bump `frame_index`, then **always** call the
   original `Present` — the game's own presentation is never altered or delayed beyond the copy.
6. Geometry change (resize / fullscreen toggle): recreate the texture, set bit 1, bump the index.
7. Unload: restore the original vtable pointers, clear bit 0, release everything.

## Consumer rules

- A missing control block or a `frame_index` that never advances is **not** an error — the app
  reports "waiting for the game to present" and keeps the WGC fallback available.
- The consumer never writes to the game's memory and never calls into the game. It only opens a
  named texture and copies out of it.
- Frames reach the compositor through the same latest-wins `CaptureSession` channel as every
  other capture path.
