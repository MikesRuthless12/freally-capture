//! # OBS scene-collection importer (CAP-M02)
//!
//! Reads an OBS Studio scene-collection file (`*.json`, the shape saved under
//! `basic/scenes/`) and maps it into a native [`Collection`], returning an
//! honest [`ImportReport`] of exactly what came across and what did not.
//!
//! Why this is a *pure* function (JSON string in, `Collection` + report out):
//! it is fully unit-testable with embedded fixtures — no filesystem, no engine.
//! The app crate handles reading the file and writing/switching the result.
//!
//! ## What maps faithfully
//! scenes + their order, item z-order (OBS is top-first; we reverse), each
//! item's visibility / lock / blend mode, source kinds we have an equivalent
//! for (color, text, image, media, display/window/video/audio captures, nested
//! scenes), per-source video **and** audio filters we recognize, and mixer
//! volume/mute.
//!
//! ## What is approximated or dropped — and always reported
//! - **Geometry.** OBS positions items by an anchored top-left corner with an
//!   optional *bounds* box; our model is center-based and derives an item's
//!   pixel size from `native_size × scale`, which a scene file does not
//!   contain. So: an item **with bounds** rides the engine's first-frame
//!   fit-into-slot (an exact match for OBS's default "fit to bounds"); an
//!   unbounded **screen capture** fits the whole canvas; any other unbounded
//!   item keeps its scale/rotation/crop with an approximated position. Lossy
//!   cases raise [`ImportNote::GeometryApproximated`].
//! - **Device bindings** (which monitor / window / camera / audio device) are
//!   machine-specific; captures import as the right kind with an empty binding
//!   and [`ImportNote::NeedsReselect`].
//! - **Unsupported source kinds** (generic browser, decklink, …) and **groups**
//!   are skipped and listed in [`ImportReport::skipped`].
//!
//! Canvas resolution lives in the OBS *profile*, not the scene collection, so
//! the import defaults to 1920×1080 (repaired later if the user's canvas
//! differs — geometry rides first-frame fit regardless).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::audio::{AudioFilter, AudioFilterKind};
use crate::filter::{Filter, FilterKind, MaskMode};
use crate::scene::{BlendMode, NormRect, Scene, SceneId, SceneItem, Transform};
use crate::source::{Rgba, Source, SourceId, SourceSettings, TextAlign};
use crate::{Collection, MIN_VOLUME_DB};

/// The result of a successful import.
#[derive(Debug, Clone, PartialEq)]
pub struct ObsImport {
    /// The mapped, sanitized collection, ready to persist or hand to the engine.
    pub collection: Collection,
    /// An honest account of what did and did not come across.
    pub report: ImportReport,
}

/// A human-facing summary of an import — counts plus per-source caveats.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportReport {
    /// The OBS collection's own name (its `name` field), or `"Imported"`.
    pub name: String,
    pub scene_count: usize,
    pub source_count: usize,
    pub item_count: usize,
    /// Sources that came across but carry something worth knowing. Empty means
    /// every imported source mapped cleanly.
    pub notes: Vec<ImportedSource>,
    /// Sources with no native equivalent — dropped, with the OBS kind named so
    /// the user knows exactly what is missing.
    pub skipped: Vec<SkippedSource>,
}

/// One imported source and the caveats attached to it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportedSource {
    pub name: String,
    /// The OBS plugin id (e.g. `"monitor_capture"`), shown verbatim — technical,
    /// not translated.
    pub obs_kind: String,
    pub notes: Vec<ImportNote>,
}

/// A caveat on an imported source. A finite set, so the UI can translate each.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ImportNote {
    /// The device / monitor / window / audio binding is machine-specific and
    /// must be re-picked before the source will show anything.
    NeedsReselect,
    /// An OBS Game Capture — no direct equivalent; imported as a Window Capture.
    GameCaptureAsWindow,
    /// References a file on disk (image / media / LUT / mask / slideshow). Run
    /// the missing-file doctor to confirm it resolves on this machine.
    ReferencesFile,
    /// One or more of the source's OBS filters had no equivalent and were dropped.
    FilterDropped,
    /// The item's position/size were approximated from OBS's layout — review it.
    GeometryApproximated,
}

/// A source that could not be imported at all.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkippedSource {
    pub name: String,
    pub obs_kind: String,
    pub reason: SkipReason,
}

/// Why a source was skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SkipReason {
    /// No native source kind matches this OBS plugin.
    UnsupportedKind,
    /// An OBS group — nested grouping is not modeled; its members are not imported.
    Group,
}

/// Why an import failed outright (as opposed to a per-source caveat).
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ObsImportError {
    /// The file was not valid JSON.
    #[error("not valid JSON: {0}")]
    Json(String),
    /// Valid JSON, but not an OBS scene collection (no `sources` array).
    #[error("not an OBS scene collection (missing a \"sources\" array)")]
    NotObs,
    /// The collection contained no scenes to import.
    #[error("the OBS collection has no scenes")]
    Empty,
}

