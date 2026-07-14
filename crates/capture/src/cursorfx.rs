//! Cursor highlight & click effects (CAP-N19): a soft halo under the pointer,
//! left/right click ripples, and optional keystroke ghosting — drawn straight
//! into display/window capture frames on the owned cursor path, so recordings
//! and streams carry them everywhere the capture goes.
//!
//! Only Windows owns its cursor rendering (win/pointer.rs draws the pointer
//! because DXGI and WGC deliver frames without it); macOS/Linux composite the
//! cursor OS-side, so there is no seam to draw effects into — the UI says so
//! honestly. This module keeps the config, registry, ripple state machine and
//! the pixel work cross-platform so the tests run everywhere; only the input
//! *sampling* (GetAsyncKeyState) and GDI label rendering live under `win/`.
//!
//! Config lives in a live registry keyed by the capture id (the CAP-N74
//! tone-map shape): a retune applies on the very next frame — no session
//! restart. With no entry the capture thread samples nothing at all.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use crate::Frame;

/// Left mouse button bit in the sampled button mask.
pub const BUTTON_LEFT: u8 = 1;
/// Right mouse button bit in the sampled button mask.
pub const BUTTON_RIGHT: u8 = 2;

/// How long one click ripple lives.
pub const RIPPLE_LIFE_MS: u64 = 600;
/// The ring's radius at end of life, in frame pixels.
pub const RIPPLE_MAX_RADIUS: f32 = 44.0;
/// Ring stroke width, in frame pixels.
const RIPPLE_THICKNESS: f32 = 3.0;
/// Ring opacity at birth (fades linearly to 0 over the life).
const RIPPLE_ALPHA: f32 = 200.0;
/// Halo opacity at the cursor hotspot (falls off quadratically to the rim) —
/// translucent on purpose: the effect hooks *after* the cursor blend, so the
/// pointer must stay readable underneath.
const HALO_ALPHA: u32 = 96;

/// One capture's cursor-effect parameters. Colors are RGB (the draw fns
/// swizzle into the BGRA frames the Windows paths produce).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorFxConfig {
    pub halo: bool,
    pub halo_color: [u8; 3],
    /// Halo radius in frame pixels (drawn clamped to 8–128).
    pub halo_radius: u32,
    pub ripples: bool,
    pub left_color: [u8; 3],
    pub right_color: [u8; 3],
    pub keystrokes: bool,
}

/// `"#rrggbb"` → RGB. The settings store keeps colors as hex strings; both
/// its validator and the config builders parse through here.
pub fn parse_color(hex: &str) -> Option<[u8; 3]> {
    let hex = hex.strip_prefix('#')?;
    // Byte-sliced below: a multibyte char inside a 6-BYTE string would panic
    // at a char boundary instead of failing — refuse non-ASCII up front.
    if hex.len() != 6 || !hex.is_ascii() {
        return None;
    }
    let byte = |at: usize| u8::from_str_radix(&hex[at..at + 2], 16).ok();
    Some([byte(0)?, byte(2)?, byte(4)?])
}

fn registry() -> &'static Mutex<HashMap<String, CursorFxConfig>> {
    static REG: OnceLock<Mutex<HashMap<String, CursorFxConfig>>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Set (or clear, with `None`) a capture's cursor effects. Live: the capture
/// thread reads this per tick. `None` (nothing enabled) means the thread
/// samples no input at all — off really is off.
pub fn set_cursor_fx(id: &str, config: Option<CursorFxConfig>) {
    let mut reg = registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    match config {
        Some(config) => {
            reg.insert(id.to_string(), config);
        }
        None => {
            reg.remove(id);
        }
    }
}

/// The capture thread's per-tick lookup. Only the Windows paths read it (the
/// owned cursor path is Windows), so it is dormant elsewhere — the writer
/// `set_cursor_fx` and the tests stay cross-platform.
#[cfg_attr(not(target_os = "windows"), allow(dead_code))]
pub(crate) fn cursor_fx_for(id: &str) -> Option<CursorFxConfig> {
    registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(id)
        .copied()
}

