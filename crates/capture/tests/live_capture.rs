//! Live-hardware smoke tests: prove real frames flow from the OS capture
//! pipelines. `#[ignore]` because CI runners are headless — run explicitly
//! with `cargo test -p fcap-capture -- --ignored` on a real desktop.
//!
//! Run ONE TEST AT A TIME (pass its name). The OS capture stacks tear down
//! racily when several sessions run inside one test process — a pre-existing
//! WinRT/D3D teardown race that intermittently ACCESS_VIOLATIONs the harness
//! even with `--test-threads=1`. Every test passes alone.

use std::time::{Duration, Instant};

use fcap_capture::{list_sources, start_capture, SourceKind};

/// Pull frames for up to `window`, returning how many arrived.
fn count_frames(session: &fcap_capture::CaptureSession, window: Duration) -> usize {
    let deadline = Instant::now() + window;
    let mut frames = 0;
    while Instant::now() < deadline {
        match session.frames().recv_timeout(Duration::from_millis(200)) {
            Ok(Some(frame)) => {
                assert!(frame.width > 0 && frame.height > 0);
                assert!(frame.stride >= frame.width * 4);
                assert!(frame.data.len() >= frame.stride as usize * frame.height as usize);
                frames += 1;
            }
            Ok(None) => {}
            Err(err) => panic!("capture ended early: {err}"),
        }
    }
    frames
}

#[test]
#[ignore = "needs a real display session (not headless CI)"]
fn display_capture_delivers_frames() {
    let sources = list_sources().expect("list sources");
    let display = sources
        .iter()
        .find(|s| s.kind == SourceKind::Display)
        .expect("at least one display");
    let session = start_capture(&display.id).expect("start display capture");
    // Displays only produce frames on change; nudge expectations low but real.
    let frames = count_frames(&session, Duration::from_secs(3));
    session.stop();
    assert!(
        frames >= 1,
        "expected at least one display frame, got {frames}"
    );
}

#[test]
#[ignore = "needs a real display session (not headless CI)"]
fn window_capture_delivers_frames() {
    let sources = list_sources().expect("list sources");
    let window = sources
        .iter()
        .find(|s| s.kind == SourceKind::Window)
        .expect("at least one visible window");
    let session = start_capture(&window.id).expect("start window capture");
    let frames = count_frames(&session, Duration::from_secs(3));
    session.stop();
    assert!(
        frames >= 1,
        "expected at least one window frame, got {frames}"
    );
}

/// The "OBS re-watch on restart" behavior: a persisted Window Capture holds a
/// HWND that is stale after any restart, so `start_capture` must re-bind to the
/// same window by its durable identity (executable + class + title). We fake a
/// stale handle by zeroing the HWND in an otherwise-real id, then prove real
/// pixels still flow — the window was re-found, not just re-validated.
#[test]
#[ignore = "needs a real display session (not headless CI)"]
fn window_capture_rebinds_after_a_stale_handle() {
    let sources = list_sources().expect("list sources");
    let window = sources
        .iter()
        .find(|s| s.kind == SourceKind::Window)
        .expect("at least one visible window");

    // The id is `window:<hwnd>:<b64 exe>:<b64 class>:<b64 title>`. Replace only
    // the handle with 0 (never a live window) so the fast path is skipped and
    // the identity re-resolver has to do the work.
    let payload = window
        .id
        .strip_prefix("window:")
        .expect("a window id starts with the window prefix");
    let mut fields: Vec<&str> = payload.split(':').collect();
    assert_eq!(fields.len(), 4, "the id carries the durable identity");
    fields[0] = "0";
    let stale_id = format!("window:{}", fields.join(":"));

    let session = start_capture(&stale_id).expect("re-bind the window by identity");
    let frames = count_frames(&session, Duration::from_secs(3));
    session.stop();
    assert!(
        frames >= 1,
        "expected the re-bound window to deliver frames, got {frames}"
    );
}

