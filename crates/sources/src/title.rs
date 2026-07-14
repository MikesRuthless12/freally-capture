//! CAP-N16 — the **title & scoreboard designer** source: layered templates
//! (text / image / solid-box layers on a fixed canvas) with an
//! animate-in/out pass (fade / slide / wipe), CAP-M16 file bindings and
//! CAP-N02 `{{variable}}` interpolation per text cell, and live control —
//! fire the animation, push new cell text — without restarting the session.
//!
//! Honest scope notes:
//! - Fully local and CPU-composed: the layer stack rasters once per
//!   *content* change, animation frames are emitted only while the in/out
//!   pass runs, and a steady title sends nothing per tick. No browser
//!   source, no web service.
//! - Image layers load once at session start; edit-and-Apply reloads them.
//! - Bound files follow the CAP-M16 rules: local files only (the studio
//!   refuses network paths before the session starts, and this module
//!   re-checks before every stat — a UNC stat is an NTLM handshake),
//!   500 ms poll, fingerprint gate, the 256 KiB cap, and the last good
//!   value held through atomic-rename write gaps.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, Weak};
use std::time::{Duration, Instant};

use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame};

use crate::compose::blit;
use crate::image::load_image_rgba;
use crate::static_source::{check_dimension, rgba_frame};
use crate::text::{render_text, TextAlign, TextStyle};
use crate::textfile;

/// The generator's frame cadence while an animation runs.
const FPS: u32 = 30;
/// Bound-file poll cadence (mirrors the studio's CAP-M16 bound-text poll).
const BOUND_POLL: Duration = Duration::from_millis(500);
/// A live SetLayerText override is bounded like an automation variable.
const MAX_OVERRIDE_CHARS: usize = 512;
/// An in/out pass longer than this is a configuration mistake, not a look.
const MAX_DURATION_MS: u32 = 10_000;

// ---------------------------------------------------------------------------
// Config (the studio maps the scene model's TitleLayer / TitleAnimation here)
// ---------------------------------------------------------------------------

/// How a text layer's bound file parses (mirrors the scene's `FileBinding`
/// plus its selector fields).
#[derive(Debug, Clone, PartialEq)]
pub enum Binding {
    Whole,
    CsvCell { row: u32, column: String },
    JsonPointer { pointer: String },
}

/// One layer, drawn in list order (later layers on top).
#[derive(Debug, Clone)]
pub enum LayerSpec {
    Text {
        x: i32,
        y: i32,
        text: String,
        font_family: Option<String>,
        font_file: Option<PathBuf>,
        size_px: f32,
        color: [u8; 4],
        align: TextAlign,
        outline_px: f32,
        outline_color: [u8; 4],
        shadow: bool,
        /// CAP-M16: the watched file (`""` = the `text` field).
        source_file: String,
        binding: Binding,
    },
    Image {
        x: i32,
        y: i32,
        path: String,
    },
    Rect {
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        color: [u8; 4],
    },
}

/// The in/out animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Animation {
    None,
    Fade,
    SlideLeft,
    SlideUp,
    Wipe,
}

#[derive(Debug, Clone)]
pub struct TitleConfig {
    pub width: u32,
    pub height: u32,
    pub layers: Vec<LayerSpec>,
    pub animation: Animation,
    pub duration_ms: u32,
}

// ---------------------------------------------------------------------------
// Variables (CAP-N02) — the studio loop feeds these at 1 Hz on change
// ---------------------------------------------------------------------------

fn variables() -> &'static Mutex<HashMap<String, String>> {
    static VARS: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
    VARS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Replace the variable map (CAP-N02). The studio loop hands the engine's
/// (bounded) map here whenever its revision moves; every running title
/// interpolates the new values on its next tick — no restart, no spec change.
pub fn set_variables(vars: HashMap<String, String>) {
    *lock(variables()) = vars;
}

/// Substitute `{{name}}` tokens; unknown names stay verbatim (a typo shows
/// itself instead of vanishing) — the automation engine's exact rule.
fn interpolate(text: &str) -> String {
    if !text.contains("{{") {
        return text.to_owned();
    }
    let vars = lock(variables());
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("{{") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let Some(end) = after.find("}}") else {
            out.push_str(&rest[start..]);
            return out;
        };
        let name = after[..end].trim();
        match vars.get(name) {
            Some(value) => out.push_str(value),
            None => {
                out.push_str("{{");
                out.push_str(&after[..end]);
                out.push_str("}}");
            }
        }
        rest = &after[end + 2..];
    }
    out.push_str(rest);
    out
}

