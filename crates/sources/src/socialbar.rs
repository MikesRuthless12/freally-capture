//! The "social & channels bar" painter (V1-D).
//!
//! A [`SourceSettings::SocialBar`](fcap_scene::SourceSettings) renders a tidy
//! vertical panel — a semi-transparent rounded card with an optional header
//! line, then one row per social account: a small brand-coloured rounded badge
//! followed by the platform name and the creator's handle. It is a purely
//! static face: fully local, CPU-composed, with nothing fetched, decoded, or
//! read off disk. No real brand logos are embedded — a coloured badge *is* the
//! design (the studio maps each platform to its brand colour + name).
//!
//! Text reuses [`crate::text::render_text`] (the shared shaping / bidi
//! pipeline) and everything is composited with the straight-alpha
//! [`crate::compose`] helpers, exactly like the countdown slate and the title
//! designer.

use std::path::PathBuf;

use fcap_capture::Frame;

use crate::compose::{blit, fill_round_rect};
use crate::static_source::{check_dimension, rgba_frame, StaticSourceError};
use crate::text::{render_text, TextStyle};

/// Rows past this are dropped — a real social bar lists a handful of accounts,
/// and the cap keeps a mistyped scene file from allocating a giant face.
const MAX_ROWS: usize = 32;

/// One resolved account row: the badge colour and the finished line of text
/// (the studio has already combined the platform name with the handle).
#[derive(Debug, Clone)]
pub struct SocialBarRow {
    /// The brand-coloured badge fill (straight RGBA, opaque).
    pub badge: [u8; 4],
    /// The line shown to the right of the badge, e.g. `YouTube  @mychannel`.
    pub text: String,
}

/// Everything the painter needs (the studio maps the scene model to this).
#[derive(Debug, Clone)]
pub struct SocialBarStyle {
    /// The header line above the rows (empty = no header).
    pub header: String,
    /// System font family; `None` = the bundled default face. A family *name*,
    /// never a path — this source reads nothing off disk.
    pub font_family: Option<String>,
    /// The handle/label type size, px.
    pub size_px: f32,
    /// The header + text colour (straight RGBA).
    pub color: [u8; 4],
    /// The panel's semi-transparent background (straight RGBA).
    pub background: [u8; 4],
    pub rows: Vec<SocialBarRow>,
}

/// Paint the social bar to a content-sized RGBA frame. An empty bar (no header,
/// no rows) yields a tiny transparent frame rather than an error, so a
/// half-configured source never shows a red error card.
pub fn render_social_bar(style: &SocialBarStyle) -> Result<Frame, StaticSourceError> {
    let size_px = style.size_px.clamp(8.0, 512.0);

    // Rasterise the header (when non-blank) and every non-blank row line first;
    // their measured sizes drive the whole layout.
    let header = if style.header.trim().is_empty() {
        None
    } else {
        Some(render_text(&text_style(style, &style.header, size_px))?)
    };

    let mut rows: Vec<(Frame, [u8; 4])> = Vec::new();
    for row in style.rows.iter().take(MAX_ROWS) {
        if row.text.trim().is_empty() {
            continue;
        }
        rows.push((
            render_text(&text_style(style, &row.text, size_px))?,
            row.badge,
        ));
    }

    if header.is_none() && rows.is_empty() {
        // Nothing to show yet — a clear 2×2 placeholder (valid, invisible).
        return Ok(rgba_frame(2, 2, vec![0u8; 2 * 2 * 4]));
    }

    // Spacing, all derived from the type size so the card scales as one piece.
    let pad = (size_px * 0.5).round() as usize;
    let row_gap = (size_px * 0.35).round() as usize;
    let header_gap = (size_px * 0.4).round() as usize;
    let badge_gap = (size_px * 0.35).round() as usize;

    // Per-row badge is a square a little shorter than the text line, and the
    // row is as tall as its text.
    let badge_side = |line: &Frame| ((line.height as f32) * 0.72).round().max(4.0) as usize;

    let row_width = |line: &Frame| badge_side(line) + badge_gap + line.width as usize;
    let content_width = header.as_ref().map_or(0, |h| h.width as usize).max(
        rows.iter()
            .map(|(line, _)| row_width(line))
            .max()
            .unwrap_or(0),
    );
    let content_height = header
        .as_ref()
        .map_or(0, |h| h.height as usize + header_gap)
        + rows
            .iter()
            .map(|(line, _)| line.height as usize)
            .sum::<usize>()
        + row_gap * rows.len().saturating_sub(1);

    let panel_w = content_width + pad * 2;
    let panel_h = content_height + pad * 2;
    let width = panel_w as u32;
    let height = panel_h as u32;
    check_dimension("social bar width", width)?;
    check_dimension("social bar height", height)?;

    let mut buf = vec![0u8; panel_w * panel_h * 4];
    // 1. The panel: a rounded, semi-transparent card over the transparent
    //    canvas (copy — the buffer starts clear, so the stored colour stays
    //    straight; the compositor blends it over the video once).
    let panel_radius = (size_px * 0.35).round() as usize;
    fill_round_rect(
        &mut buf,
        panel_w,
        (0, 0, panel_w, panel_h),
        panel_radius,
        style.background,
    );

    // 2. Content, top to bottom, inset by the padding.
    let mut y = pad;
    if let Some(header) = &header {
        blit(&mut buf, panel_w, panel_h, header, pad as i64, y as i64);
        y += header.height as usize + header_gap;
    }
    for (line, badge) in &rows {
        let row_h = line.height as usize;
        let side = badge_side(line);
        // Centre the badge vertically against its text line.
        let badge_y = y + row_h.saturating_sub(side) / 2;
        let badge_radius = (side as f32 * 0.28).round() as usize;
        fill_round_rect(
            &mut buf,
            panel_w,
            (pad, badge_y, pad + side, badge_y + side),
            badge_radius,
            *badge,
        );
        blit(
            &mut buf,
            panel_w,
            panel_h,
            line,
            (pad + side + badge_gap) as i64,
            y as i64,
        );
        y += row_h + row_gap;
    }

    Ok(rgba_frame(width, height, buf))
}

