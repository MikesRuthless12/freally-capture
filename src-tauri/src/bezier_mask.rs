//! Rasterize a freehand closed path (CAP-N28) into a soft-edged alpha mask.
//!
//! The path points are normalized 0..=1 in item space. The points are smoothed
//! into a closed Catmull-Rom curve (so it reads as a bezier, not a polygon),
//! scanline-filled, and box-blurred by `feather` for a soft edge. The result is
//! fed to the compositor as a mask image (the same GPU pass an image mask uses)
//! and, for the wipe export, saved as a grayscale luma-wipe pattern.

/// The rasterized mask resolution. Sampled stretched to the item, so a modest
/// square is plenty for a soft mask.
pub const MASK_SIZE: u32 = 256;

/// Catmull-Rom point between `p1` and `p2` (with neighbors `p0`, `p3`) at `t`.
fn catmull_rom(p0: [f32; 2], p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], t: f32) -> (f32, f32) {
    let t2 = t * t;
    let t3 = t2 * t;
    let comp = |a: f32, b: f32, c: f32, d: f32| {
        0.5 * ((2.0 * b)
            + (-a + c) * t
            + (2.0 * a - 5.0 * b + 4.0 * c - d) * t2
            + (-a + 3.0 * b - 3.0 * c + d) * t3)
    };
    (
        comp(p0[0], p1[0], p2[0], p3[0]),
        comp(p0[1], p1[1], p2[1], p3[1]),
    )
}

/// Sample the closed smoothed curve into a fine polygon in pixel space.
fn smooth_closed(points: &[[f32; 2]], size: f32) -> Vec<(f32, f32)> {
    const SEG: usize = 12;
    let n = points.len();
    let mut poly = Vec::with_capacity(n * SEG);
    for i in 0..n {
        let p0 = points[(i + n - 1) % n];
        let p1 = points[i];
        let p2 = points[(i + 1) % n];
        let p3 = points[(i + 2) % n];
        for s in 0..SEG {
            let t = s as f32 / SEG as f32;
            let (x, y) = catmull_rom(p0, p1, p2, p3, t);
            poly.push((x.clamp(0.0, 1.0) * size, y.clamp(0.0, 1.0) * size));
        }
    }
    poly
}

/// Even-odd scanline fill of a polygon into an alpha buffer (inside = 255).
fn fill_polygon(poly: &[(f32, f32)], size: u32, alpha: &mut [u8]) {
    let n = poly.len();
    let mut xs: Vec<f32> = Vec::new();
    for y in 0..size {
        let yf = y as f32 + 0.5;
        xs.clear();
        for i in 0..n {
            let (x0, y0) = poly[i];
            let (x1, y1) = poly[(i + 1) % n];
            if (y0 <= yf && y1 > yf) || (y1 <= yf && y0 > yf) {
                let t = (yf - y0) / (y1 - y0);
                xs.push(x0 + t * (x1 - x0));
            }
        }
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mut i = 0;
        while i + 1 < xs.len() {
            let start = xs[i].max(0.0).round() as i32;
            let end = xs[i + 1].min(size as f32).round() as i32;
            for x in start..end {
                if x >= 0 && (x as u32) < size {
                    alpha[(y * size + x as u32) as usize] = 255;
                }
            }
            i += 2;
        }
    }
}

/// Separable box blur (a soft feathered edge). Runs once per mask edit, so the
/// naive window is fine.
fn box_blur(alpha: &mut [u8], size: u32, radius: usize) {
    if radius == 0 {
        return;
    }
    let s = size as usize;
    let mut tmp = vec![0u16; s * s];
    for y in 0..s {
        let row = y * s;
        for x in 0..s {
            let x0 = x.saturating_sub(radius);
            let x1 = (x + radius).min(s - 1);
            let mut sum = 0u32;
            for xi in x0..=x1 {
                sum += alpha[row + xi] as u32;
            }
            tmp[row + x] = (sum / (x1 - x0 + 1) as u32) as u16;
        }
    }
    for x in 0..s {
        for y in 0..s {
            let y0 = y.saturating_sub(radius);
            let y1 = (y + radius).min(s - 1);
            let mut sum = 0u32;
            for yi in y0..=y1 {
                sum += tmp[yi * s + x] as u32;
            }
            alpha[y * s + x] = (sum / (y1 - y0 + 1) as u32) as u8;
        }
    }
}

