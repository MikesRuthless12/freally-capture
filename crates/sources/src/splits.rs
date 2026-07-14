//! CAP-N18 — the **speedrun split timer source**: a LiveSplit-style timer
//! that imports a `.lss` split file (read-only — nothing is ever written
//! back), compares the live run against **PB / best segments / average**,
//! highlights gold segments, and splits from the global hotkeys or the
//! properties dialog. Fully local.
//!
//! Honest scope notes:
//! - **Process-memory auto-splitters are deliberately excluded** (anti-cheat
//!   adjacency) — file import + hotkey/manual splitting only.
//! - The `.lss` reader is an owned, bounded parser for LiveSplit's schema
//!   subset (names, PB split times, best segment times, segment history) —
//!   no XML dependency. Unknown elements are ignored; a malformed file
//!   errors readably instead of guessing.
//! - Times shown are RTA (`RealTime`); game-time is not modeled.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, Weak};
use std::time::{Duration, Instant};

use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame};

use crate::compose::{blit, fill_rect};
use crate::static_source::{check_dimension, rgba_frame};
use crate::text::{render_text, TextAlign, TextStyle};

/// A split file larger than this is refused (a real `.lss` is a few KB).
pub const MAX_LSS_BYTES: u64 = 4 * 1024 * 1024;
/// The generator's frame cadence — deciseconds tick at 10 Hz, drawn at 30.
const FPS: u32 = 30;

// ---------------------------------------------------------------------------
// .lss parsing (owned, bounded, LiveSplit schema subset)
// ---------------------------------------------------------------------------

/// One segment as defined by the split file.
#[derive(Debug, Clone, PartialEq)]
pub struct SegmentDef {
    pub name: String,
    /// PB **cumulative** split time, ms (`None` = no PB at this segment).
    pub pb_ms: Option<u64>,
    /// Best (gold) **segment duration**, ms.
    pub best_ms: Option<u64>,
    /// Historic segment durations, ms — the average comparison's data.
    pub history_ms: Vec<u64>,
}

/// The parsed split file.
#[derive(Debug, Clone, PartialEq)]
pub struct SplitFile {
    pub game: String,
    pub category: String,
    pub segments: Vec<SegmentDef>,
}

/// Decode the five XML entities plus numeric character references.
fn decode_entities(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        let tail = &rest[amp..];
        let Some(semi) = tail.find(';') else {
            out.push_str(tail);
            return out;
        };
        let entity = &tail[1..semi];
        match entity {
            "amp" => out.push('&'),
            "lt" => out.push('<'),
            "gt" => out.push('>'),
            "quot" => out.push('"'),
            "apos" => out.push('\''),
            _ => {
                let parsed = entity
                    .strip_prefix("#x")
                    .or_else(|| entity.strip_prefix("#X"))
                    .and_then(|hex| u32::from_str_radix(hex, 16).ok())
                    .or_else(|| entity.strip_prefix('#').and_then(|dec| dec.parse().ok()))
                    .and_then(char::from_u32);
                match parsed {
                    Some(ch) => out.push(ch),
                    None => out.push_str(&tail[..=semi]),
                }
            }
        }
        rest = &tail[semi + 1..];
    }
    out.push_str(rest);
    out
}

/// Whether the byte after a matched tag name ends the name (so `<Segment`
/// never matches `<Segments`).
fn name_boundary(byte: Option<u8>) -> bool {
    matches!(
        byte,
        Some(b'>') | Some(b' ') | Some(b'\t') | Some(b'\r') | Some(b'\n') | Some(b'/')
    )
}