/// One live click ripple, anchored where the press happened (frame pixels) —
/// it keeps animating there even if the cursor moves on.
#[derive(Debug, Clone, Copy)]
pub struct Ripple {
    pub x: i32,
    pub y: i32,
    /// Right button (colors are per-button).
    pub right: bool,
    pub born: Instant,
}

/// Per-session effect state: live ripples + the previous input sample, so
/// press *edges* (not held buttons) spawn ripples and key-set changes are
/// detected. One per capture thread; never shared.
pub struct FxState {
    ripples: Vec<Ripple>,
    prev_buttons: u8,
    /// The fixed-set keys currently held (opaque ids — VK codes on Windows),
    /// in display order. Kept only to draw badges and detect changes; never
    /// stored beyond the tick, never logged.
    held_keys: Vec<i32>,
    /// The config as of the last tick — a live retune must repaint even a
    /// perfectly still screen.
    last_config: Option<CursorFxConfig>,
}

impl FxState {
    pub fn new() -> Self {
        FxState {
            ripples: Vec::new(),
            prev_buttons: 0,
            held_keys: Vec::new(),
            last_config: None,
        }
    }

    /// Feed one tick of sampled input. `x`/`y` = cursor hotspot in frame
    /// pixels; `over` = the cursor is on this capture (presses elsewhere
    /// spawn nothing). Returns true when the drawn effects changed and a
    /// frame should be synthesized even though no capture frame arrived: a
    /// ripple is animating / was born / died, the held-key set changed, or
    /// the config was retuned live.
    #[allow(clippy::too_many_arguments)] // one caller (pointer::fx_tick); a struct would just rename these
    pub fn tick(
        &mut self,
        config: Option<&CursorFxConfig>,
        buttons: u8,
        keys: &[i32],
        x: i32,
        y: i32,
        over: bool,
        now: Instant,
    ) -> bool {
        let mut dirty = false;
        if self.last_config != config.copied() {
            self.last_config = config.copied();
            dirty = true;
        }
        // Press EDGES spawn ripples — a held button draws exactly one.
        let pressed = buttons & !self.prev_buttons;
        self.prev_buttons = buttons;
        if config.is_some_and(|c| c.ripples) && over {
            if self.ripples.len() >= 16 {
                // An autoclicker can't grow the vec without bound; the oldest
                // ripple is the nearly-invisible one, so drop it first.
                self.ripples.remove(0);
            }
            if pressed & BUTTON_LEFT != 0 {
                self.ripples.push(Ripple {
                    x,
                    y,
                    right: false,
                    born: now,
                });
                dirty = true;
            }
            if pressed & BUTTON_RIGHT != 0 {
                self.ripples.push(Ripple {
                    x,
                    y,
                    right: true,
                    born: now,
                });
                dirty = true;
            }
        }
        let life = Duration::from_millis(RIPPLE_LIFE_MS);
        let before = self.ripples.len();
        self.ripples.retain(|r| now.duration_since(r.born) < life);
        if self.ripples.len() != before {
            dirty = true;
        }
        if self.held_keys != keys {
            self.held_keys = keys.to_vec();
            dirty = true;
        }
        // Live ripples animate continuously — every tick repaints.
        dirty || !self.ripples.is_empty()
    }

    /// The fixed-set keys currently held (for the caller's badge drawing).
    pub fn held_keys(&self) -> &[i32] {
        &self.held_keys
    }

    #[cfg(test)]
    fn ripples(&self) -> &[Ripple] {
        &self.ripples
    }

    /// Draw the halo + every live ripple into the frame (hooked AFTER the
    /// cursor blend). Badges are drawn by the caller, which owns the per-OS
    /// label cache.
    pub fn draw(
        &self,
        frame: &mut Frame,
        config: &CursorFxConfig,
        x: i32,
        y: i32,
        over: bool,
        now: Instant,
    ) {
        if config.halo && over {
            draw_halo(frame, x, y, config.halo_color, config.halo_radius);
        }
        if config.ripples {
            for ripple in &self.ripples {
                let progress = ripple_progress(now.duration_since(ripple.born));
                let color = if ripple.right {
                    config.right_color
                } else {
                    config.left_color
                };
                draw_ripple(frame, ripple.x, ripple.y, color, progress);
            }
        }
    }
}

