//! # fcap-compositor
//!
//! The **owned** real-time GPU compositor on `wgpu` — the engine of Freally
//! Capture. Uploads each visible source frame as a texture and composes the
//! single program frame with per-source transform (move/scale/rotate/crop),
//! blend modes, and (P2.3) the ordered per-item video filter chain; scene
//! transitions arrive in Phase 5 (→ 0.70.0).
//! Budget: sustained 60 fps at 1080p with ≥4 sources on a mid-range GPU —
//! verified by the (hardware-gated) `perf_budget` test below.
//!
//! Headless by design: no window or surface — the program frame feeds the
//! preview pipe today and the encoders/stream in later phases.

#![forbid(unsafe_code)]

mod compositor;
pub mod filters;
mod gpu;
mod native_preview;
pub mod telestrator;
pub mod transform;

pub use compositor::{Compositor, DownstreamDraw, ProgramFrame, ReactionDraw, REACTION_POOL};
pub use filters::{cube::parse_cube, effective_source_size, FilterResourceData};
pub use native_preview::{NativePreview, PreviewOverlay};
pub use telestrator::{
    stroke_expired, TelePoint, TeleStroke, TeleTool, FADE_DURATION as TELESTRATOR_FADE_DURATION,
};

use thiserror::Error;

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Why the compositor could not start or render.
#[derive(Debug, Clone, Error)]
pub enum CompositorError {
    /// No GPU adapter at all — not even a software rasterizer.
    #[error("no usable GPU adapter was found")]
    NoAdapter,
    /// The adapter refused a device with the baseline limits.
    #[error("GPU device request failed: {0}")]
    Device(String),
    /// A frame whose geometry does not hold together (bad stride / short
    /// data / over the adapter's texture size limit).
    #[error("bad frame: {0}")]
    BadFrame(String),
    /// The program-frame readback failed.
    #[error("program readback failed: {0}")]
    Readback(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use fcap_capture::{Frame, PixelFormat};
    use fcap_scene::{
        BlendMode, Collection, ItemId, Rgba, SceneId, Source, SourceId, SourceSettings,
        StingerMatte, Transform,
    };
    use std::time::Instant;

    #[test]
    fn version_is_a_semver_triple() {
        assert_eq!(
            VERSION.split('.').count(),
            3,
            "workspace version should be MAJOR.MINOR.PATCH"
        );
    }

    /// ONE shared device for the whole suite. cargo runs tests on parallel
    /// threads, and CI's software rasterizers (WARP) cannot even survive
    /// *sequential* device churn — deferred destruction let ~22 create/drop
    /// cycles exhaust its memory budget. Every test locks the shared
    /// compositor and resizes its canvas; sources/chains/resources are keyed
    /// by per-test fresh UUIDs, so state never leaks between tests.
    static SHARED_GPU: std::sync::Mutex<Option<Compositor>> = std::sync::Mutex::new(None);

    struct TestCompositor {
        guard: std::sync::MutexGuard<'static, Option<Compositor>>,
    }

    impl std::ops::Deref for TestCompositor {
        type Target = Compositor;
        fn deref(&self) -> &Compositor {
            self.guard.as_ref().expect("initialized by compositor()")
        }
    }

    impl std::ops::DerefMut for TestCompositor {
        fn deref_mut(&mut self) -> &mut Compositor {
            self.guard.as_mut().expect("initialized by compositor()")
        }
    }

    /// GPU-backed tests skip (loudly) on machines with no adapter at all —
    /// CI stays green; real coverage runs wherever a GPU or software
    /// rasterizer exists (Windows WARP, Linux lavapipe, macOS Metal).
    fn compositor(width: u32, height: u32) -> Option<TestCompositor> {
        // A panicked test only poisons the mutex; the compositor inside is
        // structurally valid (stale per-test sources are keyed by UUIDs no
        // other test knows).
        let mut guard = SHARED_GPU
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if guard.is_none() {
            match Compositor::new(width, height) {
                Ok(comp) => {
                    eprintln!("compositor test adapter: {}", comp.adapter_summary());
                    *guard = Some(comp);
                }
                Err(CompositorError::NoAdapter) => {
                    eprintln!("SKIPPED: no GPU adapter available on this machine");
                    return None;
                }
                Err(other) => panic!("compositor bring-up failed: {other}"),
            }
        }
        guard
            .as_mut()
            .expect("just initialized")
            .set_canvas_size(width, height);
        Some(TestCompositor { guard })
    }

    fn solid_frame(width: u32, height: u32, format: PixelFormat, px: [u8; 4]) -> Frame {
        let mut data = Vec::with_capacity((width * height * 4) as usize);
        for _ in 0..width * height {
            data.extend_from_slice(&px);
        }
        Frame {
            width,
            height,
            stride: width * 4,
            format,
            data,
            captured_at: Instant::now(),
        }
    }

    /// A one-scene collection with one full-canvas item for `source`.
    fn scene_with_item(
        canvas: (u32, u32),
        settings_source: SourceId,
        transform: Transform,
        blend: BlendMode,
    ) -> Collection {
        let mut collection = Collection::new();
        collection.canvas_width = canvas.0;
        collection.canvas_height = canvas.1;
        let scene = collection.active_scene;
        // The pool needs *a* source entry; the compositor only cares about ids.
        let mut source = Source::new(
            "test",
            SourceSettings::Color {
                color: Rgba::WHITE,
                width: 4,
                height: 4,
            },
        );
        source.id = settings_source;
        collection.sources.push(source);
        let item = collection
            .add_item_with_existing_source(scene, settings_source)
            .expect("add item");
        collection
            .set_item_transform(scene, item, transform)
            .expect("set transform");
        collection
            .set_item_blend(scene, item, blend)
            .expect("set blend");
        collection
    }

    fn centered(canvas: (u32, u32)) -> Transform {
        Transform {
            x: canvas.0 as f32 / 2.0,
            y: canvas.1 as f32 / 2.0,
            ..Transform::default()
        }
    }

    fn pixel(frame: &ProgramFrame, x: u32, y: u32) -> [u8; 4] {
        let idx = ((y * frame.width + x) * 4) as usize;
        frame.data[idx..idx + 4].try_into().unwrap()
    }

    /// A 2×2 white/black checkerboard — the scaler goldens' probe pattern.
    fn checker_frame() -> Frame {
        let (w, b) = ([255u8, 255, 255, 255], [0u8, 0, 0, 255]);
        let mut data = Vec::with_capacity(16);
        for px in [w, b, b, w] {
            data.extend_from_slice(&px);
        }
        Frame {
            width: 2,
            height: 2,
            stride: 8,
            format: PixelFormat::Rgba8,
            data,
            captured_at: Instant::now(),
        }
    }

    fn pure(px: [u8; 4]) -> bool {
        px == [255, 255, 255, 255] || px == [0, 0, 0, 255]
    }

    /// The checkerboard scene at `scale` with one CAP-N70 scaling mode.
    fn scaled_checker_scene(
        canvas: (u32, u32),
        source: SourceId,
        scale: f32,
        mode: fcap_scene::ScaleMode,
    ) -> Collection {
        let mut transform = centered(canvas);
        transform.scale_x = scale;
        transform.scale_y = scale;
        let mut collection = scene_with_item(canvas, source, transform, BlendMode::Normal);
        let scene = collection.active_scene;
        let item = collection.active_scene().items[0].id;
        collection
            .set_item_scaling(scene, item, mode)
            .expect("set scaling");
        collection
    }

    // -- CAP-N53 per-output visibility golden --------------------------------

    #[test]
    fn output_hidden_items_skip_the_variant_render_and_return_on_clear() {
        let Some(mut comp) = compositor(4, 4) else {
            return;
        };
        let (white, red) = (SourceId::new(), SourceId::new());
        comp.upload_frame(
            white,
            &solid_frame(4, 4, PixelFormat::Rgba8, [255, 255, 255, 255]),
        )
        .expect("upload white");
        comp.upload_frame(
            red,
            &solid_frame(4, 4, PixelFormat::Rgba8, [255, 0, 0, 255]),
        )
        .expect("upload red");
        // A white base with a red overlay covering it.
        let mut collection = scene_with_item((4, 4), white, centered((4, 4)), BlendMode::Normal);
        let scene = collection.active_scene;
        let mut overlay = Source::new(
            "overlay",
            SourceSettings::Color {
                color: Rgba::WHITE,
                width: 4,
                height: 4,
            },
        );
        overlay.id = red;
        collection.sources.push(overlay);
        let overlay_item = collection
            .add_item_with_existing_source(scene, red)
            .expect("add overlay");
        collection
            .set_item_transform(scene, overlay_item, centered((4, 4)))
            .expect("overlay transform");

        // Full program: the overlay is on top.
        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        assert_eq!(pixel(&program, 2, 2), [255, 0, 0, 255]);

        // The stream-only/recording-only variant renders without it...
        comp.set_output_hidden(std::collections::HashSet::from([overlay_item]));
        comp.render(collection.active_scene(), 0.0).expect("render");
        let variant = comp.read_program().expect("readback");
        assert_eq!(
            pixel(&variant, 2, 2),
            [255, 255, 255, 255],
            "an output-hidden item must not be composed"
        );

        // ...and clearing the set restores the full program exactly.
        comp.set_output_hidden(std::collections::HashSet::new());
        comp.render(collection.active_scene(), 0.0).expect("render");
        let restored = comp.read_program().expect("readback");
        assert_eq!(pixel(&restored, 2, 2), [255, 0, 0, 255]);
    }

    // -- CAP-N70 per-scaler goldens (the 0.200.0 close-out owed these) -------

    #[test]
    fn nearest_scaling_keeps_every_checkerboard_pixel_pure() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(source, &checker_frame()).expect("upload");
        let collection = scaled_checker_scene((8, 8), source, 4.0, fcap_scene::ScaleMode::Nearest);
        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        for y in 0..8 {
            for x in 0..8 {
                assert!(
                    pure(pixel(&program, x, y)),
                    "nearest must never blend: ({x},{y}) = {:?}",
                    pixel(&program, x, y)
                );
            }
        }
        // The quadrants land where the texels say, edges razor-sharp.
        assert_eq!(pixel(&program, 1, 1), [255, 255, 255, 255]);
        assert_eq!(pixel(&program, 6, 1), [0, 0, 0, 255]);
        assert_eq!(pixel(&program, 3, 1), [255, 255, 255, 255]);
        assert_eq!(pixel(&program, 4, 1), [0, 0, 0, 255]);
    }

