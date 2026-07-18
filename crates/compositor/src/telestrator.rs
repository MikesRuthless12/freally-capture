//! Telestrator (CAP-N57): live free-hand annotation baked into the program.
//!
//! The host draws over the live program — pen / highlighter / arrow / ellipse —
//! and the marks are composited ON TOP of the finished program (the same
//! [`crate::Compositor::render_downstream`] pattern), so they reach preview,
//! recording, and stream alike. Strokes fade after a set time or persist
//! (whiteboard mode). This module owns the pure, deterministic geometry: a
//! list of strokes (in canvas-normalized coordinates) tessellates to a
//! solid-colored triangle-list vertex buffer the compositor draws in one pass.

/// Which drawing tool produced a stroke.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeleTool {
    /// Pressure-varying free-hand line.
    Pen,
    /// Constant-width, translucent free-hand line (a highlighter).
    Highlight,
    /// A straight arrow from the first point to the last.
    Arrow,
    /// An ellipse outline bounded by the first and last points.
    Ellipse,
}

/// One sampled pointer position along a stroke. Coordinates are normalized to
/// the canvas (`0..=1`, top-left origin); `pressure` is the tablet/pen pressure
/// (`0..=1`, `1.0` for a plain mouse).
#[derive(Debug, Clone, Copy)]
pub struct TelePoint {
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
}

/// One completed telestrator stroke, ready to composite over the program.
#[derive(Debug, Clone)]
pub struct TeleStroke {
    pub tool: TeleTool,
    /// Straight-alpha RGBA, each channel `0..=1`.
    pub color: [f32; 4],
    /// Line width as a fraction of the canvas height (resolution-independent).
    pub width: f32,
    /// Sampled path, in canvas-normalized coordinates.
    pub points: Vec<TelePoint>,
    /// Seconds after `born_seconds` at which the stroke starts fading out;
    /// `None` = persistent (whiteboard mode — never fades on its own).
    pub fade_after: Option<f32>,
    /// The telestrator clock time (seconds) at which the stroke was drawn.
    pub born_seconds: f64,
}

/// How long (seconds) a fading stroke takes to fade from full to gone once its
/// `fade_after` window elapses.
pub const FADE_DURATION: f32 = 0.6;

/// A single solid-colored vertex of the telestrator triangle list, mirrored by
/// `shaders/telestrator.wgsl`. Position is in clip space; color is straight-alpha.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct TeleVertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
}

/// The alpha multiplier for a stroke at telestrator time `now`: `1.0` until its
/// fade window elapses, then a linear ramp to `0.0` over [`FADE_DURATION`].
/// Persistent (whiteboard) strokes are always fully opaque.
pub fn stroke_alpha(stroke: &TeleStroke, now: f64) -> f32 {
    match stroke.fade_after {
        None => 1.0,
        Some(after) => {
            let age = (now - stroke.born_seconds) as f32;
            if age <= after {
                1.0
            } else {
                (1.0 - (age - after) / FADE_DURATION.max(1e-3)).clamp(0.0, 1.0)
            }
        }
    }
}

/// Whether a stroke has fully faded and can be dropped from the live set.
pub fn stroke_expired(stroke: &TeleStroke, now: f64) -> bool {
    stroke.fade_after.is_some() && stroke_alpha(stroke, now) <= 0.0
}

/// Convert a canvas-pixel position to clip space (`-1..=1`, y up).
fn to_clip(px: f32, py: f32, cw: f32, ch: f32) -> [f32; 2] {
    [px / cw * 2.0 - 1.0, 1.0 - py / ch * 2.0]
}

/// Push two triangles for a quad given four clip-space corners in winding order.
fn push_quad(out: &mut Vec<TeleVertex>, corners: [[f32; 2]; 4], color: [f32; 4]) {
    let v = |pos: [f32; 2]| TeleVertex { pos, color };
    out.push(v(corners[0]));
    out.push(v(corners[1]));
    out.push(v(corners[2]));
    out.push(v(corners[0]));
    out.push(v(corners[2]));
    out.push(v(corners[3]));
}