/// Parse an OBS scene-collection JSON string into a native [`Collection`] plus
/// an [`ImportReport`]. The collection is already `sanitize()`d.
pub fn import_obs(json: &str) -> Result<ObsImport, ObsImportError> {
    let root: Value =
        serde_json::from_str(json).map_err(|e| ObsImportError::Json(e.to_string()))?;
    let obs_sources = root
        .get("sources")
        .and_then(Value::as_array)
        .ok_or(ObsImportError::NotObs)?;

    // Index OBS sources by uuid and by name so items can reference either.
    let mut by_name: HashMap<&str, &Value> = HashMap::new();
    for s in obs_sources {
        if let Some(n) = get_str(s, "name") {
            by_name.insert(n, s);
        }
    }

    // Ordered scene names: prefer `scene_order`, else every scene source in file
    // order. Keep only names that resolve to an actual scene source.
    let ordered = root
        .get("scene_order")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|e| get_str(e, "name").map(str::to_string))
                .collect::<Vec<_>>()
        })
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| {
            obs_sources
                .iter()
                .filter(|s| base_kind(s) == "scene")
                .filter_map(|s| get_str(s, "name").map(str::to_string))
                .collect()
        });
    let scene_names: Vec<String> = ordered
        .into_iter()
        .filter(|n| by_name.get(n.as_str()).map(|s| base_kind(s)) == Some("scene".to_string()))
        .collect();
    if scene_names.is_empty() {
        return Err(ObsImportError::Empty);
    }

    // Pass 1 — a stable SceneId per scene name (needed before nested-scene sources).
    let mut scene_id_by_name: HashMap<String, SceneId> = HashMap::new();
    for n in &scene_names {
        scene_id_by_name.entry(n.clone()).or_default();
    }

    // Pass 2 — intern every source into the pool once, keyed by uuid and name.
    let mut builder = PoolBuilder::default();
    for s in obs_sources {
        let name = get_str(s, "name").unwrap_or("").to_string();
        let kind = base_kind(s);
        let settings = s.get("settings").cloned().unwrap_or(Value::Null);

        let mapped = match kind.as_str() {
            "scene" => {
                // A scene doubles as a nested-scene source; unreferenced ones are
                // GC'd by sanitize().
                let scene = scene_id_by_name.get(&name).copied();
                match scene {
                    Some(scene) => Mapped::Source(SourceSettings::NestedScene { scene }, vec![]),
                    None => Mapped::Unsupported,
                }
            }
            "group" => Mapped::Group,
            _ => map_source(&kind, &name, &settings),
        };

        match mapped {
            Mapped::Source(source_settings, mut source_notes) => {
                let is_visual = !source_settings.is_audio_only();
                let mut source = Source::new(name.clone(), source_settings);

                // Split OBS's per-source filter list into our video (per-item)
                // and audio (per-source) chains.
                let (video, audio, dropped, refs_file) = split_filters(s.get("filters"));
                if dropped {
                    push_note(&mut source_notes, ImportNote::FilterDropped);
                }
                if refs_file {
                    push_note(&mut source_notes, ImportNote::ReferencesFile);
                }
                apply_mixer(&mut source, s, audio);

                builder.intern(s, source, is_visual, video, kind, source_notes);
            }
            Mapped::Group => builder.skip(&name, &kind, SkipReason::Group),
            Mapped::Unsupported => builder.skip(&name, &kind, SkipReason::UnsupportedKind),
        }
    }

    // Pass 3 — build scenes and place items (reversing OBS's top-first order).
    let mut scenes: Vec<Scene> = Vec::with_capacity(scene_names.len());
    for name in &scene_names {
        let mut scene = Scene::new(name.clone());
        scene.id = scene_id_by_name[name];
        if let Some(src) = by_name.get(name.as_str()) {
            let items = src
                .get("settings")
                .and_then(|s| s.get("items"))
                .and_then(Value::as_array);
            if let Some(items) = items {
                for obs_item in items.iter().rev() {
                    if let Some(item) = builder.build_item(obs_item) {
                        scene.items.push(item);
                    }
                }
            }
        }
        scenes.push(scene);
    }

    // Active scene: the program scene, else the current scene, else the first.
    let active_name = get_str(&root, "current_program_scene")
        .or_else(|| get_str(&root, "current_scene"))
        .and_then(|n| scene_id_by_name.get(n).map(|_| n.to_string()));
    let active_scene = active_name
        .and_then(|n| scene_id_by_name.get(&n).copied())
        .unwrap_or(scenes[0].id);

    let mut collection = Collection {
        format_version: crate::FORMAT_VERSION,
        canvas_width: crate::default_canvas_width(),
        canvas_height: crate::default_canvas_height(),
        sources: builder.pool,
        scenes,
        active_scene,
        vertical: None,
    };
    collection.sanitize();

    let name = get_str(&root, "name").unwrap_or("Imported").to_string();
    let report = builder.notes.into_report(&collection, name);

    Ok(ObsImport { collection, report })
}

/// What a single OBS source maps to.
enum Mapped {
    /// A native source plus any caveats determined from its kind.
    Source(SourceSettings, Vec<ImportNote>),
    /// An OBS group — skipped.
    Group,
    /// No native equivalent — skipped.
    Unsupported,
}

/// Accumulates the source pool + the id maps + the running report while importing.
#[derive(Default)]
struct PoolBuilder {
    pool: Vec<Source>,
    /// OBS name → the pooled `SourceId`.
    by_name: HashMap<String, SourceId>,
    /// Pooled `SourceId` → its (uncloned) video filter template and screen flag.
    meta: HashMap<SourceId, SourceMeta>,
    notes: NotesBuilder,
}

struct SourceMeta {
    /// Whether the source draws anything — false for audio-only sources, which
    /// never get a "review placement" note even when added to a scene.
    is_visual: bool,
    video_filters: Vec<Filter>,
}

impl PoolBuilder {
    fn intern(
        &mut self,
        obs: &Value,
        source: Source,
        is_visual: bool,
        video_filters: Vec<Filter>,
        obs_kind: String,
        notes: Vec<ImportNote>,
    ) {
        let id = source.id;
        if let Some(name) = get_str(obs, "name") {
            self.by_name.insert(name.to_string(), id);
        }
        self.meta.insert(
            id,
            SourceMeta {
                is_visual,
                video_filters,
            },
        );
        self.notes.record(id, source.name.clone(), obs_kind, notes);
        self.pool.push(source);
    }

    fn skip(&mut self, name: &str, obs_kind: &str, reason: SkipReason) {
        self.notes.skip(name, obs_kind, reason);
    }