impl Default for FxState {
    fn default() -> Self {
        Self::new()
    }
}

/// A ripple's age → animation progress in `[0, 1]` (1 = expired).
pub fn ripple_progress(age: Duration) -> f32 {
    (age.as_secs_f32() * 1000.0 / RIPPLE_LIFE_MS as f32).min(1.0)
}

/// Alpha-blend `color` (RGB) into one BGRA pixel. Alpha is 0–255.
#[inline]
fn blend_px(frame: &mut Frame, x: i32, y: i32, color: [u8; 3], alpha: u32) {
    if x < 0 || y < 0 || x >= frame.width as i32 || y >= frame.height as i32 {
        return;
    }
    let idx = y as usize * frame.stride as usize + x as usize * 4;
    let Some(dst) = frame.data.get_mut(idx..idx + 3) else {
        return;
    };
    // The frame is BGRA; the color is RGB — swizzle while blending.
    let src = [color[2], color[1], color[0]];
    for c in 0..3 {
        dst[c] = ((u32::from(src[c]) * alpha + u32::from(dst[c]) * (255 - alpha)) / 255) as u8;
    }
}

/// A soft translucent disc centered on the cursor hotspot: full [`HALO_ALPHA`]
/// at the center, quadratic falloff to 0 at the rim.
pub fn draw_halo(frame: &mut Frame, x: i32, y: i32, color: [u8; 3], radius: u32) {
    let radius = radius.clamp(8, 128) as i32;
    let r2 = (radius * radius) as f32;
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let d2 = (dx * dx + dy * dy) as f32;
            if d2 > r2 {
                continue;
            }
            let alpha = (HALO_ALPHA as f32 * (1.0 - d2 / r2)) as u32;
            if alpha == 0 {
                continue;
            }
            blend_px(frame, x + dx, y + dy, color, alpha);
        }
    }
}

/// One click ripple: a ~3 px ring at `progress × RIPPLE_MAX_RADIUS`, fading
/// out as it grows. `progress` outside `[0, 1)` draws nothing.
pub fn draw_ripple(frame: &mut Frame, x: i32, y: i32, color: [u8; 3], progress: f32) {
    if !(0.0..1.0).contains(&progress) {
        return;
    }
    let radius = progress * RIPPLE_MAX_RADIUS;
    let alpha_peak = RIPPLE_ALPHA * (1.0 - progress);
    let half = RIPPLE_THICKNESS / 2.0;
    let bound = (radius + half).ceil() as i32;
    for dy in -bound..=bound {
        for dx in -bound..=bound {
            let d = ((dx * dx + dy * dy) as f32).sqrt();
            // Coverage falls off linearly across the stroke — a cheap
            // anti-alias that keeps the ring smooth as it grows.
            let coverage = (half - (d - radius).abs()) / half;
            if coverage <= 0.0 {
                continue;
            }
            let alpha = (alpha_peak * coverage.min(1.0)) as u32;
            if alpha == 0 {
                continue;
            }
            blend_px(frame, x + dx, y + dy, color, alpha);
        }
    }
}

/// One rendered key label: per-pixel coverage (the GDI white-on-black
/// luminance trick on Windows; tests build these by hand). White text over a
/// translucent dark backing when drawn.
pub struct KeyBadge {
    pub width: u32,
    pub height: u32,
    /// `width × height` coverage bytes, row-major.
    pub alpha: Vec<u8>,
}

/// Backing-box padding around a badge label, in pixels.
const BADGE_PAD: i32 = 4;
/// Backing-box opacity (translucent black).
const BADGE_BACK_ALPHA: u32 = 150;
/// Gap between adjacent badges.
const BADGE_GAP: i32 = 4;

