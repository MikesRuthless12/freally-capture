//! CAP-N13 — the **input overlay source**: a live visualization of keyboard,
//! mouse, and gamepad state for the viewers — pressed key caps, mouse button
//! zones, stick circles that follow the axes, trigger fill bars. Four fixed
//! layout presets (WASD, compact keyboard, gamepad, fight stick); no custom
//! skins yet — presets keep the polled key set auditable.
//!
//! Privacy by construction (stated in-product too):
//! - Input is read **only while an overlay source is live in a scene** — the
//!   sampler runs inside this session's thread and dies with it.
//! - It is a point-in-time poll of "is this down right now?" over the
//!   layout's FIXED key set — no hook, no event queue, no buffer. **Nothing
//!   is ever logged, stored, or sent anywhere**, and free typing is never
//!   captured (a state peek at 30 Hz has no key order to reconstruct).
//! - Keyboard/mouse state is Windows-only today ([`fcap_capture::keys_down`]
//!   returns `None` elsewhere and the caps draw unpressed — the picker says
//!   so; never a fake). Gamepads read through the cross-platform `gilrs`
//!   library; init failure or no controller draws the unpressed layout.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame};

use crate::static_source::rgba_frame;
use crate::text::{render_text, TextAlign, TextStyle};

/// The generator's frame cadence — state changes repaint, idle paints once.
const FPS: u32 = 30;
/// One key cell, px. Layout geometry is expressed in cell units.
const CELL: f32 = 56.0;
/// Canvas padding around the layout, px.
const PAD: f32 = 14.0;
/// The gap carved out of each cell so caps read as separate keys, px.
const INSET: f32 = 3.0;
/// Key-cap corner radius, px.
const CAP_RADIUS: f32 = 7.0;
/// Mouse silhouette size, cells.
const MOUSE_W: f32 = 1.6 * CELL;
const MOUSE_H: f32 = 2.6 * CELL;

/// Which fixed layout the overlay draws (mirrors the scene model's enum).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Wasd,
    Keyboard,
    Gamepad,
    Fightstick,
}

/// Everything the generator needs, already resolved by the caller.
#[derive(Debug, Clone)]
pub struct InputOverlayConfig {
    pub layout: Layout,
    /// The idle key-cap / outline color (straight RGBA).
    pub color: [u8; 4],
    /// The pressed-state fill.
    pub accent: [u8; 4],
}

/// One sampled controller, gilrs conventions (X right, Y **up**, −1..1).
/// Axes arrive quantized (see [`sample_pad`]) so equal-looking states
/// compare equal and the repaint gate holds.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct PadState {
    pub connected: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
    pub north: bool,
    pub lb: bool,
    pub rb: bool,
    pub l3: bool,
    pub r3: bool,
    pub select: bool,
    pub start: bool,
    pub dpad_up: bool,
    pub dpad_down: bool,
    pub dpad_left: bool,
    pub dpad_right: bool,
    pub lx: f32,
    pub ly: f32,
    pub rx: f32,
    pub ry: f32,
    pub lt: f32,
    pub rt: f32,
}

// ---------------------------------------------------------------------------
// The fixed layouts (the WHOLE polled key set — nothing outside these is read)
// ---------------------------------------------------------------------------

/// One key cap: virtual key, label, and its cell-unit rect (1 cell high).
struct KeyDef {
    vk: u16,
    label: &'static str,
    x: f32,
    y: f32,
    w: f32,
}

const fn key(vk: u16, label: &'static str, x: f32, y: f32, w: f32) -> KeyDef {
    KeyDef { vk, label, x, y, w }
}

/// A keyboard-style layout: caps on a cell grid plus the mouse silhouette.
struct KeyboardPlan {
    keys: &'static [KeyDef],
    /// Cell x where the mouse silhouette starts.
    mouse_x: f32,
    /// Canvas extent, cells.
    cells_w: f32,
    cells_h: f32,
}

/// The WASD cluster: movement + the two keys every shooter binds.
const WASD_KEYS: &[KeyDef] = &[
    key(0x57, "W", 1.0, 0.0, 1.0),
    key(0x41, "A", 0.0, 1.0, 1.0),
    key(0x53, "S", 1.0, 1.0, 1.0),
    key(0x44, "D", 2.0, 1.0, 1.0),
    key(0xA0, "SHIFT", 0.0, 2.0, 1.45),
    key(0x20, "SPACE", 1.55, 2.0, 1.45),
];

const WASD_PLAN: KeyboardPlan = KeyboardPlan {
    keys: WASD_KEYS,
    mouse_x: 3.4,
    cells_w: 5.0,
    cells_h: 3.0,
};