    /// Resolve an OBS scene item into a native [`SceneItem`], or `None` when its
    /// source was skipped / unresolved.
    fn build_item(&mut self, obs: &Value) -> Option<SceneItem> {
        let source_name = get_str(obs, "name")?;
        let source_id = *self.by_name.get(source_name)?;
        let meta = self.meta.get(&source_id)?;

        let (pending_fit, pending_slot, transform, lossy) = item_geometry(obs);
        if lossy && meta.is_visual {
            self.notes.add(source_id, ImportNote::GeometryApproximated);
        }

        // Clone the source's video filters onto this item with fresh ids (ids
        // are unique per item), preserving each filter's enabled state.
        let filters = meta
            .video_filters
            .iter()
            .map(|f| Filter {
                id: crate::FilterId::new(),
                enabled: f.enabled,
                kind: f.kind.clone(),
            })
            .collect();

        Some(SceneItem {
            id: crate::ItemId::new(),
            source: source_id,
            visible: get_bool(obs, "visible").unwrap_or(true),
            locked: get_bool(obs, "locked").unwrap_or(false),
            blend: blend_mode(get_str(obs, "blend_type")),
            transform,
            pending_fit,
            pending_slot,
            filters,
        })
    }
}

/// Collects per-source notes (deduped, in first-seen order) and skipped sources.
#[derive(Default)]
struct NotesBuilder {
    order: Vec<SourceId>,
    entries: HashMap<SourceId, ImportedSource>,
    skipped: Vec<SkippedSource>,
}

impl NotesBuilder {
    fn record(&mut self, id: SourceId, name: String, obs_kind: String, notes: Vec<ImportNote>) {
        let entry = self.entries.entry(id).or_insert_with(|| {
            self.order.push(id);
            ImportedSource {
                name,
                obs_kind,
                notes: Vec::new(),
            }
        });
        for note in notes {
            if !entry.notes.contains(&note) {
                entry.notes.push(note);
            }
        }
    }

    fn add(&mut self, id: SourceId, note: ImportNote) {
        if let Some(entry) = self.entries.get_mut(&id) {
            if !entry.notes.contains(&note) {
                entry.notes.push(note);
            }
        }
    }

    fn skip(&mut self, name: &str, obs_kind: &str, reason: SkipReason) {
        self.skipped.push(SkippedSource {
            name: name.to_string(),
            obs_kind: obs_kind.to_string(),
            reason,
        });
    }

    /// Finalize against the sanitized collection: drop notes for sources that
    /// were GC'd (unreferenced), and keep only sources that actually carry a note.
    fn into_report(self, collection: &Collection, name: String) -> ImportReport {
        let present: std::collections::HashSet<SourceId> =
            collection.sources.iter().map(|s| s.id).collect();
        let notes = self
            .order
            .iter()
            .filter(|id| present.contains(id))
            .filter_map(|id| self.entries.get(id))
            .filter(|e| !e.notes.is_empty())
            .cloned()
            .collect();
        let item_count = collection.scenes.iter().map(|s| s.items.len()).sum();
        ImportReport {
            name,
            scene_count: collection.scenes.len(),
            source_count: collection.sources.len(),
            item_count,
            notes,
            skipped: self.skipped,
        }
    }
}

// ---------------------------------------------------------------------------
// Source-kind mapping
// ---------------------------------------------------------------------------

/// Map a non-scene, non-group OBS source to a native [`SourceSettings`] plus the
/// kind-level caveats it implies.
fn map_source(kind: &str, name: &str, s: &Value) -> Mapped {
    let label = name.to_string();
    match kind {
        // Screen / display capture (Win / mac / Linux variants).
        "monitor_capture"
        | "display_capture"
        | "screen_capture"
        | "xshm_input"
        | "pipewire-screen-capture-source"
        | "pipewire-desktop-capture-source" => Mapped::Source(
            SourceSettings::Display {
                capture_id: String::new(),
                label,
            },
            vec![ImportNote::NeedsReselect],
        ),
        // Window capture.
        "window_capture" | "xcomposite_input" | "pipewire-window-capture-source" => Mapped::Source(
            SourceSettings::Window {
                capture_id: String::new(),
                label,
            },
            vec![ImportNote::NeedsReselect],
        ),
        // Game capture → the nearest thing we have is a Window Capture.
        "game_capture" => Mapped::Source(
            SourceSettings::Window {
                capture_id: String::new(),
                label,
            },
            vec![ImportNote::GameCaptureAsWindow, ImportNote::NeedsReselect],
        ),
        // Camera / video capture device.
        "dshow_input" | "v4l2_input" | "av_capture_input" | "macos-avcapture" => Mapped::Source(
            SourceSettings::VideoDevice {
                device_id: String::new(),
                format: None,
                deinterlace: crate::DeinterlaceMode::Off,
                field_order: crate::FieldOrder::TopFirst,
            },
            vec![ImportNote::NeedsReselect],
        ),
        "image_source" => Mapped::Source(
            SourceSettings::Image {
                path: get_str(s, "file").unwrap_or("").to_string(),
            },
            vec![ImportNote::ReferencesFile],
        ),
        "ffmpeg_source" => Mapped::Source(
            SourceSettings::Media {
                path: get_str(s, "local_file")
                    .or_else(|| get_str(s, "input"))
                    .unwrap_or("")
                    .to_string(),
                looping: get_bool(s, "looping").unwrap_or(false),
                hw_decode: true,
            },
            vec![ImportNote::ReferencesFile],
        ),
        "vlc_source" => Mapped::Source(
            SourceSettings::Media {
                path: first_playlist_path(s),
                looping: get_bool(s, "loop").unwrap_or(false),
                hw_decode: true,
            },
            vec![ImportNote::ReferencesFile],
        ),
        "color_source" => Mapped::Source(
            SourceSettings::Color {
                color: obs_color(get_u64(s, "color").unwrap_or(0xFFFF_9E4A)),
                width: get_u64(s, "width").unwrap_or(1920) as u32,
                height: get_u64(s, "height").unwrap_or(1080) as u32,
            },
            vec![],
        ),
        "text_gdiplus" | "text_ft2_source" => Mapped::Source(map_text(s), vec![]),
        "slideshow" => Mapped::Source(
            SourceSettings::Slideshow {
                paths: playlist_paths(s),
                slide_ms: get_f64(s, "slide_time").unwrap_or(5000.0) as u32,
                transition_ms: get_f64(s, "transition_time").unwrap_or(300.0) as u32,
                looping: get_bool(s, "loop").unwrap_or(true),
                shuffle: get_bool(s, "randomize").unwrap_or(false),
            },
            vec![ImportNote::ReferencesFile],
        ),
        // Audio.
        "wasapi_input_capture"
        | "pulse_input_capture"
        | "coreaudio_input_capture"
        | "alsa_input_capture" => Mapped::Source(
            SourceSettings::AudioInput {
                device_id: String::new(),
            },
            vec![ImportNote::NeedsReselect],
        ),
        "wasapi_output_capture"
        | "pulse_output_capture"
        | "coreaudio_output_capture"
        | "sck_audio_capture" => Mapped::Source(
            SourceSettings::AudioOutput {
                device_id: String::new(),
            },
            vec![ImportNote::NeedsReselect],
        ),
        "wasapi_process_output_capture" => Mapped::Source(
            SourceSettings::AppAudio {
                pid: 0,
                exe: String::new(),
            },
            vec![ImportNote::NeedsReselect],
        ),
        _ => Mapped::Unsupported,
    }
}