/// A left-aligned text style for the header / a row line.
fn text_style(style: &SocialBarStyle, text: &str, size_px: f32) -> TextStyle {
    TextStyle {
        text: text.to_string(),
        font_family: style.font_family.clone(),
        font_file: None::<PathBuf>,
        size_px,
        color: style.color,
        // A soft shadow keeps the handles legible over a busy video behind the
        // translucent panel.
        shadow_px: (size_px * 0.05).clamp(1.0, 8.0),
        shadow_color: [0, 0, 0, 160],
        ..TextStyle::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn px(frame: &Frame, x: u32, y: u32) -> [u8; 4] {
        let at = (y * frame.stride + x * 4) as usize;
        [
            frame.data[at],
            frame.data[at + 1],
            frame.data[at + 2],
            frame.data[at + 3],
        ]
    }

    fn bar(header: &str, rows: Vec<SocialBarRow>) -> SocialBarStyle {
        SocialBarStyle {
            header: header.to_string(),
            font_family: None,
            size_px: 32.0,
            color: [255, 255, 255, 255],
            background: [10, 10, 15, 184],
            rows,
        }
    }

    /// An empty bar renders a clear placeholder, never an error card.
    #[test]
    fn empty_bar_is_a_transparent_placeholder() {
        let frame = render_social_bar(&bar("", vec![])).expect("renders");
        assert!(frame.data.iter().all(|&b| b == 0), "fully transparent");
    }

    /// A populated bar produces a semi-transparent panel with rounded (clear)
    /// corners and paints each row's brand badge in its colour.
    #[test]
    fn populated_bar_has_a_rounded_panel_and_brand_badges() {
        let frame = render_social_bar(&bar(
            "Follow me",
            vec![
                SocialBarRow {
                    badge: [255, 0, 0, 255],
                    text: "YouTube  @mychannel".into(),
                },
                SocialBarRow {
                    badge: [83, 252, 24, 255],
                    text: "Kick  @mychannel".into(),
                },
            ],
        ))
        .expect("renders");
        assert!(frame.width > 0 && frame.height > 0);
        // The very corner is outside the rounded arc → clear.
        assert_eq!(px(&frame, 0, 0)[3], 0, "top-left corner rounds away");
        // The panel centre is the semi-transparent background.
        let mid = px(&frame, frame.width / 2, frame.height / 2);
        assert!(mid[3] > 0 && mid[3] < 255, "panel is translucent");
        // Somewhere on the frame the red YouTube badge is painted opaque.
        let has_red = frame
            .data
            .chunks_exact(4)
            .any(|c| c[0] > 200 && c[1] < 60 && c[2] < 60 && c[3] == 255);
        assert!(has_red, "the YouTube brand badge shows");
    }

    /// Blank-handle rows never reach a badge — the painter skips empty text.
    #[test]
    fn blank_rows_are_skipped() {
        let only_blanks = render_social_bar(&bar(
            "",
            vec![SocialBarRow {
                badge: [255, 0, 0, 255],
                text: "   ".into(),
            }],
        ))
        .expect("renders");
        assert!(only_blanks.data.iter().all(|&b| b == 0), "nothing drawn");
    }
}
