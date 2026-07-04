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
const PREVIEW_MAX_WIDTH: u32 = 1280;
const PREVIEW_MAX_HEIGHT: u32 = 720;
const PREVIEW_JPEG_QUALITY: u8 = 75;

// ---------------------------------------------------------------------------
// Shared state + the command-side mutation surface
// ---------------------------------------------------------------------------

/// What the `studio` event / `studio_get` carry: the whole model.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioDto {
    pub revision: u64,
    pub collection: Collection,
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
}

/// Tauri-managed handle to the studio model.
pub struct StudioState {
    core: Arc<Mutex<StudioCore>>,
}

impl StudioState {
    /// Load the persisted collection (or start fresh) from the OS config dir.
    pub fn load_default() -> Self {
        let path = directories::ProjectDirs::from("com", "Freally", "Freally Capture")
            .map(|dirs| dirs.config_dir().join("scene-collection.json"));
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
        let core = self.lock();
        StudioDto {
            revision: core.revision,
            collection: core.collection.clone(),
        }
    }

    /// The current model revision — a cheap read the audio bridge polls to
    /// decide whether it must fetch the (cloning) [`Self::audio_specs`].
    pub fn audio_revision(&self) -> u64 {
        self.lock().revision
    }

    /// What the audio engine reconciles against: the revision plus the active
    /// scene's audio-capable sources (visible items only — the eye toggle
    /// silences audio exactly like it hides video), with their strips and
    /// retry nonces.
    pub fn audio_specs(&self) -> (u64, Vec<AudioSourceSpec>) {
        let core = self.lock();
        let scene = core.collection.active_scene();
        let specs = core
            .collection
            .sources
            .iter()
            .filter(|source| source.settings.has_audio())
            .filter(|source| {
                scene
                    .items
                    .iter()
                    .any(|item| item.source == source.id && item.visible)
            })
            .map(|source| AudioSourceSpec {
                id: source.id,
                settings: source.settings.clone(),
                audio: source.audio.clone().unwrap_or_default(),
                nonce: core.retry_nonces.get(&source.id).copied().unwrap_or(0),
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
            let dto = StudioDto {
                revision: core.revision,
                collection: core.collection.clone(),
            };
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
    let mut sessions: HashMap<SourceId, SessionSlot> = HashMap::new();
    let mut starting: HashMap<SourceId, mpsc::Receiver<Result<CaptureSession, CaptureError>>> =
        HashMap::new();
    let mut capture_specs: HashMap<SourceId, String> = HashMap::new();
    let mut static_specs: HashMap<SourceId, String> = HashMap::new();
    let mut filter_files: HashMap<FilterId, String> = HashMap::new();
    let mut statuses: HashMap<SourceId, SourceRuntime> = HashMap::new();

    let mut composed_this_second = 0u32;
    let mut last_readback = Instant::now() - READBACK_INTERVAL;
    let mut last_program_event = Instant::now();
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
    // macOS CAMetalLayer. Any other backend stays on the JPEG path.
    let mut native_disabled = !(compositor.is_dx12() || compositor.is_metal());
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
        let (revision, scene, scene_sources, canvas, nonces) = {
            let guard = lock_core(&core);
            (
                guard.revision,
                guard.collection.active_scene().clone(),
                guard
                    .collection
                    .sources
                    .iter()
                    .filter(|source| {
                        guard
                            .collection
                            .active_scene()
                            .items
                            .iter()
                            .any(|item| item.source == source.id)
                    })
                    .cloned()
                    .collect::<Vec<_>>(),
                (
                    guard.collection.canvas_width,
                    guard.collection.canvas_height,
                ),
                guard.retry_nonces.clone(),
            )
        };
        compositor.set_canvas_size(canvas.0, canvas.1);

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
                        start_session(source.id, &source.settings, &mut starting);
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

            // Filter files (LUT lattices, mask images).
            let mut live_filters: Vec<FilterId> = Vec::new();
            for item in &scene.items {
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
        let mut ended: Vec<(SourceId, CaptureError)> = Vec::new();
        for (id, slot) in sessions.iter_mut() {
            match slot.session.frames().recv_timeout(Duration::ZERO) {
                Ok(Some(frame)) => {
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
                Ok(None) => {}
                Err(err) => ended.push((*id, err)),
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
                    (
                        scene.id,
                        item.id,
                        fcap_compositor::transform::fit_to_canvas(w, h, canvas.0, canvas.1),
                    )
                })
            })
            .collect();
        if !fits.is_empty() {
            let dto = {
                let mut guard = lock_core(&core);
                for (scene_id, item_id, transform) in fits {
                    let _ = guard
                        .collection
                        .set_item_transform(scene_id, item_id, transform);
                }
                guard.revision += 1;
                guard.dirty_since.get_or_insert_with(Instant::now);
                StudioDto {
                    revision: guard.revision,
                    collection: guard.collection.clone(),
                }
            };
            if app.emit("studio", &dto).is_err() {
                break;
            }
        }

        // -- 6. Compose + preview ------------------------------------------------
        let time = started_at.elapsed().as_secs_f32();
        if let Err(err) = compositor.render(&scene, time) {
            eprintln!("studio: compose failed: {err}");
        }
        composed_this_second += 1;

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

        // The recorder wants the full-res program frame every tick while a
        // session runs; the preview JPEG keeps its ~30 fps cadence. One
        // readback serves both.
        let recording = app.state::<crate::recording::RecordingState>();
        let record_due = recording.wants_frames();
        let preview_due = last_readback.elapsed() >= READBACK_INTERVAL;
        if record_due || preview_due {
            match compositor.read_program() {
                Ok(frame) => {
                    let (frame_w, frame_h) = (frame.width, frame.height);
                    let data = Arc::new(frame.data);
                    if record_due {
                        recording.push_video(Arc::clone(&data));
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
            if app.emit("program", &status).is_err() {
                break; // the app is gone — wind down
            }
            composed_this_second = 0;
            last_program_event = Instant::now();
            statuses_changed = false;
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
) {
    let (tx, rx) = mpsc::channel();
    let settings = settings.clone();
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
