//! Chat lines → one transparent-background RGBA frame, through the same
//! owned text rasterizer every Text source uses (rustybuzz shaping, the
//! bundled Noto Sans). Newest messages at the bottom; each line carries
//! its 12-hour `h:mm:ss A.M./P.M.` timestamp — a running, time-stamped
//! record so nothing said is lost even if you missed it live.

use chrono::{Local, TimeZone, Timelike};
use fcap_capture::Frame;

use crate::text::{render_text, TextStyle};

use super::ChatMessage;

/// `9:32:53 P.M.` — 12-hour with seconds, per the spec.
pub(crate) fn timestamp_12h(unix_ms: i64) -> String {
    let time = Local
        .timestamp_millis_opt(unix_ms)
        .single()
        .unwrap_or_else(Local::now);
    let (pm, hour) = time.hour12();
    format!(
        "{}:{:02}:{:02} {}",
        hour,
        time.minute(),
        time.second(),
        if pm { "P.M." } else { "A.M." }
    )
}

fn platform_tag(platform: &str) -> &'static str {
    match platform {
        "youtube" => "[YT]",
        "twitch" => "[TW]",
        "kick" => "[KI]",
        _ => "[--]",
    }
}

/// One overlay frame: every message as
/// `9:32:53 P.M. [YT] name: message`, word-wrapped to `width`,
/// transparent background (just the text, as specced).
pub(crate) fn render_chat(
    messages: &[ChatMessage],
    width: u32,
    font_size: f32,
) -> Result<Frame, String> {
    let text = if messages.is_empty() {
        "connected — waiting for chat…".to_string()
    } else {
        messages
            .iter()
            .map(|message| {
                format!(
                    "{} {} {}: {}",
                    timestamp_12h(message.at_unix_ms),
                    platform_tag(message.platform),
                    message.username,
                    message.text
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let style = TextStyle {
        text,
        size_px: font_size,
        wrap_width: Some(width),
        ..TextStyle::default()
    };
    render_text(&style).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamps_read_like_a_clock() {
        // 21:32:53 local on a fixed date.
        let at = Local
            .with_ymd_and_hms(2026, 7, 7, 21, 32, 53)
            .single()
            .expect("valid local time")
            .timestamp_millis();
        assert_eq!(timestamp_12h(at), "9:32:53 P.M.");
        let morning = Local
            .with_ymd_and_hms(2026, 7, 7, 0, 5, 9)
            .single()
            .expect("valid local time")
            .timestamp_millis();
        assert_eq!(timestamp_12h(morning), "12:05:09 A.M.");
    }

    #[test]
    fn frames_render_transparent_with_every_message_present() {
        let messages = vec![
            ChatMessage {
                platform: "twitch",
                username: "alpha".into(),
                text: "first!".into(),
                at_unix_ms: 1_700_000_000_000,
            },
            ChatMessage {
                platform: "youtube",
                username: "beta".into(),
                text: "hello from yt".into(),
                at_unix_ms: 1_700_000_001_000,
            },
        ];
        let frame = render_chat(&messages, 480, 22.0).expect("renders");
        assert!(frame.width > 0 && frame.height > 0);
        // Transparent background: corners carry zero alpha.
        assert_eq!(frame.data[3], 0, "top-left pixel is transparent");
        // Some ink exists somewhere.
        assert!(frame.data.chunks_exact(4).any(|px| px[3] > 0));
    }
}