fn map_text(s: &Value) -> SourceSettings {
    let font = s.get("font");
    let font_family = font
        .and_then(|f| get_str(f, "face"))
        .map(str::to_string)
        .filter(|f| !f.is_empty());
    let size_px = font.and_then(|f| get_f64(f, "size")).unwrap_or(72.0) as f32;
    let color_raw = get_u64(s, "color")
        .or_else(|| get_u64(s, "color1"))
        .unwrap_or(0xFFFF_FFFF);
    let color = Rgba {
        a: 255,
        ..obs_color(color_raw)
    };
    let align = match get_str(s, "align") {
        Some("center") => TextAlign::Center,
        Some("right") => TextAlign::Right,
        _ => TextAlign::Left,
    };
    // OBS "read from file" maps onto the CAP-M16 whole-file binding.
    let source_file = match s.get("read_from_file").and_then(Value::as_bool) {
        Some(true) => get_str(s, "file").unwrap_or("").to_string(),
        _ => String::new(),
    };
    SourceSettings::Text {
        text: get_str(s, "text").unwrap_or("").to_string(),
        font_family,
        font_file: None,
        size_px,
        color,
        align,
        line_spacing: 1.0,
        force_rtl: false,
        wrap_width: None,
        source_file,
        binding: crate::FileBinding::Whole,
        csv_row: 1,
        csv_column: String::new(),
        json_pointer: String::new(),
    }
}

// ---------------------------------------------------------------------------
// Filter mapping
// ---------------------------------------------------------------------------

/// Split an OBS source's `filters` array into our video (per-item) and audio
/// (per-source) chains, plus flags for "something was dropped" and "a filter
/// references a file".
fn split_filters(filters: Option<&Value>) -> (Vec<Filter>, Vec<AudioFilter>, bool, bool) {
    let mut video = Vec::new();
    let mut audio = Vec::new();
    let mut dropped = false;
    let mut refs_file = false;
    let Some(arr) = filters.and_then(Value::as_array) else {
        return (video, audio, dropped, refs_file);
    };
    for f in arr {
        let kind = base_kind(f);
        let settings = f.get("settings").cloned().unwrap_or(Value::Null);
        let enabled = get_bool(f, "enabled").unwrap_or(true);
        if let Some((fk, file)) = map_video_filter(&kind, &settings) {
            refs_file |= file;
            video.push(Filter {
                id: crate::FilterId::new(),
                enabled,
                kind: fk,
            });
        } else if let Some(ak) = map_audio_filter(&kind, &settings) {
            audio.push(AudioFilter {
                id: crate::AudioFilterId::new(),
                enabled,
                kind: ak,
            });
        } else {
            dropped = true;
        }
    }
    (video, audio, dropped, refs_file)
}