/// Draw one key badge with its backing box at (`x`, `y`) (top-left of the
/// box). Returns the horizontal advance to the next badge slot.
pub fn draw_badge(frame: &mut Frame, x: i32, y: i32, badge: &KeyBadge) -> i32 {
    let box_w = badge.width as i32 + 2 * BADGE_PAD;
    let box_h = badge.height as i32 + 2 * BADGE_PAD;
    for dy in 0..box_h {
        for dx in 0..box_w {
            blend_px(frame, x + dx, y + dy, [0, 0, 0], BADGE_BACK_ALPHA);
        }
    }
    for row in 0..badge.height as i32 {
        for col in 0..badge.width as i32 {
            let alpha = u32::from(badge.alpha[(row * badge.width as i32 + col) as usize]);
            if alpha == 0 {
                continue;
            }
            blend_px(
                frame,
                x + BADGE_PAD + col,
                y + BADGE_PAD + row,
                [255, 255, 255],
                alpha,
            );
        }
    }
    box_w + BADGE_GAP
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PixelFormat;

    fn frame(w: u32, h: u32) -> Frame {
        Frame {
            width: w,
            height: h,
            stride: w * 4,
            format: PixelFormat::Bgra8,
            data: vec![0; (w * h * 4) as usize],
            captured_at: Instant::now(),
        }
    }

    fn px(frame: &Frame, x: u32, y: u32) -> [u8; 4] {
        let idx = (y * frame.stride + x * 4) as usize;
        frame.data[idx..idx + 4].try_into().unwrap()
    }

    fn config() -> CursorFxConfig {
        CursorFxConfig {
            halo: true,
            halo_color: [255, 213, 74],
            halo_radius: 24,
            ripples: true,
            left_color: [0, 255, 0],
            right_color: [255, 0, 0],
            keystrokes: false,
        }
    }

    #[test]
    fn parse_color_refuses_multibyte_without_panicking() {
        // 6 BYTES but not 6 ASCII chars — byte-sliced, this used to panic at
        // a char boundary inside the render loop instead of returning None.
        assert_eq!(parse_color("#aééb"), None);
        assert_eq!(parse_color("#ff8800"), Some([0xff, 0x88, 0x00]));
        assert_eq!(parse_color("#ff880"), None, "five hex digits is refused");
        assert_eq!(parse_color("ff8800"), None, "the # is required");
    }

    #[test]
    fn press_edges_spawn_ripples_and_holds_do_not() {
        let mut fx = FxState::new();
        let cfg = config();
        let t0 = Instant::now();
        // First tick with a config counts as a retune (repaint), so settle it.
        fx.tick(Some(&cfg), 0, &[], 10, 10, true, t0);
        assert!(
            !fx.tick(Some(&cfg), 0, &[], 10, 10, true, t0),
            "idle tick is quiet"
        );

        assert!(fx.tick(Some(&cfg), BUTTON_LEFT, &[], 10, 10, true, t0));
        assert_eq!(fx.ripples().len(), 1, "left press edge spawned");
        assert!(!fx.ripples()[0].right);

        // Held button: no second ripple, but the live one keeps animating.
        assert!(fx.tick(Some(&cfg), BUTTON_LEFT, &[], 12, 12, true, t0));
        assert_eq!(fx.ripples().len(), 1, "hold spawns nothing");

        // Release then right-press: a right ripple at the new position.
        fx.tick(Some(&cfg), 0, &[], 20, 20, true, t0);
        fx.tick(Some(&cfg), BUTTON_RIGHT, &[], 20, 20, true, t0);
        assert_eq!(fx.ripples().len(), 2);
        assert!(fx.ripples()[1].right);
        assert_eq!((fx.ripples()[1].x, fx.ripples()[1].y), (20, 20));
    }

    #[test]
    fn clicks_away_from_the_capture_spawn_nothing() {
        let mut fx = FxState::new();
        let cfg = config();
        let t0 = Instant::now();
        fx.tick(Some(&cfg), 0, &[], 0, 0, false, t0);
        fx.tick(Some(&cfg), BUTTON_LEFT, &[], 0, 0, false, t0);
        assert!(fx.ripples().is_empty(), "not over the capture");
        // …and with no config at all, a press over the capture spawns nothing.
        let mut fx = FxState::new();
        fx.tick(None, 0, &[], 5, 5, true, t0);
        fx.tick(None, BUTTON_LEFT, &[], 5, 5, true, t0);
        assert!(fx.ripples().is_empty(), "no config = no effects");
    }

    #[test]
    fn ripples_age_out_and_progress_is_bounded() {
        let mut fx = FxState::new();
        let cfg = config();
        let t0 = Instant::now();
        fx.tick(Some(&cfg), 0, &[], 10, 10, true, t0);
        fx.tick(Some(&cfg), BUTTON_LEFT, &[], 10, 10, true, t0);
        let mid = t0 + Duration::from_millis(RIPPLE_LIFE_MS / 2);
        assert!(
            fx.tick(Some(&cfg), BUTTON_LEFT, &[], 10, 10, true, mid),
            "still animating"
        );
        assert_eq!(fx.ripples().len(), 1);
        let after = t0 + Duration::from_millis(RIPPLE_LIFE_MS + 50);
        assert!(
            fx.tick(Some(&cfg), BUTTON_LEFT, &[], 10, 10, true, after),
            "the death tick itself repaints (to erase the ring)"
        );
        assert!(fx.ripples().is_empty(), "expired");
        assert!(
            !fx.tick(Some(&cfg), BUTTON_LEFT, &[], 10, 10, true, after),
            "then quiet"
        );

        assert_eq!(ripple_progress(Duration::from_millis(0)), 0.0);
        assert!((ripple_progress(Duration::from_millis(RIPPLE_LIFE_MS / 2)) - 0.5).abs() < 1e-3);
        assert_eq!(
            ripple_progress(Duration::from_millis(RIPPLE_LIFE_MS * 3)),
            1.0,
            "clamped"
        );
    }

    #[test]
    fn key_set_changes_and_config_retunes_repaint() {
        let mut fx = FxState::new();
        let cfg = config();
        let t0 = Instant::now();
        assert!(
            fx.tick(Some(&cfg), 0, &[], 0, 0, true, t0),
            "config arrival repaints"
        );
        assert!(
            fx.tick(Some(&cfg), 0, &[0x41], 0, 0, true, t0),
            "key down repaints"
        );
        assert_eq!(fx.held_keys(), &[0x41]);
        assert!(
            !fx.tick(Some(&cfg), 0, &[0x41], 0, 0, true, t0),
            "held key is quiet"
        );
        assert!(
            fx.tick(Some(&cfg), 0, &[], 0, 0, true, t0),
            "key up repaints"
        );
        let retuned = CursorFxConfig {
            halo_radius: 48,
            ..cfg
        };
        assert!(
            fx.tick(Some(&retuned), 0, &[], 0, 0, true, t0),
            "live retune repaints"
        );
        assert!(
            fx.tick(None, 0, &[], 0, 0, true, t0),
            "clearing the config repaints once"
        );
        assert!(!fx.tick(None, 0, &[], 0, 0, true, t0), "then fully quiet");
    }

    #[test]
    fn halo_pixels_center_bright_edge_untouched() {
        let mut f = frame(64, 64);
        draw_halo(&mut f, 32, 32, [255, 0, 0], 10);
        let center = px(&f, 32, 32);
        // Red halo on a BGRA frame: red lands in byte 2.
        assert_eq!(
            center[2],
            (255 * HALO_ALPHA / 255) as u8,
            "center at peak alpha"
        );
        assert_eq!(center[0], 0, "no blue bleed");
        let inside = px(&f, 32 + 5, 32);
        assert!(
            inside[2] > 0 && inside[2] < center[2],
            "falloff inside the disc"
        );
        assert_eq!(
            px(&f, 32 + 11, 32),
            [0, 0, 0, 0],
            "outside the radius untouched"
        );
        // Clipping: a halo at the corner must not panic or wrap.
        draw_halo(&mut f, 0, 0, [255, 0, 0], 10);
        assert!(px(&f, 0, 0)[2] > 0);
    }

    #[test]
    fn ripple_ring_hits_the_ring_not_the_center() {
        let mut f = frame(128, 128);
        // progress 0.5 → radius = half of RIPPLE_MAX_RADIUS.
        let radius = (0.5 * RIPPLE_MAX_RADIUS) as u32;
        draw_ripple(&mut f, 64, 64, [0, 255, 0], 0.5);
        let on_ring = px(&f, 64 + radius, 64);
        assert!(on_ring[1] > 0, "green on the ring (radius {radius})");
        assert_eq!(px(&f, 64, 64), [0, 0, 0, 0], "center untouched");
        assert_eq!(
            px(&f, 64 + radius + 8, 64),
            [0, 0, 0, 0],
            "outside untouched"
        );
        // An expired ripple draws nothing.
        let mut f2 = frame(128, 128);
        draw_ripple(&mut f2, 64, 64, [0, 255, 0], 1.0);
        assert!(f2.data.iter().all(|&b| b == 0));
    }

    #[test]
    fn ripple_fades_as_it_grows() {
        let mut early = frame(128, 128);
        draw_ripple(&mut early, 64, 64, [0, 255, 0], 0.25);
        let mut late = frame(128, 128);
        draw_ripple(&mut late, 64, 64, [0, 255, 0], 0.85);
        let r_early = (0.25 * RIPPLE_MAX_RADIUS) as u32;
        let r_late = (0.85 * RIPPLE_MAX_RADIUS) as u32;
        assert!(
            px(&early, 64 + r_early, 64)[1] > px(&late, 64 + r_late, 64)[1],
            "older ring is fainter"
        );
    }

    #[test]
    fn badge_draws_backing_and_label() {
        let mut f = frame(64, 64);
        // A 4×4 fully-covered label.
        let badge = KeyBadge {
            width: 4,
            height: 4,
            alpha: vec![255; 16],
        };
        let advance = draw_badge(&mut f, 10, 10, &badge);
        assert_eq!(advance, 4 + 2 * BADGE_PAD + BADGE_GAP);
        // Label pixel: white at full coverage over the dark backing.
        let label = px(&f, (10 + BADGE_PAD) as u32, (10 + BADGE_PAD) as u32);
        assert!(
            label[0] > 200 && label[1] > 200 && label[2] > 200,
            "white label"
        );
        // Backing pixel (inside the box, outside the label): darkened, not white.
        let backing = px(&f, 10, 10);
        assert_eq!(
            backing,
            [0, 0, 0, 0],
            "backing over black stays black (translucent)"
        );
        let mut lit = frame(64, 64);
        lit.data.fill(200);
        draw_badge(&mut lit, 10, 10, &badge);
        assert!(px(&lit, 10, 10)[0] < 200, "backing darkens a lit frame");
        assert_eq!(px(&lit, 9, 9)[0], 200, "outside the box untouched");
    }

    #[test]
    fn registry_lifecycle_set_get_clear() {
        assert_eq!(cursor_fx_for("fx-test-none"), None, "no entry, no work");
        let cfg = config();
        set_cursor_fx("fx-test-a", Some(cfg));
        assert_eq!(cursor_fx_for("fx-test-a"), Some(cfg));
        let retuned = CursorFxConfig {
            keystrokes: true,
            ..cfg
        };
        set_cursor_fx("fx-test-a", Some(retuned));
        assert_eq!(
            cursor_fx_for("fx-test-a"),
            Some(retuned),
            "live retune replaces"
        );
        set_cursor_fx("fx-test-a", None);
        assert_eq!(cursor_fx_for("fx-test-a"), None, "cleared");
    }

    #[test]
    fn hex_colors_parse_and_reject_garbage() {
        assert_eq!(parse_color("#ff0080"), Some([255, 0, 128]));
        assert_eq!(parse_color("#FFD54A"), Some([255, 213, 74]));
        assert_eq!(parse_color("ff0080"), None, "no leading #");
        assert_eq!(parse_color("#ff008"), None, "short");
        assert_eq!(parse_color("#ff00zz"), None, "not hex");
    }
}
