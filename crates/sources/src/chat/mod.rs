//! The **live chat overlay** source (Phase 6, TASK-613).
//!
//! ⛔ **The hard rule (Mike, 2026-07-07): the end user NEVER needs an API
//! key, a developer account, or a sign-in for YouTube or Twitch chat.**
//! The YouTube path is an owned [`innertube`] client that reads chat the
//! exact same way YouTube's own web player does — the public web
//! `INNERTUBE_API_KEY` constant scraped off the page, never a per-user
//! key. Twitch reads anonymous IRC (`justinfan…`, no credentials). Kick
//! polls its public channel endpoint. Facebook would need the user's own
//! Graph token — strictly opt-in and **not implemented yet**; it never
//! gates the others.
//!
//! **Stability ("works no matter what"):** every ingest parses
//! defensively (unknown shapes are skipped, never panicked on),
//! reconnects with backoff, re-resolves expired continuations, and treats
//! a total outage as non-fatal — the overlay just stops updating while
//! the stream/recording run on untouched. Ingest is isolated behind a
//! **bounded ring** (drop-oldest under flood) and the overlay re-renders
//! at a **capped rate**, so chat speed can never stall the encoder.

pub(crate) mod innertube;
pub(crate) mod kick;
pub(crate) mod render;
pub(crate) mod twitch;

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use fcap_capture::{frame_channel, CaptureError, CaptureSession};

/// The optional per-message tap (TASK-614's reaction watcher).
pub type MessageTap = Arc<dyn Fn(&str) + Send + Sync>;

/// One chat line, platform-tagged, timestamped at arrival (local wall
/// time — the overlay shows `h:mm:ss A.M./P.M.` per the spec).
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub platform: &'static str,
    pub username: String,
    pub text: String,
    /// Local wall-clock at arrival (unix ms).
    pub at_unix_ms: i64,
}

/// The bounded, flood-proof hand-off between ingest threads and the
/// renderer: newest [`RING_CAP`] messages win, everything older ages out.
#[derive(Clone)]
pub(crate) struct ChatSink {
    pub(crate) ring: Arc<Mutex<VecDeque<ChatMessage>>>,
    pub(crate) revision: Arc<AtomicU64>,
    /// An optional message tap (TASK-614: the reactions overlay watches
    /// chat for reaction emoji). Called AFTER sanitizing, off the render
    /// path — a slow tap can only slow its own ingest thread.
    pub(crate) on_message: Option<MessageTap>,
}

/// How many lines the ring retains (display shows fewer).
const RING_CAP: usize = 200;

/// V1-E: a global, bounded feed of every sanitized chat line across ALL
/// running overlays. The per-session ring stays private to its renderer;
/// this is what the featured-banner picker lists. Same shape as the ring
/// (`VecDeque::new` is const), so steady-state eviction is O(1).
static FEED: Mutex<VecDeque<ChatMessage>> = Mutex::new(VecDeque::new());
/// V1-E: the message pinned to the program's featured banner (None = off),
/// plus a revision so the studio re-renders the banner only on change.
static FEATURED: Mutex<Option<ChatMessage>> = Mutex::new(None);
static FEATURED_REV: AtomicU64 = AtomicU64::new(0);

/// The newest `count` chat lines across every running overlay — the
/// featured-banner picker's feed (oldest→newest, like the overlay).
pub fn recent_messages(count: usize) -> Vec<ChatMessage> {
    let feed = FEED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    feed.iter()
        .skip(feed.len().saturating_sub(count))
        .cloned()
        .collect()
}

/// Pin a message to the program's featured banner (`None` clears it).
pub fn set_featured(message: Option<ChatMessage>) {
    *FEATURED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = message;
    FEATURED_REV.fetch_add(1, Ordering::Relaxed);
}

/// The pinned message, if any (the program bake reads this per tick).
pub fn featured() -> Option<ChatMessage> {
    FEATURED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone()
}

/// Bumps on every pin/clear — lets the studio cache the rendered banner.
pub fn featured_revision() -> u64 {
    FEATURED_REV.load(Ordering::Relaxed)
}

/// Map a UI-supplied platform string back onto the static tags ingest uses
/// (`ChatMessage.platform` is `&'static str`).
pub fn intern_platform(platform: &str) -> &'static str {
    match platform {
        "youtube" => "youtube",
        "twitch" => "twitch",
        "kick" => "kick",
        _ => "chat",
    }
}