/// Every `<name …>inner</name>` in `block`, as `(attributes, inner)` pairs.
/// Self-closing tags yield `("…", "")`. The subset never nests a tag inside
/// itself, and callers scope nested lookups to the returned inner block.
fn element_blocks<'a>(block: &'a str, name: &str) -> Vec<(&'a str, &'a str)> {
    let open = format!("<{name}");
    let close = format!("</{name}>");
    let mut found = Vec::new();
    let mut cursor = 0;
    while let Some(at) = block[cursor..].find(&open) {
        let start = cursor + at;
        let after_name = start + open.len();
        if !name_boundary(block.as_bytes().get(after_name).copied()) {
            cursor = after_name;
            continue;
        }
        let Some(tag_end) = block[after_name..].find('>') else {
            break;
        };
        let tag_end = after_name + tag_end;
        let attrs = &block[after_name..tag_end];
        if attrs.trim_end().ends_with('/') {
            found.push((attrs, ""));
            cursor = tag_end + 1;
            continue;
        }
        let inner_start = tag_end + 1;
        let Some(rel_close) = block[inner_start..].find(&close) else {
            break;
        };
        found.push((attrs, &block[inner_start..inner_start + rel_close]));
        cursor = inner_start + rel_close + close.len();
    }
    found
}

/// The first `<name>`'s inner text, entity-decoded (CDATA unwrapped).
fn element_text(block: &str, name: &str) -> Option<String> {
    let (_, inner) = element_blocks(block, name).into_iter().next()?;
    let inner = inner.trim();
    let inner = inner
        .strip_prefix("<![CDATA[")
        .and_then(|rest| rest.strip_suffix("]]>"))
        .unwrap_or(inner);
    Some(decode_entities(inner))
}

/// Parse a LiveSplit `RealTime` string (`"HH:MM:SS.fffffff"`, optionally
/// with a `D.` day prefix) into milliseconds.
fn parse_realtime_ms(text: &str) -> Option<u64> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    // A dot BEFORE the first colon is the day separator.
    let (days, rest) = match (text.find('.'), text.find(':')) {
        (Some(dot), Some(colon)) if dot < colon => {
            (text[..dot].parse::<u64>().ok()?, &text[dot + 1..])
        }
        _ => (0, text),
    };
    let (hms, frac) = match rest.rfind('.') {
        Some(dot) => (&rest[..dot], &rest[dot + 1..]),
        None => (rest, ""),
    };
    let mut fields = hms.split(':');
    let (hours, minutes, seconds) =
        match (fields.next(), fields.next(), fields.next(), fields.next()) {
            (Some(h), Some(m), Some(s), None) => (
                h.parse::<u64>().ok()?,
                m.parse::<u64>().ok()?,
                s.parse::<u64>().ok()?,
            ),
            (Some(m), Some(s), None, None) => (0, m.parse::<u64>().ok()?, s.parse::<u64>().ok()?),
            _ => return None,
        };
    if minutes > 59 || seconds > 59 {
        return None;
    }
    let frac_ms = if frac.is_empty() {
        0
    } else {
        let digits: String = frac.chars().take_while(|ch| ch.is_ascii_digit()).collect();
        if digits.is_empty() {
            return None;
        }
        let value: u64 = digits[..digits.len().min(3)].parse().ok()?;
        match digits.len().min(3) {
            1 => value * 100,
            2 => value * 10,
            _ => value,
        }
    };
    Some(days * 86_400_000 + hours * 3_600_000 + minutes * 60_000 + seconds * 1_000 + frac_ms)
}

/// Parse a `.lss` document. Readable errors, never a guess.
pub fn parse_lss(content: &str) -> Result<SplitFile, String> {
    let game = element_text(content, "GameName").unwrap_or_default();
    let category = element_text(content, "CategoryName").unwrap_or_default();
    let (_, segments_block) = element_blocks(content, "Segments")
        .into_iter()
        .next()
        .ok_or("no <Segments> in the split file")?;
    let mut segments = Vec::new();
    for (_, segment) in element_blocks(segments_block, "Segment") {
        let name = element_text(segment, "Name").unwrap_or_default();
        let pb_ms = element_blocks(segment, "SplitTime")
            .into_iter()
            .find(|(attrs, _)| attrs.contains("Personal Best"))
            .and_then(|(_, inner)| element_text(inner, "RealTime"))
            .and_then(|text| parse_realtime_ms(&text));
        let best_ms = element_blocks(segment, "BestSegmentTime")
            .into_iter()
            .next()
            .and_then(|(_, inner)| element_text(inner, "RealTime"))
            .and_then(|text| parse_realtime_ms(&text));
        let history_ms = element_blocks(segment, "SegmentHistory")
            .into_iter()
            .next()
            .map(|(_, history)| {
                element_blocks(history, "Time")
                    .into_iter()
                    .filter_map(|(_, time)| element_text(time, "RealTime"))
                    .filter_map(|text| parse_realtime_ms(&text))
                    .collect()
            })
            .unwrap_or_default();
        segments.push(SegmentDef {
            name,
            pb_ms,
            best_ms,
            history_ms,
        });
    }
    if segments.is_empty() {
        return Err("the split file has no segments".into());
    }
    Ok(SplitFile {
        game,
        category,
        segments,
    })
}

