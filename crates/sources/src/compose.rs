//! Shared CPU compositing primitives for the face-generator sources:
//! straight-alpha over-blits of rastered text/images and solid rect fills
//! into an RGBA face buffer. Extracted from the split timer (CAP-N18) when
//! the title designer (CAP-N16) needed the same helpers.

use fcap_capture::Frame;

/// Straight-alpha over-blit of a raster into the face buffer. Honors the
/// raster's stride, clips at the face's edges; negative positions are fine.
pub(crate) fn blit(face: &mut [u8], width: usize, height: usize, raster: &Frame, x: i64, y: i64) {
    let stride = raster.stride as usize;
    for row in 0..raster.height as usize {
        let dst_y = y + row as i64;
        if dst_y < 0 || dst_y >= height as i64 {
            continue;
        }
        for col in 0..raster.width as usize {
            let dst_x = x + col as i64;
            if dst_x < 0 || dst_x >= width as i64 {
                continue;
            }
            let src = row * stride + col * 4;
            let alpha = raster.data[src + 3] as u32;
            if alpha == 0 {
                continue;
            }
            let dst = (dst_y as usize * width + dst_x as usize) * 4;
            // Straight-alpha OVER with a possibly-transparent destination:
            // weight the source by its alpha and the destination by the
            // coverage the source leaves it, then divide by the summed
            // weight so the stored color stays STRAIGHT — the compositor
            // multiplies by alpha exactly once, at blend time. (Weighting
            // by src alpha alone premultiplies wherever the face is still
            // transparent, which darkened every glyph edge on the canvas.)
            let under_a = face[dst + 3] as u32;
            let w_src = alpha * 255;
            let w_dst = under_a * (255 - alpha);
            let w_sum = w_src + w_dst; // > 0 — alpha is non-zero here
            for ch in 0..3 {
                let over = raster.data[src + ch] as u32;
                let under = face[dst + ch] as u32;
                face[dst + ch] = ((over * w_src + under * w_dst) / w_sum) as u8;
            }
            face[dst + 3] = (w_sum / 255) as u8;
        }
    }
}

/// [`blit`], centered on the face (the "waiting"/"connecting" card layouts).
pub(crate) fn blit_centered(face: &mut [u8], width: usize, height: usize, raster: &Frame) {
    let x = (width.saturating_sub(raster.width as usize) / 2) as i64;
    let y = (height.saturating_sub(raster.height as usize) / 2) as i64;
    blit(face, width, height, raster, x, y);
}

/// The color at a fraction of its alpha (idle fills, outlines, dimmed rows).
pub(crate) fn dim(color: [u8; 4], factor: f32) -> [u8; 4] {
    [
        color[0],
        color[1],
        color[2],
        (color[3] as f32 * factor) as u8,
    ]
}

/// Overwrite the rounded rectangle `[x0, x1) × [y0, y1)` with `color` (a copy,
/// not a blend: panels land on the clear canvas and badges are opaque).
/// Pixels outside the corner arcs are left untouched, so the corners round.
/// (Promoted from the social bar when the featured-chat banner needed the
/// same primitive.)
pub(crate) fn fill_round_rect(
    buf: &mut [u8],
    width: usize,
    rect: (usize, usize, usize, usize),
    radius: usize,
    color: [u8; 4],
) {
    let (x0, y0, x1, y1) = rect;
    if x1 <= x0 || y1 <= y0 {
        return;
    }
    let r = radius.min((x1 - x0) / 2).min((y1 - y0) / 2);
    for y in y0..y1 {
        for x in x0..x1 {
            if !inside_round(x, y, x0, y0, x1, y1, r) {
                continue;
            }
            let dst = (y * width + x) * 4;
            buf[dst..dst + 4].copy_from_slice(&color);
        }
    }
}

/// Whether `(x, y)` lies within the rounded rectangle — always true away from
/// the four corner quadrants, else inside the quarter-circle of radius `r`.
fn inside_round(x: usize, y: usize, x0: usize, y0: usize, x1: usize, y1: usize, r: usize) -> bool {
    if r == 0 {
        return true;
    }
    let left = x < x0 + r;
    let right = x + r >= x1;
    let top = y < y0 + r;
    let bottom = y + r >= y1;
    // The centre cross (edges + interior) is always filled.
    let (cx, cy) = match (left, right, top, bottom) {
        (true, _, true, _) => (x0 + r, y0 + r),
        (_, true, true, _) => ((x1 - 1).saturating_sub(r), y0 + r),
        (true, _, _, true) => (x0 + r, (y1 - 1).saturating_sub(r)),
        (_, true, _, true) => ((x1 - 1).saturating_sub(r), (y1 - 1).saturating_sub(r)),
        _ => return true,
    };
    let dx = x as f32 - cx as f32;
    let dy = y as f32 - cy as f32;
    dx * dx + dy * dy <= (r as f32) * (r as f32)
}

/// Overwrite a rect with `color` (no blending — callers pre-clamp bounds).
pub(crate) fn fill_rect(
    face: &mut [u8],
    width: usize,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    color: [u8; 4],
) {
    for y in y0..y1 {
        for x in x0..x1 {
            let dst = (y * width + x) * 4;
            face[dst..dst + 4].copy_from_slice(&color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fcap_capture::PixelFormat;
    use std::time::Instant;

    fn raster(color: [u8; 4]) -> Frame {
        Frame {
            width: 1,
            height: 1,
            stride: 4,
            format: PixelFormat::Rgba8,
            data: color.to_vec(),
            captured_at: Instant::now(),
        }
    }

    /// Over a TRANSPARENT face the color must stay straight — the compositor
    /// multiplies by alpha at blend time, so premultiplying here too darkened
    /// every glyph edge and shadow drawn over the bare canvas.
    #[test]
    fn blit_over_transparent_keeps_the_color_straight() {
        let mut face = vec![0u8; 4];
        blit(&mut face, 1, 1, &raster([200, 100, 40, 128]), 0, 0);
        assert_eq!(&face[..3], &[200, 100, 40], "color is not premultiplied");
        assert_eq!(face[3], 128, "alpha carries the coverage");
    }

    /// Over an OPAQUE face the classic over-blend still holds.
    #[test]
    fn blit_over_opaque_blends_classically() {
        let mut face = vec![0u8, 0, 0, 255];
        blit(&mut face, 1, 1, &raster([255, 255, 255, 128]), 0, 0);
        assert_eq!(face[3], 255, "an opaque face stays opaque");
        assert_eq!(face[0], 128, "half-alpha white over black is mid-gray");
    }
}
