//! Remote Guests (P2P/WebRTC) — the webview → compositor frame bridge.
//!
//! The WebRTC session lives in the webview (vendored PeerJS; media is P2P and
//! only signaling touches the broker). Decoded guest frames are pushed over
//! IPC into a per-source [`FrameSender`] registered here; the receiving half
//! is a normal [`CaptureSession`] the studio drains like any capture source.
//! There is no OS device behind a RemoteGuest source — nothing to auto-recover.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame, FrameSender, PixelFormat};
use fcap_scene::SourceId;

/// Live per-source senders. The studio's `start_session` registers one when a
/// RemoteGuest source starts; `remote_guest_push_frame` looks it up; the
/// session's keepalive thread removes it when the source stops.
fn registry() -> &'static Mutex<HashMap<SourceId, FrameSender>> {
    static REGISTRY: OnceLock<Mutex<HashMap<SourceId, FrameSender>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

fn lock_registry() -> std::sync::MutexGuard<'static, HashMap<SourceId, FrameSender>> {
    registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Open the push channel for a RemoteGuest source and hand back the session
/// the studio drains — the same shape as the OS capture backends.
pub fn start_remote_guest(id: SourceId) -> Result<CaptureSession, CaptureError> {
    let (sender, receiver) = frame_channel();
    lock_registry().insert(id, sender);
    let stop = Arc::new(AtomicBool::new(false));
    let stop_watch = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-remote-guest".into())
        .spawn(move || {
            while !stop_watch.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(100));
            }
            lock_registry().remove(&id);
        })
        .map_err(|err| CaptureError::Backend(format!("remote-guest keepalive: {err}")))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

/// Push one RGBA frame from the webview's WebRTC session into a RemoteGuest
/// source. Raw-payload command: the pixels ride as the binary request body
/// (no JSON/base64 round-trip); the metadata rides in headers.
#[tauri::command]
pub fn remote_guest_push_frame(request: tauri::ipc::Request<'_>) -> Result<(), String> {
    let tauri::ipc::InvokeBody::Raw(data) = request.body() else {
        return Err("expected a raw RGBA payload".into());
    };
    let header =
        |name: &str| -> Option<&str> { request.headers().get(name).and_then(|v| v.to_str().ok()) };
    let source_id: SourceId = header("x-fcap-source")
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.into())).ok())
        .ok_or("missing/invalid x-fcap-source header")?;
    let width: u32 = header("x-fcap-width")
        .and_then(|s| s.parse().ok())
        .filter(|w| *w > 0)
        .ok_or("missing/invalid x-fcap-width header")?;
    let height: u32 = header("x-fcap-height")
        .and_then(|s| s.parse().ok())
        .filter(|h| *h > 0)
        .ok_or("missing/invalid x-fcap-height header")?;
    push_frame(source_id, width, height, data)
}

/// The push core (unit-testable without a `tauri::ipc::Request`).
fn push_frame(source_id: SourceId, width: u32, height: u32, data: &[u8]) -> Result<(), String> {
    let expected = width as usize * height as usize * 4;
    if data.len() < expected {
        return Err(format!(
            "frame body too short: {} bytes for {width}x{height}",
            data.len()
        ));
    }

    // Session not started (yet, or already stopped): drop the frame silently —
    // the studio's reconcile opens the channel within a tick.
    let Some(sender) = lock_registry().get(&source_id).cloned() else {
        return Ok(());
    };
    sender.send(Frame {
        width,
        height,
        stride: width * 4,
        format: PixelFormat::Rgba8,
        data: data[..expected].to_vec(),
        captured_at: Instant::now(),
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The whole seam end to end minus IPC: register → push → the session's
    /// receiver yields the frame → stop unregisters (pushes become no-ops).
    #[test]
    fn pushed_frames_reach_the_session_and_stop_unregisters() {
        let id = SourceId::new();
        let session = start_remote_guest(id).expect("open the push channel");

        let rgba = vec![7u8; 4 * 2 * 4];
        push_frame(id, 4, 2, &rgba).expect("push accepts a well-formed frame");
        let frame = session
            .frames()
            .recv_timeout(Duration::from_secs(1))
            .expect("channel open")
            .expect("a frame is waiting");
        assert_eq!((frame.width, frame.height, frame.stride), (4, 2, 16));
        assert_eq!(frame.format, PixelFormat::Rgba8);
        assert_eq!(frame.data, rgba);

        push_frame(id, 4, 2, &rgba[..8]).expect_err("short body rejected");

        session.stop();
        // The keepalive removes the sender within a few of its 100 ms ticks.
        let deadline = Instant::now() + Duration::from_secs(2);
        while lock_registry().contains_key(&id) {
            assert!(Instant::now() < deadline, "stop never unregistered");
            std::thread::sleep(Duration::from_millis(25));
        }
        push_frame(id, 4, 2, &rgba).expect("unknown source drops silently");
    }
}
