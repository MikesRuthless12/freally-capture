//! The **Image Slideshow** source (Phase 6, TASK-607): an ordered set of
//! images cycling on a timer, with an optional CPU crossfade between
//! equal-sized slides (different sizes hard-cut — honestly, no silent
//! rescale), loop or hold-last, and an optional per-cycle shuffle.
//!
//! Images decode once up front; the session thread then publishes frames on
//! the same latest-wins channel every capture uses. A missing/broken file
//! fails the session loudly with the offending path — never a silent skip.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame, PixelFormat};

use crate::image::load_image_rgba;

/// A tiny deterministic LCG (numerical-recipes constants) — enough to
/// shuffle slides without pulling a rand dependency into the workspace.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

fn shuffled(order: &mut [usize], rng: &mut Lcg) {
    for i in (1..order.len()).rev() {
        let j = (rng.next() as usize) % (i + 1);
        order.swap(i, j);
    }
}

/// Blend two equal-sized RGBA frames (`t` = weight of `b`).
fn crossfade(a: &Frame, b: &Frame, t: f32) -> Frame {
    let t = (t.clamp(0.0, 1.0) * 256.0) as u32;
    let data: Vec<u8> = a
        .data
        .iter()
        .zip(&b.data)
        .map(|(&pa, &pb)| (((pa as u32) * (256 - t) + (pb as u32) * t) >> 8) as u8)
        .collect();
    Frame {
        width: a.width,
        height: a.height,
        stride: a.stride,
        format: a.format,
        data,
        captured_at: std::time::Instant::now(),
    }
}

/// Start a slideshow session over `paths`.
pub fn start_slideshow(
    paths: &[String],
    slide_ms: u32,
    transition_ms: u32,
    looping: bool,
    shuffle: bool,
) -> Result<CaptureSession, CaptureError> {
    if paths.is_empty() {
        return Err(CaptureError::Backend(
            "add at least one image in the slideshow's properties".into(),
        ));
    }
    // Decode everything up front — a broken path fails loudly here, with
    // the file named, instead of flashing black mid-show. Tight RGBA rows.
    let mut slides: Vec<Frame> = Vec::with_capacity(paths.len());
    for path in paths {
        let frame = load_image_rgba(std::path::Path::new(path))
            .map_err(|err| CaptureError::Backend(format!("{path}: {err}")))?;
        slides.push(frame);
    }
    debug_assert!(slides.iter().all(|s| s.format == PixelFormat::Rgba8));

    let slide = Duration::from_millis(u64::from(slide_ms.max(100)));
    let fade = Duration::from_millis(u64::from(transition_ms)).min(slide / 2);

    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-slideshow".into())
        .spawn(move || {
            let mut rng = Lcg(0x5eed_5eed ^ std::process::id() as u64);
            let mut order: Vec<usize> = (0..slides.len()).collect();
            if shuffle {
                shuffled(&mut order, &mut rng);
            }
            let mut at = 0usize;
            let mut slide_started = Instant::now();
            sender.send(slides[order[0]].clone());
            loop {
                if thread_stop.load(Ordering::Relaxed) || !sender.is_open() {
                    return;
                }
                let elapsed = slide_started.elapsed();
                let last = at + 1 >= order.len();
                if last && !looping {
                    // Hold the final slide forever (still honor stop).
                    std::thread::sleep(Duration::from_millis(100));
                    continue;
                }
                if elapsed >= slide {
                    // Advance (re-shuffling each full cycle keeps it fresh).
                    at += 1;
                    if at >= order.len() {
                        at = 0;
                        if shuffle {
                            shuffled(&mut order, &mut rng);
                        }
                    }
                    slide_started = Instant::now();
                    sender.send(slides[order[at]].clone());
                } else if !fade.is_zero() && elapsed >= slide - fade {
                    // Crossfade into the NEXT slide (equal sizes only —
                    // different sizes hard-cut at the boundary instead).
                    let next = order[(at + 1) % order.len()];
                    let current = &slides[order[at]];
                    let upcoming = &slides[next];
                    if current.width == upcoming.width && current.height == upcoming.height {
                        let t =
                            1.0 - (slide - elapsed).as_secs_f32() / fade.as_secs_f32().max(1e-3);
                        sender.send(crossfade(current, upcoming, t));
                    }
                    std::thread::sleep(Duration::from_millis(33)); // ~30 fps blend
                    continue;
                }
                std::thread::sleep(Duration::from_millis(15));
            }
        })
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_slideshow_is_refused() {
        let Err(err) = start_slideshow(&[], 5_000, 300, true, false) else {
            panic!("an empty slideshow must not start");
        };
        assert!(err.to_string().contains("at least one image"));
    }

    #[test]
    fn crossfade_blends_toward_the_next_slide() {
        let a = Frame {
            width: 2,
            height: 1,
            stride: 8,
            format: PixelFormat::Rgba8,
            data: vec![0; 8],
            captured_at: Instant::now(),
        };
        let b = Frame {
            width: 2,
            height: 1,
            stride: 8,
            format: PixelFormat::Rgba8,
            data: vec![200; 8],
            captured_at: Instant::now(),
        };
        assert!(crossfade(&a, &b, 0.0).data.iter().all(|&px| px == 0));
        let half = crossfade(&a, &b, 0.5);
        assert!(half.data.iter().all(|&px| (px as i32 - 100).abs() <= 2));
        assert!(crossfade(&a, &b, 1.0).data.iter().all(|&px| px >= 198));
    }

    #[test]
    fn shuffle_is_a_permutation() {
        let mut order: Vec<usize> = (0..16).collect();
        let mut rng = Lcg(42);
        shuffled(&mut order, &mut rng);
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..16).collect::<Vec<_>>());
    }
}
