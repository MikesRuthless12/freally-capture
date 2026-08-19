//! The Windows COM implementation of the frame-server source.
//!
//! Split so the intricate parts stay legible:
//! - [`registration`] — the per-user self-register / unregister + the DLL exports
//!   (`DllGetClassObject`, `DllRegisterServer`, …) the OS calls.
//! - [`shmem`] — opening the app's shared transport region by name.
//! - [`source`] — the `IMFMediaSource` + `IMFMediaStream` objects the Frame
//!   Server drives.
//!
//! AUDITED `unsafe`: COM interop + the shared-memory mapping. Every block stays
//! small and local; the pure logic lives in `crate::reader`.
#![allow(unsafe_code)]

pub mod registration;
pub mod shmem;
pub mod source;

use fcap_stream::vcam::transport::VCAM_SOURCE_CLSID_U128;
use windows_core::GUID;

/// The source's CLSID — built from the same shared constant the app's
/// `fcap_vcam_win::VCAM_SOURCE_CLSID` uses for `MFCreateVirtualCamera`, so the
/// registration and the lookup cannot drift.
pub const VCAM_SOURCE_CLSID: GUID = GUID::from_u128(VCAM_SOURCE_CLSID_U128);

/// The camera's default geometry when the app has not published a header yet.
pub const DEFAULT_WIDTH: u32 = 1920;
/// Default height (see [`DEFAULT_WIDTH`]).
pub const DEFAULT_HEIGHT: u32 = 1080;
/// The frame rate the source advertises. The app paces its own pushes; this is
/// the nominal MF media-type rate.
pub const DEFAULT_FPS: u32 = 30;
