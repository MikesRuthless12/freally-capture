//! The studio runtime: the scene collection + the render loop behind it.
//!
//! Commands mutate the [`fcap_scene::Collection`] under [`StudioState`]'s
//! lock (each mutation bumps a revision, marks the autosave dirty, and pushes
//! the whole model to the UI on the `studio` event). The render thread
//! reconciles the *active scene* against reality on every tick:
//!
//! - capture-backed sources (display / window / portal / webcam) get their
//!   sessions started on helper threads (the Linux portal blocks on the
//!   user's pick) and their newest frames uploaded;
//! - static sources (image / color / text) render once per settings change;
//! - LUT / mask files load once per path and feed the filter chain;
//! - items awaiting their first frame get fitted + centered;
//!
//! then composes the program frame at ~60 fps, JPEG-encodes it at ~30 fps
//! into the `preview://` pipe, and reports honest per-source status + fps on
//! the `program` event (1 Hz, or immediately on a state change).
//!
//! The collection persists as `scene-collection.json` in the OS config dir —
//! atomic writes, debounced ~800 ms behind the last mutation, flushed on exit.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, Runtime};

use fcap_capture::{CaptureError, CaptureSession};
use fcap_compositor::{parse_cube, Compositor, CompositorError, FilterResourceData, ProgramFrame};
use fcap_scene::{
    AudioSettings, Collection, Crop, Filter, FilterId, FilterKind, History, HistoryState, ItemId,
    Scene, SceneError, SceneId, SceneItem, SourceId, SourceSettings, Transform,
};
use fcap_sources::video_device::{self, VideoFormatInfo};
use fcap_sources::{color, countdown, image, text};

use crate::preview::PreviewState;
use crate::settings::write_atomic;

const TICK: Duration = Duration::from_millis(16);
const READBACK_INTERVAL: Duration = Duration::from_millis(33);
const PROGRAM_EVENT_INTERVAL: Duration = Duration::from_secs(1);
const AUTOSAVE_DEBOUNCE: Duration = Duration::from_millis(800);
/// Multiview thumbnails (CAP-M06) refresh slower than the program — they don't
/// need 30 fps — and only a few render per tick so no single tick stalls on all
/// of them.
const MULTIVIEW_INTERVAL: Duration = Duration::from_millis(150);
const MULTIVIEW_BATCH: usize = 4;
/// Multiview thumbnail width in px; height derives from the canvas aspect.
const MULTIVIEW_THUMB_WIDTH: u32 = 320;
/// Auto-recover backoff (OBS-style): the first re-attempt of an errored
/// device/window capture fires this soon after it fails…
const AUTO_RETRY_MIN: Duration = Duration::from_secs(3);
/// …and each further attempt doubles the wait up to this cap, so a source whose
/// window/device never returns settles to an occasional retry instead of
/// thrashing its status every few seconds — while still recovering within the
/// cap once it comes back.
const AUTO_RETRY_MAX: Duration = Duration::from_secs(60);
const PREVIEW_MAX_WIDTH: u32 = 1280;
const PREVIEW_MAX_HEIGHT: u32 = 720;
const PREVIEW_JPEG_QUALITY: u8 = 75;
/// How long a finished countdown's face flashes (CAP-M15).
const TIMER_FLASH_WINDOW: Duration = Duration::from_secs(5);
/// The flash highlight color (straight RGBA) a finished countdown blinks in.
const TIMER_FLASH_COLOR: [u8; 4] = [239, 68, 68, 255];
/// V1-C: the countdown slate's message line renders at this fraction of the
/// number's font size (the number is the headline; the message is a caption).
const SLATE_MESSAGE_SCALE: f32 = 0.4;

/// A black or white outline, whichever contrasts the text colour — the ring
/// that keeps slate text readable over any background regardless of what the
/// operator picked for the text itself (V1-C). Rec. 601 luma.
fn contrasting_outline(color: [u8; 4]) -> [u8; 4] {
    let luma = 0.299 * color[0] as f32 + 0.587 * color[1] as f32 + 0.114 * color[2] as f32;
    if luma > 140.0 {
        [0, 0, 0, 255]
    } else {
        [255, 255, 255, 255]
    }
}
/// How often a bound Text source's file is polled (CAP-M16) — one stat per
/// poll while unchanged; OBS-comparable freshness.
const BOUND_TEXT_POLL: Duration = Duration::from_millis(500);
/// How often the stats HUD (CAP-N14) re-reads its numbers — matches the
/// stats emitter's sampling cadence; the face repaints only on change.
const STATS_FACE_INTERVAL: Duration = Duration::from_millis(500);
/// The system emoji face reaction sprites rasterize from (monochrome
/// outlines tinted per emoji — we own no color-emoji rasterizer, honestly).
#[cfg(target_os = "windows")]
const EMOJI_FONT: &str = "Segoe UI Emoji";
#[cfg(target_os = "macos")]
const EMOJI_FONT: &str = "Apple Color Emoji";
#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
const EMOJI_FONT: &str = "Noto Color Emoji";

/// V1-E: the featured chat banner's sprite key in the compositor's reaction
/// rig (an emoji sprite key is always a single emoji, so this can't collide).
const FEATURED_SPRITE: &str = "featured-banner";

// ---------------------------------------------------------------------------
// Shared state + the command-side mutation surface
// ---------------------------------------------------------------------------

/// What the `studio` event / `studio_get` carry: the whole model.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioDto {
    pub revision: u64,
    pub collection: Collection,
    /// Recently-live scenes for the "Recent Scenes" quick-recall list (V1-B),
    /// most-recent first. Session-only, so it rides the DTO rather than the
    /// collection's own (persisted) serialization; omitted while empty.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recent_scenes: Vec<SceneId>,
    /// Studio Mode (Phase 5): present while enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_mode: Option<StudioModeDto>,
    /// Undo/redo availability + the viewable history list (CAP-M01).
    pub history: HistoryState,
    /// Panic engaged (CAP-M22) — absent when false, so old fixtures hold.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub panic: bool,
}

/// The Studio-Mode slice of the model (session state, never persisted).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioModeDto {
    pub preview_scene: SceneId,
    /// A Preview→Program blend is running right now.
    pub transitioning: bool,
}

/// The media-hub id the stinger's decoded audio is keyed under (CAP-N29): the
/// producer (`start_media`) and the transition-duck consumer must agree on it.
const STINGER_HUB_ID: &str = "stinger-transition";

/// A running Preview→Program commit blend.
struct ActiveTransition {
    from: SceneId,
    kind: fcap_scene::TransitionKind,
    duration: Duration,
    started: Instant,
    /// The stinger video playing over the cut (TASK-606), when the kind is
    /// `Stinger`. Dropping it stops the decode.
    stinger: Option<fcap_capture::CaptureSession>,
    /// When (0..1 of the duration) the scene swap shows through a stinger.
    cut: f32,
    /// The custom luma-wipe image (tight RGBA), when the kind is `LumaImage`.
    luma: Option<Arc<(u32, u32, Vec<u8>)>>,
    /// Track-matte mode for a stinger (CAP-N29) — `None` for plain stingers.
    matte: fcap_scene::StingerMatte,
}

/// What the render loop pulls per tick while a transition runs.
pub(crate) struct TransitionFramePack {
    pub from_scene: fcap_scene::Scene,
    pub kind: fcap_scene::TransitionKind,
    pub progress: f32,
    pub cut: f32,
    /// The newest stinger video frame (None between frames — the last
    /// uploaded one keeps covering).
    pub stinger_frame: Option<fcap_capture::Frame>,
    pub luma: Option<Arc<(u32, u32, Vec<u8>)>>,
    /// Track-matte mode for a stinger (CAP-N29).
    pub matte: fcap_scene::StingerMatte,
}

/// Smootherstep easing (CAP-N21): a linear 0..1 progress eased to a smooth
/// accelerate-in / settle-out curve (zero first and second derivatives at both
/// ends) — the blend/move transitions read this so nothing starts or stops with
/// a jolt. Endpoints are exact (0→0, 1→1), so transition completion is unaffected.
fn ease_in_out(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Keying-workbench render mode (CAP-M26). The workbench shows one source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkbenchMode {
    /// The raw source, no filters — the "before" view + eyedropper source.
    Source,
    /// The source with its full filter chain (the keyed "after").
    Keyed,
    /// The keyer's matte: alpha rendered as opaque grayscale.
    Matte,
    /// Raw source on the left, keyed on the right, split at `split`.
    Split,
}

/// An open keying workbench: which item, how to render it, and (Split only) the
/// divider position 0..1.
#[derive(Debug, Clone, Copy)]
struct Workbench {
    item: ItemId,
    mode: WorkbenchMode,
    split: f32,
}

/// What a still-frame grab captures (CAP-M08).
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum StillTarget {
    /// The composed program frame.
    Program,
    /// A single source item, with or without its filter chain.
    Source { item: ItemId, pre_filter: bool },
}

/// A still resolved against the model, ready for the render thread to grab.
enum StillJob {
    Program,
    Source(SourceId, Vec<Filter>),
}

/// A projector's full-res render job for one due tick (CAP-M07 extension). The
/// `String` is the preview-slot key (`"scene:<id>"` / `"source:<id>"`).
enum ProjectorJob {
    Scene(String, Scene),
    Source(String, SourceId),
}

/// A scene or single source shown fullscreen by a projector window (CAP-M07
/// extension). Program/preview projectors reuse existing slots, so only these
/// two kinds need a dedicated full-res render each tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectorTarget {
    Scene(SceneId),
    Source(SourceId),
    /// CAP-N69: the low-latency passthrough monitor — the device's RAW
    /// frames, bypassing composition, filters, and every buffer the studio
    /// path adds. Published straight from the capture drain.
    Passthrough(SourceId),
}

impl ProjectorTarget {
    /// The preview-slot key: `"scene:<id>"` / `"source:<id>"` /
    /// `"passthrough:<id>"`.
    fn key(&self) -> String {
        match self {
            ProjectorTarget::Scene(id) => format!("scene:{}", id.0),
            ProjectorTarget::Source(id) => format!("source:{}", id.0),
            ProjectorTarget::Passthrough(id) => format!("passthrough:{}", id.0),
        }
    }
}

struct StudioCore {
    collection: Collection,
    revision: u64,
    path: Option<PathBuf>,
    dirty_since: Option<Instant>,
    /// Per-source retry counters (not persisted): bumping one changes the
    /// source's reconcile fingerprint, forcing the engine to restart it —
    /// the recovery path for errored captures (unplugged camera, permission
    /// granted after a denial, closed window reopened).
    retry_nonces: HashMap<SourceId, u64>,
    /// Punch-in zoom lenses (CAP-N71, not persisted): per-item target
    /// zoom/anchor/follow the render loop spring-animates each tick. Drawn
    /// only — the model's transforms and the undo history never see them.
    zoom_lenses: HashMap<ItemId, LensTarget>,
    /// Auto black-bar crop requests (CAP-N72, not persisted): one-shot
    /// `pending` detections and armed `follow` re-checks, keyed by item.
    autocrop: HashMap<ItemId, AutocropState>,
    /// CAP-N69: the passthrough monitor's measured capture→publish latency
    /// per source, in ms (a rolling mean; runtime-only).
    passthrough_latency: HashMap<SourceId, f32>,
    /// Studio Mode (Phase 5): the preview-side scene while enabled.
    preview_scene: Option<SceneId>,
    /// The blend a commit is currently rendering, if any.
    transition: Option<ActiveTransition>,
    /// Multi-step undo/redo for scene editing (CAP-M01). Snapshots ride
    /// alongside the collection under the same lock; not persisted to disk.
    undo: History,
    /// Monotonic base for undo gesture-coalescing timestamps (see
    /// [`History::edit`]). Not wall-clock — only elapsed millis matter.
    epoch: Instant,
    /// The open keying workbench, if any (CAP-M26). Render-only session state.
    workbench: Option<Workbench>,
    /// Whether a multiview monitor is open (CAP-M06): while true the render loop
    /// keeps every scene's sources live and publishes per-scene thumbnails.
    multiview: bool,
    /// A pending still-frame grab (CAP-M08): the render loop fulfils it once and
    /// clears it. The path is pre-computed by the command (which has settings).
    still_request: Option<(StillTarget, PathBuf)>,
    /// Open scene/source projectors (CAP-M07 extension): while non-empty the
    /// render loop keeps their sources live and publishes their full-res slots.
    projectors: HashSet<ProjectorTarget>,
    /// Panic (CAP-M22): while `Some`, the render loop composes this frozen
    /// slate instead of the program, every capture stops, and the audio
    /// engine runs no sources. Session state, never persisted.
    panic: Option<PanicPack>,
    /// Timer run state (CAP-M15): per-source countdown/stopwatch clocks,
    /// driven by commands/hotkeys and read by the render loop each tick.
    /// Session state, never persisted — a relaunch resets timers, honestly.
    timers: HashMap<SourceId, crate::timers::TimerRun>,
}

/// The frozen slate composition an engaged panic renders (CAP-M22): built
/// once at engage time so its synthetic source ids stay stable per tick.
#[derive(Debug, Clone)]
pub struct PanicPack {
    scene: Scene,
    sources: Vec<fcap_scene::Source>,
    vertical_scene: Option<Scene>,
}

/// Build the slate: a full-canvas colour fill, with the optional image
/// drawn at its native size, centered (a canvas-sized image fills exactly).
fn build_panic_pack(
    slate: &crate::settings::PanicSlateSettings,
    canvas: (u32, u32),
    vertical: Option<(u32, u32)>,
) -> PanicPack {
    use fcap_scene::{Rgba, Source, SourceSettings};

    let hex = slate.color.strip_prefix('#').unwrap_or("10141a");
    let channel =
        |at: usize| u8::from_str_radix(hex.get(at..at + 2).unwrap_or("10"), 16).unwrap_or(0x10);
    let color = Rgba::new(channel(0), channel(2), channel(4), 255);

    let mut sources = Vec::new();
    let image_source = (!slate.image.trim().is_empty()).then(|| {
        Source::new(
            "Panic Slate Image",
            SourceSettings::Image {
                path: slate.image.trim().to_string(),
            },
        )
    });

    let build_scene = |dims: (u32, u32), sources: &mut Vec<Source>| -> Scene {
        let fill = Source::new(
            "Panic Slate",
            SourceSettings::Color {
                color,
                width: dims.0,
                height: dims.1,
            },
        );
        let mut fill_item = SceneItem::new(fill.id);
        fill_item.transform = Transform {
            x: dims.0 as f32 / 2.0,
            y: dims.1 as f32 / 2.0,
            ..Default::default()
        };
        fill_item.pending_fit = false;
        let mut scene = Scene::new("panic");
        scene.items = vec![fill_item];
        if let Some(image) = &image_source {
            let mut image_item = SceneItem::new(image.id);
            image_item.transform = Transform {
                x: dims.0 as f32 / 2.0,
                y: dims.1 as f32 / 2.0,
                ..Default::default()
            };
            image_item.pending_fit = false;
            scene.items.push(image_item);
        }
        sources.push(fill);
        scene
    };

    let scene = build_scene(canvas, &mut sources);
    let vertical_scene = vertical.map(|dims| build_scene(dims, &mut sources));
    if let Some(image) = image_source {
        sources.push(image);
    }
    PanicPack {
        scene,
        sources,
        vertical_scene,
    }
}

/// The DTO for the current core state (one shape for every emit site).
fn dto_of(core: &StudioCore) -> StudioDto {
    StudioDto {
        revision: core.revision,
        recent_scenes: core.collection.recent_scenes().to_vec(),
        collection: core.collection.clone(),
        studio_mode: core.preview_scene.map(|preview_scene| StudioModeDto {
            preview_scene,
            transitioning: core.transition.is_some(),
        }),
        history: core.undo.state(),
        panic: core.panic.is_some(),
    }
}

/// Build an undo-coalesce key from an operation tag and the target it acts on,
/// so every frame of one continuous gesture (a drag, a fader ride) on the same
/// target folds into a single undo step. Different tags or targets never
/// coalesce. See [`History::edit`].
pub(crate) fn coalesce_key<T: std::hash::Hash>(tag: &str, target: T) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    tag.hash(&mut hasher);
    target.hash(&mut hasher);
    hasher.finish()
}

/// One punch-in lens's target state (CAP-N71) — what the commands set and
/// the render loop spring-animates toward. Runtime-only, never persisted.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LensTarget {
    /// Target zoom, `1..=8` (1 = flat; the loop forgets settled 1× lenses).
    pub zoom: f32,
    /// Zoom anchor in normalized content coordinates (0,0 = top-left).
    pub anchor: (f32, f32),
    /// Follow the OS cursor while zoomed (display/window captures;
    /// Windows-first — elsewhere the anchor stays manual).
    pub follow: bool,
}

/// One auto-crop request's state (CAP-N72) — runtime-only, never persisted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AutocropState {
    pub scene: SceneId,
    pub source: SourceId,
    /// Re-detect when the source's resolution changes.
    pub follow: bool,
    /// One-shot: detect + apply on the next frame, then clear.
    pub pending: bool,
}

/// A timer-control action (CAP-M15) — commands and hotkeys both drive these.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerCmd {
    Start,
    Pause,
    Toggle,
    Reset,
}

impl TimerCmd {
    /// Parse the wire form the UI sends.
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "start" => Some(TimerCmd::Start),
            "pause" => Some(TimerCmd::Pause),
            "toggle" => Some(TimerCmd::Toggle),
            "reset" => Some(TimerCmd::Reset),
            _ => None,
        }
    }
}

/// Tauri-managed handle to the studio model.
pub struct StudioState {
    core: Arc<Mutex<StudioCore>>,
}

