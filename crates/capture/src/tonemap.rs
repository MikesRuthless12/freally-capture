//! HDR→SDR tone-mapping (CAP-N74): deterministic operators for FP16 scRGB
//! desktop frames — plain clip, maxRGB, Reinhard, and a BT.2408-style knee —
//! applied **CPU-side in the capture thread**, so every downstream consumer
//! keeps seeing ordinary 8-bit SDR frames and the canvas stays SDR (the
//! charter's line). No ML anywhere; every operator is a closed formula.
//!
//! Config lives in a live registry keyed by the capture id (the same shape
//! as the media pause flags): a paper-white change retunes the very next
//! frame — no session restart, no model churn.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Which curve compresses HDR highlights into SDR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToneMapOperator {
    /// No compression: clamp at SDR white (what an unaware pipeline does —
    /// blown highlights, kept as the honest default until the user opts in).
    #[default]
    Clip,
    /// Reinhard on the max channel, colors scaled together — compresses
    /// highlights while preserving hue ratios.
    MaxRgb,
    /// Classic per-channel Reinhard `x/(1+x)` — softer, may desaturate
    /// extreme highlights.
    Reinhard,
    /// BT.2408-style knee: linear (untouched) below the knee, a smooth
    /// roll-off above — SDR-ish content stays exact.
    Bt2408,
}

impl ToneMapOperator {
    /// The wire names the settings store uses.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "clip" => Some(Self::Clip),
            "maxRgb" => Some(Self::MaxRgb),
            "reinhard" => Some(Self::Reinhard),
            "bt2408" => Some(Self::Bt2408),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Clip => "clip",
            Self::MaxRgb => "maxRgb",
            Self::Reinhard => "reinhard",
            Self::Bt2408 => "bt2408",
        }
    }
}

/// One capture's HDR→SDR mapping parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToneMapConfig {
    pub operator: ToneMapOperator,
    /// The luminance mapped to SDR reference white, in nits. scRGB encodes
    /// 1.0 = 80 nits, so a 200-nit paper white scales inputs by 80/200.
    pub paper_white_nits: f32,
}

impl Default for ToneMapConfig {
    fn default() -> Self {
        Self {
            operator: ToneMapOperator::Clip,
            paper_white_nits: 200.0,
        }
    }
}

fn registry() -> &'static Mutex<HashMap<String, ToneMapConfig>> {
    static REG: OnceLock<Mutex<HashMap<String, ToneMapConfig>>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Set (or reset, with the default config) a capture's tone-map. Live: the
/// capture thread reads this per frame.
pub fn set_tone_map(id: &str, config: ToneMapConfig) {
    registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .insert(id.to_string(), config);
}

/// The capture thread's per-frame lookup. Only the Windows DXGI FP16 path
/// reads it (HDR capture is Windows-only), so it is dormant elsewhere — the
/// registry writer `set_tone_map` and the tests stay cross-platform.
#[cfg_attr(not(target_os = "windows"), allow(dead_code))]
pub(crate) fn tone_map_for(id: &str) -> ToneMapConfig {
    registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(id)
        .copied()
        .unwrap_or_default()
}

/// IEEE 754 half → f32 (subnormals honored; NaN/negative clamp to 0, so a
/// hostile surface can never poison the frame).
fn half_to_f32(bits: u16) -> f32 {
    let sign = bits >> 15;
    let exp = (bits >> 10) & 0x1f;
    let mantissa = bits & 0x3ff;
    let value = match exp {
        0 => f32::from(mantissa) * 2f32.powi(-24), // subnormal
        0x1f => {
            if mantissa == 0 {
                f32::INFINITY
            } else {
                return 0.0; // NaN → black, never poison
            }
        }
        _ => (1.0 + f32::from(mantissa) / 1024.0) * 2f32.powi(i32::from(exp) - 15),
    };
    if sign == 1 {
        0.0 // scRGB negatives (wide gamut) clamp — the canvas is sRGB
    } else {
        value
    }
}

/// The 65,536-entry half→f32 table the per-pixel loop indexes.
fn half_lut() -> &'static [f32; 65536] {
    static LUT: OnceLock<Box<[f32; 65536]>> = OnceLock::new();
    LUT.get_or_init(|| {
        let mut table = vec![0f32; 65536].into_boxed_slice();
        for (bits, slot) in table.iter_mut().enumerate() {
            *slot = half_to_f32(bits as u16);
        }
        table.try_into().expect("exactly 65536 entries")
    })
}