/// Push a segment (a thick line) between two canvas-pixel points with per-end
/// half-widths, plus round-ish caps so joints don't gap. `canvas` is `[w, h]`.
fn push_segment(
    out: &mut Vec<TeleVertex>,
    a: [f32; 2],
    b: [f32; 2],
    ha: f32,
    hb: f32,
    canvas: [f32; 2],
    color: [f32; 4],
) {
    let (cw, ch) = (canvas[0], canvas[1]);
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let len = (dx * dx + dy * dy).sqrt();
    if len >= 1e-4 {
        let (nx, ny) = (-dy / len, dx / len);
        push_quad(
            out,
            [
                to_clip(a[0] + nx * ha, a[1] + ny * ha, cw, ch),
                to_clip(b[0] + nx * hb, b[1] + ny * hb, cw, ch),
                to_clip(b[0] - nx * hb, b[1] - ny * hb, cw, ch),
                to_clip(a[0] - nx * ha, a[1] - ny * ha, cw, ch),
            ],
            color,
        );
    }
    // A square cap at the far end covers the joint to the next segment.
    push_dot(out, b, hb, cw, ch, color);
}

/// Push a small square "dot" centered on a canvas-pixel point (a cap / a tap).
fn push_dot(out: &mut Vec<TeleVertex>, p: [f32; 2], r: f32, cw: f32, ch: f32, color: [f32; 4]) {
    if r <= 0.0 {
        return;
    }
    push_quad(
        out,
        [
            to_clip(p[0] - r, p[1] - r, cw, ch),
            to_clip(p[0] + r, p[1] - r, cw, ch),
            to_clip(p[0] + r, p[1] + r, cw, ch),
            to_clip(p[0] - r, p[1] + r, cw, ch),
        ],
        color,
    );
}

