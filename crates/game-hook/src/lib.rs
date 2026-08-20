//! CAP-N78 — the **game-capture hook** (Windows), protocol v1.
//!
//! Injected into a game the user explicitly opted in for, this DLL patches the
//! COM vtable entries for `IDXGISwapChain::Present` **and
//! `IDXGISwapChain1::Present1`**, copies each presented back buffer into a
//! **named shared texture**, and publishes geometry + a frame counter through a
//! named shared-memory control block.
//!
//! Both slots matter: DXGI 1.2+ **flip-model** titles — most of what runs in
//! exclusive fullscreen today — present through `Present1` and never touch the
//! `Present` slot at all. A D3D12 title presents through the same vtable but its
//! back buffers are `ID3D12Resource`, which this copy path does not implement;
//! that case publishes an honest "not D3D11" block so the app falls back to
//! Window Capture immediately instead of waiting out its first-frame timeout. The app side
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
//! patches, and restores those vtable slots on unload — explicitly from
//! `DLL_PROCESS_DETACH`, and only if they still hold our functions, so an
//! overlay that hooked after us is never silently unhooked.

pub mod protocol;

#[cfg(windows)]
pub mod win;

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