/// A compact staggered keyboard: digits, letters, and the modifier row.
const KEYBOARD_KEYS: &[KeyDef] = &[
    key(0x31, "1", 0.5, 0.0, 1.0),
    key(0x32, "2", 1.5, 0.0, 1.0),
    key(0x33, "3", 2.5, 0.0, 1.0),
    key(0x34, "4", 3.5, 0.0, 1.0),
    key(0x35, "5", 4.5, 0.0, 1.0),
    key(0x36, "6", 5.5, 0.0, 1.0),
    key(0x37, "7", 6.5, 0.0, 1.0),
    key(0x38, "8", 7.5, 0.0, 1.0),
    key(0x39, "9", 8.5, 0.0, 1.0),
    key(0x30, "0", 9.5, 0.0, 1.0),
    key(0x51, "Q", 0.75, 1.0, 1.0),
    key(0x57, "W", 1.75, 1.0, 1.0),
    key(0x45, "E", 2.75, 1.0, 1.0),
    key(0x52, "R", 3.75, 1.0, 1.0),
    key(0x54, "T", 4.75, 1.0, 1.0),
    key(0x59, "Y", 5.75, 1.0, 1.0),
    key(0x55, "U", 6.75, 1.0, 1.0),
    key(0x49, "I", 7.75, 1.0, 1.0),
    key(0x4F, "O", 8.75, 1.0, 1.0),
    key(0x50, "P", 9.75, 1.0, 1.0),
    key(0x41, "A", 1.0, 2.0, 1.0),
    key(0x53, "S", 2.0, 2.0, 1.0),
    key(0x44, "D", 3.0, 2.0, 1.0),
    key(0x46, "F", 4.0, 2.0, 1.0),
    key(0x47, "G", 5.0, 2.0, 1.0),
    key(0x48, "H", 6.0, 2.0, 1.0),
    key(0x4A, "J", 7.0, 2.0, 1.0),
    key(0x4B, "K", 8.0, 2.0, 1.0),
    key(0x4C, "L", 9.0, 2.0, 1.0),
    key(0x0D, "ENTER", 10.05, 2.0, 1.45),
    key(0xA0, "SHIFT", 0.0, 3.0, 1.2),
    key(0x5A, "Z", 1.3, 3.0, 1.0),
    key(0x58, "X", 2.3, 3.0, 1.0),
    key(0x43, "C", 3.3, 3.0, 1.0),
    key(0x56, "V", 4.3, 3.0, 1.0),
    key(0x42, "B", 5.3, 3.0, 1.0),
    key(0x4E, "N", 6.3, 3.0, 1.0),
    key(0x4D, "M", 7.3, 3.0, 1.0),
    key(0xA2, "CTRL", 0.0, 4.0, 1.2),
    key(0xA4, "ALT", 1.3, 4.0, 1.2),
    key(0x20, "SPACE", 2.6, 4.0, 4.4),
];

const KEYBOARD_PLAN: KeyboardPlan = KeyboardPlan {
    keys: KEYBOARD_KEYS,
    mouse_x: 11.9,
    cells_w: 13.5,
    cells_h: 5.0,
};

/// The mouse buttons, as virtual keys (left, right, middle) — appended after
/// the layout's keyboard keys in the sampled order.
const MOUSE_VKS: [u16; 3] = [0x01, 0x02, 0x04];

fn keyboard_plan(layout: Layout) -> Option<&'static KeyboardPlan> {
    match layout {
        Layout::Wasd => Some(&WASD_PLAN),
        Layout::Keyboard => Some(&KEYBOARD_PLAN),
        Layout::Gamepad | Layout::Fightstick => None,
    }
}

/// The COMPLETE set of virtual keys a layout ever polls (keyboard caps
/// first, then LMB/RMB/MMB). Pad layouts poll no keys at all.
pub fn vk_set(layout: Layout) -> Vec<u16> {
    match keyboard_plan(layout) {
        Some(plan) => plan
            .keys
            .iter()
            .map(|key| key.vk)
            .chain(MOUSE_VKS)
            .collect(),
        None => Vec::new(),
    }
}

/// The layout's fixed canvas size, px.
pub fn layout_size(layout: Layout) -> (u32, u32) {
    match layout {
        Layout::Wasd | Layout::Keyboard => {
            let plan = keyboard_plan(layout).expect("keyboard layouts have plans");
            (
                (PAD * 2.0 + plan.cells_w * CELL) as u32,
                (PAD * 2.0 + plan.cells_h * CELL) as u32,
            )
        }
        Layout::Gamepad => (520, 300),
        Layout::Fightstick => (520, 240),
    }
}

// ---------------------------------------------------------------------------
// Drawing (pure — unit-testable without any OS input)
// ---------------------------------------------------------------------------

/// The color at a fraction of its alpha (idle fills, outlines).
fn dim(color: [u8; 4], factor: f32) -> [u8; 4] {
    [
        color[0],
        color[1],
        color[2],
        (color[3] as f32 * factor) as u8,
    ]
}

#[derive(Clone, Copy)]
struct RectF {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}

