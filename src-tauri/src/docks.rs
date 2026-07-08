//! Browser docks (Phase 7, TASK-702): a URL (chat popout, alerts panel,
//! Companion web buttons) opened as its own dock window beside the studio.
//!
//! Each dock is a separate Tauri `WebviewWindow` pointed at the external
//! URL — never an iframe inside the studio webview (the app CSP stays
//! strict), and the dock window is **not** in the app's capability set, so
//! the remote page gets no IPC surface at all: it renders, and that's it.
//! Docks open only on an explicit user click; the list persists in settings.

use tauri::{AppHandle, Manager, Url, WebviewUrl, WebviewWindowBuilder};

/// Validate a dock URL: http(s) only, bounded, no control characters.
fn validate_dock_url(url: &str) -> Result<Url, String> {
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
fn dock_label(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    format!("dock-{}", cleaned.to_ascii_lowercase())
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
    fn labels_are_windowsafe_and_stable() {
        assert_eq!(dock_label("Twitch Chat"), "dock-twitch-chat");
        assert_eq!(dock_label("émoji �à"), "dock--moji---");
        assert_eq!(dock_label(""), "dock-");
    }
}