/// Spawn a titled console window and return (child, its HWND). The window is
/// moved to `rect` (x, y, w, h in physical px) so tests control occlusion.
#[cfg(target_os = "windows")]
fn spawn_console_at(title: &str, rect: (i32, i32, i32, i32)) -> (std::process::Child, isize) {
    use std::os::windows::process::CommandExt;
    use windows::core::HSTRING;
    use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, MoveWindow};
    const CREATE_NEW_CONSOLE: u32 = 0x0000_0010;

    let child = std::process::Command::new("cmd")
        .args(["/k", &format!("title {title}")])
        .creation_flags(CREATE_NEW_CONSOLE)
        .spawn()
        .expect("spawn a probe console");

    let deadline = Instant::now() + Duration::from_secs(10);
    let hwnd = loop {
        // SAFETY: FindWindowW by exact title; null class matches any.
        let found = unsafe { FindWindowW(None, &HSTRING::from(title)) };
        if let Ok(hwnd) = found {
            if !hwnd.is_invalid() {
                break hwnd;
            }
        }
        assert!(
            Instant::now() < deadline,
            "console '{title}' never appeared"
        );
        std::thread::sleep(Duration::from_millis(100));
    };
    // SAFETY: positioning our own live window.
    unsafe {
        let _ = MoveWindow(hwnd, rect.0, rect.1, rect.2, rect.3, true);
    }
    std::thread::sleep(Duration::from_millis(300));
    (child, hwnd.0 as isize)
}

#[cfg(target_os = "windows")]
fn capture_id_for(title: &str) -> String {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if let Some(s) = list_sources()
            .expect("list sources")
            .into_iter()
            .find(|s| s.kind == SourceKind::Window && s.label.contains(title))
        {
            return s.id;
        }
        assert!(Instant::now() < deadline, "'{title}' never listed");
        std::thread::sleep(Duration::from_millis(200));
    }
}

/// The cursor must keep tracking over a captured window that is NOT focused,
/// as long as the cursor is over the window's *visible* pixels. WGC only
/// composites the cursor for the focused window (and sends no frames for
/// cursor-only motion over an unfocused one), so `win/pointer.rs` draws it and
/// synthesizes frames. Target on the left, an unrelated focus-holder on the
/// right (no overlap), cursor wiggled inside the target → steady frames.
#[test]
#[cfg(target_os = "windows")]
#[ignore = "needs a real display session (not headless CI)"]
fn window_capture_tracks_cursor_when_unfocused() {
    use windows::Win32::UI::WindowsAndMessaging::{SetCursorPos, SetProcessDPIAware};
    // SAFETY: plain process-wide DPI opt-in for physical-pixel coordinates.
    unsafe {
        let _ = SetProcessDPIAware();
    }

    let (mut target, _target_hwnd) = spawn_console_at("fcap-cursor-target", (60, 80, 760, 560));
    // The focus-holder, spawned last so it owns the foreground, well clear of
    // the target's rect.
    let (mut thief, _) = spawn_console_at("fcap-cursor-focus-thief", (980, 80, 620, 420));
    let id = capture_id_for("fcap-cursor-target");
    let (cx, cy) = (440, 360); // inside the target's left-side rect

    let session = start_capture(&id).expect("capture the unfocused console");
    let start = Instant::now();
    let mut frames = 0;
    let mut i = 0u64;
    while start.elapsed() < Duration::from_secs(3) {
        let dx = ((i as f64) * 0.35).sin() * 120.0;
        let dy = ((i as f64) * 0.35).cos() * 80.0;
        // SAFETY: plain cursor move on the interactive desktop.
        unsafe {
            let _ = SetCursorPos(cx + dx as i32, cy + dy as i32);
        }
        i += 1;
        match session.frames().recv_timeout(Duration::from_millis(10)) {
            Ok(Some(_)) => frames += 1,
            Ok(None) => {}
            Err(err) => panic!("capture ended early: {err}"),
        }
    }
    session.stop();
    let _ = target.kill();
    let _ = target.wait();
    let _ = thief.kill();
    let _ = thief.wait();
    assert!(
        frames >= 30,
        "expected a steady cursor-driven frame flow over the visible unfocused window, got {frames} in 3s"
    );
}

