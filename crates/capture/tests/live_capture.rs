//! Live-hardware smoke tests: prove real frames flow from the OS capture
//! pipelines. `#[ignore]` because CI runners are headless — run explicitly
//! with `cargo test -p fcap-capture -- --ignored` on a real desktop.

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
