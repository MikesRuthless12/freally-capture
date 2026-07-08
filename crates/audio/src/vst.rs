//! Optional VST2/3 audio plugins (Phase 8, TASK-804) — **scoped, not shipped.**
//!
//! VST hosting is deliberately deferred, and this module is the honest seam that
//! says why rather than a fake toggle:
//!
//! - **VST2** — Steinberg withdrew the VST2 SDK and no longer licenses it;
//!   shipping a VST2 host means redistributing an SDK we cannot legally obtain.
//! - **VST3** — dual-licensed GPLv3 *or* Steinberg's proprietary license.
//!   GPLv3 is incompatible with this project's proprietary distribution, and the
//!   proprietary route requires signing Steinberg's agreement — a licensing
//!   cost, exactly the kind the charter's "$0, nothing shipped we can't license"
//!   rule declines.
//!
//! So VST stays **behind this interface + flag**, always reporting unavailable
//! with the honest reason, and pointing at the owned alternative: Freally
//! Capture already ships a full classic-DSP filter set (spectral denoise, noise
//! gate, compressor, limiter, 3-band EQ, gain, sidechain ducking) that covers
//! the common plugin use cases with zero licensing entanglement. If a
//! permissively-licensed host path appears, it slots in behind this same type.

/// Why VST hosting is unavailable — surfaced verbatim in the UI so the boundary
/// is never a silent no-op.
pub const VST_STATUS: &str = "VST2/3 plugins are not available: the VST2 SDK is no longer licensed \
     by Steinberg, and VST3 is GPLv3-or-proprietary — both conflict with a $0, \
     no-extra-license build. Use the built-in filters (denoise, gate, \
     compressor, limiter, EQ, gain, ducking), which need no plugin and stay on \
     this machine.";

/// The VST support state. Only one variant today; kept as an enum so a future
/// permissively-licensed host lights up without changing callers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VstSupport {
    /// Hosting is unavailable for a licensing reason (the string is `VST_STATUS`).
    Unavailable(&'static str),
}

/// The current VST support — always `Unavailable` with the honest reason.
pub fn support() -> VstSupport {
    VstSupport::Unavailable(VST_STATUS)
}

/// Whether any VST hosting is available (always false today; the honest flag).
pub fn is_available() -> bool {
    !matches!(support(), VstSupport::Unavailable(_))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vst_is_unavailable_with_an_honest_reason() {
        assert!(!is_available());
        let VstSupport::Unavailable(reason) = support();
        assert!(reason.contains("VST"));
        assert!(reason.contains("license") || reason.contains("licensed"));
        // Points at the shipped alternative rather than dead-ending.
        assert!(reason.to_lowercase().contains("filter"));
    }
}