// ---------------------------------------------------------------------------
// Comparisons
// ---------------------------------------------------------------------------

/// Which reference the live run is compared against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    /// The PB run's cumulative split times.
    Pb,
    /// Cumulative sum of the best (gold) segments.
    BestSegments,
    /// Cumulative mean of each segment's history.
    Average,
}

/// The comparison's **cumulative** time at each split (`None` where the
/// file has no data — e.g. a segment never finished in any run).
pub fn comparison_cum(file: &SplitFile, comparison: Comparison) -> Vec<Option<u64>> {
    match comparison {
        Comparison::Pb => file.segments.iter().map(|segment| segment.pb_ms).collect(),
        Comparison::BestSegments => {
            let mut total = Some(0u64);
            file.segments
                .iter()
                .map(|segment| {
                    total = match (total, segment.best_ms) {
                        (Some(sum), Some(best)) => Some(sum + best),
                        _ => None,
                    };
                    total
                })
                .collect()
        }
        Comparison::Average => {
            let mut total = Some(0u64);
            file.segments
                .iter()
                .map(|segment| {
                    let mean = if segment.history_ms.is_empty() {
                        None
                    } else {
                        Some(
                            segment.history_ms.iter().sum::<u64>()
                                / segment.history_ms.len() as u64,
                        )
                    };
                    total = match (total, mean) {
                        (Some(sum), Some(mean)) => Some(sum + mean),
                        _ => None,
                    };
                    total
                })
                .collect()
        }
    }
}

// ---------------------------------------------------------------------------
// The run state machine (pure — the registry wraps it with a clock)
// ---------------------------------------------------------------------------

/// The live run: cumulative split times, `None` = skipped segment.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RunMachine {
    segments: usize,
    splits: Vec<Option<u64>>,
}

impl RunMachine {
    pub fn new(segments: usize) -> Self {
        Self {
            segments,
            splits: Vec::new(),
        }
    }

    /// The segment the runner is on (== completed split count).
    pub fn current(&self) -> usize {
        self.splits.len()
    }

    pub fn finished(&self) -> bool {
        self.segments > 0 && self.splits.len() == self.segments
    }

    pub fn splits(&self) -> &[Option<u64>] {
        &self.splits
    }

    /// Record a split at `now_ms` (cumulative, from the run start).
    pub fn split(&mut self, now_ms: u64) {
        if !self.finished() {
            self.splits.push(Some(now_ms));
        }
    }

    /// Un-split — the timer keeps running, the segment reopens.
    pub fn undo(&mut self) {
        self.splits.pop();
    }

    /// Skip the current segment (records no time). The FINAL segment can't
    /// be skipped — a run ends by splitting, exactly like LiveSplit.
    pub fn skip(&mut self) {
        if !self.finished() && self.current() + 1 < self.segments {
            self.splits.push(None);
        }
    }

    pub fn reset(&mut self) {
        self.splits.clear();
    }

    /// This run's duration of segment `index` — `None` when the segment (or
    /// the one before it) was skipped, so no honest duration exists.
    pub fn segment_duration(&self, index: usize) -> Option<u64> {
        let end = *self.splits.get(index)?;
        let start = if index == 0 {
            Some(0)
        } else {
            *self.splits.get(index - 1)?
        };
        Some(end? - start?)
    }

