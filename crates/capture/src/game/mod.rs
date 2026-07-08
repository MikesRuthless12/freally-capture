//! Game capture (GPU-API hook) — the seam + the honest risk model (Phase 8,
//! TASK-801).
//!
//! "Game capture" in OBS-class tools means **injecting** into the game process
//! and hooking its DX/GL/Vulkan present call to copy the swap-chain frame. That
//! injection is powerful but genuinely risky, so this module is deliberately the
//! *seam* — a per-OS capability query, the explicit consent/risk text, and the
//! working fallback — not a silent injector:
//!
//! - **Never inject silently.** The hook is opt-in per game, behind an explicit
//!   acknowledgement of the risk below. Nothing here injects anything on its own.
//! - **Anti-cheat + AV honesty.** Injecting a hook into a protected game can be
//!   flagged by anti-cheat as tampering (**up to an account ban**) and by AV as
//!   suspicious. That warning is surfaced *in-product*, not buried.
//! - **Wayland has no hook.** On Wayland, injected global capture is impossible
//!   by design — the honest path is the ScreenCast **portal** (the user picks
//!   the game window in the system dialog). Said plainly.
//!
//! Until the injected hook lands (its own flagged milestone — it needs a signed
//! helper + per-anti-cheat compatibility work), **Window Capture** already
//! captures the vast majority of games run in *borderless/windowed* mode with
//! zero injection and zero anti-cheat exposure, so that is the recommended path
//! and what [`GameCaptureStatus::fallback`] names.
//!
//! This module is safe (`deny(unsafe_code)` at the crate root holds); the
//! injection milestone would add per-OS submodules with isolated `unsafe`, the
//! same way `win`/`macos` isolate the screen-capture `unsafe`.

/// How game capture can work on the current OS/session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCaptureSupport {
    /// Windows / Linux-X11: an injected GPU-API hook is *architecturally*
    /// possible here — but it is a flagged milestone, not yet shipped. Window
    /// Capture is the working path today.
    HookPlanned,
    /// Wayland: no injected hook is possible; the ScreenCast portal is the path.
    PortalOnly,
    /// macOS: no injection model; ScreenCaptureKit window capture is the path.
    WindowCaptureOnly,
}

/// The recommended working capture path while the injected hook is a milestone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCaptureFallback {
    /// Capture the game as a window (borderless/windowed) — no injection.
    WindowCapture,
    /// Wayland: pick the game window in the ScreenCast portal dialog.
    Portal,
}

/// The full honest picture the UI shows for a "Game Capture" request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameCaptureStatus {
    pub support: GameCaptureSupport,
    /// True where an injected hook is even architecturally possible (Win/X11).
    pub hook_possible: bool,
    /// The anti-cheat / AV consent text (always shown before any hook).
    pub risk: String,
    /// The working path to use today.
    pub fallback: GameCaptureFallback,
    /// A one-line, user-facing summary of the fallback recommendation.
    pub guidance: String,
}

/// The anti-cheat / AV risk consent — shown *before* any injected hook could be
/// enabled. Deliberately blunt.
pub fn risk_warning() -> String {
    "Game Capture works by injecting a hook into the game to copy its GPU frames. \
     Anti-cheat systems can treat that injection as tampering — in competitive \
     games this may get your account BANNED — and antivirus may flag it. Freally \
     Capture never injects silently: it stays off until you explicitly enable it \
     for a specific game, and even then, prefer Window Capture (below) for any \
     game with anti-cheat."
        .to_string()
}

/// Probe how game capture can work here, with the honest fallback + guidance.
pub fn status() -> GameCaptureStatus {
    #[cfg(target_os = "windows")]
    {
        game_status(GameCaptureSupport::HookPlanned)
    }
    #[cfg(target_os = "macos")]
    {
        game_status(GameCaptureSupport::WindowCaptureOnly)
    }
    #[cfg(target_os = "linux")]
    {
        // Wayland can't be hooked; X11 could (still a milestone). Prefer the
        // honest Wayland answer when we're on Wayland, else the X11 answer.
        if is_wayland() {
            game_status(GameCaptureSupport::PortalOnly)
        } else {
            game_status(GameCaptureSupport::HookPlanned)
        }
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        game_status(GameCaptureSupport::WindowCaptureOnly)
    }
}

/// Build the status for a given support level (pure — the per-OS branch above
/// only chooses the level, so this is what the tests exercise).
fn game_status(support: GameCaptureSupport) -> GameCaptureStatus {
    let (hook_possible, fallback, guidance) = match support {
        GameCaptureSupport::HookPlanned => (
            true,
            GameCaptureFallback::WindowCapture,
            "The injected GPU-API hook is a flagged, opt-in milestone. Today, run the \
             game in borderless/windowed mode and add it as a Window Capture — it \
             captures most games with no injection and no anti-cheat risk."
                .to_string(),
        ),
        GameCaptureSupport::WindowCaptureOnly => (
            false,
            GameCaptureFallback::WindowCapture,
            "This OS has no game-injection model. Add the game as a Window Capture \
             (borderless/windowed) — no injection, no anti-cheat risk."
                .to_string(),
        ),
        GameCaptureSupport::PortalOnly => (
            false,
            GameCaptureFallback::Portal,
            "Wayland can't be hooked. Add a Screen Capture (Portal) source and pick \
             the game window in the system dialog — no injection, no anti-cheat risk."
                .to_string(),
        ),
    };
    GameCaptureStatus {
        support,
        hook_possible,
        risk: risk_warning(),
        fallback,
        guidance,
    }
}

/// Best-effort Wayland detection (session type / display env).
#[cfg(target_os = "linux")]
fn is_wayland() -> bool {
    std::env::var("WAYLAND_DISPLAY").is_ok()
        || std::env::var("XDG_SESSION_TYPE")
            .map(|s| s.eq_ignore_ascii_case("wayland"))
            .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hook_paths_recommend_window_capture_and_flag_the_risk() {
        let s = game_status(GameCaptureSupport::HookPlanned);
        assert!(s.hook_possible);
        assert_eq!(s.fallback, GameCaptureFallback::WindowCapture);
        // The risk text must name the ban risk — this is the "flag it" rule.
        assert!(s.risk.to_uppercase().contains("BAN"));
        assert!(s.guidance.to_lowercase().contains("window capture"));
    }

    #[test]
    fn wayland_is_portal_only_and_never_claims_a_hook() {
        let s = game_status(GameCaptureSupport::PortalOnly);
        assert!(!s.hook_possible);
        assert_eq!(s.fallback, GameCaptureFallback::Portal);
        assert!(s.guidance.to_lowercase().contains("portal"));
    }

    #[test]
    fn macos_is_window_capture_only() {
        let s = game_status(GameCaptureSupport::WindowCaptureOnly);
        assert!(!s.hook_possible);
        assert_eq!(s.fallback, GameCaptureFallback::WindowCapture);
    }

    #[test]
    fn live_status_is_self_consistent() {
        // Whatever this OS reports, it never claims a hook is possible while
        // pointing at a non-Window fallback, and always carries the risk text.
        let s = status();
        assert!(!s.risk.is_empty());
        if s.support == GameCaptureSupport::PortalOnly {
            assert!(!s.hook_possible);
        }
    }
}
