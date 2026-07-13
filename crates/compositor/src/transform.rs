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

/// A transform that fit-contains a source into a target rectangle (canvas
/// pixels), centered in it and keeping the source's aspect ratio. Unlike
/// [`fit_to_canvas`] this *may* upscale, so every corner-cam slot renders at a
/// consistent size regardless of the camera's native resolution — the
/// placement for the screen-plus-corners layout.
pub fn fit_into_slot(
    source_w: u32,
    source_h: u32,
    slot_x: f32,
    slot_y: f32,
    slot_w: f32,
    slot_h: f32,
) -> Transform {
    let scale = (slot_w / source_w.max(1) as f32).min(slot_h / source_h.max(1) as f32);
    Transform {
        x: slot_x + slot_w * 0.5,
        y: slot_y + slot_h * 0.5,
        scale_x: scale,
        scale_y: scale,
        rotation: 0.0,
        crop: Crop::default(),
    }
}

/// The backdrop wallpaper's placement: lay `source` into `region` (canvas
/// pixels, `(x, y, w, h)`) — cover-fit for the whole-canvas mode (fills
/// edge-to-edge, overflow cropped by the canvas bounds), fit-contain for a
/// half split (the whole picture stays visible) — then apply the item's
/// transform as zoom (`scale_x`, clamped to `1..=8` of the baseline) and pan
/// (`x`/`y` pixel offsets, clamped to the overflow), so zooming and panning
/// can only choose *which part* of the picture shows: blank canvas never
/// enters the region, no matter what the webview sends.
pub fn backdrop_layout(
    source_w: u32,
    source_h: u32,
    region: (f32, f32, f32, f32),
    contain: bool,
    zoom: f32,
    pan: (f32, f32),
) -> Transform {
    let (rx, ry, rw, rh) = region;
    let sw = source_w.max(1) as f32;
    let sh = source_h.max(1) as f32;
    let base = if contain {
        (rw / sw).min(rh / sh)
    } else {
        (rw / sw).max(rh / sh)
    };
    let zoom = if zoom.is_finite() {
        zoom.clamp(1.0, 8.0)
    } else {
        1.0
    };
    let scale = base * zoom;
    let max_dx = ((sw * scale - rw) * 0.5).max(0.0);
    let max_dy = ((sh * scale - rh) * 0.5).max(0.0);
    let pan_x = if pan.0.is_finite() {
        pan.0.clamp(-max_dx, max_dx)
    } else {
        0.0
    };
    let pan_y = if pan.1.is_finite() {
        pan.1.clamp(-max_dy, max_dy)
    } else {
        0.0
    };
    Transform {
        x: rx + rw * 0.5 + pan_x,
        y: ry + rh * 0.5 + pan_y,
        scale_x: scale,
        scale_y: scale,
        rotation: 0.0,
        crop: Crop::default(),
    }
}

/// The punch-in zoom lens (CAP-N71): scale an item's drawn transform by
/// `zoom` about `anchor` (a point in normalized content coordinates,
/// `(0,0)` top-left → `(1,1)` bottom-right), so the anchored spot stays
/// fixed on the canvas while everything grows around it. Runtime-only —
/// callers apply this to the *drawn* transform; the model's transform (and
/// the undo history) never see it. Zoom clamps to `1..=8`; hostile values
/// fall back to no zoom.
pub fn apply_lens(
    mut transform: Transform,
    content: (f32, f32),
    zoom: f32,
    anchor: (f32, f32),
) -> Transform {
    let zoom = if zoom.is_finite() {
        zoom.clamp(1.0, 8.0)
    } else {
        1.0
    };
    if (zoom - 1.0).abs() < 1e-4 {
        return transform;
    }
    let m = affine(&transform, content);
    let ax = anchor.0.clamp(0.0, 1.0) * content.0;
    let ay = anchor.1.clamp(0.0, 1.0) * content.1;
    let px = m[0] * ax + m[1] * ay + m[2];
    let py = m[3] * ax + m[4] * ay + m[5];
    transform.scale_x *= zoom;
    transform.scale_y *= zoom;
    transform.x = px + (transform.x - px) * zoom;
    transform.y = py + (transform.y - py) * zoom;
    transform
}

