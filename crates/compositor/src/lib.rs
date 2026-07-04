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
pub mod transform;

pub use compositor::{Compositor, ProgramFrame};
pub use filters::{cube::parse_cube, FilterResourceData};
pub use native_preview::{NativePreview, PreviewOverlay};

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
    use fcap_scene::{BlendMode, Collection, Rgba, Source, SourceId, SourceSettings, Transform};
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
