//! The countdown "slate" painter (V1-C, "Starting Soon").
//!
//! A [`SourceSettings::Timer`](fcap_scene::SourceSettings) in slate mode renders
//! a full-canvas pre-show card instead of the inline clock face: a background
//! (solid, a vertical two-colour gradient, or transparent), an optional message
//! line, and the big countdown number centred beneath it. The time math, the
//! flash-at-zero and the hold live in the studio render loop — this module only
//! composites the picture.
//!
//! Text reuses [`crate::text::render_text`] (the same shaping / bidi / outline /
//! shadow pipeline as the Text source), so the two rasterised frames are blitted
//! onto the background with the shared straight-alpha [`crate::compose::blit`].

use fcap_capture::Frame;

use crate::compose::blit;
use crate::static_source::{check_dimension, rgba_frame, StaticSourceError};
use crate::text::{render_text, TextStyle};

/// The full-canvas background a slate paints (straight RGBA, opaque).
#[derive(Debug, Clone, Copy)]
pub enum SlateBg<'a> {
    /// No fill — leave the canvas transparent so an underlying source shows.
    Transparent,
    /// A flat colour across the whole canvas.
    Solid([u8; 4]),
    /// A top-to-bottom linear gradient between two colours.
    Gradient([u8; 4], [u8; 4]),
    /// A still image drawn contain-fit (whole picture visible, centred,
    /// letterboxed on transparency — never cropped). The
    /// borrow is the already-decoded frame — the caller owns the decode/cache.
    Image(&'a Frame),
}

/// The gap between the message line and the number, as a fraction of the
/// number's height. Keeps the two visually grouped at any font size.
const BLOCK_GAP_FRAC: f32 = 0.15;

/// Paint one countdown slate at `width`×`height`. `message` may carry empty
/// text (then only the number shows); `number` is the formatted countdown.
/// Both styles are honoured as-is (font, size, colour, outline, shadow) — the
/// caller sets alignment; horizontal centring on the canvas is done here.
pub fn render_countdown_slate(
    width: u32,
    height: u32,
    bg: SlateBg<'_>,
    message: &TextStyle,
    number: &TextStyle,
) -> Result<Frame, StaticSourceError> {
    check_dimension("countdown slate width", width)?;
    check_dimension("countdown slate height", height)?;

    let (w, h) = (width as usize, height as usize);
    let mut buf = vec![0u8; w * h * 4];
    paint_background(&mut buf, width, height, bg);

    // Rasterise the number (always) and the message (only when non-blank),
    // reusing the Text source's shaping so RTL / fallback fonts just work.
    let number_frame = render_text(number)?;
    let message_frame = if message.text.trim().is_empty() {
        None
    } else {
        Some(render_text(message)?)
    };

    let gap = if message_frame.is_some() {
        (number_frame.height as f32 * BLOCK_GAP_FRAC).round() as u32
    } else {
        0
    };
    let block_height = message_frame.as_ref().map_or(0, |f| f.height + gap) + number_frame.height;
    let mut y = (height.saturating_sub(block_height) / 2) as i64;

    if let Some(msg) = &message_frame {
        let x = centre_x(width, msg.width);
        blit(&mut buf, w, h, msg, x, y);
        y += (msg.height + gap) as i64;
    }
    let x = centre_x(width, number_frame.width);
    blit(&mut buf, w, h, &number_frame, x, y);

    Ok(rgba_frame(width, height, buf))
}

/// The left edge that centres `content` px within `canvas` px (clamped ≥ 0).
fn centre_x(canvas: u32, content: u32) -> i64 {
    (canvas.saturating_sub(content) / 2) as i64
}

/// Fill the whole buffer per [`SlateBg`]. Transparent leaves the zeroed
/// (fully transparent) canvas untouched.
fn paint_background(buf: &mut [u8], width: u32, height: u32, bg: SlateBg<'_>) {
    let (w, h) = (width as usize, height as usize);
    match bg {
        SlateBg::Transparent => {}
        SlateBg::Solid(color) => {
            for px in buf.chunks_exact_mut(4) {
                px.copy_from_slice(&color);
            }
        }
        SlateBg::Image(image) => paint_fit_image(buf, width, height, image),
        SlateBg::Gradient(from, to) => {
            // One colour per row (a top→bottom gradient); a single row of h==1
            // uses the start colour rather than dividing by zero.
            let last = (h.max(1) - 1).max(1) as f32;
            for row in 0..h {
                let t = row as f32 / last;
                let color = [
                    lerp(from[0], to[0], t),
                    lerp(from[1], to[1], t),
                    lerp(from[2], to[2], t),
                    lerp(from[3], to[3], t),
                ];
                let start = row * w * 4;
                for px in buf[start..start + w * 4].chunks_exact_mut(4) {
                    px.copy_from_slice(&color);
                }
            }
        }
    }
}