/// Rounded-rect hit test (the clamp-to-corner-center trick).
fn in_round_rect(px: f32, py: f32, rect: RectF, radius: f32) -> bool {
    if px < rect.x0 || px >= rect.x1 || py < rect.y0 || py >= rect.y1 {
        return false;
    }
    let radius = radius
        .min((rect.x1 - rect.x0) * 0.5)
        .min((rect.y1 - rect.y0) * 0.5)
        .max(0.0);
    let cx = px.clamp(rect.x0 + radius, rect.x1 - radius);
    let cy = py.clamp(rect.y0 + radius, rect.y1 - radius);
    let (dx, dy) = (px - cx, py - cy);
    dx * dx + dy * dy <= radius * radius
}

/// A tiny CPU canvas over the RGBA face buffer — just the vocabulary the
/// layouts need. Shapes write hard-edged pixels (crisp at overlay sizes);
/// labels alpha-blend over them.
struct Canvas<'a> {
    data: &'a mut [u8],
    w: usize,
    h: usize,
}

impl Canvas<'_> {
    fn set(&mut self, x: usize, y: usize, color: [u8; 4]) {
        if x < self.w && y < self.h {
            let at = (y * self.w + x) * 4;
            self.data[at..at + 4].copy_from_slice(&color);
        }
    }

    /// Scan `rect`'s bounding box, painting pixels whose centers pass `hit`.
    fn paint(&mut self, rect: RectF, color: [u8; 4], hit: impl Fn(f32, f32) -> bool) {
        let x_lo = rect.x0.floor().max(0.0) as usize;
        let y_lo = rect.y0.floor().max(0.0) as usize;
        let x_hi = (rect.x1.ceil().max(0.0) as usize).min(self.w);
        let y_hi = (rect.y1.ceil().max(0.0) as usize).min(self.h);
        for y in y_lo..y_hi {
            for x in x_lo..x_hi {
                if hit(x as f32 + 0.5, y as f32 + 0.5) {
                    self.set(x, y, color);
                }
            }
        }
    }

    fn fill_round_rect(&mut self, rect: RectF, radius: f32, color: [u8; 4]) {
        self.paint(rect, color, |px, py| in_round_rect(px, py, rect, radius));
    }

    fn stroke_round_rect(&mut self, rect: RectF, radius: f32, thickness: f32, color: [u8; 4]) {
        let inner = RectF {
            x0: rect.x0 + thickness,
            y0: rect.y0 + thickness,
            x1: rect.x1 - thickness,
            y1: rect.y1 - thickness,
        };
        self.paint(rect, color, |px, py| {
            in_round_rect(px, py, rect, radius)
                && !in_round_rect(px, py, inner, (radius - thickness).max(0.0))
        });
    }

    fn fill_circle(&mut self, cx: f32, cy: f32, r: f32, color: [u8; 4]) {
        let rect = RectF {
            x0: cx - r,
            y0: cy - r,
            x1: cx + r,
            y1: cy + r,
        };
        self.paint(rect, color, |px, py| {
            let (dx, dy) = (px - cx, py - cy);
            dx * dx + dy * dy <= r * r
        });
    }

    fn stroke_circle(&mut self, cx: f32, cy: f32, r: f32, thickness: f32, color: [u8; 4]) {
        let rect = RectF {
            x0: cx - r,
            y0: cy - r,
            x1: cx + r,
            y1: cy + r,
        };
        let inner = (r - thickness).max(0.0);
        self.paint(rect, color, |px, py| {
            let (dx, dy) = (px - cx, py - cy);
            let d2 = dx * dx + dy * dy;
            d2 <= r * r && d2 >= inner * inner
        });
    }

    /// Straight-alpha over-blit of a text raster (same math as the split
    /// timer's face compositor).
    fn blit(&mut self, raster: &Frame, x: i64, y: i64) {
        let stride = raster.stride as usize;
        for row in 0..raster.height as usize {
            let dst_y = y + row as i64;
            if dst_y < 0 || dst_y >= self.h as i64 {
                continue;
            }
            for col in 0..raster.width as usize {
                let dst_x = x + col as i64;
                if dst_x < 0 || dst_x >= self.w as i64 {
                    continue;
                }
                let src = row * stride + col * 4;
                let alpha = raster.data[src + 3] as u32;
                if alpha == 0 {
                    continue;
                }
                let dst = (dst_y as usize * self.w + dst_x as usize) * 4;
                for ch in 0..3 {
                    let over = raster.data[src + ch] as u32;
                    let under = self.data[dst + ch] as u32;
                    self.data[dst + ch] = ((over * alpha + under * (255 - alpha)) / 255) as u8;
                }
                let under_a = self.data[dst + 3] as u32;
                self.data[dst + 3] = (alpha + under_a * (255 - alpha) / 255) as u8;
            }
        }
    }
}

