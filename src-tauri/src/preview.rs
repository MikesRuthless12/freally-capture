//! The program-frame pipe: the newest composed JPEG behind `preview://`.
//!
//! Phase 2 replaced the Phase 1 single-source pump with the studio render
//! thread (`studio.rs`): the compositor's program frame is downscaled +
//! JPEG-encoded at ~30 fps and parked here; the `preview://` custom URI
//! scheme serves it to the UI's canvas. No sockets, no disk — frames never
//! leave the process (the privacy invariant).

use std::borrow::Cow;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

/// One encoded program frame, served by the `preview://` scheme.
struct EncodedFrame {
    jpeg: Vec<u8>,
    seq: u64,
}

/// Managed state: the newest encoded program frame, plus (Studio Mode) the
/// preview pane's frame — routed by the request path.
#[derive(Default)]
pub struct PreviewState {
    latest: Mutex<Option<EncodedFrame>>,
    /// Studio Mode's preview-side scene (`/studio-preview`).
    studio_preview: Mutex<Option<EncodedFrame>>,
    frame_seq: AtomicU64,
}

impl PreviewState {
    /// Park a freshly encoded program JPEG (called by the studio thread).
    pub fn publish(&self, jpeg: Vec<u8>) {
        let seq = self.frame_seq.fetch_add(1, Ordering::Relaxed) + 1;
        *self.latest.lock().expect("preview frame slot poisoned") =
            Some(EncodedFrame { jpeg, seq });
    }

    /// Park the Studio-Mode preview pane's JPEG (None clears it — mode off).
    pub fn publish_studio_preview(&self, jpeg: Option<Vec<u8>>) {
        let seq = self.frame_seq.fetch_add(1, Ordering::Relaxed) + 1;
        *self
            .studio_preview
            .lock()
            .expect("studio preview slot poisoned") = jpeg.map(|jpeg| EncodedFrame { jpeg, seq });
    }

    /// The `preview://` scheme body: newest JPEG or 204 while there is none.
    ///
    /// CORS is pinned to the app's own webview origins — never `*` — so that
    /// composed program frames can never be fetched by remote content, even
    /// when future phases render remote pages (browser sources).
    pub fn protocol_response(
        &self,
        origin: Option<&str>,
        path: &str,
    ) -> tauri::http::Response<Cow<'static, [u8]>> {
        const APP_ORIGINS: [&str; 3] = [
            "http://tauri.localhost", // Windows production webview
            "tauri://localhost",      // macOS/Linux production webview
            "http://localhost:1420",  // `tauri dev`
        ];
        let allow_origin = origin
            .filter(|candidate| APP_ORIGINS.contains(candidate))
            .unwrap_or(APP_ORIGINS[0]);

        let slot = if path.ends_with("studio-preview") {
            &self.studio_preview
        } else {
            &self.latest
        };
        let latest = slot.lock().expect("preview frame slot poisoned");
        match latest.as_ref() {
            Some(frame) => tauri::http::Response::builder()
                .status(200)
                .header("content-type", "image/jpeg")
                .header("cache-control", "no-store")
                .header("access-control-allow-origin", allow_origin)
                .header("access-control-expose-headers", "x-frame-seq")
                .header("x-frame-seq", frame.seq.to_string())
                .body(Cow::Owned(frame.jpeg.clone()))
                .expect("static response parts"),
            None => tauri::http::Response::builder()
                .status(204)
                .header("cache-control", "no-store")
                .header("access-control-allow-origin", allow_origin)
                .body(Cow::Borrowed(&[][..]))
                .expect("static response parts"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_slot_serves_204() {
        let state = PreviewState::default();
        let response = state.protocol_response(Some("http://tauri.localhost"), "/frame");
        assert_eq!(response.status(), 204);
    }

    #[test]
    fn published_frames_serve_with_a_growing_seq() {
        let state = PreviewState::default();
        state.publish(vec![0xFF, 0xD8, 1]);
        let first = state.protocol_response(Some("http://tauri.localhost"), "/frame");
        assert_eq!(first.status(), 200);
        let first_seq = first.headers()["x-frame-seq"].to_str().unwrap().to_owned();

        state.publish(vec![0xFF, 0xD8, 2]);
        let second = state.protocol_response(Some("http://tauri.localhost"), "/frame");
        let second_seq = second.headers()["x-frame-seq"].to_str().unwrap();
        assert_ne!(first_seq, second_seq, "the poller sees a new frame");
        assert_eq!(second.body().as_ref(), &[0xFF, 0xD8, 2]);
    }

    #[test]
    fn cors_stays_pinned_to_app_origins() {
        let state = PreviewState::default();
        let evil = state.protocol_response(Some("https://evil.example"), "/frame");
        assert_eq!(
            evil.headers()["access-control-allow-origin"],
            "http://tauri.localhost",
            "unknown origins never get themselves echoed back"
        );
    }

    #[test]
    fn studio_preview_rides_its_own_slot() {
        let state = PreviewState::default();
        state.publish(vec![1]);
        state.publish_studio_preview(Some(vec![2]));
        let program = state.protocol_response(Some("http://tauri.localhost"), "/frame");
        let preview = state.protocol_response(Some("http://tauri.localhost"), "/studio-preview");
        assert_eq!(program.body().as_ref(), &[1]);
        assert_eq!(preview.body().as_ref(), &[2]);
        // Clearing (mode off) returns the pane to 204.
        state.publish_studio_preview(None);
        let cleared = state.protocol_response(Some("http://tauri.localhost"), "/studio-preview");
        assert_eq!(cleared.status(), 204);
    }
}
