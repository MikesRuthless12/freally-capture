//! Shared bits of the static (non-streaming) sources: Image, Color, Text.
//!
//! Static sources produce one [`Frame`] when created or reconfigured — the
//! studio uploads it to the compositor once and re-renders only on settings
//! changes, so a text lower-third costs nothing per frame.

use std::time::Instant;

use fcap_capture::{Frame, PixelFormat};
use thiserror::Error;

/// The largest edge a static source may produce. Keeps a mistyped size or a
/// gigapixel photo from allocating the GPU into the ground; real captures are
/// far below this.
pub const MAX_STATIC_DIMENSION: u32 = 16_384;

/// Why a static source could not render.
#[derive(Debug, Clone, Error)]
pub enum StaticSourceError {
    #[error("could not read {path}: {message}")]
    Io { path: String, message: String },
    #[error("could not decode {path}: {message}")]
    Decode { path: String, message: String },
    #[error("no usable font was found ({0})")]
    NoFont(String),
    #[error("{what} is {size} px — the limit is {MAX_STATIC_DIMENSION}")]
    TooLarge { what: &'static str, size: u32 },
}

/// Wrap tightly-packed RGBA pixels as a capture-shaped [`Frame`].
pub(crate) fn rgba_frame(width: u32, height: u32, data: Vec<u8>) -> Frame {
    debug_assert_eq!(data.len(), (width as usize) * (height as usize) * 4);
    Frame {
        width,
        height,
        stride: width * 4,
        format: PixelFormat::Rgba8,
        data,
        captured_at: Instant::now(),
    }
}

pub(crate) fn check_dimension(what: &'static str, size: u32) -> Result<(), StaticSourceError> {
    if size == 0 || size > MAX_STATIC_DIMENSION {
        return Err(StaticSourceError::TooLarge { what, size });
    }
    Ok(())
}
