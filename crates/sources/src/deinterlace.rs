//! CAP-M17 — **Deinterlacing for device sources**: the classic CPU
//! algorithms, applied to a device session's RGBA frames on the capture
//! thread (never the render loop). A capture card fed by an interlaced
//! camera/console hands "weaved" frames — both fields in one image, combing
//! on motion. All five modes are pure pixel math, identical on every OS.
//!
//! - **Discard** — keep the dominant field, line-double it. Half the
//!   vertical detail, zero artifacts, cheapest.
//! - **Bob** — like discard, but the kept field alternates every frame
//!   (temporal bob at the source's frame rate).
//! - **Linear** — keep the dominant field, interpolate the missing lines
//!   from their neighbors.
//! - **Blend** — average the two fields (visible ghosting on motion, full
//!   vertical detail on stills).
//! - **MotionAdaptive** (yadif-class) — per pixel: still areas keep the
//!   previous frame's line (full detail), moving areas interpolate
//!   spatially (no combing). Needs one frame of history.

use fcap_capture::Frame;

/// Which classic algorithm runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Discard,
    Bob,
    Linear,
    Blend,
    MotionAdaptive,
}

/// Which field is dominant (the one shot first).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldOrder {
    TopFirst,
    BottomFirst,
}

/// Per-pixel motion threshold (max channel delta) past which the adaptive
/// mode abandons the temporal line for the spatial interpolation.
const MOTION_THRESHOLD: u8 = 24;

/// One device session's deinterlacer (owns the frame history bob/adaptive need).
pub struct Deinterlacer {
    mode: Mode,
    order: FieldOrder,
    /// The previous RAW (pre-deinterlace) frame — MotionAdaptive's temporal
    /// reference. Dropped whenever the frame geometry changes.
    prev: Option<(u32, u32, Vec<u8>)>,
    /// Bob alternates the kept field per frame.
    bob_bottom: bool,
}

impl Deinterlacer {
    pub fn new(mode: Mode, order: FieldOrder) -> Self {
        Self {
            mode,
            order,
            prev: None,
            bob_bottom: false,
        }
    }

    /// Deinterlace one frame in place (RGBA, any stride ≥ width×4).
    pub fn process(&mut self, frame: &mut Frame) {
        if frame.height < 2 {
            return;
        }
        let raw = match self.mode {
            // Only the adaptive mode pays for the history copy.
            Mode::MotionAdaptive => Some(frame.data.clone()),
            _ => None,
        };
        let dominant_bottom = match self.mode {
            Mode::Bob => {
                self.bob_bottom = !self.bob_bottom;
                // The FIRST field of the pair leads: top-first starts top.
                match self.order {
                    FieldOrder::TopFirst => !self.bob_bottom,
                    FieldOrder::BottomFirst => self.bob_bottom,
                }
            }
            _ => matches!(self.order, FieldOrder::BottomFirst),
        };
        match self.mode {
            Mode::Discard | Mode::Bob => line_double(frame, dominant_bottom),
            Mode::Linear => linear(frame, dominant_bottom),
            Mode::Blend => blend(frame),
            Mode::MotionAdaptive => {
                let usable_prev = self
                    .prev
                    .as_ref()
                    .filter(|(w, h, _)| *w == frame.width && *h == frame.height)
                    .map(|(_, _, data)| data.as_slice());
                motion_adaptive(frame, dominant_bottom, usable_prev);
            }
        }
        if let Some(raw) = raw {
            self.prev = Some((frame.width, frame.height, raw));
        }
    }
}

fn row_bytes(frame: &Frame) -> (usize, usize) {
    (frame.stride as usize, frame.width as usize * 4)
}

/// The rows the dominant field does NOT own (the ones to fill).
fn missing_rows(height: usize, dominant_bottom: bool) -> impl Iterator<Item = usize> {
    let start = if dominant_bottom { 0 } else { 1 };
    (start..height).step_by(2)
}

/// The nearest dominant row for a missing row (clamped inside the frame).
fn nearest_dominant(y: usize, height: usize, dominant_bottom: bool) -> usize {
    let candidate = if dominant_bottom {
        y + 1
    } else {
        y.wrapping_sub(1)
    };
    if candidate < height {
        candidate
    } else if dominant_bottom {
        y - 1
    } else {
        y + 1
    }
}

fn line_double(frame: &mut Frame, dominant_bottom: bool) {
    let (stride, row) = row_bytes(frame);
    let height = frame.height as usize;
    for y in missing_rows(height, dominant_bottom) {
        let src = nearest_dominant(y, height, dominant_bottom);
        let (from, to) = (src * stride, y * stride);
        frame.data.copy_within(from..from + row, to);
    }
}

fn linear(frame: &mut Frame, dominant_bottom: bool) {
    let (stride, row) = row_bytes(frame);
    let height = frame.height as usize;
    for y in missing_rows(height, dominant_bottom) {
        if y == 0 || y + 1 >= height {
            let src = nearest_dominant(y, height, dominant_bottom);
            let from = src * stride;
            frame.data.copy_within(from..from + row, y * stride);
            continue;
        }
        let (above, below, at) = ((y - 1) * stride, (y + 1) * stride, y * stride);
        for x in 0..row {
            frame.data[at + x] =
                ((frame.data[above + x] as u16 + frame.data[below + x] as u16) / 2) as u8;
        }
    }
}

