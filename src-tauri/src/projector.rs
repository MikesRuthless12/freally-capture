//! Projector + auxiliary output windows (CAP-M07).
//!
//! A projector is a separate in-app (`WebviewUrl::App`) window that shows a
//! clean feed — the program, the Studio-Mode preview, or (later) a single scene
//! or source — fullscreen on a chosen monitor or as a floating window. What it
//! shows is encoded in the **window label** (`projector-program`, …), which the
//! frontend parses; the window itself needs no IPC — it only fetches the
//! `preview://` slot (the app-origin CORS rule lets it), so these windows are
//! deliberately outside the capability set, like browser docks.
//!
//! The same spawn-on-a-display mechanism backs the multiview monitor's
//! "open on any display" (`multiview` label).

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

/// One connected display, for the "open on…" picker.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayInfo {
    pub index: usize,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub primary: bool,
}

/// Enumerate the connected monitors (CAP-M07). Queried through the main window.
#[tauri::command]
pub fn list_displays(app: AppHandle) -> Result<Vec<DisplayInfo>, String> {
    let main = app
        .get_webview_window("main")
        .ok_or("the main window is not available")?;
    let monitors = main.available_monitors().map_err(|err| err.to_string())?;
    let primary_name = main
        .primary_monitor()
        .ok()
        .flatten()
        .and_then(|monitor| monitor.name().cloned());
    Ok(monitors
        .iter()
        .enumerate()
        .map(|(index, monitor)| {
            let size = monitor.size();
            let position = monitor.position();
            let name = monitor.name().cloned();
            DisplayInfo {
                index,
                primary: name.is_some() && name == primary_name,
                name: name.unwrap_or_else(|| format!("Display {}", index + 1)),
                width: size.width,
                height: size.height,
                x: position.x,
                y: position.y,
            }
        })
        .collect())
}

/// The scene/source target a projector label encodes, if any (CAP-M07
/// extension). `projector-scene:<id>` / `projector-source:<id>` map to a render
/// target; program/preview/multiview reuse existing slots and return `None`.
pub fn parse_target(label: &str) -> Option<crate::studio::ProjectorTarget> {
    use crate::studio::ProjectorTarget;
    let rest = label.strip_prefix("projector-")?;
    if let Some(id) = rest.strip_prefix("scene:") {
        let scene: fcap_scene::SceneId = serde_json::from_str(&format!("\"{id}\"")).ok()?;
        return Some(ProjectorTarget::Scene(scene));
    }
    if let Some(id) = rest.strip_prefix("source:") {
        let source: fcap_scene::SourceId = serde_json::from_str(&format!("\"{id}\"")).ok()?;
        return Some(ProjectorTarget::Source(source));
    }
    // CAP-N69: the low-latency passthrough monitor.
    if let Some(id) = rest.strip_prefix("passthrough:") {
        let source: fcap_scene::SourceId = serde_json::from_str(&format!("\"{id}\"")).ok()?;
        return Some(ProjectorTarget::Passthrough(source));
    }
    None
}

/// Tell the render loop a scene/source projector is now open (no-op for
/// program/preview/multiview labels).
fn register_target(app: &AppHandle, label: &str) {
    if let Some(target) = parse_target(label) {
        app.state::<crate::studio::StudioState>()
            .set_projector(target, true);
    }
}

/// Whether `label` is a safe, expected aux-window label.
fn is_valid_label(label: &str) -> bool {
    !label.is_empty()
        && label.len() <= 128
        && (label == "multiview" || label.starts_with("projector-"))
        && label
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == ':')
}

/// Build (or focus) the aux window. **Synchronous builder body**, shared by the
/// async command and `reopen_saved`.
///
/// ⚠️ Must NOT be called from a Tauri command running on the main thread while
/// the event loop is pumping: building a webview that loads our app and issues an
/// IPC call on mount (e.g. the teleprompter projector's `teleprompter_get`)
/// deadlocks — the new webview's IPC cannot be serviced while the main thread is
/// blocked inside `build()`, so the whole app freezes (blank windows, dead close
/// button). The `#[tauri::command]` wrapper below is therefore `async` (Tauri
/// runs it off the main thread, so `build()` marshals creation to the free main
/// event loop and the initial IPC completes). `reopen_saved` calls this directly,
/// but only from `.setup()` — before `app.run()` pumps — where window creation
/// defers the webview load, exactly like the main window itself.
fn open_or_focus_aux(
    app: &AppHandle,
    label: &str,
    title: &str,
    display: Option<usize>,
    fullscreen: bool,
) -> Result<(), String> {
    if !is_valid_label(label) {
        return Err("invalid window label".to_owned());
    }
    if let Some(existing) = app.get_webview_window(label) {
        register_target(app, label);
        let _ = existing.set_focus();
        return Ok(());
    }
    let main = app
        .get_webview_window("main")
        .ok_or("the main window is not available")?;
    let monitors = main.available_monitors().map_err(|err| err.to_string())?;

    let title = if title.trim().is_empty() {
        "Freally Capture"
    } else {
        title
    };
    let target_monitor = display.and_then(|index| monitors.get(index));
    // A "fullscreen" projector is a *borderless window sized to the chosen
    // monitor* — never OS exclusive fullscreen. Asking wry/tao to enter
    // exclusive fullscreen *during window creation* on the primary display
    // deadlocks the Windows event loop mid mode-switch: the whole desktop
    // froze and the app had to be killed from Task Manager. A borderless
    // monitor-filling window is what OBS's "fullscreen projector" actually is,
    // triggers no DWM mode change, and stays escapable (Esc / Alt+F4 / the
    // taskbar all keep working — it is not always-on-top).
    let mut builder = WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
        .title(title)
        .inner_size(960.0, 540.0)
        .decorations(!fullscreen)
        .always_on_top(!fullscreen);
    if let Some(monitor) = target_monitor {
        let position = monitor.position();
        builder = builder.position(position.x as f64, position.y as f64);
    }
    let window = builder
        .build()
        .map_err(|err| format!("could not open the window: {err}"))?;
    // Cover the monitor exactly. Physical units sidestep the logical/scale
    // mismatch on HiDPI displays, so the projector fills the screen edge-to-edge.
    if fullscreen {
        if let Some(monitor) = target_monitor {
            let _ = window.set_position(*monitor.position());
            let _ = window.set_size(*monitor.size());
        }
    }
    register_target(app, label);
    Ok(())
}

