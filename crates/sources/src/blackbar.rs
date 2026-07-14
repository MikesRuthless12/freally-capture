//! Auto black-bar crop (CAP-N72): find the letterbox/pillarbox bars a source
//! carries via row/column luma scanning — classic CV, no ML — so one click
//! crops them away.
//!
//! A line (row or column) counts as "bar" when nearly all of its sampled
//! pixels sit below a dark threshold. Two guards keep dark *scenes* from
//! being eaten (the DoD's false-positive case):
//! - a per-side cap (no more than 45% of an axis is ever called "bar"), and
//! - a content check — whatever remains inside the detected bars must
//!   actually be brighter than the bars, or the whole detection returns zero.

use fcap_capture::{Frame, PixelFormat};

/// Luma at or below this is "dark enough to be a bar" (0–255).
const BAR_LUMA: u32 = 18;
/// At least this share of a line's sampled pixels must be dark.
const BAR_SHARE: f32 = 0.98;
/// No side ever claims more than this share of its axis.
const MAX_SIDE_SHARE: f32 = 0.45;
/// The surviving content must average at least this luma, or the frame is
/// just dark (a night scene, a fade-out) and nothing is cropped.
const CONTENT_LUMA: u32 = 30;
/// Bars thinner than this are sensor noise, not letterboxing.
const MIN_BAR_PX: u32 = 2;
/// Sample every Nth pixel along a line (speed; bars are uniform anyway).
const SAMPLE_STRIDE: usize = 4;

/// Detected bar widths in source pixels — the values a transform crop takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DetectedBars {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

impl DetectedBars {
    pub fn is_zero(&self) -> bool {
        *self == DetectedBars::default()
    }
}

/// Rec. 601 luma from one pixel, channel order per the frame's format.
#[inline]
fn luma(px: &[u8], format: PixelFormat) -> u32 {
    let (r, g, b) = match format {
        PixelFormat::Bgra8 => (px[2] as u32, px[1] as u32, px[0] as u32),
        _ => (px[0] as u32, px[1] as u32, px[2] as u32),
    };
    (r * 299 + g * 587 + b * 114) / 1000
}

/// Whether one row (`fixed` = y) reads as a bar, and its mean luma.
fn row_stats(frame: &Frame, y: u32) -> (bool, u32) {
    let stride = frame.stride as usize;
    let row = &frame.data[y as usize * stride..];
    let mut dark = 0usize;
    let mut total = 0usize;
    let mut sum = 0u64;
    let mut x = 0usize;
    while x < frame.width as usize {
        let px = &row[x * 4..x * 4 + 4];
        let l = luma(px, frame.format);
        sum += u64::from(l);
        if l <= BAR_LUMA {
            dark += 1;
        }
        total += 1;
        x += SAMPLE_STRIDE;
    }
    if total == 0 {
        return (false, 0);
    }
    (
        dark as f32 / total as f32 >= BAR_SHARE,
        (sum / total as u64) as u32,
    )
}

/// Whether one column (`fixed` = x) reads as a bar.
fn column_is_bar(frame: &Frame, x: u32) -> bool {
    let stride = frame.stride as usize;
    let mut dark = 0usize;
    let mut total = 0usize;
    let mut y = 0usize;
    while y < frame.height as usize {
        let px = &frame.data[y * stride + x as usize * 4..][..4];
        if luma(px, frame.format) <= BAR_LUMA {
            dark += 1;
        }
        total += 1;
        y += SAMPLE_STRIDE;
    }
    total > 0 && dark as f32 / total as f32 >= BAR_SHARE
}

/// Mean luma of the region inside the detected bars (sampled).
fn content_mean(frame: &Frame, bars: DetectedBars) -> u32 {
    let stride = frame.stride as usize;
    let mut sum = 0u64;
    let mut count = 0u64;
    let mut y = bars.top as usize;
    while y < (frame.height - bars.bottom) as usize {
        let row = &frame.data[y * stride..];
        let mut x = bars.left as usize;
        while x < (frame.width - bars.right) as usize {
            sum += u64::from(luma(&row[x * 4..x * 4 + 4], frame.format));
            count += 1;
            x += SAMPLE_STRIDE;
        }
        y += SAMPLE_STRIDE;
    }
    sum.checked_div(count).unwrap_or(0) as u32
}

