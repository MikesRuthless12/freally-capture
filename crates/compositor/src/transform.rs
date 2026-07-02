//! Item transform math — one authoritative mapping from a scene item's
//! [`Transform`] to canvas pixels and clip space.
//!
//! Coordinates: the canvas is y-down pixels, origin top-left. An item's
//! content is its source *after* the transform crop; `Transform::x/y` place
//! the content's **center**, `scale_*` multiply the cropped size, `rotation`
//! is degrees clockwise about that center. The UI's on-canvas handles
//! (`ui/src/panels/PreviewPanel.tsx`) mirror exactly this math — keep them in
//! lockstep or the handles drift off the pixels.

use fcap_scene::{Crop, Transform};

/// The item's content size in source pixels after the transform crop.
/// `None` when the crop consumes the whole source (nothing to draw).
/// (Chained saturating subtractions — a hostile file's `u32::MAX` crops must
/// clamp, not overflow the `left + right` sum.)
pub fn content_size(source_w: u32, source_h: u32, crop: &Crop) -> Option<(f32, f32)> {
    let w = source_w
        .saturating_sub(crop.left)
        .saturating_sub(crop.right);
    let h = source_h
        .saturating_sub(crop.top)
        .saturating_sub(crop.bottom);
    if w == 0 || h == 0 {
        None
    } else {
        Some((w as f32, h as f32))
    }
}

/// The UV rectangle (u0, v0, u1, v1) selecting the cropped region of the
/// source texture.
pub fn uv_rect(source_w: u32, source_h: u32, crop: &Crop) -> [f32; 4] {
    let sw = source_w.max(1) as f32;
    let sh = source_h.max(1) as f32;
    [
        crop.left as f32 / sw,
        crop.top as f32 / sh,
        (source_w.saturating_sub(crop.right)) as f32 / sw,
        (source_h.saturating_sub(crop.bottom)) as f32 / sh,
    ]
}

/// The 2D affine (row-major `[a, b, tx; c, d, ty]`) mapping the item's local
/// content pixels `[0..w]×[0..h]` into canvas pixels:
/// `T(center) · R(rotation) · S(scale) · T(-content/2)`.
pub fn affine(transform: &Transform, content: (f32, f32)) -> [f32; 6] {
    let radians = transform.rotation.to_radians();
    let (sin, cos) = radians.sin_cos();
    // Clockwise in y-down screen space = the standard math matrix.
    let a = cos * transform.scale_x;
    let b = -sin * transform.scale_y;
    let c = sin * transform.scale_x;
    let d = cos * transform.scale_y;
    let half_w = content.0 * 0.5;
    let half_h = content.1 * 0.5;
    let tx = transform.x - (a * half_w + b * half_h);
    let ty = transform.y - (c * half_w + d * half_h);
    [a, b, tx, c, d, ty]
}

/// The item's four content corners in canvas pixels, in local corner order
/// `(0,0) (w,0) (0,h) (w,h)` — what the UI draws its selection box through.
pub fn corners(transform: &Transform, content: (f32, f32)) -> [[f32; 2]; 4] {
    let m = affine(transform, content);
    let map = |x: f32, y: f32| [m[0] * x + m[1] * y + m[2], m[3] * x + m[4] * y + m[5]];
    [
        map(0.0, 0.0),
        map(content.0, 0.0),
        map(0.0, content.1),
        map(content.0, content.1),
    ]
}