/// Returns the mapped video filter and whether it references a file.
fn map_video_filter(kind: &str, s: &Value) -> Option<(FilterKind, bool)> {
    let f = match kind {
        "chroma_key_filter" => (
            FilterKind::ChromaKey {
                key: key_color(s),
                similarity: frac1000(s, "similarity", 400.0),
                smoothness: frac1000(s, "smoothness", 80.0),
                spill: frac1000(s, "spill", 100.0),
            },
            false,
        ),
        "color_key_filter" => (
            FilterKind::ColorKey {
                key: key_color(s),
                similarity: frac1000(s, "similarity", 400.0),
                smoothness: frac1000(s, "smoothness", 80.0),
            },
            false,
        ),
        "luma_key_filter" => (
            FilterKind::LumaKey {
                luma_min: clampf(get_f64(s, "luma_min").unwrap_or(0.0), 0.0, 1.0),
                luma_max: clampf(get_f64(s, "luma_max").unwrap_or(1.0), 0.0, 1.0),
                smoothness: clampf(
                    get_f64(s, "luma_max_smooth")
                        .or_else(|| get_f64(s, "luma_min_smooth"))
                        .unwrap_or(0.08),
                    0.0,
                    1.0,
                ),
            },
            false,
        ),
        "color_filter" => (
            FilterKind::ColorCorrection {
                gamma: clampf(get_f64(s, "gamma").unwrap_or(0.0), -3.0, 3.0),
                brightness: clampf(get_f64(s, "brightness").unwrap_or(0.0), -1.0, 1.0),
                contrast: clampf(get_f64(s, "contrast").unwrap_or(0.0), -1.0, 1.0),
                saturation: clampf(get_f64(s, "saturation").unwrap_or(1.0), 0.0, 4.0),
                hue_shift: clampf(get_f64(s, "hue_shift").unwrap_or(0.0), -180.0, 180.0),
                opacity: {
                    let raw = get_f64(s, "opacity").unwrap_or(100.0);
                    clampf(if raw > 1.0 { raw / 100.0 } else { raw }, 0.0, 1.0)
                },
            },
            false,
        ),
        "crop_filter" => (
            FilterKind::Crop {
                left: crop_px(s, "left"),
                top: crop_px(s, "top"),
                right: crop_px(s, "right"),
                bottom: crop_px(s, "bottom"),
            },
            false,
        ),
        "sharpness_filter" => (
            FilterKind::Sharpen {
                amount: clampf(get_f64(s, "sharpness").unwrap_or(0.08), 0.0, 2.0),
            },
            false,
        ),
        "mask_filter" => (
            FilterKind::Mask {
                path: get_str(s, "image_path")
                    .or_else(|| get_str(s, "path"))
                    .unwrap_or("")
                    .to_string(),
                mode: MaskMode::Alpha,
                invert: false,
            },
            true,
        ),
        "gpu_delay" | "async_delay_filter" => (
            FilterKind::RenderDelay {
                delay_ms: get_u64(s, "delay_ms").unwrap_or(0) as u32,
            },
            false,
        ),
        "clut_filter" | "lut_filter" => (
            FilterKind::Lut {
                path: get_str(s, "image_path")
                    .or_else(|| get_str(s, "file"))
                    .unwrap_or("")
                    .to_string(),
                amount: clampf(
                    get_f64(s, "clut_amount")
                        .or_else(|| get_f64(s, "amount"))
                        .unwrap_or(1.0),
                    0.0,
                    1.0,
                ),
            },
            true,
        ),
        _ => return None,
    };
    Some(f)
}

fn map_audio_filter(kind: &str, s: &Value) -> Option<AudioFilterKind> {
    let getf = |key: &str, default: f64| get_f64(s, key).unwrap_or(default) as f32;
    let f = match kind {
        "gain_filter" => AudioFilterKind::Gain {
            db: getf("db", 0.0),
        },
        "noise_gate_filter" => AudioFilterKind::NoiseGate {
            open_threshold_db: getf("open_threshold", -26.0),
            close_threshold_db: getf("close_threshold", -32.0),
            attack_ms: getf("attack_time", 25.0),
            hold_ms: getf("hold_time", 200.0),
            release_ms: getf("release_time", 150.0),
        },
        "compressor_filter" => AudioFilterKind::Compressor {
            ratio: getf("ratio", 4.0),
            threshold_db: getf("threshold", -18.0),
            attack_ms: getf("attack_time", 6.0),
            release_ms: getf("release_time", 60.0),
            output_gain_db: getf("output_gain", 0.0),
        },
        "limiter_filter" => AudioFilterKind::Limiter {
            threshold_db: getf("threshold", -3.0),
            release_ms: getf("release_time", 60.0),
        },
        "noise_suppress_filter" => AudioFilterKind::Denoise {
            strength: clampf(
                get_f64(s, "suppress_level").unwrap_or(-30.0).abs() / 60.0,
                0.0,
                1.0,
            ),
        },
        _ => return None,
    };
    Some(f)
}

/// Carry OBS mixer volume/mute and the mapped audio filter chain onto a source
/// (only meaningful for audio-capable kinds; `Source::new` already seeded audio).
fn apply_mixer(source: &mut Source, obs: &Value, audio_filters: Vec<AudioFilter>) {
    let Some(settings) = source.audio.as_mut() else {
        return;
    };
    if let Some(v) = get_f64(obs, "volume") {
        settings.volume_db = if v > 0.0 {
            (20.0 * v.log10() as f32).clamp(MIN_VOLUME_DB, crate::MAX_VOLUME_DB)
        } else {
            MIN_VOLUME_DB
        };
    }
    if let Some(muted) = get_bool(obs, "muted") {
        settings.muted = muted;
    }
    settings.filters = audio_filters;
    // sanitize() re-clamps at load, but keep the in-hand value tidy too.
    settings.clamp();
}

// ---------------------------------------------------------------------------
// Geometry
// ---------------------------------------------------------------------------