/// Detect letterbox/pillarbox bars on a decoded frame. Returns zero bars for
/// frames that are too small, all-dark (a night scene must never be eaten),
/// or whose "bars" are thinner than sensor noise.
pub fn detect_bars(frame: &Frame) -> DetectedBars {
    if frame.width < 16
        || frame.height < 16
        || frame.stride < frame.width * 4
        || frame.data.len() < frame.stride as usize * frame.height as usize
    {
        return DetectedBars::default();
    }
    let max_v = (frame.height as f32 * MAX_SIDE_SHARE) as u32;
    let max_h = (frame.width as f32 * MAX_SIDE_SHARE) as u32;

    let mut bars = DetectedBars::default();
    while bars.top < max_v && row_stats(frame, bars.top).0 {
        bars.top += 1;
    }
    while bars.bottom < max_v && row_stats(frame, frame.height - 1 - bars.bottom).0 {
        bars.bottom += 1;
    }
    while bars.left < max_h && column_is_bar(frame, bars.left) {
        bars.left += 1;
    }
    while bars.right < max_h && column_is_bar(frame, frame.width - 1 - bars.right) {
        bars.right += 1;
    }

    // Noise guard: hairline "bars" are not letterboxing.
    if bars.top < MIN_BAR_PX {
        bars.top = 0;
    }
    if bars.bottom < MIN_BAR_PX {
        bars.bottom = 0;
    }
    if bars.left < MIN_BAR_PX {
        bars.left = 0;
    }
    if bars.right < MIN_BAR_PX {
        bars.right = 0;
    }
    if bars.is_zero() {
        return bars;
    }
    // Dark-scene guard: the content inside the bars must actually be
    // brighter than the bars, or this is a dark frame, not a letterbox.
    if content_mean(frame, bars) < CONTENT_LUMA {
        return DetectedBars::default();
    }
    bars
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    /// A synthetic RGBA frame: `content` luma inside, `bar` luma in the
    /// given bar widths.
    fn frame_with_bars(
        width: u32,
        height: u32,
        bars: DetectedBars,
        bar_value: u8,
        content_value: u8,
    ) -> Frame {
        let mut data = vec![0u8; (width * height * 4) as usize];
        for y in 0..height {
            for x in 0..width {
                let inside = x >= bars.left
                    && x < width - bars.right
                    && y >= bars.top
                    && y < height - bars.bottom;
                let value = if inside { content_value } else { bar_value };
                let at = ((y * width + x) * 4) as usize;
                data[at..at + 3].copy_from_slice(&[value, value, value]);
                data[at + 3] = 255;
            }
        }
        Frame {
            width,
            height,
            stride: width * 4,
            format: fcap_capture::PixelFormat::Rgba8,
            data,
            captured_at: Instant::now(),
        }
    }

    #[test]
    fn detects_letterbox_and_pillarbox() {
        let letter = DetectedBars {
            top: 60,
            bottom: 60,
            ..DetectedBars::default()
        };
        let frame = frame_with_bars(320, 240, letter, 0, 128);
        assert_eq!(detect_bars(&frame), letter);

        let pillar = DetectedBars {
            left: 40,
            right: 40,
            ..DetectedBars::default()
        };
        let frame = frame_with_bars(320, 240, pillar, 8, 200);
        assert_eq!(detect_bars(&frame), pillar);

        // Windowboxed: all four sides at once.
        let boxed = DetectedBars {
            left: 16,
            top: 24,
            right: 16,
            bottom: 24,
        };
        let frame = frame_with_bars(320, 240, boxed, 4, 90);
        assert_eq!(detect_bars(&frame), boxed);
    }

    #[test]
    fn a_dark_scene_is_never_eaten() {
        // Whole frame dark (a night shot / fade-out): the per-side caps
        // would happily call 45% of it "bar" — the content guard says no.
        let dark = frame_with_bars(320, 240, DetectedBars::default(), 0, 10);
        assert!(detect_bars(&dark).is_zero(), "all-dark frame: zero crop");

        // Bars around content that is itself too dark to trust.
        let dim = frame_with_bars(
            320,
            240,
            DetectedBars {
                top: 40,
                bottom: 40,
                ..DetectedBars::default()
            },
            0,
            20,
        );
        assert!(detect_bars(&dim).is_zero(), "dim content: zero crop");
    }

    #[test]
    fn clean_frames_and_noise_lines_stay_uncropped() {
        let clean = frame_with_bars(320, 240, DetectedBars::default(), 0, 150);
        assert!(detect_bars(&clean).is_zero(), "no bars = no crop");

        // A single dark scanline is noise, not a letterbox.
        let hairline = frame_with_bars(
            320,
            240,
            DetectedBars {
                top: 1,
                ..DetectedBars::default()
            },
            0,
            150,
        );
        assert!(detect_bars(&hairline).is_zero(), "1 px = noise");
    }

    #[test]
    fn bright_pixels_inside_a_bar_break_the_bar() {
        // Subtitles burned into the bottom bar: the bright line stops the
        // scan, so only the region below the subtitle is cropped.
        let mut frame = frame_with_bars(
            320,
            240,
            DetectedBars {
                bottom: 60,
                ..DetectedBars::default()
            },
            0,
            140,
        );
        // Paint a bright row 20 px above the bottom edge (inside the bar).
        let y = 240 - 20;
        for x in 0..320usize {
            let at = (y * 320 + x) * 4;
            frame.data[at..at + 3].copy_from_slice(&[220, 220, 220]);
        }
        let bars = detect_bars(&frame);
        assert!(bars.bottom < 20, "the scan stops at the subtitle row");
    }

    #[test]
    fn hostile_geometry_is_rejected() {
        let mut bad = frame_with_bars(320, 240, DetectedBars::default(), 0, 150);
        bad.data.truncate(100); // shorter than its geometry
        assert!(detect_bars(&bad).is_zero());
        let tiny = frame_with_bars(8, 8, DetectedBars::default(), 0, 150);
        assert!(detect_bars(&tiny).is_zero());
    }
}