// ---------------------------------------------------------------------------
// Live control: the session registry (splits.rs shape)
// ---------------------------------------------------------------------------

/// Where the title sits in its in/out lifecycle.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Phase {
    In { started: Instant },
    Shown,
    Out { started: Instant },
    Hidden,
}

/// What a command asks of a live title.
#[derive(Debug, Clone, PartialEq)]
pub enum TitleAction {
    FireIn,
    FireOut,
    /// Push live text into the layer at this index (runtime-only — Apply
    /// rebuilds the session from the model and clears these overrides).
    SetLayerText(usize, String),
}

/// One live title's shared state — the generator renders it, the properties
/// dialog mutates it.
pub struct TitleSession {
    phase: Phase,
    /// Live text overrides keyed by layer index.
    overrides: HashMap<usize, String>,
}

impl TitleSession {
    fn new() -> Self {
        Self {
            // A title entering the scene fires its In pass — the broadcast
            // expectation ("animate-in"), and what the DoD golden pins.
            phase: Phase::In {
                started: Instant::now(),
            },
            overrides: HashMap::new(),
        }
    }

    fn control(&mut self, action: TitleAction) {
        match action {
            TitleAction::FireIn => {
                self.phase = Phase::In {
                    started: Instant::now(),
                }
            }
            TitleAction::FireOut => {
                self.phase = Phase::Out {
                    started: Instant::now(),
                }
            }
            TitleAction::SetLayerText(index, value) => {
                self.overrides
                    .insert(index, value.chars().take(MAX_OVERRIDE_CHARS).collect());
            }
        }
    }

    /// Animation progress right now (1 = fully shown, 0 = hidden); settles a
    /// finished In/Out into its steady phase.
    fn progress(&mut self, duration: Duration) -> f32 {
        match self.phase {
            Phase::Shown => 1.0,
            Phase::Hidden => 0.0,
            Phase::In { started } => {
                if duration.is_zero() || started.elapsed() >= duration {
                    self.phase = Phase::Shown;
                    1.0
                } else {
                    started.elapsed().as_secs_f32() / duration.as_secs_f32()
                }
            }
            Phase::Out { started } => {
                if duration.is_zero() || started.elapsed() >= duration {
                    self.phase = Phase::Hidden;
                    0.0
                } else {
                    1.0 - started.elapsed().as_secs_f32() / duration.as_secs_f32()
                }
            }
        }
    }

    fn animating(&self) -> bool {
        matches!(self.phase, Phase::In { .. } | Phase::Out { .. })
    }
}

fn registry() -> &'static Mutex<HashMap<String, Weak<Mutex<TitleSession>>>> {
    static REG: OnceLock<Mutex<HashMap<String, Weak<Mutex<TitleSession>>>>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Drive one live title (the properties dialog). `false` = not running.
pub fn control(id: &str, action: TitleAction) -> bool {
    let live = lock(registry()).get(id).and_then(Weak::upgrade);
    match live {
        Some(session) => {
            lock(&session).control(action);
            true
        }
        None => false,
    }
}

// ---------------------------------------------------------------------------
// Animation math (pure — the tests drive these directly)
// ---------------------------------------------------------------------------

/// Where the animation places the content at progress `t` (0 = hidden,
/// 1 = fully shown): a pixel offset, an alpha multiplier, and how much of
/// the canvas the wipe has revealed (left → right).
#[derive(Debug, Clone, PartialEq)]
pub struct Pose {
    pub dx: i32,
    pub dy: i32,
    pub alpha: f32,
    pub reveal: f32,
}

pub fn pose(animation: Animation, t: f32, width: u32, height: u32) -> Pose {
    let t = t.clamp(0.0, 1.0);
    match animation {
        Animation::None => Pose {
            dx: 0,
            dy: 0,
            alpha: if t > 0.0 { 1.0 } else { 0.0 },
            reveal: 1.0,
        },
        Animation::Fade => Pose {
            dx: 0,
            dy: 0,
            alpha: t,
            reveal: 1.0,
        },
        Animation::SlideLeft => Pose {
            // Enters from the right edge moving left; exits back right.
            dx: ((1.0 - t) * width as f32).round() as i32,
            dy: 0,
            alpha: 1.0,
            reveal: 1.0,
        },
        Animation::SlideUp => Pose {
            // Enters from the bottom edge moving up; exits back down.
            dx: 0,
            dy: ((1.0 - t) * height as f32).round() as i32,
            alpha: 1.0,
            reveal: 1.0,
        },
        Animation::Wipe => Pose {
            dx: 0,
            dy: 0,
            alpha: 1.0,
            reveal: t,
        },
    }
}