/// Linear [0,1] → sRGB-encoded byte.
#[inline]
fn encode_srgb(linear: f32) -> u8 {
    let x = linear.clamp(0.0, 1.0);
    let encoded = if x <= 0.003_130_8 {
        12.92 * x
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    };
    (encoded * 255.0 + 0.5) as u8
}

/// The BT.2408-style knee: exact below `KNEE`, smooth roll-off above,
/// approaching (never exceeding) 1.
const KNEE: f32 = 0.75;
#[inline]
fn knee(x: f32) -> f32 {
    if x <= KNEE {
        x
    } else {
        let over = x - KNEE;
        KNEE + (1.0 - KNEE) * (over / (over + (1.0 - KNEE)))
    }
}

/// Map one linear scRGB pixel (1.0 = 80 nits) to SDR sRGB bytes.
pub fn map_scrgb(rgb: [f32; 3], config: &ToneMapConfig) -> [u8; 3] {
    let scale = 80.0 / config.paper_white_nits.clamp(80.0, 1000.0);
    let v = [
        (rgb[0] * scale).max(0.0),
        (rgb[1] * scale).max(0.0),
        (rgb[2] * scale).max(0.0),
    ];
    let mapped = match config.operator {
        ToneMapOperator::Clip => [v[0].min(1.0), v[1].min(1.0), v[2].min(1.0)],
        ToneMapOperator::Reinhard => [
            v[0] / (1.0 + v[0]),
            v[1] / (1.0 + v[1]),
            v[2] / (1.0 + v[2]),
        ],
        ToneMapOperator::MaxRgb => {
            // Reinhard on the max channel, all channels scaled together —
            // hue ratios survive exactly; SDR content dims slightly (use
            // BT.2408 when SDR exactness matters more than highlight room).
            let peak = v[0].max(v[1]).max(v[2]);
            if peak <= 1e-6 {
                v
            } else {
                let ratio = (peak / (1.0 + peak)) / peak;
                [v[0] * ratio, v[1] * ratio, v[2] * ratio]
            }
        }
        ToneMapOperator::Bt2408 => {
            let peak = v[0].max(v[1]).max(v[2]);
            if peak <= KNEE {
                v
            } else {
                let ratio = knee(peak) / peak;
                [v[0] * ratio, v[1] * ratio, v[2] * ratio]
            }
        }
    };
    [
        encode_srgb(mapped[0]),
        encode_srgb(mapped[1]),
        encode_srgb(mapped[2]),
    ]
}

