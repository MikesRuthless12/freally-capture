//! Optional VST audio plugins — the honest seam for the VST host path.
//!
//! **Licensing update (Steinberg, 29 Oct 2025):** the **VST 3.8 SDK is now
//! MIT-licensed** (previously dual GPLv3-or-proprietary). That removes the old
//! licensing blocker entirely — **VST3 is now a permissively-licensed, $0-clean
//! host path, on the same footing as CLAP** (see [`crate::claphost`]). So the
//! reason VST hosting isn't live yet is **not** licensing; it is the same work
//! CLAP needs: a **crash-isolated host process** that loads and runs each
//! plugin (so a bad plugin can't take down the mix) with its own GUI window.
//! That integration is in progress; this module is the honest interface until
//! it lands, not a fake toggle.
//!
//! - **VST3** — MIT since the VST 3.8 SDK (2025-10-29). Hostable with no
//!   licensing cost; pending the crash-isolated host integration.
//! - **VST2** — Steinberg withdrew the legacy VST2 SDK and no longer
//!   distributes it, so a fresh VST2 host can't obtain the SDK. VST3 is the
//!   path forward.
//!
//! Meanwhile Freally Capture ships a full owned classic-DSP filter set
//! (spectral denoise, gate, compressor, limiter, parametric EQ, de-esser,
//! rumble guard, sidechain ducking) that covers the common plugin needs with
//! nothing to install and no licensing entanglement.

/// The honest status of the VST host path — surfaced verbatim in the UI so the
/// boundary is never a silent no-op.
pub const VST_STATUS: &str =
    "VST3 is now MIT-licensed (Steinberg's VST 3.8 SDK, October 2025), so it's a \
     free, $0-clean plugin path like CLAP — no licensing blocker. Live hosting \
     runs each plugin in a separate, crash-isolated process (so a bad plugin \
     can't take down the mix) with its own GUI; that host-process integration is \
     in progress. Legacy VST2 stays unavailable — Steinberg withdrew its SDK. \
     Meanwhile the built-in filters (denoise, gate, compressor, limiter, \
     parametric EQ, de-esser, rumble guard, ducking) need no plugin and stay on \
     this machine.";

/// The VST support state. Only one variant today; kept as an enum so the host
/// integration lights it up without changing callers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VstSupport {
    /// Hosting isn't wired yet (the string is `VST_STATUS`) — pending the
    /// crash-isolated host process, **not** a licensing block anymore.
    Unavailable(&'static str),
}

/// The current VST support — `Unavailable` until the host process is wired.
pub fn support() -> VstSupport {
    VstSupport::Unavailable(VST_STATUS)
}

/// Whether any VST hosting is available (false until the host lands).
pub fn is_available() -> bool {
    !matches!(support(), VstSupport::Unavailable(_))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vst_status_reflects_the_mit_relicense() {
        assert!(!is_available());
        let VstSupport::Unavailable(reason) = support();
        assert!(reason.contains("VST3"));
        // The current, correct reason: MIT-licensed now, host integration pending.
        assert!(reason.contains("MIT"));
        assert!(reason.to_lowercase().contains("crash-isolated"));
        // Still points at the shipped owned alternative rather than dead-ending.
        assert!(reason.to_lowercase().contains("filter"));
    }
}