/// Resolve an OBS scene item's placement into our model. Returns
/// `(pending_fit, pending_slot, transform, lossy)` where `lossy` flags an
/// approximation worth reporting.
///
/// Our model has no explicit item pixel size — it derives one from
/// `native_size × scale` on the source's first frame — and a scene file does
/// not carry native sizes. So geometry rides the engine's first-frame fit:
/// - a **bounds** box maps to a normalized slot (`fit_into_slot` reproduces
///   OBS's default "fit to bounds"); rotation/crop are dropped there.
/// - an **unbounded** item fits the whole canvas — faithful for full-canvas
///   content (backgrounds, full-screen captures); a scaled/positioned unbounded
///   item is the rarer case (resizing in OBS creates a bounds box) and is
///   flagged lossy.
fn item_geometry(item: &Value) -> (bool, Option<NormRect>, Transform, bool) {
    let cw = crate::default_canvas_width() as f32;
    let ch = crate::default_canvas_height() as f32;

    let pos = item.get("pos").cloned().unwrap_or(Value::Null);
    let scale = item.get("scale").cloned().unwrap_or(Value::Null);
    let pos_x = get_f64(&pos, "x").unwrap_or(0.0) as f32;
    let pos_y = get_f64(&pos, "y").unwrap_or(0.0) as f32;
    let scale_x = get_f64(&scale, "x").unwrap_or(1.0) as f32;
    let scale_y = get_f64(&scale, "y").unwrap_or(1.0) as f32;
    let rot = get_f64(item, "rot").unwrap_or(0.0) as f32;
    let align = get_u64(item, "align").unwrap_or(5); // OBS default: top-left (LEFT|TOP)
    let has_crop = crop_px(item, "crop_left")
        | crop_px(item, "crop_top")
        | crop_px(item, "crop_right")
        | crop_px(item, "crop_bottom")
        != 0;

    let bounds_type = get_u64(item, "bounds_type").unwrap_or(0);
    let bounds = item.get("bounds").cloned().unwrap_or(Value::Null);
    let bw = get_f64(&bounds, "x").unwrap_or(0.0) as f32;
    let bh = get_f64(&bounds, "y").unwrap_or(0.0) as f32;

    if bounds_type != 0 && bw > 0.5 && bh > 0.5 {
        // Bounds box → a first-frame fit-into-slot (matches OBS "fit to bounds").
        // The slot path centers content and drops rotation/crop, so flag those.
        let left = anchor_min(pos_x, bw, align & 0x1 != 0, align & 0x2 != 0);
        let top = anchor_min(pos_y, bh, align & 0x4 != 0, align & 0x8 != 0);
        let slot = NormRect {
            x: left / cw,
            y: top / ch,
            w: bw / cw,
            h: bh / ch,
        };
        let lossy = rot != 0.0 || has_crop;
        return (true, Some(slot), Transform::default(), lossy);
    }

    // Unbounded → whole-canvas fit-and-center. Identity (native, unmoved) is
    // exact; a scaled/rotated/cropped/moved item is approximated.
    let identity = (scale_x - 1.0).abs() < 1e-3
        && (scale_y - 1.0).abs() < 1e-3
        && pos_x.abs() < 1.0
        && pos_y.abs() < 1.0
        && rot == 0.0
        && !has_crop;
    (true, None, Transform::default(), !identity)
}

/// The min (left or top) edge of an anchored box: `pos` is the box's `low`
/// edge, its `high` edge, or (default) its center.
fn anchor_min(pos: f32, size: f32, low: bool, high: bool) -> f32 {
    if low {
        pos
    } else if high {
        pos - size
    } else {
        pos - size * 0.5
    }
}

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

/// The OBS plugin id with any `_vN` version suffix stripped (`color_source_v3`
/// → `color_source`). Prefers `id`, falls back to `versioned_id`.
fn base_kind(v: &Value) -> String {
    let raw = get_str(v, "id")
        .or_else(|| get_str(v, "versioned_id"))
        .unwrap_or("");
    match raw.rsplit_once("_v") {
        Some((base, ver)) if !ver.is_empty() && ver.bytes().all(|b| b.is_ascii_digit()) => {
            base.to_string()
        }
        _ => raw.to_string(),
    }
}

/// Push a note if the vec does not already carry it.
fn push_note(notes: &mut Vec<ImportNote>, note: ImportNote) {
    if !notes.contains(&note) {
        notes.push(note);
    }
}

fn blend_mode(name: Option<&str>) -> BlendMode {
    match name {
        Some("additive") => BlendMode::Additive,
        Some("subtract") => BlendMode::Subtract,
        Some("screen") => BlendMode::Screen,
        Some("multiply") => BlendMode::Multiply,
        Some("lighten") => BlendMode::Lighten,
        Some("darken") => BlendMode::Darken,
        _ => BlendMode::Normal,
    }
}

/// Decode an OBS packed color (`0xAABBGGRR`). A zero alpha byte is treated as
/// fully opaque (OBS omits alpha on many color values).
fn obs_color(raw: u64) -> Rgba {
    let r = (raw & 0xFF) as u8;
    let g = ((raw >> 8) & 0xFF) as u8;
    let b = ((raw >> 16) & 0xFF) as u8;
    let a = ((raw >> 24) & 0xFF) as u8;
    Rgba {
        r,
        g,
        b,
        a: if a == 0 { 255 } else { a },
    }
}

/// A keyer's target color: an OBS preset name or a custom packed color.
fn key_color(s: &Value) -> Rgba {
    match get_str(s, "key_color_type") {
        Some("green") => Rgba {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        },
        Some("blue") => Rgba {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        },
        Some("magenta") => Rgba {
            r: 255,
            g: 0,
            b: 255,
            a: 255,
        },
        _ => Rgba {
            a: 255,
            ..obs_color(get_u64(s, "key_color").unwrap_or(0x0000_FF00))
        },
    }
}

/// An OBS `1..=1000` slider read as a `0.0..=1.0` fraction.
fn frac1000(s: &Value, key: &str, default: f64) -> f32 {
    clampf(get_f64(s, key).unwrap_or(default) / 1000.0, 0.0, 1.0)
}

fn clampf(v: f64, lo: f64, hi: f64) -> f32 {
    v.clamp(lo, hi) as f32
}

fn crop_px(v: &Value, key: &str) -> u32 {
    v.get(key).and_then(Value::as_i64).unwrap_or(0).max(0) as u32
}

/// The first entry of an OBS playlist / `files` array (VLC / slideshow).
fn first_playlist_path(s: &Value) -> String {
    playlist_paths(s).into_iter().next().unwrap_or_default()
}

