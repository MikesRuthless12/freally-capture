//! The Text source: shaped, rasterized text.
//!
//! The pipeline reuses the Freally Snipper font concepts: **rustybuzz**
//! shapes each run (real shaping — Arabic joining, ligatures, kerning),
//! **unicode-bidi** orders mixed-direction lines (UAX #9), and the glyph
//! outlines (via rustybuzz's `ttf-parser`) are filled anti-aliased with
//! **tiny-skia**.
//!
//! The **complete Noto Sans family is bundled** (variable fonts — every
//! weight/width — upright + italic, plus Arabic and Hebrew; OFL-1.1, see
//! `fonts/README.md`), so the default face renders identically on every
//! machine. System families stay selectable, an explicit font file
//! overrides, and each run falls back per-script to a bundled face that
//! actually covers its characters (Arabic text never renders as tofu just
//! because the picked family is Latin-only). CJK is not bundled (size) and
//! uses system fonts — said honestly in the fonts README.
//!
//! Word wrap happens on the logical text, then each line is reordered
//! visually — the order the bidi algorithm expects.

use std::collections::HashMap;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

use fcap_capture::Frame;
use rustybuzz::ttf_parser::{GlyphId, OutlineBuilder};
use unicode_bidi::{BidiInfo, Level};

use crate::static_source::{check_dimension, rgba_frame, StaticSourceError, MAX_STATIC_DIMENSION};

/// Horizontal alignment of the rendered lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Everything a text render needs (mirrors the scene model's Text settings).
#[derive(Debug, Clone)]
pub struct TextStyle {
    pub text: String,
    /// Font family; `None` = the bundled Noto Sans (identical everywhere).
    pub font_family: Option<String>,
    /// Explicit font file — overrides `font_family` when set.
    pub font_file: Option<PathBuf>,
    pub size_px: f32,
    /// Straight RGBA.
    pub color: [u8; 4],
    pub align: TextAlign,
    /// Line height multiplier (1.0 = the font's natural spacing).
    pub line_spacing: f32,
    /// Force right-to-left paragraph direction (otherwise auto-detected).
    pub force_rtl: bool,
    /// Word-wrap width in px; `None` = never wrap.
    pub wrap_width: Option<u32>,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            text: String::new(),
            font_family: None,
            font_file: None,
            size_px: 72.0,
            color: [255, 255, 255, 255],
            align: TextAlign::Left,
            line_spacing: 1.0,
            force_rtl: false,
            wrap_width: None,
        }
    }
}

/// Anti-alias breathing room around the ink.
const PAD: f32 = 2.0;

/// The bundled Noto Sans complete family (variable fonts; OFL-1.1 — see
/// `crates/sources/fonts/README.md` for provenance + hashes).
const NOTO_SANS: &[u8] = include_bytes!("../fonts/NotoSans[wdth,wght].ttf");
const NOTO_SANS_ITALIC: &[u8] = include_bytes!("../fonts/NotoSans-Italic[wdth,wght].ttf");
const NOTO_SANS_ARABIC: &[u8] = include_bytes!("../fonts/NotoSansArabic[wdth,wght].ttf");
const NOTO_SANS_HEBREW: &[u8] = include_bytes!("../fonts/NotoSansHebrew[wdth,wght].ttf");

/// Script-fallback faces tried, in order, for runs the primary face does not
/// cover (the upright default first — a niche user family may lack Latin).
const FALLBACK_FONTS: [&[u8]; 3] = [NOTO_SANS, NOTO_SANS_ARABIC, NOTO_SANS_HEBREW];

struct LoadedFont {
    data: Vec<u8>,
    index: u32,
}

fn font_database() -> &'static fontdb::Database {
    static DB: OnceLock<fontdb::Database> = OnceLock::new();
    DB.get_or_init(|| {
        let mut db = fontdb::Database::new();
        // Bundled first — the guaranteed baseline — then whatever the OS has.
        for data in [
            NOTO_SANS,
            NOTO_SANS_ITALIC,
            NOTO_SANS_ARABIC,
            NOTO_SANS_HEBREW,
        ] {
            db.load_font_data(data.to_vec());
        }
        db.load_system_fonts();
        // The default face is the bundled one, not whatever "Arial" resolves
        // to on this machine — identical rendering everywhere.
        db.set_sans_serif_family("Noto Sans");
        tracing::info!(
            faces = db.len(),
            "font database loaded (bundled Noto + system)"
        );
        db
    })
}

