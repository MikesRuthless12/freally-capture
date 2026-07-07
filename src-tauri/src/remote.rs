//! Remote Guests (P2P/WebRTC) — the webview → compositor + mixer bridge.
//!
//! The WebRTC session lives in the webview (vendored PeerJS; media is P2P and
//! only signaling touches the broker). The guest's video frames are pushed
//! over IPC into a per-source [`FrameSender`] (drained by the studio like any
//! capture), and the guest's mic audio into the same media-hub ring the Media
//! source uses (drained by the mixer). Both are registered here per source;
//! there is no OS device behind a RemoteGuest source — nothing to auto-recover.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use fcap_audio::capture::CaptureRing;
use fcap_audio::media_hub;
use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame, FrameSender, PixelFormat};
use fcap_scene::SourceId;

/// The live channels a RemoteGuest source pushes into: its video sender and
/// the mixer audio ring (held so the ring outlives brief engine gaps).
struct GuestChannels {
    video: FrameSender,
    audio: Arc<CaptureRing>,
}

/// Live per-source channels. `start_remote_guest` registers an entry;
/// `remote_guest_push_frame`/`_audio` look it up; the session's keepalive
/// thread removes it when the source stops.
fn registry() -> &'static Mutex<HashMap<SourceId, GuestChannels>> {
    static REGISTRY: OnceLock<Mutex<HashMap<SourceId, GuestChannels>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

fn lock_registry() -> std::sync::MutexGuard<'static, HashMap<SourceId, GuestChannels>> {
    registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// A `freally://` invite that arrived BEFORE the webview was listening — the
/// cold-start deep link (the plugin emits its launch-arg URL during init,
/// long before the UI registers `remote-invite`). Stored here; the UI takes
/// it once on startup.
fn pending_invite() -> &'static Mutex<Option<String>> {
    static PENDING: OnceLock<Mutex<Option<String>>> = OnceLock::new();
    PENDING.get_or_init(|| Mutex::new(None))
}

/// Stash a deep-link URL for the UI to pick up on startup.
pub fn store_pending_invite(url: String) {
    let mut guard = pending_invite()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    *guard = Some(url);
}

/// One-shot pickup of a cold-start invite (the UI calls this once its
/// `remote-invite` listener is registered). The URL is untrusted input — the
/// webview parses it with the invite validator and only shows a join prompt.
#[tauri::command]
pub fn remote_pending_invite() -> Option<String> {
    pending_invite()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .take()
}

/// Open the push channels for a RemoteGuest source and hand back the video
/// session the studio drains — the same shape as the OS capture backends.
pub fn start_remote_guest(id: SourceId) -> Result<CaptureSession, CaptureError> {
    let (sender, receiver) = frame_channel();
    // Rendezvous on the same hub ring the audio engine drains for this id.
    let audio = media_hub::ring(&id.0.to_string());
    lock_registry().insert(
        id,
        GuestChannels {
            video: sender,
            audio,
        },
    );
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
    let Some(sender) = lock_registry().get(&source_id).map(|g| g.video.clone()) else {
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

/// Push the guest's mic audio (interleaved stereo **48 kHz** f32, little-endian
/// bytes) from the webview's WebRTC session into the mixer. Raw-payload command
/// like the video path; the source id rides in a header.
#[tauri::command]
pub fn remote_guest_push_audio(request: tauri::ipc::Request<'_>) -> Result<(), String> {
    let tauri::ipc::InvokeBody::Raw(data) = request.body() else {
        return Err("expected a raw f32 PCM payload".into());
    };
    let source_id: SourceId = request
        .headers()
        .get("x-fcap-source")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.into())).ok())
        .ok_or("missing/invalid x-fcap-source header")?;
    // Reinterpret the byte body as little-endian f32 samples (WebView2 and the
    // Rust host are both x86 LE, so the frontend's Float32Array maps directly).
    let samples: Vec<f32> = data
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .collect();
    push_audio(source_id, &samples);
    Ok(())
}

/// The audio push core (unit-testable without a `tauri::ipc::Request`).
fn push_audio(source_id: SourceId, samples: &[f32]) {
    // Not started yet / already stopped → drop silently, like the video path.
    if let Some(ring) = lock_registry()
        .get(&source_id)
        .map(|g| Arc::clone(&g.audio))
    {
        ring.push(samples);
    }
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

    /// Guest audio pushed for a live source lands in the same hub ring the
    /// mixer drains (keyed by the source id); after stop, pushes are dropped.
    #[test]
    fn pushed_audio_reaches_the_hub_ring() {
        let id = SourceId::new();
        let session = start_remote_guest(id).expect("open the push channels");
        let ring = media_hub::ring(&id.0.to_string());
        assert_eq!(ring.len(), 0);

        push_audio(id, &[0.1, -0.1, 0.2, -0.2]); // 2 interleaved stereo frames
        assert_eq!(ring.len(), 4, "the guest samples landed in the mixer ring");

        session.stop();
        let deadline = Instant::now() + Duration::from_secs(2);
        while lock_registry().contains_key(&id) {
            assert!(Instant::now() < deadline, "stop never unregistered");
            std::thread::sleep(Duration::from_millis(25));
        }
        // No panic / no registry entry: an unknown source drops silently.
        push_audio(id, &[0.0, 0.0]);
    }
}
