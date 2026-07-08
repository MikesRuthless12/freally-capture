//! Browser docks (Phase 7, TASK-702): a URL (chat popout, alerts panel,
//! Companion web buttons) opened as its own dock window beside the studio.
//!
//! Each dock is a separate Tauri `WebviewWindow` pointed at the external
//! URL — never an iframe inside the studio webview (the app CSP stays
//! strict), and the dock window is **not** in the app's capability set, so
//! the remote page gets no IPC surface at all: it renders, and that's it.
//! Docks open only on an explicit user click; the list persists in settings.

use sha2::{Digest, Sha256};
use tauri::{AppHandle, Manager, Url, WebviewUrl, WebviewWindowBuilder};

/// Validate a dock URL: http(s) only, bounded, no control characters. The
/// single source of truth — `settings.rs` validation and this open-time check
/// both go through it, so a URL that saves is a URL that opens (no "saves fine,
/// never opens" divergence).
pub fn validate_dock_url(url: &str) -> Result<Url, String> {
    if url.len() > 2048 || url.chars().any(char::is_control) {
        return Err("invalid dock URL".to_owned());
    }
    let parsed = Url::parse(url).map_err(|err| format!("invalid dock URL: {err}"))?;
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err("a dock URL must be http:// or https://".to_owned());
    }
    Ok(parsed)
}

/// A window label derived from the dock name (labels allow [a-zA-Z0-9-/:_]).
/// A readable sanitized stem PLUS a hash of the *full* name, so distinct names
/// that sanitize to the same stem ("Twitch Chat" vs "Twitch_Chat") get
/// distinct labels — otherwise opening the second would just navigate the
/// first dock's window to a different URL, destroying its page.
fn dock_label(name: &str) -> String {
    let stem: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    let digest = Sha256::digest(name.as_bytes());
    let mut suffix = String::with_capacity(12);
    for byte in &digest[..6] {
        suffix.push_str(&format!("{byte:02x}"));
    }
    format!("dock-{}-{suffix}", stem.to_ascii_lowercase())
}

/// Open (or focus) the named dock window on `url`.
#[tauri::command]
pub fn browser_dock_open(app: AppHandle, name: String, url: String) -> Result<(), String> {
    let parsed = validate_dock_url(&url)?;
    let label = dock_label(&name);
    if let Some(existing) = app.get_webview_window(&label) {
        // Same dock re-opened: refocus; if the URL changed, point it there.
        let _ = existing.set_focus();
        if existing
            .url()
            .map(|current| current != parsed)
            .unwrap_or(false)
        {
            let _ = existing.navigate(parsed);
        }
        return Ok(());
    }
    let title = if name.trim().is_empty() {
        "Dock".to_owned()
    } else {
        name.clone()
    };
    WebviewWindowBuilder::new(&app, &label, WebviewUrl::External(parsed))
        .title(title)
        .inner_size(420.0, 640.0)
        .build()
        .map_err(|err| format!("could not open the dock: {err}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_are_gated_to_http_and_https() {
        assert!(validate_dock_url("https://www.twitch.tv/popout/x/chat").is_ok());
        assert!(validate_dock_url("http://127.0.0.1:8000/companion").is_ok());
        assert!(validate_dock_url("file:///C:/secrets.txt").is_err());
        assert!(validate_dock_url("javascript:alert(1)").is_err());
        assert!(validate_dock_url("ftp://example.com").is_err());
        assert!(validate_dock_url("not a url").is_err());
        assert!(validate_dock_url("https://x.example/\u{0007}").is_err());
    }

    #[test]
    fn labels_are_windowsafe_stable_and_collision_free() {
        // Stable for a given name (starts with the readable stem).
        assert!(dock_label("Twitch Chat").starts_with("dock-twitch-chat-"));
        assert_eq!(dock_label("Twitch Chat"), dock_label("Twitch Chat"));
        // Names that sanitize to the same stem must NOT collide — the whole
        // point of the hash suffix (else opening one navigates the other).
        assert_ne!(dock_label("Twitch Chat"), dock_label("Twitch_Chat"));
        assert_ne!(dock_label("Twitch Chat"), dock_label("twitch-chat"));
        // A label never targets the app's own "main" IPC window.
        assert!(dock_label("main").starts_with("dock-"));
        assert_ne!(dock_label("main"), "main");
    }
}