/// The cursor must NOT be drawn where another window covers the captured one
/// (the occlusion hit-test — OBS's behaviour). An occluding console is placed
/// directly over the target and the cursor parked on the overlap: since the
/// captured content is static and the cursor resolves to the *other* window,
/// `pointer.rs` draws nothing, so no cursor-driven frames are synthesized
/// (only WGC's initial content frame or two).
#[test]
#[cfg(target_os = "windows")]
#[ignore = "needs a real display session (not headless CI)"]
fn window_capture_hides_cursor_when_occluded() {
    use windows::Win32::UI::WindowsAndMessaging::{SetCursorPos, SetProcessDPIAware};
    // SAFETY: plain process-wide DPI opt-in.
    unsafe {
        let _ = SetProcessDPIAware();
    }

    let rect = (120, 120, 700, 520);
    let (mut target, _) = spawn_console_at("fcap-occ-target", rect);
    // The occluder covers the same rect and, spawned last, sits on top.
    let (mut occluder, _) = spawn_console_at("fcap-occ-cover", rect);
    let id = capture_id_for("fcap-occ-target");
    // Park the cursor squarely on the overlap (over the occluder, not us).
    // SAFETY: plain cursor move.
    unsafe {
        let _ = SetCursorPos(rect.0 + 350, rect.1 + 260);
    }

    let session = start_capture(&id).expect("capture the occluded console");
    let start = Instant::now();
    let mut frames = 0;
    let mut i = 0u64;
    while start.elapsed() < Duration::from_secs(2) {
        // Jiggle a little *within the occluded region* — still over the
        // occluder, so a rect-only cursor test would (wrongly) stream frames.
        let dx = ((i as f64) * 0.3).sin() * 60.0;
        // SAFETY: plain cursor move.
        unsafe {
            let _ = SetCursorPos(rect.0 + 350 + dx as i32, rect.1 + 260);
        }
        i += 1;
        match session.frames().recv_timeout(Duration::from_millis(10)) {
            Ok(Some(_)) => frames += 1,
            Ok(None) => {}
            Err(err) => panic!("capture ended early: {err}"),
        }
    }
    session.stop();
    let _ = target.kill();
    let _ = target.wait();
    let _ = occluder.kill();
    let _ = occluder.wait();
    assert!(
        frames <= 3,
        "the cursor was drawn over an occluding window — expected ~0 synthesized frames, got {frames}"
    );
}

/// A captured window whose process dies must END the capture with an error —
/// never idle "live" on a frozen last frame (issue #6: WGC's `Closed` event is
/// unreliable for some apps, so `wgc.rs` backstops it with an `IsWindow` poll;
/// the error is what arms the studio's auto-recover). Spawns a throwaway
/// console window with a unique title, captures it, kills the process, and
/// asserts the frame stream errors within a bounded time.
#[test]
#[cfg(target_os = "windows")]
#[ignore = "needs a real display session (not headless CI)"]
fn window_capture_ends_when_the_window_dies() {
    use std::os::windows::process::CommandExt;
    const CREATE_NEW_CONSOLE: u32 = 0x0000_0010;
    const TITLE: &str = "fcap-close-detect-probe";

    let mut child = std::process::Command::new("cmd")
        .args(["/k", &format!("title {TITLE}")])
        .creation_flags(CREATE_NEW_CONSOLE)
        .spawn()
        .expect("spawn the probe console");

    // The console window takes a moment to appear and retitle.
    let deadline = Instant::now() + Duration::from_secs(10);
    let id = loop {
        if let Some(source) = list_sources()
            .expect("list sources")
            .into_iter()
            .find(|s| s.kind == SourceKind::Window && s.label.contains(TITLE))
        {
            break source.id;
        }
        assert!(
            Instant::now() < deadline,
            "the probe console window never appeared in the picker"
        );
        std::thread::sleep(Duration::from_millis(200));
    };

    let session = start_capture(&id).expect("start capturing the probe console");
    let frames = count_frames(&session, Duration::from_secs(3));
    assert!(frames >= 1, "expected live probe frames, got {frames}");

    child.kill().expect("kill the probe console");
    let _ = child.wait();

    // The stream must error (WGC `Closed` or the `IsWindow` poll) — not idle.
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        match session.frames().recv_timeout(Duration::from_millis(200)) {
            Ok(_) => assert!(
                Instant::now() < deadline,
                "the capture kept running after the captured window died"
            ),
            Err(err) => {
                println!("capture ended as expected: {err}");
                break;
            }
        }
    }
    session.stop();
}
