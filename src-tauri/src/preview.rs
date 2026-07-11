//! The program-frame pipe: the newest composed JPEG behind `preview://`.
//!
//! Phase 2 replaced the Phase 1 single-source pump with the studio render
//! thread (`studio.rs`): the compositor's program frame is downscaled +
//! JPEG-encoded at ~30 fps and parked here; the `preview://` custom URI
//! scheme serves it to the UI's canvas. No sockets, no disk — frames never
//! leave the process (the privacy invariant).

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
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
    /// The second (vertical) canvas (`/vertical-preview`).
    vertical_preview: Mutex<Option<EncodedFrame>>,
    /// The keying workbench's single-source view (`/workbench-preview`, CAP-M26).
    workbench_preview: Mutex<Option<EncodedFrame>>,
    /// Per-scene multiview thumbnails, keyed by scene id (`/multiview/<id>`, CAP-M06).
    multiview: Mutex<HashMap<String, EncodedFrame>>,
    /// Full-res scene/source projector frames (CAP-M07 extension), keyed by the
    /// target string `"scene:<id>"` / `"source:<id>"`, served at
    /// `/projector-scene/<id>` / `/projector-source/<id>`.
    projectors: Mutex<HashMap<String, EncodedFrame>>,
    frame_seq: AtomicU64,
}