    /// Whether segment `index` is a gold (beat the file's best segment).
    pub fn is_gold(&self, index: usize, file: &SplitFile) -> bool {
        match (
            self.segment_duration(index),
            file.segments.get(index).and_then(|segment| segment.best_ms),
        ) {
            (Some(run), Some(best)) => run < best,
            _ => false,
        }
    }
}

/// What a hotkey / command asks of the timer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitAction {
    /// Start the run, or record the next split (finishes on the last one).
    Split,
    Undo,
    Skip,
    Reset,
}

/// One live timer's shared state — the generator renders it, hotkeys and
/// the properties dialog mutate it.
pub struct SplitSession {
    pub machine: RunMachine,
    started: Option<Instant>,
    /// The frozen elapsed at the finishing split.
    final_ms: Option<u64>,
}

impl SplitSession {
    fn new(segments: usize) -> Self {
        Self {
            machine: RunMachine::new(segments),
            started: None,
            final_ms: None,
        }
    }

    /// Elapsed run time right now, ms (frozen once finished, 0 before start).
    pub fn elapsed_ms(&self) -> u64 {
        if let Some(frozen) = self.final_ms {
            return frozen;
        }
        self.started
            .map(|at| at.elapsed().as_millis() as u64)
            .unwrap_or(0)
    }

    pub fn running(&self) -> bool {
        self.started.is_some() && self.final_ms.is_none()
    }

    pub fn control(&mut self, action: SplitAction) {
        match action {
            SplitAction::Split => {
                if self.started.is_none() {
                    self.machine.reset();
                    self.final_ms = None;
                    self.started = Some(Instant::now());
                } else if !self.machine.finished() {
                    let now = self.elapsed_ms();
                    self.machine.split(now);
                    if self.machine.finished() {
                        self.final_ms = Some(now);
                    }
                }
            }
            SplitAction::Undo => {
                if self.machine.current() > 0 {
                    self.machine.undo();
                    self.final_ms = None;
                }
            }
            SplitAction::Skip => {
                if self.started.is_some() && self.final_ms.is_none() {
                    self.machine.skip();
                }
            }
            SplitAction::Reset => {
                self.machine.reset();
                self.started = None;
                self.final_ms = None;
            }
        }
    }
}

/// Weak registry keyed by source id — sessions register on start and
/// unsubscribe by dropping (same shape as the visualizer taps).
fn registry() -> &'static Mutex<HashMap<String, Weak<Mutex<SplitSession>>>> {
    static REG: OnceLock<Mutex<HashMap<String, Weak<Mutex<SplitSession>>>>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(HashMap::new()))
}

fn lock_registry() -> std::sync::MutexGuard<'static, HashMap<String, Weak<Mutex<SplitSession>>>> {
    registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn lock_session(session: &Mutex<SplitSession>) -> std::sync::MutexGuard<'_, SplitSession> {
    session
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Drive one timer (the properties dialog). `false` = no live timer here.
pub fn control(id: &str, action: SplitAction) -> bool {
    let live = lock_registry().get(id).and_then(Weak::upgrade);
    match live {
        Some(session) => {
            lock_session(&session).control(action);
            true
        }
        None => false,
    }
}

/// Drive EVERY live split timer (the global hotkeys — a global key can't
/// name a source, exactly like the CAP-M15 timer keys).
pub fn control_all(action: SplitAction) {
    let live: Vec<Arc<Mutex<SplitSession>>> = {
        let mut registry = lock_registry();
        registry.retain(|_, weak| weak.strong_count() > 0);
        registry.values().filter_map(Weak::upgrade).collect()
    };
    for session in live {
        lock_session(&session).control(action);
    }
}

// ---------------------------------------------------------------------------
// Formatting + face rendering
// ---------------------------------------------------------------------------