/// Apply a pose to the steady buffer: shift the content by `(dx, dy)`,
/// scale every alpha, and clear everything right of the wipe's reveal edge.
pub fn apply_pose(steady: &[u8], width: u32, height: u32, pose: &Pose) -> Vec<u8> {
    let (w, h) = (width as i64, height as i64);
    let mut out = vec![0u8; steady.len()];
    let reveal_px = (pose.reveal.clamp(0.0, 1.0) * width as f32).round() as i64;
    let alpha = (pose.alpha.clamp(0.0, 1.0) * 255.0).round() as u32;
    if alpha == 0 || reveal_px == 0 {
        return out;
    }
    for y in 0..h {
        let src_y = y - pose.dy as i64;
        if src_y < 0 || src_y >= h {
            continue;
        }
        for x in 0..reveal_px.min(w) {
            let src_x = x - pose.dx as i64;
            if src_x < 0 || src_x >= w {
                continue;
            }
            let src = ((src_y * w + src_x) * 4) as usize;
            let dst = ((y * w + x) * 4) as usize;
            out[dst..dst + 3].copy_from_slice(&steady[src..src + 3]);
            out[dst + 3] = ((steady[src + 3] as u32 * alpha) / 255) as u8;
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Layer composition
// ---------------------------------------------------------------------------

/// Compose the layer stack into a straight-alpha RGBA buffer. `texts` holds
/// each text layer's RESOLVED content (override / bound value / spec text,
/// variables already interpolated), indexed like `layers`; `images` the
/// pre-loaded image layers. Pure given those inputs.
pub fn compose_layers(config: &TitleConfig, texts: &[String], images: &[Option<Frame>]) -> Vec<u8> {
    let (w, h) = (config.width as usize, config.height as usize);
    let mut face = vec![0u8; w * h * 4];
    for (index, layer) in config.layers.iter().enumerate() {
        match layer {
            LayerSpec::Rect {
                x,
                y,
                width,
                height,
                color,
            } => {
                // Clamp to the visible intersection, then reuse the shared
                // blit — clipping and alpha-over in one code path (a
                // translucent bar must blend over the layers below it).
                let x0 = (*x).max(0).min(w as i32) as usize;
                let y0 = (*y).max(0).min(h as i32) as usize;
                let x1 = (x.saturating_add(*width as i32)).max(0).min(w as i32) as usize;
                let y1 = (y.saturating_add(*height as i32)).max(0).min(h as i32) as usize;
                if x1 <= x0 || y1 <= y0 {
                    continue;
                }
                let solid = rgba_frame(
                    (x1 - x0) as u32,
                    (y1 - y0) as u32,
                    color.as_slice().repeat((x1 - x0) * (y1 - y0)),
                );
                blit(&mut face, w, h, &solid, x0 as i64, y0 as i64);
            }
            LayerSpec::Image { x, y, .. } => {
                if let Some(Some(frame)) = images.get(index) {
                    blit(&mut face, w, h, frame, i64::from(*x), i64::from(*y));
                }
            }
            LayerSpec::Text {
                x,
                y,
                font_family,
                font_file,
                size_px,
                color,
                align,
                outline_px,
                outline_color,
                shadow,
                ..
            } => {
                let content = texts.get(index).map(String::as_str).unwrap_or("");
                if content.trim().is_empty() {
                    continue;
                }
                let style = TextStyle {
                    text: content.to_string(),
                    font_family: font_family.clone(),
                    font_file: font_file.clone(),
                    size_px: *size_px,
                    color: *color,
                    align: *align,
                    outline_px: *outline_px,
                    outline_color: *outline_color,
                    // The designer's shadow is one checkbox: a soft offset
                    // scaled with the type size, not four more fields.
                    shadow_px: if *shadow {
                        (size_px * 0.08).clamp(1.0, 12.0)
                    } else {
                        0.0
                    },
                    ..TextStyle::default()
                };
                // A cell that cannot raster (no usable font) is skipped, not
                // fatal — the rest of the title still shows.
                if let Ok(raster) = render_text(&style) {
                    blit(&mut face, w, h, &raster, i64::from(*x), i64::from(*y));
                }
            }
        }
    }
    face
}

/// Which text a cell shows: a live override wins, then the bound file's
/// last good value, then the spec's text — and `{{variables}}` interpolate
/// into whichever won.
fn effective_text(spec_text: &str, override_value: Option<&str>, bound: Option<&str>) -> String {
    interpolate(override_value.or(bound).unwrap_or(spec_text))
}

// ---------------------------------------------------------------------------
// Bound files (CAP-M16 rules)
// ---------------------------------------------------------------------------

/// Mirrors `commands::studio::is_remote` (that crate sits above this one):
/// a UNC / URL path is never statted — on Windows the stat alone forces an
/// SMB/NTLM handshake. The studio refuses these before the session starts;
/// this is the generator's own belt to that suspenders.
fn remote_path(path: &str) -> bool {
    path.contains("://") || path.starts_with("\\\\") || path.starts_with("//")
}

#[derive(Default)]
struct BoundState {
    next_poll: Option<Instant>,
    fingerprint: Option<(Option<std::time::SystemTime>, u64)>,
    /// The last successfully extracted value — held through transient read
    /// gaps (atomic temp+rename writers) and selector misses, so a
    /// scoreboard never blanks on a half-written row.
    last_good: Option<String>,
}

/// One poll step: stat → fingerprint gate → bounded read → extract. Errors
/// keep `last_good`; only a real new value replaces it.
fn poll_bound(state: &mut BoundState, path: &str, binding: &Binding, now: Instant) {
    if state.next_poll.is_some_and(|at| now < at) {
        return;
    }
    state.next_poll = Some(now + BOUND_POLL);
    if remote_path(path) {
        return;
    }
    let Ok(meta) = std::fs::metadata(path) else {
        return; // transient gap (atomic rename) — hold the last good value
    };
    if meta.len() > textfile::MAX_BOUND_FILE_BYTES {
        return;
    }
    let fingerprint = (meta.modified().ok(), meta.len());
    if state.fingerprint.as_ref() == Some(&fingerprint) {
        return;
    }
    let Ok(content) = std::fs::read_to_string(path) else {
        return;
    };
    state.fingerprint = Some(fingerprint);
    let value = match binding {
        Binding::Whole => Ok(content.trim_end().to_string()),
        Binding::CsvCell { row, column } => textfile::csv_cell(&content, *row, column),
        Binding::JsonPointer { pointer } => textfile::json_value(&content, pointer),
    };
    if let Ok(value) = value {
        state.last_good = Some(value);
    }
}

// ---------------------------------------------------------------------------
// The session generator
// ---------------------------------------------------------------------------

/// Start the title session. The caller (the studio's start arm) has already
/// refused network paths for every layer; image layers load here, once, on
/// the starter thread — never in the render loop.
pub fn start_title(id: &str, config: TitleConfig) -> Result<CaptureSession, CaptureError> {
    check_dimension("title width", config.width)
        .and_then(|()| check_dimension("title height", config.height))
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    let mut images: Vec<Option<Frame>> = Vec::with_capacity(config.layers.len());
    for layer in &config.layers {
        images.push(match layer {
            LayerSpec::Image { path, .. } => {
                let trimmed = path.trim();
                if trimmed.is_empty() {
                    None
                } else if remote_path(trimmed) {
                    return Err(CaptureError::Backend(
                        "network paths are not read — title images must be local files".into(),
                    ));
                } else {
                    Some(
                        load_image_rgba(std::path::Path::new(trimmed))
                            .map_err(|err| CaptureError::Backend(err.to_string()))?,
                    )
                }
            }
            _ => None,
        });
    }

    let session = Arc::new(Mutex::new(TitleSession::new()));
    lock(registry()).insert(id.to_string(), Arc::downgrade(&session));
    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-title".into())
        .spawn(move || run_title(config, images, session, sender, thread_stop))
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

fn run_title(
    config: TitleConfig,
    images: Vec<Option<Frame>>,
    session: Arc<Mutex<TitleSession>>,
    sender: fcap_capture::FrameSender,
    stop: Arc<AtomicBool>,
) {
    let duration = Duration::from_millis(u64::from(config.duration_ms.min(MAX_DURATION_MS)));
    let period = Duration::from_micros(1_000_000 / u64::from(FPS));
    let mut next = Instant::now();
    // Per-layer bound-file poll state (text layers with a file only).
    let mut bound: Vec<Option<BoundState>> = config
        .layers
        .iter()
        .map(|layer| {
            matches!(layer, LayerSpec::Text { source_file, .. } if !source_file.trim().is_empty())
                .then(BoundState::default)
        })
        .collect();
    let mut steady: Vec<u8> = Vec::new();
    let mut steady_texts: Option<Vec<String>> = None;
    let mut content_revision: u64 = 0;
    // What the steady-state last sent: (content revision, shown?). While an
    // animation runs, frames are sent every tick instead.
    let mut last_sent: Option<(u64, bool)> = None;
    loop {
        if stop.load(Ordering::Relaxed) || !sender.is_open() {
            return;
        }
        let now = Instant::now();

        // 1. Resolve every text cell (override > bound file > spec text,
        //    then {{variables}}); the resolved vector is the change gate.
        let overrides = { lock(&session).overrides.clone() };
        let mut texts: Vec<String> = Vec::with_capacity(config.layers.len());
        for (index, layer) in config.layers.iter().enumerate() {
            let LayerSpec::Text {
                text,
                source_file,
                binding,
                ..
            } = layer
            else {
                texts.push(String::new());
                continue;
            };
            let bound_value = match bound[index].as_mut() {
                Some(state) => {
                    poll_bound(state, source_file.trim(), binding, now);
                    state.last_good.as_deref()
                }
                None => None,
            };
            texts.push(effective_text(
                text,
                overrides.get(&index).map(String::as_str),
                bound_value,
            ));
        }

        // 2. Recompose the steady buffer only when the content moved.
        if steady_texts.as_ref() != Some(&texts) {
            steady = compose_layers(&config, &texts, &images);
            steady_texts = Some(texts);
            content_revision += 1;
        }

        // 3. Step the animation; emit per-tick while it runs, on change when
        //    it does not (a steady title costs nothing).
        let (t, animating) = {
            let mut live = lock(&session);
            let t = live.progress(duration);
            (t, live.animating())
        };
        if animating {
            let posed = apply_pose(
                &steady,
                config.width,
                config.height,
                &pose(config.animation, t, config.width, config.height),
            );
            sender.send(rgba_frame(config.width, config.height, posed));
            last_sent = None;
        } else {
            let shown = t >= 1.0;
            // Hidden frames all look alike — content changes need no re-send.
            let key = (if shown { content_revision } else { 0 }, shown);
            if last_sent != Some(key) {
                let frame = if shown {
                    steady.clone()
                } else {
                    vec![0u8; steady.len()]
                };
                sender.send(rgba_frame(config.width, config.height, frame));
                last_sent = Some(key);
            }
        }

        next += period;
        let wake = Instant::now();
        if next > wake {
            std::thread::sleep(next - wake);
        } else {
            next = wake;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect_config(width: u32, height: u32, layers: Vec<LayerSpec>) -> TitleConfig {
        TitleConfig {
            width,
            height,
            layers,
            animation: Animation::SlideLeft,
            duration_ms: 400,
        }
    }

    fn pixel(buffer: &[u8], width: u32, x: u32, y: u32) -> [u8; 4] {
        let at = ((y * width + x) * 4) as usize;
        [buffer[at], buffer[at + 1], buffer[at + 2], buffer[at + 3]]
    }

    #[test]
    fn poses_follow_each_animation_curve() {
        // Fade: alpha rides t, no movement.
        assert_eq!(pose(Animation::Fade, 0.0, 8, 4).alpha, 0.0);
        assert_eq!(pose(Animation::Fade, 0.5, 8, 4).alpha, 0.5);
        assert_eq!(
            pose(Animation::Fade, 1.0, 8, 4),
            Pose {
                dx: 0,
                dy: 0,
                alpha: 1.0,
                reveal: 1.0
            }
        );
        // SlideLeft: starts a full canvas to the right, lands at zero.
        assert_eq!(pose(Animation::SlideLeft, 0.0, 8, 4).dx, 8);
        assert_eq!(pose(Animation::SlideLeft, 0.5, 8, 4).dx, 4);
        assert_eq!(pose(Animation::SlideLeft, 1.0, 8, 4).dx, 0);
        // SlideUp: same, vertically.
        assert_eq!(pose(Animation::SlideUp, 0.0, 8, 4).dy, 4);
        assert_eq!(pose(Animation::SlideUp, 1.0, 8, 4).dy, 0);
        // Wipe: the reveal edge rides t.
        assert_eq!(pose(Animation::Wipe, 0.25, 8, 4).reveal, 0.25);
        // None: a hard cut — hidden at 0, fully there for any t above it.
        assert_eq!(pose(Animation::None, 0.0, 8, 4).alpha, 0.0);
        assert_eq!(pose(Animation::None, 0.3, 8, 4).alpha, 1.0);
        // Out-of-range progress clamps instead of overshooting.
        assert_eq!(pose(Animation::SlideLeft, 2.0, 8, 4).dx, 0);
    }

    #[test]
    fn apply_pose_shifts_fades_and_wipes() {
        // A 4×2 canvas with one opaque red pixel at (0, 0).
        let mut steady = vec![0u8; 4 * 2 * 4];
        steady[..4].copy_from_slice(&[255, 0, 0, 255]);
        // Shift right by 2: the pixel lands at (2, 0).
        let slid = apply_pose(
            &steady,
            4,
            2,
            &Pose {
                dx: 2,
                dy: 0,
                alpha: 1.0,
                reveal: 1.0,
            },
        );
        assert_eq!(pixel(&slid, 4, 2, 0), [255, 0, 0, 255]);
        assert_eq!(pixel(&slid, 4, 0, 0)[3], 0, "the origin emptied");
        // Fade to half: alpha scales, color channels stay.
        let faded = apply_pose(
            &steady,
            4,
            2,
            &Pose {
                dx: 0,
                dy: 0,
                alpha: 0.5,
                reveal: 1.0,
            },
        );
        assert_eq!(pixel(&faded, 4, 0, 0), [255, 0, 0, 128]);
        // A half wipe clears everything right of the reveal edge.
        let mut wide = vec![0u8; 4 * 2 * 4];
        for x in 0..4 {
            wide[(x * 4)..(x * 4) + 4].copy_from_slice(&[0, 255, 0, 255]);
        }
        let wiped = apply_pose(
            &wide,
            4,
            2,
            &Pose {
                dx: 0,
                dy: 0,
                alpha: 1.0,
                reveal: 0.5,
            },
        );
        assert_eq!(pixel(&wiped, 4, 1, 0)[3], 255, "left of the edge shows");
        assert_eq!(pixel(&wiped, 4, 2, 0)[3], 0, "right of the edge is clear");
    }

    /// The DoD's "title animate-in" golden: compose a real layer stack and
    /// walk the slide-in — off-canvas at t=0, halfway at t=0.5, seated at 1.
    #[test]
    fn animate_in_slides_the_composed_content_into_place() {
        let config = rect_config(
            8,
            4,
            vec![LayerSpec::Rect {
                x: 0,
                y: 0,
                width: 2,
                height: 4,
                color: [0, 0, 255, 255],
            }],
        );
        let steady = compose_layers(&config, &[String::new()], &[None]);
        assert_eq!(pixel(&steady, 8, 0, 0), [0, 0, 255, 255]);
        let at = |t: f32| apply_pose(&steady, 8, 4, &pose(Animation::SlideLeft, t, 8, 4));
        assert!(
            at(0.0).chunks_exact(4).all(|px| px[3] == 0),
            "t=0: fully off-canvas"
        );
        let half = at(0.5);
        assert_eq!(pixel(&half, 8, 4, 0), [0, 0, 255, 255], "t=0.5: mid-slide");
        assert_eq!(pixel(&half, 8, 0, 0)[3], 0);
        assert_eq!(at(1.0), steady, "t=1: seated exactly on the steady frame");
    }

    #[test]
    fn layers_compose_in_order_at_their_positions() {
        let config = rect_config(
            8,
            8,
            vec![
                LayerSpec::Rect {
                    x: 1,
                    y: 1,
                    width: 4,
                    height: 4,
                    color: [0, 0, 255, 255],
                },
                // Drawn after → on top; also proves edge clipping survives.
                LayerSpec::Rect {
                    x: 3,
                    y: 3,
                    width: 200,
                    height: 2,
                    color: [255, 0, 0, 255],
                },
            ],
        );
        let face = compose_layers(&config, &[String::new(), String::new()], &[None, None]);
        assert_eq!(pixel(&face, 8, 0, 0)[3], 0, "outside both rects: clear");
        assert_eq!(pixel(&face, 8, 1, 1), [0, 0, 255, 255], "the base rect");
        assert_eq!(pixel(&face, 8, 3, 3), [255, 0, 0, 255], "later layer wins");
        assert_eq!(pixel(&face, 8, 7, 3), [255, 0, 0, 255], "clipped, not lost");
    }

    #[test]
    fn overrides_beat_bound_values_beat_spec_text_and_variables_expand() {
        // One test on purpose: `set_variables` replaces the global map, so
        // parallel tests poking it would race each other.
        set_variables(HashMap::from([("n16score".to_string(), "7".to_string())]));
        assert_eq!(effective_text("Score {{n16score}}", None, None), "Score 7");
        assert_eq!(
            effective_text("spec", None, Some("bound {{n16score}}")),
            "bound 7"
        );
        assert_eq!(
            effective_text("spec", Some("live"), Some("bound")),
            "live",
            "the operator's override wins"
        );
        assert_eq!(
            effective_text("Hi {{n16missing}}", None, None),
            "Hi {{n16missing}}",
            "unknown names stay visible"
        );
    }

    #[test]
    fn control_registry_runs_the_fire_and_override_lifecycle() {
        let session = Arc::new(Mutex::new(TitleSession::new()));
        lock(registry()).insert("title-test-live".into(), Arc::downgrade(&session));
        assert!(lock(&session).animating(), "a fresh title is animating in");
        // A zero duration settles instantly; Out then hides it.
        assert_eq!(lock(&session).progress(Duration::ZERO), 1.0);
        assert!(!lock(&session).animating());
        assert!(control("title-test-live", TitleAction::FireOut));
        assert_eq!(lock(&session).progress(Duration::ZERO), 0.0);
        assert!(control("title-test-live", TitleAction::FireIn));
        assert!(lock(&session).animating());
        // Overrides land bounded.
        assert!(control(
            "title-test-live",
            TitleAction::SetLayerText(2, "x".repeat(2000))
        ));
        assert_eq!(
            lock(&session).overrides.get(&2).map(String::len),
            Some(MAX_OVERRIDE_CHARS)
        );
        drop(session);
        assert!(
            !control("title-test-live", TitleAction::FireIn),
            "dead is dead"
        );
    }

    #[test]
    fn bound_files_poll_with_a_fingerprint_and_hold_the_last_good_value() {
        let path =
            std::env::temp_dir().join(format!("fcap-title-bound-{}.csv", std::process::id()));
        std::fs::write(&path, "team,score\nRed,17\n").expect("fixture writes");
        let text = path.display().to_string();
        let binding = Binding::CsvCell {
            row: 1,
            column: "score".into(),
        };
        let mut state = BoundState::default();
        let mut now = Instant::now();
        poll_bound(&mut state, &text, &binding, now);
        assert_eq!(state.last_good.as_deref(), Some("17"));
        // A different-length write moves the fingerprint (mtime alone can be
        // too coarse within one test).
        std::fs::write(&path, "team,score\nRed,2145\n").expect("fixture rewrites");
        now += BOUND_POLL;
        poll_bound(&mut state, &text, &binding, now);
        assert_eq!(state.last_good.as_deref(), Some("2145"));
        // The file vanishing (an atomic writer mid-rename) keeps the value.
        std::fs::remove_file(&path).expect("fixture removes");
        now += BOUND_POLL;
        poll_bound(&mut state, &text, &binding, now);
        assert_eq!(state.last_good.as_deref(), Some("2145"));
    }

    #[test]
    fn network_paths_are_never_statted() {
        // Mirrors the studio's is_remote — pinned here because the poll and
        // the image loader gate on this exact predicate.
        for hostile in [
            "\\\\attacker\\share\\score.csv",
            "//attacker/share/score.csv",
            "http://attacker/score.csv",
        ] {
            assert!(remote_path(hostile), "{hostile} must be refused");
            let mut state = BoundState::default();
            poll_bound(&mut state, hostile, &Binding::Whole, Instant::now());
            assert_eq!(state.last_good, None);
        }
        assert!(!remote_path("C:/data/score.csv"));
    }
}