/// The full clip-space matrix (column-major, ready for a WGSL `mat4x4<f32>`)
/// taking local content pixels to NDC: the affine above composed with the
/// canvas-to-NDC projection (x right, y **down** in pixels → y up in NDC).
pub fn clip_matrix(
    transform: &Transform,
    content: (f32, f32),
    canvas: (f32, f32),
) -> [[f32; 4]; 4] {
    let [a, b, tx, c, d, ty] = affine(transform, content);
    let sx = 2.0 / canvas.0;
    let sy = 2.0 / canvas.1;
    // NDC_x = sx·x' - 1 ;  NDC_y = 1 - sy·y'
    [
        [sx * a, -sy * c, 0.0, 0.0],
        [sx * b, -sy * d, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [sx * tx - 1.0, 1.0 - sy * ty, 0.0, 1.0],
    ]
}

/// A transform that centers a source on the canvas, scaled down (never up)
/// to fit — the first-frame placement for newly added items.
pub fn fit_to_canvas(source_w: u32, source_h: u32, canvas_w: u32, canvas_h: u32) -> Transform {
    let scale = if source_w > canvas_w || source_h > canvas_h {
        (canvas_w as f32 / source_w.max(1) as f32).min(canvas_h as f32 / source_h.max(1) as f32)
    } else {
        1.0
    };
    Transform {
        x: canvas_w as f32 * 0.5,
        y: canvas_h as f32 * 0.5,
        scale_x: scale,
        scale_y: scale,
        rotation: 0.0,
        crop: Crop::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 1e-4;

    fn assert_close(actual: [f32; 2], expected: [f32; 2]) {
        assert!(
            (actual[0] - expected[0]).abs() < EPS && (actual[1] - expected[1]).abs() < EPS,
            "expected {expected:?}, got {actual:?}"
        );
    }

    fn plain(x: f32, y: f32) -> Transform {
        Transform {
            x,
            y,
            ..Transform::default()
        }
    }

    #[test]
    fn identity_centers_content_on_its_position() {
        let corners = corners(&plain(50.0, 30.0), (20.0, 10.0));
        assert_close(corners[0], [40.0, 25.0]);
        assert_close(corners[1], [60.0, 25.0]);
        assert_close(corners[2], [40.0, 35.0]);
        assert_close(corners[3], [60.0, 35.0]);
    }

    #[test]
    fn scale_grows_around_the_center() {
        let transform = Transform {
            scale_x: 2.0,
            scale_y: 3.0,
            ..plain(0.0, 0.0)
        };
        let corners = corners(&transform, (10.0, 10.0));
        assert_close(corners[0], [-10.0, -15.0]);
        assert_close(corners[3], [10.0, 15.0]);
    }

    #[test]
    fn rotation_is_clockwise_in_screen_space() {
        // A wide bar rotated +90° must become a tall bar; its former right
        // edge midpoint (w, h/2 locally) must point straight DOWN from the
        // center (clockwise with y-down).
        let transform = Transform {
            rotation: 90.0,
            ..plain(0.0, 0.0)
        };
        let corners = corners(&transform, (10.0, 2.0));
        // Local (10, 1) is the right-edge midpoint → distance 5 from center.
        let right_mid = [
            (corners[1][0] + corners[3][0]) / 2.0,
            (corners[1][1] + corners[3][1]) / 2.0,
        ];
        assert_close(right_mid, [0.0, 5.0]);
    }

    #[test]
    fn crop_shrinks_the_content_and_uvs() {
        let crop = Crop {
            left: 2,
            top: 1,
            right: 4,
            bottom: 3,
        };
        assert_eq!(content_size(10, 8, &crop), Some((4.0, 4.0)));
        let uv = uv_rect(10, 8, &crop);
        assert!((uv[0] - 0.2).abs() < EPS);
        assert!((uv[1] - 0.125).abs() < EPS);
        assert!((uv[2] - 0.6).abs() < EPS);
        assert!((uv[3] - 0.625).abs() < EPS);
    }

    #[test]
    fn overcrop_yields_nothing_to_draw() {
        let crop = Crop {
            left: 6,
            top: 0,
            right: 6,
            bottom: 0,
        };
        assert_eq!(content_size(10, 8, &crop), None);
    }

    #[test]
    fn hostile_crop_values_clamp_instead_of_overflowing() {
        // u32::MAX + 1 would panic in debug builds with a plain `left + right`.
        let crop = Crop {
            left: u32::MAX,
            top: 0,
            right: 1,
            bottom: 0,
        };
        assert_eq!(content_size(10, 8, &crop), None);
    }

    #[test]
    fn clip_matrix_maps_canvas_corners_to_ndc() {
        // Content exactly covering a 100×50 canvas.
        let transform = plain(50.0, 25.0);
        let m = clip_matrix(&transform, (100.0, 50.0), (100.0, 50.0));
        let map = |x: f32, y: f32| {
            [
                m[0][0] * x + m[1][0] * y + m[3][0],
                m[0][1] * x + m[1][1] * y + m[3][1],
            ]
        };
        assert_close(map(0.0, 0.0), [-1.0, 1.0]); // top-left → NDC top-left
        assert_close(map(100.0, 50.0), [1.0, -1.0]); // bottom-right
        assert_close(map(50.0, 25.0), [0.0, 0.0]); // center
    }

    #[test]
    fn fit_shrinks_large_sources_and_leaves_small_ones() {
        let fitted = fit_to_canvas(3840, 2160, 1920, 1080);
        assert!((fitted.scale_x - 0.5).abs() < EPS);
        assert!((fitted.x - 960.0).abs() < EPS);
        assert!((fitted.y - 540.0).abs() < EPS);

        let small = fit_to_canvas(640, 360, 1920, 1080);
        assert!((small.scale_x - 1.0).abs() < EPS, "never upscales");
    }

    #[test]
    fn matrix_and_corners_agree() {
        let transform = Transform {
            x: 123.0,
            y: 456.0,
            scale_x: 1.5,
            scale_y: 0.75,
            rotation: -37.0,
            crop: Crop::default(),
        };
        let content = (200.0, 100.0);
        let canvas = (1920.0, 1080.0);
        let m = clip_matrix(&transform, content, canvas);
        let expected = corners(&transform, content);
        for (corner, local) in expected.iter().zip([
            [0.0, 0.0],
            [content.0, 0.0],
            [0.0, content.1],
            [content.0, content.1],
        ]) {
            let ndc = [
                m[0][0] * local[0] + m[1][0] * local[1] + m[3][0],
                m[0][1] * local[0] + m[1][1] * local[1] + m[3][1],
            ];
            // Convert the canvas-space corner to NDC independently.
            let via_canvas = [
                2.0 * corner[0] / canvas.0 - 1.0,
                1.0 - 2.0 * corner[1] / canvas.1,
            ];
            assert_close(ndc, via_canvas);
        }
    }
}
