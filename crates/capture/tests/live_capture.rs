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
