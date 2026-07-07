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

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime};

use fcap_capture::{CaptureError, CaptureSession};
use fcap_compositor::{parse_cube, Compositor, CompositorError, FilterResourceData};
use fcap_scene::{
    AudioSettings, Collection, FilterId, FilterKind, SceneError, SceneId, SourceId, SourceSettings,
    Transform,
};
use fcap_sources::video_device::{self, VideoFormatInfo};
use fcap_sources::{color, image, text};

use crate::preview::PreviewState;
use crate::settings::write_atomic;

const TICK: Duration = Duration::from_millis(16);
const READBACK_INTERVAL: Duration = Duration::from_millis(33);
const PROGRAM_EVENT_INTERVAL: Duration = Duration::from_secs(1);
const AUTOSAVE_DEBOUNCE: Duration = Duration::from_millis(800);
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
/// The system emoji face reaction sprites rasterize from (monochrome
/// outlines tinted per emoji — we own no color-emoji rasterizer, honestly).
#[cfg(target_os = "windows")]
const EMOJI_FONT: &str = "Segoe UI Emoji";
#[cfg(target_os = "macos")]
const EMOJI_FONT: &str = "Apple Color Emoji";
#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
const EMOJI_FONT: &str = "Noto Color Emoji";

// ---------------------------------------------------------------------------
// Shared state + the command-side mutation surface
// ---------------------------------------------------------------------------

/// What the `studio` event / `studio_get` carry: the whole model.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioDto {
    pub revision: u64,
    pub collection: Collection,
    /// Studio Mode (Phase 5): present while enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_mode: Option<StudioModeDto>,
}

/// The Studio-Mode slice of the model (session state, never persisted).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioModeDto {
    pub preview_scene: SceneId,
    /// A Preview→Program blend is running right now.
    pub transitioning: bool,
}

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
    /// Studio Mode (Phase 5): the preview-side scene while enabled.
    preview_scene: Option<SceneId>,
    /// The blend a commit is currently rendering, if any.
    transition: Option<ActiveTransition>,
}