    #[test]
    fn auto_scaling_blends_the_checkerboard_boundary() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(source, &checker_frame()).expect("upload");
        let collection = scaled_checker_scene((8, 8), source, 4.0, fcap_scene::ScaleMode::Auto);
        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        // Smooth (linear) sampling must produce at least one in-between
        // value along the texel boundary — that mix IS the filter.
        let mixed = (0..8)
            .flat_map(|y| (0..8).map(move |x| (x, y)))
            .any(|(x, y)| !pure(pixel(&program, x, y)));
        assert!(mixed, "auto/linear scaling should blend somewhere");
    }

    #[test]
    fn sharp_bilinear_is_sharper_than_plain_bilinear_at_fractional_scale() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(source, &checker_frame()).expect("upload");
        // A fractional scale (×3.5) — where plain bilinear smears and
        // sharp-bilinear is supposed to keep pixels closer to the two
        // source values. The golden is the whole-frame RELATION (total
        // distance from purity), which is stable across rasterizers and
        // indifferent to where each mode places its transition band.
        let mut deviation = |mode: fcap_scene::ScaleMode| -> u32 {
            let collection = scaled_checker_scene((8, 8), source, 3.5, mode);
            comp.render(collection.active_scene(), 0.0).expect("render");
            let program = comp.read_program().expect("readback");
            (0..8)
                .flat_map(|y| (0..8).map(move |x| (x, y)))
                .map(|(x, y)| {
                    let r = pixel(&program, x, y)[0];
                    u32::from(r.min(255 - r))
                })
                .sum()
        };
        let auto = deviation(fcap_scene::ScaleMode::Auto);
        let sharp = deviation(fcap_scene::ScaleMode::SharpBilinear);
        assert!(
            auto > 0,
            "plain bilinear should blend somewhere at a fractional scale"
        );
        assert!(
            sharp < auto,
            "sharp-bilinear ({sharp}) must stay closer to the source than plain bilinear ({auto})"
        );
    }

    #[test]
    fn integer_scaling_snaps_the_drawn_size_to_whole_multiples() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(source, &checker_frame()).expect("upload");
        // ×3.5 rounds UP to ×4 (integer_snap rounds; reciprocals go down).
        // The item center sits off-grid at (4.25, 4.25) so the snap is
        // visible at a pixel CENTER: snapped ×4 spans 0.25..8.25 and paints
        // (0,0); an unsnapped ×3.5 draw would span 0.75..7.75 and leave
        // (0,0)'s center (0.5) uncovered.
        let mut transform = Transform {
            x: 4.25,
            y: 4.25,
            ..Transform::default()
        };
        transform.scale_x = 3.5;
        transform.scale_y = 3.5;
        let mut collection = scene_with_item((8, 8), source, transform, BlendMode::Normal);
        let scene = collection.active_scene;
        let item = collection.active_scene().items[0].id;
        collection
            .set_item_scaling(scene, item, fcap_scene::ScaleMode::Integer)
            .expect("set scaling");
        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        assert_eq!(
            pixel(&program, 0, 0),
            [255, 255, 255, 255],
            "×3.5 must snap to ×4 and cover (0,0) with the white texel"
        );
        // Integer mode samples nearest — every covered pixel stays pure,
        // and the quadrants land where ×4 texels (boundary at 4.25) say.
        for y in 0..8 {
            for x in 0..8 {
                let px = pixel(&program, x, y);
                assert!(pure(px), "integer scaling must be hard: ({x},{y}) = {px:?}");
            }
        }
        assert_eq!(pixel(&program, 6, 1), [0, 0, 0, 255]);
        assert_eq!(pixel(&program, 1, 6), [0, 0, 0, 255]);
        assert_eq!(pixel(&program, 6, 6), [255, 255, 255, 255]);
    }

    #[test]
    fn rgba_source_fills_the_canvas() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(
            source,
            &solid_frame(4, 4, PixelFormat::Rgba8, [255, 0, 0, 255]),
        )
        .expect("upload");
        let mut transform = centered((8, 8));
        transform.scale_x = 2.0;
        transform.scale_y = 2.0;
        let collection = scene_with_item((8, 8), source, transform, BlendMode::Normal);

        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        assert_eq!(program.width, 8);
        for y in 0..8 {
            for x in 0..8 {
                assert_eq!(pixel(&program, x, y), [255, 0, 0, 255], "at ({x},{y})");
            }
        }
    }

    #[test]
    fn bgra_uploads_swizzle_to_rgba() {
        let Some(mut comp) = compositor(4, 4) else {
            return;
        };
        let source = SourceId::new();
        // BGRA bytes: blue channel full → the program must read back blue.
        comp.upload_frame(
            source,
            &solid_frame(4, 4, PixelFormat::Bgra8, [255, 0, 0, 255]),
        )
        .expect("upload");
        let collection = scene_with_item((4, 4), source, centered((4, 4)), BlendMode::Normal);

        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        assert_eq!(pixel(&program, 2, 2), [0, 0, 255, 255], "BGRA → blue");
    }

    #[test]
    fn placement_is_pixel_accurate() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(
            source,
            &solid_frame(2, 2, PixelFormat::Rgba8, [255, 255, 255, 255]),
        )
        .expect("upload");
        // A 2×2 item centered at (4,4) covers exactly pixels [3..5)×[3..5).
        let collection = scene_with_item((8, 8), source, centered((8, 8)), BlendMode::Normal);

        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        for y in 0..8 {
            for x in 0..8 {
                let expected = if (3..5).contains(&x) && (3..5).contains(&y) {
                    [255, 255, 255, 255]
                } else {
                    [0, 0, 0, 255]
                };
                assert_eq!(pixel(&program, x, y), expected, "at ({x},{y})");
            }
        }
    }

    #[test]
    fn rotation_turns_the_content_clockwise() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(
            source,
            &solid_frame(6, 2, PixelFormat::Rgba8, [255, 255, 255, 255]),
        )
        .expect("upload");
        let mut transform = centered((8, 8));
        transform.rotation = 90.0;
        let collection = scene_with_item((8, 8), source, transform, BlendMode::Normal);

        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        // The 6×2 bar becomes 2×6: x ∈ [3..5), y ∈ [1..7).
        assert_eq!(pixel(&program, 3, 1), [255, 255, 255, 255]);
        assert_eq!(pixel(&program, 4, 6), [255, 255, 255, 255]);
        assert_eq!(pixel(&program, 1, 4), [0, 0, 0, 255], "left of the bar");
        assert_eq!(pixel(&program, 4, 0), [0, 0, 0, 255], "above the bar");
    }

    #[test]
    fn additive_blend_sums_the_layers() {
        let Some(mut comp) = compositor(4, 4) else {
            return;
        };
        let base = SourceId::new();
        let overlay = SourceId::new();
        comp.upload_frame(
            base,
            &solid_frame(4, 4, PixelFormat::Rgba8, [100, 100, 100, 255]),
        )
        .expect("upload base");
        comp.upload_frame(
            overlay,
            &solid_frame(4, 4, PixelFormat::Rgba8, [50, 60, 70, 255]),
        )
        .expect("upload overlay");

        let mut collection = scene_with_item((4, 4), base, centered((4, 4)), BlendMode::Normal);
        let scene = collection.active_scene;
        let mut source = Source::new(
            "overlay",
            SourceSettings::Color {
                color: Rgba::WHITE,
                width: 4,
                height: 4,
            },
        );
        source.id = overlay;
        collection.sources.push(source);
        let item = collection
            .add_item_with_existing_source(scene, overlay)
            .expect("add overlay");
        collection
            .set_item_transform(scene, item, centered((4, 4)))
            .expect("transform");
        collection
            .set_item_blend(scene, item, BlendMode::Additive)
            .expect("blend");

        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        assert_eq!(pixel(&program, 1, 1), [150, 160, 170, 255]);
    }

    #[test]
    fn crop_selects_the_subrect() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        // 4×4 source: top-left 2×2 quadrant red, the rest green.
        let mut data = Vec::new();
        for y in 0..4u32 {
            for x in 0..4u32 {
                if x < 2 && y < 2 {
                    data.extend_from_slice(&[255, 0, 0, 255]);
                } else {
                    data.extend_from_slice(&[0, 255, 0, 255]);
                }
            }
        }
        let frame = Frame {
            width: 4,
            height: 4,
            stride: 16,
            format: PixelFormat::Rgba8,
            data,
            captured_at: Instant::now(),
        };
        let source = SourceId::new();
        comp.upload_frame(source, &frame).expect("upload");

        let mut transform = centered((8, 8));
        transform.crop = fcap_scene::Crop {
            left: 0,
            top: 0,
            right: 2,
            bottom: 2,
        };
        let collection = scene_with_item((8, 8), source, transform, BlendMode::Normal);

        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        // Cropped content is the red 2×2, centered → pixels [3..5)².
        assert_eq!(pixel(&program, 3, 3), [255, 0, 0, 255]);
        assert_eq!(pixel(&program, 4, 4), [255, 0, 0, 255]);
        assert_eq!(pixel(&program, 5, 5), [0, 0, 0, 255], "outside the crop");
    }

    #[test]
    fn padded_strides_upload_and_read_back() {
        let Some(mut comp) = compositor(3, 2) else {
            return;
        };
        // 3×2 frame with a padded 64-byte stride (like DXGI row pitches).
        let stride = 64u32;
        let mut data = vec![0u8; (stride * 2) as usize];
        for y in 0..2usize {
            for x in 0..3usize {
                let at = y * stride as usize + x * 4;
                data[at..at + 4].copy_from_slice(&[10, 200, 30, 255]);
            }
        }
        let frame = Frame {
            width: 3,
            height: 2,
            stride,
            format: PixelFormat::Rgba8,
            data,
            captured_at: Instant::now(),
        };
        let source = SourceId::new();
        comp.upload_frame(source, &frame).expect("upload");
        let collection = scene_with_item(
            (3, 2),
            source,
            Transform {
                x: 1.5,
                y: 1.0,
                ..Transform::default()
            },
            BlendMode::Normal,
        );

        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        for y in 0..2 {
            for x in 0..3 {
                assert_eq!(pixel(&program, x, y), [10, 200, 30, 255], "at ({x},{y})");
            }
        }
    }

    #[test]
    fn an_empty_scene_is_opaque_black() {
        let Some(mut comp) = compositor(4, 4) else {
            return;
        };
        let collection = Collection::new();
        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        assert!(program.data.chunks(4).all(|px| px == [0, 0, 0, 255]));
    }

    #[test]
    fn broken_frames_are_rejected() {
        let Some(mut comp) = compositor(4, 4) else {
            return;
        };
        let source = SourceId::new();
        let mut short = solid_frame(4, 4, PixelFormat::Rgba8, [1, 2, 3, 255]);
        short.data.truncate(10);
        assert!(matches!(
            comp.upload_frame(source, &short),
            Err(CompositorError::BadFrame(_))
        ));

        let mut bad_stride = solid_frame(4, 4, PixelFormat::Rgba8, [1, 2, 3, 255]);
        bad_stride.stride = 8; // < width * 4
        assert!(matches!(
            comp.upload_frame(source, &bad_stride),
            Err(CompositorError::BadFrame(_))
        ));
    }

    // -- P2.3: the filter chain ---------------------------------------------

    use fcap_scene::{FilterKind, MaskMode};

    /// The first (and only) item of the collection's active scene.
    fn only_item(collection: &Collection) -> fcap_scene::ItemId {
        collection.active_scene().items[0].id
    }

    /// A 4×4 frame: left half white, right half black.
    fn half_and_half() -> Frame {
        let mut data = Vec::new();
        for _y in 0..4 {
            for x in 0..4u32 {
                if x < 2 {
                    data.extend_from_slice(&[255, 255, 255, 255]);
                } else {
                    data.extend_from_slice(&[0, 0, 0, 255]);
                }
            }
        }
        Frame {
            width: 4,
            height: 4,
            stride: 16,
            format: PixelFormat::Rgba8,
            data,
            captured_at: Instant::now(),
        }
    }

    #[test]
    fn chroma_key_removes_the_keyed_color() {
        let Some(mut comp) = compositor(4, 4) else {
            return;
        };
        let background = SourceId::new();
        let keyed = SourceId::new();
        comp.upload_frame(
            background,
            &solid_frame(4, 4, PixelFormat::Rgba8, [200, 0, 0, 255]),
        )
        .expect("upload background");
        comp.upload_frame(
            keyed,
            &solid_frame(4, 4, PixelFormat::Rgba8, [0, 255, 0, 255]),
        )
        .expect("upload keyed");

        let mut collection =
            scene_with_item((4, 4), background, centered((4, 4)), BlendMode::Normal);
        let scene = collection.active_scene;
        let mut source = Source::new(
            "keyed",
            SourceSettings::Color {
                color: Rgba::WHITE,
                width: 4,
                height: 4,
            },
        );
        source.id = keyed;
        collection.sources.push(source);
        let item = collection
            .add_item_with_existing_source(scene, keyed)
            .expect("add");
        collection
            .set_item_transform(scene, item, centered((4, 4)))
            .expect("transform");
        collection
            .add_filter(
                scene,
                item,
                FilterKind::ChromaKey {
                    key: Rgba::new(0, 255, 0, 255),
                    similarity: 0.4,
                    smoothness: 0.08,
                    spill: 0.1,
                },
            )
            .expect("add filter");

        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        // The pure-green overlay keys out entirely → the background shows.
        assert_eq!(pixel(&program, 2, 2), [200, 0, 0, 255]);
    }

    #[test]
    fn color_correction_brightness_and_opacity_apply() {
        let Some(mut comp) = compositor(2, 2) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(
            source,
            &solid_frame(2, 2, PixelFormat::Rgba8, [100, 100, 100, 255]),
        )
        .expect("upload");
        let mut collection = scene_with_item((2, 2), source, centered((2, 2)), BlendMode::Normal);
        let scene = collection.active_scene;
        let item = only_item(&collection);
        let filter = collection
            .add_filter(
                scene,
                item,
                FilterKind::ColorCorrection {
                    gamma: 0.0,
                    brightness: 1.0,
                    contrast: 0.0,
                    saturation: 1.0,
                    hue_shift: 0.0,
                    opacity: 1.0,
                },
            )
            .expect("add");

        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        assert_eq!(
            pixel(&program, 1, 1),
            [255, 255, 255, 255],
            "brightness 1 → white"
        );

        collection
            .update_filter(
                scene,
                item,
                filter,
                FilterKind::ColorCorrection {
                    gamma: 0.0,
                    brightness: 0.0,
                    contrast: 0.0,
                    saturation: 1.0,
                    hue_shift: 0.0,
                    opacity: 0.0,
                },
            )
            .expect("update");
        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        assert_eq!(
            pixel(&program, 1, 1),
            [0, 0, 0, 255],
            "opacity 0 → invisible"
        );
    }

    #[test]
    fn lut_maps_colors_through_the_lattice() {
        let Some(mut comp) = compositor(2, 2) else {
            return;
        };
        // An inverting 2³ LUT: every corner maps to its complement.
        let mut cube_text = String::from("LUT_3D_SIZE 2\n");
        for b in 0..2 {
            for g in 0..2 {
                for r in 0..2 {
                    cube_text.push_str(&format!("{} {} {}\n", 1 - r, 1 - g, 1 - b));
                }
            }
        }
        let lut = parse_cube(&cube_text).expect("parse");

        let source = SourceId::new();
        comp.upload_frame(
            source,
            &solid_frame(2, 2, PixelFormat::Rgba8, [51, 102, 204, 255]),
        )
        .expect("upload");
        let mut collection = scene_with_item((2, 2), source, centered((2, 2)), BlendMode::Normal);
        let scene = collection.active_scene;
        let item = only_item(&collection);
        let filter = collection
            .add_filter(
                scene,
                item,
                FilterKind::Lut {
                    path: "test.cube".into(),
                    amount: 1.0,
                },
            )
            .expect("add");
        comp.set_filter_resource(filter, &FilterResourceData::Lut3d(lut))
            .expect("resource");

        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        let px = pixel(&program, 0, 0);
        // Inverted (with 8-bit LUT + trilinear tolerance).
        assert!(
            (px[0] as i32 - 204).abs() <= 3
                && (px[1] as i32 - 153).abs() <= 3
                && (px[2] as i32 - 51).abs() <= 3,
            "expected ~inverted color, got {px:?}"
        );
    }

    #[test]
    fn unloaded_lut_renders_the_item_unfiltered() {
        let Some(mut comp) = compositor(2, 2) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(
            source,
            &solid_frame(2, 2, PixelFormat::Rgba8, [10, 20, 30, 255]),
        )
        .expect("upload");
        let mut collection = scene_with_item((2, 2), source, centered((2, 2)), BlendMode::Normal);
        let scene = collection.active_scene;
        let item = only_item(&collection);
        collection
            .add_filter(
                scene,
                item,
                FilterKind::Lut {
                    path: "not-loaded-yet.cube".into(),
                    amount: 1.0,
                },
            )
            .expect("add");

        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        assert_eq!(
            pixel(&program, 1, 1),
            [10, 20, 30, 255],
            "skipped, not black"
        );
    }

    #[test]
    fn blur_spreads_and_disabling_it_restores() {
        let Some(mut comp) = compositor(9, 9) else {
            return;
        };
        // A single bright impulse in the middle of black.
        let mut data = vec![0u8; 9 * 9 * 4];
        for px in data.chunks_exact_mut(4) {
            px[3] = 255;
        }
        let center = (4 * 9 + 4) * 4;
        data[center..center + 4].copy_from_slice(&[255, 255, 255, 255]);
        let frame = Frame {
            width: 9,
            height: 9,
            stride: 36,
            format: PixelFormat::Rgba8,
            data,
            captured_at: Instant::now(),
        };
        let source = SourceId::new();
        comp.upload_frame(source, &frame).expect("upload");
        let mut collection = scene_with_item((9, 9), source, centered((9, 9)), BlendMode::Normal);
        let scene = collection.active_scene;
        let item = only_item(&collection);
        let filter = collection
            .add_filter(scene, item, FilterKind::Blur { radius: 3.0 })
            .expect("add");

        comp.render(collection.active_scene(), 0.0).expect("render");
        let blurred = comp.read_program().expect("readback");
        let center_px = pixel(&blurred, 4, 4);
        let neighbor = pixel(&blurred, 6, 4);
        assert!(center_px[0] < 255, "energy spread away from the impulse");
        assert!(neighbor[0] > 0, "…and into the neighbors");
        assert_eq!(
            pixel(&blurred, 6, 4),
            pixel(&blurred, 2, 4),
            "gaussian is symmetric"
        );

        collection
            .set_filter_enabled(scene, item, filter, false)
            .expect("disable");
        comp.render(collection.active_scene(), 0.0).expect("render");
        let crisp = comp.read_program().expect("readback");
        assert_eq!(pixel(&crisp, 4, 4), [255, 255, 255, 255]);
        assert_eq!(pixel(&crisp, 6, 4), [0, 0, 0, 255], "disabled = identity");
    }

    /// CAP-N27: a horizontal directional blur streaks only along its axis —
    /// the impulse spreads left/right but never up/down.
    #[test]
    fn directional_blur_streaks_along_its_axis() {
        let Some(mut comp) = compositor(9, 9) else {
            return;
        };
        let mut data = vec![0u8; 9 * 9 * 4];
        for px in data.chunks_exact_mut(4) {
            px[3] = 255;
        }
        let center = (4 * 9 + 4) * 4;
        data[center..center + 4].copy_from_slice(&[255, 255, 255, 255]);
        let frame = Frame {
            width: 9,
            height: 9,
            stride: 36,
            format: PixelFormat::Rgba8,
            data,
            captured_at: Instant::now(),
        };
        let source = SourceId::new();
        comp.upload_frame(source, &frame).expect("upload");
        let mut collection = scene_with_item((9, 9), source, centered((9, 9)), BlendMode::Normal);
        let scene = collection.active_scene;
        let item = only_item(&collection);
        collection
            .add_filter(
                scene,
                item,
                FilterKind::DirectionalBlur {
                    radius: 3.0,
                    angle: 0.0,
                },
            )
            .expect("add");
        comp.render(collection.active_scene(), 0.0).expect("render");
        let out = comp.read_program().expect("readback");
        assert!(pixel(&out, 4, 4)[0] < 255, "the impulse spread");
        assert!(pixel(&out, 6, 4)[0] > 0, "…horizontally, along the angle");
        assert_eq!(pixel(&out, 4, 6), [0, 0, 0, 255], "but not vertically");
        assert_eq!(
            pixel(&out, 2, 4),
            pixel(&out, 6, 4),
            "symmetric on the axis"
        );
    }

    /// CAP-N27: pixelate collapses each block to one color — a 1px checkerboard
    /// under a full-frame block becomes a single flat color.
    #[test]
    fn pixelate_flattens_a_block() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        let mut data = vec![0u8; 8 * 8 * 4];
        for y in 0..8u32 {
            for x in 0..8u32 {
                let i = ((y * 8 + x) * 4) as usize;
                let v = if (x + y) % 2 == 0 { 255 } else { 0 };
                data[i..i + 4].copy_from_slice(&[v, v, v, 255]);
            }
        }
        let frame = Frame {
            width: 8,
            height: 8,
            stride: 32,
            format: PixelFormat::Rgba8,
            data,
            captured_at: Instant::now(),
        };
        let source = SourceId::new();
        comp.upload_frame(source, &frame).expect("upload");
        let mut collection = scene_with_item((8, 8), source, centered((8, 8)), BlendMode::Normal);
        let scene = collection.active_scene;
        let item = only_item(&collection);
        collection
            .add_filter(scene, item, FilterKind::Pixelate { size: 8.0 })
            .expect("add");
        comp.render(collection.active_scene(), 0.0).expect("render");
        let out = comp.read_program().expect("readback");
        // One 8×8 block → every pixel samples the same block center.
        let corner = pixel(&out, 0, 0);
        assert_eq!(pixel(&out, 7, 0), corner, "block is flat across x");
        assert_eq!(pixel(&out, 0, 7), corner, "block is flat across y");
        assert_eq!(pixel(&out, 7, 7), corner, "block is flat diagonally");
    }

    /// CAP-N27: zoom blur smears an off-center impulse along its ray from the
    /// center, not across it.
    #[test]
    fn zoom_blur_smears_toward_the_center() {
        let Some(mut comp) = compositor(9, 9) else {
            return;
        };
        let mut data = vec![0u8; 9 * 9 * 4];
        for px in data.chunks_exact_mut(4) {
            px[3] = 255;
        }
        // Impulse two pixels right of center (a horizontal ray from center).
        let impulse = (4 * 9 + 6) * 4;
        data[impulse..impulse + 4].copy_from_slice(&[255, 255, 255, 255]);
        let frame = Frame {
            width: 9,
            height: 9,
            stride: 36,
            format: PixelFormat::Rgba8,
            data,
            captured_at: Instant::now(),
        };
        let source = SourceId::new();
        comp.upload_frame(source, &frame).expect("upload");
        let mut collection = scene_with_item((9, 9), source, centered((9, 9)), BlendMode::Normal);
        let scene = collection.active_scene;
        let item = only_item(&collection);
        collection
            .add_filter(
                scene,
                item,
                FilterKind::ZoomBlur {
                    amount: 1.0,
                    center_x: 0.5,
                    center_y: 0.5,
                },
            )
            .expect("add");
        comp.render(collection.active_scene(), 0.0).expect("render");
        let out = comp.read_program().expect("readback");
        assert!(pixel(&out, 6, 4)[0] < 255, "the impulse spread");
        assert!(pixel(&out, 6, 4)[0] > 0, "…but energy stays on its ray");
        assert_eq!(
            pixel(&out, 6, 6),
            [0, 0, 0, 255],
            "nothing appears off the ray"
        );
    }

    /// CAP-N24: a downstream keyer composites its source over the finished
    /// program at its opacity — here a green layer at 50% over a red program.
    #[test]
    fn downstream_keyer_composites_over_the_program() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        let red = SourceId::new();
        comp.upload_frame(
            red,
            &solid_frame(8, 8, PixelFormat::Rgba8, [255, 0, 0, 255]),
        )
        .expect("upload red");
        let green = SourceId::new();
        comp.upload_frame(
            green,
            &solid_frame(8, 8, PixelFormat::Rgba8, [0, 255, 0, 255]),
        )
        .expect("upload green");
        let collection = scene_with_item((8, 8), red, centered((8, 8)), BlendMode::Normal);
        comp.render(collection.active_scene(), 0.0).expect("render");
        assert_eq!(pixel(&comp.read_program().unwrap(), 4, 4), [255, 0, 0, 255]);

        comp.render_downstream(&[DownstreamDraw {
            source: green,
            transform: centered((8, 8)),
            opacity: 0.5,
        }])
        .expect("downstream");
        let out = comp.read_program().expect("readback");
        let px = pixel(&out, 4, 4);
        assert!((px[0] as i32 - 127).abs() <= 4, "red halved: {px:?}");
        assert!((px[1] as i32 - 127).abs() <= 4, "green keyed in: {px:?}");

        // A disabled/absent keyer leaves the program untouched.
        comp.render(collection.active_scene(), 0.0).expect("render");
        comp.render_downstream(&[]).expect("no layers");
        assert_eq!(pixel(&comp.read_program().unwrap(), 4, 4), [255, 0, 0, 255]);
    }

    /// CAP-N57: a telestrator stroke is baked over the finished program — a
    /// green horizontal pen line lands on the pixels it crosses, and an empty
    /// stroke set leaves the program pixel-identical.
    #[test]
    fn telestrator_bakes_a_stroke_over_the_program() {
        let Some(mut comp) = compositor(32, 32) else {
            return;
        };
        let red = SourceId::new();
        comp.upload_frame(
            red,
            &solid_frame(32, 32, PixelFormat::Rgba8, [255, 0, 0, 255]),
        )
        .expect("upload red");
        let collection = scene_with_item((32, 32), red, centered((32, 32)), BlendMode::Normal);

        // A thick opaque-green horizontal line across the vertical middle.
        let stroke = TeleStroke {
            tool: TeleTool::Pen,
            color: [0.0, 1.0, 0.0, 1.0],
            width: 0.15,
            points: vec![
                TelePoint {
                    x: 0.1,
                    y: 0.5,
                    pressure: 1.0,
                },
                TelePoint {
                    x: 0.9,
                    y: 0.5,
                    pressure: 1.0,
                },
            ],
            fade_after: None,
            born_seconds: 0.0,
        };
        comp.render(collection.active_scene(), 0.0).expect("render");
        comp.render_telestrator(std::slice::from_ref(&stroke), 0.0)
            .expect("telestrator");
        let out = comp.read_program().expect("readback");
        let on_line = pixel(&out, 16, 16);
        assert!(
            on_line[1] > 200 && on_line[0] < 60,
            "the stroke painted green on the line: {on_line:?}"
        );
        assert_eq!(
            pixel(&out, 16, 0),
            [255, 0, 0, 255],
            "away from the stroke the program is untouched"
        );

        // No strokes → the program is exactly the composed red.
        comp.render(collection.active_scene(), 0.0).expect("render");
        comp.render_telestrator(&[], 0.0).expect("no strokes");
        assert_eq!(
            pixel(&comp.read_program().unwrap(), 16, 16),
            [255, 0, 0, 255]
        );

        // A fully-faded stroke also contributes nothing.
        let faded = TeleStroke {
            fade_after: Some(1.0),
            ..stroke
        };
        comp.render(collection.active_scene(), 0.0).expect("render");
        comp.render_telestrator(std::slice::from_ref(&faded), 100.0)
            .expect("faded");
        assert_eq!(
            pixel(&comp.read_program().unwrap(), 16, 16),
            [255, 0, 0, 255]
        );
    }

    /// CAP-N25: freeze is per-item — freezing one item holds its snapshot while
    /// a clone of the same source (CAP-N26) keeps updating.
    #[test]
    fn freezing_one_item_holds_it_while_a_clone_stays_live() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(
            source,
            &solid_frame(8, 8, PixelFormat::Rgba8, [255, 0, 0, 255]),
        )
        .expect("upload red");
        // Two items sharing one source: left half and right half.
        let left = Transform {
            x: 2.0,
            y: 4.0,
            scale_x: 0.5,
            scale_y: 1.0,
            ..Transform::default()
        };
        let right = Transform { x: 6.0, ..left };
        let mut collection = scene_with_item((8, 8), source, left, BlendMode::Normal);
        let scene = collection.active_scene;
        let item_left = only_item(&collection);
        let item_right = collection
            .add_item_with_existing_source(scene, source)
            .expect("clone");
        collection
            .set_item_transform(scene, item_right, right)
            .expect("transform");

        // Freeze the left item (snapshots red), then push a new frame to the source.
        comp.freeze_items(&[(item_left, source)]);
        comp.upload_frame(
            source,
            &solid_frame(8, 8, PixelFormat::Rgba8, [0, 255, 0, 255]),
        )
        .expect("upload green");
        comp.render(collection.active_scene(), 0.0).expect("render");
        let out = comp.read_program().expect("readback");
        assert_eq!(
            pixel(&out, 2, 4),
            [255, 0, 0, 255],
            "frozen item holds the old frame"
        );
        assert_eq!(pixel(&out, 6, 4), [0, 255, 0, 255], "its clone stays live");

        // Thaw: the left item resumes the live source.
        comp.retain_frozen(&[]);
        comp.render(collection.active_scene(), 0.0).expect("render");
        assert_eq!(
            pixel(&comp.read_program().unwrap(), 2, 4),
            [0, 255, 0, 255],
            "thaw resumes the live source"
        );
    }

    /// CAP-N29: a track-matte stinger derives per-pixel alpha from the matte
    /// half — fill shows where the matte is white, the program shows through
    /// where it is black — for both the horizontal and vertical splits.
    #[test]
    fn track_matte_stinger_keys_fill_by_the_matte_half() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        // A solid blue program to show through the transparent parts.
        let bg = SourceId::new();
        comp.upload_frame(bg, &solid_frame(8, 8, PixelFormat::Rgba8, [0, 0, 255, 255]))
            .expect("upload background");
        let collection = scene_with_item((8, 8), bg, centered((8, 8)), BlendMode::Normal);

        // Horizontal: red fill in the LEFT half; matte in the RIGHT half —
        // white (opaque) on top, black (transparent) on the bottom.
        let mut h = Vec::with_capacity(8 * 8 * 4);
        for y in 0..8u32 {
            for x in 0..8u32 {
                let px = if x < 4 {
                    [255, 0, 0, 255]
                } else if y < 4 {
                    [255, 255, 255, 255]
                } else {
                    [0, 0, 0, 255]
                };
                h.extend_from_slice(&px);
            }
        }
        let frame = Frame {
            width: 8,
            height: 8,
            stride: 32,
            format: PixelFormat::Rgba8,
            data: h,
            captured_at: Instant::now(),
        };
        comp.render_scene_with_stinger(
            collection.active_scene(),
            0.0,
            Some(&frame),
            StingerMatte::Horizontal,
        )
        .expect("horizontal matte");
        let out = comp.read_program().expect("readback");
        let top = pixel(&out, 2, 1);
        assert!(
            top[0] > 250 && top[1] < 5 && top[2] < 5,
            "white matte → the red fill is opaque: {top:?}"
        );
        let bottom = pixel(&out, 2, 6);
        assert!(
            bottom[2] > 250 && bottom[0] < 5 && bottom[1] < 5,
            "black matte → the blue program shows through: {bottom:?}"
        );

        // Vertical: red fill in the TOP half; matte in the BOTTOM half — white
        // (opaque) on the left, black (transparent) on the right.
        let mut v = Vec::with_capacity(8 * 8 * 4);
        for y in 0..8u32 {
            for x in 0..8u32 {
                let px = if y < 4 {
                    [255, 0, 0, 255]
                } else if x < 4 {
                    [255, 255, 255, 255]
                } else {
                    [0, 0, 0, 255]
                };
                v.extend_from_slice(&px);
            }
        }
        let frame = Frame {
            width: 8,
            height: 8,
            stride: 32,
            format: PixelFormat::Rgba8,
            data: v,
            captured_at: Instant::now(),
        };
        comp.render_scene_with_stinger(
            collection.active_scene(),
            0.0,
            Some(&frame),
            StingerMatte::Vertical,
        )
        .expect("vertical matte");
        let out = comp.read_program().expect("readback");
        let left = pixel(&out, 1, 2);
        assert!(
            left[0] > 250 && left[1] < 5 && left[2] < 5,
            "white matte → the red fill is opaque: {left:?}"
        );
        let right = pixel(&out, 6, 2);
        assert!(
            right[2] > 250 && right[0] < 5 && right[1] < 5,
            "black matte → the blue program shows through: {right:?}"
        );
    }

    /// CAP-N20: a move transition interpolates a matched item's transform —
    /// a source in both scenes travels from its outgoing position to its
    /// incoming one, reaching the midpoint at progress 0.5.
    #[test]
    fn move_transition_interpolates_a_matched_item() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(
            source,
            &solid_frame(8, 8, PixelFormat::Rgba8, [255, 0, 0, 255]),
        )
        .expect("upload red");

        // A 2px-wide vertical strip of the source, at x=2 in the outgoing scene
        // and x=6 in the incoming one — so its center travels 2 → 4 → 6.
        let strip = |x: f32| Transform {
            x,
            y: 4.0,
            scale_x: 0.25,
            scale_y: 1.0,
            ..Transform::default()
        };
        let mut collection = scene_with_item((8, 8), source, strip(2.0), BlendMode::Normal);
        let from_scene = collection.active_scene().clone();
        let scene_id = collection.active_scene;
        let item = collection.active_scene().items[0].id;
        collection
            .set_item_transform(scene_id, item, strip(6.0))
            .expect("to transform");
        let to_scene = collection.active_scene().clone();

        let is_red = |px: [u8; 4]| px[0] > 200 && px[1] < 50 && px[2] < 50;
        let is_black = |px: [u8; 4]| px[0] < 50 && px[1] < 50 && px[2] < 50;

        // progress 0 → the outgoing position (x=2).
        comp.render_move(&from_scene, &to_scene, 0.0, 0.0)
            .expect("p=0");
        let out = comp.read_program().expect("readback");
        assert!(is_red(pixel(&out, 2, 4)), "start: strip at x=2");
        assert!(is_black(pixel(&out, 6, 4)), "start: nothing at x=6");

        // progress 0.5 → the midpoint (x=4), nowhere near either end.
        comp.render_move(&from_scene, &to_scene, 0.5, 0.0)
            .expect("p=0.5");
        let out = comp.read_program().expect("readback");
        assert!(is_red(pixel(&out, 4, 4)), "mid: strip reached x=4");
        assert!(is_black(pixel(&out, 2, 4)), "mid: it left x=2");
        assert!(is_black(pixel(&out, 6, 4)), "mid: not yet at x=6");

        // progress 1 → the incoming position (x=6).
        comp.render_move(&from_scene, &to_scene, 1.0, 0.0)
            .expect("p=1");
        let out = comp.read_program().expect("readback");
        assert!(is_red(pixel(&out, 6, 4)), "end: strip at x=6");
        assert!(is_black(pixel(&out, 2, 4)), "end: nothing at x=2");
    }

    /// CAP-N22: a valid user WGSL effect runs (invert turns red → cyan); an
    /// invalid one is skipped so the item renders unfiltered — never a crash.
    #[test]
    fn user_shader_runs_and_a_bad_one_is_skipped() {
        use fcap_scene::FilterKind;
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(
            source,
            &solid_frame(8, 8, PixelFormat::Rgba8, [255, 0, 0, 255]),
        )
        .expect("upload red");
        let mut collection = scene_with_item((8, 8), source, centered((8, 8)), BlendMode::Normal);
        let scene = collection.active_scene;
        let item = collection.active_scene().items[0].id;

        // A valid invert effect: cyan out of red.
        let invert = "fn effect(uv: vec2<f32>, color: vec4<f32>, p: vec4<f32>, texel: vec4<f32>, time: f32) -> vec4<f32> { return vec4<f32>(vec3<f32>(1.0) - color.rgb, color.a); }";
        let fid = collection
            .add_filter(
                scene,
                item,
                FilterKind::UserShader {
                    source: invert.into(),
                    params: Vec::new(),
                },
            )
            .expect("add shader");
        comp.render(collection.active_scene(), 0.0).expect("render");
        assert_eq!(
            pixel(&comp.read_program().unwrap(), 4, 4),
            [0, 255, 255, 255],
            "the invert shader ran"
        );

        // Replace it with invalid WGSL: the pass is skipped, the item renders raw.
        collection
            .update_filter(
                scene,
                item,
                fid,
                FilterKind::UserShader {
                    source: "definitely not wgsl".into(),
                    params: Vec::new(),
                },
            )
            .expect("update shader");
        comp.render(collection.active_scene(), 0.0).expect("render");
        assert_eq!(
            pixel(&comp.read_program().unwrap(), 4, 4),
            [255, 0, 0, 255],
            "an invalid shader is skipped → the item renders unfiltered"
        );
    }

    #[test]
    fn mask_gates_alpha_by_the_image() {
        let Some(mut comp) = compositor(4, 4) else {
            return;
        };
        let background = SourceId::new();
        let masked = SourceId::new();
        comp.upload_frame(
            background,
            &solid_frame(4, 4, PixelFormat::Rgba8, [200, 0, 0, 255]),
        )
        .expect("upload background");
        comp.upload_frame(
            masked,
            &solid_frame(4, 4, PixelFormat::Rgba8, [255, 255, 255, 255]),
        )
        .expect("upload masked");

        let mut collection =
            scene_with_item((4, 4), background, centered((4, 4)), BlendMode::Normal);
        let scene = collection.active_scene;
        let mut source = Source::new(
            "masked",
            SourceSettings::Color {
                color: Rgba::WHITE,
                width: 4,
                height: 4,
            },
        );
        source.id = masked;
        collection.sources.push(source);
        let item = collection
            .add_item_with_existing_source(scene, masked)
            .expect("add");
        collection
            .set_item_transform(scene, item, centered((4, 4)))
            .expect("transform");
        let filter = collection
            .add_filter(
                scene,
                item,
                FilterKind::Mask {
                    path: "mask.png".into(),
                    mode: MaskMode::Alpha,
                    invert: false,
                },
            )
            .expect("add filter");

        // Mask image: left half transparent, right half opaque.
        let mut mask = Vec::new();
        for _y in 0..4 {
            for x in 0..4u32 {
                mask.extend_from_slice(&[255, 255, 255, if x < 2 { 0 } else { 255 }]);
            }
        }
        comp.set_filter_resource(
            filter,
            &FilterResourceData::Image {
                width: 4,
                height: 4,
                rgba: mask,
            },
        )
        .expect("resource");

        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        assert_eq!(
            pixel(&program, 0, 2),
            [200, 0, 0, 255],
            "masked away → background"
        );
        assert_eq!(
            pixel(&program, 3, 2),
            [255, 255, 255, 255],
            "kept → overlay"
        );
    }

    #[test]
    fn scroll_wraps_the_content() {
        let Some(mut comp) = compositor(4, 4) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(source, &half_and_half()).expect("upload");
        let mut collection = scene_with_item((4, 4), source, centered((4, 4)), BlendMode::Normal);
        let scene = collection.active_scene;
        let item = only_item(&collection);
        collection
            .add_filter(
                scene,
                item,
                FilterKind::Scroll {
                    speed_x: 2.0,
                    speed_y: 0.0,
                },
            )
            .expect("add");

        // t = 1s → offset = 2px = half the width → halves swap.
        comp.render(collection.active_scene(), 1.0).expect("render");
        let program = comp.read_program().expect("readback");
        assert_eq!(pixel(&program, 0, 1), [0, 0, 0, 255], "black scrolled in");
        assert_eq!(
            pixel(&program, 3, 1),
            [255, 255, 255, 255],
            "white wrapped around"
        );
    }

    /// Green base + a white overlay item — the probe pair for the Mike-FX
    /// goldens (Perspective / Fade Loop): where the overlay thins or fades,
    /// the green base shows through.
    fn green_base_white_overlay(comp: &mut Compositor) -> (Collection, SceneId, ItemId) {
        let (base, plane) = (SourceId::new(), SourceId::new());
        comp.upload_frame(
            base,
            &solid_frame(8, 8, PixelFormat::Rgba8, [0, 255, 0, 255]),
        )
        .expect("upload base");
        comp.upload_frame(
            plane,
            &solid_frame(8, 8, PixelFormat::Rgba8, [255, 255, 255, 255]),
        )
        .expect("upload overlay");
        let mut collection = scene_with_item((8, 8), base, centered((8, 8)), BlendMode::Normal);
        let scene = collection.active_scene;
        let mut source = Source::new(
            "overlay",
            SourceSettings::Color {
                color: Rgba::WHITE,
                width: 8,
                height: 8,
            },
        );
        source.id = plane;
        collection.sources.push(source);
        let item = collection
            .add_item_with_existing_source(scene, plane)
            .expect("add overlay");
        collection
            .set_item_transform(scene, item, centered((8, 8)))
            .expect("transform");
        (collection, scene, item)
    }

    #[test]
    fn perspective_tilts_the_plane_and_fades_the_far_edge() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        let (collection, scene, item) = green_base_white_overlay(&mut comp);
        let mut render_with = |tilt: f32, fade: f32| {
            let mut tilted = collection.clone();
            tilted
                .add_filter(scene, item, FilterKind::Perspective { tilt, fade })
                .expect("add filter");
            comp.render(tilted.active_scene(), 0.0).expect("render");
            comp.read_program().expect("readback")
        };
        // Tilted, no fade: the near (bottom) edge keeps its full width...
        let tilted = render_with(55.0, 0.0);
        assert_eq!(
            pixel(&tilted, 1, 7),
            [255, 255, 255, 255],
            "the near edge stays full width"
        );
        // ...the far corners fall off the narrowed plane (base shows)...
        assert_eq!(
            pixel(&tilted, 0, 0),
            [0, 255, 0, 255],
            "the far corner is off the plane"
        );
        // ...and the far middle is still the readable plane (Mike's capture
        // tilt: no forced fade).
        assert_eq!(
            pixel(&tilted, 4, 0),
            [255, 255, 255, 255],
            "the far middle stays readable"
        );
        // With full fade the far middle dims into the base.
        let faded = render_with(55.0, 1.0);
        let px = pixel(&faded, 4, 0);
        assert!(px[0] < 120, "the far edge should fade out, got {px:?}");
    }

    #[test]
    fn fade_loop_cycles_between_visible_and_hidden_forever() {
        let Some(mut comp) = compositor(8, 8) else {
            return;
        };
        let (mut collection, scene, item) = green_base_white_overlay(&mut comp);
        collection
            .add_filter(
                scene,
                item,
                FilterKind::FadeLoop {
                    fade_in_s: 1.0,
                    visible_s: 1.0,
                    fade_out_s: 1.0,
                    hidden_s: 1.0,
                },
            )
            .expect("add filter");
        let mut at = |t: f32| {
            comp.render(collection.active_scene(), t).expect("render");
            comp.read_program().expect("readback")
        };
        assert_eq!(pixel(&at(1.5), 4, 4), [255, 255, 255, 255], "held visible");
        assert_eq!(pixel(&at(3.5), 4, 4), [0, 255, 0, 255], "held hidden");
        let mid = pixel(&at(0.5), 4, 4);
        assert!(
            mid[0] > 100 && mid[0] < 155,
            "mid fade-in should blend, got {mid:?}"
        );
        // The whole point: the cycle repeats unattended (t = 4.5 ≡ 0.5).
        assert_eq!(mid, pixel(&at(4.5), 4, 4), "the loop repeats exactly");
    }

    /// A 4×4 frame: top half white, bottom half black — the vertical probe.
    fn top_and_bottom() -> Frame {
        let mut data = Vec::new();
        for y in 0..4u32 {
            for _x in 0..4 {
                if y < 2 {
                    data.extend_from_slice(&[255, 255, 255, 255]);
                } else {
                    data.extend_from_slice(&[0, 0, 0, 255]);
                }
            }
        }
        Frame {
            width: 4,
            height: 4,
            stride: 16,
            format: PixelFormat::Rgba8,
            data,
            captured_at: Instant::now(),
        }
    }

    #[test]
    fn filter_passes_preserve_vertical_orientation() {
        // Regression: the fullscreen pass must flip v (NDC y-up vs texture
        // v-down) or every ODD-length chain renders upside-down.
        let Some(mut comp) = compositor(4, 4) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(source, &top_and_bottom())
            .expect("upload");
        let mut collection = scene_with_item((4, 4), source, centered((4, 4)), BlendMode::Normal);
        let scene = collection.active_scene;
        let item = only_item(&collection);
        // A neutral one-pass filter: output should equal input, upright.
        collection
            .add_filter(
                scene,
                item,
                FilterKind::ColorCorrection {
                    gamma: 0.0,
                    brightness: 0.0,
                    contrast: 0.0,
                    saturation: 1.0,
                    hue_shift: 0.0,
                    opacity: 1.0,
                },
            )
            .expect("add");

        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        assert_eq!(
            pixel(&program, 1, 0),
            [255, 255, 255, 255],
            "top stays white"
        );
        assert_eq!(pixel(&program, 1, 3), [0, 0, 0, 255], "bottom stays black");
    }

    #[test]
    fn a_shrunken_filter_chain_stays_live() {
        // Regression: disabling a filter shrinks the pass count; the stale
        // extra chain texture must not keep being sampled (frozen frame).
        let Some(mut comp) = compositor(4, 4) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(
            source,
            &solid_frame(4, 4, PixelFormat::Rgba8, [200, 0, 0, 255]),
        )
        .expect("upload");
        let mut collection = scene_with_item((4, 4), source, centered((4, 4)), BlendMode::Normal);
        let scene = collection.active_scene;
        let item = only_item(&collection);
        collection
            .add_filter(scene, item, FilterKind::Sharpen { amount: 0.25 })
            .expect("sharpen");
        let blur = collection
            .add_filter(scene, item, FilterKind::Blur { radius: 2.0 })
            .expect("blur");
        // 3 passes cached (sharpen + blur H + blur V).
        comp.render(collection.active_scene(), 0.0).expect("render");
        let _ = comp.read_program().expect("readback");

        // Shrink the chain to 1 pass and change the source color — the
        // program must show the NEW color, not the frozen pass-2 texture.
        collection
            .set_filter_enabled(scene, item, blur, false)
            .expect("disable blur");
        comp.upload_frame(
            source,
            &solid_frame(4, 4, PixelFormat::Rgba8, [0, 200, 0, 255]),
        )
        .expect("upload green");
        comp.render(collection.active_scene(), 0.0).expect("render");
        let program = comp.read_program().expect("readback");
        let px = pixel(&program, 2, 2);
        assert!(
            px[1] > 150 && px[0] < 60,
            "live green, not a frozen red frame: {px:?}"
        );
    }

    #[test]
    fn oversized_canvas_requests_clamp_to_the_adapter() {
        let Some(mut comp) = compositor(4, 4) else {
            return;
        };
        // Must not panic inside wgpu validation; must clamp instead.
        comp.set_canvas_size(1_000_000, 1_000_000);
        let (w, h) = comp.canvas_size();
        assert!(w >= 4 && h >= 4, "still a usable canvas");
        assert!(w <= 32_768 && h <= 32_768, "clamped to a real limit");
        comp.render(&Collection::new().active_scene().clone(), 0.0)
            .expect("still renders");
    }

    #[test]
    fn filter_order_is_respected() {
        let Some(mut comp) = compositor(4, 4) else {
            return;
        };
        let source = SourceId::new();
        comp.upload_frame(source, &half_and_half()).expect("upload");

        // A: scroll half a width, then crop to the right half → white.
        let mut collection = scene_with_item((4, 4), source, centered((4, 4)), BlendMode::Normal);
        let scene = collection.active_scene;
        let item = only_item(&collection);
        collection
            .add_filter(
                scene,
                item,
                FilterKind::Scroll {
                    speed_x: 2.0,
                    speed_y: 0.0,
                },
            )
            .expect("scroll");
        collection
            .add_filter(
                scene,
                item,
                FilterKind::Crop {
                    left: 2,
                    top: 0,
                    right: 0,
                    bottom: 0,
                },
            )
            .expect("crop");
        comp.render(collection.active_scene(), 1.0).expect("render");
        let a = comp.read_program().expect("readback");
        // The chain output is 2×4, centered → columns 1..3.
        assert_eq!(
            pixel(&a, 1, 2),
            [255, 255, 255, 255],
            "scroll→crop keeps white"
        );

        // B: crop to the right half (black), then scroll — still black.
        let mut collection = scene_with_item((4, 4), source, centered((4, 4)), BlendMode::Normal);
        let scene = collection.active_scene;
        let item = only_item(&collection);
        collection
            .add_filter(
                scene,
                item,
                FilterKind::Crop {
                    left: 2,
                    top: 0,
                    right: 0,
                    bottom: 0,
                },
            )
            .expect("crop");
        collection
            .add_filter(
                scene,
                item,
                FilterKind::Scroll {
                    speed_x: 2.0,
                    speed_y: 0.0,
                },
            )
            .expect("scroll");
        comp.render(collection.active_scene(), 1.0).expect("render");
        let b = comp.read_program().expect("readback");
        assert_eq!(pixel(&b, 1, 2), [0, 0, 0, 255], "crop→scroll stays black");
    }

    /// The Phase 2 budget: 60 fps at 1080p with ≥4 sources. Hardware-gated —
    /// run on a real GPU with:
    /// `cargo test -p fcap-compositor --release -- --ignored perf_budget`
    /// (Software rasterizers on CI runners are honestly too slow to gate on.)
    #[test]
    #[ignore = "perf gate — run on real GPU hardware, in release mode"]
    fn perf_budget_60fps_1080p_4_sources() {
        let Some(mut comp) = compositor(1920, 1080) else {
            return;
        };
        let sources: Vec<SourceId> = (0..4).map(|_| SourceId::new()).collect();
        let frames: Vec<Frame> = (0..4)
            .map(|i| {
                solid_frame(
                    1920,
                    1080,
                    if i % 2 == 0 {
                        PixelFormat::Bgra8
                    } else {
                        PixelFormat::Rgba8
                    },
                    [60 * i as u8, 128, 200, 255],
                )
            })
            .collect();

        let mut collection = Collection::new();
        collection.canvas_width = 1920;
        collection.canvas_height = 1080;
        let scene = collection.active_scene;
        for (i, id) in sources.iter().enumerate() {
            let mut source = Source::new(
                format!("s{i}"),
                SourceSettings::Color {
                    color: Rgba::WHITE,
                    width: 4,
                    height: 4,
                },
            );
            source.id = *id;
            collection.sources.push(source);
            let item = collection
                .add_item_with_existing_source(scene, *id)
                .expect("add");
            collection
                .set_item_transform(
                    scene,
                    item,
                    Transform {
                        x: 480.0 + 320.0 * i as f32,
                        y: 270.0 + 180.0 * i as f32,
                        scale_x: 0.5,
                        scale_y: 0.5,
                        rotation: 7.0 * i as f32,
                        ..Transform::default()
                    },
                )
                .expect("transform");
        }

        // Warm up (first-frame allocations, pipeline warm).
        for (id, frame) in sources.iter().zip(&frames) {
            comp.upload_frame(*id, frame).expect("upload");
        }
        comp.render(collection.active_scene(), 0.0).expect("render");
        let _ = comp.read_program().expect("readback");

        let frames_to_time = 120u32;
        let started = Instant::now();
        for tick in 0..frames_to_time {
            // A realistic tick: all four sources deliver, compose at 60 fps,
            // read the program back at 30 fps (the preview's pace).
            for (id, frame) in sources.iter().zip(&frames) {
                comp.upload_frame(*id, frame).expect("upload");
            }
            comp.render(collection.active_scene(), tick as f32 / 60.0)
                .expect("render");
            if tick % 2 == 0 {
                let _ = comp.read_program().expect("readback");
            }
        }
        let elapsed = started.elapsed();
        let per_frame = elapsed / frames_to_time;
        eprintln!(
            "perf: {frames_to_time} composed frames in {elapsed:?} → {per_frame:?}/frame on {}",
            comp.adapter_summary()
        );
        assert!(
            per_frame.as_secs_f32() < 1.0 / 60.0,
            "budget: 60 fps at 1080p with 4 sources (got {per_frame:?}/frame)"
        );
    }
}