impl StudioState {
    /// Load the *active* collection (per `workspace.json`) — so a scene-
    /// collection switch survives a restart — or start fresh.
    pub fn load_default() -> Self {
        let config_dir = crate::paths::config_dir();
        let path = config_dir
            .as_ref()
            .map(|dir| collection_file(dir, &active_collection_name(dir)));
        let collection = match &path {
            Some(path) => read_collection(path),
            None => {
                eprintln!("studio: no home directory — scenes will not persist this session");
                Collection::new()
            }
        };
        Self {
            core: Arc::new(Mutex::new(StudioCore {
                collection,
                revision: 1,
                path,
                dirty_since: None,
                retry_nonces: HashMap::new(),
                zoom_lenses: HashMap::new(),
                autocrop: HashMap::new(),
                passthrough_latency: HashMap::new(),
                preview_scene: None,
                transition: None,
                undo: History::new(),
                epoch: Instant::now(),
                workbench: None,
                multiview: false,
                still_request: None,
                projectors: HashSet::new(),
                panic: None,
                timers: HashMap::new(),
            })),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, StudioCore> {
        self.core
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Force one source to restart on the engine's next tick. No model
    /// change and nothing persisted — just a reconcile nudge (the recovery
    /// path for errored captures).
    pub fn retry_source(&self, source: SourceId) -> Result<(), String> {
        let mut core = self.lock();
        if core.collection.source(source).is_none() {
            return Err("source not found".to_string());
        }
        *core.retry_nonces.entry(source).or_insert(0) += 1;
        core.revision += 1;
        Ok(())
    }

    /// Engage/restore the panic slate (CAP-M22). Engaging freezes the slate
    /// config; the next render tick swaps the composed world for it, stops
    /// every capture, and empties the audio engine. Restoring reconciles
    /// everything back. Idempotent; emits `studio` on every real change.
    pub fn set_panic<R: Runtime>(&self, app: &AppHandle<R>, on: bool) {
        let dto = {
            let mut core = self.lock();
            if on == core.panic.is_some() {
                return;
            }
            core.panic = on.then(|| {
                let slate = app
                    .state::<crate::settings::SettingsStore>()
                    .get()
                    .panic_slate;
                build_panic_pack(
                    &slate,
                    (core.collection.canvas_width, core.collection.canvas_height),
                    core.collection
                        .vertical
                        .map(|config| (config.width, config.height)),
                )
            });
            core.revision += 1;
            dto_of(&core)
        };
        println!("studio: panic {}", if on { "ENGAGED" } else { "restored" });
        let _ = app.emit("studio", &dto);
    }

    /// Whether the panic slate is engaged (CAP-M22).
    pub fn is_panicked(&self) -> bool {
        self.lock().panic.is_some()
    }

    /// Clear the "Recent Scenes" quick-recall list (V1-B). Bumps the revision
    /// and refreshes the UI, but never marks the project dirty (nothing
    /// persisted changed) and never touches the undo stack. A no-op — and no
    /// emit — when the list is already empty.
    pub fn clear_recent_scenes<R: Runtime>(&self, app: &AppHandle<R>) {
        let dto = {
            let mut core = self.lock();
            if core.collection.recent_scenes().is_empty() {
                return;
            }
            core.collection.clear_recent_scenes();
            core.revision += 1;
            dto_of(&core)
        };
        let _ = app.emit("studio", &dto);
    }

    /// A snapshot for `studio_get` / event payloads.
    pub fn snapshot(&self) -> StudioDto {
        dto_of(&self.lock())
    }

    /// Whether the panic slate currently holds the program (CAP-M22) — read
    /// at recording start so ISO lanes born mid-panic start held (CAP-N40).
    pub fn panic_active(&self) -> bool {
        self.lock().panic.is_some()
    }

    /// Read the live collection under the lock (CAP-M03 missing-file scans).
    pub fn with_collection<T>(&self, f: impl FnOnce(&Collection) -> T) -> T {
        f(&self.lock().collection)
    }

    /// Open/update the keying workbench (CAP-M26): render `item` in `mode`. No
    /// model change and nothing emitted — the render thread picks it up and
    /// publishes to the `workbench-preview` slot.
    pub fn set_workbench(&self, item: ItemId, mode: WorkbenchMode, split: f32) {
        self.lock().workbench = Some(Workbench {
            item,
            mode,
            split: split.clamp(0.0, 1.0),
        });
    }

    /// Close the keying workbench (the render loop clears the preview slot).
    pub fn close_workbench(&self) {
        self.lock().workbench = None;
    }

    /// Open/close the multiview monitor (CAP-M06). While open, the render loop
    /// keeps every scene's sources live and publishes per-scene thumbnails to
    /// the `/multiview/<id>` slots; closing it clears them.
    pub fn set_multiview(&self, on: bool) {
        self.lock().multiview = on;
    }

    /// Register/deregister a scene or source projector (CAP-M07 extension). The
    /// render loop reads the set each tick to publish/retire the full-res slots.
    /// Returns whether the set changed.
    pub fn set_projector(&self, target: ProjectorTarget, on: bool) -> bool {
        let mut core = self.lock();
        if on {
            core.projectors.insert(target)
        } else {
            core.projectors.remove(&target)
        }
    }

    /// Whether a projector target still resolves in the current collection (used
    /// to skip stale scene/source projectors when reopening on launch).
    pub fn has_target(&self, target: ProjectorTarget) -> bool {
        let core = self.lock();
        match target {
            ProjectorTarget::Scene(id) => core.collection.scene(id).is_some(),
            ProjectorTarget::Source(id) | ProjectorTarget::Passthrough(id) => {
                core.collection.source(id).is_some()
            }
        }
    }

    /// Queue a still-frame grab (CAP-M08); the render loop saves it to `path`
    /// on its next tick and emits `still-saved`/`still-error`.
    pub fn request_still(&self, target: StillTarget, path: PathBuf) {
        self.lock().still_request = Some((target, path));
    }

    /// Drive one timer source's run state (CAP-M15). Runtime-only — no model
    /// change, no undo history, nothing emitted; the render loop repaints the
    /// face on its next tick. Unknown ids are a quiet no-op.
    /// Set a punch-in lens's target zoom (CAP-N71). Absolute: `zoom` clamps
    /// to `1..=8`; `anchor` (normalized content coords) replaces the stored
    /// one when given, else the existing anchor (center for a new lens)
    /// stays. A lens at 1× with follow off is removed — the loop animates
    /// back to flat and forgets it. Runtime-only: no revision, no undo.
    pub fn zoom_set(&self, item: ItemId, zoom: f32, anchor: Option<(f32, f32)>) {
        let mut core = self.lock();
        let entry = core.zoom_lenses.entry(item).or_insert(LensTarget {
            zoom: 1.0,
            anchor: (0.5, 0.5),
            follow: false,
        });
        if zoom.is_finite() {
            entry.zoom = zoom.clamp(1.0, 8.0);
        }
        if let Some((ax, ay)) = anchor {
            if ax.is_finite() && ay.is_finite() {
                entry.anchor = (ax.clamp(0.0, 1.0), ay.clamp(0.0, 1.0));
            }
        }
        if entry.zoom <= 1.0 + f32::EPSILON && !entry.follow {
            core.zoom_lenses.remove(&item);
        }
    }

    /// Multiply a lens's target zoom (the canvas wheel) about `anchor`.
    pub fn zoom_scroll(&self, item: ItemId, factor: f32, anchor: (f32, f32)) {
        if !factor.is_finite() || factor <= 0.0 {
            return;
        }
        let current = self
            .lock()
            .zoom_lenses
            .get(&item)
            .map_or(1.0, |lens| lens.zoom);
        self.zoom_set(item, current * factor, Some(anchor));
    }

    /// Toggle follow-the-cursor panning for a lens (CAP-N71; Windows maps
    /// the cursor into display/window captures — elsewhere the anchor just
    /// stays manual and the UI says so).
    pub fn zoom_follow(&self, item: ItemId, follow: bool) {
        let mut core = self.lock();
        if follow {
            core.zoom_lenses
                .entry(item)
                .or_insert(LensTarget {
                    zoom: 1.0,
                    anchor: (0.5, 0.5),
                    follow: false,
                })
                .follow = true;
        } else if let Some(lens) = core.zoom_lenses.get_mut(&item) {
            lens.follow = false;
            if lens.zoom <= 1.0 + f32::EPSILON {
                core.zoom_lenses.remove(&item);
            }
        }
    }

    /// A lens's current target, for the UI (`(zoom, follow)`).
    pub fn zoom_get(&self, item: ItemId) -> (f32, bool) {
        self.lock()
            .zoom_lenses
            .get(&item)
            .map_or((1.0, false), |lens| (lens.zoom, lens.follow))
    }

    /// Request a one-shot auto black-bar crop (CAP-N72) on `item`: the next
    /// frame of its source is scanned and the detected crop applied as an
    /// undoable edit. Errors when the item is missing or is the backdrop.
    pub fn autocrop_request(&self, scene: SceneId, item: ItemId) -> Result<(), String> {
        let mut core = self.lock();
        let source = resolve_autocrop_target(&core.collection, scene, item)?;
        core.autocrop
            .entry(item)
            .and_modify(|state| {
                state.pending = true;
                state.scene = scene;
                state.source = source;
            })
            .or_insert(AutocropState {
                scene,
                source,
                follow: false,
                pending: true,
            });
        Ok(())
    }

    /// Arm or disarm continuous re-checks on resolution change (CAP-N72's
    /// follow mode; off by default per the DoD).
    pub fn autocrop_set_follow(
        &self,
        scene: SceneId,
        item: ItemId,
        follow: bool,
    ) -> Result<(), String> {
        let mut core = self.lock();
        if follow {
            let source = resolve_autocrop_target(&core.collection, scene, item)?;
            core.autocrop
                .entry(item)
                .and_modify(|state| {
                    state.follow = true;
                    state.scene = scene;
                    state.source = source;
                })
                .or_insert(AutocropState {
                    scene,
                    source,
                    follow: true,
                    pending: false,
                });
        } else if let Some(state) = core.autocrop.get_mut(&item) {
            state.follow = false;
            if !state.pending {
                core.autocrop.remove(&item);
            }
        }
        Ok(())
    }

    /// Whether `item` has follow-mode auto-crop armed (the UI checkbox).
    pub fn autocrop_get(&self, item: ItemId) -> bool {
        self.lock()
            .autocrop
            .get(&item)
            .is_some_and(|state| state.follow)
    }

    /// Loop-side: a detection ran — clear `pending` (the entry survives only
    /// while follow keeps it alive). `drop_entirely` removes it regardless
    /// (the item vanished; never retry per-frame forever).
    pub fn autocrop_settle(&self, item: ItemId, drop_entirely: bool) {
        let mut core = self.lock();
        if drop_entirely {
            core.autocrop.remove(&item);
            return;
        }
        if let Some(state) = core.autocrop.get_mut(&item) {
            state.pending = false;
            if !state.follow {
                core.autocrop.remove(&item);
            }
        }
    }

    /// CAP-N69: the passthrough monitor's measured latency for `source`
    /// (capture timestamp → publish), in ms. `None` until it has run.
    pub fn passthrough_latency(&self, source: SourceId) -> Option<f32> {
        self.lock().passthrough_latency.get(&source).copied()
    }

    /// Loop-side: fold one measurement into the rolling mean.
    fn record_passthrough_latency(&self, source: SourceId, ms: f32) {
        let mut core = self.lock();
        let entry = core.passthrough_latency.entry(source).or_insert(ms);
        *entry = *entry * 0.9 + ms * 0.1; // gentle EMA — a stable readout
    }

    pub fn timer_control(&self, source: SourceId, cmd: TimerCmd) {
        let now = Instant::now();
        let mut core = self.lock();
        if core.collection.source(source).is_none() {
            return;
        }
        let run = core.timers.entry(source).or_default();
        match cmd {
            TimerCmd::Start => run.start(now),
            TimerCmd::Pause => run.pause(now),
            TimerCmd::Toggle => run.toggle(now),
            TimerCmd::Reset => run.reset(),
        }
    }

    /// Drive EVERY timer source at once — the global hotkeys' semantics
    /// (a hotkey can't name a source; the settings row says "all timers").
    pub fn timer_control_all(&self, cmd: TimerCmd) {
        let now = Instant::now();
        let mut core = self.lock();
        let ids: Vec<SourceId> = core
            .collection
            .sources
            .iter()
            .filter(|source| matches!(source.settings, SourceSettings::Timer { .. }))
            .map(|source| source.id)
            .collect();
        for id in ids {
            let run = core.timers.entry(id).or_default();
            match cmd {
                TimerCmd::Start => run.start(now),
                TimerCmd::Pause => run.pause(now),
                TimerCmd::Toggle => run.toggle(now),
                TimerCmd::Reset => run.reset(),
            }
        }
    }

    /// Toggle Studio Mode: on = the preview side starts on the program scene;
    /// off = any running blend finishes as a cut.
    pub fn set_studio_mode<R: Runtime>(&self, app: &AppHandle<R>, on: bool) {
        let dto = {
            let mut core = self.lock();
            core.preview_scene = on.then(|| core.collection.active_scene);
            if !on {
                core.transition = None;
            }
            core.revision += 1;
            dto_of(&core)
        };
        let _ = app.emit("studio", &dto);
    }

    /// Point the preview pane at a scene (Studio Mode only).
    pub fn set_preview_scene<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        scene: SceneId,
    ) -> Result<(), String> {
        let dto = {
            let mut core = self.lock();
            if core.preview_scene.is_none() {
                return Err("studio mode is off".to_string());
            }
            if core.collection.scene(scene).is_none() {
                return Err("scene not found".to_string());
            }
            core.preview_scene = Some(scene);
            core.revision += 1;
            dto_of(&core)
        };
        let _ = app.emit("studio", &dto);
        Ok(())
    }

    /// Commit Preview → Program through `kind` over `duration` (the audience
    /// sees the blend; the panes swap, OBS-style).
    pub fn begin_transition<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        settings: &crate::settings::TransitionSettings,
    ) -> Result<(), String> {
        let kind = settings.kind;
        let duration =
            Duration::from_millis(u64::from(settings.duration_ms)).max(Duration::from_millis(50));
        // The pack extras load/start OUTSIDE the core lock (file I/O).
        let stinger = if kind == fcap_scene::TransitionKind::Stinger {
            let path = settings.stinger_path.trim();
            if path.is_empty() {
                return Err(
                    "the Stinger transition needs a video file — pick one next to the transition controls"
                        .to_string(),
                );
            }
            // The stinger's audio is decoded into the media hub (CAP-N29 reads
            // its envelope to duck the program) but never mixed into the
            // program itself — video only, always forward.
            Some(
                fcap_sources::media::start_media(STINGER_HUB_ID, path, false, true, false)
                    .map_err(|err| format!("stinger: {err}"))?,
            )
        } else {
            None
        };
        let luma = if kind == fcap_scene::TransitionKind::LumaImage {
            let path = settings.luma_image.trim();
            if path.is_empty() {
                return Err(
                    "the Image Wipe transition needs a grayscale image — pick one next to the transition controls"
                        .to_string(),
                );
            }
            let frame = fcap_sources::image::load_image_rgba(std::path::Path::new(path))
                .map_err(|err| format!("luma image: {err}"))?;
            Some(Arc::new(tight_rgba(&frame)))
        } else {
            None
        };
        let cut = (settings.stinger_cut_ms.min(settings.duration_ms) as f32
            / settings.duration_ms.max(1) as f32)
            .clamp(0.0, 1.0);

        let dto = {
            let mut core = self.lock();
            let preview = core.preview_scene.ok_or("studio mode is off")?;
            let from = core.collection.active_scene;
            if preview == from {
                return Ok(()); // both panes show the same scene — nothing to do
            }
            core.collection
                .set_active_scene(preview)
                .map_err(|err| err.to_string())?;
            core.preview_scene = Some(from); // the panes swap
            if !matches!(kind, fcap_scene::TransitionKind::Cut) {
                core.transition = Some(ActiveTransition {
                    from,
                    kind,
                    duration,
                    started: Instant::now(),
                    stinger,
                    cut,
                    luma,
                    matte: settings.stinger_matte,
                });
            }
            core.revision += 1;
            core.dirty_since.get_or_insert_with(Instant::now);
            dto_of(&core)
        };
        let _ = app.emit("studio", &dto);
        // CAP-N29: a stinger with a configured duck depth ducks the program
        // under its own audio envelope. Reaching here means a real transition
        // started (the panes-equal / studio-off / empty-path cases returned
        // above). Off when the depth is 0 (the default).
        if kind == fcap_scene::TransitionKind::Stinger && settings.stinger_duck_db > 0.0 {
            app.state::<crate::audio::AudioRuntime>()
                .engine
                .set_transition_duck(Some(fcap_audio::TransitionDuckSpec {
                    hub_id: STINGER_HUB_ID.to_string(),
                    depth_db: settings.stinger_duck_db,
                    attack_ms: 40.0,
                    release_ms: 250.0,
                    threshold_db: -45.0,
                }));
        }
        Ok(())
    }

    /// A bezier mask's path/feather/invert (CAP-N28), for the wipe export.
    pub fn bezier_mask_params(
        &self,
        scene_id: SceneId,
        item_id: ItemId,
        filter_id: FilterId,
    ) -> Result<(Vec<[f32; 2]>, f32, bool), String> {
        let core = self.lock();
        let scene = core.collection.scene(scene_id).ok_or("scene not found")?;
        let item = scene
            .items
            .iter()
            .find(|item| item.id == item_id)
            .ok_or("item not found")?;
        let filter = item
            .filters
            .iter()
            .find(|filter| filter.id == filter_id)
            .ok_or("filter not found")?;
        match &filter.kind {
            fcap_scene::FilterKind::BezierMask {
                points,
                feather,
                invert,
            } => Ok((points.clone(), *feather, *invert)),
            _ => Err("not a bezier mask".into()),
        }
    }

    /// The per-scene-pair transition rule (CAP-N21) for the pending
    /// Preview→Program commit — `(kind, duration_ms)` — or `None` if studio
    /// mode is off or no rule matches the active→preview pair.
    pub fn transition_override(&self) -> Option<(fcap_scene::TransitionKind, u32)> {
        let core = self.lock();
        let to = core.preview_scene?;
        let from = core.collection.active_scene;
        core.collection
            .transition_override(from, to)
            .map(|rule| (rule.kind, rule.duration_ms))
    }

    /// The current model revision — a cheap read the audio bridge polls to
    /// decide whether it must fetch the (cloning) [`Self::audio_specs`].
    pub fn audio_revision(&self) -> u64 {
        self.lock().revision
    }

    /// What the audio engine reconciles against: the revision plus the
    /// audible sources of the program scene **and every scene nested into
    /// it** (their audio plays exactly like their video composes). Visible
    /// items only — the eye toggle (and a group's eye, Phase 6) silences
    /// audio exactly like it hides video — with the program scene's
    /// per-scene mixer overrides applied (TASK-605).
    pub fn audio_specs(&self) -> (u64, Vec<AudioSourceSpec>) {
        let core = self.lock();
        // Panic (CAP-M22): the engine runs NO sources — a hard mute that
        // stops capturing the microphone at all, and moots push-to-talk.
        if core.panic.is_some() {
            return (core.revision, Vec::new());
        }
        let collection = &core.collection;
        // The program scene + its transitively nested scenes.
        let mut scene_ids = vec![collection.active_scene];
        let mut index = 0;
        while index < scene_ids.len() {
            let current = scene_ids[index];
            index += 1;
            if let Some(scene) = collection.scene(current) {
                for item in &scene.items {
                    if let Some(source) = collection.source(item.source) {
                        if let SourceSettings::NestedScene { scene: target } = &source.settings {
                            if !scene_ids.contains(target) {
                                scene_ids.push(*target);
                            }
                        }
                    }
                }
            }
        }
        let audible = |source_id: SourceId| -> bool {
            scene_ids.iter().any(|id| {
                collection.scene(*id).is_some_and(|scene| {
                    scene.items.iter().any(|item| {
                        item.source == source_id && item.visible && !scene.group_hides(item.id)
                    })
                })
            })
        };
        let active = collection.active_scene();
        let specs = collection
            .sources
            .iter()
            .filter(|source| source.settings.has_audio())
            .filter(|source| audible(source.id))
            .map(|source| {
                let mut audio = source.audio.clone().unwrap_or_default();
                if let Some(entry) = active
                    .audio_overrides
                    .iter()
                    .find(|entry| entry.source == source.id)
                {
                    audio.volume_db = entry.volume_db;
                    audio.muted = entry.muted;
                }
                AudioSourceSpec {
                    id: source.id,
                    settings: source.settings.clone(),
                    audio,
                    nonce: core.retry_nonces.get(&source.id).copied().unwrap_or(0),
                }
            })
            .collect();
        (core.revision, specs)
    }

    /// Apply one mutation: validate, bump the revision, mark the autosave
    /// dirty, and push the fresh model to the UI. The collection is never
    /// left half-mutated — `apply` is all-or-nothing per fcap-scene.
    pub fn mutate<R: Runtime, T>(
        &self,
        app: &AppHandle<R>,
        apply: impl FnOnce(&mut Collection) -> Result<T, SceneError>,
    ) -> Result<T, String> {
        let dto = {
            let mut core = self.lock();
            let value = apply(&mut core.collection).map_err(|err| err.to_string())?;
            core.revision += 1;
            core.dirty_since.get_or_insert_with(Instant::now);
            let dto = dto_of(&core);
            (value, dto)
        };
        let _ = app.emit("studio", &dto.1);
        Ok(dto.0)
    }

    /// Like [`Self::mutate`], but records the edit on the undo stack (CAP-M01):
    /// a checkpoint is pushed only when `apply` actually changed the collection
    /// (no-op edits and rejected ones leave history untouched), and continuous
    /// gestures sharing a `Some(coalesce)` key within the coalesce window fold
    /// into a single undo step. Discrete edits pass `None`.
    ///
    /// Use this for every *document* edit. The three live-routing writes that
    /// choose what's on program right now — selecting the active scene,
    /// center-view, and focus — deliberately go through the plain
    /// [`Self::mutate`] instead, so Ctrl+Z reverses edits, not the live show.
    pub fn mutate_tracked<R: Runtime, T>(
        &self,
        app: &AppHandle<R>,
        label: &str,
        coalesce: Option<u64>,
        apply: impl FnOnce(&mut Collection) -> Result<T, SceneError>,
    ) -> Result<T, String> {
        let dto = {
            let mut core = self.lock();
            let now_ms = core.epoch.elapsed().as_millis() as u64;
            // Split the borrow so the history can edit the collection in place.
            let core = &mut *core;
            let value = core
                .undo
                .edit(&mut core.collection, label, coalesce, now_ms, apply)
                .map_err(|err| err.to_string())?;
            core.revision += 1;
            core.dirty_since.get_or_insert_with(Instant::now);
            let dto = dto_of(core);
            (value, dto)
        };
        let _ = app.emit("studio", &dto.1);
        Ok(dto.0)
    }

    /// Undo the newest recorded edit and push the restored model to the UI.
    /// Returns the reversed edit's label, or `None` when there is nothing to
    /// undo (in which case nothing is emitted). The revision still climbs so
    /// the UI's `revision >=` echo guard accepts the restore.
    pub fn undo<R: Runtime>(&self, app: &AppHandle<R>) -> Option<String> {
        let (label, dto) = {
            let mut core = self.lock();
            let core = &mut *core;
            let label = core.undo.undo(&mut core.collection)?;
            core.revision += 1;
            core.dirty_since.get_or_insert_with(Instant::now);
            (label, dto_of(core))
        };
        let _ = app.emit("studio", &dto);
        Some(label)
    }

    /// Redo the most recently undone edit. Mirror of [`Self::undo`].
    pub fn redo<R: Runtime>(&self, app: &AppHandle<R>) -> Option<String> {
        let (label, dto) = {
            let mut core = self.lock();
            let core = &mut *core;
            let label = core.undo.redo(&mut core.collection)?;
            core.revision += 1;
            core.dirty_since.get_or_insert_with(Instant::now);
            (label, dto_of(core))
        };
        let _ = app.emit("studio", &dto);
        Some(label)
    }

    /// Persist immediately if dirty (exit path — never lose the last edit).
    pub fn save_now(&self) {
        let mut core = self.lock();
        if core.dirty_since.is_some() {
            persist(&mut core);
        }
    }

    /// The active collection's on-disk path, if one is bound. Used by the backup
    /// restore to tell whether it just overwrote the file the studio is editing.
    pub fn active_path(&self) -> Option<PathBuf> {
        self.lock().path.clone()
    }

    /// Reload the active collection from its file on disk, dropping the in-memory
    /// copy, its dirty state, and its undo history. Used after a backup restore
    /// overwrote the active file: without this the autosave would clobber the
    /// restored scenes straight back to the old in-memory ones. Mirrors the load
    /// half of [`Self::switch_collection_file`] (same file, no save-first).
    pub fn reload_active<R: Runtime>(&self, app: &AppHandle<R>) {
        let dto = {
            let mut core = self.lock();
            let Some(path) = core.path.clone() else {
                return;
            };
            core.collection = read_collection(&path);
            core.dirty_since = None;
            core.preview_scene = None;
            core.transition = None;
            core.retry_nonces.clear();
            // A different document loaded — undo must not cross the boundary.
            core.undo.clear();
            core.revision += 1;
            dto_of(&core)
        };
        let _ = app.emit("studio", &dto);
    }

    /// Scene-collection switching (TASK-506): save the current scenes to
    /// `save_as` (migrating them into the collections dir on first use),
    /// then either keep them as a duplicate under `load` (create) or load
    /// `load` from disk (switch). The autosave writes to the new file from
    /// here on. Studio Mode state resets — it referenced the old scenes.
    pub fn switch_collection_file<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        save_as: PathBuf,
        load: PathBuf,
        duplicate: bool,
    ) -> Result<(), String> {
        let dto = {
            let mut core = self.lock();
            let json = serde_json::to_string_pretty(&core.collection)
                .expect("the scene collection always serializes");
            write_atomic(&save_as, &json).map_err(|err| err.to_string())?;
            if duplicate {
                write_atomic(&load, &json).map_err(|err| err.to_string())?;
            } else {
                core.collection = read_collection(&load);
            }
            core.path = Some(load);
            core.dirty_since = None;
            core.preview_scene = None;
            core.transition = None;
            core.retry_nonces.clear();
            // A different document loaded — undo must not cross the boundary.
            core.undo.clear();
            core.revision += 1;
            dto_of(&core)
        };
        let _ = app.emit("studio", &dto);
        Ok(())
    }
}

/// **Test-only** (env `FCAP_SMOKE`): seed a bright **magenta** color source into
/// the active scene and force the native preview region on, so the headless
/// `screenshot-smoke` job actually exercises the native GPU surface. The UI
/// never reports a preview region without interaction, so without this the
/// render loop builds no surface and the screenshot shows only the HTML shell
/// (the same on every OS — the native surface was only ever proven interactively
/// on real hardware). A no-op unless the env var is set; never touches normal use.
pub fn seed_smoke_scene<R: Runtime>(app: &AppHandle<R>) {
    use fcap_scene::{Rgba, Source, SourceSettings, Transform};

    let studio = app.state::<StudioState>();
    let seeded = studio.mutate(app, |collection| {
        let scene_id = collection.active_scene().id;
        let magenta = Source::new(
            "Smoke Magenta",
            SourceSettings::Color {
                color: Rgba::new(255, 0, 255, 255),
                width: 1920,
                height: 1080,
            },
        );
        let (_source_id, item_id) = collection.add_item_with_new_source(scene_id, magenta)?;
        // Resize to a centered ~55% box (mimics a manual resize). A full-canvas
        // item pushes the selection box + handles to the surface edges and the
        // rotate handle above the top edge — all clipped; a smaller centered box
        // keeps the whole overlay on-screen. This also clears `pending_fit`, so
        // the first-frame auto-fit won't overwrite it back to fill-canvas.
        let transform = Transform {
            x: 960.0,
            y: 540.0,
            scale_x: 0.55,
            scale_y: 0.55,
            ..Default::default()
        };
        collection.set_item_transform(scene_id, item_id, transform)?;
        Ok(item_id)
    });

    let native = app.state::<crate::native_preview::NativePreviewState>();
    // Force the preview region on (physical px, parent-client top-left) so the
    // studio thread builds + presents the native surface. A large central rect
    // that stays visible at 1x or 2x backing scale — exact placement doesn't
    // matter here; the "surface created" log line + a magenta frame are the proof.
    native.set_region(
        fcap_preview::Bounds {
            x: 100,
            y: 100,
            width: 1000,
            height: 600,
        },
        true,
    );
    match seeded {
        // Select the seeded item so the selection box + corner/edge handles + the
        // rotate handle draw *into* the native GPU frame (NP.3b) — proving the
        // interactive overlay on the surface, not just the surface itself.
        Ok(item_id) => native.set_selection(Some(item_id)),
        Err(err) => eprintln!("smoke: could not seed the magenta scene: {err}"),
    }
    println!("smoke: seeded + selected the magenta scene, forced the preview region on");
}

/// One audio source the engine should run (see [`StudioState::audio_specs`]).
#[derive(Debug, Clone)]
pub struct AudioSourceSpec {
    pub id: SourceId,
    pub settings: SourceSettings,
    pub audio: AudioSettings,
    /// The retry nonce — bumping it forces a device reopen.
    pub nonce: u64,
}

/// A capture frame flattened to tight RGBA rows (the luma-wipe upload shape).
fn tight_rgba(frame: &fcap_capture::Frame) -> (u32, u32, Vec<u8>) {
    let (width, height) = (frame.width, frame.height);
    let mut data = Vec::with_capacity((width * height * 4) as usize);
    for row in 0..height {
        let start = (row * frame.stride) as usize;
        let end = start + (width * 4) as usize;
        let row_bytes = &frame.data[start..end.min(frame.data.len())];
        match frame.format {
            fcap_capture::PixelFormat::Rgba8 => data.extend_from_slice(row_bytes),
            fcap_capture::PixelFormat::Bgra8 => {
                for px in row_bytes.chunks_exact(4) {
                    data.extend_from_slice(&[px[2], px[1], px[0], px[3]]);
                }
            }
        }
    }
    (width, height, data)
}

/// The on-disk file for a named scene collection: the live
/// `scene-collection.json` for `"Default"`, else `collections/<name>.json`.
/// The single source of truth for this mapping (used by load + the switcher).
pub fn collection_file(config_dir: &std::path::Path, name: &str) -> PathBuf {
    if name == crate::profiles::DEFAULT_NAME {
        config_dir.join("scene-collection.json")
    } else {
        config_dir.join("collections").join(format!("{name}.json"))
    }
}

/// The active collection name from `workspace.json` (or `"Default"`).
fn active_collection_name(config_dir: &std::path::Path) -> String {
    std::fs::read_to_string(config_dir.join("workspace.json"))
        .ok()
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .and_then(|value| {
            value
                .get("collection")
                .and_then(|name| name.as_str())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| crate::profiles::DEFAULT_NAME.to_string())
}

pub(crate) fn read_collection(path: &std::path::Path) -> Collection {
    match std::fs::read_to_string(path) {
        Ok(text) => match serde_json::from_str::<Collection>(&text) {
            Ok(mut collection) => {
                collection.sanitize();
                collection
            }
            Err(err) => {
                eprintln!(
                    "studio: {} is not a valid scene collection ({err}); starting fresh (the file is kept until the next save)",
                    path.display()
                );
                Collection::new()
            }
        },
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Collection::new(),
        Err(err) => {
            eprintln!(
                "studio: cannot read {} ({err}); starting fresh",
                path.display()
            );
            Collection::new()
        }
    }
}

fn persist(core: &mut StudioCore) {
    let Some(path) = &core.path else {
        core.dirty_since = None;
        return;
    };
    let json = serde_json::to_string_pretty(&core.collection)
        .expect("the scene collection always serializes");
    if let Err(err) = write_atomic(path, &json) {
        eprintln!("studio: could not save {} ({err})", path.display());
    }
    core.dirty_since = None;
}

// ---------------------------------------------------------------------------
// The `program` event payload
// ---------------------------------------------------------------------------

/// Live status of one source, keyed by its id in [`ProgramStatus::sources`].
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceRuntime {
    /// "waiting" | "live" | "error"
    pub state: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<u32>,
    /// An HDR display feeds this source (CAP-N74) — the UI offers the
    /// tone-map controls when set.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub hdr: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Ms since this source last delivered a frame (capture-backed sources
    /// only) — the health dashboard's staleness signal (CAP-M13).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_frame_ms: Option<u64>,
    /// Capture frames overwritten before the compositor took them (CAP-M13).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropped: Option<u64>,
    /// Pipeline restarts so far (manual retry + auto-recover), absent until
    /// the first one — the auto-recover history (CAP-M13).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<u64>,
}

impl SourceRuntime {
    fn waiting() -> Self {
        SourceRuntime {
            state: "waiting",
            hdr: false,
            width: None,
            height: None,
            fps: None,
            error_code: None,
            error_message: None,
            last_frame_ms: None,
            dropped: None,
            retries: None,
        }
    }

    fn live(width: u32, height: u32) -> Self {
        SourceRuntime {
            state: "live",
            width: Some(width),
            height: Some(height),
            ..SourceRuntime::waiting()
        }
    }

    fn error(code: &'static str, message: String) -> Self {
        SourceRuntime {
            state: "error",
            error_code: Some(code),
            error_message: Some(message),
            ..SourceRuntime::waiting()
        }
    }
}

/// The `program` event: compose-loop health + per-source states.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramStatus {
    /// "starting" | "running" | "noGpu"
    pub state: &'static str,
    pub width: u32,
    pub height: u32,
    /// Composed frames in the last second.
    pub fps: u32,
    /// CPU cost of the last compose (encode + submit), microseconds.
    pub render_micros: u64,
    pub adapter: String,
    /// Capture frames overwritten before the compositor took them.
    pub dropped: u64,
    /// Keyed by source id (UUID string).
    pub sources: HashMap<String, SourceRuntime>,
}

fn error_code(err: &CaptureError) -> &'static str {
    match err {
        CaptureError::PermissionDenied => "permission",
        CaptureError::Cancelled => "cancelled",
        CaptureError::NotFound(_) => "notFound",
        CaptureError::Unsupported(_) => "unsupported",
        CaptureError::Stopped => "stopped",
        CaptureError::Backend(_) => "backend",
    }
}

// ---------------------------------------------------------------------------
// The render thread
// ---------------------------------------------------------------------------

/// Spawn the studio render thread. It outlives everything until the app
/// exits (emit failures are the shutdown signal, matching the stats emitter).
pub fn spawn_studio_thread<R: Runtime>(app: AppHandle<R>, state: &StudioState) {
    let core = Arc::clone(&state.core);
    std::thread::Builder::new()
        .name("fcap-studio".into())
        .spawn(move || run_studio(app, core))
        .expect("studio thread spawns");
}

/// One bound Text source's poll state (CAP-M16).
#[derive(Default)]
struct BoundTextState {
    /// (modified, len) of the last successfully read file.
    fingerprint: Option<(Option<std::time::SystemTime>, u64)>,
    /// The last successfully painted content — atomic-write tolerance: a
    /// failed stat/read (mid temp+rename) keeps this on screen.
    last_good: Option<String>,
    /// The next poll is due at this instant.
    next_poll: Option<Instant>,
}

/// Everything the loop tracks per capture-backed source.
struct SessionSlot {
    session: CaptureSession,
    frames_this_second: u32,
    live_size: Option<(u32, u32)>,
    /// When the last frame was uploaded — the dashboard's last-frame age
    /// (CAP-M13).
    last_frame: Option<Instant>,
    /// Render Delay (TASK-608): frames held back by the filter, bounded —
    /// raw frames are memory, so both the delay (500 ms) and the queue
    /// length are capped.
    delayed: std::collections::VecDeque<(Instant, fcap_capture::Frame)>,
}