/// Percent-decode a request path. Tauri's `convertFileSrc` percent-encodes the
/// slot key, so a multi-segment key like `multiview/<id>` arrives at the custom
/// scheme handler as `multiview%2F<id>` (the handler gets the raw, still-encoded
/// path). Single-segment slots have nothing to decode, so this is a no-op there.
fn percent_decode(path: &str) -> Cow<'_, str> {
    if !path.contains('%') {
        return Cow::Borrowed(path);
    }
    let bytes = path.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hi = (bytes[i + 1] as char).to_digit(16);
            let lo = (bytes[i + 2] as char).to_digit(16);
            if let (Some(hi), Some(lo)) = (hi, lo) {
                out.push((hi * 16 + lo) as u8);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    Cow::Owned(String::from_utf8_lossy(&out).into_owned())
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

    /// Park the vertical canvas's JPEG (None clears it — canvas off).
    pub fn publish_vertical_preview(&self, jpeg: Option<Vec<u8>>) {
        let seq = self.frame_seq.fetch_add(1, Ordering::Relaxed) + 1;
        *self
            .vertical_preview
            .lock()
            .expect("vertical preview slot poisoned") = jpeg.map(|jpeg| EncodedFrame { jpeg, seq });
    }

    /// Park the keying workbench's single-source JPEG (None clears it — closed).
    pub fn publish_workbench_preview(&self, jpeg: Option<Vec<u8>>) {
        let seq = self.frame_seq.fetch_add(1, Ordering::Relaxed) + 1;
        *self
            .workbench_preview
            .lock()
            .expect("workbench preview slot poisoned") =
            jpeg.map(|jpeg| EncodedFrame { jpeg, seq });
    }

    /// Park (or clear, with `None`) one multiview scene thumbnail (CAP-M06).
    pub fn publish_multiview(&self, id: &str, jpeg: Option<Vec<u8>>) {
        let seq = self.frame_seq.fetch_add(1, Ordering::Relaxed) + 1;
        let mut map = self.multiview.lock().expect("multiview slot poisoned");
        match jpeg {
            Some(jpeg) => {
                map.insert(id.to_owned(), EncodedFrame { jpeg, seq });
            }
            None => {
                map.remove(id);
            }
        }
    }

    /// Drop every multiview thumbnail (the monitor closed).
    pub fn clear_multiview(&self) {
        self.multiview
            .lock()
            .expect("multiview slot poisoned")
            .clear();
    }

    /// Park (or clear, with `None`) one scene/source projector's full-res frame
    /// (CAP-M07 extension). `key` is `"scene:<id>"` / `"source:<id>"`.
    pub fn publish_projector(&self, key: &str, jpeg: Option<Vec<u8>>) {
        let seq = self.frame_seq.fetch_add(1, Ordering::Relaxed) + 1;
        let mut map = self.projectors.lock().expect("projector slot poisoned");
        match jpeg {
            Some(jpeg) => {
                map.insert(key.to_owned(), EncodedFrame { jpeg, seq });
            }
            None => {
                map.remove(key);
            }
        }
    }

    /// Drop projector slots whose target is no longer open (`live` = open keys).
    pub fn retain_projectors(&self, live: &HashSet<String>) {
        self.projectors
            .lock()
            .expect("projector slot poisoned")
            .retain(|key, _| live.contains(key));
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

        // `convertFileSrc` percent-encodes the `/` in a multi-segment slot key,
        // so decode before routing (single-segment slots are untouched).
        let decoded = percent_decode(path);
        let path = decoded.as_ref();

        // Scene/source projectors (`/projector-scene|source/<id>`) and multiview
        // thumbnails (`/multiview/<id>`) ride per-id maps; everything else rides
        // a fixed slot.
        let projector_key = path
            .rfind("projector-scene/")
            .map(|pos| {
                format!(
                    "scene:{}",
                    path[pos + "projector-scene/".len()..].trim_end_matches('/')
                )
            })
            .or_else(|| {
                path.rfind("projector-source/").map(|pos| {
                    format!(
                        "source:{}",
                        path[pos + "projector-source/".len()..].trim_end_matches('/')
                    )
                })
            });
        let frame: Option<(u64, Vec<u8>)> = if let Some(key) = projector_key {
            self.projectors
                .lock()
                .expect("projector slot poisoned")
                .get(&key)
                .map(|frame| (frame.seq, frame.jpeg.clone()))
        } else if let Some(pos) = path.rfind("multiview/") {
            let id = path[pos + "multiview/".len()..].trim_end_matches('/');
            self.multiview
                .lock()
                .expect("multiview slot poisoned")
                .get(id)
                .map(|frame| (frame.seq, frame.jpeg.clone()))
        } else {
            let slot = if path.ends_with("studio-preview") {
                &self.studio_preview
            } else if path.ends_with("vertical-preview") {
                &self.vertical_preview
            } else if path.ends_with("workbench-preview") {
                &self.workbench_preview
            } else {
                &self.latest
            };
            slot.lock()
                .expect("preview frame slot poisoned")
                .as_ref()
                .map(|frame| (frame.seq, frame.jpeg.clone()))
        };
        match frame {
            Some((seq, jpeg)) => tauri::http::Response::builder()
                .status(200)
                .header("content-type", "image/jpeg")
                .header("cache-control", "no-store")
                .header("access-control-allow-origin", allow_origin)
                .header("access-control-expose-headers", "x-frame-seq")
                .header("x-frame-seq", seq.to_string())
                .body(Cow::Owned(jpeg))
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
    fn vertical_preview_rides_its_own_slot() {
        let state = PreviewState::default();
        state.publish(vec![1]);
        state.publish_vertical_preview(Some(vec![9]));
        let program = state.protocol_response(Some("http://tauri.localhost"), "/frame");
        let vertical = state.protocol_response(Some("http://tauri.localhost"), "/vertical-preview");
        assert_eq!(program.body().as_ref(), &[1]);
        assert_eq!(vertical.body().as_ref(), &[9]);
        state.publish_vertical_preview(None);
        let cleared = state.protocol_response(Some("http://tauri.localhost"), "/vertical-preview");
        assert_eq!(cleared.status(), 204);
    }

    #[test]
    fn workbench_preview_rides_its_own_slot() {
        let state = PreviewState::default();
        state.publish(vec![1]);
        state.publish_workbench_preview(Some(vec![7]));
        let program = state.protocol_response(Some("http://tauri.localhost"), "/frame");
        let workbench =
            state.protocol_response(Some("http://tauri.localhost"), "/workbench-preview");
        assert_eq!(program.body().as_ref(), &[1]);
        assert_eq!(workbench.body().as_ref(), &[7]);
        state.publish_workbench_preview(None);
        let cleared = state.protocol_response(Some("http://tauri.localhost"), "/workbench-preview");
        assert_eq!(cleared.status(), 204);
    }

    #[test]
    fn multiview_thumbnails_ride_a_per_scene_map() {
        let state = PreviewState::default();
        state.publish_multiview("scene-a", Some(vec![5]));
        state.publish_multiview("scene-b", Some(vec![6]));
        let a = state.protocol_response(Some("http://tauri.localhost"), "/multiview/scene-a");
        let b = state.protocol_response(Some("http://tauri.localhost"), "/multiview/scene-b");
        assert_eq!(a.body().as_ref(), &[5]);
        assert_eq!(b.body().as_ref(), &[6]);
        // An unknown scene → 204.
        let missing = state.protocol_response(Some("http://tauri.localhost"), "/multiview/scene-z");
        assert_eq!(missing.status(), 204);
        // Removing one scene leaves the others; clearing drops them all.
        state.publish_multiview("scene-a", None);
        assert_eq!(
            state
                .protocol_response(Some("http://tauri.localhost"), "/multiview/scene-a")
                .status(),
            204
        );
        state.clear_multiview();
        assert_eq!(
            state
                .protocol_response(Some("http://tauri.localhost"), "/multiview/scene-b")
                .status(),
            204
        );
    }

    #[test]
    fn percent_encoded_slot_paths_route_correctly() {
        // `convertFileSrc` sends `multiview/<id>` as `multiview%2F<id>`; the
        // handler must decode it, or every multiview tile / scene projector
        // falls through to the program frame (the HIGH bug this guards).
        let state = PreviewState::default();
        state.publish(vec![0xAA]); // the program frame
        state.publish_multiview("scene-a", Some(vec![0xBB]));
        state.publish_projector("scene:xyz", Some(vec![0xCC]));

        let mv = state.protocol_response(Some("http://tauri.localhost"), "/multiview%2Fscene-a");
        assert_eq!(
            mv.body().as_ref(),
            &[0xBB],
            "encoded multiview path must not return the program"
        );

        let pj = state.protocol_response(Some("http://tauri.localhost"), "/projector-scene%2Fxyz");
        assert_eq!(pj.body().as_ref(), &[0xCC]);
    }

    #[test]
    fn projector_frames_ride_scene_and_source_maps() {
        let state = PreviewState::default();
        state.publish_projector("scene:abc", Some(vec![1]));
        state.publish_projector("source:xyz", Some(vec![2]));
        assert_eq!(
            state
                .protocol_response(Some("http://tauri.localhost"), "/projector-scene/abc")
                .body()
                .as_ref(),
            &[1]
        );
        assert_eq!(
            state
                .protocol_response(Some("http://tauri.localhost"), "/projector-source/xyz")
                .body()
                .as_ref(),
            &[2]
        );
        // An unknown target → 204.
        assert_eq!(
            state
                .protocol_response(Some("http://tauri.localhost"), "/projector-scene/nope")
                .status(),
            204
        );
        // retain drops targets no longer open.
        let live: HashSet<String> = ["scene:abc".to_owned()].into_iter().collect();
        state.retain_projectors(&live);
        assert_eq!(
            state
                .protocol_response(Some("http://tauri.localhost"), "/projector-source/xyz")
                .status(),
            204
        );
        assert_eq!(
            state
                .protocol_response(Some("http://tauri.localhost"), "/projector-scene/abc")
                .status(),
            200
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