/// Ingest's sanitizer: control characters stripped, length-capped, trimmed.
fn clean(s: &str, cap: usize) -> String {
    s.chars()
        .filter(|c| !c.is_control())
        .take(cap)
        .collect::<String>()
        .trim()
        .to_string()
}

/// Build a [`ChatMessage`] from untrusted parts under ingest's OWN rules
/// (the featured-banner command re-cleans UI echoes through this, so the
/// caps and control-char policy live in exactly one place). `None` when the
/// text sanitizes to nothing.
pub fn sanitize_line(
    platform: &str,
    username: &str,
    text: &str,
    at_unix_ms: i64,
) -> Option<ChatMessage> {
    let text = clean(text, 200);
    if text.is_empty() {
        return None;
    }
    Some(ChatMessage {
        platform: intern_platform(platform),
        username: clean(username, 40),
        text,
        at_unix_ms,
    })
}

/// The featured-message banner frame (delegates to the private renderer).
pub fn render_featured_banner(
    message: &ChatMessage,
    width: u32,
    bg: [u8; 4],
    fg: [u8; 4],
) -> Result<fcap_capture::Frame, String> {
    render::render_featured_banner(message, width, bg, fg)
}
/// The renderer's minimum interval — a chat flood coalesces into at most
/// ~4 redraws a second, never per-message work on the render path.
const RENDER_TICK: Duration = Duration::from_millis(250);

impl ChatSink {
    fn new(on_message: Option<MessageTap>) -> Self {
        Self {
            ring: Arc::new(Mutex::new(VecDeque::new())),
            revision: Arc::new(AtomicU64::new(0)),
            on_message,
        }
    }

    /// Push one message (sanitized + bounded); floods drop the oldest.
    pub(crate) fn push(&self, platform: &'static str, username: &str, text: &str) {
        let Some(message) = sanitize_line(
            platform,
            username,
            text,
            chrono::Local::now().timestamp_millis(),
        ) else {
            return;
        };
        if let Some(tap) = &self.on_message {
            tap(&message.text);
        }
        // V1-E: mirror into the global picker feed (same bound, drop-oldest).
        {
            let mut feed = FEED
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            feed.push_back(message.clone());
            while feed.len() > RING_CAP {
                feed.pop_front();
            }
        }
        let mut ring = self
            .ring
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        ring.push_back(message);
        while ring.len() > RING_CAP {
            ring.pop_front();
        }
        self.revision.fetch_add(1, Ordering::Relaxed);
    }

    fn snapshot_tail(&self, count: usize) -> Vec<ChatMessage> {
        let ring = self
            .ring
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        ring.iter()
            .skip(ring.len().saturating_sub(count))
            .cloned()
            .collect()
    }
}

/// What the overlay ingests + how it draws.
#[derive(Debug, Clone)]
pub struct ChatOverlayConfig {
    /// A YouTube channel / watch / live_chat URL (empty = off).
    pub youtube: String,
    /// A Twitch channel name (empty = off).
    pub twitch: String,
    /// A Kick channel slug (empty = off).
    pub kick: String,
    /// Overlay width in canvas pixels.
    pub width: u32,
    /// How many newest lines stay on screen.
    pub max_lines: u32,
    pub font_size: f32,
}

/// Sleep in small steps so a stop request never waits out a long backoff.
pub(crate) fn interruptible_sleep(total: Duration, stop: &AtomicBool) {
    let mut remaining = total;
    while !remaining.is_zero() && !stop.load(Ordering::Relaxed) {
        let step = remaining.min(Duration::from_millis(200));
        std::thread::sleep(step);
        remaining = remaining.saturating_sub(step);
    }
}

