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

/// V1-E: the featured-message banner — an opaque rounded bar (`bg`) with a
/// small `[TW] name` header over the message text (`fg`), sized to `width`.
/// The studio pins it to the bottom of the program; this just draws the bar.
pub(crate) fn render_featured_banner(
    message: &ChatMessage,
    width: u32,
    bg: [u8; 4],
    fg: [u8; 4],
) -> Result<Frame, String> {
    let width = width.clamp(240, 3840);
    const PAD: u32 = 18;
    const GAP: u32 = 6;
    const RADIUS: u32 = 14;
    let wrap = width.saturating_sub(PAD * 2);

    let header = render_text(&TextStyle {
        text: format!("{} {}", platform_tag(message.platform), message.username),
        size_px: 22.0,
        color: [fg[0], fg[1], fg[2], 200],
        wrap_width: Some(wrap),
        ..TextStyle::default()
    })
    .map_err(|err| err.to_string())?;
    let body = render_text(&TextStyle {
        text: message.text.clone(),
        size_px: 34.0,
        color: fg,
        wrap_width: Some(wrap),
        ..TextStyle::default()
    })
    .map_err(|err| err.to_string())?;

    let height = PAD + header.height + GAP + body.height + PAD;
    // The shared face-painter primitives: a rounded bar on the clear canvas,
    // then the straight-alpha over-blit every generated face uses.
    let mut data = vec![0u8; (width * height * 4) as usize];
    crate::compose::fill_round_rect(
        &mut data,
        width as usize,
        (0, 0, width as usize, height as usize),
        RADIUS as usize,
        bg,
    );
    let w = width as usize;
    let h = height as usize;
    crate::compose::blit(&mut data, w, h, &header, PAD as i64, PAD as i64);
    crate::compose::blit(
        &mut data,
        w,
        h,
        &body,
        PAD as i64,
        (PAD + header.height + GAP) as i64,
    );
    Ok(crate::static_source::rgba_frame(width, height, data))
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

    #[test]
    fn featured_banner_is_a_rounded_bar_with_ink() {
        let message = ChatMessage {
            platform: "twitch",
            username: "alpha".into(),
            text: "pin me to the program!".into(),
            at_unix_ms: 1_700_000_000_000,
        };
        let bg = [16, 20, 26, 255];
        let fg = [255, 255, 255, 255];
        let frame = render_featured_banner(&message, 900, bg, fg).expect("renders");
        assert_eq!(frame.width, 900);
        assert!(frame.height > 0);
        // Rounded corner: the exact corner pixel is carved transparent…
        assert_eq!(frame.data[3], 0, "top-left corner is outside the arc");
        // …while the bar's mid-left edge carries the background color.
        let mid = ((frame.height / 2 * frame.width) * 4) as usize;
        assert_eq!(&frame.data[mid..mid + 4], &bg);
        // And the text put ink somewhere brighter than the bar.
        assert!(frame
            .data
            .chunks_exact(4)
            .any(|px| px[3] == 255 && px[0] > 100));
    }
}