/// Integer-lock (CAP-N70): snap each axis's scale to a whole multiple —
/// upscales to the nearest integer (min 1×), downscales to the nearest
/// exact reciprocal (1/2, 1/3, …) — so every source pixel maps to an exact
/// block of canvas pixels. Sign (flip filters never come through here, but
/// hostile files might) and center are preserved; zero/non-finite scales
/// pass through untouched for the ordinary draw path to handle.
pub fn integer_snap(mut transform: Transform) -> Transform {
    let snap = |scale: f32| -> f32 {
        if !scale.is_finite() || scale == 0.0 {
            return scale;
        }
        let magnitude = scale.abs();
        let snapped = if magnitude >= 1.0 {
            magnitude.round().max(1.0)
        } else {
            1.0 / (1.0 / magnitude).round().max(1.0)
        };
        snapped.copysign(scale)
    };
    transform.scale_x = snap(transform.scale_x);
    transform.scale_y = snap(transform.scale_y);
    transform
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
    fn fit_into_slot_centers_and_contains() {
        // A 1920×1080 camera into a 576×324 corner slot at (1324, 21):
        // 16:9 into 16:9 fills exactly, scale = 576/1920 = 0.3, positioned at
        // the slot's center.
        let t = fit_into_slot(1920, 1080, 1324.0, 21.0, 576.0, 324.0);
        assert!((t.scale_x - 0.3).abs() < EPS);
        assert!((t.scale_y - 0.3).abs() < EPS);
        assert!((t.x - (1324.0 + 288.0)).abs() < EPS);
        assert!((t.y - (21.0 + 162.0)).abs() < EPS);
    }

    #[test]
    fn fit_into_slot_keeps_aspect_when_source_and_slot_differ() {
        // A 4:3 (640×480) camera into a 16:9 slot (576×324): the height binds,
        // scale = min(576/640, 324/480) = min(0.9, 0.675) = 0.675, and both
        // axes share it so the picture never stretches.
        let t = fit_into_slot(640, 480, 0.0, 0.0, 576.0, 324.0);
        assert!((t.scale_x - 0.675).abs() < EPS, "height-bound fit-contain");
        assert!(
            (t.scale_x - t.scale_y).abs() < EPS,
            "uniform scale keeps aspect"
        );
    }

    #[test]
    fn backdrop_cover_fills_the_canvas_edge_to_edge() {
        // A 16:9 source on a 16:9 canvas covers exactly (scale 1 at 1:1 px).
        let t = backdrop_layout(
            1920,
            1080,
            (0.0, 0.0, 1920.0, 1080.0),
            false,
            1.0,
            (0.0, 0.0),
        );
        assert!((t.scale_x - 1.0).abs() < EPS);
        assert!((t.x - 960.0).abs() < EPS);
        // A 4:3 source covers by width: 1920/1440 h = 1440 > 1080, cropped.
        let t = backdrop_layout(
            1440,
            1080,
            (0.0, 0.0, 1920.0, 1080.0),
            false,
            1.0,
            (0.0, 0.0),
        );
        assert!(
            (t.scale_x - (1920.0 / 1440.0)).abs() < EPS,
            "the larger ratio wins"
        );
    }

    #[test]
    fn backdrop_contain_keeps_the_whole_picture_in_its_half() {
        // A 16:9 video fit-contained into the left half of a 1920×1080 canvas
        // (960×1080): width binds, scale = 960/1920 = 0.5.
        let t = backdrop_layout(1920, 1080, (0.0, 0.0, 960.0, 1080.0), true, 1.0, (0.0, 0.0));
        assert!((t.scale_x - 0.5).abs() < EPS);
        assert!((t.x - 480.0).abs() < EPS, "centered in its half");
        assert!((t.y - 540.0).abs() < EPS);
    }

    #[test]
    fn backdrop_zoom_and_pan_never_reveal_blank_canvas() {
        // Zoom below baseline clamps to 1 (never smaller than the fit)...
        let t = backdrop_layout(
            1920,
            1080,
            (0.0, 0.0, 1920.0, 1080.0),
            false,
            0.25,
            (0.0, 0.0),
        );
        assert!((t.scale_x - 1.0).abs() < EPS);
        // ...and above 8× clamps too.
        let t = backdrop_layout(
            1920,
            1080,
            (0.0, 0.0, 1920.0, 1080.0),
            false,
            99.0,
            (0.0, 0.0),
        );
        assert!((t.scale_x - 8.0).abs() < EPS);
        // At 2× zoom the overflow is half a canvas each way — a huge pan
        // clamps exactly there, so the edge never crosses into the region.
        let t = backdrop_layout(
            1920,
            1080,
            (0.0, 0.0, 1920.0, 1080.0),
            false,
            2.0,
            (99999.0, -99999.0),
        );
        assert!((t.x - (960.0 + 960.0)).abs() < EPS);
        assert!((t.y - (540.0 - 540.0)).abs() < EPS);
        // At zoom 1 with a cover fit, only the overflow axis can pan: a 4:3
        // source covers a 16:9 canvas by width (1440×1080 → 1920×1440), so
        // the vertical crop can be repositioned by ±180 px, the horizontal
        // not at all.
        let t = backdrop_layout(
            1440,
            1080,
            (0.0, 0.0, 1920.0, 1080.0),
            false,
            1.0,
            (500.0, 500.0),
        );
        assert!((t.x - 960.0).abs() < EPS, "no horizontal overflow to pan");
        assert!(
            (t.y - (540.0 + 180.0)).abs() < EPS,
            "vertical pan clamps to the overflow"
        );
        // Hostile NaN zoom/pan land on the safe defaults.
        let t = backdrop_layout(
            1920,
            1080,
            (0.0, 0.0, 1920.0, 1080.0),
            false,
            f32::NAN,
            (f32::NAN, f32::NAN),
        );
        assert!((t.scale_x - 1.0).abs() < EPS);
        assert!((t.x - 960.0).abs() < EPS);
    }

    #[test]
    fn lens_zoom_keeps_the_anchor_fixed_on_canvas() {
        // A 200×100 item centered at (400, 300), zoomed 2× about its own
        // center: the center must not move, the corners spread out.
        let t = plain(400.0, 300.0);
        let content = (200.0, 100.0);
        let zoomed = apply_lens(t, content, 2.0, (0.5, 0.5));
        assert!((zoomed.x - 400.0).abs() < EPS);
        assert!((zoomed.y - 300.0).abs() < EPS);
        assert!((zoomed.scale_x - 2.0).abs() < EPS);

        // Zoomed about the top-left corner: that canvas point stays put.
        let before = corners(&t, content)[0];
        let zoomed = apply_lens(t, content, 2.0, (0.0, 0.0));
        let after = corners(&zoomed, content)[0];
        assert_close(after, before);

        // Zoom 1 is the identity; hostile zoom is too.
        let same = apply_lens(t, content, 1.0, (0.3, 0.7));
        assert!((same.x - t.x).abs() < EPS && (same.scale_x - 1.0).abs() < EPS);
        let hostile = apply_lens(t, content, f32::NAN, (0.5, 0.5));
        assert!((hostile.scale_x - 1.0).abs() < EPS);
        // And it clamps at 8×.
        let capped = apply_lens(t, content, 99.0, (0.5, 0.5));
        assert!((capped.scale_x - 8.0).abs() < EPS);
    }

    #[test]
    fn integer_snap_locks_scales_to_whole_multiples() {
        let snapped = integer_snap(Transform {
            scale_x: 2.7,
            scale_y: 3.2,
            ..plain(10.0, 10.0)
        });
        assert!((snapped.scale_x - 3.0).abs() < EPS);
        assert!((snapped.scale_y - 3.0).abs() < EPS);
        // Never snaps an upscale below 1×.
        let one = integer_snap(Transform {
            scale_x: 1.2,
            scale_y: 1.0,
            ..plain(0.0, 0.0)
        });
        assert!((one.scale_x - 1.0).abs() < EPS);
        // Downscales land on exact reciprocals (1/2, 1/3, …) by rounding
        // the divisor: 0.45 → 1/2, 0.4 (divisor 2.5) → 1/3, 0.3 → 1/3.
        let down = integer_snap(Transform {
            scale_x: 0.45,
            scale_y: 0.3,
            ..plain(0.0, 0.0)
        });
        assert!((down.scale_x - 0.5).abs() < EPS, "0.45 → 1/2");
        assert!((down.scale_y - (1.0 / 3.0)).abs() < EPS, "0.3 → 1/3");
        // Hostile values pass through instead of poisoning the matrix.
        let hostile = integer_snap(Transform {
            scale_x: f32::NAN,
            scale_y: 0.0,
            ..plain(0.0, 0.0)
        });
        assert!(hostile.scale_x.is_nan());
        assert_eq!(hostile.scale_y, 0.0);
        // A flip's negative scale keeps its sign.
        let flipped = integer_snap(Transform {
            scale_x: -1.8,
            scale_y: 1.0,
            ..plain(0.0, 0.0)
        });
        assert!((flipped.scale_x + 2.0).abs() < EPS);
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