/// Draw `image` contain-fit into the canvas buffer: scaled so the WHOLE image
/// is visible (letterboxed, never cropped) and centred; the surround stays
/// transparent. Nearest-neighbour — a background needs no resampling niceties,
/// and this avoids any premultiply round-trip.
fn paint_fit_image(buf: &mut [u8], width: u32, height: u32, image: &Frame) {
    let (iw, ih) = (image.width.max(1), image.height.max(1));
    // `min` = contain: the larger dimension just fits, the other is centred.
    let scale = (width as f32 / iw as f32).min(height as f32 / ih as f32);
    let draw_w = (iw as f32 * scale).round().max(1.0);
    let draw_h = (ih as f32 * scale).round().max(1.0);
    let off_x = (width as f32 - draw_w) / 2.0;
    let off_y = (height as f32 - draw_h) / 2.0;
    let stride = image.stride as usize;
    // Only touch the letterboxed rect the image actually occupies.
    let x0 = off_x.max(0.0) as u32;
    let y0 = off_y.max(0.0) as u32;
    let x1 = (off_x + draw_w).min(width as f32).max(0.0) as u32;
    let y1 = (off_y + draw_h).min(height as f32).max(0.0) as u32;
    for y in y0..y1 {
        let sy = (((y as f32 - off_y) / scale) as i32).clamp(0, ih as i32 - 1) as usize;
        for x in x0..x1 {
            let sx = (((x as f32 - off_x) / scale) as i32).clamp(0, iw as i32 - 1) as usize;
            let si = sy * stride + sx * 4;
            let di = ((y * width + x) * 4) as usize;
            buf[di] = image.data[si];
            buf[di + 1] = image.data[si + 1];
            buf[di + 2] = image.data[si + 2];
            // The image's own alpha carries through — a transparent PNG shows
            // whatever is beneath the slate; the letterbox stays transparent.
            buf[di + 3] = image.data[si + 3];
        }
    }
}

/// Linear-interpolate one 8-bit channel.
fn lerp(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t)
        .round()
        .clamp(0.0, 255.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    fn style(text: &str, size: f32, color: [u8; 4]) -> TextStyle {
        TextStyle {
            text: text.to_string(),
            size_px: size,
            color,
            align: crate::text::TextAlign::Center,
            ..TextStyle::default()
        }
    }

    /// A solid slate is opaque everywhere and paints its fill colour into the
    /// corners (which the centred text never reaches).
    #[test]
    fn solid_slate_fills_the_canvas() {
        let frame = render_countdown_slate(
            320,
            180,
            SlateBg::Solid([10, 20, 30, 255]),
            &style("", 40.0, [255, 255, 255, 255]),
            &style("0:05", 80.0, [255, 255, 255, 255]),
        )
        .expect("slate");
        assert_eq!((frame.width, frame.height), (320, 180));
        // Top-left corner is background, not text.
        assert_eq!(&frame.data[0..4], &[10, 20, 30, 255]);
    }

    /// A transparent slate leaves the corners fully transparent so an
    /// underlying source (image/video/backdrop) shows through.
    #[test]
    fn transparent_slate_keeps_the_corners_clear() {
        let frame = render_countdown_slate(
            200,
            120,
            SlateBg::Transparent,
            &style("Starting Soon", 24.0, [255, 255, 255, 255]),
            &style("1:00", 60.0, [255, 255, 255, 255]),
        )
        .expect("slate");
        assert_eq!(frame.data[3], 0, "top-left stays transparent");
    }

    /// The gradient runs top→bottom: the first row is the start colour, the
    /// last row the end colour.
    #[test]
    fn gradient_runs_top_to_bottom() {
        let frame = render_countdown_slate(
            8,
            64,
            SlateBg::Gradient([0, 0, 0, 255], [200, 100, 40, 255]),
            &style("", 10.0, [255, 255, 255, 255]),
            &style(" ", 10.0, [255, 255, 255, 255]),
        )
        .expect("slate");
        assert_eq!(&frame.data[0..4], &[0, 0, 0, 255], "top row is the start");
        let last = frame.data.len() - 4;
        assert_eq!(
            &frame.data[last..],
            &[200, 100, 40, 255],
            "bottom row is the end"
        );
    }

    /// An image background is contain-fit and centred: the WHOLE image shows
    /// (a square on a 16:9 canvas letterboxes left/right), opaque, and the
    /// letterbox surround stays transparent.
    #[test]
    fn image_slate_contains_and_centers() {
        let red = rgba_frame(4, 4, [200u8, 30, 30, 255].repeat(16));
        let frame = render_countdown_slate(
            160,
            90,
            SlateBg::Image(&red),
            &style("", 20.0, [255, 255, 255, 255]),
            &style("0:10", 40.0, [255, 255, 255, 255]),
        )
        .expect("slate");
        // 4:4 image on 160×90 → a 90×90 image centred at x∈[35,125].
        assert_eq!(frame.data[3], 0, "top-left corner is transparent letterbox");
        let inside = ((5 * 160 + 40) * 4) as usize; // (40, 5): inside the image, above the text
        assert_eq!(
            &frame.data[inside..inside + 4],
            &[200, 30, 30, 255],
            "the whole image shows, opaque and centred"
        );
    }
}
