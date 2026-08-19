//! Windows implementation of the game-capture hook.
//!
//! AUDITED `unsafe`: this module tree is the crate's entire FFI surface —
//! file mappings, D3D/DXGI interop, and the vtable patch. Every `unsafe` block
//! stays small and local; nothing outside `win` uses `unsafe`.
#![allow(unsafe_code)]

pub mod entry;
pub mod hook;
pub mod shmem;
pub mod texture;
pub mod vtable;
