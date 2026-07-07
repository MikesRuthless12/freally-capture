//! The per-item video filter chain.
//!
//! Each [`crate::SceneItem`] carries an ordered `Vec<Filter>`; the compositor
//! applies enabled filters top-to-bottom on the GPU before the item is
//! composed. Parameters are plain serde data — the GPU uniforms mirror them
//! in `fcap-compositor`.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::source::Rgba;

/// Stable identity of one filter instance (UI list keys, reorder targets).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FilterId(pub Uuid);

impl FilterId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for FilterId {
    fn default() -> Self {
        Self::new()
    }
}

/// How a [`FilterKind::Mask`] image is read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MaskMode {
    /// The mask image's alpha channel multiplies the item's alpha.
    #[default]
    Alpha,
    /// The mask image's luminance multiplies the item's alpha.
    Luma,
}

fn default_similarity() -> f32 {
    0.4
}

fn default_smoothness() -> f32 {
    0.08
}

fn default_spill() -> f32 {
    0.1
}

fn default_one() -> f32 {
    1.0
}

fn default_blur_radius() -> f32 {
    8.0
}

fn default_sharpen() -> f32 {
    0.25
}

/// One filter's kind + parameters. Tagged by `type` in JSON.
///
/// Ranges are documented per field; the compositor clamps defensively, the UI
/// keeps its controls inside the same bounds. `rename_all_fields` is
/// load-bearing: without it `hueShift`/`speedX`/`speedY` from the UI would
/// silently parse to their defaults (variant-only renaming).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum FilterKind {
    /// Key out a chroma color (green screen).
    ChromaKey {
        #[serde(default = "Rgba::default_key")]
        key: Rgba,
        /// Chroma distance that counts as "the key color", 0..=1.
        #[serde(default = "default_similarity")]
        similarity: f32,
        /// Width of the soft edge past `similarity`, 0..=1.
        #[serde(default = "default_smoothness")]
        smoothness: f32,
        /// Strength of key-color spill suppression, 0..=1.
        #[serde(default = "default_spill")]
        spill: f32,
    },
    /// Gamma / brightness / contrast / saturation / hue / opacity.
    ColorCorrection {
        /// -3..=3; 0 = neutral (applied as pow(2, -gamma)).
        #[serde(default)]
        gamma: f32,
        /// -1..=1 additive.
        #[serde(default)]
        brightness: f32,
        /// -1..=1; 0 = neutral.
        #[serde(default)]
        contrast: f32,
        /// 0..=4; 1 = neutral.
        #[serde(default = "default_one")]
        saturation: f32,
        /// Degrees, -180..=180.
        #[serde(default)]
        hue_shift: f32,
        /// 0..=1 multiplies the item's alpha.
        #[serde(default = "default_one")]
        opacity: f32,
    },
    /// A .cube color lookup table.
    Lut {
        #[serde(default)]
        path: String,
        /// Blend between the original (0) and the LUT'd color (1).
        #[serde(default = "default_one")]
        amount: f32,
    },
    /// Gaussian blur.
    Blur {
        /// Radius in source pixels, 0..=64.
        #[serde(default = "default_blur_radius")]
        radius: f32,
    },
    /// Multiply the item's alpha by an image mask.
    Mask {
        #[serde(default)]
        path: String,
        #[serde(default)]
        mode: MaskMode,
        #[serde(default)]
        invert: bool,
    },
    /// Key out an arbitrary color by RGB distance (a non-green backdrop).
    ColorKey {
        #[serde(default = "Rgba::default_key")]
        key: Rgba,
        /// RGB distance that counts as "the key color", 0..=1.
        #[serde(default = "default_similarity")]
        similarity: f32,
        /// Width of the soft edge past `similarity`, 0..=1.
        #[serde(default = "default_smoothness")]
        smoothness: f32,
    },
    /// Key on brightness: pixels outside `luma_min..=luma_max` go
    /// transparent (soft edges via `smoothness`).
    LumaKey {
        /// 0..=1; pixels darker than this key out.
        #[serde(default)]
        luma_min: f32,
        /// 0..=1; pixels brighter than this key out.
        #[serde(default = "default_one")]
        luma_max: f32,
        /// Soft edge width, 0..=1.
        #[serde(default = "default_smoothness")]
        smoothness: f32,
    },
    /// Delay this source's video by N ms (sync a source to audio). Applied
    /// at the source-frame stage behind a bounded buffer — capped at 500 ms
    /// because raw frames are memory (the same cap OBS uses, honestly).
    RenderDelay {
        #[serde(default)]
        delay_ms: u32,
    },
    /// Unsharp-mask sharpening.
    Sharpen {
        /// 0..=2; 0 = off.
        #[serde(default = "default_sharpen")]
        amount: f32,
    },
    /// Endless scroll (ticker/credits); content wraps.
    Scroll {
        /// Horizontal speed in source px/second (negative = leftward content flow).
        #[serde(default)]
        speed_x: f32,
        /// Vertical speed in source px/second.
        #[serde(default)]
        speed_y: f32,
    },
    /// Cut pixels off the item's edges (after the source's own crop).
    Crop {
        #[serde(default)]
        left: u32,
        #[serde(default)]
        top: u32,
        #[serde(default)]
        right: u32,
        #[serde(default)]
        bottom: u32,
    },
}

impl Rgba {
    fn default_key() -> Self {
        // Standard green-screen green.
        Rgba::new(0x00, 0xff, 0x00, 0xff)
    }
}

impl FilterKind {
    /// Machine name of this filter type (mirrors the serde tag).
    pub fn type_name(&self) -> &'static str {
        match self {
            FilterKind::ChromaKey { .. } => "chromaKey",
            FilterKind::ColorCorrection { .. } => "colorCorrection",
            FilterKind::Lut { .. } => "lut",
            FilterKind::Blur { .. } => "blur",
            FilterKind::Mask { .. } => "mask",
            FilterKind::ColorKey { .. } => "colorKey",
            FilterKind::LumaKey { .. } => "lumaKey",
            FilterKind::RenderDelay { .. } => "renderDelay",
            FilterKind::Sharpen { .. } => "sharpen",
            FilterKind::Scroll { .. } => "scroll",
            FilterKind::Crop { .. } => "crop",
        }
    }

    /// Human display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            FilterKind::ChromaKey { .. } => "Chroma Key",
            FilterKind::ColorCorrection { .. } => "Color Correction",
            FilterKind::Lut { .. } => "Apply LUT",
            FilterKind::Blur { .. } => "Blur",
            FilterKind::Mask { .. } => "Image Mask",
            FilterKind::ColorKey { .. } => "Color Key",
            FilterKind::LumaKey { .. } => "Luma Key",
            FilterKind::RenderDelay { .. } => "Render Delay",
            FilterKind::Sharpen { .. } => "Sharpen",
            FilterKind::Scroll { .. } => "Scroll",
            FilterKind::Crop { .. } => "Crop",
        }
    }
}

fn default_enabled() -> bool {
    true
}

/// One filter instance in an item's chain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    #[serde(default)]
    pub id: FilterId,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(flatten)]
    pub kind: FilterKind,
}

impl Filter {
    /// A new enabled filter with a fresh id.
    pub fn new(kind: FilterKind) -> Self {
        Self {
            id: FilterId::new(),
            enabled: true,
            kind,
        }
    }
}