/// Start the chat overlay: one ingest thread per configured platform, all
/// feeding the bounded ring; a renderer publishing transparent-background
/// text frames on the standard latest-wins session channel.
pub fn start_chat_overlay(
    config: &ChatOverlayConfig,
    on_message: Option<MessageTap>,
) -> Result<CaptureSession, CaptureError> {
    let youtube = config.youtube.trim().to_string();
    let twitch = config.twitch.trim().trim_start_matches('#').to_string();
    let kick = config.kick.trim().to_string();
    if youtube.is_empty() && twitch.is_empty() && kick.is_empty() {
        return Err(CaptureError::Backend(
            "point the chat overlay at a YouTube URL, a Twitch channel, or a Kick channel in its properties".into(),
        ));
    }

    let sink = ChatSink::new(on_message);
    let stop = Arc::new(AtomicBool::new(false));
    let (sender, receiver) = frame_channel();

    // The ingest threads run detached (only the render thread is joined by
    // CaptureSession::stop, keeping stop non-blocking on the studio render
    // loop). They watch `stop` between polls, and each platform's HTTP/IRC
    // timeout is bounded (8 s / 2 s), so a removed overlay's threads
    // self-terminate within that window — no unbounded lingering, no
    // stacking of long-lived pollers across a rapid restart.
    if !youtube.is_empty() {
        let sink = sink.clone();
        let stop = Arc::clone(&stop);
        std::thread::Builder::new()
            .name("fcap-chat-youtube".into())
            .spawn(move || innertube::run(&youtube, &sink, &stop))
            .map_err(|err| CaptureError::Backend(err.to_string()))?;
    }
    if !twitch.is_empty() {
        let sink = sink.clone();
        let stop = Arc::clone(&stop);
        std::thread::Builder::new()
            .name("fcap-chat-twitch".into())
            .spawn(move || twitch::run(&twitch, &sink, &stop))
            .map_err(|err| CaptureError::Backend(err.to_string()))?;
    }
    if !kick.is_empty() {
        let sink = sink.clone();
        let stop = Arc::clone(&stop);
        std::thread::Builder::new()
            .name("fcap-chat-kick".into())
            .spawn(move || kick::run(&kick, &sink, &stop))
            .map_err(|err| CaptureError::Backend(err.to_string()))?;
    }

    let width = config.width.clamp(120, 3840);
    let max_lines = config.max_lines.clamp(1, 50) as usize;
    let font_size = config.font_size.clamp(10.0, 96.0);
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-chat-render".into())
        .spawn(move || {
            let mut seen_revision = u64::MAX; // force the first (placeholder) render
            loop {
                if thread_stop.load(Ordering::Relaxed) || !sender.is_open() {
                    return;
                }
                let revision = sink.revision.load(Ordering::Relaxed);
                if revision != seen_revision {
                    seen_revision = revision;
                    let messages = sink.snapshot_tail(max_lines);
                    match render::render_chat(&messages, width, font_size) {
                        Ok(frame) => sender.send(frame),
                        Err(err) => eprintln!("chat overlay: render failed: {err}"),
                    }
                }
                std::thread::sleep(RENDER_TICK);
            }
        })
        .map_err(|err| CaptureError::Backend(err.to_string()))?;

    Ok(CaptureSession::from_parts(receiver, stop, join))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_global_feed_mirrors_pushes_and_featured_pins_bump_the_revision() {
        let sink = ChatSink::new(None);
        sink.push("twitch", "alpha", "feed me");
        let recent = recent_messages(RING_CAP);
        assert!(
            recent.iter().any(|m| m.text == "feed me"),
            "the picker feed sees every sanitized push"
        );
        let before = featured_revision();
        set_featured(recent.last().cloned());
        assert_eq!(
            featured().expect("pinned").text,
            recent.last().unwrap().text
        );
        set_featured(None);
        assert!(featured().is_none(), "None clears the pin");
        assert!(featured_revision() >= before + 2, "every pin/clear bumps");
    }

    #[test]
    fn the_ring_drops_oldest_under_flood_and_never_grows() {
        let sink = ChatSink::new(None);
        for index in 0..(RING_CAP + 50) {
            sink.push("twitch", "user", &format!("message {index}"));
        }
        let ring = sink.ring.lock().unwrap();
        assert_eq!(
            ring.len(),
            RING_CAP,
            "bounded — a flood only ages lines out"
        );
        assert!(ring.front().unwrap().text.contains("50"), "oldest dropped");
    }

    #[test]
    fn messages_sanitize_control_chars_and_bound_length() {
        let sink = ChatSink::new(None);
        sink.push(
            "twitch",
            "user\u{0007}",
            &format!("hi\u{0000}{}", "x".repeat(500)),
        );
        let ring = sink.ring.lock().unwrap();
        let message = ring.front().unwrap();
        assert_eq!(message.username, "user");
        assert!(message.text.len() <= 200);
        assert!(!message.text.contains('\u{0000}'));
    }

    #[test]
    fn an_unconfigured_overlay_is_refused() {
        let config = ChatOverlayConfig {
            youtube: String::new(),
            twitch: "  ".into(),
            kick: String::new(),
            width: 480,
            max_lines: 12,
            font_size: 22.0,
        };
        assert!(start_chat_overlay(&config, None).is_err());
    }
}