/// The DTO for the current core state (one shape for every emit site).
fn dto_of(core: &StudioCore) -> StudioDto {
    StudioDto {
        revision: core.revision,
        collection: core.collection.clone(),
        studio_mode: core.preview_scene.map(|preview_scene| StudioModeDto {
            preview_scene,
            transitioning: core.transition.is_some(),
        }),
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
        let config_dir = directories::ProjectDirs::from("com", "Freally", "Freally Capture")
            .map(|dirs| dirs.config_dir().to_path_buf());
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
                preview_scene: None,
                transition: None,
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

    /// A snapshot for `studio_get` / event payloads.
    pub fn snapshot(&self) -> StudioDto {
        dto_of(&self.lock())
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
            // The stinger's own audio is not mixed (v1) — video only.
            Some(
                fcap_sources::media::start_media("stinger-transition", path, false, true)
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
                });
            }
            core.revision += 1;
            core.dirty_since.get_or_insert_with(Instant::now);
            dto_of(&core)
        };
        let _ = app.emit("studio", &dto);
        Ok(())
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

    /// Persist immediately if dirty (exit path — never lose the last edit).
    pub fn save_now(&self) {
        let mut core = self.lock();
        if core.dirty_since.is_some() {
            persist(&mut core);
        }
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

fn read_collection(path: &std::path::Path) -> Collection {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl SourceRuntime {
    fn waiting() -> Self {
        SourceRuntime {
            state: "waiting",
            width: None,
            height: None,
            fps: None,
            error_code: None,
            error_message: None,
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

/// Everything the loop tracks per capture-backed source.
struct SessionSlot {
    session: CaptureSession,
    frames_this_second: u32,
    live_size: Option<(u32, u32)>,
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
    let reactions_queue = app
        .state::<crate::reactions::ReactionState>()
        .queue_handle();
    let mut last_readback = Instant::now() - READBACK_INTERVAL;
    let mut last_program_event = Instant::now();
    // Whether the Studio-Mode preview pane currently has a published frame
    // (so turning the mode off clears the slot exactly once).
    let mut studio_preview_live = false;
    // Per-source auto-recover backoff: source id → (next attempt time, last wait).
    let mut retry_schedule: HashMap<SourceId, (Instant, Duration)> = HashMap::new();
    let mut statuses_changed = true;

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

        // -- 1. Snapshot the model (brief lock) --------------------------------
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
            (
                guard.revision,
                guard.collection.active_scene().clone(),
                guard
                    .collection
                    .sources
                    .iter()
                    .filter(|source| live_sources.contains(&source.id))
                    .cloned()
                    .collect::<Vec<_>>(),
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
            )
        };
        if let Some(dto) = transition_ended {
            let _ = app.emit("studio", &dto);
        }
        compositor.set_canvas_size(canvas.0, canvas.1);
        // Hand the compositor a fresh nested-scene pool only when the model
        // changed — never a per-frame deep clone.
        if let Some(pool) = scene_pool_rebuild {
            cached_scene_pool = pool;
            pool_revision = revision;
            compositor.set_scene_pool(cached_scene_pool.clone(), scene_refs);
        }
        let scene_pool = &cached_scene_pool;

        // -- 2. Reconcile sources against the active scene ---------------------
        if revision != seen_revision {
            seen_revision = revision;

            // Stop sessions whose source left the scene or changed settings.
            let mut keep_ids: Vec<SourceId> = Vec::new();
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
                let spec = format!("{}#{nonce}", source_spec(&source.settings));
                let is_capture = is_capture_backed(&source.settings);
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
                        start_session(source.id, &source.settings, &mut starting, &reactions_queue);
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
                        let status = match render_static(&source.settings) {
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
                statuses_changed = true;
            }
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
        {
            let mut scan = |items: &[fcap_scene::SceneItem]| {
                for item in items {
                    for filter in item.filters.iter().filter(|filter| filter.enabled) {
                        if let fcap_scene::FilterKind::RenderDelay { delay_ms } = &filter.kind {
                            let entry = delay_by_source.entry(item.source).or_insert(0);
                            *entry = (*entry).max((*delay_ms).min(500));
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
        let mut ended: Vec<(SourceId, CaptureError)> = Vec::new();
        for (id, slot) in sessions.iter_mut() {
            let delay_ms = delay_by_source.get(id).copied().unwrap_or(0);
            match slot.session.frames().recv_timeout(Duration::ZERO) {
                Ok(Some(frame)) => {
                    if delay_ms == 0 && slot.delayed.is_empty() {
                        let size = (frame.width, frame.height);
                        match compositor.upload_frame(*id, &frame) {
                            Ok(()) => {
                                slot.frames_this_second += 1;
                                if slot.live_size != Some(size) {
                                    slot.live_size = Some(size);
                                    statuses.insert(*id, SourceRuntime::live(size.0, size.1));
                                    statuses_changed = true;
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
                    match compositor.upload_frame(*id, &frame) {
                        Ok(()) => {
                            slot.frames_this_second += 1;
                            if slot.live_size != Some(size) {
                                slot.live_size = Some(size);
                                statuses.insert(*id, SourceRuntime::live(size.0, size.1));
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
            transition_was_active = false;
        }
        let compose_result = match &transition_pack {
            Some(pack) if pack.kind == fcap_scene::TransitionKind::Stinger => {
                // The audience sees the outgoing scene until the cut point,
                // the new program after — the stinger video covers the swap.
                let shown = if pack.progress < pack.cut {
                    &pack.from_scene
                } else {
                    &scene
                };
                compositor.render_scene_with_stinger(shown, time, pack.stinger_frame.as_ref())
            }
            Some(pack) => compositor.render_transition(
                &pack.from_scene,
                &scene,
                pack.kind,
                pack.progress,
                time,
            ),
            None => compositor.render(&scene, time),
        };
        if let Err(err) = compose_result {
            eprintln!("studio: compose failed: {err}");
        }
        composed_this_second += 1;

        // -- 6r. Floating reactions (TASK-614): baked INTO the program ----------
        // Drawn after the compose (and any transition/stinger), before the
        // native present + readback — so preview, recording, and stream all
        // carry the exact same floating emoji.
        {
            let pending = app.state::<crate::reactions::ReactionState>().drain();
            crate::reactions::spawn(&mut reaction_particles, pending, time, &mut reaction_rng);
            let draws = crate::reactions::step(&mut reaction_particles, time, canvas);
            if !draws.is_empty() {
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
                                if let Err(err) =
                                    compositor.set_reaction_sprite(&draw.sprite, &frame)
                                {
                                    eprintln!("studio: reaction sprite upload failed: {err}");
                                }
                            }
                            Err(err) => {
                                eprintln!("studio: reaction sprite render failed: {err}")
                            }
                        }
                    }
                }
                if let Err(err) = compositor.render_reactions(&draws) {
                    eprintln!("studio: reactions pass failed: {err}");
                }
            }
        }

        // -- 6a. Native preview surface (no readback — the "OBS feel") -----------
        if !native_disabled {
            let native = app.state::<crate::native_preview::NativePreviewState>();
            if let Some(handle) = native.composition_handle() {
                let (gen, bounds) = native.region();
                let sized = bounds.width > 0 && bounds.height > 0;
                if sized && native.is_visible() {
                    match &mut native_surface {
                        None => match handle
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
                            }) {
                            Ok(surface) => {
                                println!(
                                    "native preview: surface created {}x{} at ({},{})",
                                    bounds.width, bounds.height, bounds.x, bounds.y
                                );
                                native_surface = Some((surface, gen));
                                // wgpu just bound the swapchain to the visual;
                                // commit so DComp composites it immediately, not
                                // only after the next resize.
                                native.commit();
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
                    let overlay =
                        native_selection_overlay(native.selection(), &scene, &compositor, canvas);
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
        // preview JPEG keeps its ~30 fps cadence. One readback serves all.
        let recording = app.state::<crate::recording::RecordingState>();
        let streaming = app.state::<crate::stream::StreamBridgeState>();
        let replaying = app.state::<crate::replay::ReplayState>();
        let record_due = recording.wants_frames();
        let stream_due = streaming.wants_frames();
        let replay_due = replaying.wants_frames();
        let preview_due = last_readback.elapsed() >= READBACK_INTERVAL;
        if record_due || stream_due || replay_due || preview_due {
            match compositor.read_program() {
                Ok(frame) => {
                    let (frame_w, frame_h) = (frame.width, frame.height);
                    let data = Arc::new(frame.data);
                    if record_due {
                        recording.push_video(Arc::clone(&data));
                    }
                    if stream_due {
                        streaming.push_video(Arc::clone(&data));
                    }
                    if replay_due {
                        replaying.push_video(Arc::clone(&data));
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
                if vertical_record || vertical_stream || preview_due {
                    match compositor
                        .render_vertical(vertical_scene, started_at.elapsed().as_secs_f32())
                    {
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
            }
            None if vertical_preview_live => {
                preview.publish_vertical_preview(None);
                vertical_preview_live = false;
            }
            None => {}
        }

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
                    slot.frames_this_second = 0;
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
            // Hand the render numbers to the stats dock's emitter.
            app.state::<crate::events::RuntimeStats>().publish(
                status.fps,
                (composed_vertical_this_second as f32 / elapsed).round() as u32,
                status.dropped,
                status.render_micros,
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

/// The native preview's selection overlay, computed from the model: the
/// selected item's four content corners (canvas px) + its locked flag, or
/// `None` when nothing usable is selected (no selection, awaiting first frame,
/// no known size, or fully cropped). Preview-only chrome — this never touches
/// the program frame the recorder and stream read.
fn native_selection_overlay(
    selection: Option<fcap_scene::ItemId>,
    scene: &fcap_scene::Scene,
    compositor: &Compositor,
    canvas: (u32, u32),
) -> Option<fcap_compositor::PreviewOverlay> {
    let id = selection?;
    let item = scene.items.iter().find(|it| it.id == id)?;
    if item.pending_fit {
        return None;
    }
    let (source_w, source_h) = compositor.source_size(item.source)?;
    let (eff_w, eff_h) = effective_source_size(source_w, source_h, &item.filters);
    let content = fcap_compositor::transform::content_size(eff_w, eff_h, &item.transform.crop)?;
    Some(fcap_compositor::PreviewOverlay {
        corners: fcap_compositor::transform::corners(&item.transform, content),
        canvas: (canvas.0 as f32, canvas.1 as f32),
        locked: item.locked,
    })
}

/// The source size the compositor actually composes for an item: the reported
/// resolution after its enabled Crop *filters* (the only size-changing filter
/// kind), folded in chain order with the engine's skip semantics. Mirrors
/// `effectiveSourceSize` in `ui/src/lib/transform.ts` so the native selection
/// box hugs the same pixels the HTML one does.
fn effective_source_size(
    source_w: u32,
    source_h: u32,
    filters: &[fcap_scene::Filter],
) -> (u32, u32) {
    let mut w = source_w;
    let mut h = source_h;
    for filter in filters {
        if !filter.enabled {
            continue;
        }
        if let FilterKind::Crop {
            left,
            top,
            right,
            bottom,
        } = filter.kind
        {
            if left == 0 && top == 0 && right == 0 && bottom == 0 {
                continue;
            }
            let out_w = w.saturating_sub(left).saturating_sub(right);
            let out_h = h.saturating_sub(top).saturating_sub(bottom);
            if out_w == 0 || out_h == 0 {
                continue; // the engine skips a crop that would zero an axis
            }
            w = out_w;
            h = out_h;
        }
    }
    (w, h)
}

/// A change-detection fingerprint of a source's settings.
fn source_spec(settings: &SourceSettings) -> String {
    serde_json::to_string(settings).expect("source settings always serialize")
}

/// Kick off a capture session on a helper thread (portal starts block on the
/// system dialog; nothing may stall the render loop).
fn start_session(
    id: SourceId,
    settings: &SourceSettings,
    starting: &mut HashMap<SourceId, mpsc::Receiver<Result<CaptureSession, CaptureError>>>,
    reactions_queue: &Arc<Mutex<Vec<String>>>,
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
                SourceSettings::VideoDevice { device_id, format } => {
                    let format = format.as_ref().map(|f| VideoFormatInfo {
                        width: f.width,
                        height: f.height,
                        fps: f.fps,
                        fourcc: f.fourcc.clone(),
                    });
                    video_device::start_video_device(device_id, format.as_ref())
                }
                SourceSettings::Media {
                    path,
                    looping,
                    hw_decode,
                } => {
                    // The source id keys the mixer-side audio ring.
                    fcap_sources::media::start_media(&id.0.to_string(), path, *looping, *hw_decode)
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
        SourceSettings::Text {
            text: content,
            font_family,
            font_file,
            size_px,
            color,
            align,
            line_spacing,
            force_rtl,
            wrap_width,
        } => {
            let style = text::TextStyle {
                text: content.clone(),
                font_family: font_family.clone(),
                font_file: font_file.as_ref().map(PathBuf::from),
                size_px: *size_px,
                color: [color.r, color.g, color.b, color.a],
                align: match align {
                    fcap_scene::TextAlign::Left => text::TextAlign::Left,
                    fcap_scene::TextAlign::Center => text::TextAlign::Center,
                    fcap_scene::TextAlign::Right => text::TextAlign::Right,
                },
                line_spacing: *line_spacing,
                force_rtl: *force_rtl,
                wrap_width: *wrap_width,
            };
            text::render_text(&style).map_err(|err| err.to_string())
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

/// Downscale (integer nearest-neighbor) + JPEG-encode the program frame.
fn encode_program_jpeg(
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
