//! CAP-N76 — the virtual camera's app-side wiring.
//!
//! The OS camera object (`fcap_vcam_win::MfVirtualCamera`) is a COM object that
//! must be created, fed and released on **one** thread, so it is owned by the
//! studio render loop and never touched from a command. The commands only move
//! this latch; the loop reconciles it on its next tick, exactly like the
//! recorder and stream bridges do.
//!
//! Feeding is deliberately a tee off the *existing* program readback in
//! `studio.rs` — the camera never forces a readback of its own, so switching it
//! on costs one `push_frame` per frame that was already being read.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

/// What the UI asked for, what the studio thread achieved, and why not.
#[derive(Default)]
pub struct VirtualCameraState {
    /// The UI's request. The studio loop reconciles `running` toward it.
    wanted: AtomicBool,
    /// Mirrors the studio thread — true only while the OS camera is up.
    running: AtomicBool,
    /// The last start failure, cleared on the next successful start. Held for
    /// the UI so a failed click explains itself instead of silently reverting.
    error: Mutex<Option<String>>,
}

impl VirtualCameraState {
    /// Ask for the camera. The studio loop brings it up on its next tick.
    pub fn request_on(&self) {
        self.wanted.store(true, Ordering::Relaxed);
    }

    /// Ask for the camera to stop.
    pub fn request_off(&self) {
        self.wanted.store(false, Ordering::Relaxed);
    }

    /// The studio loop's view of what it should be doing.
    pub fn wanted(&self) -> bool {
        self.wanted.load(Ordering::Relaxed)
    }

    /// Is the OS camera up right now?
    pub fn running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    /// Studio-thread only: record that the camera came up. Only the Windows
    /// `reconcile` path reaches this; elsewhere the start fails outright.
    #[cfg_attr(not(target_os = "windows"), allow(dead_code))]
    pub fn mark_running(&self) {
        self.running.store(true, Ordering::Relaxed);
        *lock(&self.error) = None;
    }

    /// Studio-thread only: record that the camera is down. Windows-only, for
    /// the same reason as `mark_running`.
    #[cfg_attr(not(target_os = "windows"), allow(dead_code))]
    pub fn mark_stopped(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    /// Studio-thread only: a start failed. Also clears the request, so the loop
    /// does not retry the same failure every tick — the operator clicks again.
    pub fn mark_failed(&self, err: String) {
        self.running.store(false, Ordering::Relaxed);
        self.wanted.store(false, Ordering::Relaxed);
        *lock(&self.error) = Some(err);
    }

    /// The last start failure, if the camera is not running because of one.
    pub fn error(&self) -> Option<String> {
        lock(&self.error).clone()
    }
}

/// Poison-tolerant lock: a panic while holding this latch must not take the
/// camera controls down with it — the worst case is a stale error string.
fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// The studio thread's owner of the OS camera.
///
/// It lives in the render loop's stack, never in shared state, because the COM
/// object underneath must be created, used and released on one thread. Off
/// Windows the whole thing is inert — the CoreMediaIO and v4l2loopback
/// components are their own milestones, and `virtual_camera_start` already
/// refuses there, so this never comes up.
#[derive(Default)]
pub struct CameraHost {
    #[cfg(target_os = "windows")]
    camera: Option<fcap_vcam_win::MfVirtualCamera>,
    /// The geometry the camera was started at; a canvas resize restarts it,
    /// because the shared frame region is sized once at start.
    #[cfg(target_os = "windows")]
    started_at: Option<(u32, u32)>,
}

impl CameraHost {
    /// Bring the camera up or down to match the latch, once per tick.
    #[allow(unused_variables)]
    pub fn reconcile(&mut self, state: &VirtualCameraState, width: u32, height: u32, fps: u32) {
        #[cfg(target_os = "windows")]
        {
            use fcap_stream::vcam::VcamOutput;

            let want = state.wanted();
            let resized = self.started_at.is_some_and(|at| at != (width, height));
            if self.camera.is_some() && (!want || resized) {
                if let Some(mut camera) = self.camera.take() {
                    let _ = camera.stop();
                }
                self.started_at = None;
                state.mark_stopped();
            }
            if want && self.camera.is_none() && width > 0 && height > 0 {
                let mut camera =
                    fcap_vcam_win::MfVirtualCamera::new(fcap_stream::vcam::VcamFeed::Program);
                match camera.start(width, height, fps) {
                    Ok(()) => {
                        self.camera = Some(camera);
                        self.started_at = Some((width, height));
                        state.mark_running();
                    }
                    Err(err) => state.mark_failed(err),
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            if state.wanted() {
                state.mark_failed("the virtual camera ships first on Windows".to_string());
            }
        }
    }

    /// Does the camera want this tick's program frame? False keeps the camera
    /// out of the readback decision entirely when it is off.
    pub fn wants_frames(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            self.camera.is_some()
        }
        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }

    /// Publish one composed RGBA frame. A failure here tears the camera down
    /// rather than logging once per frame forever.
    #[allow(unused_variables)]
    pub fn push_frame(&mut self, state: &VirtualCameraState, rgba: &[u8], width: u32, height: u32) {
        #[cfg(target_os = "windows")]
        {
            use fcap_stream::vcam::VcamOutput;

            let Some(camera) = self.camera.as_mut() else {
                return;
            };
            if let Err(err) = camera.push_frame(rgba, width, height) {
                eprintln!("studio: virtual camera push failed ({err}) — stopping it");
                if let Some(mut camera) = self.camera.take() {
                    let _ = camera.stop();
                }
                self.started_at = None;
                state.mark_failed(err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_latch_starts_off_and_follows_requests() {
        let state = VirtualCameraState::default();
        assert!(!state.wanted());
        assert!(!state.running());

        state.request_on();
        assert!(state.wanted());
        // Requesting is not running — only the studio thread may say that.
        assert!(!state.running());

        state.mark_running();
        assert!(state.running());
        assert_eq!(state.error(), None);

        state.request_off();
        state.mark_stopped();
        assert!(!state.wanted());
        assert!(!state.running());
    }

    #[test]
    fn a_failed_start_clears_the_request_so_the_loop_does_not_spin() {
        let state = VirtualCameraState::default();
        state.request_on();

        state.mark_failed("the source DLL is not registered".into());

        // The request is dropped, so the studio loop retries only when the
        // operator asks again — a failing start must not run once per frame.
        assert!(!state.wanted());
        assert!(!state.running());
        assert_eq!(
            state.error().as_deref(),
            Some("the source DLL is not registered")
        );
    }

    #[test]
    fn a_successful_start_clears_a_previous_failure() {
        let state = VirtualCameraState::default();
        state.mark_failed("no camera".into());
        assert!(state.error().is_some());

        state.request_on();
        state.mark_running();
        assert_eq!(state.error(), None);
    }
}