fn font_cache() -> &'static Mutex<HashMap<String, Arc<LoadedFont>>> {
    static CACHE: OnceLock<Mutex<HashMap<String, Arc<LoadedFont>>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Resolve the styled font to raw bytes: explicit file > named family >
/// platform sans-serif > any face at all (an honest error only when the
/// machine truly has no fonts).
fn resolve_font(style: &TextStyle) -> Result<Arc<LoadedFont>, StaticSourceError> {
    let key = match (&style.font_file, &style.font_family) {
        (Some(path), _) => format!("file:{}", path.display()),
        (None, Some(family)) => format!("family:{family}"),
        (None, None) => "family:<sans-serif>".to_string(),
    };
    if let Some(found) = font_cache()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(&key)
    {
        return Ok(Arc::clone(found));
    }

    let loaded = if let Some(path) = &style.font_file {
        let data = std::fs::read(path).map_err(|err| StaticSourceError::Io {
            path: path.display().to_string(),
            message: err.to_string(),
        })?;
        LoadedFont { data, index: 0 }
    } else {
        let db = font_database();
        let mut families = Vec::new();
        if let Some(family) = &style.font_family {
            families.push(fontdb::Family::Name(family));
        }
        families.push(fontdb::Family::SansSerif);
        let query = fontdb::Query {
            families: &families,
            weight: fontdb::Weight::NORMAL,
            stretch: fontdb::Stretch::Normal,
            style: fontdb::Style::Normal,
        };
        let id = db
            .query(&query)
            .or_else(|| db.faces().next().map(|face| face.id))
            .ok_or_else(|| {
                StaticSourceError::NoFont("this system exposes no fonts at all".to_string())
            })?;
        db.with_face_data(id, |data, index| LoadedFont {
            data: data.to_vec(),
            index,
        })
        .ok_or_else(|| StaticSourceError::NoFont("the selected face failed to load".to_string()))?
    };

    let loaded = Arc::new(loaded);
    font_cache()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .insert(key, Arc::clone(&loaded));
    Ok(loaded)
}

/// One glyph, positioned relative to its line's left edge and baseline
/// (y grows *up*, font-style; the rasterizer flips). `face` indexes the
/// render's face list — the run's script decides which face shaped it.
struct ShapedGlyph {
    id: u16,
    face: usize,
    x: f32,
    y: f32,
}