/// Convert one row of RGBA-FP16 scRGB pixels into BGRA8 SDR. `src` holds
/// `width * 8` bytes, `dst` `width * 4`.
pub fn convert_row_fp16_to_bgra(src: &[u8], dst: &mut [u8], width: usize, config: &ToneMapConfig) {
    let lut = half_lut();
    for x in 0..width {
        let at = x * 8;
        let r = lut[u16::from_le_bytes([src[at], src[at + 1]]) as usize];
        let g = lut[u16::from_le_bytes([src[at + 2], src[at + 3]]) as usize];
        let b = lut[u16::from_le_bytes([src[at + 4], src[at + 5]]) as usize];
        let [sr, sg, sb] = map_scrgb([r, g, b], config);
        let out = x * 4;
        dst[out] = sb;
        dst[out + 1] = sg;
        dst[out + 2] = sr;
        dst[out + 3] = 255;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(operator: ToneMapOperator, nits: f32) -> ToneMapConfig {
        ToneMapConfig {
            operator,
            paper_white_nits: nits,
        }
    }

    #[test]
    fn half_decoding_matches_known_values() {
        assert_eq!(half_to_f32(0x0000), 0.0);
        assert!((half_to_f32(0x3c00) - 1.0).abs() < 1e-6, "1.0");
        assert!((half_to_f32(0x4000) - 2.0).abs() < 1e-6, "2.0");
        assert!((half_to_f32(0x3800) - 0.5).abs() < 1e-6, "0.5");
        assert_eq!(half_to_f32(0x7e00), 0.0, "NaN clamps to black");
        assert_eq!(half_to_f32(0xbc00), 0.0, "negatives clamp (sRGB canvas)");
        assert!(half_to_f32(0x0001) > 0.0, "subnormals decode");
    }

    #[test]
    fn clip_blows_highlights_and_operators_recover_them() {
        // A 4× overbright pixel at 80-nit paper white (scale 1).
        let hot = [4.0, 4.0, 4.0];
        let clip = map_scrgb(hot, &cfg(ToneMapOperator::Clip, 80.0));
        assert_eq!(clip, [255, 255, 255], "clip = blown white");
        let reinhard = map_scrgb(hot, &cfg(ToneMapOperator::Reinhard, 80.0));
        assert!(reinhard[0] < 255, "reinhard keeps highlight detail");
        let maxrgb = map_scrgb(hot, &cfg(ToneMapOperator::MaxRgb, 80.0));
        assert!(maxrgb[0] < 255, "maxRGB keeps highlight detail");
    }

    #[test]
    fn maxrgb_preserves_hue_ratios() {
        // An orange highlight: r=4, g=2, b=1 — after mapping the channel
        // ratios must survive (the whole point vs per-channel Reinhard).
        let mapped = map_scrgb([4.0, 2.0, 1.0], &cfg(ToneMapOperator::MaxRgb, 80.0));
        // Decode back to linear to compare ratios.
        let lin = |byte: u8| {
            let x = byte as f32 / 255.0;
            if x <= 0.040_45 {
                x / 12.92
            } else {
                ((x + 0.055) / 1.055).powf(2.4)
            }
        };
        let (r, g, b) = (lin(mapped[0]), lin(mapped[1]), lin(mapped[2]));
        assert!((r / g - 2.0).abs() < 0.05, "r:g stays 2:1, got {}", r / g);
        assert!((g / b - 2.0).abs() < 0.05, "g:b stays 2:1, got {}", g / b);
    }

    #[test]
    fn bt2408_knee_leaves_sdr_content_exact_and_is_continuous() {
        // Below the knee: bit-exact with clip (SDR content untouched).
        let sdr = [0.5, 0.25, 0.7];
        assert_eq!(
            map_scrgb(sdr, &cfg(ToneMapOperator::Bt2408, 80.0)),
            map_scrgb(sdr, &cfg(ToneMapOperator::Clip, 80.0))
        );
        // Continuity at the knee.
        let below = knee(KNEE - 1e-4);
        let above = knee(KNEE + 1e-4);
        assert!((above - below).abs() < 1e-3, "no step at the knee");
        // Monotonic and bounded.
        let mut last = 0.0;
        for step in 0..100 {
            let x = step as f32 * 0.1;
            let y = knee(x);
            assert!(y >= last && y <= 1.0 + 1e-6);
            last = y;
        }
    }

    #[test]
    fn paper_white_scales_brightness() {
        // The same 200-nit pixel (2.5 in scRGB): at 200-nit paper white it IS
        // reference white; at 400 it lands at half.
        let px = [2.5, 2.5, 2.5];
        let at_200 = map_scrgb(px, &cfg(ToneMapOperator::Clip, 200.0));
        let at_400 = map_scrgb(px, &cfg(ToneMapOperator::Clip, 400.0));
        assert_eq!(at_200, [255, 255, 255]);
        assert!(at_400[0] < 200, "doubling paper white dims the pixel");
    }

    #[test]
    fn row_conversion_swizzles_to_bgra() {
        // One red FP16 pixel: r=1.0 (0x3c00), g=b=0, a=1.
        let src = [0x00u8, 0x3c, 0, 0, 0, 0, 0, 0x3c];
        let mut dst = [0u8; 4];
        convert_row_fp16_to_bgra(&src, &mut dst, 1, &ToneMapConfig::default());
        // scRGB 1.0 = 80 nits < 200-nit paper white → dim red, blue slot 0.
        assert_eq!(dst[0], 0, "B");
        assert!(dst[2] > 100, "R landed in the red slot");
        assert_eq!(dst[3], 255, "opaque");
    }

    #[test]
    fn registry_round_trips_and_defaults() {
        assert_eq!(tone_map_for("tm-test-none"), ToneMapConfig::default());
        let config = cfg(ToneMapOperator::MaxRgb, 250.0);
        set_tone_map("tm-test-a", config);
        assert_eq!(tone_map_for("tm-test-a"), config);
        assert_eq!(
            ToneMapOperator::from_name("maxRgb"),
            Some(ToneMapOperator::MaxRgb)
        );
        assert_eq!(ToneMapOperator::from_name("nope"), None);
        assert_eq!(ToneMapOperator::Bt2408.name(), "bt2408");
    }
}
