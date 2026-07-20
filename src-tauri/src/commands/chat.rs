//! V1-E — the featured chat banner's command surface: list the recent
//! cross-overlay chat lines for the picker, and pin/clear the banner. The
//! pin is transient by design (a banner should never survive a restart);
//! only its colors persist, in `Settings::featured_banner`.

use serde::{Deserialize, Serialize};

/// One chat line as the UI sees it (`ChatMessage` with the platform tag
/// flattened to a plain string).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatLine {
    pub platform: String,
    pub username: String,
    pub text: String,
    pub at_unix_ms: i64,
}

impl From<fcap_sources::chat::ChatMessage> for ChatLine {
    fn from(message: fcap_sources::chat::ChatMessage) -> Self {
        Self {
            platform: message.platform.to_string(),
            username: message.username,
            text: message.text,
            at_unix_ms: message.at_unix_ms,
        }
    }
}

/// The newest chat lines across every running overlay (oldest→newest) —
/// the featured-banner picker's feed.
#[tauri::command]
pub fn chat_recent(count: Option<u32>) -> Vec<ChatLine> {
    let count = count.unwrap_or(50).clamp(1, 200) as usize;
    fcap_sources::chat::recent_messages(count)
        .into_iter()
        .map(ChatLine::from)
        .collect()
}

/// Pin a message to the program's featured banner; `None` clears it.
/// Input runs through ingest's OWN sanitizer (the UI normally echoes back a
/// line from `chat_recent`, but the renderer never trusts that).
#[tauri::command]
pub fn chat_feature(message: Option<ChatLine>) {
    fcap_sources::chat::set_featured(message.and_then(|line| {
        fcap_sources::chat::sanitize_line(
            &line.platform,
            &line.username,
            &line.text,
            line.at_unix_ms,
        )
    }));
}

/// The currently pinned message, if any (the picker shows the active pin).
#[tauri::command]
pub fn chat_featured() -> Option<ChatLine> {
    fcap_sources::chat::featured().map(ChatLine::from)
}
