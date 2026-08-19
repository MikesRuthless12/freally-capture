//! CAP-N78 — the **game-capture hook** (Windows), protocol v1.
//!
//! Injected into a game the user explicitly opted in for, this DLL patches the
//! COM vtable entry for `IDXGISwapChain::Present`, copies each presented back
//! buffer into a **named shared texture**, and publishes geometry + a frame
//! counter through a named shared-memory control block. The app side
//! (`fcap_capture::win::hook`) opens both by name and reads frames out.
//! `design/game-hook-protocol.md` is normative.
//!
//! **This exists only for what WGC cannot reach** — exclusive-fullscreen
//! titles, and capture at the game's own present rate. Windows.Graphics.Capture
//! (`fcap_capture::win::wgc`) already handles borderless/windowed games with no
//! injection and no anti-cheat exposure, and stays the recommended path.
//!
//! The injection rules in the protocol doc are enforced by the *app* side, not
//! here: this DLL does nothing until it is loaded, and it is only ever loaded
//! after a per-title, per-executable opt-in with the blunt consent text shown.
//! It never phones home, never touches game memory outside the DXGI vtable it
//! patches, and restores that vtable on unload.

pub mod protocol;

#[cfg(windows)]
pub mod win;

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
