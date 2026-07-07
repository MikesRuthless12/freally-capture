//! The virtual camera — the honest state (Phase 6, TASK-611).
//!
//! A real virtual camera is an **OS driver component**, not in-process
//! code: other apps (Zoom, Discord, a browser) enumerate cameras from the
//! OS, so something must register there on our behalf. That component is
//! its own milestone (signing included) — this module ships the parts that
//! are honestly buildable *now*: the feed-selection model and the output
//! seam the driver milestone plugs into, so the app-side plumbing (and its
//! settings/UI) never has to change shape when the driver lands.
//!
//! ## What each OS needs (documented honestly)
//!
//! - **Windows 11+**: `MFCreateVirtualCamera` — a **signed COM media-source
//!   DLL** registered as a Frame Server camera. Works without a kernel
//!   driver, but the DLL must be present + registered (installer work) and
//!   code-signed to be trusted in practice.
//! - **Windows 10**: the DirectShow route (what OBS ships) — a signed
//!   DirectShow filter, registered for both 32/64-bit hosts.
//! - **macOS**: a **CoreMediaIO camera extension** (`CMIOExtension`,
//!   macOS 12.3+), sandboxed, notarized, installed by the .app. CMIO
//!   extensions carry **video only** — macOS has no virtual *microphone*
//!   API, so "audio to the vcam" is honestly **not possible** there
//!   without a separate audio driver (out of scope by design).
//! - **Linux**: `v4l2loopback` (a kernel module the USER installs from
//!   their distro; we never install kernel modules) — we open the loopback
//!   node and write frames. Audio rides a PulseAudio/PipeWire **null sink**
//!   the user selects in the target app.
//!
//! ## Audio (VC-5), honestly
//!
//! "Audio to the virtual camera" is a misnomer everywhere: cameras are
//! video devices. What apps actually consume is a paired virtual
//! *microphone* — possible on Windows (a paired audio render/capture
//! driver) and Linux (null sink), **not** on macOS (no public API). The
//! feed model below already carries the audio track choice so the driver
//! milestone only implements transport.

/// What the virtual camera would output — the **program**, the vertical
/// canvas, or one **single source** (VC-4: share just a camera or a
/// window into a meeting while the program keeps compositing elsewhere).
/// The source is its scene-model id (a UUID string) — this crate stays
/// scene-model-agnostic.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VcamFeed {
    /// The composed program canvas.
    #[default]
    Program,
    /// The second (vertical) canvas, when configured.
    Vertical,
    /// One source's raw frames (by scene-model id), not the composition.
    Source(String),
}

/// The transport seam the driver milestone implements per OS. The app side
/// pushes frames exactly like it feeds the recorder/stream; `start`
/// registers the OS-side camera, `stop` unregisters it.
pub trait VcamOutput: Send {
    /// Bring the OS-side camera up at the given geometry.
    fn start(&mut self, width: u32, height: u32, fps: u32) -> Result<(), String>;
    /// Push one RGBA frame (the newest wins; implementations pace).
    fn push_frame(&mut self, rgba: &[u8], width: u32, height: u32) -> Result<(), String>;
    /// Tear the OS-side camera down.
    fn stop(&mut self) -> Result<(), String>;
}

/// Whether a driver-backed [`VcamOutput`] exists on this build/OS. `false`
/// everywhere today — the UI keeps its button disabled with the honest
/// tooltip instead of pretending.
pub fn driver_available() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_driver_ships_yet_and_the_ui_must_say_so() {
        assert!(
            !driver_available(),
            "flip this only when a real OS driver component lands"
        );
    }
}