/// The face (by index) that actually covers `text`: the primary wins when it
/// can; otherwise the first fallback with a glyph for the run's first
/// letter. A run nobody covers stays on the primary (honest tofu).
fn face_for_text(faces: &[rustybuzz::Face<'_>], text: &str) -> usize {
    let probe = text
        .chars()
        .find(|ch| !ch.is_whitespace() && !ch.is_ascii_punctuation());
    let Some(ch) = probe else { return 0 };
    faces
        .iter()
        .position(|face| face.glyph_index(ch).is_some())
        .unwrap_or(0)
}

/// Shape one direction-uniform run with `faces[face_index]`; returns the
/// run's advance and appends its glyphs.
fn shape_run(
    faces: &[rustybuzz::Face<'_>],
    face_index: usize,
    text: &str,
    rtl: bool,
    scale: f32,
    pen: f32,
    out: &mut Vec<ShapedGlyph>,
) -> f32 {
    let mut buffer = rustybuzz::UnicodeBuffer::new();
    buffer.push_str(text);
    buffer.set_direction(if rtl {
        rustybuzz::Direction::RightToLeft
    } else {
        rustybuzz::Direction::LeftToRight
    });
    buffer.guess_segment_properties();
    let shaped = rustybuzz::shape(&faces[face_index], &[], buffer);
    let mut advance = 0.0;
    for (info, position) in shaped
        .glyph_infos()
        .iter()
        .zip(shaped.glyph_positions().iter())
    {
        out.push(ShapedGlyph {
            id: info.glyph_id as u16,
            face: face_index,
            x: pen + advance + position.x_offset as f32 * scale,
            y: position.y_offset as f32 * scale,
        });
        advance += position.x_advance as f32 * scale;
    }
    advance
}

/// Measure a slice's advance without keeping the glyphs.
fn measure(faces: &[rustybuzz::Face<'_>], text: &str, rtl: bool, scale: f32) -> f32 {
    let mut scratch = Vec::new();
    let face = face_for_text(faces, text);
    shape_run(faces, face, text, rtl, scale, 0.0, &mut scratch)
}

/// Greedy word wrap on the logical paragraph: byte ranges of each line.
fn wrap_paragraph(
    faces: &[rustybuzz::Face<'_>],
    paragraph: &str,
    rtl: bool,
    scale: f32,
    max_width: f32,
) -> Vec<Range<usize>> {
    // Tokens alternate word / whitespace, each with its byte range.
    let mut tokens: Vec<(Range<usize>, bool)> = Vec::new(); // (range, is_space)
    let mut start = 0;
    let mut was_space = None::<bool>;
    for (offset, ch) in paragraph.char_indices() {
        let is_space = ch.is_whitespace();
        match was_space {
            Some(prev) if prev == is_space => {}
            Some(_) => {
                tokens.push((start..offset, was_space.unwrap()));
                start = offset;
                was_space = Some(is_space);
            }
            None => was_space = Some(is_space),
        }
    }
    if !paragraph.is_empty() {
        tokens.push((start..paragraph.len(), was_space.unwrap_or(false)));
    }

    let mut lines = Vec::new();
    let mut line_start = 0usize;
    let mut line_end = 0usize;
    let mut line_width = 0.0f32;
    for (range, is_space) in tokens {
        let width = measure(faces, &paragraph[range.clone()], rtl, scale);
        if !is_space && line_end > line_start && line_width + width > max_width {
            lines.push(line_start..line_end);
            line_start = range.start;
            line_width = 0.0;
        }
        line_end = range.end;
        line_width += width;
    }
    lines.push(line_start..line_end.max(line_start));
    lines
}

struct Line {
    glyphs: Vec<ShapedGlyph>,
    width: f32,
}

/// Feeds glyph outlines (font units, y-up) into a tiny-skia path
/// (canvas px, y-down) at a fixed offset.
struct GlyphOutline {
    builder: tiny_skia::PathBuilder,
    scale: f32,
    dx: f32,
    dy: f32,
}

impl OutlineBuilder for GlyphOutline {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder
            .move_to(self.dx + x * self.scale, self.dy - y * self.scale);
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.builder
            .line_to(self.dx + x * self.scale, self.dy - y * self.scale);
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.builder.quad_to(
            self.dx + x1 * self.scale,
            self.dy - y1 * self.scale,
            self.dx + x * self.scale,
            self.dy - y * self.scale,
        );
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.builder.cubic_to(
            self.dx + x1 * self.scale,
            self.dy - y1 * self.scale,
            self.dx + x2 * self.scale,
            self.dy - y2 * self.scale,
            self.dx + x * self.scale,
            self.dy - y * self.scale,
        );
    }
    fn close(&mut self) {
        self.builder.close();
    }
}

/// Render the styled text to a straight-alpha RGBA frame sized to the ink
/// (plus a small pad). Empty text yields a 1×1 transparent frame.
pub fn render_text(style: &TextStyle) -> Result<Frame, StaticSourceError> {
    let font = resolve_font(style)?;
    let primary = rustybuzz::Face::from_slice(&font.data, font.index).ok_or_else(|| {
        StaticSourceError::NoFont("the font file could not be parsed".to_string())
    })?;
    // The face list: the resolved primary, then the bundled script fallbacks
    // (Noto Sans / Arabic / Hebrew) for runs the primary does not cover.
    let mut faces = vec![primary];
    for data in FALLBACK_FONTS {
        if let Some(face) = rustybuzz::Face::from_slice(data, 0) {
            faces.push(face);
        }
    }

    let size = style.size_px.clamp(4.0, 512.0);
    let scale = size / faces[0].units_per_em() as f32;
    let ascent = faces[0].ascender() as f32 * scale;
    let descent = faces[0].descender() as f32 * scale; // typically negative
    let line_gap = faces[0].line_gap() as f32 * scale;
    let natural = (ascent - descent + line_gap).max(size * 0.5);
    let line_height = natural * style.line_spacing.clamp(0.25, 4.0);

    // Shape every paragraph into visually-ordered lines.
    let mut lines: Vec<Line> = Vec::new();
    for paragraph in style.text.split('\n') {
        if paragraph.is_empty() {
            lines.push(Line {
                glyphs: Vec::new(),
                width: 0.0,
            });
            continue;
        }
        let bidi = BidiInfo::new(paragraph, style.force_rtl.then(Level::rtl));
        let para = &bidi.paragraphs[0];
        let para_rtl = para.level.is_rtl();

        let line_ranges = match style.wrap_width {
            Some(max) if max > 0 => wrap_paragraph(
                &faces,
                paragraph,
                para_rtl,
                scale,
                (max as f32 - 2.0 * PAD).max(size),
            ),
            _ => std::iter::once(0..paragraph.len()).collect(),
        };

        for range in line_ranges {
            let (levels, runs) = bidi.visual_runs(para, range);
            let mut glyphs = Vec::new();
            let mut pen = 0.0f32;
            for run in runs {
                let rtl = levels[run.start].is_rtl();
                let text = &paragraph[run.clone()];
                let face = face_for_text(&faces, text);
                pen += shape_run(&faces, face, text, rtl, scale, pen, &mut glyphs);
            }
            lines.push(Line { glyphs, width: pen });
        }
    }

    let content_width = lines.iter().map(|line| line.width).fold(0.0f32, f32::max);
    let width = (content_width + 2.0 * PAD).ceil() as u32;
    let height = (lines.len() as f32 * line_height + 2.0 * PAD).ceil() as u32;
    if content_width <= 0.0 || style.text.trim().is_empty() {
        return Ok(rgba_frame(1, 1, vec![0, 0, 0, 0]));
    }
    check_dimension("rendered text width", width.min(MAX_STATIC_DIMENSION + 1))?;
    check_dimension("rendered text height", height.min(MAX_STATIC_DIMENSION + 1))?;

    let mut pixmap = tiny_skia::Pixmap::new(width, height).ok_or(StaticSourceError::TooLarge {
        what: "rendered text",
        size: width.max(height),
    })?;
    let mut paint = tiny_skia::Paint::default();
    paint.set_color_rgba8(
        style.color[0],
        style.color[1],
        style.color[2],
        style.color[3],
    );
    paint.anti_alias = true;

    for (line_index, line) in lines.iter().enumerate() {
        let baseline = PAD + ascent + line_index as f32 * line_height;
        let left = PAD
            + match style.align {
                TextAlign::Left => 0.0,
                TextAlign::Center => (content_width - line.width) / 2.0,
                TextAlign::Right => content_width - line.width,
            };
        for glyph in &line.glyphs {
            let mut outline = GlyphOutline {
                builder: tiny_skia::PathBuilder::new(),
                scale,
                dx: left + glyph.x,
                dy: baseline - glyph.y,
            };
            faces[glyph.face].outline_glyph(GlyphId(glyph.id), &mut outline);
            if let Some(path) = outline.builder.finish() {
                pixmap.fill_path(
                    &path,
                    &paint,
                    tiny_skia::FillRule::Winding,
                    tiny_skia::Transform::identity(),
                    None,
                );
            }
        }
    }

    // tiny-skia stores premultiplied alpha; the compositor blends straight.
    let mut data = pixmap.take();
    for px in data.chunks_exact_mut(4) {
        let alpha = px[3] as u32;
        if alpha > 0 && alpha < 255 {
            for channel in &mut px[..3] {
                *channel = ((*channel as u32 * 255) / alpha).min(255) as u8;
            }
        }
    }
    Ok(rgba_frame(width, height, data))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Machines (or CI containers) with no fonts skip loudly instead of
    /// failing — everywhere real, the OS ships fonts.
    fn render(style: &TextStyle) -> Option<Frame> {
        match render_text(style) {
            Ok(frame) => Some(frame),
            Err(StaticSourceError::NoFont(why)) => {
                eprintln!("SKIPPED: no usable font here ({why})");
                None
            }
            Err(other) => panic!("text render failed: {other}"),
        }
    }

    fn style(text: &str) -> TextStyle {
        TextStyle {
            text: text.to_string(),
            size_px: 32.0,
            ..TextStyle::default()
        }
    }

    fn ink_count(frame: &Frame) -> usize {
        frame.data.chunks(4).filter(|px| px[3] > 0).count()
    }

    #[test]
    fn renders_visible_ink() {
        let Some(frame) = render(&style("Hello")) else {
            return;
        };
        assert!(frame.width > 10 && frame.height > 10);
        assert!(ink_count(&frame) > 20, "letters leave ink");
    }

    #[test]
    fn longer_text_is_wider() {
        let (Some(one), Some(three)) = (render(&style("A")), render(&style("AAA"))) else {
            return;
        };
        assert!(three.width > one.width);
    }

    #[test]
    fn the_color_applies() {
        let mut red = style("Hi");
        red.color = [255, 0, 0, 255];
        let Some(frame) = render(&red) else { return };
        let solid = frame
            .data
            .chunks(4)
            .find(|px| px[3] == 255)
            .expect("some fully-opaque pixel");
        assert_eq!(&solid[..3], &[255, 0, 0]);
    }

    #[test]
    fn newlines_stack_lines() {
        let (Some(one), Some(two)) = (render(&style("a")), render(&style("a\na"))) else {
            return;
        };
        assert!(two.height > one.height);
    }

    #[test]
    fn wrapping_grows_downward() {
        let long = "words words words words words words words words";
        let unwrapped = style(long);
        let mut wrapped = style(long);
        wrapped.wrap_width = Some(160);
        let (Some(flat), Some(tall)) = (render(&unwrapped), render(&wrapped)) else {
            return;
        };
        assert!(tall.height > flat.height, "wrap adds lines");
        assert!(tall.width < flat.width, "wrap respects the width");
    }

    #[test]
    fn rtl_text_renders() {
        // Arabic: shaping joins letters; bidi orders the line RTL.
        let Some(frame) = render(&style("سلام عليكم")) else {
            return;
        };
        assert!(ink_count(&frame) > 20, "Arabic leaves ink");
    }

    #[test]
    fn alignment_moves_the_ink() {
        let text = "a\nlonger line";
        let mut left = style(text);
        left.align = TextAlign::Left;
        let mut right = style(text);
        right.align = TextAlign::Right;
        let (Some(l), Some(r)) = (render(&left), render(&right)) else {
            return;
        };
        // The short first line's ink: left-aligned it hugs x≈0; right-aligned
        // it sits near the right edge. Compare the first inked column within
        // the first text line.
        let first_ink_x = |frame: &Frame| {
            let row_span = (frame.height / 4).max(1);
            for x in 0..frame.width {
                for y in 0..row_span {
                    let idx = ((y * frame.width + x) * 4 + 3) as usize;
                    if frame.data[idx] > 0 {
                        return x;
                    }
                }
            }
            0
        };
        assert!(
            first_ink_x(&r) > first_ink_x(&l) + 5,
            "right-aligned short line starts further right"
        );
    }

    #[test]
    fn empty_text_is_a_transparent_pixel() {
        let Some(frame) = render(&style("   ")) else {
            return;
        };
        assert_eq!((frame.width, frame.height), (1, 1));
        assert_eq!(frame.data[3], 0);
    }

    #[test]
    fn unknown_families_fall_back_to_bundled_noto() {
        // The bundled family makes NoFont unreachable for family lookups —
        // render_text is called directly (no skip helper) on purpose.
        let mut styled = style("Bundled");
        styled.font_family = Some("Definitely Not A Real Font 123".into());
        let frame = render_text(&styled).expect("bundled fonts guarantee a render");
        assert!(ink_count(&frame) > 20, "renders with the bundled Noto Sans");
    }

    #[test]
    fn face_selection_covers_scripts_with_the_bundled_fallbacks() {
        let faces: Vec<rustybuzz::Face<'_>> = [NOTO_SANS, NOTO_SANS_ARABIC, NOTO_SANS_HEBREW]
            .iter()
            .filter_map(|data| rustybuzz::Face::from_slice(data, 0))
            .collect();
        assert_eq!(faces.len(), 3, "all bundled faces parse");
        assert_eq!(face_for_text(&faces, "Hello"), 0, "Latin stays primary");
        assert_eq!(face_for_text(&faces, "سلام"), 1, "Arabic → Noto Sans Arabic");
        assert_eq!(
            face_for_text(&faces, "שלום"),
            2,
            "Hebrew → Noto Sans Hebrew"
        );
        assert_eq!(face_for_text(&faces, "…!?"), 0, "punctuation stays primary");
    }

    #[test]
    fn bundled_italic_parses_too() {
        assert!(
            rustybuzz::Face::from_slice(NOTO_SANS_ITALIC, 0).is_some(),
            "the bundled italic face is valid"
        );
    }
}