/// `"9.4"`, `"1:23.4"`, `"1:02:03.4"` — deciseconds everywhere; hours only
/// when they exist.
pub fn format_ms(ms: u64) -> String {
    let decis = (ms % 1_000) / 100;
    let seconds = ms / 1_000;
    let (h, m, s) = (seconds / 3_600, (seconds / 60) % 60, seconds % 60);
    if h > 0 {
        format!("{h}:{m:02}:{s:02}.{decis}")
    } else if m > 0 {
        format!("{m}:{s:02}.{decis}")
    } else {
        format!("{s}.{decis}")
    }
}

/// A signed delta: `"+1.2"` / `"-0.8"` (ASCII sign — every face font has it).
pub fn format_delta(run_ms: u64, comparison_ms: u64) -> String {
    if run_ms >= comparison_ms {
        format!("+{}", format_ms(run_ms - comparison_ms))
    } else {
        format!("-{}", format_ms(comparison_ms - run_ms))
    }
}

/// Everything the generator needs, already resolved by the caller.
#[derive(Debug, Clone)]
pub struct SplitTimerConfig {
    pub file: SplitFile,
    pub comparison: Comparison,
    pub width: u32,
    pub height: u32,
    pub size_px: f32,
    pub color: [u8; 4],
    pub ahead: [u8; 4],
    pub behind: [u8; 4],
    pub gold: [u8; 4],
}

fn dim(color: [u8; 4], factor: f32) -> [u8; 4] {
    [
        color[0],
        color[1],
        color[2],
        (color[3] as f32 * factor) as u8,
    ]
}

fn raster(text: &str, size_px: f32, color: [u8; 4]) -> Option<Frame> {
    if text.is_empty() {
        return None;
    }
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
}

/// One face. Pure given a state snapshot — unit-testable without a session.
fn render_face(
    config: &SplitTimerConfig,
    machine: &RunMachine,
    elapsed_ms: u64,
    started: bool,
) -> Vec<u8> {
    let (w, h) = (config.width as usize, config.height as usize);
    let mut face = vec![0u8; w * h * 4];
    let size = config.size_px.clamp(8.0, 96.0);
    let pad = (size * 0.4) as usize;
    let row_h = (size * 1.5) as usize;
    let big_h = (size * 3.0) as usize;
    let comparison = comparison_cum(&config.file, config.comparison);

    // Title.
    let title = match (config.file.game.is_empty(), config.file.category.is_empty()) {
        (false, false) => format!("{} — {}", config.file.game, config.file.category),
        (false, true) => config.file.game.clone(),
        (true, false) => config.file.category.clone(),
        (true, true) => String::new(),
    };
    let mut y = pad;
    if let Some(text) = raster(&title, size, dim(config.color, 0.7)) {
        blit(&mut face, w, h, &text, pad as i64, y as i64);
        y += row_h;
    }

    // Segment rows — a window that keeps the current segment in view.
    let rows_area = h.saturating_sub(y + big_h + pad);
    let visible = (rows_area / row_h.max(1)).max(1);
    let total = config.file.segments.len();
    let current = machine.current().min(total.saturating_sub(1));
    let first = current
        .saturating_sub(visible.saturating_sub(2))
        .min(total.saturating_sub(visible.min(total)));
    for (row, index) in (first..total.min(first + visible)).enumerate() {
        let row_y = y + row * row_h;
        let segment = &config.file.segments[index];
        // The current segment carries a faint underline.
        if index == machine.current() && started && !machine.finished() {
            let line_y = (row_y + row_h).saturating_sub(2).min(h.saturating_sub(1));
            fill_rect(
                &mut face,
                w,
                pad,
                line_y,
                w.saturating_sub(pad),
                line_y + 1,
                dim(config.color, 0.5),
            );
        }
        if let Some(name) = raster(&segment.name, size, config.color) {
            blit(&mut face, w, h, &name, pad as i64, row_y as i64);
        }
        let time_slot = (size * 5.0) as usize;
        match machine.splits().get(index) {
            // Completed: actual cumulative time + colored delta.
            Some(Some(cum)) => {
                let time_color = if machine.is_gold(index, &config.file) {
                    config.gold
                } else {
                    config.color
                };
                if let Some(time) = raster(&format_ms(*cum), size, time_color) {
                    blit(
                        &mut face,
                        w,
                        h,
                        &time,
                        (w - pad) as i64 - time.width as i64,
                        row_y as i64,
                    );
                }
                if let Some(reference) = comparison[index] {
                    let delta_color = if machine.is_gold(index, &config.file) {
                        config.gold
                    } else if *cum <= reference {
                        config.ahead
                    } else {
                        config.behind
                    };
                    if let Some(delta) = raster(&format_delta(*cum, reference), size, delta_color) {
                        let x = (w - pad - time_slot) as i64 - delta.width as i64;
                        blit(&mut face, w, h, &delta, x, row_y as i64);
                    }
                }
            }
            // Skipped: an honest dash, no delta.
            Some(None) => {
                if let Some(dash) = raster("—", size, dim(config.color, 0.5)) {
                    blit(
                        &mut face,
                        w,
                        h,
                        &dash,
                        (w - pad) as i64 - dash.width as i64,
                        row_y as i64,
                    );
                }
            }
            // Upcoming: the comparison's time, dimmed.
            None => {
                if let Some(reference) = comparison[index] {
                    if let Some(time) = raster(&format_ms(reference), size, dim(config.color, 0.55))
                    {
                        blit(
                            &mut face,
                            w,
                            h,
                            &time,
                            (w - pad) as i64 - time.width as i64,
                            row_y as i64,
                        );
                    }
                }
            }
        }
    }

    // The big timer, colored by the live delta.
    let big_color = if !started {
        config.color
    } else if machine.finished() {
        let beat = comparison
            .last()
            .copied()
            .flatten()
            .map(|total| elapsed_ms < total);
        match beat {
            Some(true) => config.gold,
            Some(false) => config.behind,
            None => config.color,
        }
    } else {
        match comparison.get(machine.current()).copied().flatten() {
            Some(reference) if elapsed_ms > reference => config.behind,
            Some(_) => config.ahead,
            None => config.color,
        }
    };
    if let Some(big) = raster(&format_ms(elapsed_ms), size * 2.2, big_color) {
        let x = (w - pad) as i64 - big.width as i64;
        let y = (h - pad) as i64 - big.height as i64;
        blit(&mut face, w, h, &big, x, y);
    }
    face
}