/// Tessellate every stroke into one solid-colored triangle list, in clip space,
/// at telestrator time `now`. Pure and deterministic: identical inputs always
/// yield an identical vertex sequence (no clock, no randomness, no map order),
/// which is what the stroke-replay determinism test relies on.
pub(crate) fn tessellate(
    strokes: &[TeleStroke],
    now: f64,
    canvas_w: u32,
    canvas_h: u32,
) -> Vec<TeleVertex> {
    let cw = canvas_w.max(1) as f32;
    let ch = canvas_h.max(1) as f32;
    let mut out: Vec<TeleVertex> = Vec::new();
    for stroke in strokes {
        let alpha = stroke_alpha(stroke, now);
        if alpha <= 0.0 || stroke.points.is_empty() {
            continue;
        }
        let color = [
            stroke.color[0],
            stroke.color[1],
            stroke.color[2],
            (stroke.color[3] * alpha).clamp(0.0, 1.0),
        ];
        // Half-width in canvas pixels (width is a fraction of canvas height).
        let base = (stroke.width.clamp(0.0005, 0.2) * ch * 0.5).max(0.5);
        // Points in canvas-pixel space, with clamped coordinates + pressure.
        let pts: Vec<([f32; 2], f32)> = stroke
            .points
            .iter()
            .map(|p| {
                let pr = p.pressure.clamp(0.05, 1.0);
                ([p.x.clamp(0.0, 1.0) * cw, p.y.clamp(0.0, 1.0) * ch], pr)
            })
            .collect();

        match stroke.tool {
            TeleTool::Pen | TeleTool::Highlight => {
                let pressure_varies = matches!(stroke.tool, TeleTool::Pen);
                let half = |pr: f32| if pressure_varies { base * pr } else { base };
                if pts.len() == 1 {
                    push_dot(&mut out, pts[0].0, half(pts[0].1), cw, ch, color);
                } else {
                    push_dot(&mut out, pts[0].0, half(pts[0].1), cw, ch, color);
                    for pair in pts.windows(2) {
                        push_segment(
                            &mut out,
                            pair[0].0,
                            pair[1].0,
                            half(pair[0].1),
                            half(pair[1].1),
                            [cw, ch],
                            color,
                        );
                    }
                }
            }
            TeleTool::Arrow => {
                let a = pts[0].0;
                let b = pts[pts.len() - 1].0;
                let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
                let len = (dx * dx + dy * dy).sqrt();
                if len < 1e-3 {
                    push_dot(&mut out, a, base, cw, ch, color);
                } else {
                    let (ux, uy) = (dx / len, dy / len);
                    // Arrowhead sized to the line, clamped to the shaft length.
                    let head = (base * 6.0).min(len * 0.6);
                    let hw = head * 0.6;
                    let tip = b;
                    let neck = [b[0] - ux * head, b[1] - uy * head];
                    // Shaft stops at the neck so the head is solid.
                    push_segment(&mut out, a, neck, base, base, [cw, ch], color);
                    let (nx, ny) = (-uy, ux);
                    let left = [neck[0] + nx * hw, neck[1] + ny * hw];
                    let right = [neck[0] - nx * hw, neck[1] - ny * hw];
                    let v = |p: [f32; 2]| TeleVertex {
                        pos: to_clip(p[0], p[1], cw, ch),
                        color,
                    };
                    out.push(v(tip));
                    out.push(v(left));
                    out.push(v(right));
                }
            }
            TeleTool::Ellipse => {
                let a = pts[0].0;
                let b = pts[pts.len() - 1].0;
                let cx = (a[0] + b[0]) * 0.5;
                let cy = (a[1] + b[1]) * 0.5;
                let rx = ((b[0] - a[0]) * 0.5).abs();
                let ry = ((b[1] - a[1]) * 0.5).abs();
                const SEGMENTS: usize = 64;
                for i in 0..SEGMENTS {
                    let t0 = (i as f32) / SEGMENTS as f32 * std::f32::consts::TAU;
                    let t1 = ((i + 1) as f32) / SEGMENTS as f32 * std::f32::consts::TAU;
                    let ring = |t: f32, r: f32| [cx + (rx + r) * t.cos(), cy + (ry + r) * t.sin()];
                    push_quad(
                        &mut out,
                        [
                            to_clip(ring(t0, -base)[0], ring(t0, -base)[1], cw, ch),
                            to_clip(ring(t1, -base)[0], ring(t1, -base)[1], cw, ch),
                            to_clip(ring(t1, base)[0], ring(t1, base)[1], cw, ch),
                            to_clip(ring(t0, base)[0], ring(t0, base)[1], cw, ch),
                        ],
                        color,
                    );
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pen(points: Vec<TelePoint>, fade_after: Option<f32>, born: f64) -> TeleStroke {
        TeleStroke {
            tool: TeleTool::Pen,
            color: [1.0, 0.0, 0.0, 1.0],
            width: 0.01,
            points,
            fade_after,
            born_seconds: born,
        }
    }

    fn p(x: f32, y: f32) -> TelePoint {
        TelePoint {
            x,
            y,
            pressure: 1.0,
        }
    }

    /// The stroke-replay determinism guarantee (Phase 7 DoD): the same strokes
    /// at the same time tessellate to a byte-identical vertex buffer.
    #[test]
    fn tessellation_is_deterministic() {
        let strokes = vec![
            pen(vec![p(0.1, 0.1), p(0.4, 0.3), p(0.6, 0.5)], None, 0.0),
            TeleStroke {
                tool: TeleTool::Ellipse,
                color: [0.0, 1.0, 0.0, 0.8],
                width: 0.008,
                points: vec![p(0.2, 0.2), p(0.7, 0.6)],
                fade_after: Some(3.0),
                born_seconds: 0.0,
            },
            TeleStroke {
                tool: TeleTool::Arrow,
                color: [1.0, 1.0, 0.0, 1.0],
                width: 0.01,
                points: vec![p(0.1, 0.9), p(0.8, 0.4)],
                fade_after: None,
                born_seconds: 0.0,
            },
        ];
        let a = tessellate(&strokes, 1.0, 1920, 1080);
        let b = tessellate(&strokes, 1.0, 1920, 1080);
        assert_eq!(a, b, "identical inputs must yield identical geometry");
        assert!(!a.is_empty());
        assert_eq!(a.len() % 3, 0, "a triangle list is whole triangles");
        assert!(
            a.iter()
                .all(|v| v.pos.iter().chain(v.color.iter()).all(|f| f.is_finite())),
            "no NaN/inf leaks into the vertex buffer"
        );
    }

    #[test]
    fn a_faded_stroke_contributes_nothing() {
        // fade_after 2s + 0.6s ramp → fully gone by t = 3s.
        let strokes = vec![pen(vec![p(0.1, 0.1), p(0.5, 0.5)], Some(2.0), 0.0)];
        assert!(tessellate(&strokes, 3.0, 1920, 1080).is_empty());
        assert!(stroke_expired(&strokes[0], 3.0));
        // …but it is fully present before the fade window.
        assert!(!tessellate(&strokes, 0.5, 1920, 1080).is_empty());
        assert!(!stroke_expired(&strokes[0], 0.5));
        assert_eq!(stroke_alpha(&strokes[0], 0.5), 1.0);
    }

    #[test]
    fn persistent_strokes_never_fade() {
        let s = pen(vec![p(0.0, 0.0), p(1.0, 1.0)], None, 0.0);
        assert_eq!(stroke_alpha(&s, 10_000.0), 1.0);
        assert!(!stroke_expired(&s, 10_000.0));
    }

    #[test]
    fn a_single_point_still_draws_a_dot() {
        let strokes = vec![pen(vec![p(0.5, 0.5)], None, 0.0)];
        let verts = tessellate(&strokes, 0.0, 100, 100);
        assert_eq!(verts.len(), 6, "one dot = one quad = two triangles");
    }
}