fn blend(frame: &mut Frame) {
    let (stride, row) = row_bytes(frame);
    let height = frame.height as usize;
    // Average each field pair into both of its rows (classic field blend).
    let mut pair = 0;
    while pair + 1 < height {
        let (a, b) = (pair * stride, (pair + 1) * stride);
        for x in 0..row {
            let avg = ((frame.data[a + x] as u16 + frame.data[b + x] as u16) / 2) as u8;
            frame.data[a + x] = avg;
            frame.data[b + x] = avg;
        }
        pair += 2;
    }
}

fn motion_adaptive(frame: &mut Frame, dominant_bottom: bool, prev: Option<&[u8]>) {
    let Some(prev) = prev else {
        // No usable history (first frame / geometry change): spatial only.
        linear(frame, dominant_bottom);
        return;
    };
    let (stride, row) = row_bytes(frame);
    let height = frame.height as usize;
    for y in missing_rows(height, dominant_bottom) {
        if y == 0 || y + 1 >= height {
            let src = nearest_dominant(y, height, dominant_bottom);
            let from = src * stride;
            frame.data.copy_within(from..from + row, y * stride);
            continue;
        }
        let (above, below, at) = ((y - 1) * stride, (y + 1) * stride, y * stride);
        let mut x = 0;
        while x < row {
            // Per-pixel motion: how much the neighboring dominant lines moved
            // since the last frame (max channel delta of both).
            let mut moved = 0u8;
            for c in 0..4 {
                moved = moved
                    .max(frame.data[above + x + c].abs_diff(prev[above + x + c]))
                    .max(frame.data[below + x + c].abs_diff(prev[below + x + c]));
            }
            if moved > MOTION_THRESHOLD {
                for c in 0..4 {
                    frame.data[at + x + c] = ((frame.data[above + x + c] as u16
                        + frame.data[below + x + c] as u16)
                        / 2) as u8;
                }
            } else {
                for c in 0..4 {
                    frame.data[at + x + c] = prev[at + x + c];
                }
            }
            x += 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fcap_capture::PixelFormat;
    use std::time::Instant;

    /// A weaved test frame: every row filled with `rows[y]`.
    fn weave(rows: &[u8]) -> Frame {
        let width = 4u32;
        let data: Vec<u8> = rows
            .iter()
            .flat_map(|&value| vec![value; width as usize * 4])
            .collect();
        Frame {
            width,
            height: rows.len() as u32,
            stride: width * 4,
            format: PixelFormat::Rgba8,
            data,
            captured_at: Instant::now(),
        }
    }

    fn row_value(frame: &Frame, y: usize) -> u8 {
        frame.data[y * frame.stride as usize]
    }

    #[test]
    fn discard_line_doubles_the_dominant_field() {
        let mut top = Deinterlacer::new(Mode::Discard, FieldOrder::TopFirst);
        let mut frame = weave(&[100, 200, 100, 200]);
        top.process(&mut frame);
        assert!(
            (0..4).all(|y| row_value(&frame, y) == 100),
            "top field kept"
        );

        let mut bottom = Deinterlacer::new(Mode::Discard, FieldOrder::BottomFirst);
        let mut frame = weave(&[100, 200, 100, 200]);
        bottom.process(&mut frame);
        assert!((0..4).all(|y| row_value(&frame, y) == 200), "bottom kept");
    }

    #[test]
    fn bob_alternates_the_kept_field_per_frame() {
        let mut bob = Deinterlacer::new(Mode::Bob, FieldOrder::TopFirst);
        let mut first = weave(&[100, 200, 100, 200]);
        bob.process(&mut first);
        let mut second = weave(&[100, 200, 100, 200]);
        bob.process(&mut second);
        assert_eq!(row_value(&first, 0), 100, "frame 1 = the first field");
        assert_eq!(row_value(&second, 0), 200, "frame 2 = the other field");
    }

    #[test]
    fn linear_interpolates_missing_lines() {
        // Dominant (even) rows ramp 0,20,40… — interior odd rows must land
        // exactly between their neighbors.
        let mut deint = Deinterlacer::new(Mode::Linear, FieldOrder::TopFirst);
        let mut frame = weave(&[0, 255, 20, 255, 40, 255]);
        deint.process(&mut frame);
        assert_eq!(row_value(&frame, 1), 10);
        assert_eq!(row_value(&frame, 3), 30);
        assert_eq!(row_value(&frame, 5), 40, "edge row copies its neighbor");
    }

    #[test]
    fn blend_averages_each_field_pair() {
        let mut deint = Deinterlacer::new(Mode::Blend, FieldOrder::TopFirst);
        let mut frame = weave(&[100, 200, 100, 200]);
        deint.process(&mut frame);
        assert!((0..4).all(|y| row_value(&frame, y) == 150));
    }

    #[test]
    fn adaptive_keeps_still_lines_and_interpolates_motion() {
        let mut deint = Deinterlacer::new(Mode::MotionAdaptive, FieldOrder::TopFirst);
        // Frame 1: no history → spatial (linear); rows 100/200 → odd = 100.
        let mut first = weave(&[100, 200, 100, 200]);
        deint.process(&mut first);
        assert_eq!(row_value(&first, 1), 100);

        // Frame 2, STILL scene (same weave): the temporal line survives —
        // full vertical detail (the raw 200 comes back).
        let mut still = weave(&[100, 200, 100, 200]);
        deint.process(&mut still);
        assert_eq!(row_value(&still, 1), 200, "still pixels keep the field");

        // Frame 3, MOTION (dominant rows jumped): combing would show — the
        // spatial interpolation takes over instead of the stale field.
        let mut moving = weave(&[10, 200, 10, 200]);
        deint.process(&mut moving);
        assert_eq!(row_value(&moving, 1), 10, "moving pixels interpolate");
    }
}
