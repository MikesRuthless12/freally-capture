//! Live-hardware webcam smoke test. `#[ignore]` because CI has no cameras —
//! run explicitly with `cargo test -p fcap-sources -- --ignored` on a desktop
//! with a webcam attached.

use std::time::{Duration, Instant};

use fcap_sources::video_device::{list_video_devices, start_video_device};

#[test]
#[ignore = "needs a physical camera (not headless CI)"]
fn webcam_delivers_frames() {
    let devices = list_video_devices().expect("enumerate devices");
    let Some(device) = devices.first() else {
        eprintln!("no camera attached — nothing to smoke-test");
        return;
    };
    let session = start_video_device(&device.id, None).expect("start webcam");
    let deadline = Instant::now() + Duration::from_secs(8); // cams warm up slowly
    let mut frames = 0;
    while Instant::now() < deadline && frames < 5 {
        match session.frames().recv_timeout(Duration::from_millis(500)) {
            Ok(Some(frame)) => {
                assert!(frame.width > 0 && frame.height > 0);
                assert_eq!(
                    frame.stride,
                    frame.width * 4,
                    "webcam frames are tight RGBA"
                );
                frames += 1;
            }
            Ok(None) => {}
            Err(err) => panic!("webcam capture ended early: {err}"),
        }
    }
    session.stop();
    assert!(frames >= 5, "expected 5 webcam frames, got {frames}");
}