fn lock_core(core: &Arc<Mutex<StudioCore>>) -> std::sync::MutexGuard<'_, StudioCore> {
    core.lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn run_studio<R: Runtime>(app: AppHandle<R>, core: Arc<Mutex<StudioCore>>) {
    // Bring up the GPU; without any adapter the studio reports that honestly
    // (and keeps reporting it, in case the UI mounts later) instead of lying
    // with a frozen canvas.
    let (canvas_w, canvas_h) = {
        let guard = lock_core(&core);
        (
            guard.collection.canvas_width,
            guard.collection.canvas_height,
        )
    };
    println!("studio: creating the compositor ({canvas_w}x{canvas_h})...");
    let mut compositor = match Compositor::new(canvas_w, canvas_h) {
        Ok(compositor) => compositor,
        Err(err) => {
            eprintln!("studio: no GPU — the compositor cannot run ({err})");
            loop {
                let status = ProgramStatus {
                    state: "noGpu",
                    width: canvas_w,
                    height: canvas_h,
                    fps: 0,
                    render_micros: 0,
                    adapter: err.to_string(),
                    dropped: 0,
                    sources: HashMap::new(),
                };
                if app.emit("program", &status).is_err() {
                    return; // app shut down
                }
                // Scene edits still work without a GPU — honor the debounced
                // autosave here too, or a GPU-less session persists nothing
                // until a clean exit.
                {
                    let mut guard = lock_core(&core);
                    if guard
                        .dirty_since
                        .is_some_and(|since| since.elapsed() >= AUTOSAVE_DEBOUNCE)
                    {
                        persist(&mut guard);
                    }
                }
                std::thread::sleep(Duration::from_secs(5));
            }
        }
    };
    println!("studio: compositor up on {}", compositor.adapter_summary());

    let preview = app.state::<PreviewState>();

    let started_at = Instant::now();
    let mut seen_revision = 0u64;
    // CAP-N02: the automation variables revision the source reconcile last ran
    // against — a variable change repaints Text sources that interpolate it.
    let mut seen_var_revision = 0u64;
    // The nested-scene pool the compositor resolves against, deep-cloned only
    // when the model revision changes (not per frame) — see the snapshot.
    let mut cached_scene_pool: Vec<fcap_scene::Scene> = Vec::new();
    let mut pool_revision = u64::MAX;
    let mut sessions: HashMap<SourceId, SessionSlot> = HashMap::new();
    let mut starting: HashMap<SourceId, mpsc::Receiver<Result<CaptureSession, CaptureError>>> =
        HashMap::new();
    let mut capture_specs: HashMap<SourceId, String> = HashMap::new();
    let mut static_specs: HashMap<SourceId, String> = HashMap::new();
    let mut filter_files: HashMap<FilterId, String> = HashMap::new();
    // Bezier masks (CAP-N28): the signature of each mask filter last rasterized
    // — re-raster only when the path/feather/invert actually change.
    let mut bezier_sigs: HashMap<FilterId, u64> = HashMap::new();
    let mut statuses: HashMap<SourceId, SourceRuntime> = HashMap::new();

    let mut composed_this_second = 0u32;
    let mut composed_vertical_this_second = 0u32;
    let mut vertical_preview_live = false;
    // Rising-edge detector for transitions (uploads the luma image / resets
    // the stinger exactly once per commit).
    let mut transition_was_active = false;
    // Floating reactions (TASK-614): the live particle pool + its rng, and
    // the shared queue chat ingests push into.
    let mut reaction_particles: Vec<crate::reactions::Particle> = Vec::new();
    let mut reaction_rng = crate::reactions::Lcg(0x9E37_79B9_7F4A_7C15);
    // V1-E: the featured chat banner — re-rendered + re-uploaded only when
    // the pin (or the canvas) changes; the cached draw then rides the normal
    // reactions pass of every bake. `None` draw = nothing pinned.
    let mut featured_seen: Option<(u64, (u32, u32))> = None;
    let mut featured_draw_cache: Option<fcap_compositor::ReactionDraw> = None;
    let reactions_queue = app
        .state::<crate::reactions::ReactionState>()
        .queue_handle();
    let mut last_readback = Instant::now() - READBACK_INTERVAL;
    let mut last_program_event = Instant::now();
    // Whether the Studio-Mode preview pane currently has a published frame
    // (so turning the mode off clears the slot exactly once).
    let mut studio_preview_live = false;
    // Same, for the keying-workbench slot (CAP-M26).
    let mut workbench_live = false;
    // Multiview (CAP-M06): its own slow cadence, a round-robin cursor so no tick
    // renders every scene at once, and a was-open flag to clear the slots on close.
    // Automation (CAP-N01): evaluated at 1 Hz — triggers are human-scale and
    // a cheap tick keeps the 60 fps budget untouched.
    let mut last_automation = Instant::now();
    // CAP-N08: the program scene the PTZ layer last acted on (so a preset is
    // recalled once per switch, not once per tick).
    let mut ptz_scene: Option<String> = None;
    let mut last_multiview = Instant::now();
    let mut multiview_cursor = 0usize;
    let mut multiview_was_on = false;
    // Scene/source projectors (CAP-M07 extension): rendered at the readback
    // cadence; closed targets' slots are retired by `retain_projectors`.
    let mut last_projector = Instant::now() - READBACK_INTERVAL;
    // Per-source auto-recover backoff: source id → (next attempt time, last wait).
    let mut retry_schedule: HashMap<SourceId, (Instant, Duration)> = HashMap::new();
    let mut statuses_changed = true;
    // Black/frozen program watch (CAP-M10), fed by the readback below.
    let mut video_watch = crate::alarms::VideoWatch::default();
    // Timer & clock faces (CAP-M15): the live timer set (rebuilt at reconcile),
    // the last face painted per source (repaint only on change), the fire-once
    // guard for end-of-countdown actions, and the flash window per fired timer.
    let mut timer_sources: HashMap<SourceId, SourceSettings> = HashMap::new();
    let mut timer_specs: HashMap<SourceId, String> = HashMap::new();
    let mut timer_faces: HashMap<SourceId, (String, bool)> = HashMap::new();
    let mut timer_done: HashSet<SourceId> = HashSet::new();
    let mut timer_flash: HashMap<SourceId, Instant> = HashMap::new();
    // V1-C: the decoded still image behind an Image-slate countdown, cached by
    // path so it is read from disk once, not on every ~1 Hz face repaint.
    let mut timer_slate_images: HashMap<SourceId, (String, fcap_capture::Frame)> = HashMap::new();
    // Reset generations seen per timer — a Reset re-arms a latched wall
    // countdown (and clears its flash) without a model revision.
    let mut timer_resets: HashMap<SourceId, u32> = HashMap::new();
    // Bound Text sources (CAP-M16): the live set + per-source poll state.
    let mut bound_texts: HashMap<SourceId, SourceSettings> = HashMap::new();
    let mut bound_specs: HashMap<SourceId, String> = HashMap::new();
    let mut bound_states: HashMap<SourceId, BoundTextState> = HashMap::new();
    // System-stats HUD faces (CAP-N14): the live set (rebuilt at reconcile)
    // + the last face painted per source (repaint only on change), refreshed
    // at the samplers' cadence, not per frame.
    let mut stats_sources: HashMap<SourceId, SourceSettings> = HashMap::new();
    let mut stats_specs: HashMap<SourceId, String> = HashMap::new();
    let mut stats_faces: HashMap<SourceId, String> = HashMap::new();
    let mut last_stats_face = Instant::now() - STATS_FACE_INTERVAL;
    // Playlist "now playing" variables (CAP-N17): last value written per
    // source, polled at 1 Hz — a variable write only happens on change.
    let mut last_now_playing = Instant::now();
    let mut now_playing_cache: HashMap<SourceId, String> = HashMap::new();
    // Title variables (CAP-N16): the variables revision last handed to the
    // title registry — checked at 1 Hz, fed only when it moved.
    let mut last_title_vars = Instant::now();
    let mut fed_title_vars: Option<u64> = None;
    // Recording-synced media (the backdrop's "start with recording" hold):
    // the last recording state the loop saw, and sources whose next uploaded
    // frame becomes their poster (upload once, then pause until recording).
    let mut was_recording = false;
    let mut poster_pause: HashSet<SourceId> = HashSet::new();
    // ISO lanes (CAP-N40): once-per-session compose-failure log, and the
    // loop's view of the panic hold it keeps in sync on the lanes.
    let mut iso_errors_logged: HashSet<SourceId> = HashSet::new();
    let mut iso_panic_held = false;
    // Alpha recording (CAP-N42): once-per-session compose-failure log.
    let mut alpha_error_logged = false;
    // Split-on-scene-change (CAP-N43): the scene the recording last saw.
    // `None` while idle, so a session never splits on its very first tick.
    let mut split_last_scene: Option<fcap_scene::SceneId> = None;
    // Auto-markers (CAP-N44): edge trackers for the 1 Hz reconnect and
    // dropped-frame-burst watches.
    let mut was_reconnecting = false;
    let mut was_dropping = false;
    let mut last_dropped: Option<u64> = None;
    // Punch-in zoom lenses (CAP-N71): the loop-side animated state, keyed by
    // item. Entries whose target vanished spring back to flat, then drop.
    let mut lens_anim: HashMap<ItemId, LensAnim> = HashMap::new();
    // Show/hide fade-in (CAP-N21): the active scene last seen (a scene switch
    // resets tracking so a transition never double-animates), each item's last
    // visibility (to catch the make-visible edge), and when a reveal began.
    let mut reveal_scene: Option<SceneId> = None;
    let mut reveal_prev_visible: HashMap<ItemId, bool> = HashMap::new();
    let mut reveal_started: HashMap<ItemId, Instant> = HashMap::new();
    // Auto black-bar crop (CAP-N72): the source resolution each armed item
    // last saw — follow-mode re-detects only when it changes.
    let mut autocrop_dims: HashMap<ItemId, (u32, u32)> = HashMap::new();
    // HDR displays (CAP-N74): sources whose output reports an HDR color
    // space at reconcile — their live status carries the tone-map hint.
    let mut hdr_sources: HashSet<SourceId> = HashSet::new();
    // LAN listeners (CAP-N11): their runtime state follows the session's
    // "sender connected" flag, not bare frame arrival — the waiting face
    // they draw is a frame too, and calling it "live" would lie.
    // Keyed with the id's STRING form too — the drain consults the ingest
    // registry per drained frame, and re-stringing the uuid each time is a
    // per-frame allocation.
    let mut lan_sources: HashMap<SourceId, String> = HashMap::new();

    // The native preview surface (the "OBS feel" path): created lazily once
    // the UI reports a non-zero preview region, then presented every frame
    // with no readback. `None` while unsupported/uncreated; `disabled` stops
    // retrying after a creation failure so it can't spin.
    let mut native_surface: Option<(fcap_compositor::NativePreview, u64)> = None;
    // The native GPU preview needs the DX12 backend (DirectComposition only
    // accepts DirectX swapchains); on any other adapter it stays disabled and
    // the JPEG path renders.
    // The native preview needs a backend whose swapchain the OS compositor
    // accepts: DX12 for the Windows DirectComposition visual, Metal for the
    // macOS CAMetalLayer, Vulkan/GL for the Linux X11 child window. Anything
    // else stays on the JPEG path.
    let mut native_disabled =
        !(compositor.is_dx12() || compositor.is_metal() || compositor.is_vulkan_or_gl());
    {
        // Tell the UI up front whether the native path is viable, so a non-DX12
        // machine (or a missing overlay) never hides its JPEG canvas over a
        // surface that can never present.
        let native = app.state::<crate::native_preview::NativePreviewState>();
        native.set_viable(!native_disabled && native.composition_handle().is_some());
    }

    loop {
        let tick_started = Instant::now();

        // -- 0. Recording-synced media (the backdrop's "start playback with
        // recording" option): on EITHER recording edge, every flagged Media
        // source restarts from the top — playing when recording just began,
        // re-armed to hold on its first frame (the poster, registered at
        // session start below) when it just ended — so each take starts the
        // wallpaper video at 0:00 no matter what a preview did to it.
        let recording_now = app
            .state::<crate::recording::RecordingState>()
            .recording_since()
            .is_some();
        if recording_now != was_recording {
            was_recording = recording_now;
            let mut guard = lock_core(&core);
            let flagged: Vec<SourceId> = guard
                .collection
                .sources
                .iter()
                .filter(|source| {
                    matches!(
                        source.settings,
                        SourceSettings::Media {
                            start_with_recording: true,
                            ..
                        }
                    )
                })
                .map(|source| source.id)
                .collect();
            if !flagged.is_empty() {
                for id in &flagged {
                    *guard.retry_nonces.entry(*id).or_insert(0) += 1;
                    fcap_sources::media::set_media_paused(&id.0.to_string(), false);
                }
                guard.revision += 1;
            }
        }

        // -- 1. Snapshot the model (brief lock) --------------------------------
        let multiview_due = last_multiview.elapsed() >= MULTIVIEW_INTERVAL;
        let projector_due = last_projector.elapsed() >= READBACK_INTERVAL;
        let mut transition_ended: Option<StudioDto> = None;
        let (
            revision,
            scene,
            scene_sources,
            canvas,
            nonces,
            preview_scene,
            transition_pack,
            vertical_pack,
            scene_pool_rebuild,
            scene_refs,
            workbench_pack,
            multiview_on,
            multiview_scenes,
            still_job,
            projector_pack,
            panic_active,
            timers_snapshot,
            zoom_lenses,
            autocrop_snapshot,
            passthrough_targets,
            downstream_draws,
        ) = {
            let mut guard = lock_core(&core);
            // A finished blend clears here, under the command lock.
            let transition_pack = match &guard.transition {
                Some(tr) => {
                    let progress =
                        tr.started.elapsed().as_secs_f32() / tr.duration.as_secs_f32().max(1e-3);
                    if progress >= 1.0 {
                        guard.transition = None;
                        guard.revision += 1;
                        // Emit so the UI's `transitioning` flag flips off (the
                        // loop's own studio emit is gated on first-frame fits).
                        transition_ended = Some(dto_of(&guard));
                        None
                    } else {
                        // The newest stinger video frame, when one plays.
                        let stinger_frame = tr.stinger.as_ref().and_then(|session| {
                            session.frames().recv_timeout(Duration::ZERO).ok().flatten()
                        });
                        guard.collection.scene(tr.from).cloned().map(|from_scene| {
                            TransitionFramePack {
                                from_scene,
                                kind: tr.kind,
                                progress,
                                cut: tr.cut,
                                stinger_frame,
                                luma: tr.luma.clone(),
                                matte: tr.matte,
                            }
                        })
                    }
                }
                None => None,
            };
            let preview_scene = guard
                .preview_scene
                .filter(|id| *id != guard.collection.active_scene)
                .and_then(|id| guard.collection.scene(id).cloned());
            // The sources that must run: the program scene's, plus (Studio
            // Mode) the preview pane's, plus the outgoing scene's mid-blend.
            let mut live_sources: Vec<SourceId> = guard
                .collection
                .active_scene()
                .items
                .iter()
                .map(|item| item.source)
                .collect();
            // Downstream keyers (CAP-N24): keep EVERY keyer's source live — even
            // a disabled one — so toggling a keyer never tears down and re-opens
            // its capture (no blank/blink on re-enable); render only the enabled
            // ones. A persistent overlay renders even when its source is in no
            // scene.
            live_sources.extend(guard.collection.downstream.iter().map(|dsk| dsk.source));
            let downstream_draws: Vec<fcap_compositor::DownstreamDraw> = guard
                .collection
                .downstream
                .iter()
                .filter(|dsk| dsk.enabled)
                .map(|dsk| fcap_compositor::DownstreamDraw {
                    source: dsk.source,
                    transform: dsk.transform,
                    opacity: dsk.opacity,
                })
                .collect();
            if let Some(preview) = &preview_scene {
                live_sources.extend(preview.items.iter().map(|item| item.source));
            }
            if let Some(pack) = &transition_pack {
                live_sources.extend(pack.from_scene.items.iter().map(|item| item.source));
            }
            // The second (vertical) canvas keeps ITS scene's sources live too.
            let vertical_pack = guard.collection.vertical.and_then(|config| {
                guard
                    .collection
                    .scene(config.scene)
                    .cloned()
                    .map(|scene| (scene, config.width, config.height))
            });
            if let Some((vertical_scene, _, _)) = &vertical_pack {
                live_sources.extend(vertical_scene.items.iter().map(|item| item.source));
            }
            // The keying workbench (CAP-M26) renders one item's source — keep it
            // live even when its scene is off program.
            let workbench_pack: Option<(SourceId, Vec<Filter>, WorkbenchMode, f32)> =
                guard.workbench.and_then(|wb| {
                    guard.collection.scenes.iter().find_map(|scene| {
                        scene
                            .item(wb.item)
                            .map(|item| (item.source, item.filters.clone(), wb.mode, wb.split))
                    })
                });
            if let Some((source, _, _, _)) = &workbench_pack {
                live_sources.push(*source);
            }
            // Multiview (CAP-M06): while a monitor is open, keep every scene's
            // sources live so off-program thumbnails render; clone the scenes for
            // the thumbnail render only on a due tick.
            let multiview_on = guard.multiview;
            if multiview_on {
                for scene in &guard.collection.scenes {
                    live_sources.extend(scene.items.iter().map(|item| item.source));
                }
            }
            let multiview_scenes: Option<Vec<Scene>> =
                (multiview_on && multiview_due).then(|| guard.collection.scenes.clone());
            // Scene/source projectors (CAP-M07 extension): keep their sources
            // live every tick; clone the render jobs only on a due tick. The
            // `live` key set lets the render loop retire closed targets' slots.
            let projector_live: HashSet<String> =
                guard.projectors.iter().map(ProjectorTarget::key).collect();
            for target in guard.projectors.iter() {
                match target {
                    ProjectorTarget::Scene(id) => {
                        if let Some(scene) = guard.collection.scene(*id) {
                            live_sources.extend(scene.items.iter().map(|item| item.source));
                        }
                    }
                    ProjectorTarget::Source(id) | ProjectorTarget::Passthrough(id) => {
                        // A passthrough monitor keeps its device alive even
                        // when no scene shows it — playing off the monitor is
                        // the whole point (CAP-N69).
                        if guard.collection.source(*id).is_some() {
                            live_sources.push(*id);
                        }
                    }
                }
            }
            // CAP-N69: which sources have a passthrough monitor open — the
            // drain publishes their RAW frames with no composition at all.
            let passthrough_targets: HashSet<SourceId> = guard
                .projectors
                .iter()
                .filter_map(|target| match target {
                    ProjectorTarget::Passthrough(id) => Some(*id),
                    _ => None,
                })
                .collect();
            let projector_jobs: Option<Vec<ProjectorJob>> =
                (!guard.projectors.is_empty() && projector_due).then(|| {
                    guard
                        .projectors
                        .iter()
                        .filter_map(|target| match target {
                            ProjectorTarget::Scene(id) => guard
                                .collection
                                .scene(*id)
                                .map(|scene| ProjectorJob::Scene(target.key(), scene.clone())),
                            ProjectorTarget::Source(id) => guard
                                .collection
                                .source(*id)
                                .map(|_| ProjectorJob::Source(target.key(), *id)),
                            // CAP-N69: the passthrough monitor publishes raw
                            // frames straight from the capture drain — it must
                            // NOT ride the composited readback path.
                            ProjectorTarget::Passthrough(_) => None,
                        })
                        .collect()
                });
            let projector_pack = (projector_live, projector_jobs);
            // A pending still-frame grab (CAP-M08), resolved against the model and
            // taken once so it fires exactly one frame.
            let still_job: Option<(StillJob, PathBuf)> =
                guard.still_request.take().and_then(|(target, path)| {
                    let job = match target {
                        StillTarget::Program => Some(StillJob::Program),
                        StillTarget::Source { item, pre_filter } => {
                            guard.collection.scenes.iter().find_map(|scene| {
                                scene.item(item).map(|it| {
                                    let filters = if pre_filter {
                                        Vec::new()
                                    } else {
                                        it.filters.clone()
                                    };
                                    StillJob::Source(it.source, filters)
                                })
                            })
                        }
                    };
                    job.map(|job| (job, path))
                });
            // Nested scenes (Phase 6): the source→scene map, the pool the
            // compositor resolves against, and every transitively nested
            // scene's sources joining the live set.
            let scene_refs: HashMap<SourceId, SceneId> = guard
                .collection
                .sources
                .iter()
                .filter_map(|source| match &source.settings {
                    SourceSettings::NestedScene { scene } => Some((source.id, *scene)),
                    _ => None,
                })
                .collect();
            // The reachability walk runs every tick (cheap — it reads items,
            // no deep copy) so nested scenes' sources always join the live
            // set; the expensive full-scene deep clone into the pool happens
            // only when the model actually changed (revision), reused from
            // the cache otherwise.
            if !scene_refs.is_empty() {
                let mut reachable: Vec<SceneId> = live_sources
                    .iter()
                    .filter_map(|source| scene_refs.get(source).copied())
                    .collect();
                let mut index = 0;
                while index < reachable.len() {
                    let current = reachable[index];
                    index += 1;
                    if let Some(nested) = guard.collection.scene(current) {
                        for item in &nested.items {
                            live_sources.push(item.source);
                            if let Some(target) = scene_refs.get(&item.source) {
                                if !reachable.contains(target) {
                                    reachable.push(*target);
                                }
                            }
                        }
                    }
                }
            }
            let scene_pool_rebuild: Option<Vec<fcap_scene::Scene>> =
                (guard.revision != pool_revision).then(|| {
                    if scene_refs.is_empty() {
                        Vec::new()
                    } else {
                        guard.collection.scenes.clone()
                    }
                });
            // Panic (CAP-M22): the slate replaces the composed world.
            // Program + vertical show it, the live set collapses to the
            // slate's static sources (every capture stops), monitoring
            // renders stop, and projectors — possibly on a PUBLIC display —
            // show the slate too. Multiview thumbnails simply freeze
            // (operator-side monitoring only).
            let panic_pack = guard.panic.clone();
            let scene = match &panic_pack {
                Some(pack) => pack.scene.clone(),
                None => guard.collection.active_scene().clone(),
            };
            let scene_sources = match &panic_pack {
                Some(pack) => pack.sources.clone(),
                None => guard
                    .collection
                    .sources
                    .iter()
                    .filter(|source| live_sources.contains(&source.id))
                    .cloned()
                    .collect::<Vec<_>>(),
            };
            let (preview_scene, transition_pack, workbench_pack, multiview_scenes) =
                if panic_pack.is_some() {
                    (None, None, None, None)
                } else {
                    (
                        preview_scene,
                        transition_pack,
                        workbench_pack,
                        multiview_scenes,
                    )
                };
            let vertical_pack = match (&panic_pack, vertical_pack) {
                (Some(pack), Some((_, w, h))) => {
                    pack.vertical_scene.clone().map(|slate| (slate, w, h))
                }
                (_, vertical) => vertical,
            };
            let projector_pack = match &panic_pack {
                Some(pack) => (
                    projector_pack.0,
                    projector_pack.1.map(|jobs| {
                        jobs.into_iter()
                            .map(|job| match job {
                                ProjectorJob::Scene(key, _) => {
                                    ProjectorJob::Scene(key, pack.scene.clone())
                                }
                                ProjectorJob::Source(key, _) => {
                                    ProjectorJob::Scene(key, pack.scene.clone())
                                }
                            })
                            .collect()
                    }),
                ),
                None => projector_pack,
            };
            // Timer runs (CAP-M15) follow their sources out of the collection.
            if guard.revision != seen_revision && !guard.timers.is_empty() {
                let live: HashSet<SourceId> =
                    guard.collection.sources.iter().map(|s| s.id).collect();
                guard.timers.retain(|id, _| live.contains(id));
            }
            (
                guard.revision,
                scene,
                scene_sources,
                (
                    guard.collection.canvas_width,
                    guard.collection.canvas_height,
                ),
                guard.retry_nonces.clone(),
                preview_scene,
                transition_pack,
                vertical_pack,
                scene_pool_rebuild,
                scene_refs,
                workbench_pack,
                multiview_on,
                multiview_scenes,
                still_job,
                projector_pack,
                panic_pack.is_some(),
                guard.timers.clone(),
                guard.zoom_lenses.clone(),
                guard.autocrop.clone(),
                passthrough_targets,
                downstream_draws,
            )
        };
        if let Some(dto) = transition_ended {
            let _ = app.emit("studio", &dto);
        }
        compositor.set_canvas_size(canvas.0, canvas.1);

        // -- 1b. Punch-in zoom lenses (CAP-N71): spring each drawn zoom/anchor
        // toward its target — a fixed-step, critically-damped spring, so the
        // motion is deterministic. Follow lenses re-anchor onto the OS cursor
        // (display/window captures; off-Windows the mapping returns None and
        // the anchor just stays where it was).
        {
            let dt = TICK.as_secs_f32();
            for (item_id, target) in &zoom_lenses {
                let anim = lens_anim.entry(*item_id).or_insert(LensAnim {
                    zoom: 1.0,
                    zoom_vel: 0.0,
                    anchor: target.anchor,
                    anchor_vel: (0.0, 0.0),
                });
                let desired_anchor = if target.follow {
                    follow_anchor(&scene, &scene_sources, *item_id).unwrap_or(anim.anchor)
                } else {
                    target.anchor
                };
                spring_step(&mut anim.zoom, &mut anim.zoom_vel, target.zoom, dt);
                spring_step(
                    &mut anim.anchor.0,
                    &mut anim.anchor_vel.0,
                    desired_anchor.0,
                    dt,
                );
                spring_step(
                    &mut anim.anchor.1,
                    &mut anim.anchor_vel.1,
                    desired_anchor.1,
                    dt,
                );
            }
            lens_anim.retain(|id, anim| {
                if zoom_lenses.contains_key(id) {
                    return true;
                }
                spring_step(&mut anim.zoom, &mut anim.zoom_vel, 1.0, dt);
                anim.zoom > 1.0 + 1e-3
            });
            compositor.set_lenses(
                lens_anim
                    .iter()
                    .filter(|(_, anim)| anim.zoom > 1.0 + 1e-3)
                    .map(|(id, anim)| (*id, (anim.zoom, anim.anchor)))
                    .collect(),
            );
        }

        // -- 1c. Show/hide fade-in (CAP-N21): when an item with reveal_ms > 0
        // is made visible WITHIN the live scene, ramp its opacity 0→1. A scene
        // switch resets tracking (no edge), so the transition owns that motion
        // and items never double-animate. Fully-revealed items leave the map
        // (absent = opaque), so a scene with no reveal is pixel-identical.
        {
            let scene_changed = reveal_scene != Some(scene.id);
            if scene_changed {
                reveal_scene = Some(scene.id);
                reveal_prev_visible.clear();
                reveal_started.clear();
                for item in &scene.items {
                    reveal_prev_visible.insert(item.id, item.visible);
                }
            } else {
                for item in &scene.items {
                    let was = reveal_prev_visible.get(&item.id).copied().unwrap_or(true);
                    if item.reveal_ms > 0 && item.visible && !was {
                        reveal_started.insert(item.id, Instant::now());
                    }
                    reveal_prev_visible.insert(item.id, item.visible);
                }
            }
            let mut opacities: HashMap<ItemId, f32> = HashMap::new();
            reveal_started.retain(|id, started| {
                let Some(item) = scene.items.iter().find(|it| it.id == *id) else {
                    return false; // the item left the scene — drop it
                };
                let progress =
                    started.elapsed().as_secs_f32() / (item.reveal_ms as f32 / 1000.0).max(1e-3);
                if progress >= 1.0 {
                    return false; // fully revealed — opaque, no override needed
                }
                opacities.insert(*id, progress.clamp(0.0, 1.0));
                true
            });
            compositor.set_item_opacity(opacities);
        }

        // Hand the compositor a fresh nested-scene pool only when the model
        // changed — never a per-frame deep clone.
        if let Some(pool) = scene_pool_rebuild {
            cached_scene_pool = pool;
            pool_revision = revision;
            compositor.set_scene_pool(cached_scene_pool.clone(), scene_refs);
        }
        let scene_pool = &cached_scene_pool;

        // -- 2. Reconcile sources against the active scene ---------------------
        // Also re-run when an automation variable changed (CAP-N02): a Text
        // source's interpolated face rides its spec, so a variable write must
        // repaint it even though it bumps no collection revision. The reconcile
        // is idempotent for unchanged sources (matching specs → no restart).
        let var_revision = app
            .state::<crate::automation::AutomationState>()
            .variables_revision();
        if revision != seen_revision || var_revision != seen_var_revision {
            seen_revision = revision;
            seen_var_revision = var_revision;

            // Stop sessions whose source left the scene or changed settings.
            let mut keep_ids: Vec<SourceId> = Vec::new();
            // Which sources took the Timer / bound-text branch THIS pass. A
            // kind flip (Timer → Color, or a cleared file binding) keeps the
            // id in `keep_ids` via its new branch, so retaining on keep_ids
            // alone would strand the old face and keep painting a ghost.
            let mut timer_seen: HashSet<SourceId> = HashSet::new();
            let mut bound_seen: HashSet<SourceId> = HashSet::new();
            let mut stats_seen: HashSet<SourceId> = HashSet::new();
            for source in &scene_sources {
                // Audio-only sources belong to the audio engine (their state
                // rides the `audio` event) — shed any stale video pipeline
                // from a kind flip and skip them here. (Media has audio too
                // but keeps its video pipeline — it composes AND mixes.)
                if source.settings.is_audio_only() {
                    if let Some(slot) = sessions.remove(&source.id) {
                        slot.session.stop();
                    }
                    starting.remove(&source.id);
                    capture_specs.remove(&source.id);
                    static_specs.remove(&source.id);
                    if statuses.remove(&source.id).is_some() {
                        statuses_changed = true;
                    }
                    compositor.remove_source(source.id);
                    continue;
                }
                // Timer & clock faces (CAP-M15) paint from the per-tick
                // refresh step (§5b) — no OS pipeline. The spec fingerprint
                // gates the cache drop: an UNRELATED model edit (a fader, a
                // rename elsewhere) bumps the revision and lands here too, and
                // must NOT force a repaint of every face.
                if matches!(source.settings, SourceSettings::Timer { .. }) {
                    if let Some(slot) = sessions.remove(&source.id) {
                        slot.session.stop();
                    }
                    starting.remove(&source.id);
                    capture_specs.remove(&source.id);
                    static_specs.remove(&source.id);
                    let face_spec = source_spec(&source.settings);
                    if timer_specs.get(&source.id) != Some(&face_spec) {
                        timer_specs.insert(source.id, face_spec);
                        timer_sources.insert(source.id, source.settings.clone());
                        timer_faces.remove(&source.id);
                    }
                    timer_seen.insert(source.id);
                    if let std::collections::hash_map::Entry::Vacant(slot) =
                        statuses.entry(source.id)
                    {
                        slot.insert(SourceRuntime::waiting());
                        statuses_changed = true;
                    }
                    keep_ids.push(source.id);
                    continue;
                }
                // The system-stats HUD (CAP-N14) paints from its own refresh
                // step (§5b2) — no OS pipeline. Same fingerprint gate as the
                // timer faces above.
                if matches!(source.settings, SourceSettings::SystemStats { .. }) {
                    if let Some(slot) = sessions.remove(&source.id) {
                        slot.session.stop();
                    }
                    starting.remove(&source.id);
                    capture_specs.remove(&source.id);
                    static_specs.remove(&source.id);
                    let face_spec = source_spec(&source.settings);
                    if stats_specs.get(&source.id) != Some(&face_spec) {
                        stats_specs.insert(source.id, face_spec);
                        stats_sources.insert(source.id, source.settings.clone());
                        stats_faces.remove(&source.id);
                    }
                    stats_seen.insert(source.id);
                    if let std::collections::hash_map::Entry::Vacant(slot) =
                        statuses.entry(source.id)
                    {
                        slot.insert(SourceRuntime::waiting());
                        statuses_changed = true;
                    }
                    keep_ids.push(source.id);
                    continue;
                }
                // Text bound to a watched file (CAP-M16) paints from the
                // poll step (§5c). Same fingerprint gate: dropping the poll
                // state on an unrelated edit would re-stat every bound file
                // and, worse, discard `last_good` — so a file caught mid
                // atomic rename would flash "not found" on air.
                if is_bound_text(&source.settings) {
                    if let Some(slot) = sessions.remove(&source.id) {
                        slot.session.stop();
                    }
                    starting.remove(&source.id);
                    capture_specs.remove(&source.id);
                    static_specs.remove(&source.id);
                    let bind_spec = source_spec(&source.settings);
                    if bound_specs.get(&source.id) != Some(&bind_spec) {
                        bound_specs.insert(source.id, bind_spec);
                        bound_texts.insert(source.id, source.settings.clone());
                        bound_states.remove(&source.id);
                    }
                    bound_seen.insert(source.id);
                    if let std::collections::hash_map::Entry::Vacant(slot) =
                        statuses.entry(source.id)
                    {
                        slot.insert(SourceRuntime::waiting());
                        statuses_changed = true;
                    }
                    keep_ids.push(source.id);
                    continue;
                }
                // Nested scenes need no OS pipeline: the compositor composes
                // them straight into their slot every frame.
                if matches!(source.settings, SourceSettings::NestedScene { .. }) {
                    if let Some(slot) = sessions.remove(&source.id) {
                        slot.session.stop();
                    }
                    starting.remove(&source.id);
                    capture_specs.remove(&source.id);
                    static_specs.remove(&source.id);
                    if let std::collections::hash_map::Entry::Vacant(slot) =
                        statuses.entry(source.id)
                    {
                        slot.insert(SourceRuntime::live(canvas.0, canvas.1));
                        statuses_changed = true;
                    }
                    keep_ids.push(source.id);
                    continue;
                }
                // The retry nonce is part of the fingerprint: bumping it is
                // how an errored source gets restarted with equal settings.
                let nonce = nonces.get(&source.id).copied().unwrap_or(0);
                // CAP-N02: a Text source interpolates `{{variable}}` tokens.
                // The interpolated text rides the SPEC, so setting a variable
                // repaints the face on the next tick (and an unchanged
                // variable costs nothing — the fingerprint is identical).
                let effective = interpolate_settings(&app, &source.settings);
                let spec = format!("{}#{nonce}", source_spec(&effective));
                let is_capture = is_capture_backed(&effective);
                if is_capture {
                    let changed = capture_specs.get(&source.id) != Some(&spec);
                    if changed {
                        if let Some(slot) = sessions.remove(&source.id) {
                            slot.session.stop();
                        }
                        starting.remove(&source.id);
                        // A kind flip (static → capture) sheds the old family.
                        static_specs.remove(&source.id);
                        compositor.remove_source(source.id);
                        capture_specs.insert(source.id, spec);
                        // CAP-N74: seed the display's tone-map registry from
                        // settings and remember whether its output is HDR
                        // (the badge that suggests enabling an operator).
                        if let SourceSettings::Display { capture_id, .. } = &source.settings {
                            let config = app
                                .state::<crate::settings::SettingsStore>()
                                .hdr_tone_map(capture_id)
                                .and_then(|tone| {
                                    Some(fcap_capture::tonemap::ToneMapConfig {
                                        operator:
                                            fcap_capture::tonemap::ToneMapOperator::from_name(
                                                &tone.operator,
                                            )?,
                                        paper_white_nits: tone.paper_white_nits as f32,
                                    })
                                })
                                .unwrap_or_default();
                            fcap_capture::tonemap::set_tone_map(capture_id, config);
                            if fcap_capture::display_is_hdr(capture_id) == Some(true) {
                                hdr_sources.insert(source.id);
                            } else {
                                hdr_sources.remove(&source.id);
                            }
                        }
                        // CAP-N11: remember which sources are LAN listeners —
                        // the frame drain below mirrors their session's
                        // connected flag into the runtime state.
                        if matches!(&source.settings, SourceSettings::LanIngest { .. }) {
                            lan_sources
                                .entry(source.id)
                                .or_insert_with(|| source.id.0.to_string());
                        } else {
                            lan_sources.remove(&source.id);
                        }
                        // CAP-N19: seed the capture's cursor-effects registry
                        // from settings — both owned-cursor paths (display +
                        // window). Live retunes go through `cursor_fx_set`;
                        // this covers session (re)starts and app relaunches.
                        if let SourceSettings::Display { capture_id, .. }
                        | SourceSettings::Window { capture_id, .. } = &source.settings
                        {
                            let fx = app
                                .state::<crate::settings::SettingsStore>()
                                .cursor_fx(capture_id)
                                .and_then(|fx| fx.to_config());
                            fcap_capture::cursorfx::set_cursor_fx(capture_id, fx);
                        }
                        // CAP-M18: the saved control profile rides into every
                        // device (re)open — hotplug/auto-recover included.
                        let camera_profile = match &source.settings {
                            SourceSettings::VideoDevice { device_id, .. } => app
                                .state::<crate::settings::SettingsStore>()
                                .camera_profile(device_id),
                            _ => Vec::new(),
                        };
                        start_session(
                            source.id,
                            &source.settings,
                            &mut starting,
                            &reactions_queue,
                            camera_profile,
                        );
                        // Recording-synced media: a fresh session must not
                        // inherit a stale pause flag (it would freeze before
                        // its first frame); outside a recording it holds on
                        // its first frame (the poster) until recording starts.
                        if matches!(
                            source.settings,
                            SourceSettings::Media {
                                start_with_recording: true,
                                ..
                            }
                        ) {
                            fcap_sources::media::set_media_paused(&source.id.0.to_string(), false);
                            if recording_now {
                                poster_pause.remove(&source.id);
                            } else {
                                poster_pause.insert(source.id);
                            }
                        } else {
                            poster_pause.remove(&source.id);
                        }
                        statuses.insert(source.id, SourceRuntime::waiting());
                        statuses_changed = true;
                    }
                } else {
                    let changed = static_specs.get(&source.id) != Some(&spec);
                    if changed {
                        // A kind flip (capture → static) must stop the OS
                        // pipeline — otherwise the camera stays open and its
                        // frames keep overwriting the static texture.
                        if let Some(slot) = sessions.remove(&source.id) {
                            slot.session.stop();
                        }
                        starting.remove(&source.id);
                        capture_specs.remove(&source.id);
                        static_specs.insert(source.id, spec);
                        let status = match render_static(&effective) {
                            Ok(frame) => {
                                let (w, h) = (frame.width, frame.height);
                                match compositor.upload_frame(source.id, &frame) {
                                    Ok(()) => SourceRuntime::live(w, h),
                                    Err(err) => SourceRuntime::error("backend", err.to_string()),
                                }
                            }
                            Err(message) => SourceRuntime::error("backend", message),
                        };
                        statuses.insert(source.id, status);
                        statuses_changed = true;
                    }
                }
                keep_ids.push(source.id);
            }
            // Sources that left the scene entirely. capture_specs is part of
            // the union: an errored capture lives ONLY there (its session is
            // already gone), and a stale fingerprint would block its restart
            // when the source re-enters the scene.
            let gone: Vec<SourceId> = sessions
                .keys()
                .chain(starting.keys())
                .chain(static_specs.keys())
                .chain(capture_specs.keys())
                .filter(|id| !keep_ids.contains(id))
                .copied()
                .collect();
            for id in gone {
                if let Some(slot) = sessions.remove(&id) {
                    slot.session.stop();
                }
                starting.remove(&id);
                capture_specs.remove(&id);
                static_specs.remove(&id);
                statuses.remove(&id);
                poster_pause.remove(&id);
                statuses_changed = true;
            }
            // CAP-N73: linked window↔app-audio pairs re-resolve the audio's
            // pid from the window's LIVE process (a window rebind after an
            // app restart bumps the revision, which lands here). Windows
            // resolves; elsewhere `window_process` is None and nothing moves.
            let mut relinks: Vec<(SourceId, u32, String)> = Vec::new();
            for audio_source in &scene_sources {
                let SourceSettings::AppAudio {
                    pid: old_pid,
                    linked_window: Some(window_source),
                    ..
                } = &audio_source.settings
                else {
                    continue;
                };
                let Some(capture_id) = scene_sources.iter().find_map(|entry| {
                    (entry.id == *window_source)
                        .then(|| match &entry.settings {
                            SourceSettings::Window { capture_id, .. } => Some(capture_id.clone()),
                            _ => None,
                        })
                        .flatten()
                }) else {
                    continue;
                };
                if let Some((pid, exe)) = fcap_capture::window_process(&capture_id) {
                    if pid != *old_pid {
                        relinks.push((audio_source.id, pid, exe));
                    }
                }
            }
            if !relinks.is_empty() {
                let studio = app.state::<StudioState>();
                for (source_id, pid, exe) in relinks {
                    let _ = studio.mutate(&app, |collection| {
                        let Some(entry) = collection
                            .sources
                            .iter_mut()
                            .find(|entry| entry.id == source_id)
                        else {
                            return Ok(());
                        };
                        if let SourceSettings::AppAudio {
                            pid: stored_pid,
                            exe: stored_exe,
                            ..
                        } = &mut entry.settings
                        {
                            *stored_pid = pid;
                            *stored_exe = exe;
                        }
                        Ok(())
                    });
                }
            }
            // Timers: keep only the ones that took the Timer branch this pass.
            // A kind flip (Timer → Color) hands the face to the static branch,
            // which owns its health row from now on; one that left the scene
            // entirely drops the row too (timers live in no session/spec map,
            // so the `gone` union above can't see them).
            timer_sources.retain(|id, _| {
                if timer_seen.contains(id) {
                    return true;
                }
                if !keep_ids.contains(id) && statuses.remove(id).is_some() {
                    statuses_changed = true;
                }
                false
            });
            timer_specs.retain(|id, _| timer_sources.contains_key(id));
            timer_faces.retain(|id, _| timer_sources.contains_key(id));
            timer_done.retain(|id| timer_sources.contains_key(id));
            timer_flash.retain(|id, _| timer_sources.contains_key(id));
            timer_resets.retain(|id, _| timer_sources.contains_key(id));
            timer_slate_images.retain(|id, _| timer_sources.contains_key(id));
            // Bound texts: keep only the ones still bound this pass. One that
            // merely lost its binding stays in the scene (the static branch
            // now owns its face and health row); one that left the scene
            // drops its row like the timers above.
            bound_texts.retain(|id, _| {
                if bound_seen.contains(id) {
                    return true;
                }
                if !keep_ids.contains(id) && statuses.remove(id).is_some() {
                    statuses_changed = true;
                }
                false
            });
            bound_specs.retain(|id, _| bound_texts.contains_key(id));
            bound_states.retain(|id, _| bound_texts.contains_key(id));
            // Stats HUDs: same rule as the timers — they live in no
            // session/spec map, so the `gone` union above can't see them.
            stats_sources.retain(|id, _| {
                if stats_seen.contains(id) {
                    return true;
                }
                if !keep_ids.contains(id) && statuses.remove(id).is_some() {
                    statuses_changed = true;
                }
                false
            });
            stats_specs.retain(|id, _| stats_sources.contains_key(id));
            stats_faces.retain(|id, _| stats_sources.contains_key(id));
            compositor.retain_sources(&keep_ids);

            // Filter files (LUT lattices, mask images) — the preview pane's,
            // the outgoing scene's, and the vertical canvas's filters render too.
            let extra_items: Vec<fcap_scene::SceneItem> = preview_scene
                .iter()
                .flat_map(|preview| preview.items.iter().cloned())
                .chain(
                    transition_pack
                        .iter()
                        .flat_map(|pack| pack.from_scene.items.iter().cloned()),
                )
                .chain(
                    vertical_pack
                        .iter()
                        .flat_map(|(vertical_scene, _, _)| vertical_scene.items.iter().cloned()),
                )
                .chain(
                    // Nested scenes' filters render too (pool is empty
                    // unless nesting is in use).
                    scene_pool
                        .iter()
                        .flat_map(|nested| nested.items.iter().cloned()),
                )
                .collect();
            let mut live_filters: Vec<FilterId> = Vec::new();
            for item in scene.items.iter().chain(extra_items.iter()) {
                for filter in &item.filters {
                    // Bezier masks (CAP-N28): rasterize the path app-side into
                    // the same alpha-mask resource an image mask uses, only when
                    // its signature changes.
                    if let fcap_scene::FilterKind::BezierMask {
                        points,
                        feather,
                        invert,
                    } = &filter.kind
                    {
                        live_filters.push(filter.id);
                        let sig = bezier_signature(points, *feather, *invert);
                        if bezier_sigs.get(&filter.id) != Some(&sig) {
                            bezier_sigs.insert(filter.id, sig);
                            match crate::bezier_mask::mask_rgba(points, *feather, *invert) {
                                Some(rgba) => {
                                    let data = FilterResourceData::Image {
                                        width: crate::bezier_mask::MASK_SIZE,
                                        height: crate::bezier_mask::MASK_SIZE,
                                        rgba,
                                    };
                                    if let Err(err) =
                                        compositor.set_filter_resource(filter.id, &data)
                                    {
                                        eprintln!("studio: bezier mask: {err}");
                                    }
                                }
                                // <3 points: no shape — drop the resource so the
                                // item renders unmasked.
                                None => compositor.remove_filter_resource(filter.id),
                            }
                        }
                        continue;
                    }
                    if let Some(path) = filter_file_path(&filter.kind) {
                        live_filters.push(filter.id);
                        if filter_files.get(&filter.id) != Some(&path) {
                            filter_files.insert(filter.id, path.clone());
                            match load_filter_resource(&filter.kind, &path) {
                                Ok(data) => {
                                    if let Err(err) =
                                        compositor.set_filter_resource(filter.id, &data)
                                    {
                                        eprintln!("studio: filter file {path}: {err}");
                                    }
                                }
                                Err(message) => {
                                    compositor.remove_filter_resource(filter.id);
                                    eprintln!("studio: filter file {path}: {message}");
                                }
                            }
                        }
                    }
                }
            }
            filter_files.retain(|id, _| live_filters.contains(id));
            bezier_sigs.retain(|id, _| live_filters.contains(id));
            compositor.retain_filter_resources(&live_filters);
        }

        // -- 3. Sessions that finished starting ---------------------------------
        let mut finished: Vec<(SourceId, Result<CaptureSession, CaptureError>)> = Vec::new();
        starting.retain(|id, rx| match rx.try_recv() {
            Ok(result) => {
                finished.push((*id, result));
                false
            }
            Err(mpsc::TryRecvError::Empty) => true,
            Err(mpsc::TryRecvError::Disconnected) => {
                finished.push((
                    *id,
                    Err(CaptureError::Backend("the starter thread died".into())),
                ));
                false
            }
        });
        for (id, result) in finished {
            // Still wanted? (The scene may have moved on mid-start.)
            let wanted = capture_specs.contains_key(&id);
            match result {
                Ok(session) if wanted => {
                    sessions.insert(
                        id,
                        SessionSlot {
                            session,
                            frames_this_second: 0,
                            live_size: None,
                            last_frame: None,
                            delayed: std::collections::VecDeque::new(),
                        },
                    );
                }
                Ok(session) => session.stop(),
                Err(err) if wanted => {
                    statuses.insert(id, SourceRuntime::error(error_code(&err), err.to_string()));
                    statuses_changed = true;
                }
                Err(_) => {}
            }
        }

        // -- 4. Drain the newest frame from every live session ------------------
        // Render Delay (TASK-608): a source whose visible items carry the
        // filter shows frames from `delay_ms` ago — held in a bounded queue
        // (the 500 ms cap is honest: raw frames are memory).
        let mut delay_by_source: HashMap<SourceId, u32> = HashMap::new();
        // CAP-N25: (item, source) pairs with an enabled Freeze filter. The
        // compositor snapshots each frozen ITEM and samples that held texture,
        // so a clone of the same source (or another placement) stays live.
        let mut frozen_items: Vec<(ItemId, SourceId)> = Vec::new();
        {
            let mut scan = |items: &[fcap_scene::SceneItem]| {
                for item in items {
                    for filter in item.filters.iter().filter(|filter| filter.enabled) {
                        match &filter.kind {
                            fcap_scene::FilterKind::RenderDelay { delay_ms } => {
                                let entry = delay_by_source.entry(item.source).or_insert(0);
                                *entry = (*entry).max((*delay_ms).min(500));
                            }
                            fcap_scene::FilterKind::Freeze => {
                                frozen_items.push((item.id, item.source));
                            }
                            _ => {}
                        }
                    }
                }
            };
            scan(&scene.items);
            if let Some(preview) = &preview_scene {
                scan(&preview.items);
            }
            if let Some(pack) = &transition_pack {
                scan(&pack.from_scene.items);
            }
            if let Some((vertical_scene, _, _)) = &vertical_pack {
                scan(&vertical_scene.items);
            }
            for nested in scene_pool {
                scan(&nested.items);
            }
        }
        // CAP-M20: the workbench's armed video probe, checked once per tick —
        // the drain below records a mean-luma sample per received frame on
        // the armed source (CPU-side, before upload; ~2 k sampled pixels).
        let calibration_state = app.state::<crate::calibration::CalibrationState>();
        let calibration_armed = calibration_state.armed_source();
        let mut ended: Vec<(SourceId, CaptureError)> = Vec::new();
        for (id, slot) in sessions.iter_mut() {
            let delay_ms = delay_by_source.get(id).copied().unwrap_or(0);
            match slot.session.frames().recv_timeout(Duration::ZERO) {
                Ok(Some(frame)) => {
                    if calibration_armed == Some(*id) {
                        calibration_state.push_frame(
                            *id,
                            frame.captured_at,
                            crate::calibration::mean_luma(&frame),
                        );
                    }
                    // CAP-N69: the passthrough monitor gets this frame FIRST
                    // — raw, unfiltered, uncomposited, and never delayed —
                    // and its capture→publish latency is measured here.
                    if passthrough_targets.contains(id) {
                        let latency_ms = frame.captured_at.elapsed().as_secs_f32() * 1000.0;
                        if let Some(jpeg) = encode_capture_jpeg(&frame) {
                            app.state::<crate::preview::PreviewState>()
                                .publish_projector(&format!("passthrough:{}", id.0), Some(jpeg));
                            app.state::<StudioState>()
                                .record_passthrough_latency(*id, latency_ms);
                        }
                    }
                    // CAP-N25 freeze is now per-item in the compositor (it
                    // snapshots the frozen item's texture), so every source keeps
                    // uploading normally here — a live clone of a frozen source is
                    // never held back, and Source Health / Render-Delay behave as
                    // usual.
                    if delay_ms == 0 && slot.delayed.is_empty() {
                        let size = (frame.width, frame.height);
                        run_autocrop(&app, &frame, *id, &autocrop_snapshot, &mut autocrop_dims);
                        match compositor.upload_frame(*id, &frame) {
                            Ok(()) => {
                                slot.frames_this_second += 1;
                                slot.last_frame = Some(Instant::now());
                                if slot.live_size != Some(size) {
                                    slot.live_size = Some(size);
                                    statuses.insert(
                                        *id,
                                        SourceRuntime {
                                            hdr: hdr_sources.contains(id),
                                            ..SourceRuntime::live(size.0, size.1)
                                        },
                                    );
                                    statuses_changed = true;
                                }
                                // CAP-N11: a LAN listener's waiting face rides
                                // this same pipe at the same fixed geometry, so
                                // frame arrival can't mean "live" — mirror the
                                // session's own connected flag instead. Every
                                // transition is accompanied by a frame (face or
                                // feed), so checking here catches them all.
                                if let Some(key) = lan_sources.get(id) {
                                    let connected = fcap_sources::laningest::receiving(key);
                                    let desired = if connected { "live" } else { "waiting" };
                                    if statuses.get(id).map(|runtime| runtime.state)
                                        != Some(desired)
                                    {
                                        statuses.insert(
                                            *id,
                                            if connected {
                                                SourceRuntime::live(size.0, size.1)
                                            } else {
                                                SourceRuntime::waiting()
                                            },
                                        );
                                        statuses_changed = true;
                                    }
                                }
                            }
                            Err(err) => {
                                eprintln!("studio: dropped a broken frame: {err}");
                            }
                        }
                    } else {
                        slot.delayed.push_back((Instant::now(), frame));
                        if slot.delayed.len() > 64 {
                            slot.delayed.pop_front(); // bounded, oldest goes
                        }
                    }
                }
                Ok(None) => {}
                Err(err) => ended.push((*id, err)),
            }
            // A recording-synced source pauses on its poster: the first
            // uploaded frame stays on the canvas, playback holds until the
            // recording edge unpauses it (a preview toggle may too).
            if slot.last_frame.is_some() && poster_pause.remove(id) {
                fcap_sources::media::set_media_paused(&id.0.to_string(), true);
            }
            // Release the newest delayed frame that has come due (all of
            // them at once when the filter was just removed).
            if !slot.delayed.is_empty() {
                let due = Duration::from_millis(u64::from(delay_ms));
                let mut release: Option<fcap_capture::Frame> = None;
                while slot
                    .delayed
                    .front()
                    .is_some_and(|(arrived, _)| arrived.elapsed() >= due)
                {
                    release = slot.delayed.pop_front().map(|(_, frame)| frame);
                }
                if let Some(frame) = release {
                    let size = (frame.width, frame.height);
                    run_autocrop(&app, &frame, *id, &autocrop_snapshot, &mut autocrop_dims);
                    match compositor.upload_frame(*id, &frame) {
                        Ok(()) => {
                            slot.frames_this_second += 1;
                            slot.last_frame = Some(Instant::now());
                            if slot.live_size != Some(size) {
                                slot.live_size = Some(size);
                                statuses.insert(
                                    *id,
                                    SourceRuntime {
                                        hdr: hdr_sources.contains(id),
                                        ..SourceRuntime::live(size.0, size.1)
                                    },
                                );
                                statuses_changed = true;
                            }
                        }
                        Err(err) => {
                            eprintln!("studio: dropped a broken frame: {err}");
                        }
                    }
                }
            }
        }
        for (id, err) in ended {
            sessions.remove(&id);
            let message = match &err {
                CaptureError::Stopped => "The source ended.".to_string(),
                other => other.to_string(),
            };
            statuses.insert(id, SourceRuntime::error(error_code(&err), message));
            statuses_changed = true;
        }

        // -- 5. First-frame fit for newly added items ----------------------------
        let fits: Vec<(SceneId, fcap_scene::ItemId, Transform)> = scene
            .items
            .iter()
            .filter(|item| item.pending_fit)
            .filter_map(|item| {
                compositor.source_size(item.source).map(|(w, h)| {
                    // A layout corner fits the source into its normalized slot;
                    // otherwise the ordinary whole-canvas fit-and-center.
                    let transform = match item.pending_slot {
                        Some(slot) => {
                            let cw = canvas.0 as f32;
                            let ch = canvas.1 as f32;
                            fcap_compositor::transform::fit_into_slot(
                                w,
                                h,
                                slot.x * cw,
                                slot.y * ch,
                                slot.w * cw,
                                slot.h * ch,
                            )
                        }
                        None => fcap_compositor::transform::fit_to_canvas(w, h, canvas.0, canvas.1),
                    };
                    (scene.id, item.id, transform)
                })
            })
            .collect();
        if !fits.is_empty() {
            let dto = {
                let mut guard = lock_core(&core);
                for (scene_id, item_id, transform) in fits {
                    // resolve_pending (not set_item_transform): the item's
                    // seat survives placement, so seat-swap can read it.
                    let _ = guard
                        .collection
                        .resolve_pending(scene_id, item_id, transform);
                }
                guard.revision += 1;
                guard.dirty_since.get_or_insert_with(Instant::now);
                dto_of(&guard)
            };
            if app.emit("studio", &dto).is_err() {
                break;
            }
        }

        // -- 5b. Timer & clock faces (CAP-M15) ----------------------------------
        // Every tick, cheap: compute each live timer's display text and repaint
        // (render_text + upload) only when the text or flash phase changed —
        // ~1 Hz per timer in practice. End-of-countdown actions fire exactly
        // once per zero crossing (the `timer_done` guard re-arms on reset).
        if !timer_sources.is_empty() {
            let now = Instant::now();
            let mut switch_to: Option<SceneId> = None;
            for (id, settings) in &timer_sources {
                let SourceSettings::Timer {
                    mode,
                    format,
                    utc_offset_min,
                    countdown_ms,
                    target,
                    end_action,
                    end_scene,
                    font_family,
                    font_file,
                    size_px,
                    color,
                    message,
                    slate,
                } = settings
                else {
                    continue;
                };
                let run = timers_snapshot.get(id).copied().unwrap_or_default();
                // A Reset re-arms a latched wall countdown (and clears the
                // flash) — it bumps no revision, so the run's reset counter
                // is the only signal the loop gets.
                let resets = run.resets();
                if timer_resets.get(id).copied().unwrap_or(0) != resets {
                    timer_resets.insert(*id, resets);
                    timer_done.remove(id);
                    timer_flash.remove(id);
                    timer_faces.remove(id);
                }
                let (text, at_zero) = match mode {
                    fcap_scene::TimerMode::WallClock => {
                        let zone = crate::timers::clock_zone(*utc_offset_min);
                        let at = chrono::Local::now().with_timezone(&zone);
                        (crate::timers::render_time(&at, format), false)
                    }
                    fcap_scene::TimerMode::Countdown => {
                        if let Some((hours, minutes)) = crate::timers::parse_wall_target(target) {
                            // Once the target passes, HOLD at zero until the
                            // user resets. Recomputing the remainder would roll
                            // to tomorrow's occurrence and put "23:59:59" on
                            // air one second after the countdown ended.
                            if timer_done.contains(id) {
                                ("0:00".to_string(), true)
                            } else {
                                let local = chrono::Local::now();
                                let hit = crate::timers::wall_target_hit(&local, hours, minutes);
                                let text = if hit {
                                    "0:00".to_string()
                                } else {
                                    crate::timers::format_hms(crate::timers::remaining_to_wall(
                                        &local, hours, minutes,
                                    ))
                                };
                                (text, hit)
                            }
                        } else {
                            let total = Duration::from_millis(*countdown_ms);
                            let elapsed = run.elapsed(now);
                            let remaining = total.saturating_sub(elapsed);
                            let at_zero = remaining.is_zero()
                                && (run.is_running() || elapsed > Duration::ZERO);
                            (crate::timers::format_hms(remaining), at_zero)
                        }
                    }
                    fcap_scene::TimerMode::Stopwatch => {
                        (crate::timers::format_hms(run.elapsed(now)), false)
                    }
                    fcap_scene::TimerMode::SinceLive => {
                        let since = app.state::<crate::stream::StreamBridgeState>().live_since();
                        let elapsed = since
                            .map(|at| now.saturating_duration_since(at))
                            .unwrap_or_default();
                        (crate::timers::format_hms(elapsed), false)
                    }
                    fcap_scene::TimerMode::SinceRecording => {
                        let since = app
                            .state::<crate::recording::RecordingState>()
                            .recording_since();
                        let elapsed = since
                            .map(|at| now.saturating_duration_since(at))
                            .unwrap_or_default();
                        (crate::timers::format_hms(elapsed), false)
                    }
                };
                if at_zero {
                    if timer_done.insert(*id) {
                        match end_action {
                            fcap_scene::CountdownEnd::None => {}
                            fcap_scene::CountdownEnd::Flash => {
                                timer_flash.insert(*id, now);
                            }
                            fcap_scene::CountdownEnd::SwitchScene => {
                                if let Some(scene_id) = end_scene {
                                    switch_to = Some(*scene_id);
                                }
                            }
                        }
                    }
                } else {
                    timer_done.remove(id);
                }
                let highlight = match timer_flash.get(id) {
                    Some(fired) if now.duration_since(*fired) < TIMER_FLASH_WINDOW => {
                        (now.duration_since(*fired).as_millis() / 250) % 2 == 0
                    }
                    Some(_) => {
                        timer_flash.remove(id);
                        false
                    }
                    None => false,
                };
                let face = (text, highlight);
                if timer_faces.get(id) == Some(&face) {
                    continue;
                }
                // At zero a flashing countdown paints its number red; otherwise
                // its configured colour.
                let number_color = if highlight {
                    TIMER_FLASH_COLOR
                } else {
                    [color.r, color.g, color.b, color.a]
                };
                let base_style =
                    |text: String, size_px: f32, color: [u8; 4], align| text::TextStyle {
                        text,
                        font_family: font_family.clone(),
                        font_file: font_file.as_ref().map(PathBuf::from),
                        size_px,
                        color,
                        align,
                        line_spacing: 1.0,
                        force_rtl: false,
                        wrap_width: None,
                        ..text::TextStyle::default()
                    };
                // V1-C: slate mode paints a full-canvas "Starting Soon" card
                // (message above a big centred number) over a colour / gradient
                // / transparent background; otherwise the classic inline face.
                let frame_result = match slate {
                    Some(slate) => {
                        // Slate text always gets a contrasting outline + soft
                        // shadow so the number and label stay legible over ANY
                        // background — a busy photo, a video, any colour.
                        let legible = |mut s: text::TextStyle| {
                            s.outline_color = contrasting_outline(s.color);
                            s.outline_px = (s.size_px * 0.06).clamp(1.5, 12.0);
                            s.shadow_px = (s.size_px * 0.05).clamp(1.0, 14.0);
                            s.shadow_color = [0, 0, 0, 150];
                            s
                        };
                        let number = legible(base_style(
                            face.0.clone(),
                            *size_px,
                            number_color,
                            text::TextAlign::Center,
                        ));
                        let message_style = legible(base_style(
                            message.clone(),
                            (*size_px * SLATE_MESSAGE_SCALE).max(8.0),
                            [color.r, color.g, color.b, color.a],
                            text::TextAlign::Center,
                        ));
                        // One match: an image slate decodes once and caches by
                        // path (a failed decode falls back to a transparent
                        // slate — text still shows — rather than blanking the
                        // countdown); every other slate drops any cached image.
                        let bg = match slate.as_ref() {
                            fcap_scene::CountdownSlate::Transparent => {
                                timer_slate_images.remove(id);
                                countdown::SlateBg::Transparent
                            }
                            fcap_scene::CountdownSlate::Solid { color } => {
                                timer_slate_images.remove(id);
                                countdown::SlateBg::Solid([color.r, color.g, color.b, color.a])
                            }
                            fcap_scene::CountdownSlate::Gradient { from, to } => {
                                timer_slate_images.remove(id);
                                countdown::SlateBg::Gradient(
                                    [from.r, from.g, from.b, from.a],
                                    [to.r, to.g, to.b, to.a],
                                )
                            }
                            fcap_scene::CountdownSlate::Image { path } => {
                                // The CAP-M16 rule for EVERY user path: refuse
                                // network paths before any stat/probe (a UNC
                                // read leaks the NTLM hash) — a hostile scene
                                // collection controls this string. Refused =
                                // the transparent fallback, text still shows.
                                if crate::commands::studio::is_remote(path.trim()) {
                                    timer_slate_images.remove(id);
                                    countdown::SlateBg::Transparent
                                } else {
                                    let stale = timer_slate_images
                                        .get(id)
                                        .map_or(true, |(cached, _)| cached != path);
                                    if stale {
                                        match fcap_sources::image::load_image_rgba(
                                            std::path::Path::new(path),
                                        ) {
                                            Ok(frame) => {
                                                timer_slate_images
                                                    .insert(*id, (path.clone(), frame));
                                            }
                                            Err(err) => {
                                                // Negative-cache the failure (a
                                                // 1×1 transparent frame keyed by
                                                // the same path) so a bad file
                                                // fails ONCE — not a disk read +
                                                // decode + log line every 1 Hz
                                                // repaint for the whole show.
                                                timer_slate_images.insert(
                                                    *id,
                                                    (
                                                        path.clone(),
                                                        fcap_capture::Frame {
                                                            width: 1,
                                                            height: 1,
                                                            stride: 4,
                                                            format:
                                                                fcap_capture::PixelFormat::Rgba8,
                                                            data: vec![0; 4],
                                                            captured_at: std::time::Instant::now(),
                                                        },
                                                    ),
                                                );
                                                eprintln!("studio: countdown slate image: {err}");
                                            }
                                        }
                                    }
                                    match timer_slate_images.get(id) {
                                        Some((_, frame)) => countdown::SlateBg::Image(frame),
                                        None => countdown::SlateBg::Transparent,
                                    }
                                }
                            }
                        };
                        countdown::render_countdown_slate(
                            canvas.0,
                            canvas.1,
                            bg,
                            &message_style,
                            &number,
                        )
                    }
                    None => text::render_text(&base_style(
                        face.0.clone(),
                        *size_px,
                        number_color,
                        text::TextAlign::Left,
                    )),
                };
                match frame_result {
                    Ok(frame) => {
                        let (w, h) = (frame.width, frame.height);
                        match compositor.upload_frame(*id, &frame) {
                            Ok(()) => {
                                timer_faces.insert(*id, face);
                                let stale = statuses.get(id).map_or(true, |prev| {
                                    prev.state != "live"
                                        || prev.width != Some(w)
                                        || prev.height != Some(h)
                                });
                                if stale {
                                    statuses.insert(*id, SourceRuntime::live(w, h));
                                    statuses_changed = true;
                                }
                            }
                            Err(err) => eprintln!("studio: timer face dropped: {err}"),
                        }
                    }
                    Err(err) => {
                        // Cache the failed face too — no 60 Hz retry storm; a
                        // settings edit clears the cache and tries again.
                        timer_faces.insert(*id, face);
                        statuses.insert(*id, SourceRuntime::error("backend", err.to_string()));
                        statuses_changed = true;
                    }
                }
            }
            if let Some(scene_id) = switch_to {
                // A countdown's scene jump is live routing (no undo), done
                // loop-side — the transition-ended emit pattern.
                let dto = {
                    let mut guard = lock_core(&core);
                    match guard.collection.set_active_scene(scene_id) {
                        Ok(()) => {
                            guard.revision += 1;
                            Some(dto_of(&guard))
                        }
                        Err(err) => {
                            eprintln!("studio: countdown scene switch failed: {err}");
                            None
                        }
                    }
                };
                if let Some(dto) = dto {
                    let _ = app.emit("studio", &dto);
                }
            }
        }

        // -- 5b2. System-stats HUD faces (CAP-N14) -------------------------------
        // Re-read the numbers at the samplers' cadence (not per frame) and
        // repaint a face only when its formatted text changed — the rounding
        // in `format_hud` pins that to visible movement, so painting stays
        // ~2 Hz worst-case per HUD.
        if !stats_sources.is_empty() && last_stats_face.elapsed() >= STATS_FACE_INTERVAL {
            last_stats_face = Instant::now();
            let (_, fps, _, dropped, render_micros) =
                app.state::<crate::events::RuntimeStats>().latest();
            let (cpu, memory_mb) = crate::events::latest_system();
            let stream = app.state::<crate::stream::StreamBridgeState>().status();
            let bitrate_kbps = matches!(stream.state.as_str(), "live" | "reconnecting")
                .then(|| stream.targets.iter().map(|target| target.kbps).sum());
            // CAP-N47: the burn-in reads the LTC decode only when some HUD
            // actually shows it — the light `ltc()` accessor clones one small
            // string, and the common (no-timecode) HUD pays nothing.
            let wants_ltc = stats_sources.values().any(|settings| {
                matches!(
                    settings,
                    SourceSettings::SystemStats {
                        show_timecode: true,
                        ..
                    }
                )
            });
            let numbers = crate::statshud::StatsNumbers {
                fps,
                cpu_percent: cpu,
                memory_mb,
                render_ms: render_micros as f32 / 1000.0,
                dropped,
                bitrate_kbps,
                ltc: wants_ltc
                    .then(|| app.state::<crate::audio::AudioRuntime>().engine.ltc())
                    .flatten(),
            };
            for (id, settings) in &stats_sources {
                let SourceSettings::SystemStats {
                    show_fps,
                    show_cpu,
                    show_memory,
                    show_render_ms,
                    show_dropped,
                    show_bitrate,
                    show_timecode,
                    font_family,
                    font_file,
                    size_px,
                    color,
                } = settings
                else {
                    continue;
                };
                let face = crate::statshud::format_hud(
                    &numbers,
                    &crate::statshud::HudToggles {
                        fps: *show_fps,
                        cpu: *show_cpu,
                        memory: *show_memory,
                        render_ms: *show_render_ms,
                        dropped: *show_dropped,
                        bitrate: *show_bitrate,
                        timecode: *show_timecode,
                    },
                );
                if stats_faces.get(id) == Some(&face) {
                    continue;
                }
                let style = text::TextStyle {
                    text: face.clone(),
                    font_family: font_family.clone(),
                    // A remote font path is refused inside render_text's
                    // font funnel (text::resolve_font) — no guard needed here.
                    font_file: font_file.as_ref().map(PathBuf::from),
                    size_px: *size_px,
                    color: [color.r, color.g, color.b, color.a],
                    align: text::TextAlign::Left,
                    line_spacing: 1.0,
                    force_rtl: false,
                    wrap_width: None,
                    ..text::TextStyle::default()
                };
                match text::render_text(&style) {
                    Ok(frame) => {
                        let (w, h) = (frame.width, frame.height);
                        match compositor.upload_frame(*id, &frame) {
                            Ok(()) => {
                                stats_faces.insert(*id, face);
                                let stale = statuses.get(id).map_or(true, |prev| {
                                    prev.state != "live"
                                        || prev.width != Some(w)
                                        || prev.height != Some(h)
                                });
                                if stale {
                                    statuses.insert(*id, SourceRuntime::live(w, h));
                                    statuses_changed = true;
                                }
                            }
                            Err(err) => eprintln!("studio: stats face dropped: {err}"),
                        }
                    }
                    Err(err) => {
                        // Cache the failed face too — no retry storm; a
                        // settings edit clears the cache and tries again.
                        stats_faces.insert(*id, face);
                        statuses.insert(*id, SourceRuntime::error("backend", err.to_string()));
                        statuses_changed = true;
                    }
                }
            }
        }

        // -- 5b3. Playlist "now playing" variables (CAP-N17) ---------------------
        // At 1 Hz: copy each playlist's playing-item name into its configured
        // studio variable (CAP-N02). Text sources interpolating it repaint
        // through the ordinary variable-revision path; writes happen only on
        // an actual change, so an idle playlist costs one registry read.
        if last_now_playing.elapsed() >= Duration::from_secs(1) {
            last_now_playing = Instant::now();
            for source in &scene_sources {
                let SourceSettings::Playlist {
                    now_playing_variable,
                    ..
                } = &source.settings
                else {
                    continue;
                };
                let variable = now_playing_variable.trim();
                if variable.is_empty() {
                    continue;
                }
                let Some(playing) = fcap_sources::playlist::now_playing(&source.id.0.to_string())
                else {
                    continue;
                };
                if now_playing_cache.get(&source.id) != Some(&playing) {
                    app.state::<crate::automation::AutomationState>()
                        .set_variable(variable, &playing);
                    now_playing_cache.insert(source.id, playing);
                }
            }
            now_playing_cache.retain(|id, _| scene_sources.iter().any(|source| source.id == *id));
        }

        // -- 5b4. Title variables (CAP-N16) --------------------------------------
        // The playlist pattern inverted: at 1 Hz, when the automation
        // variables revision moved, hand the whole (bounded) map to the
        // title registry — running titles interpolate {{name}} on their next
        // tick, with no session restart and no spec change (unlike Text,
        // whose interpolated content rides the spec).
        if last_title_vars.elapsed() >= Duration::from_secs(1) {
            last_title_vars = Instant::now();
            let any_titles = scene_sources
                .iter()
                .any(|source| matches!(source.settings, SourceSettings::Title { .. }));
            if any_titles {
                let automation = app.state::<crate::automation::AutomationState>();
                let revision = automation.variables_revision();
                if fed_title_vars != Some(revision) {
                    fed_title_vars = Some(revision);
                    fcap_sources::title::set_variables(automation.variables());
                }
            }
        }

        // -- 5c. Bound Text faces (CAP-M16) --------------------------------------
        // Poll each bound file at BOUND_TEXT_POLL: one stat while unchanged;
        // read + extract + repaint only when the fingerprint moved AND the
        // extracted text differs. Transient read gaps (atomic temp+rename
        // writers) keep the last good face; selector/parse misses surface as
        // the source's honest error status without blanking the canvas.
        if !bound_texts.is_empty() {
            let now = Instant::now();
            for (id, settings) in &bound_texts {
                let state = bound_states.entry(*id).or_default();
                if state.next_poll.is_some_and(|at| now < at) {
                    continue;
                }
                state.next_poll = Some(now + BOUND_TEXT_POLL);
                let SourceSettings::Text {
                    source_file,
                    binding,
                    csv_row,
                    csv_column,
                    json_pointer,
                    ..
                } = settings
                else {
                    continue;
                };
                let trimmed = source_file.trim();
                // NEVER stat a UNC/URL path: on Windows that forces an SMB
                // connection (and an NTLM handshake) to the host, so an
                // untrusted collection binding a text source to
                // `\\attacker\share\x.txt` would leak the user's credential
                // hash every poll. Same guard the missing-file doctor uses.
                if crate::commands::studio::is_remote(trimmed) {
                    if statuses.get(id).map_or(true, |s| s.state != "error") {
                        statuses.insert(
                            *id,
                            SourceRuntime::error(
                                "unsupported",
                                "network paths (\\\\host\\share, url://) are not read".to_string(),
                            ),
                        );
                        statuses_changed = true;
                    }
                    continue;
                }
                let path = std::path::Path::new(trimmed);
                let Ok(meta) = std::fs::metadata(path) else {
                    // Missing right now: a rename gap (hold the face) or a
                    // genuinely absent file (say so once, honestly).
                    if state.last_good.is_none()
                        && statuses.get(id).map_or(true, |s| s.state != "error")
                    {
                        statuses.insert(
                            *id,
                            SourceRuntime::error(
                                "notFound",
                                format!("text file not found: {trimmed}"),
                            ),
                        );
                        statuses_changed = true;
                    }
                    continue;
                };
                let fingerprint = (meta.modified().ok(), meta.len());
                if state.fingerprint == Some(fingerprint) {
                    continue; // unchanged — the cheap steady state
                }
                if meta.len() > fcap_sources::textfile::MAX_BOUND_FILE_BYTES {
                    statuses.insert(
                        *id,
                        SourceRuntime::error(
                            "backend",
                            format!(
                                "the bound file is {} bytes — the cap is {}",
                                meta.len(),
                                fcap_sources::textfile::MAX_BOUND_FILE_BYTES
                            ),
                        ),
                    );
                    statuses_changed = true;
                    state.fingerprint = Some(fingerprint); // don't re-report every poll
                    continue;
                }
                let Ok(bytes) = std::fs::read(path) else {
                    continue; // mid-rename read gap — retry next poll
                };
                state.fingerprint = Some(fingerprint);
                let content = String::from_utf8_lossy(&bytes);
                let extracted = match binding {
                    fcap_scene::FileBinding::Whole => Ok(content.trim_end().to_string()),
                    fcap_scene::FileBinding::CsvCell => {
                        fcap_sources::textfile::csv_cell(&content, *csv_row, csv_column)
                    }
                    fcap_scene::FileBinding::JsonPointer => {
                        fcap_sources::textfile::json_value(&content, json_pointer)
                    }
                };
                match extracted {
                    Ok(value) => {
                        if state.last_good.as_deref() == Some(value.as_str()) {
                            continue; // the file changed but the shown text didn't
                        }
                        let style = text_style(settings, value.clone())
                            .expect("a bound source is always a Text kind");
                        match text::render_text(&style) {
                            Ok(frame) => {
                                let (w, h) = (frame.width, frame.height);
                                match compositor.upload_frame(*id, &frame) {
                                    Ok(()) => {
                                        state.last_good = Some(value);
                                        let stale = statuses.get(id).map_or(true, |prev| {
                                            prev.state != "live"
                                                || prev.width != Some(w)
                                                || prev.height != Some(h)
                                        });
                                        if stale {
                                            statuses.insert(*id, SourceRuntime::live(w, h));
                                            statuses_changed = true;
                                        }
                                    }
                                    Err(err) => {
                                        eprintln!("studio: bound text face dropped: {err}")
                                    }
                                }
                            }
                            Err(err) => {
                                statuses
                                    .insert(*id, SourceRuntime::error("backend", err.to_string()));
                                statuses_changed = true;
                            }
                        }
                    }
                    Err(message) => {
                        statuses.insert(*id, SourceRuntime::error("backend", message));
                        statuses_changed = true;
                    }
                }
            }
        }

        // -- 6. Compose + preview ------------------------------------------------
        let time = started_at.elapsed().as_secs_f32();
        // A running Studio-Mode commit renders BOTH scenes and blends them
        // (a stinger covers the swap with its video; a custom luma image
        // uploads once per transition); otherwise the program scene
        // composes directly.
        if let Some(pack) = &transition_pack {
            if !transition_was_active {
                transition_was_active = true;
                compositor.reset_stinger();
                compositor.set_transition_luma(pack.luma.as_ref().map(|luma| (**luma).clone()));
            }
        } else {
            if transition_was_active {
                // CAP-N29: the transition just ended — whether it completed
                // above or was cleared externally (collection load / save-as,
                // studio-mode off) — so release the audio duck exactly once
                // (a no-op if one was never armed).
                app.state::<crate::audio::AudioRuntime>()
                    .engine
                    .set_transition_duck(None);
            }
            transition_was_active = false;
        }
        // CAP-N25: snapshot newly-frozen items (their source is uploaded above)
        // and drop snapshots for items no longer frozen, before composing.
        compositor.freeze_items(&frozen_items);
        let frozen_ids: Vec<ItemId> = frozen_items.iter().map(|(id, _)| *id).collect();
        compositor.retain_frozen(&frozen_ids);
        // -- 6r. Floating reactions (TASK-614): computed ONCE per tick ----------
        // (particle state advances once), sprites uploaded up front; the draws
        // are baked into the program by every `bake` below, so preview,
        // recording, and stream all carry the exact same floating emoji.
        let mut reaction_draws = {
            let pending = app.state::<crate::reactions::ReactionState>().drain();
            crate::reactions::spawn(&mut reaction_particles, pending, time, &mut reaction_rng);
            let draws = crate::reactions::step(&mut reaction_particles, time, canvas);
            for draw in &draws {
                if !compositor.has_reaction_sprite(&draw.sprite) {
                    let style = fcap_sources::text::TextStyle {
                        text: draw.sprite.clone(),
                        font_family: Some(EMOJI_FONT.to_string()),
                        size_px: 96.0,
                        color: crate::reactions::tint_of(&draw.sprite),
                        ..Default::default()
                    };
                    match fcap_sources::text::render_text(&style) {
                        Ok(frame) => {
                            if let Err(err) = compositor.set_reaction_sprite(&draw.sprite, &frame) {
                                eprintln!("studio: reaction sprite upload failed: {err}");
                            }
                        }
                        Err(err) => {
                            eprintln!("studio: reaction sprite render failed: {err}")
                        }
                    }
                }
            }
            draws
        };
        // -- 6t. Telestrator (CAP-N57): the live annotation set, snapshot ONCE
        // per tick (faded strokes GC'd), baked over every `bake` below on its
        // own clock — so preview, recording, and stream all carry the same
        // marks at the moment they were drawn.
        let (telestrator_strokes, telestrator_now) = {
            let state = app.state::<crate::telestrator::TelestratorState>();
            let now = state.now();
            (state.snapshot(now), now)
        };
        // -- 6u. Featured chat banner (V1-E): re-render + re-upload ONLY when
        // the pin (or canvas) changes; the cached draw then joins the normal
        // reactions pass of every bake below — preview, recording, and stream
        // all carry it, with no extra render pass of its own.
        {
            let revision = fcap_sources::chat::featured_revision();
            if featured_seen != Some((revision, canvas)) {
                featured_seen = Some((revision, canvas));
                featured_draw_cache = fcap_sources::chat::featured().and_then(|message| {
                    let colors = app
                        .state::<crate::settings::SettingsStore>()
                        .get()
                        .featured_banner;
                    let bg =
                        crate::settings::FeaturedBannerSetting::rgba(&colors.bg, [16, 26, 42, 255]);
                    let fg = crate::settings::FeaturedBannerSetting::rgba(
                        &colors.text,
                        [255, 255, 255, 255],
                    );
                    // Banner width: 60% of the canvas, at least 240 px but
                    // never wider than the canvas itself. max-then-min (NOT
                    // clamp) — clamp panics when the canvas is under 240.
                    let width = (canvas.0 * 3 / 5).max(240).min(canvas.0);
                    match fcap_sources::chat::render_featured_banner(&message, width, bg, fg) {
                        Ok(frame) => {
                            match compositor.set_reaction_sprite(FEATURED_SPRITE, &frame) {
                                Ok(()) => Some(fcap_compositor::ReactionDraw {
                                    sprite: FEATURED_SPRITE.to_string(),
                                    // Bottom-centre, one banner-height above the edge.
                                    x: (canvas.0.saturating_sub(frame.width) / 2) as f32,
                                    y: canvas.1.saturating_sub(frame.height + frame.height / 2)
                                        as f32,
                                    size: frame.width as f32,
                                    alpha: 1.0,
                                }),
                                Err(err) => {
                                    eprintln!("studio: featured banner upload failed: {err}");
                                    None
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("studio: featured banner render failed: {err}");
                            None
                        }
                    }
                });
            }
        }
        if let Some(draw) = &featured_draw_cache {
            // Last = drawn topmost; the truncate keeps the banner inside the
            // pass's hard pool cap even under an emoji flood.
            reaction_draws.truncate(fcap_compositor::REACTION_POOL - 1);
            reaction_draws.push(draw.clone());
        }

        // One full bake of the program: the compose (with any transition or
        // stinger), then the floating reactions, then the downstream keyers
        // (CAP-N24) — persistent overlays composited over the finished
        // program, above every scene, surviving cuts — then the telestrator
        // annotation (CAP-N57). The CAP-N53 per-output variants below re-run
        // this with `set_output_hidden` armed, so a stream-only /
        // recording-only frame still carries transitions, reactions, keyers,
        // and telestrator marks.
        let bake = |compositor: &mut fcap_compositor::Compositor| {
            let compose_result = match &transition_pack {
                Some(pack) if pack.kind == fcap_scene::TransitionKind::Stinger => {
                    // The audience sees the outgoing scene until the cut point,
                    // the new program after — the stinger video covers the swap.
                    let shown = if pack.progress < pack.cut {
                        &pack.from_scene
                    } else {
                        &scene
                    };
                    compositor.render_scene_with_stinger(
                        shown,
                        time,
                        pack.stinger_frame.as_ref(),
                        pack.matte,
                    )
                }
                Some(pack) if pack.kind == fcap_scene::TransitionKind::Move => {
                    // CAP-N20: matched items morph between the two layouts.
                    // CAP-N21: eased so the motion accelerates in and settles out.
                    compositor.render_move(
                        &pack.from_scene,
                        &scene,
                        ease_in_out(pack.progress),
                        time,
                    )
                }
                Some(pack) => compositor.render_transition(
                    &pack.from_scene,
                    &scene,
                    pack.kind,
                    // CAP-N21: ease the blend (linear progress still drives cut /
                    // completion timing; only the visible curve is eased).
                    ease_in_out(pack.progress),
                    time,
                ),
                None => compositor.render(&scene, time),
            };
            if let Err(err) = compose_result {
                eprintln!("studio: compose failed: {err}");
            }
            if !reaction_draws.is_empty() {
                if let Err(err) = compositor.render_reactions(&reaction_draws) {
                    eprintln!("studio: reactions pass failed: {err}");
                }
            }
            if let Err(err) = compositor.render_downstream(&downstream_draws) {
                eprintln!("studio: downstream keyer pass failed: {err}");
            }
            if !telestrator_strokes.is_empty() {
                if let Err(err) =
                    compositor.render_telestrator(&telestrator_strokes, telestrator_now)
                {
                    eprintln!("studio: telestrator pass failed: {err}");
                }
            }
        };
        bake(&mut compositor);
        composed_this_second += 1;

        // -- 6a. Native preview surface (no readback — the "OBS feel") -----------
        if !native_disabled {
            let native = app.state::<crate::native_preview::NativePreviewState>();
            if let Some(handle) = native.composition_handle() {
                let (gen, bounds) = native.region();
                let sized = bounds.width > 0 && bounds.height > 0;
                if sized && native.is_visible() {
                    match &mut native_surface {
                        // The build runs under the overlay lock: its SetContent
                        // must not interleave with a UI-thread Commit, and the
                        // post-build commit rides the same lock.
                        None => match native.build_surface_serialized(|| {
                            handle
                                .create_surface(compositor.instance())
                                .map_err(|err| err.to_string())
                                .and_then(|surface| {
                                    compositor
                                        .native_preview_from_surface(
                                            surface,
                                            bounds.width,
                                            bounds.height,
                                        )
                                        .map_err(|err| err.to_string())
                                })
                        }) {
                            Ok(surface) => {
                                println!(
                                    "native preview: surface created {}x{} at ({},{})",
                                    bounds.width, bounds.height, bounds.x, bounds.y
                                );
                                native_surface = Some((surface, gen));
                            }
                            Err(err) => {
                                eprintln!(
                                    "studio: native preview surface failed ({err}) — \
                                     falling back to the JPEG preview"
                                );
                                native_disabled = true;
                                native.set_viable(false);
                            }
                        },
                        Some((surface, seen_gen)) => {
                            if *seen_gen != gen {
                                compositor.resize_native(surface, bounds.width, bounds.height);
                                *seen_gen = gen;
                            }
                        }
                    }
                }
                if native.is_visible() {
                    // The selection box + handles, drawn into the native frame
                    // (they'd sit hidden under the opaque surface otherwise).
                    let overlay = native_selection_overlay(
                        native.selection(),
                        &native.alignment_overlay(),
                        &scene,
                        &compositor,
                        canvas,
                    );
                    if let Some((surface, _)) = &mut native_surface {
                        match compositor.present_native(surface, overlay.as_ref()) {
                            Ok(_) => {}
                            Err(err) => {
                                eprintln!("studio: native present failed ({err})");
                                native_disabled = true;
                                native_surface = None;
                                native.set_viable(false);
                            }
                        }
                    }
                }
            }
        }

        // The recorder, the live stream and the replay buffer want the
        // full-res program frame every tick while their sessions run; the
        // preview JPEG keeps its ~30 fps cadence. One readback serves all —
        // except an output whose CAP-N53 variant is armed, which gets its own
        // bake + readback below (live outputs = stream lanes + Freally Link;
        // local-disk outputs = recorder + replay buffer).
        let recording = app.state::<crate::recording::RecordingState>();
        let streaming = app.state::<crate::stream::StreamBridgeState>();
        let replaying = app.state::<crate::replay::ReplayState>();
        let linking = app.state::<crate::link::LinkState>();
        let record_due = recording.wants_frames();
        let stream_due = streaming.wants_frames();
        let replay_due = replaying.wants_frames();
        // CAP-N42: an alpha recording gets its own transparent-clear render
        // below — the shared opaque readback then skips the recorder push.
        let record_alpha = record_due && recording.wants_alpha_frames();
        // Freally Link (CAP-N12): only while a receiver is connected; the
        // link thread JPEG-encodes on its own time — this hands it an Arc.
        let link_due = linking.wants_frames();
        let preview_due = last_readback.elapsed() >= READBACK_INTERVAL;

        // CAP-N53: which composed items are flagged off each output this tick
        // (master-hidden / group-hidden items are already skipped, so a flag
        // on one never forces a variant). Non-empty set = that output gets its
        // own program bake below; empty = the shared frame serves it. Only the
        // per-output-visibility consumers care, so the whole walk is skipped
        // when none is live — the preview always shows the full program.
        let (stream_hidden, record_hidden) = if record_due || stream_due || replay_due || link_due {
            let mut stream_hidden: HashSet<ItemId> = HashSet::new();
            let mut record_hidden: HashSet<ItemId> = HashSet::new();
            let relevant = std::iter::once(&scene)
                .chain(transition_pack.as_ref().map(|pack| &pack.from_scene))
                .chain(
                    vertical_pack
                        .as_ref()
                        .map(|(vertical_scene, _, _)| vertical_scene),
                )
                .chain(scene_pool.iter());
            for shown_scene in relevant {
                for item in &shown_scene.items {
                    if !item.visible || shown_scene.group_hides(item.id) {
                        continue;
                    }
                    if !item.on_stream {
                        stream_hidden.insert(item.id);
                    }
                    if !item.on_record {
                        record_hidden.insert(item.id);
                    }
                }
            }
            (stream_hidden, record_hidden)
        } else {
            (HashSet::new(), HashSet::new())
        };
        let stream_split = !stream_hidden.is_empty();
        let record_split = !record_hidden.is_empty();
        let record_shared = record_due && !record_alpha && !record_split;
        let stream_shared = stream_due && !stream_split;
        let replay_shared = replay_due && !record_split;
        let link_shared = link_due && !stream_split;
        if record_shared || stream_shared || replay_shared || link_shared || preview_due {
            match compositor.read_program() {
                Ok(frame) => {
                    let (frame_w, frame_h) = (frame.width, frame.height);
                    let data = Arc::new(frame.data);
                    if record_shared {
                        recording.push_video(Arc::clone(&data));
                    }
                    if stream_shared {
                        streaming.push_video(Arc::clone(&data));
                    }
                    if replay_shared {
                        replaying.push_video(Arc::clone(&data));
                    }
                    if link_shared {
                        linking.push_video(Arc::clone(&data), frame_w, frame_h);
                    }
                    // CAP-M10: black/frozen watch over THIS readback (never
                    // an extra one); it only alarms while the picture goes
                    // somewhere. ~2k sampled pixels — no budget impact.
                    // A deliberate panic slate (CAP-M22) is neither black
                    // nor frozen worth alarming about.
                    if preview_due {
                        let engaged = (record_due || stream_due) && !panic_active;
                        // A deliberate Freeze filter (CAP-N25) holds the program
                        // static on purpose — don't raise the frozen-frame alarm
                        // the operator caused (a genuine black frame still alarms).
                        let freeze_active = !frozen_items.is_empty();
                        for (kind, active) in video_watch.evaluate(&data, engaged, Instant::now()) {
                            if freeze_active && matches!(kind, crate::alarms::AlarmKind::Frozen) {
                                continue;
                            }
                            crate::alarms::emit_alarm(&app, kind, active, None);
                        }
                    }
                    if preview_due {
                        if let Some(jpeg) = encode_program_jpeg(
                            frame_w,
                            frame_h,
                            &data,
                            PREVIEW_MAX_WIDTH,
                            PREVIEW_MAX_HEIGHT,
                            PREVIEW_JPEG_QUALITY,
                        ) {
                            preview.publish(jpeg);
                        }
                        last_readback = Instant::now();
                    }
                }
                Err(err) => eprintln!("studio: program readback failed: {err}"),
            }
        }

        // -- 5a-N53. Per-output variants (CAP-N53): an output with flagged
        //    items gets its own full bake (transitions, reactions, and
        //    downstream keyers included) + readback. The native preview
        //    already presented the FULL program above, so the operator keeps
        //    seeing every item. The program texture is re-baked from scratch
        //    next tick, so leaving a variant in it here is safe — nothing
        //    else reads it out-of-band.
        if stream_split && (stream_due || link_due) {
            compositor.set_output_hidden(stream_hidden.clone());
            bake(&mut compositor);
            match compositor.read_program() {
                Ok(frame) => {
                    let (frame_w, frame_h) = (frame.width, frame.height);
                    let data = Arc::new(frame.data);
                    if stream_due {
                        streaming.push_video(Arc::clone(&data));
                    }
                    if link_due {
                        linking.push_video(Arc::clone(&data), frame_w, frame_h);
                    }
                }
                Err(err) => eprintln!("studio: stream-variant readback failed: {err}"),
            }
        }
        if record_split && ((record_due && !record_alpha) || replay_due) {
            compositor.set_output_hidden(record_hidden.clone());
            bake(&mut compositor);
            match compositor.read_program() {
                Ok(frame) => {
                    let data = Arc::new(frame.data);
                    if record_due && !record_alpha {
                        recording.push_video(Arc::clone(&data));
                    }
                    if replay_due {
                        replaying.push_video(Arc::clone(&data));
                    }
                }
                Err(err) => eprintln!("studio: record-variant readback failed: {err}"),
            }
        }

        // -- 5a-alpha. The CAP-N42 alpha recording: the same scene composed
        //    over a TRANSPARENT clear, feeding ONLY the recorder. The shared
        //    program (preview/stream/replay) stays opaque above. Recording-
        //    hidden items (CAP-N53) stay out of the alpha master too.
        if record_alpha {
            compositor.set_output_hidden(record_hidden.clone());
            match compositor.render_iso_view(&scene, started_at.elapsed().as_secs_f32(), true) {
                Ok(frame) => recording.push_video(Arc::new(frame.data)),
                Err(err) => {
                    if !alpha_error_logged {
                        alpha_error_logged = true;
                        eprintln!("studio: alpha compose failed ({err}) — recording holds");
                    }
                }
            }
        } else if alpha_error_logged {
            alpha_error_logged = false;
        }
        // CAP-N53: variants done — every later render this tick (Studio-Mode
        // preview pane, vertical canvas fast path, ISO lanes, projectors,
        // multiview, workbench) composes the full program again.
        compositor.set_output_hidden(HashSet::new());

        // -- CAP-N43/N44: scene-switch reactions — cut the frec lanes to a
        //    new part and/or drop a typed auto-marker when the program scene
        //    switches. Never on the panic slate (its scene is synthetic) and
        //    never on a session's first tick.
        let scene_reactions = recording.splits_on_scene() || recording.auto_markers();
        if recording.wants_frames() && scene_reactions && !panic_active {
            match split_last_scene {
                Some(last) if last != scene.id => {
                    if recording.splits_on_scene() {
                        recording.request_split_all();
                    }
                    if recording.auto_markers() {
                        crate::recording::add_auto_marker(&app, &format!("Scene: {}", scene.name));
                    }
                    split_last_scene = Some(scene.id);
                }
                None => split_last_scene = Some(scene.id),
                _ => {}
            }
        } else if split_last_scene.is_some() && !recording.wants_frames() {
            split_last_scene = None;
        }

        // -- 5b. The Studio-Mode preview pane (its own JPEG slot) ----------------
        if preview_due {
            match &preview_scene {
                Some(pane_scene) => {
                    match compositor
                        .render_preview_scene(pane_scene, started_at.elapsed().as_secs_f32())
                    {
                        Ok(frame) => {
                            let jpeg = encode_program_jpeg(
                                frame.width,
                                frame.height,
                                &frame.data,
                                PREVIEW_MAX_WIDTH,
                                PREVIEW_MAX_HEIGHT,
                                PREVIEW_JPEG_QUALITY,
                            );
                            preview.publish_studio_preview(jpeg);
                        }
                        Err(err) => eprintln!("studio: preview pane compose failed: {err}"),
                    }
                    studio_preview_live = true;
                }
                None if studio_preview_live => {
                    // Mode off (or the panes show one scene): clear the slot.
                    preview.publish_studio_preview(None);
                    studio_preview_live = false;
                }
                None => {}
            }
        }

        // -- 5c. The second (vertical) canvas (TASK-604) -------------------------
        // Rendered at full rate while a recorder/stream lane consumes it, at
        // the preview cadence otherwise (so its dialog preview stays live).
        compositor.set_vertical_canvas(vertical_pack.as_ref().map(|(_, w, h)| (*w, *h)));
        match &vertical_pack {
            Some((vertical_scene, _, _)) => {
                let vertical_record = recording.wants_vertical_frames();
                let vertical_stream = streaming.wants_vertical_frames();
                let vertical_time = started_at.elapsed().as_secs_f32();
                if !stream_split && !record_split {
                    // Fast path (no CAP-N53 flags in play): one render serves
                    // the recorder lane, the stream lane, and the dialog
                    // preview — exactly the pre-N53 behavior.
                    if vertical_record || vertical_stream || preview_due {
                        match compositor.render_vertical(vertical_scene, vertical_time) {
                            Ok(frame) => {
                                composed_vertical_this_second += 1;
                                let (frame_w, frame_h) = (frame.width, frame.height);
                                let data = Arc::new(frame.data);
                                if vertical_record {
                                    recording.push_video_vertical(Arc::clone(&data));
                                }
                                if vertical_stream {
                                    streaming.push_video_vertical(Arc::clone(&data));
                                }
                                if preview_due {
                                    let jpeg = encode_program_jpeg(
                                        frame_w,
                                        frame_h,
                                        &data,
                                        PREVIEW_MAX_WIDTH,
                                        PREVIEW_MAX_HEIGHT,
                                        PREVIEW_JPEG_QUALITY,
                                    );
                                    preview.publish_vertical_preview(jpeg);
                                    vertical_preview_live = true;
                                }
                            }
                            Err(err) => eprintln!("studio: vertical compose failed: {err}"),
                        }
                    }
                } else if vertical_record || vertical_stream || preview_due {
                    // CAP-N53 variants: each consumer composes with its own
                    // hidden set; the dialog preview stays the full canvas.
                    let mut rendered_any = false;
                    if vertical_stream {
                        compositor.set_output_hidden(stream_hidden.clone());
                        match compositor.render_vertical(vertical_scene, vertical_time) {
                            Ok(frame) => {
                                rendered_any = true;
                                streaming.push_video_vertical(Arc::new(frame.data));
                            }
                            Err(err) => eprintln!("studio: vertical compose failed: {err}"),
                        }
                    }
                    if vertical_record {
                        compositor.set_output_hidden(record_hidden.clone());
                        match compositor.render_vertical(vertical_scene, vertical_time) {
                            Ok(frame) => {
                                rendered_any = true;
                                recording.push_video_vertical(Arc::new(frame.data));
                            }
                            Err(err) => eprintln!("studio: vertical compose failed: {err}"),
                        }
                    }
                    if preview_due {
                        compositor.set_output_hidden(HashSet::new());
                        match compositor.render_vertical(vertical_scene, vertical_time) {
                            Ok(frame) => {
                                rendered_any = true;
                                let jpeg = encode_program_jpeg(
                                    frame.width,
                                    frame.height,
                                    &frame.data,
                                    PREVIEW_MAX_WIDTH,
                                    PREVIEW_MAX_HEIGHT,
                                    PREVIEW_JPEG_QUALITY,
                                );
                                preview.publish_vertical_preview(jpeg);
                                vertical_preview_live = true;
                            }
                            Err(err) => eprintln!("studio: vertical compose failed: {err}"),
                        }
                    }
                    // One vertical program frame per tick, however many
                    // variants composed it — the fps stat counts ticks.
                    if rendered_any {
                        composed_vertical_this_second += 1;
                    }
                    compositor.set_output_hidden(HashSet::new());
                }
            }
            None if vertical_preview_live => {
                preview.publish_vertical_preview(None);
                vertical_preview_live = false;
            }
            None => {}
        }

        // -- 5c-iso. The ISO lanes (CAP-N40): one clean per-source render +
        //    push per tick while lanes record. Uses the compositor's own ISO
        //    target, so it shares a tick with the preview/workbench passes.
        //    While the panic slate is up (CAP-M22) the lanes are HELD — they
        //    record sources clean, so panic must gate them or the cut
        //    content lands in every ISO file.
        {
            let iso_hold = panic_active && recording.wants_iso_frames();
            if iso_hold != iso_panic_held {
                recording.set_iso_panic_hold(iso_hold);
                iso_panic_held = iso_hold;
            }
        }
        if recording.wants_iso_frames() && !panic_active {
            for (source, post_filter) in recording.iso_lanes_wanted() {
                let Some((sw, sh)) = compositor.source_size(source) else {
                    // No frame yet (or the source left every scene): the
                    // lane's CFR clock waits or duplicates its last frame.
                    continue;
                };
                let filters = if post_filter {
                    scene
                        .items
                        .iter()
                        .find(|item| item.source == source)
                        .map(|item| item.filters.clone())
                        .unwrap_or_default()
                } else {
                    Vec::new()
                };
                let iso_scene = workbench_scene(source, filters, sw, sh, canvas);
                match compositor.render_iso_view(
                    &iso_scene,
                    started_at.elapsed().as_secs_f32(),
                    false,
                ) {
                    Ok(frame) => recording.push_video_iso(source, Arc::new(frame.data)),
                    Err(err) => {
                        if iso_errors_logged.insert(source) {
                            eprintln!(
                                "studio: ISO compose failed ({err}) — the lane holds its last frame"
                            );
                        }
                    }
                }
            }
        } else if !iso_errors_logged.is_empty() {
            iso_errors_logged.clear();
        }

        // -- 5d. The keying workbench (its own JPEG slot; CAP-M26) ---------------
        if preview_due {
            match &workbench_pack {
                Some((source, filters, mode, split)) => {
                    match render_workbench(
                        &mut compositor,
                        *source,
                        filters,
                        *mode,
                        *split,
                        canvas,
                        started_at.elapsed().as_secs_f32(),
                    ) {
                        Ok(Some((width, height, data))) => {
                            let jpeg = encode_program_jpeg(
                                width,
                                height,
                                &data,
                                PREVIEW_MAX_WIDTH,
                                PREVIEW_MAX_HEIGHT,
                                PREVIEW_JPEG_QUALITY,
                            );
                            preview.publish_workbench_preview(jpeg);
                            workbench_live = true;
                        }
                        Ok(None) => {} // no source frame yet — keep the last shown
                        Err(err) => eprintln!("studio: workbench compose failed: {err}"),
                    }
                }
                None if workbench_live => {
                    preview.publish_workbench_preview(None);
                    workbench_live = false;
                }
                None => {}
            }
        }

        // -- 5e. Multiview thumbnails (CAP-M06) ---------------------------------
        if let Some(scenes) = &multiview_scenes {
            if !scenes.is_empty() {
                let (tw, th) = multiview_thumb_size(canvas);
                let count = MULTIVIEW_BATCH.min(scenes.len());
                let time = started_at.elapsed().as_secs_f32();
                for offset in 0..count {
                    let scene = &scenes[(multiview_cursor + offset) % scenes.len()];
                    match compositor.render_thumbnail(scene, time, tw, th) {
                        Ok(frame) => {
                            let jpeg = encode_program_jpeg(
                                frame.width,
                                frame.height,
                                &frame.data,
                                tw,
                                th,
                                PREVIEW_JPEG_QUALITY,
                            );
                            preview.publish_multiview(&scene.id.0.to_string(), jpeg);
                        }
                        Err(err) => eprintln!("studio: multiview thumbnail failed: {err}"),
                    }
                }
                multiview_cursor = (multiview_cursor + count) % scenes.len();
            }
            last_multiview = Instant::now();
        }
        // Clear every thumbnail slot once when the monitor closes.
        if !multiview_on && multiview_was_on {
            preview.clear_multiview();
        }
        multiview_was_on = multiview_on;

        // -- 5f. Still-frame grab (CAP-M08) -------------------------------------
        if let Some((job, path)) = &still_job {
            let result = match job {
                StillJob::Program => {
                    // CAP-N53: a split tick (above) left a per-output VARIANT
                    // in the program texture. `output_hidden` is cleared by
                    // now, so re-bake the full program before the readback —
                    // a saved still must match the operator's program view,
                    // not the stream-only / recording-only frame.
                    if stream_split || record_split {
                        bake(&mut compositor);
                    }
                    compositor.read_program()
                }
                StillJob::Source(source, filters) => match compositor.source_size(*source) {
                    Some((sw, sh)) => {
                        let scene = workbench_scene(*source, filters.clone(), sw, sh, canvas);
                        compositor.render_source_view(
                            &scene,
                            started_at.elapsed().as_secs_f32(),
                            false,
                        )
                    }
                    None => Err(CompositorError::BadFrame(
                        "the source has no frame yet".into(),
                    )),
                },
            };
            match result {
                Ok(frame) => match save_still_png(&frame, path) {
                    Ok(()) => {
                        let _ = app.emit("still-saved", path.to_string_lossy().to_string());
                    }
                    Err(err) => {
                        eprintln!("studio: still save failed: {err}");
                        let _ = app.emit("still-error", err);
                    }
                },
                Err(err) => {
                    eprintln!("studio: still render failed: {err}");
                    let _ = app.emit("still-error", err.to_string());
                }
            }
        }

        // -- 5g. Scene/source projectors (CAP-M07 extension) --------------------
        let (projector_live, projector_jobs) = projector_pack;
        if let Some(jobs) = projector_jobs {
            let time = started_at.elapsed().as_secs_f32();
            for job in &jobs {
                let (key, frame) = match job {
                    ProjectorJob::Scene(key, scene) => {
                        (key, compositor.render_preview_scene(scene, time))
                    }
                    ProjectorJob::Source(key, source) => match compositor.source_size(*source) {
                        Some((sw, sh)) => {
                            let scene = workbench_scene(*source, Vec::new(), sw, sh, canvas);
                            (key, compositor.render_source_view(&scene, time, false))
                        }
                        None => (
                            key,
                            Err(CompositorError::BadFrame(
                                "the source has no frame yet".into(),
                            )),
                        ),
                    },
                };
                match frame {
                    Ok(frame) => {
                        // Full-res: max bounds = the frame size, so no downscale.
                        let jpeg = encode_program_jpeg(
                            frame.width,
                            frame.height,
                            &frame.data,
                            frame.width,
                            frame.height,
                            PREVIEW_JPEG_QUALITY,
                        );
                        preview.publish_projector(key, jpeg);
                    }
                    Err(err) => eprintln!("studio: projector render failed: {err}"),
                }
            }
            last_projector = Instant::now();
        }
        // Retire the slots of any projector target that has closed (an empty
        // live set clears them all once the last projector window is gone).
        preview.retain_projectors(&projector_live);

        // -- 7. The program event (1 Hz, or sooner on a state change) -----------
        if statuses_changed || last_program_event.elapsed() >= PROGRAM_EVENT_INTERVAL {
            let elapsed = last_program_event.elapsed().as_secs_f32().max(0.001);
            let mut sources: HashMap<String, SourceRuntime> = HashMap::new();
            for (id, status) in &statuses {
                let mut status = status.clone();
                if let Some(slot) = sessions.get_mut(id) {
                    if status.state == "live" {
                        status.fps =
                            Some((slot.frames_this_second as f32 / elapsed).round() as u32);
                    }
                    // Health dashboard extras (CAP-M13): staleness + drops.
                    status.last_frame_ms = slot
                        .last_frame
                        .map(|at| at.elapsed().as_millis().min(u64::MAX as u128) as u64);
                    status.dropped = Some(slot.session.frames().dropped());
                    slot.frames_this_second = 0;
                }
                // Restart history (manual retry + auto-recover), from the
                // tick snapshot's nonce clone.
                let retries = nonces.get(id).copied().unwrap_or(0);
                if retries > 0 {
                    status.retries = Some(retries);
                }
                sources.insert(id.0.to_string(), status);
            }
            let dropped = sessions
                .values()
                .map(|slot| slot.session.frames().dropped())
                .sum();
            let status = ProgramStatus {
                state: "running",
                width: canvas.0,
                height: canvas.1,
                fps: (composed_this_second as f32 / elapsed).round() as u32,
                render_micros: compositor.last_render_cpu_micros(),
                adapter: compositor.adapter_summary().to_string(),
                dropped,
                sources,
            };
            // Hand the render numbers to the stats dock's emitter, and the
            // errored-source count to the CAP-M09 pre-flight hold.
            app.state::<crate::events::RuntimeStats>().publish(
                status.fps,
                (composed_vertical_this_second as f32 / elapsed).round() as u32,
                status.dropped,
                status.render_micros,
            );
            app.state::<crate::events::RuntimeStats>()
                .set_errored_sources(
                    status
                        .sources
                        .values()
                        .filter(|source| source.state == "error")
                        .count() as u32,
                );
            if app.emit("program", &status).is_err() {
                break; // the app is gone — wind down
            }
            composed_this_second = 0;
            composed_vertical_this_second = 0;
            last_program_event = Instant::now();
            statuses_changed = false;
        }

        // -- 7b. Auto-recover errored device/window captures (OBS-style) --------
        // Re-attempt sources that errored because their window / display /
        // camera wasn't available, so they re-bind on their own the moment it
        // comes back — no manual Retry. Bumping the retry nonce (+ revision) is
        // exactly what the manual Retry does; the next tick's reconcile restarts
        // them. Each source backs off (AUTO_RETRY_MIN doubling to AUTO_RETRY_MAX)
        // so a never-returning source settles to an occasional retry instead of
        // thrashing waiting↔error; recovering (going live) or leaving the scene
        // clears its schedule, so a fresh error starts the backoff over.
        {
            let now = Instant::now();
            // Keep a schedule entry across the transient waiting↔error churn of a
            // retry; drop it only once the source recovered or left the scene.
            retry_schedule.retain(|id, _| {
                scene_sources.iter().any(|source| source.id == *id)
                    && statuses.get(id).map(|status| status.state) != Some("live")
            });
            let mut due: Vec<SourceId> = Vec::new();
            for source in &scene_sources {
                let errored = auto_recoverable(&source.settings)
                    && statuses
                        .get(&source.id)
                        .is_some_and(|status| status.state == "error");
                if !errored {
                    continue;
                }
                match retry_schedule.get_mut(&source.id) {
                    // Newly errored: wait one MIN interval before the first try.
                    None => {
                        retry_schedule.insert(source.id, (now + AUTO_RETRY_MIN, AUTO_RETRY_MIN));
                    }
                    Some((next, backoff)) if now >= *next => {
                        due.push(source.id);
                        *backoff = (*backoff * 2).min(AUTO_RETRY_MAX);
                        *next = now + *backoff;
                    }
                    Some(_) => {}
                }
            }
            if !due.is_empty() {
                let mut guard = lock_core(&core);
                for id in &due {
                    *guard.retry_nonces.entry(*id).or_insert(0) += 1;
                }
                guard.revision += 1;
            }
        }

        // -- 7b. Automation (CAP-N01/N02): evaluate the rules against this
        // tick's signals and advance any running macros. A no-op while no
        // rule is enabled (off by default).
        if last_automation.elapsed() >= Duration::from_secs(1) {
            last_automation = Instant::now();
            let (scene_name, source_names) = {
                let guard = lock_core(&core);
                let name = guard.collection.active_scene().name.clone();
                let names: HashMap<SourceId, String> = guard
                    .collection
                    .sources
                    .iter()
                    .map(|source| (source.id, source.name.clone()))
                    .collect();
                (name, names)
            };
            let errored: Vec<String> = statuses
                .iter()
                .filter(|(_, status)| status.state == "error")
                .filter_map(|(id, _)| source_names.get(id).cloned())
                .collect();
            crate::automation::tick(&app, &scene_name, errored, &source_names);
            // The rundown (CAP-N09) shares the 1 Hz cadence: it keeps the
            // countdown live and auto-advances an expired step (when the
            // operator enabled that).
            crate::rundown::tick(&app);
            // CAP-N44: reconnect + dropped-burst auto-markers share the 1 Hz
            // cadence too — both are edge-triggered so a long outage marks
            // once, not sixty times a minute.
            if recording.wants_frames() && recording.auto_markers() {
                let stream = app.state::<crate::stream::StreamBridgeState>().status();
                let reconnecting = stream.state == "reconnecting";
                if reconnecting && !was_reconnecting {
                    crate::recording::add_auto_marker(&app, "Stream reconnecting");
                }
                was_reconnecting = reconnecting;
                let (_, _, _, dropped, _) = app.state::<crate::events::RuntimeStats>().latest();
                // A burst = ≥ 30 frames dropped inside one second; edge-trigger
                // it so a sustained stall marks the moment it STARTS, not every
                // second it lasts.
                let bursting = last_dropped.is_some_and(|last| dropped.saturating_sub(last) >= 30);
                if bursting && !was_dropping {
                    crate::recording::add_auto_marker(&app, "Dropped frames");
                }
                was_dropping = bursting;
                last_dropped = Some(dropped);
            } else {
                was_reconnecting = false;
                was_dropping = false;
                last_dropped = None;
            }
            // PTZ per-scene preset auto-recall (CAP-N08) — on the switch only.
            if ptz_scene.as_deref() != Some(scene_name.as_str()) {
                ptz_scene = Some(scene_name.clone());
                crate::ptz::on_scene(&app, &scene_name);
            }
        }

        // -- 8. Autosave (debounced behind the last mutation) --------------------
        {
            let mut guard = lock_core(&core);
            if guard
                .dirty_since
                .is_some_and(|since| since.elapsed() >= AUTOSAVE_DEBOUNCE)
            {
                persist(&mut guard);
            }
        }

        // -- 9. Pace to ~60 fps ---------------------------------------------------
        let spent = tick_started.elapsed();
        if spent < TICK {
            std::thread::sleep(TICK - spent);
        }
    }

    // Shutdown: stop the OS pipelines deliberately.
    for (_, slot) in sessions.drain() {
        slot.session.stop();
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Map the model's deinterlace choice (CAP-M17) onto the capture pipeline's
/// algorithm pair; `Off` runs no pass at all.
fn deinterlace_config(
    mode: fcap_scene::DeinterlaceMode,
    order: fcap_scene::FieldOrder,
) -> Option<(
    fcap_sources::deinterlace::Mode,
    fcap_sources::deinterlace::FieldOrder,
)> {
    use fcap_scene::DeinterlaceMode as M;
    use fcap_sources::deinterlace::Mode;
    let mode = match mode {
        M::Off => return None,
        M::Discard => Mode::Discard,
        M::Bob => Mode::Bob,
        M::Linear => Mode::Linear,
        M::Blend => Mode::Blend,
        M::MotionAdaptive => Mode::MotionAdaptive,
    };
    let order = match order {
        fcap_scene::FieldOrder::TopFirst => fcap_sources::deinterlace::FieldOrder::TopFirst,
        fcap_scene::FieldOrder::BottomFirst => fcap_sources::deinterlace::FieldOrder::BottomFirst,
    };
    Some((mode, order))
}

fn is_capture_backed(settings: &SourceSettings) -> bool {
    matches!(
        settings,
        SourceSettings::Display { .. }
            | SourceSettings::Window { .. }
            | SourceSettings::Portal {}
            | SourceSettings::VideoDevice { .. }
            | SourceSettings::Media { .. }
            | SourceSettings::Slideshow { .. }
            | SourceSettings::ChatOverlay { .. }
            | SourceSettings::RemoteGuest { .. }
            | SourceSettings::TestSweep { .. }
            | SourceSettings::TestFlashBeep { .. }
            | SourceSettings::AudioVisualizer { .. }
            | SourceSettings::SplitTimer { .. }
            | SourceSettings::Playlist { .. }
            | SourceSettings::ReplayPlayback { .. }
            | SourceSettings::LanIngest { .. }
            | SourceSettings::InputOverlay { .. }
            | SourceSettings::Title { .. }
            | SourceSettings::FreallyLink { .. }
    )
}

/// Which source kinds auto-recover on a timer: hardware / window captures that
/// can come back (a reopened window, a replugged display, a reconnected
/// camera). Portal and media are excluded on purpose — a portal retry would
/// re-pop the system picker dialog, and a bad media path won't fix itself.
fn auto_recoverable(settings: &SourceSettings) -> bool {
    matches!(
        settings,
        SourceSettings::Display { .. }
            | SourceSettings::Window { .. }
            | SourceSettings::VideoDevice { .. }
    )
}

/// Read + parse a `.lss` split file: existence, the size cap, UTF-8, and the
/// parse, each mapped to one honest "split file: …" error. The caller has
/// already refused remote paths (the CAP-M16 rule).
fn load_lss(path: &str) -> Result<fcap_sources::splits::SplitFile, CaptureError> {
    let meta = std::fs::metadata(path)
        .map_err(|err| CaptureError::Backend(format!("split file: {err}")))?;
    if meta.len() > fcap_sources::splits::MAX_LSS_BYTES {
        return Err(CaptureError::Backend("split file is too large".into()));
    }
    let content = std::fs::read_to_string(path)
        .map_err(|err| CaptureError::Backend(format!("split file: {err}")))?;
    fcap_sources::splits::parse_lss(&content)
        .map_err(|err| CaptureError::Backend(format!("split file: {err}")))
}

/// The native preview's selection overlay, computed from the model: the
/// selected item's four content corners (canvas px) + its locked flag, or
/// `None` when nothing usable is selected (no selection, awaiting first frame,
/// no known size, or fully cropped). Preview-only chrome — this never touches
/// the program frame the recorder and stream read.
fn native_selection_overlay(
    selection: Option<fcap_scene::ItemId>,
    alignment: &crate::native_preview::AlignmentOverlay,
    scene: &fcap_scene::Scene,
    compositor: &Compositor,
    canvas: (u32, u32),
) -> Option<fcap_compositor::PreviewOverlay> {
    // The selected item's corners + lock state, when a usable item is selected.
    let selection = selection.and_then(|id| {
        let item = scene.items.iter().find(|it| it.id == id)?;
        if item.pending_fit {
            return None;
        }
        let (source_w, source_h) = compositor.source_size(item.source)?;
        let (eff_w, eff_h) =
            fcap_compositor::effective_source_size((source_w, source_h), &item.filters);
        let content = fcap_compositor::transform::content_size(eff_w, eff_h, &item.transform.crop)?;
        Some((
            fcap_compositor::transform::corners(&item.transform, content),
            item.locked,
        ))
    });
    // The alignment aids (safe areas + guides) the UI pushed (CAP-M04).
    let safe_areas: Vec<[f32; 4]> = alignment
        .safe_areas
        .iter()
        .map(|rect| [rect.x, rect.y, rect.w, rect.h])
        .collect();
    let guides: Vec<[f32; 4]> = alignment
        .guides
        .iter()
        .map(|guide| guide.segment())
        .collect();
    // Nothing to draw → no overlay (the blit runs alone).
    if selection.is_none() && safe_areas.is_empty() && guides.is_empty() {
        return None;
    }
    Some(fcap_compositor::PreviewOverlay {
        canvas: (canvas.0 as f32, canvas.1 as f32),
        selection,
        safe_areas,
        guides,
    })
}

/// Queue a still-frame grab (CAP-M08): compute a timestamped PNG path in the
/// recordings folder and hand it to the render thread. Shared by the command
/// and the global hotkey.
pub fn capture_still<R: Runtime>(app: &AppHandle<R>, target: StillTarget) {
    let recording = app
        .state::<crate::settings::SettingsStore>()
        .get()
        .recording;
    let folder = crate::recording::output_folder(&recording, &recording.still_folder);
    // A custom still folder may not exist yet — create it here so the render
    // thread's save can't fail on a missing directory.
    if let Err(err) = std::fs::create_dir_all(&folder) {
        eprintln!("still: could not create {}: {err}", folder.display());
    }
    let counter = crate::recording::counter_for(app, &recording.still_template, recording.counter);
    let canvas = app
        .state::<StudioState>()
        .with_collection(|collection| (collection.canvas_width, collection.canvas_height));
    let naming = crate::recording::naming_context(app, "Still".to_owned(), canvas, counter);
    let stem = crate::filename::resolve_template(&recording.still_template, &naming);
    let path = crate::recording::unique_recording_path(&folder, &stem, "png", false);
    app.state::<StudioState>().request_still(target, path);
}

/// Save a tight RGBA frame as a lossless PNG (CAP-M08). `::image` names the
/// crate — `image` alone is the `fcap_sources::image` module in this file.
fn save_still_png(frame: &ProgramFrame, path: &std::path::Path) -> Result<(), String> {
    let buffer: ::image::RgbaImage =
        ::image::ImageBuffer::from_raw(frame.width, frame.height, frame.data.clone())
            .ok_or_else(|| "still frame buffer size mismatch".to_string())?;
    buffer.save(path).map_err(|err| err.to_string())
}

/// The multiview thumbnail size for a canvas — a fixed width, height matched to
/// the canvas aspect so thumbnails are never distorted (CAP-M06).
fn multiview_thumb_size(canvas: (u32, u32)) -> (u32, u32) {
    let (cw, ch) = canvas;
    let width = MULTIVIEW_THUMB_WIDTH;
    let height = ((width as u64 * ch.max(1) as u64) / cw.max(1) as u64).max(1) as u32;
    (width, height)
}

/// Build a one-item scene that fits `source` to the canvas — the keying
/// workbench's synthetic scene (CAP-M26). `filters` are the item's chain
/// (empty for the raw "Source" view).
fn workbench_scene(
    source: SourceId,
    filters: Vec<Filter>,
    sw: u32,
    sh: u32,
    canvas: (u32, u32),
) -> Scene {
    let scale = (canvas.0 as f32 / sw.max(1) as f32).min(canvas.1 as f32 / sh.max(1) as f32);
    let mut item = SceneItem::new(source);
    item.transform = Transform {
        x: canvas.0 as f32 / 2.0,
        y: canvas.1 as f32 / 2.0,
        scale_x: scale,
        scale_y: scale,
        rotation: 0.0,
        crop: Crop::default(),
        ..Default::default()
    };
    item.pending_fit = false;
    item.filters = filters;
    let mut scene = Scene::new("workbench");
    scene.items = vec![item];
    scene
}

/// Stitch two same-sized frames into a left|right split at `split` (0..1) —
/// the workbench's before/after view (CAP-M26).
fn stitch_split(left: &ProgramFrame, right: &ProgramFrame, split: f32) -> Vec<u8> {
    let (w, h) = (left.width, left.height);
    let cut = ((split.clamp(0.0, 1.0) * w as f32) as u32).min(w);
    let row_bytes = (w * 4) as usize;
    let mut data = left.data.clone();
    if right.data.len() == data.len() {
        for y in 0..h as usize {
            let row = y * row_bytes;
            let start = row + (cut * 4) as usize;
            let end = row + row_bytes;
            data[start..end].copy_from_slice(&right.data[start..end]);
        }
    }
    data
}

/// Render the keying workbench's single source (CAP-M26) in `mode`, returning
/// `(width, height, RGBA)` — or `None` when the source has no frame yet.
fn render_workbench(
    compositor: &mut Compositor,
    source: SourceId,
    filters: &[Filter],
    mode: WorkbenchMode,
    split: f32,
    canvas: (u32, u32),
    time: f32,
) -> Result<Option<(u32, u32, Vec<u8>)>, CompositorError> {
    let Some((sw, sh)) = compositor.source_size(source) else {
        return Ok(None);
    };
    let keyed = workbench_scene(source, filters.to_vec(), sw, sh, canvas);
    let raw = workbench_scene(source, Vec::new(), sw, sh, canvas);
    match mode {
        WorkbenchMode::Source => {
            let frame = compositor.render_source_view(&raw, time, false)?;
            Ok(Some((frame.width, frame.height, frame.data)))
        }
        WorkbenchMode::Keyed => {
            let frame = compositor.render_source_view(&keyed, time, false)?;
            Ok(Some((frame.width, frame.height, frame.data)))
        }
        WorkbenchMode::Matte => {
            let frame = compositor.render_source_view(&keyed, time, true)?;
            Ok(Some((frame.width, frame.height, frame.data)))
        }
        WorkbenchMode::Split => {
            let left = compositor.render_source_view(&raw, time, false)?;
            let right = compositor.render_source_view(&keyed, time, false)?;
            let data = stitch_split(&left, &right, split);
            Ok(Some((left.width, left.height, data)))
        }
    }
}

/// A change-detection fingerprint of a source's settings.
fn source_spec(settings: &SourceSettings) -> String {
    serde_json::to_string(settings).expect("source settings always serialize")
}

/// Kick off a capture session on a helper thread (portal starts block on the
/// system dialog; nothing may stall the render loop). `camera_profile` is
/// the CAP-M18 saved control set (VideoDevice only; empty otherwise).
fn start_session(
    id: SourceId,
    settings: &SourceSettings,
    starting: &mut HashMap<SourceId, mpsc::Receiver<Result<CaptureSession, CaptureError>>>,
    reactions_queue: &Arc<Mutex<Vec<String>>>,
    camera_profile: Vec<(String, i64)>,
) {
    let (tx, rx) = mpsc::channel();
    let settings = settings.clone();
    let reactions_queue = Arc::clone(reactions_queue);
    let spawned = std::thread::Builder::new()
        .name("fcap-source-start".into())
        .spawn(move || {
            let result = match &settings {
                SourceSettings::Display { capture_id, .. }
                | SourceSettings::Window { capture_id, .. } => {
                    fcap_capture::start_capture(capture_id)
                }
                SourceSettings::Portal {} => fcap_capture::start_capture("portal"),
                SourceSettings::VideoDevice {
                    device_id,
                    format,
                    deinterlace,
                    field_order,
                } => {
                    let format = format.as_ref().map(|f| VideoFormatInfo {
                        width: f.width,
                        height: f.height,
                        fps: f.fps,
                        fourcc: f.fourcc.clone(),
                    });
                    video_device::start_video_device(
                        device_id,
                        format.as_ref(),
                        deinterlace_config(*deinterlace, *field_order),
                        camera_profile,
                    )
                }
                SourceSettings::Media {
                    path,
                    looping,
                    hw_decode,
                    reverse,
                    ..
                } => {
                    // The source id keys the mixer-side audio ring.
                    fcap_sources::media::start_media(
                        &id.0.to_string(),
                        path,
                        *looping,
                        *hw_decode,
                        *reverse,
                    )
                }
                SourceSettings::Slideshow {
                    paths,
                    slide_ms,
                    transition_ms,
                    looping,
                    shuffle,
                } => fcap_sources::slideshow::start_slideshow(
                    paths,
                    *slide_ms,
                    *transition_ms,
                    *looping,
                    *shuffle,
                ),
                SourceSettings::ChatOverlay {
                    youtube,
                    twitch,
                    kick,
                    width,
                    max_lines,
                    font_size,
                } => fcap_sources::chat::start_chat_overlay(
                    &fcap_sources::chat::ChatOverlayConfig {
                        youtube: youtube.clone(),
                        twitch: twitch.clone(),
                        kick: kick.clone(),
                        width: *width,
                        max_lines: *max_lines,
                        font_size: *font_size,
                    },
                    // TASK-614: reaction emoji spotted in chat float over
                    // the program — the same no-key ingest, no extra API.
                    Some({
                        let queue = Arc::clone(&reactions_queue);
                        Arc::new(move |text: &str| {
                            for emoji in crate::reactions::reactions_in_chat(text) {
                                let _ = crate::reactions::push_into(&queue, emoji);
                            }
                        })
                    }),
                ),
                // Frames are pushed from the webview's WebRTC session over
                // IPC — the session just opens the push channel.
                SourceSettings::RemoteGuest { .. } => crate::remote::start_remote_guest(id),
                SourceSettings::TestSweep { width, height } => {
                    fcap_sources::testsignal::start_sweep(*width, *height)
                }
                SourceSettings::TestFlashBeep { width, height } => {
                    // The source id keys the mixer-side audio ring (like Media).
                    fcap_sources::testsignal::start_flash_beep(&id.0.to_string(), *width, *height)
                }
                SourceSettings::AudioVisualizer {
                    style,
                    target,
                    track,
                    source,
                    width,
                    height,
                    bands,
                    color,
                    peak_hold,
                    decay,
                    classic,
                } => {
                    let target = match target {
                        fcap_scene::VisTargetKind::Master => {
                            Some(fcap_audio::vis::VisTarget::Master)
                        }
                        fcap_scene::VisTargetKind::Track => {
                            let bus = (*track).clamp(1, fcap_scene::TRACK_COUNT as u32);
                            Some(fcap_audio::vis::VisTarget::Track((bus - 1) as usize))
                        }
                        fcap_scene::VisTargetKind::Source => source
                            .as_ref()
                            .map(|id| fcap_audio::vis::VisTarget::Source(id.0.to_string())),
                    };
                    match target {
                        Some(target) => fcap_sources::visualizer::start_visualizer(
                            fcap_sources::visualizer::VisualizerConfig {
                                style: match style {
                                    fcap_scene::VisStyle::Bars => {
                                        fcap_sources::visualizer::Style::Bars
                                    }
                                    fcap_scene::VisStyle::Scope => {
                                        fcap_sources::visualizer::Style::Scope
                                    }
                                    fcap_scene::VisStyle::Vu => fcap_sources::visualizer::Style::Vu,
                                },
                                target,
                                width: *width,
                                height: *height,
                                bands: *bands,
                                color: [color.r, color.g, color.b, color.a],
                                peak_hold: *peak_hold,
                                decay_db_per_s: *decay,
                                classic: *classic,
                            },
                        ),
                        // No strip picked yet — an honest error beats a
                        // guessed signal (the properties dialog fixes it).
                        None => Err(CaptureError::Backend(
                            "pick an audio source for the visualizer".into(),
                        )),
                    }
                }
                SourceSettings::SplitTimer {
                    path,
                    comparison,
                    width,
                    height,
                    size_px,
                    color,
                    ahead,
                    behind,
                    gold,
                } => {
                    let trimmed = path.trim();
                    if trimmed.is_empty() {
                        Err(CaptureError::Backend("pick a .lss split file".into()))
                    } else if crate::commands::studio::is_remote(trimmed) {
                        // The CAP-M16 rule: statting a UNC path on Windows
                        // forces an SMB/NTLM handshake — never probe one.
                        Err(CaptureError::Backend(
                            "network paths are not read — use a local split file".into(),
                        ))
                    } else {
                        load_lss(trimmed)
                            .and_then(|file| {
                                fcap_sources::splits::start_split_timer(
                                    &id.0.to_string(),
                                    fcap_sources::splits::SplitTimerConfig {
                                        file,
                                        comparison: match comparison {
                                            fcap_scene::SplitComparison::PersonalBest => {
                                                fcap_sources::splits::Comparison::Pb
                                            }
                                            fcap_scene::SplitComparison::BestSegments => {
                                                fcap_sources::splits::Comparison::BestSegments
                                            }
                                            fcap_scene::SplitComparison::Average => {
                                                fcap_sources::splits::Comparison::Average
                                            }
                                        },
                                        width: *width,
                                        height: *height,
                                        size_px: *size_px,
                                        color: [color.r, color.g, color.b, color.a],
                                        ahead: [ahead.r, ahead.g, ahead.b, ahead.a],
                                        behind: [behind.r, behind.g, behind.b, behind.a],
                                        gold: [gold.r, gold.g, gold.b, gold.a],
                                    },
                                )
                            })
                    }
                }
                SourceSettings::Playlist {
                    items,
                    looping,
                    shuffle,
                    hold_last,
                    hw_decode,
                    hidden_face,
                    ..
                } => {
                    // The CAP-M16 rule for EVERY user path: refuse network
                    // paths before any stat/probe (NTLM handshake leak).
                    let hostile = items
                        .iter()
                        .find(|item| crate::commands::studio::is_remote(item.path.trim()));
                    if items.is_empty() {
                        Err(CaptureError::Backend("the playlist is empty".into()))
                    } else if hostile.is_some() {
                        Err(CaptureError::Backend(
                            "network paths are not read — playlist items must be local files"
                                .into(),
                        ))
                    } else {
                        fcap_sources::playlist::start_playlist(
                            &id.0.to_string(),
                            fcap_sources::playlist::PlaylistConfig {
                                items: items
                                    .iter()
                                    .map(|item| fcap_sources::playlist::PlaylistItemSpec {
                                        path: item.path.trim().to_string(),
                                        in_s: item.in_s.max(0.0),
                                        out_s: item.out_s.max(0.0),
                                    })
                                    .collect(),
                                looping: *looping,
                                shuffle: *shuffle,
                                hold_last: *hold_last,
                                hw_decode: *hw_decode,
                                hidden_face: *hidden_face,
                            },
                        )
                    }
                }
                SourceSettings::ReplayPlayback {
                    speed, hw_decode, ..
                } => fcap_sources::replaysrc::start_replay_source(
                    &id.0.to_string(),
                    fcap_sources::replaysrc::ReplaySourceConfig {
                        speed: match speed {
                            fcap_scene::ReplaySpeed::Full => fcap_sources::replaysrc::Speed::Full,
                            fcap_scene::ReplaySpeed::Half => fcap_sources::replaysrc::Speed::Half,
                            fcap_scene::ReplaySpeed::Quarter => {
                                fcap_sources::replaysrc::Speed::Quarter
                            }
                        },
                        hw_decode: *hw_decode,
                    },
                ),
                // CAP-N13: input is polled inside this session's thread only
                // — reading starts when the source goes live and stops with
                // it; nothing is ever logged (the module doc pins the scope).
                SourceSettings::InputOverlay {
                    layout,
                    color,
                    accent,
                } => fcap_sources::inputoverlay::start_input_overlay(
                    fcap_sources::inputoverlay::InputOverlayConfig {
                        layout: match layout {
                            fcap_scene::InputLayout::Wasd => {
                                fcap_sources::inputoverlay::Layout::Wasd
                            }
                            fcap_scene::InputLayout::Keyboard => {
                                fcap_sources::inputoverlay::Layout::Keyboard
                            }
                            fcap_scene::InputLayout::Gamepad => {
                                fcap_sources::inputoverlay::Layout::Gamepad
                            }
                            fcap_scene::InputLayout::Fightstick => {
                                fcap_sources::inputoverlay::Layout::Fightstick
                            }
                        },
                        color: [color.r, color.g, color.b, color.a],
                        accent: [accent.r, accent.g, accent.b, accent.a],
                    },
                ),
                SourceSettings::Title {
                    width,
                    height,
                    layers,
                    animation,
                    duration_ms,
                } => {
                    // The CAP-M16 rule for EVERY user path: refuse network
                    // paths before any stat/probe (NTLM handshake leak) —
                    // image layers load at start, bound files and fonts are
                    // read while the session runs.
                    let hostile = layers.iter().any(|layer| match layer {
                        fcap_scene::TitleLayer::Image { path, .. } => {
                            crate::commands::studio::is_remote(path.trim())
                        }
                        fcap_scene::TitleLayer::Text {
                            source_file,
                            font_file,
                            ..
                        } => {
                            crate::commands::studio::is_remote(source_file.trim())
                                || font_file
                                    .as_deref()
                                    .is_some_and(|file| crate::commands::studio::is_remote(file.trim()))
                        }
                        fcap_scene::TitleLayer::Rect { .. } => false,
                    });
                    if hostile {
                        Err(CaptureError::Backend(
                            "network paths are not read — title images, fonts, and bound files must be local".into(),
                        ))
                    } else {
                        fcap_sources::title::start_title(
                            &id.0.to_string(),
                            fcap_sources::title::TitleConfig {
                                width: *width,
                                height: *height,
                                layers: layers.iter().map(title_layer_spec).collect(),
                                animation: match animation {
                                    fcap_scene::TitleAnimation::None => {
                                        fcap_sources::title::Animation::None
                                    }
                                    fcap_scene::TitleAnimation::Fade => {
                                        fcap_sources::title::Animation::Fade
                                    }
                                    fcap_scene::TitleAnimation::SlideLeft => {
                                        fcap_sources::title::Animation::SlideLeft
                                    }
                                    fcap_scene::TitleAnimation::SlideUp => {
                                        fcap_sources::title::Animation::SlideUp
                                    }
                                    fcap_scene::TitleAnimation::Wipe => {
                                        fcap_sources::title::Animation::Wipe
                                    }
                                },
                                duration_ms: *duration_ms,
                            },
                        )
                    }
                }
                SourceSettings::LanIngest {
                    protocol,
                    port,
                    passphrase,
                } => {
                    let proto = match protocol {
                        fcap_scene::IngestProtocol::Srt => {
                            fcap_sources::laningest::IngestProtocol::Srt
                        }
                        fcap_scene::IngestProtocol::Rtmp => {
                            fcap_sources::laningest::IngestProtocol::Rtmp
                        }
                    };
                    // The waiting face shows the reachable URL. The local-IP
                    // probe only asks the OS which interface faces the LAN —
                    // nothing is sent (webpanel's trick).
                    let host = crate::commands::studio::lan_ip();
                    fcap_sources::laningest::start_lan_ingest(
                        &id.0.to_string(),
                        fcap_sources::laningest::LanIngestConfig {
                            protocol: proto,
                            port: *port,
                            passphrase: passphrase.clone(),
                            connect_url: fcap_sources::laningest::connect_url(
                                proto, &host, *port, passphrase,
                            ),
                        },
                    )
                }
                // Freally Link (CAP-N12): a network address by design — it
                // is never treated as a path, so there is nothing to guard
                // with is_remote(); no filesystem probe happens anywhere.
                // The session reconnects with backoff on its own.
                SourceSettings::FreallyLink { host, port, key, .. } => {
                    fcap_sources::link::start_link(&id.0.to_string(), host, *port, key)
                }
                other => Err(CaptureError::Unsupported(format!(
                    "{} is not capture-backed",
                    other.kind_name()
                ))),
            };
            let _ = tx.send(result);
        });
    if spawned.is_ok() {
        starting.insert(id, rx);
    }
}

/// A Text source's style with `content` as the shown text — shared between
/// the static render and the CAP-M16 bound-file refresh (which substitutes
/// the file's extracted value for the `text` field).
fn text_style(settings: &SourceSettings, content: String) -> Option<text::TextStyle> {
    let SourceSettings::Text {
        font_family,
        font_file,
        size_px,
        color,
        align,
        line_spacing,
        force_rtl,
        wrap_width,
        ..
    } = settings
    else {
        return None;
    };
    Some(text::TextStyle {
        text: content,
        font_family: font_family.clone(),
        font_file: font_file.as_ref().map(PathBuf::from),
        size_px: *size_px,
        color: [color.r, color.g, color.b, color.a],
        align: align_of(*align),
        line_spacing: *line_spacing,
        force_rtl: *force_rtl,
        wrap_width: *wrap_width,
        ..text::TextStyle::default()
    })
}

/// Map the scene model's text alignment onto the renderer's.
fn align_of(align: fcap_scene::TextAlign) -> text::TextAlign {
    match align {
        fcap_scene::TextAlign::Left => text::TextAlign::Left,
        fcap_scene::TextAlign::Center => text::TextAlign::Center,
        fcap_scene::TextAlign::Right => text::TextAlign::Right,
    }
}

/// Map one scene-model title layer (CAP-N16) onto the generator's spec.
fn title_layer_spec(layer: &fcap_scene::TitleLayer) -> fcap_sources::title::LayerSpec {
    match layer {
        fcap_scene::TitleLayer::Text {
            x,
            y,
            text: content,
            font_family,
            font_file,
            size_px,
            color,
            align,
            outline_px,
            outline_color,
            shadow,
            source_file,
            binding,
            csv_row,
            csv_column,
            json_pointer,
        } => fcap_sources::title::LayerSpec::Text {
            x: *x,
            y: *y,
            text: content.clone(),
            font_family: font_family.clone(),
            font_file: font_file.as_ref().map(PathBuf::from),
            size_px: *size_px,
            color: [color.r, color.g, color.b, color.a],
            align: align_of(*align),
            outline_px: *outline_px,
            outline_color: [
                outline_color.r,
                outline_color.g,
                outline_color.b,
                outline_color.a,
            ],
            shadow: *shadow,
            source_file: source_file.trim().to_string(),
            binding: match binding {
                fcap_scene::FileBinding::Whole => fcap_sources::title::Binding::Whole,
                fcap_scene::FileBinding::CsvCell => fcap_sources::title::Binding::CsvCell {
                    row: *csv_row,
                    column: csv_column.clone(),
                },
                fcap_scene::FileBinding::JsonPointer => fcap_sources::title::Binding::JsonPointer {
                    pointer: json_pointer.clone(),
                },
            },
        },
        fcap_scene::TitleLayer::Image { x, y, path } => fcap_sources::title::LayerSpec::Image {
            x: *x,
            y: *y,
            path: path.trim().to_string(),
        },
        fcap_scene::TitleLayer::Rect {
            x,
            y,
            width,
            height,
            color,
        } => fcap_sources::title::LayerSpec::Rect {
            x: *x,
            y: *y,
            width: *width,
            height: *height,
            color: [color.r, color.g, color.b, color.a],
        },
    }
}

/// Whether a Text source binds to a watched file (CAP-M16) — those render
/// from the loop's poll step, not the static branch.
fn is_bound_text(settings: &SourceSettings) -> bool {
    matches!(settings, SourceSettings::Text { source_file, .. } if !source_file.trim().is_empty())
}

/// The loop-side animated state of one punch-in lens (CAP-N71).
struct LensAnim {
    zoom: f32,
    zoom_vel: f32,
    anchor: (f32, f32),
    anchor_vel: (f32, f32),
}

/// One fixed-step tick of a critically-damped spring — the deterministic
/// "smooth" behind CAP-N71's zoom/pan (no randomness; snaps once settled).
fn spring_step(current: &mut f32, velocity: &mut f32, target: f32, dt: f32) {
    const OMEGA: f32 = 10.0; // rad/s — settles in roughly 0.4 s
    let accel = -2.0 * OMEGA * *velocity - OMEGA * OMEGA * (*current - target);
    *velocity += accel * dt;
    *current += *velocity * dt;
    if (*current - target).abs() < 1e-3 && velocity.abs() < 1e-3 {
        *current = target;
        *velocity = 0.0;
    }
}

/// The cursor's position inside `item`'s display/window capture as a
/// normalized content anchor. `None` for non-capture items, off-Windows, or
/// while the cursor is outside the captured surface — the caller then holds
/// the lens's previous anchor instead of snapping around.
fn follow_anchor(
    scene: &fcap_scene::Scene,
    sources: &[fcap_scene::Source],
    item_id: ItemId,
) -> Option<(f32, f32)> {
    let item = scene.item(item_id)?;
    let source = sources.iter().find(|source| source.id == item.source)?;
    let capture_id = match &source.settings {
        SourceSettings::Display { capture_id, .. } | SourceSettings::Window { capture_id, .. } => {
            capture_id
        }
        _ => return None,
    };
    let (cx, cy) = fcap_capture::cursor_screen_position()?;
    let (rx, ry, rw, rh) = fcap_capture::source_screen_rect(capture_id)?;
    if rw == 0 || rh == 0 {
        return None;
    }
    let ax = (cx - rx) as f32 / rw as f32;
    let ay = (cy - ry) as f32 / rh as f32;
    if !(0.0..=1.0).contains(&ax) || !(0.0..=1.0).contains(&ay) {
        return None;
    }
    Some((ax, ay))
}

/// Validate an auto-crop target and return its source id (CAP-N72): the
/// item must exist, must not be the backdrop (its layout is engine-owned),
/// and must be a video kind.
fn resolve_autocrop_target(
    collection: &Collection,
    scene: SceneId,
    item: ItemId,
) -> Result<SourceId, String> {
    let scene_ref = collection
        .scene(scene)
        .ok_or_else(|| "scene not found".to_string())?;
    let item_ref = scene_ref
        .item(item)
        .ok_or_else(|| "item not found".to_string())?;
    if item_ref.backdrop.is_some() {
        return Err("the backdrop lays itself out; auto-crop applies to ordinary items".into());
    }
    let source = collection
        .source(item_ref.source)
        .ok_or_else(|| "source not found".to_string())?;
    if source.settings.is_audio_only() {
        return Err("auto-crop needs a video source".into());
    }
    Ok(source.id)
}

/// CAP-N72, loop-side: a frame just arrived for `source` — run black-bar
/// detection when an auto-crop entry wants it (one-shot `pending`, or an
/// armed `follow` whose resolution changed). The one-click apply is an
/// undoable edit; follow re-applies go through the untracked path so a
/// resolution flap never floods the undo stack.
fn run_autocrop<R: Runtime>(
    app: &AppHandle<R>,
    frame: &fcap_capture::Frame,
    source: SourceId,
    autocrop: &HashMap<ItemId, AutocropState>,
    dims_seen: &mut HashMap<ItemId, (u32, u32)>,
) {
    for (item_id, request) in autocrop.iter().filter(|(_, req)| req.source == source) {
        let dims = (frame.width, frame.height);
        let dims_changed = dims_seen.get(item_id) != Some(&dims);
        if !(request.pending || (request.follow && dims_changed)) {
            continue;
        }
        dims_seen.insert(*item_id, dims);
        let bars = fcap_sources::blackbar::detect_bars(frame);
        let crop = Crop {
            left: bars.left,
            top: bars.top,
            right: bars.right,
            bottom: bars.bottom,
        };
        let studio = app.state::<StudioState>();
        let scene_id = request.scene;
        let item = *item_id;
        let apply = move |collection: &mut Collection| {
            let current = collection
                .scene(scene_id)
                .and_then(|scene| scene.item(item))
                .map(|entry| entry.transform)
                .ok_or(SceneError::ItemNotFound)?;
            if current.crop == crop {
                return Ok(()); // unchanged: no edit, no event churn
            }
            collection.set_item_transform(scene_id, item, Transform { crop, ..current })
        };
        let result = if request.pending {
            studio.mutate_tracked(app, "autoCrop", None, apply)
        } else {
            studio.mutate(app, apply)
        };
        match result {
            Ok(()) => studio.autocrop_settle(item, false),
            Err(_) => studio.autocrop_settle(item, true),
        }
    }
}

/// CAP-N02: substitute studio variables into a Text source's content. Every
/// other kind passes through untouched (cheaply — no clone when there is
/// nothing to interpolate).
fn interpolate_settings<R: Runtime>(
    app: &AppHandle<R>,
    settings: &SourceSettings,
) -> SourceSettings {
    let SourceSettings::Text { text, .. } = settings else {
        return settings.clone();
    };
    if !text.contains("{{") {
        return settings.clone();
    }
    let expanded = app
        .state::<crate::automation::AutomationState>()
        .interpolate(text);
    let mut next = settings.clone();
    if let SourceSettings::Text { text, .. } = &mut next {
        *text = expanded;
    }
    next
}

/// Render a static source (image / color / text) to its frame.
fn render_static(settings: &SourceSettings) -> Result<fcap_capture::Frame, String> {
    match settings {
        SourceSettings::Image { path } => {
            image::load_image_rgba(std::path::Path::new(path)).map_err(|err| err.to_string())
        }
        SourceSettings::Color {
            color: rgba,
            width,
            height,
        } => color::solid_color_frame([rgba.r, rgba.g, rgba.b, rgba.a], *width, *height)
            .map_err(|err| err.to_string()),
        SourceSettings::Text { text: content, .. } => {
            let style =
                text_style(settings, content.clone()).expect("a Text kind always yields a style");
            text::render_text(&style).map_err(|err| err.to_string())
        }
        SourceSettings::TestBars { width, height } => {
            fcap_sources::testsignal::smpte_bars_frame(*width, *height)
                .map_err(|err| err.to_string())
        }
        SourceSettings::TestGrid { width, height } => {
            fcap_sources::testsignal::grid_frame(*width, *height).map_err(|err| err.to_string())
        }
        // V1-D: the social & channels bar. A purely static, audio-free face —
        // bundled platforms map to their brand colour + name; a Custom row
        // supplies its own; blank-handle rows are skipped here so the painter
        // never draws an empty badge.
        SourceSettings::SocialBar {
            header,
            rows,
            font_family,
            size_px,
            color,
            background,
        } => {
            let rows = rows
                .iter()
                .filter(|row| !row.handle.trim().is_empty())
                .map(|row| {
                    let (label, badge) = match row.platform {
                        fcap_scene::SocialPlatform::Custom => (row.label.clone(), row.color),
                        other => (other.display_name().to_string(), other.brand_color()),
                    };
                    let handle = row.handle.trim();
                    let label = label.trim();
                    let text = if label.is_empty() {
                        handle.to_string()
                    } else {
                        format!("{label}  {handle}")
                    };
                    fcap_sources::socialbar::SocialBarRow {
                        badge: [badge.r, badge.g, badge.b, badge.a],
                        text,
                    }
                })
                .collect();
            fcap_sources::socialbar::render_social_bar(&fcap_sources::socialbar::SocialBarStyle {
                header: header.clone(),
                font_family: font_family.clone(),
                size_px: *size_px,
                color: [color.r, color.g, color.b, color.a],
                background: [background.r, background.g, background.b, background.a],
                rows,
            })
            .map_err(|err| err.to_string())
        }
        other => Err(format!("{} is not a static source", other.kind_name())),
    }
}

/// The file path a filter needs loaded, if any.
fn filter_file_path(kind: &FilterKind) -> Option<String> {
    match kind {
        FilterKind::Lut { path, .. } | FilterKind::Mask { path, .. } => {
            (!path.trim().is_empty()).then(|| path.clone())
        }
        _ => None,
    }
}

/// A stable hash of a bezier mask's inputs (CAP-N28) so the render loop only
/// re-rasterizes when the path, feather, or invert actually change.
fn bezier_signature(points: &[[f32; 2]], feather: f32, invert: bool) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for point in points {
        point[0].to_bits().hash(&mut hasher);
        point[1].to_bits().hash(&mut hasher);
    }
    feather.to_bits().hash(&mut hasher);
    invert.hash(&mut hasher);
    hasher.finish()
}

/// Load + decode a filter's file into compositor-ready data.
fn load_filter_resource(kind: &FilterKind, path: &str) -> Result<FilterResourceData, String> {
    match kind {
        FilterKind::Lut { .. } => {
            let text = std::fs::read_to_string(path)
                .map_err(|err| format!("could not read {path}: {err}"))?;
            let lut = parse_cube(&text).map_err(|err: CompositorError| err.to_string())?;
            Ok(FilterResourceData::Lut3d(lut))
        }
        FilterKind::Mask { .. } => {
            let frame = image::load_image_rgba(std::path::Path::new(path))
                .map_err(|err| err.to_string())?;
            Ok(FilterResourceData::Image {
                width: frame.width,
                height: frame.height,
                rgba: frame.data,
            })
        }
        other => Err(format!("{} needs no file", other.type_name())),
    }
}

/// The passthrough monitor's encode budget (CAP-N69): full-ish resolution
/// but bounded, and a lighter JPEG than the program preview — every
/// millisecond here lands in the measured latency.
const PASSTHROUGH_MAX_EDGE: u32 = 1920;
const PASSTHROUGH_JPEG_QUALITY: u8 = 70;

/// JPEG-encode a RAW capture frame (CAP-N69's passthrough path): capture
/// frames are strided and may be BGRA, so they are repacked tight-RGBA
/// first — no compositor, no filters, no GPU round-trip.
fn encode_capture_jpeg(frame: &fcap_capture::Frame) -> Option<Vec<u8>> {
    let (width, height) = (frame.width, frame.height);
    if width == 0 || height == 0 || frame.stride < width * 4 {
        return None;
    }
    let needed = frame.stride as usize * height as usize;
    if frame.data.len() < needed {
        return None;
    }
    let bgra = matches!(frame.format, fcap_capture::PixelFormat::Bgra8);
    let mut tight = vec![0u8; width as usize * height as usize * 4];
    for y in 0..height as usize {
        let src = &frame.data[y * frame.stride as usize..][..width as usize * 4];
        let dst = &mut tight[y * width as usize * 4..][..width as usize * 4];
        if bgra {
            for x in 0..width as usize {
                let px = &src[x * 4..x * 4 + 4];
                let out = &mut dst[x * 4..x * 4 + 4];
                out[0] = px[2];
                out[1] = px[1];
                out[2] = px[0];
                out[3] = 255;
            }
        } else {
            dst.copy_from_slice(src);
        }
    }
    encode_program_jpeg(
        width,
        height,
        &tight,
        PASSTHROUGH_MAX_EDGE,
        PASSTHROUGH_MAX_EDGE,
        PASSTHROUGH_JPEG_QUALITY,
    )
}

/// Downscale (integer nearest-neighbor) + JPEG-encode the program frame.
/// `max_w/max_h` = the frame's own size means factor 1 — no downscale
/// (the Freally Link sender's full-resolution path).
pub(crate) fn encode_program_jpeg(
    width: u32,
    height: u32,
    data: &[u8],
    max_w: u32,
    max_h: u32,
    quality: u8,
) -> Option<Vec<u8>> {
    if width == 0 || height == 0 {
        return None;
    }
    let factor = width.div_ceil(max_w).max(height.div_ceil(max_h)).max(1);
    let out_w = width.div_ceil(factor);
    let out_h = height.div_ceil(factor);

    let mut rgb = Vec::with_capacity(out_w as usize * out_h as usize * 3);
    for y in 0..out_h {
        let src_row = (y * factor) as usize * width as usize * 4;
        for x in 0..out_w {
            let src = src_row + (x * factor) as usize * 4;
            rgb.extend_from_slice(&data[src..src + 3]);
        }
    }

    let mut out = Vec::new();
    let encoder = jpeg_encoder::Encoder::new(&mut out, quality);
    encoder
        .encode(
            &rgb,
            out_w as u16,
            out_h as u16,
            jpeg_encoder::ColorType::Rgb,
        )
        .ok()?;
    Some(out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn spring_step_converges_and_is_deterministic() {
        let mut a = (1.0f32, 0.0f32);
        let mut b = (1.0f32, 0.0f32);
        for _ in 0..120 {
            super::spring_step(&mut a.0, &mut a.1, 2.0, 1.0 / 60.0);
            super::spring_step(&mut b.0, &mut b.1, 2.0, 1.0 / 60.0);
        }
        assert_eq!(a, b, "identical runs are bit-identical (no randomness)");
        assert!(
            (a.0 - 2.0).abs() < 1e-2,
            "settles at the target, got {}",
            a.0
        );
        // Once settled it snaps exactly and stays put.
        for _ in 0..120 {
            super::spring_step(&mut a.0, &mut a.1, 2.0, 1.0 / 60.0);
        }
        assert_eq!(a, (2.0, 0.0));
    }

    use super::*;

    #[test]
    fn program_jpeg_downscales_to_the_box() {
        let data = vec![0x60; 1920 * 1080 * 4];
        let jpeg = encode_program_jpeg(1920, 1080, &data, 1280, 720, 75).expect("encodable");
        assert_eq!(&jpeg[..2], &[0xFF, 0xD8], "JPEG magic");
    }

    #[test]
    fn capture_backed_classification_matches_the_kinds() {
        assert!(is_capture_backed(&SourceSettings::Portal {}));
        assert!(!is_capture_backed(&SourceSettings::Color {
            color: fcap_scene::Rgba::WHITE,
            width: 8,
            height: 8,
        }));
        // Test signals (CAP-M21): the animated ones are sessions, the static
        // patterns render once, the tone is the mixer's alone.
        assert!(is_capture_backed(&SourceSettings::TestSweep {
            width: 8,
            height: 8,
        }));
        assert!(is_capture_backed(&SourceSettings::TestFlashBeep {
            width: 8,
            height: 8,
        }));
        assert!(!is_capture_backed(&SourceSettings::TestBars {
            width: 8,
            height: 8,
        }));
        assert!(!is_capture_backed(&SourceSettings::TestTone {}));
        assert!(SourceSettings::TestTone {}.is_audio_only());
        // The replay playback source (CAP-N10) is a session with a strip.
        let replay = SourceSettings::ReplayPlayback {
            seconds: 15,
            speed: fcap_scene::ReplaySpeed::Half,
            hw_decode: true,
        };
        assert!(is_capture_backed(&replay));
        assert!(replay.has_audio());
        // The playlist (CAP-N17) is a session generator with a mixer strip.
        let playlist = SourceSettings::Playlist {
            items: Vec::new(),
            looping: true,
            shuffle: false,
            hold_last: true,
            hw_decode: true,
            now_playing_variable: String::new(),
            hidden_face: false,
        };
        assert!(is_capture_backed(&playlist));
        assert!(playlist.has_audio());
        assert!(!playlist.is_audio_only());
        // Freally Link (CAP-N12) is a session with a mixer strip: video
        // composites, the sender's master audio rides the source's strip.
        let link = SourceSettings::FreallyLink {
            host: "192.168.1.20".into(),
            port: 9720,
            label: "Gaming PC".into(),
            key: "gaming-pc-key".into(),
        };
        assert!(is_capture_backed(&link));
        assert!(link.has_audio());
        assert!(!link.is_audio_only());
        assert!(!link.is_screen_view());
        // The split timer (CAP-N18) is a session generator like the sweep.
        assert!(is_capture_backed(&SourceSettings::SplitTimer {
            path: "C:/runs/game.lss".into(),
            comparison: fcap_scene::SplitComparison::PersonalBest,
            width: 8,
            height: 8,
            size_px: 18.0,
            color: fcap_scene::Rgba::WHITE,
            ahead: fcap_scene::Rgba::WHITE,
            behind: fcap_scene::Rgba::WHITE,
            gold: fcap_scene::Rgba::WHITE,
        }));
        // The title designer (CAP-N16) is a session generator (its in/out
        // animation and bound cells repaint live); silent by design.
        let title = SourceSettings::Title {
            width: 8,
            height: 8,
            layers: Vec::new(),
            animation: fcap_scene::TitleAnimation::Fade,
            duration_ms: 400,
        };
        assert!(is_capture_backed(&title));
        assert!(!title.has_audio());
        assert!(!title.is_audio_only());
        // The visualizer (CAP-N15) is a session generator like the sweep.
        assert!(is_capture_backed(&SourceSettings::AudioVisualizer {
            style: fcap_scene::VisStyle::Bars,
            target: fcap_scene::VisTargetKind::Master,
            track: 1,
            source: None,
            width: 8,
            height: 8,
            bands: 8,
            color: fcap_scene::Rgba::WHITE,
            peak_hold: true,
            decay: 30.0,
            classic: false,
        }));
        // The LAN listener (CAP-N11) is session-backed with a mixer strip;
        // it composes video, so it is never audio-only.
        let lan = SourceSettings::LanIngest {
            protocol: fcap_scene::IngestProtocol::Srt,
            port: 9710,
            passphrase: String::new(),
        };
        assert!(is_capture_backed(&lan));
        assert!(lan.has_audio());
        assert!(!lan.is_audio_only());
        // The input overlay (CAP-N13) is a session generator with no audio;
        // its sampler lives and dies with the session (the privacy scope).
        let overlay = SourceSettings::InputOverlay {
            layout: fcap_scene::InputLayout::Wasd,
            color: fcap_scene::Rgba::WHITE,
            accent: fcap_scene::Rgba::WHITE,
        };
        assert!(is_capture_backed(&overlay));
        assert!(!overlay.has_audio());
        assert!(!overlay.is_audio_only());
        // The stats HUD (CAP-N14) is a text face: no session, no static
        // render — it paints from its own refresh step like the timers.
        assert!(!is_capture_backed(&SourceSettings::SystemStats {
            show_fps: true,
            show_cpu: true,
            show_memory: true,
            show_render_ms: true,
            show_dropped: true,
            show_bitrate: true,
            show_timecode: false,
            font_family: None,
            font_file: None,
            size_px: 28.0,
            color: fcap_scene::Rgba::WHITE,
        }));
        assert!(render_static(&SourceSettings::TestGrid {
            width: 8,
            height: 8,
        })
        .is_ok());
    }

    /// CAP-M16 + the security review: the bound-text poll must never stat a
    /// UNC/URL path — on Windows that forces an SMB/NTLM handshake to the
    /// host, so an untrusted collection could harvest the user's credential
    /// hash on a 500 ms timer. The poll refuses these before any filesystem
    /// call; this pins the predicate the loop gates on.
    #[test]
    fn bound_text_never_reaches_a_network_path() {
        for hostile in [
            "\\\\attacker\\share\\score.txt",
            "//attacker/share/score.txt",
            "http://attacker/score.txt",
            "smb://attacker/score.txt",
        ] {
            assert!(
                crate::commands::studio::is_remote(hostile),
                "{hostile} must be refused before any stat/read"
            );
        }
        assert!(!crate::commands::studio::is_remote("C:/data/score.txt"));
        assert!(!crate::commands::studio::is_remote("/home/mike/score.txt"));
    }

    #[test]
    fn filter_paths_only_come_from_lut_and_mask() {
        assert!(filter_file_path(&FilterKind::Blur { radius: 2.0 }).is_none());
        assert!(filter_file_path(&FilterKind::Lut {
            path: "x.cube".into(),
            amount: 1.0
        })
        .is_some());
        assert!(filter_file_path(&FilterKind::Lut {
            path: "  ".into(),
            amount: 1.0
        })
        .is_none());
    }
}