/// Rasterized labels, cached per (text, size) — shaping is the only
/// expensive step in this renderer, and every label is a static layout
/// string. One session = one config, so the color never varies in a cache.
#[derive(Default)]
pub struct LabelCache(HashMap<(&'static str, u32), Option<Frame>>);

impl LabelCache {
    fn label(&mut self, text: &'static str, size_px: f32, color: [u8; 4]) -> Option<&Frame> {
        self.0
            .entry((text, size_px as u32))
            .or_insert_with(|| {
                render_text(&TextStyle {
                    text: text.to_string(),
                    font_family: None,
                    font_file: None,
                    size_px,
                    color,
                    align: TextAlign::Left,
                    line_spacing: 1.0,
                    force_rtl: false,
                    wrap_width: None,
                    ..TextStyle::default()
                })
                .ok()
            })
            .as_ref()
    }
}

/// Blit `label` centered on (`cx`, `cy`).
fn center_label(
    canvas: &mut Canvas,
    cache: &mut LabelCache,
    label: &'static str,
    size_px: f32,
    color: [u8; 4],
    cx: f32,
    cy: f32,
) {
    if let Some(raster) = cache.label(label, size_px, color) {
        let x = (cx - raster.width as f32 * 0.5) as i64;
        let y = (cy - raster.height as f32 * 0.5) as i64;
        canvas.blit(raster, x, y);
    }
}

/// Map stick axes (gilrs: X right, Y **up**, unit range) to a screen-space
/// dot offset — clamped to the unit disc, scaled to `travel` px (screen Y
/// grows downward, hence the flip).
fn stick_offset(x: f32, y: f32, travel: f32) -> (f32, f32) {
    let mag = (x * x + y * y).sqrt();
    let scale = if mag > 1.0 { travel / mag } else { travel };
    (x * scale, -y * scale)
}

/// The fight-stick gate: snap a direction to one of 8 unit directions, or
/// center inside the deadzone.
fn snap_8way(x: f32, y: f32) -> (f32, f32) {
    if x * x + y * y < 0.25 {
        return (0.0, 0.0);
    }
    let step = std::f32::consts::FRAC_PI_4;
    let angle = (y.atan2(x) / step).round() * step;
    (angle.cos(), angle.sin())
}

/// Render one face — pure given the sampled state. `keys` follows
/// [`vk_set`]'s order (keyboard caps, then LMB/RMB/MMB).
pub fn render_overlay(
    config: &InputOverlayConfig,
    keys: &[bool],
    pad: &PadState,
    cache: &mut LabelCache,
) -> Vec<u8> {
    let (width, height) = layout_size(config.layout);
    let mut data = vec![0u8; width as usize * height as usize * 4];
    let mut canvas = Canvas {
        data: &mut data,
        w: width as usize,
        h: height as usize,
    };
    match config.layout {
        Layout::Wasd => draw_keyboard_face(&mut canvas, &WASD_PLAN, config, keys, cache),
        Layout::Keyboard => draw_keyboard_face(&mut canvas, &KEYBOARD_PLAN, config, keys, cache),
        Layout::Gamepad => draw_gamepad_face(&mut canvas, config, pad, cache),
        Layout::Fightstick => draw_fightstick_face(&mut canvas, config, pad, cache),
    }
    data
}

fn draw_keyboard_face(
    canvas: &mut Canvas,
    plan: &KeyboardPlan,
    config: &InputOverlayConfig,
    keys: &[bool],
    cache: &mut LabelCache,
) {
    let idle = dim(config.color, 0.16);
    let edge = dim(config.color, 0.5);
    for (index, def) in plan.keys.iter().enumerate() {
        let pressed = keys.get(index).copied().unwrap_or(false);
        let rect = RectF {
            x0: PAD + def.x * CELL + INSET,
            y0: PAD + def.y * CELL + INSET,
            x1: PAD + (def.x + def.w) * CELL - INSET,
            y1: PAD + (def.y + 1.0) * CELL - INSET,
        };
        if pressed {
            canvas.fill_round_rect(rect, CAP_RADIUS, config.accent);
        } else {
            canvas.fill_round_rect(rect, CAP_RADIUS, idle);
            canvas.stroke_round_rect(rect, CAP_RADIUS, 1.5, edge);
        }
        let size = if def.label.len() > 1 { 13.0 } else { 18.0 };
        center_label(
            canvas,
            cache,
            def.label,
            size,
            config.color,
            (rect.x0 + rect.x1) * 0.5,
            (rect.y0 + rect.y1) * 0.5,
        );
    }
    let base = plan.keys.len();
    let button = |index: usize| keys.get(base + index).copied().unwrap_or(false);
    draw_mouse(
        canvas,
        PAD + plan.mouse_x * CELL,
        PAD,
        [button(0), button(1), button(2)],
        config,
    );
}

/// The mouse silhouette: body, left/right button zones, and the wheel.
fn draw_mouse(
    canvas: &mut Canvas,
    ox: f32,
    oy: f32,
    buttons: [bool; 3],
    config: &InputOverlayConfig,
) {
    let idle = dim(config.color, 0.16);
    let edge = dim(config.color, 0.5);
    let body = RectF {
        x0: ox,
        y0: oy,
        x1: ox + MOUSE_W,
        y1: oy + MOUSE_H,
    };
    canvas.fill_round_rect(body, MOUSE_W * 0.45, idle);
    canvas.stroke_round_rect(body, MOUSE_W * 0.45, 1.5, edge);
    // Button zones: the top of the shell, split around a center gap.
    let split_y = oy + MOUSE_H * 0.42;
    let mid = ox + MOUSE_W * 0.5;
    let zones = [
        RectF {
            x0: ox + 3.0,
            y0: oy + 3.0,
            x1: mid - 4.0,
            y1: split_y,
        },
        RectF {
            x0: mid + 4.0,
            y0: oy + 3.0,
            x1: ox + MOUSE_W - 3.0,
            y1: split_y,
        },
    ];
    for (zone, pressed) in zones.iter().zip([buttons[0], buttons[1]]) {
        if pressed {
            canvas.fill_round_rect(*zone, MOUSE_W * 0.35, config.accent);
        } else {
            canvas.stroke_round_rect(*zone, MOUSE_W * 0.35, 1.5, edge);
        }
    }
    // The wheel, riding the gap.
    let wheel = RectF {
        x0: mid - 3.5,
        y0: oy + MOUSE_H * 0.12,
        x1: mid + 3.5,
        y1: oy + MOUSE_H * 0.32,
    };
    canvas.fill_round_rect(wheel, 3.5, if buttons[2] { config.accent } else { edge });
}

/// A labeled round button (face buttons, fight-stick buttons).
fn draw_button(
    canvas: &mut Canvas,
    center: (f32, f32),
    r: f32,
    pressed: bool,
    label: &'static str,
    config: &InputOverlayConfig,
    cache: &mut LabelCache,
) {
    let fill = if pressed {
        config.accent
    } else {
        dim(config.color, 0.16)
    };
    canvas.fill_circle(center.0, center.1, r, fill);
    canvas.stroke_circle(center.0, center.1, r, 1.5, dim(config.color, 0.5));
    center_label(
        canvas,
        cache,
        label,
        r * 0.9,
        config.color,
        center.0,
        center.1,
    );
}

/// An analog stick: the gate ring + a dot offset by the axes.
fn draw_stick(
    canvas: &mut Canvas,
    cx: f32,
    cy: f32,
    axes: (f32, f32),
    clicked: bool,
    config: &InputOverlayConfig,
) {
    canvas.stroke_circle(cx, cy, 38.0, 2.0, dim(config.color, 0.5));
    let (dx, dy) = stick_offset(axes.0, axes.1, 19.0);
    let fill = if clicked {
        config.accent
    } else {
        dim(config.color, 0.35)
    };
    canvas.fill_circle(cx + dx, cy + dy, 17.0, fill);
    canvas.stroke_circle(cx + dx, cy + dy, 17.0, 1.5, dim(config.color, 0.7));
}

/// A trigger's fill bar — pulls fill it downward with the analog value.
fn draw_trigger(canvas: &mut Canvas, x: f32, value: f32, config: &InputOverlayConfig) {
    let rect = RectF {
        x0: x,
        y0: 16.0,
        x1: x + 16.0,
        y1: 76.0,
    };
    canvas.stroke_round_rect(rect, 5.0, 1.5, dim(config.color, 0.5));
    let value = value.clamp(0.0, 1.0);
    if value > 0.01 {
        let fill = RectF {
            x0: x + 2.0,
            y0: 18.0,
            x1: x + 14.0,
            y1: 18.0 + value * 56.0,
        };
        canvas.fill_round_rect(fill, 4.0, config.accent);
    }
}

/// The D-pad cross; pressed arms light up individually.
fn draw_dpad(canvas: &mut Canvas, cx: f32, cy: f32, dirs: [bool; 4], config: &InputOverlayConfig) {
    let (arm, half) = (40.0, 13.0);
    let idle = dim(config.color, 0.16);
    let edge = dim(config.color, 0.5);
    let vertical = RectF {
        x0: cx - half,
        y0: cy - arm,
        x1: cx + half,
        y1: cy + arm,
    };
    let horizontal = RectF {
        x0: cx - arm,
        y0: cy - half,
        x1: cx + arm,
        y1: cy + half,
    };
    canvas.fill_round_rect(vertical, 5.0, idle);
    canvas.fill_round_rect(horizontal, 5.0, idle);
    canvas.stroke_round_rect(vertical, 5.0, 1.5, edge);
    canvas.stroke_round_rect(horizontal, 5.0, 1.5, edge);
    let arms = [
        // up, down, left, right — matching `dirs`.
        RectF {
            x0: cx - half,
            y0: cy - arm,
            x1: cx + half,
            y1: cy - half,
        },
        RectF {
            x0: cx - half,
            y0: cy + half,
            x1: cx + half,
            y1: cy + arm,
        },
        RectF {
            x0: cx - arm,
            y0: cy - half,
            x1: cx - half,
            y1: cy + half,
        },
        RectF {
            x0: cx + half,
            y0: cy - half,
            x1: cx + arm,
            y1: cy + half,
        },
    ];
    for (rect, pressed) in arms.iter().zip(dirs) {
        if pressed {
            canvas.fill_round_rect(*rect, 3.0, config.accent);
        }
    }
}

/// The dual-stick pad: triggers, bumpers, sticks, D-pad, face buttons,
/// select/start.
fn draw_gamepad_face(
    canvas: &mut Canvas,
    config: &InputOverlayConfig,
    pad: &PadState,
    cache: &mut LabelCache,
) {
    let idle = dim(config.color, 0.16);
    let edge = dim(config.color, 0.5);
    draw_trigger(canvas, 70.0, pad.lt, config);
    draw_trigger(canvas, 434.0, pad.rt, config);
    for (x, pressed, label) in [(40.0, pad.lb, "LB"), (390.0, pad.rb, "RB")] {
        let rect = RectF {
            x0: x,
            y0: 84.0,
            x1: x + 90.0,
            y1: 106.0,
        };
        if pressed {
            canvas.fill_round_rect(rect, 9.0, config.accent);
        } else {
            canvas.fill_round_rect(rect, 9.0, idle);
            canvas.stroke_round_rect(rect, 9.0, 1.5, edge);
        }
        center_label(canvas, cache, label, 13.0, config.color, x + 45.0, 95.0);
    }
    draw_stick(canvas, 130.0, 170.0, (pad.lx, pad.ly), pad.l3, config);
    draw_stick(canvas, 325.0, 226.0, (pad.rx, pad.ry), pad.r3, config);
    draw_dpad(
        canvas,
        195.0,
        226.0,
        [pad.dpad_up, pad.dpad_down, pad.dpad_left, pad.dpad_right],
        config,
    );
    for (dx, dy, pressed, label) in [
        (0.0, -30.0, pad.north, "Y"),
        (30.0, 0.0, pad.east, "B"),
        (0.0, 30.0, pad.south, "A"),
        (-30.0, 0.0, pad.west, "X"),
    ] {
        draw_button(
            canvas,
            (390.0 + dx, 170.0 + dy),
            15.0,
            pressed,
            label,
            config,
            cache,
        );
    }
    for (x, pressed) in [(222.0, pad.select), (270.0, pad.start)] {
        let rect = RectF {
            x0: x,
            y0: 163.0,
            x1: x + 28.0,
            y1: 177.0,
        };
        if pressed {
            canvas.fill_round_rect(rect, 7.0, config.accent);
        } else {
            canvas.stroke_round_rect(rect, 7.0, 1.5, edge);
        }
    }
}

/// The arcade stick: an 8-way gated ball top + the classic two button rows.
fn draw_fightstick_face(
    canvas: &mut Canvas,
    config: &InputOverlayConfig,
    pad: &PadState,
    cache: &mut LabelCache,
) {
    // Fight sticks usually report the lever as a hat (D-pad); fall back to
    // the left stick, snapped to the same 8-way gate either way.
    let dir = if pad.dpad_up || pad.dpad_down || pad.dpad_left || pad.dpad_right {
        (
            (i32::from(pad.dpad_right) - i32::from(pad.dpad_left)) as f32,
            (i32::from(pad.dpad_up) - i32::from(pad.dpad_down)) as f32,
        )
    } else {
        (pad.lx, pad.ly)
    };
    let (sx, sy) = snap_8way(dir.0, dir.1);
    canvas.stroke_circle(120.0, 120.0, 52.0, 2.0, dim(config.color, 0.5));
    let moved = sx != 0.0 || sy != 0.0;
    let ball = if moved {
        config.accent
    } else {
        dim(config.color, 0.35)
    };
    // Screen Y grows downward — the gate flip mirrors the stick draw.
    let (bx, by) = (120.0 + sx * 30.0, 120.0 - sy * 30.0);
    canvas.fill_circle(bx, by, 22.0, ball);
    canvas.stroke_circle(bx, by, 22.0, 1.5, dim(config.color, 0.7));
    for (cx, cy, pressed, label) in [
        (250.0, 86.0, pad.west, "X"),
        (310.0, 74.0, pad.north, "Y"),
        (370.0, 74.0, pad.rb, "RB"),
        (430.0, 86.0, pad.lb, "LB"),
        (250.0, 156.0, pad.south, "A"),
        (310.0, 144.0, pad.east, "B"),
        (370.0, 144.0, pad.rt > 0.5, "RT"),
        (430.0, 156.0, pad.lt > 0.5, "LT"),
    ] {
        draw_button(canvas, (cx, cy), 18.0, pressed, label, config, cache);
    }
}

// ---------------------------------------------------------------------------
// Sampling + the session
// ---------------------------------------------------------------------------

/// Drain pending events (gilrs refreshes its cached state through them),
/// then read the FIRST connected controller. Axes are quantized to 1/64 so
/// sensor noise doesn't defeat the repaint gate.
fn sample_pad(gilrs: &mut gilrs::Gilrs) -> PadState {
    while gilrs.next_event().is_some() {}
    let Some((_, pad)) = gilrs.gamepads().next() else {
        return PadState::default();
    };
    use gilrs::{Axis, Button};
    let trigger =
        |button: Button| -> f32 { pad.button_data(button).map_or(0.0, |data| data.value()) };
    let quantize = |value: f32| (value.clamp(-1.0, 1.0) * 64.0).round() / 64.0;
    PadState {
        connected: true,
        south: pad.is_pressed(Button::South),
        east: pad.is_pressed(Button::East),
        west: pad.is_pressed(Button::West),
        north: pad.is_pressed(Button::North),
        lb: pad.is_pressed(Button::LeftTrigger),
        rb: pad.is_pressed(Button::RightTrigger),
        l3: pad.is_pressed(Button::LeftThumb),
        r3: pad.is_pressed(Button::RightThumb),
        select: pad.is_pressed(Button::Select),
        start: pad.is_pressed(Button::Start),
        dpad_up: pad.is_pressed(Button::DPadUp),
        dpad_down: pad.is_pressed(Button::DPadDown),
        dpad_left: pad.is_pressed(Button::DPadLeft),
        dpad_right: pad.is_pressed(Button::DPadRight),
        lx: quantize(pad.value(Axis::LeftStickX)),
        ly: quantize(pad.value(Axis::LeftStickY)),
        rx: quantize(pad.value(Axis::RightStickX)),
        ry: quantize(pad.value(Axis::RightStickY)),
        lt: quantize(trigger(Button::LeftTrigger2)),
        rt: quantize(trigger(Button::RightTrigger2)),
    }
}

/// Start the input-overlay session thread.
///
/// The privacy contract is enforced by scope: all input reading happens on
/// this thread, so it starts with the session and stops with it. Controller
/// init failure (no backend, no permission) draws the honest unpressed
/// layout instead of erroring the source.
pub fn start_input_overlay(config: InputOverlayConfig) -> Result<CaptureSession, CaptureError> {
    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-inputoverlay".into())
        .spawn(move || {
            // Keyboard layouts never touch the controller API, and pad
            // layouts never poll a key — each preset reads exactly its own
            // fixed inputs, nothing more.
            let mut pad_backend = match config.layout {
                Layout::Gamepad | Layout::Fightstick => gilrs::Gilrs::new().ok(),
                Layout::Wasd | Layout::Keyboard => None,
            };
            let vks = vk_set(config.layout);
            let (width, height) = layout_size(config.layout);
            let mut cache = LabelCache::default();
            let period = Duration::from_micros(1_000_000 / u64::from(FPS));
            let mut next = Instant::now();
            let mut last_shown: Option<(Vec<bool>, PadState)> = None;
            loop {
                if thread_stop.load(Ordering::Relaxed) || !sender.is_open() {
                    return;
                }
                // Off-Windows the keyboard sampler returns None — the caps
                // draw unpressed (said honestly in the picker), never a fake.
                let keys = fcap_capture::keys_down(&vks).unwrap_or_else(|| vec![false; vks.len()]);
                let pad = pad_backend.as_mut().map(sample_pad).unwrap_or_default();
                let shown = (keys, pad);
                // Repaint only when the sampled state moved — an untouched
                // overlay paints once and costs nothing per frame.
                if last_shown.as_ref() != Some(&shown) {
                    let face = render_overlay(&config, &shown.0, &shown.1, &mut cache);
                    sender.send(rgba_frame(width, height, face));
                    last_shown = Some(shown);
                }
                next += period;
                let now = Instant::now();
                if next > now {
                    std::thread::sleep(next - now);
                } else {
                    next = now; // fell behind — never burst to catch up
                }
            }
        })
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(layout: Layout) -> InputOverlayConfig {
        InputOverlayConfig {
            layout,
            color: [255, 255, 255, 255],
            accent: [255, 0, 0, 255],
        }
    }

    fn pixel(data: &[u8], width: u32, x: u32, y: u32) -> [u8; 4] {
        let at = ((y * width + x) * 4) as usize;
        [data[at], data[at + 1], data[at + 2], data[at + 3]]
    }

    #[test]
    fn a_pressed_key_cap_fills_with_the_accent_over_transparency() {
        let config = config(Layout::Wasd);
        let mut cache = LabelCache::default();
        let (width, _) = layout_size(Layout::Wasd);
        let mut keys = vec![false; vk_set(Layout::Wasd).len()];
        let idle = render_overlay(&config, &keys, &PadState::default(), &mut cache);
        keys[0] = true; // W — the first WASD key.
        let pressed = render_overlay(&config, &keys, &PadState::default(), &mut cache);
        // Inside W's cap (cell 1,0), off-center so the glyph can't cover it.
        let (x, y) = (
            (PAD + CELL + INSET + 6.0) as u32,
            (PAD + INSET + 6.0) as u32,
        );
        assert_eq!(pixel(&pressed, width, x, y), [255, 0, 0, 255], "accent");
        assert!(
            pixel(&idle, width, x, y)[3] < 64,
            "idle cap is a faint fill"
        );
        // The canvas corner stays transparent either way.
        assert_eq!(pixel(&idle, width, 0, 0)[3], 0);
        assert_eq!(pixel(&pressed, width, 0, 0)[3], 0);
    }

    #[test]
    fn the_mouse_left_button_zone_lights_up() {
        let config = config(Layout::Wasd);
        let mut cache = LabelCache::default();
        let (width, _) = layout_size(Layout::Wasd);
        let mut keys = vec![false; vk_set(Layout::Wasd).len()];
        let idle = render_overlay(&config, &keys, &PadState::default(), &mut cache);
        keys[WASD_KEYS.len()] = true; // LMB follows the keyboard caps.
        let clicked = render_overlay(&config, &keys, &PadState::default(), &mut cache);
        // The left button zone's center.
        let ox = PAD + WASD_PLAN.mouse_x * CELL;
        let (x, y) = (
            ((ox + 3.0 + ox + MOUSE_W * 0.5 - 4.0) * 0.5) as u32,
            ((PAD + 3.0 + PAD + MOUSE_H * 0.42) * 0.5) as u32,
        );
        assert_eq!(pixel(&clicked, width, x, y), [255, 0, 0, 255]);
        assert_ne!(pixel(&idle, width, x, y), [255, 0, 0, 255]);
    }

    #[test]
    fn stick_offset_scales_and_clamps_to_the_disc() {
        assert_eq!(stick_offset(0.0, 0.0, 19.0), (0.0, 0.0));
        assert_eq!(stick_offset(1.0, 0.0, 19.0), (19.0, 0.0));
        // Y up in axis space is up on screen (negative Y).
        assert_eq!(stick_offset(0.0, 1.0, 19.0), (0.0, -19.0));
        // Magnitude 5 clamps to the disc: direction kept, length = travel.
        let (dx, dy) = stick_offset(3.0, 4.0, 10.0);
        assert!(
            (dx - 6.0).abs() < 1e-4 && (dy + 8.0).abs() < 1e-4,
            "{dx},{dy}"
        );
    }

    #[test]
    fn snap_8way_gates_to_diagonals_and_center() {
        assert_eq!(snap_8way(0.1, 0.1), (0.0, 0.0), "deadzone");
        assert_eq!(snap_8way(0.9, 0.05), (1.0, 0.0), "cardinal");
        let (dx, dy) = snap_8way(0.7, 0.7);
        let diag = std::f32::consts::FRAC_1_SQRT_2;
        assert!((dx - diag).abs() < 1e-4 && (dy - diag).abs() < 1e-4);
        let (dx, dy) = snap_8way(0.0, -0.9);
        assert!(dx.abs() < 1e-4 && (dy + 1.0).abs() < 1e-4, "down");
    }

    #[test]
    fn gamepad_triggers_fill_with_the_pull_over_transparency() {
        let config = config(Layout::Gamepad);
        let mut cache = LabelCache::default();
        let (width, _) = layout_size(Layout::Gamepad);
        let released = render_overlay(&config, &[], &PadState::default(), &mut cache);
        let pad = PadState {
            connected: true,
            rt: 1.0,
            ..PadState::default()
        };
        let pulled = render_overlay(&config, &[], &pad, &mut cache);
        // Inside the right trigger's fill area (bar at x 434..450).
        assert_eq!(pixel(&pulled, width, 441, 45), [255, 0, 0, 255]);
        assert_eq!(pixel(&released, width, 441, 45)[3], 0, "empty bar");
        assert_eq!(pixel(&released, width, 0, 0)[3], 0, "transparent corner");
    }

    #[test]
    fn the_fightstick_ball_follows_the_gate() {
        let config = config(Layout::Fightstick);
        let mut cache = LabelCache::default();
        let (width, _) = layout_size(Layout::Fightstick);
        let centered = render_overlay(&config, &[], &PadState::default(), &mut cache);
        let pad = PadState {
            connected: true,
            dpad_right: true,
            ..PadState::default()
        };
        let held = render_overlay(&config, &[], &pad, &mut cache);
        // The ball's resting spot sits at (120,120); held right it moves to
        // (150,120) and lights accent.
        assert_eq!(pixel(&held, width, 150, 120), [255, 0, 0, 255]);
        assert_eq!(pixel(&centered, width, 150, 120)[3], 0, "gate interior");
    }

    /// Pins the privacy scope: keyboard layouts poll exactly their fixed VK
    /// set (each key once), and pad layouts poll no keys at all.
    #[test]
    fn vk_sets_are_fixed_unique_and_empty_for_pads() {
        for layout in [Layout::Wasd, Layout::Keyboard] {
            let vks = vk_set(layout);
            let unique: std::collections::HashSet<u16> = vks.iter().copied().collect();
            assert_eq!(unique.len(), vks.len(), "{layout:?} has duplicate VKs");
            assert!(vks.contains(&0x01) && vks.contains(&0x02), "mouse buttons");
        }
        assert!(vk_set(Layout::Gamepad).is_empty());
        assert!(vk_set(Layout::Fightstick).is_empty());
    }
}