/// Start the split-timer session thread. The caller has already guarded the
/// path (never a network path) and read + parsed the file.
pub fn start_split_timer(
    id: &str,
    config: SplitTimerConfig,
) -> Result<CaptureSession, CaptureError> {
    check_dimension("split timer width", config.width)
        .and_then(|()| check_dimension("split timer height", config.height))
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    let session = Arc::new(Mutex::new(SplitSession::new(config.file.segments.len())));
    lock_registry().insert(id.to_string(), Arc::downgrade(&session));
    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-splittimer".into())
        .spawn(move || {
            let period = Duration::from_micros(1_000_000 / u64::from(FPS));
            let mut next = Instant::now();
            let mut last_face: Option<(RunMachine, u64, bool)> = None;
            loop {
                if thread_stop.load(Ordering::Relaxed) || !sender.is_open() {
                    return;
                }
                let (machine, elapsed, started) = {
                    let session = lock_session(&session);
                    (
                        session.machine.clone(),
                        session.elapsed_ms(),
                        session.started.is_some(),
                    )
                };
                // Repaint only when the shown state moved (deciseconds tick
                // at 10 Hz; idle timers paint once).
                let shown = (machine, elapsed / 100, started);
                if last_face.as_ref() != Some(&shown) {
                    let face = render_face(&config, &shown.0, elapsed, started);
                    sender.send(rgba_frame(config.width, config.height, face));
                    last_face = Some(shown);
                }
                next += period;
                let now = Instant::now();
                if next > now {
                    std::thread::sleep(next - now);
                } else {
                    next = now;
                }
            }
        })
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Run version="1.7.0">
  <GameName>Cave &amp; Castle</GameName>
  <CategoryName>Any%</CategoryName>
  <AttemptCount>12</AttemptCount>
  <Segments>
    <Segment>
      <Name>Tutorial &lt;skip&gt;</Name>
      <SplitTimes>
        <SplitTime name="Personal Best">
          <RealTime>00:01:00.5000000</RealTime>
        </SplitTime>
      </SplitTimes>
      <BestSegmentTime>
        <RealTime>00:00:55.0000000</RealTime>
      </BestSegmentTime>
      <SegmentHistory>
        <Time id="1"><RealTime>00:01:10.0000000</RealTime></Time>
        <Time id="2"><RealTime>00:00:50.0000000</RealTime></Time>
      </SegmentHistory>
    </Segment>
    <Segment>
      <Name>Castle</Name>
      <SplitTimes>
        <SplitTime name="Personal Best">
          <RealTime>00:02:30.0000000</RealTime>
        </SplitTime>
      </SplitTimes>
      <BestSegmentTime>
        <RealTime>00:01:20.0000000</RealTime>
      </BestSegmentTime>
      <SegmentHistory>
        <Time id="1"><RealTime>00:01:30.0000000</RealTime></Time>
      </SegmentHistory>
    </Segment>
    <Segment>
      <Name>Boss</Name>
      <SplitTimes>
        <SplitTime name="Personal Best" />
      </SplitTimes>
      <BestSegmentTime />
      <SegmentHistory />
    </Segment>
  </Segments>
</Run>"#;

    #[test]
    fn the_fixture_parses_names_pb_best_and_history() {
        let file = parse_lss(FIXTURE).expect("parses");
        assert_eq!(file.game, "Cave & Castle");
        assert_eq!(file.category, "Any%");
        assert_eq!(file.segments.len(), 3);
        assert_eq!(file.segments[0].name, "Tutorial <skip>");
        assert_eq!(file.segments[0].pb_ms, Some(60_500));
        assert_eq!(file.segments[0].best_ms, Some(55_000));
        assert_eq!(file.segments[0].history_ms, vec![70_000, 50_000]);
        assert_eq!(file.segments[1].pb_ms, Some(150_000));
        // A segment with no recorded times parses honestly empty.
        assert_eq!(file.segments[2].pb_ms, None);
        assert_eq!(file.segments[2].best_ms, None);
        assert!(file.segments[2].history_ms.is_empty());
    }

    #[test]
    fn a_file_with_no_segments_errors_readably() {
        let err = parse_lss("<Run><Segments></Segments></Run>").unwrap_err();
        assert!(err.contains("no segments"), "{err}");
        let err = parse_lss("not xml at all").unwrap_err();
        assert!(err.contains("<Segments>"), "{err}");
    }

    #[test]
    fn realtime_parses_hms_fractions_and_days() {
        assert_eq!(parse_realtime_ms("00:01:00.5000000"), Some(60_500));
        assert_eq!(parse_realtime_ms("01:02:03"), Some(3_723_000));
        assert_eq!(parse_realtime_ms("1.01:00:00"), Some(90_000_000));
        assert_eq!(parse_realtime_ms("00:00:00.05"), Some(50));
        assert_eq!(parse_realtime_ms(""), None);
        assert_eq!(parse_realtime_ms("junk"), None);
    }

    #[test]
    fn comparisons_accumulate_and_go_dark_after_missing_data() {
        let file = parse_lss(FIXTURE).expect("parses");
        assert_eq!(
            comparison_cum(&file, Comparison::Pb),
            vec![Some(60_500), Some(150_000), None]
        );
        assert_eq!(
            comparison_cum(&file, Comparison::BestSegments),
            vec![Some(55_000), Some(135_000), None]
        );
        // Averages: (70+50)/2 = 60 s, +90 s = 150 s, then no history.
        assert_eq!(
            comparison_cum(&file, Comparison::Average),
            vec![Some(60_000), Some(150_000), None]
        );
    }

    #[test]
    fn the_machine_splits_skips_undoes_and_finishes() {
        let mut machine = RunMachine::new(3);
        machine.split(10_000);
        assert_eq!(machine.current(), 1);
        machine.skip();
        assert_eq!(machine.splits(), &[Some(10_000), None]);
        // The final segment can't be skipped.
        machine.skip();
        assert_eq!(machine.current(), 2);
        machine.split(30_000);
        assert!(machine.finished());
        // Splitting past the end does nothing.
        machine.split(31_000);
        assert_eq!(machine.current(), 3);
        machine.undo();
        assert!(!machine.finished());
        assert_eq!(machine.current(), 2);
    }

    #[test]
    fn golds_need_real_durations_on_both_sides() {
        let file = parse_lss(FIXTURE).expect("parses");
        let mut machine = RunMachine::new(3);
        machine.split(50_000); // beats the 55 s best → gold
        assert!(machine.is_gold(0, &file));
        machine.split(140_000); // 90 s segment vs 80 s best → no gold
        assert!(!machine.is_gold(1, &file));
        // A segment after a skip has no honest duration → never gold.
        let mut skipped = RunMachine::new(3);
        skipped.split(50_000);
        skipped.skip();
        skipped.split(200_000);
        assert_eq!(skipped.segment_duration(2), None);
        assert!(!skipped.is_gold(2, &file));
    }

    #[test]
    fn session_control_runs_the_full_lifecycle() {
        let mut session = SplitSession::new(2);
        assert_eq!(session.elapsed_ms(), 0);
        session.control(SplitAction::Split); // starts
        assert!(session.running());
        assert_eq!(session.machine.current(), 0, "starting records no split");
        session.control(SplitAction::Split);
        session.control(SplitAction::Split); // finishes
        assert!(session.machine.finished());
        let frozen = session.elapsed_ms();
        assert_eq!(session.elapsed_ms(), frozen, "finished time is frozen");
        session.control(SplitAction::Undo);
        assert!(!session.machine.finished(), "undo reopens the run");
        session.control(SplitAction::Reset);
        assert!(!session.running());
        assert_eq!(session.machine.current(), 0);
    }

    #[test]
    fn registry_control_reaches_live_sessions_only() {
        let session = Arc::new(Mutex::new(SplitSession::new(2)));
        lock_registry().insert("split-test-live".into(), Arc::downgrade(&session));
        assert!(control("split-test-live", SplitAction::Split));
        assert!(lock_session(&session).running());
        control_all(SplitAction::Reset);
        assert!(!lock_session(&session).running());
        drop(session);
        assert!(
            !control("split-test-live", SplitAction::Split),
            "dead is dead"
        );
    }

    #[test]
    fn times_and_deltas_format_like_a_split_timer() {
        assert_eq!(format_ms(9_400), "9.4");
        assert_eq!(format_ms(83_400), "1:23.4");
        assert_eq!(format_ms(3_723_400), "1:02:03.4");
        assert_eq!(format_delta(61_200, 60_000), "+1.2");
        assert_eq!(format_delta(59_200, 60_000), "-0.8");
    }

    #[test]
    fn the_face_draws_digits_over_transparency() {
        let file = parse_lss(FIXTURE).expect("parses");
        let config = SplitTimerConfig {
            file,
            comparison: Comparison::Pb,
            width: 320,
            height: 240,
            size_px: 16.0,
            color: [255, 255, 255, 255],
            ahead: [0, 255, 0, 255],
            behind: [255, 0, 0, 255],
            gold: [255, 200, 0, 255],
        };
        let mut machine = RunMachine::new(3);
        machine.split(50_000);
        let face = render_face(&config, &machine, 65_000, true);
        assert_eq!(face.len(), 320 * 240 * 4);
        let painted = face.chunks_exact(4).filter(|px| px[3] > 0).count();
        assert!(painted > 500, "the face has ink: {painted} px");
        // The corners stay transparent (nothing draws into (0,0)).
        assert_eq!(face[3], 0, "top-left transparent");
    }
}