fn playlist_paths(s: &Value) -> Vec<String> {
    s.get("playlist")
        .or_else(|| s.get("files"))
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|e| get_str(e, "value").map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn get_str<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    v.get(key).and_then(Value::as_str)
}
fn get_f64(v: &Value, key: &str) -> Option<f64> {
    v.get(key).and_then(Value::as_f64)
}
fn get_u64(v: &Value, key: &str) -> Option<u64> {
    v.get(key).and_then(Value::as_u64)
}
fn get_bool(v: &Value, key: &str) -> Option<bool> {
    v.get(key).and_then(Value::as_bool)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A representative OBS scene collection exercising two scenes, z-order,
    /// bounds vs unbounded geometry, a nested scene, every mapped source family,
    /// per-source video + audio filters (one of each unknown), a browser source
    /// (unsupported), and a group (skipped).
    fn fixture() -> &'static str {
        r#"{
          "name": "My Stream",
          "current_program_scene": "Intro",
          "current_scene": "Intro",
          "scene_order": [{"name":"Intro"},{"name":"Main"}],
          "sources": [
            {"name":"Intro","id":"scene","versioned_id":"scene","settings":{"items":[
              {"name":"Logo","visible":true,"locked":false,"align":5,"rot":0.0,
               "pos":{"x":100.0,"y":50.0},"scale":{"x":1.0,"y":1.0},
               "bounds_type":2,"bounds_align":0,"bounds":{"x":640.0,"y":360.0}},
              {"name":"Background","visible":true,"locked":false,"align":5,"rot":0.0,
               "pos":{"x":0.0,"y":0.0},"scale":{"x":1.0,"y":1.0},"bounds_type":0}
            ]}},
            {"name":"Main","id":"scene","versioned_id":"scene","settings":{"items":[
              {"name":"Desktop","visible":true,"locked":false,"align":5,
               "pos":{"x":0.0,"y":0.0},"scale":{"x":1.0,"y":1.0},"bounds_type":0},
              {"name":"Webcam","visible":false,"locked":true,"align":0,"rot":90.0,
               "pos":{"x":1600.0,"y":900.0},"scale":{"x":0.5,"y":0.5},"bounds_type":0,
               "crop_left":10,"crop_right":10},
              {"name":"Intro","visible":true,"locked":false,"align":5,
               "pos":{"x":0.0,"y":0.0},"scale":{"x":1.0,"y":1.0},"bounds_type":0},
              {"name":"Chat","visible":true,"locked":false,"align":5,
               "pos":{"x":0.0,"y":0.0},"scale":{"x":1.0,"y":1.0},"bounds_type":0},
              {"name":"MyGroup","visible":true,"locked":false,"align":5,
               "pos":{"x":0.0,"y":0.0},"scale":{"x":1.0,"y":1.0},"bounds_type":0},
              {"name":"Mic","visible":true,"locked":false,"align":5,
               "pos":{"x":0.0,"y":0.0},"scale":{"x":1.0,"y":1.0},"bounds_type":0}
            ]}},
            {"name":"Background","id":"color_source_v3","versioned_id":"color_source_v3",
             "settings":{"color":4278190080,"width":1920,"height":1080}},
            {"name":"Logo","id":"image_source","versioned_id":"image_source",
             "settings":{"file":"C:/img/logo.png"}},
            {"name":"Desktop","id":"monitor_capture","versioned_id":"monitor_capture",
             "settings":{"monitor":0}},
            {"name":"Webcam","id":"dshow_input","versioned_id":"dshow_input",
             "settings":{"video_device_id":"cam-xyz"},
             "filters":[
               {"name":"Key","id":"chroma_key_filter","versioned_id":"chroma_key_filter_v2",
                "enabled":true,"settings":{"key_color_type":"green","similarity":420,"smoothness":90,"spill":110}},
               {"name":"Weird","id":"streamfx-blur","enabled":true,"settings":{}}
             ]},
            {"name":"Mic","id":"wasapi_input_capture","versioned_id":"wasapi_input_capture",
             "volume":0.5,"muted":true,"settings":{"device_id":"mic-1"},
             "filters":[
               {"name":"Gain","id":"gain_filter","enabled":true,"settings":{"db":6.0}},
               {"name":"Gate","id":"noise_gate_filter","enabled":false,
                "settings":{"open_threshold":-24.0,"close_threshold":-30.0}}
             ]},
            {"name":"Chat","id":"browser_source","versioned_id":"browser_source",
             "settings":{"url":"https://example.com/chat"}},
            {"name":"MyGroup","id":"group","versioned_id":"group","settings":{"items":[]}}
          ]
        }"#
    }

    fn settings_by_name<'a>(c: &'a Collection, name: &str) -> &'a SourceSettings {
        &c.sources
            .iter()
            .find(|s| s.name == name)
            .unwrap_or_else(|| panic!("source {name} present"))
            .settings
    }

    #[test]
    fn imports_scenes_and_reverses_z_order() {
        let out = import_obs(fixture()).expect("import");
        let c = &out.collection;
        assert_eq!(out.report.name, "My Stream");
        assert_eq!(
            c.scenes.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(),
            ["Intro", "Main"]
        );
        // The program scene is honored as the active one.
        assert_eq!(c.active_scene, c.scenes[0].id);

        // Intro: OBS top-first [Logo, Background] → our bottom-first [Background, Logo].
        let intro = &c.scenes[0];
        assert_eq!(intro.items.len(), 2);
        assert!(matches!(
            c.sources
                .iter()
                .find(|s| s.id == intro.items[0].source)
                .unwrap()
                .settings,
            SourceSettings::Color { .. }
        ));
        assert!(matches!(
            c.sources
                .iter()
                .find(|s| s.id == intro.items[1].source)
                .unwrap()
                .settings,
            SourceSettings::Image { .. }
        ));

        // Main: Chat (unsupported) + MyGroup (group) items are dropped, leaving
        // 4, with the Desktop capture painted last (on top).
        let main = &c.scenes[1];
        assert_eq!(main.items.len(), 4);
        let top = c
            .sources
            .iter()
            .find(|s| s.id == main.items[3].source)
            .unwrap();
        assert_eq!(top.name, "Desktop");
    }

    #[test]
    fn maps_every_source_family_and_the_nested_scene() {
        let out = import_obs(fixture()).expect("import");
        let c = &out.collection;
        assert_eq!(out.report.source_count, 6);
        assert_eq!(out.report.item_count, 6);

        assert!(matches!(
            settings_by_name(c, "Background"),
            SourceSettings::Color {
                width: 1920,
                height: 1080,
                ..
            }
        ));
        assert!(matches!(
            settings_by_name(c, "Logo"),
            SourceSettings::Image { .. }
        ));
        assert!(matches!(
            settings_by_name(c, "Desktop"),
            SourceSettings::Display { .. }
        ));
        assert!(matches!(
            settings_by_name(c, "Webcam"),
            SourceSettings::VideoDevice { .. }
        ));
        assert!(matches!(
            settings_by_name(c, "Mic"),
            SourceSettings::AudioInput { .. }
        ));

        // The "Intro" source is the nested scene, pointing at the real Intro scene.
        let intro_id = c.scenes[0].id;
        match settings_by_name(c, "Intro") {
            SourceSettings::NestedScene { scene } => assert_eq!(*scene, intro_id),
            other => panic!("Intro should be a nested scene, got {other:?}"),
        }
    }

    #[test]
    fn bounds_ride_a_slot_and_unbounded_fits_the_canvas() {
        let out = import_obs(fixture()).expect("import");
        let intro = &out.collection.scenes[0];
        // Background (index 0) unbounded, native → whole-canvas fit, no slot.
        assert!(intro.items[0].pending_fit);
        assert_eq!(intro.items[0].pending_slot, None);
        // Logo (index 1) had a bounds box → a normalized slot the engine fits into.
        assert!(intro.items[1].pending_fit);
        let slot = intro.items[1]
            .pending_slot
            .expect("Logo keeps its bounds slot");
        assert!((slot.w - 640.0 / 1920.0).abs() < 1e-4);
        assert!((slot.x - 100.0 / 1920.0).abs() < 1e-4);
    }

    #[test]
    fn reports_caveats_and_skips_honestly() {
        let out = import_obs(fixture()).expect("import");
        let r = &out.report;

        // Skipped: the browser source and the group, each named with its OBS kind.
        assert_eq!(r.skipped.len(), 2);
        let chat = r.skipped.iter().find(|s| s.name == "Chat").unwrap();
        assert_eq!(chat.obs_kind, "browser_source");
        assert_eq!(chat.reason, SkipReason::UnsupportedKind);
        assert_eq!(
            r.skipped
                .iter()
                .find(|s| s.name == "MyGroup")
                .unwrap()
                .reason,
            SkipReason::Group
        );

        let note = |name: &str| {
            r.notes
                .iter()
                .find(|n| n.name == name)
                .map(|n| n.notes.clone())
        };
        assert_eq!(note("Logo"), Some(vec![ImportNote::ReferencesFile]));
        assert_eq!(note("Desktop"), Some(vec![ImportNote::NeedsReselect]));
        assert_eq!(note("Mic"), Some(vec![ImportNote::NeedsReselect]));
        let webcam = note("Webcam").expect("webcam noted");
        assert!(webcam.contains(&ImportNote::NeedsReselect));
        assert!(webcam.contains(&ImportNote::FilterDropped));
        assert!(webcam.contains(&ImportNote::GeometryApproximated));
        // A clean full-canvas background raises no note.
        assert_eq!(note("Background"), None);
    }

    #[test]
    fn maps_video_and_audio_filters() {
        let out = import_obs(fixture()).expect("import");
        let c = &out.collection;

        // The chroma key rode onto the Webcam item; the unknown filter was dropped.
        let webcam_src = c.sources.iter().find(|s| s.name == "Webcam").unwrap().id;
        let webcam_item = c
            .scenes
            .iter()
            .flat_map(|s| &s.items)
            .find(|i| i.source == webcam_src)
            .unwrap();
        assert_eq!(webcam_item.filters.len(), 1);
        assert!(matches!(
            webcam_item.filters[0].kind,
            FilterKind::ChromaKey {
                key: Rgba {
                    r: 0,
                    g: 255,
                    b: 0,
                    ..
                },
                ..
            }
        ));

        // The mic carried its two audio filters, volume, and mute.
        let mic = c.sources.iter().find(|s| s.name == "Mic").unwrap();
        let audio = mic.audio.as_ref().expect("mic keeps audio");
        assert_eq!(audio.filters.len(), 2);
        assert!(
            matches!(audio.filters[0].kind, AudioFilterKind::Gain { db } if (db - 6.0).abs() < 1e-3)
        );
        assert!(
            !audio.filters[1].enabled,
            "the disabled gate stays disabled"
        );
        assert!(audio.muted);
        assert!(
            (audio.volume_db - (-6.02)).abs() < 0.1,
            "0.5 linear ≈ -6 dB"
        );
    }

    #[test]
    fn result_survives_a_serde_round_trip() {
        let out = import_obs(fixture()).expect("import");
        let json = serde_json::to_string(&out.collection).expect("serialize");
        let back: Collection = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, out.collection);
        // The report is a serializable DTO for the UI.
        serde_json::to_string(&out.report).expect("report serializes");
    }

    #[test]
    fn rejects_input_that_is_not_an_obs_collection() {
        assert!(matches!(
            import_obs("not json"),
            Err(ObsImportError::Json(_))
        ));
        assert_eq!(import_obs("{}"), Err(ObsImportError::NotObs));
        // Valid sources array, but nothing is a scene.
        let no_scenes = r#"{"sources":[{"name":"x","id":"color_source","settings":{}}]}"#;
        assert_eq!(import_obs(no_scenes), Err(ObsImportError::Empty));
    }
}