/// Rasterize the closed path into a single-channel alpha mask (`MASK_SIZE²`).
/// `None` when fewer than three points describe no shape. `feather` is 0..=1 of
/// the mask size; `invert` flips inside/outside.
pub fn rasterize_alpha(points: &[[f32; 2]], feather: f32, invert: bool) -> Option<Vec<u8>> {
    if points.len() < 3 {
        return None;
    }
    let size = MASK_SIZE;
    let mut alpha = vec![0u8; (size * size) as usize];
    let poly = smooth_closed(points, size as f32);
    fill_polygon(&poly, size, &mut alpha);
    let radius = (feather.clamp(0.0, 1.0) * size as f32 * 0.5).round() as usize;
    box_blur(&mut alpha, size, radius.min(size as usize / 2));
    if invert {
        for value in &mut alpha {
            *value = 255 - *value;
        }
    }
    Some(alpha)
}

/// Expand a single-channel alpha into RGBA, packing each byte with `pack`.
fn expand(alpha: Vec<u8>, pack: impl Fn(u8) -> [u8; 4]) -> Vec<u8> {
    let mut rgba = Vec::with_capacity(alpha.len() * 4);
    for a in alpha {
        rgba.extend_from_slice(&pack(a));
    }
    rgba
}

/// The mask as straight RGBA the compositor's alpha-mask pass reads (`a` = the
/// mask; rgb opaque).
pub fn mask_rgba(points: &[[f32; 2]], feather: f32, invert: bool) -> Option<Vec<u8>> {
    let alpha = rasterize_alpha(points, feather, invert)?;
    Some(expand(alpha, |a| [255, 255, 255, a]))
}

/// The path as an opaque grayscale luma-wipe pattern (rgb = the mask, `a` = 255)
/// — a shape-reveal transition when selected as an Image Wipe.
pub fn wipe_rgba(points: &[[f32; 2]], feather: f32, invert: bool) -> Option<Vec<u8>> {
    let alpha = rasterize_alpha(points, feather, invert)?;
    Some(expand(alpha, |a| [a, a, a, 255]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_centered_triangle_fills_its_middle_and_clears_a_corner() {
        // A big centered triangle, no feather.
        let tri = [[0.5, 0.1], [0.9, 0.9], [0.1, 0.9]];
        let alpha = rasterize_alpha(&tri, 0.0, false).expect("three points");
        let at = |x: f32, y: f32| {
            let px = (x * MASK_SIZE as f32) as u32;
            let py = (y * MASK_SIZE as f32) as u32;
            alpha[(py * MASK_SIZE + px) as usize]
        };
        assert_eq!(at(0.5, 0.7), 255, "the interior is filled");
        assert_eq!(at(0.02, 0.02), 0, "a corner is outside");

        // Invert flips it.
        let inv = rasterize_alpha(&tri, 0.0, true).expect("inverted");
        assert_eq!(
            inv[(180 * MASK_SIZE + 128) as usize],
            0,
            "interior now clear"
        );

        // Fewer than three points → no shape.
        assert!(rasterize_alpha(&[[0.5, 0.5], [0.6, 0.6]], 0.0, false).is_none());
    }

    #[test]
    fn feather_softens_the_edge() {
        let square = [[0.25, 0.25], [0.75, 0.25], [0.75, 0.75], [0.25, 0.75]];
        let soft = rasterize_alpha(&square, 0.1, false).expect("square");
        // Somewhere near the edge there is a partial (feathered) value.
        assert!(
            soft.iter().any(|&a| a > 5 && a < 250),
            "a feathered edge has intermediate alpha"
        );
    }
}
