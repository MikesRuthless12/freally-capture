//! # fcap-audio
//!
//! The owned audio engine: `cpal` capture (mic + per-OS system/loopback
//! audio), a mixing graph with **up to 6 tracks** and per-source assignment,
//! the **owned classic-DSP filter set** (spectral denoise — no ML — noise
//! gate, compressor, limiter, 3-band EQ, gain, sidechain ducker), per-source
//! sync offset, push-to-talk/mute gating, monitoring, and **BS.1770 LUFS**
//! metering.
//!
//! Internally everything runs as interleaved **stereo f32 at 48 kHz** in
//! 10 ms blocks; capture streams resample into that clock. macOS system audio
//! is guided honestly (ScreenCaptureKit audio later, or a virtual device —
//! never silently installed).

#![forbid(unsafe_code)]

pub mod capture;
pub mod delay;
pub mod devices;
pub mod dsp;
pub mod engine;
pub mod fft;
pub mod filters;
pub mod graph;
pub mod lufs;
pub mod media_hub;
pub mod meter;
pub mod monitor;
pub mod resample;
pub mod vst;

pub use devices::{
    list_input_devices, list_loopback_devices, list_output_devices, AudioDeviceInfo,
};
pub use engine::{
    AudioEngine, EngineSnapshot, RecordTap, SourceConfig, SourceSnapshot, SourceState,
};
pub use graph::{MixerCore, StripControl};
pub use meter::Levels;

use thiserror::Error;

/// Why a device/stream operation failed. `code()` is the stable wire tag the
/// UI keys its guidance on.
#[derive(Debug, Clone, Error)]
pub enum AudioError {
    #[error("audio device not found: {0}")]
    DeviceNotFound(String),
    #[error("no audio devices are available")]
    NoDevices,
    /// Desktop audio isn't capturable here without help — the message is the
    /// per-OS honest guidance.
    #[error("{0}")]
    NoLoopback(String),
    #[error("unsupported audio configuration: {0}")]
    Unsupported(String),
    #[error("audio backend error: {0}")]
    Backend(String),
}

impl AudioError {
    pub fn code(&self) -> &'static str {
        match self {
            AudioError::DeviceNotFound(_) => "deviceNotFound",
            AudioError::NoDevices => "noDevices",
            AudioError::NoLoopback(_) => "noLoopback",
            AudioError::Unsupported(_) => "unsupported",
            AudioError::Backend(_) => "backend",
        }
    }
}

/// What a source captures: a plain input device, desktop audio
/// ("loopback" — resolved per OS, see [`capture::open_capture`]), or a
/// media source's decoded audio (fed through [`media_hub`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputSpec {
    Input { device_id: String },
    Loopback { device_id: String },
    Media { id: String },
}

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The engine's internal sample rate.
pub const SAMPLE_RATE: u32 = 48_000;

/// The engine is stereo end-to-end (mono capture is upmixed).
pub const CHANNELS: usize = 2;

/// Frames per mix block (10 ms at 48 kHz).
pub const BLOCK_FRAMES: usize = 480;

#[cfg(test)]
mod tests {
    use super::VERSION;

    #[test]
    fn version_is_a_semver_triple() {
        assert_eq!(
            VERSION.split('.').count(),
            3,
            "workspace version should be MAJOR.MINOR.PATCH"
        );
    }
}