/// Open (or focus) an auxiliary window on a chosen display (CAP-M07). `label`
/// says what it shows (the frontend parses it — `projector-program`,
/// `projector-preview`, `multiview`); `display` positions it on that monitor;
/// `fullscreen` fullscreens it there, otherwise it floats always-on-top.
///
/// `async` so Tauri runs it off the main thread — see `open_or_focus_aux`: a
/// synchronous (main-thread) command deadlocks building any window whose webview
/// calls back into IPC on load (the teleprompter projector).
#[tauri::command]
pub async fn aux_window_open(
    app: AppHandle,
    label: String,
    title: String,
    display: Option<usize>,
    fullscreen: bool,
) -> Result<(), String> {
    open_or_focus_aux(&app, &label, &title, display, fullscreen)
}

/// Reopen the projectors remembered from last session (CAP-M07 extension).
/// Scene/source targets are validated against the loaded collection — stale
/// ids (a scene/source that no longer exists) are silently skipped.
pub fn reopen_saved(app: &AppHandle) {
    let Some(workspace) = app.try_state::<crate::profiles::WorkspaceState>() else {
        return;
    };
    let studio = app.state::<crate::studio::StudioState>();
    for saved in workspace.projectors() {
        // Skip a scene/source projector whose target is gone.
        if let Some(target) = parse_target(&saved.label) {
            if !studio.has_target(target) {
                continue;
            }
        }
        let _ = open_or_focus_aux(
            app,
            &saved.label,
            &saved.title,
            saved.display,
            saved.fullscreen,
        );
    }
}

/// Snapshot the currently-open aux windows into the workspace so they reopen
/// next launch (called at exit, while the windows are still alive).
pub fn remember_open(app: &AppHandle) {
    let Some(workspace) = app.try_state::<crate::profiles::WorkspaceState>() else {
        return;
    };
    let monitors = app
        .get_webview_window("main")
        .and_then(|main| main.available_monitors().ok())
        .unwrap_or_default();
    let open: Vec<crate::profiles::ProjectorState> = app
        .webview_windows()
        .into_iter()
        .filter(|(label, _)| label == "multiview" || label.starts_with("projector-"))
        .map(|(label, window)| {
            let display = window
                .current_monitor()
                .ok()
                .flatten()
                .and_then(|c| monitors.iter().position(|m| m.position() == c.position()));
            // A "fullscreen" projector is a borderless window sized to its
            // monitor (OS exclusive fullscreen froze the desktop). It is the
            // ONLY aux window we open without decorations, so decorations are a
            // reliable, resize-proof signal — a floating projector the user
            // dragged to fill the screen still keeps its title bar.
            let fullscreen = !window.is_decorated().unwrap_or(true);
            crate::profiles::ProjectorState {
                title: window.title().unwrap_or_default(),
                fullscreen,
                display,
                label,
            }
        })
        .collect();
    workspace.set_projectors(open);
}

/// Close an aux window by label (used from the opener; the window also has its
/// own close button).
#[tauri::command]
pub fn aux_window_close(app: AppHandle, label: String) {
    if let Some(window) = app.get_webview_window(&label) {
        let _ = window.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_and_source_targets_parse_from_labels() {
        use crate::studio::ProjectorTarget;
        let uuid = "6f9619ff-8b86-d011-b42d-00cf4fc964ff";
        assert!(matches!(
            parse_target(&format!("projector-scene:{uuid}")),
            Some(ProjectorTarget::Scene(_))
        ));
        assert!(matches!(
            parse_target(&format!("projector-source:{uuid}")),
            Some(ProjectorTarget::Source(_))
        ));
        // Program/preview/multiview reuse existing slots — no render target.
        assert!(parse_target("projector-program").is_none());
        assert!(parse_target("projector-preview").is_none());
        assert!(parse_target("multiview").is_none());
        // A malformed id is rejected, not panicked on.
        assert!(parse_target("projector-scene:not-a-uuid").is_none());
    }

    #[test]
    fn passthrough_targets_parse_from_labels() {
        use crate::studio::ProjectorTarget;
        let uuid = "6f9619ff-8b86-d011-b42d-00cf4fc964ff";
        assert!(matches!(
            parse_target(&format!("projector-passthrough:{uuid}")),
            Some(ProjectorTarget::Passthrough(_))
        ));
        assert!(is_valid_label(&format!("projector-passthrough:{uuid}")));
        assert!(parse_target("projector-passthrough:nope").is_none());
    }

    #[test]
    fn labels_are_validated() {
        assert!(is_valid_label("projector-program"));
        assert!(is_valid_label("projector-preview"));
        assert!(is_valid_label("multiview"));
        assert!(is_valid_label("projector-scene:abc-123"));
        // Never the main IPC window, never arbitrary labels, never junk.
        assert!(!is_valid_label("main"));
        assert!(!is_valid_label(""));
        assert!(!is_valid_label("dock-evil"));
        assert!(!is_valid_label("projector-<script>"));
    }
}
