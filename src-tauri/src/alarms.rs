//! Broadcast safety alarms (CAP-M10): watchdogs that run whenever output is
//! actually going somewhere — silent program audio, sustained clipping, a
//! black/frozen program picture, and a low-disk **forecast**. Detection is
//! classic CV over data the pipeline already produces (sampled luma +
//! frame delta on the existing ~33 ms readback; the mixer's master peak;
//! free space × configured bitrate) — **no ML** (charter), no new readbacks,
//! and nothing here ever blocks or mutates the pipeline. Non-modal by
//! design: transitions ride the `alarm` event into a dismissible banner and
//! the a11y announcer.

use std::path::Path;
use std::time::{Duration, Instant};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime};

/// A program whose brightest sampled pixel stays at/below this is "black".
pub const BLACK_LUMA_MAX: u8 = 16;
pub const BLACK_SUSTAIN: Duration = Duration::from_secs(5);
/// Identical samples for this long = "frozen" (a truly static program is
/// worth a nudge too — a dead capture repeats its last frame exactly).
pub const FROZEN_SUSTAIN: Duration = Duration::from_secs(10);
/// Linear master peak below this is silence (≈ −50 dBFS).
pub const SILENCE_LINEAR: f32 = 0.003_2;
pub const SILENCE_SUSTAIN: Duration = Duration::from_secs(10);
/// Linear master peak at/above this is clipping.
pub const CLIP_LINEAR: f32 = 0.999;
pub const CLIP_SUSTAIN: Duration = Duration::from_secs(1);
/// The forecast alarm raises below this many seconds of space left…
pub const LOW_DISK_FLOOR_SECS: u64 = 10 * 60;
/// …and clears above this (hysteresis, so it can't flap at the boundary).
pub const LOW_DISK_CLEAR_SECS: u64 = 12 * 60;
/// Sampled pixels per probe — a fixed cost that never touches the 60 fps
/// budget (~2k of ~2M pixels at 1080p).
const PROBE_SAMPLES: usize = 2048;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AlarmKind {
    SilentAudio,
    Clipping,
    Black,
    Frozen,
    LowDisk,
}

/// The `alarm` event payload (mirrored in `ui/src/api/types.ts`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlarmDto {
    pub kind: AlarmKind,
    pub active: bool,
    /// LowDisk only: the forecast ("~22 min left at the current bitrate").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes_left: Option<u32>,
}

pub(crate) fn emit_alarm<R: Runtime>(
    app: &AppHandle<R>,
    kind: AlarmKind,
    active: bool,
    minutes_left: Option<u32>,
) {
    let _ = app.emit(
        "alarm",
        AlarmDto {
            kind,
            active,
            minutes_left,
        },
    );
}

/// Raise only after `sustain` of continuous truth; clear immediately.
/// `update` returns the transition (`Some(raised)`) or `None`.
#[derive(Debug)]
pub struct Sustained {
    since: Option<Instant>,
    active: bool,
    sustain: Duration,
}

impl Sustained {
    pub fn new(sustain: Duration) -> Self {
        Sustained {
            since: None,
            active: false,
            sustain,
        }
    }

    pub fn update(&mut self, condition: bool, now: Instant) -> Option<bool> {
        if condition {
            let since = *self.since.get_or_insert(now);
            if !self.active && now.duration_since(since) >= self.sustain {
                self.active = true;
                return Some(true);
            }
        } else {
            self.since = None;
            if self.active {
                self.active = false;
                return Some(false);
            }
        }
        None
    }
}

/// One probe of the program readback: the brightest sampled value and
/// whether anything changed since the previous probe.
pub struct FrameProbe {
    pub luma_max: u8,
    pub changed: bool,
}

/// Sample ~[`PROBE_SAMPLES`] RGB triplets of a tightly-packed RGBA frame.
/// Exact-compare against the previous probe: the compositor renders a
/// static scene bit-identically, so any real motion differs.
pub fn probe_frame(data: &[u8], prev: &mut Vec<u8>) -> FrameProbe {
    let pixels = data.len() / 4;
    if pixels == 0 {
        prev.clear();
        return FrameProbe {
            luma_max: 0,
            changed: false,
        };
    }
    let step = (pixels / PROBE_SAMPLES).max(1);
    let mut samples = Vec::with_capacity((pixels / step + 1) * 3);
    let mut luma_max = 0u8;
    let mut index = 0;
    while index < pixels {
        let at = index * 4;
        let (r, g, b) = (data[at], data[at + 1], data[at + 2]);
        luma_max = luma_max.max(r).max(g).max(b);
        samples.extend_from_slice(&[r, g, b]);
        index += step;
    }
    let changed = samples != *prev;
    *prev = samples;
    FrameProbe { luma_max, changed }
}

/// The render loop's black/frozen watch, fed by the existing readback.
pub struct VideoWatch {
    prev: Vec<u8>,
    black: Sustained,
    frozen: Sustained,
}

impl Default for VideoWatch {
    fn default() -> Self {
        VideoWatch {
            prev: Vec::new(),
            black: Sustained::new(BLACK_SUSTAIN),
            frozen: Sustained::new(FROZEN_SUSTAIN),
        }
    }
}

impl VideoWatch {
    /// Feed one program frame. `engaged` = the picture is going somewhere
    /// (live/recording) — disengaging clears any active alarm.
    pub fn evaluate(&mut self, data: &[u8], engaged: bool, now: Instant) -> Vec<(AlarmKind, bool)> {
        let mut out = Vec::new();
        if !engaged {
            self.prev.clear();
            if self.black.update(false, now) == Some(false) {
                out.push((AlarmKind::Black, false));
            }
            if self.frozen.update(false, now) == Some(false) {
                out.push((AlarmKind::Frozen, false));
            }
            return out;
        }
        let probe = probe_frame(data, &mut self.prev);
        if let Some(active) = self.black.update(probe.luma_max <= BLACK_LUMA_MAX, now) {
            out.push((AlarmKind::Black, active));
        }
        if let Some(active) = self.frozen.update(!probe.changed, now) {
            out.push((AlarmKind::Frozen, active));
        }
        out
    }
}

/// The audio bridge's silence/clipping watch over the master mix peak.
pub struct AudioWatch {
    silent: Sustained,
    clip: Sustained,
}

impl Default for AudioWatch {
    fn default() -> Self {
        AudioWatch {
            silent: Sustained::new(SILENCE_SUSTAIN),
            clip: Sustained::new(CLIP_SUSTAIN),
        }
    }
}

impl AudioWatch {
    pub fn evaluate(&mut self, peak: f32, engaged: bool, now: Instant) -> Vec<(AlarmKind, bool)> {
        let mut out = Vec::new();
        let silent = engaged && peak < SILENCE_LINEAR;
        let clipping = engaged && peak >= CLIP_LINEAR;
        if let Some(active) = self.silent.update(silent, now) {
            out.push((AlarmKind::SilentAudio, active));
        }
        if let Some(active) = self.clip.update(clipping, now) {
            out.push((AlarmKind::Clipping, active));
        }
        out
    }
}

/// Seconds of recording left: free space ÷ write rate. `None` = no rate.
pub fn forecast_secs(free_bytes: u64, bytes_per_sec: u64) -> Option<u64> {
    (bytes_per_sec > 0).then(|| free_bytes / bytes_per_sec)
}

/// The recording's write rate from its settings — an honest CBR-ish
/// estimate ("~22 min", never a promise).
pub fn recording_bytes_per_sec(video_kbps: u32, audio_kbps: u32, tracks: u32) -> u64 {
    (video_kbps as u64 + audio_kbps as u64 * tracks as u64) * 1000 / 8
}

/// The write rate for the SESSION's container. Wire containers ride the
/// configured encoder bitrate; the owned lossless .frec writes
/// FLZ-compressed raw canvas frames — orders of magnitude faster, so the
/// wire formula would forecast hours where minutes remain. Estimated at
/// HALF the raw RGBA rate (screen content typically compresses 2–5×;
/// overestimating the rate errs on the early-warning side) plus the PCM
/// audio lanes.
pub fn recording_write_rate(
    container: fcap_encode::Container,
    canvas: (u32, u32),
    fps: u32,
    video_kbps: u32,
    audio_kbps: u32,
    tracks: u32,
) -> u64 {
    match container {
        fcap_encode::Container::Frec => {
            let raw = canvas.0 as u64 * canvas.1 as u64 * 4 * fps.max(1) as u64;
            let audio = 48_000u64 * 2 * 4 * tracks as u64;
            raw / 2 + audio
        }
        _ => recording_bytes_per_sec(video_kbps, audio_kbps, tracks),
    }
}

/// Low-disk state with hysteresis; re-announces while active when the
/// forecast minute changes (the banner counts down honestly).
#[derive(Debug, Default)]
pub struct DiskWatch {
    active: bool,
    last_minutes: Option<u32>,
}

impl DiskWatch {
    /// `secs_left = None` (unknown rate / not recording) clears.
    pub fn evaluate(&mut self, secs_left: Option<u64>) -> Option<AlarmDto> {
        match secs_left {
            Some(secs)
                if secs < LOW_DISK_FLOOR_SECS || (self.active && secs < LOW_DISK_CLEAR_SECS) =>
            {
                let minutes = (secs / 60) as u32;
                let fresh = !self.active || self.last_minutes != Some(minutes);
                self.active = true;
                self.last_minutes = Some(minutes);
                fresh.then_some(AlarmDto {
                    kind: AlarmKind::LowDisk,
                    active: true,
                    minutes_left: Some(minutes),
                })
            }
            _ => {
                let was = self.active;
                self.active = false;
                self.last_minutes = None;
                was.then_some(AlarmDto {
                    kind: AlarmKind::LowDisk,
                    active: false,
                    minutes_left: None,
                })
            }
        }
    }
}

/// The go-live pre-flight's disk item (CAP-M09): whole minutes of recording
/// left at the configured bitrate, `None` when free space can't be read.
#[tauri::command]
pub fn preflight_disk<R: Runtime>(app: AppHandle<R>) -> Option<u32> {
    preflight_disk_minutes(&app)
}

/// The command's body, callable from the backend hold gate in `stream::start`.
pub fn preflight_disk_minutes<R: Runtime>(app: &AppHandle<R>) -> Option<u32> {
    let settings = app
        .state::<crate::settings::SettingsStore>()
        .get()
        .recording;
    let canvas = app
        .state::<crate::studio::StudioState>()
        .with_collection(|collection| (collection.canvas_width, collection.canvas_height));
    let rate = recording_write_rate(
        settings.container,
        canvas,
        settings.fps,
        settings.rate_control.bitrate_kbps,
        settings.audio_bitrate_kbps,
        settings.tracks_mask.count_ones(),
    );
    let free = free_space_for(&crate::recording::recordings_folder(&settings))?;
    forecast_secs(free, rate).map(|secs| (secs / 60).min(u32::MAX as u64) as u32)
}

/// Free bytes on the volume holding `path` (best-effort, sysinfo `Disks`):
/// the disk with the longest mount-point prefix wins.
pub fn free_space_for(path: &Path) -> Option<u64> {
    let disks = sysinfo::Disks::new_with_refreshed_list();
    disks
        .iter()
        .filter(|disk| path.starts_with(disk.mount_point()))
        .max_by_key(|disk| disk.mount_point().as_os_str().len())
        .map(|disk| disk.available_space())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(luma: u8, pixels: usize) -> Vec<u8> {
        let mut data = vec![0u8; pixels * 4];
        for pixel in data.chunks_exact_mut(4) {
            pixel[0] = luma;
            pixel[1] = luma;
            pixel[2] = luma;
            pixel[3] = 255;
        }
        data
    }

    #[test]
    fn black_raises_after_sustain_and_clears_on_light() {
        let mut watch = VideoWatch::default();
        let base = Instant::now();
        let dark = frame(4, 4096);
        assert!(watch.evaluate(&dark, true, base).is_empty());
        // Still dark 6 s later → the black alarm raises (frozen needs 10 s).
        let at = base + Duration::from_secs(6);
        assert_eq!(
            watch.evaluate(&dark, true, at),
            vec![(AlarmKind::Black, true)]
        );
        // A bright frame clears it immediately.
        let bright = frame(200, 4096);
        assert_eq!(
            watch.evaluate(&bright, true, at + Duration::from_secs(1)),
            vec![(AlarmKind::Black, false)]
        );
    }

    #[test]
    fn frozen_raises_on_identical_frames_and_clears_on_motion() {
        let mut watch = VideoWatch::default();
        let base = Instant::now();
        let still = frame(120, 4096);
        // First frame seeds the probe; the second (identical) starts the
        // freeze clock; 11 s of stillness later the alarm raises.
        assert!(watch.evaluate(&still, true, base).is_empty());
        assert!(watch
            .evaluate(&still, true, base + Duration::from_secs(1))
            .is_empty());
        assert_eq!(
            watch.evaluate(&still, true, base + Duration::from_secs(12)),
            vec![(AlarmKind::Frozen, true)]
        );
        let moving = frame(121, 4096);
        assert_eq!(
            watch.evaluate(&moving, true, base + Duration::from_secs(13)),
            vec![(AlarmKind::Frozen, false)]
        );
    }

    #[test]
    fn disengaging_clears_active_video_alarms() {
        let mut watch = VideoWatch::default();
        let base = Instant::now();
        let dark = frame(0, 4096);
        watch.evaluate(&dark, true, base);
        watch.evaluate(&dark, true, base + Duration::from_secs(6));
        let cleared = watch.evaluate(&dark, false, base + Duration::from_secs(7));
        assert!(cleared.contains(&(AlarmKind::Black, false)));
    }

    #[test]
    fn silence_and_clipping_watch_the_master_peak() {
        let mut watch = AudioWatch::default();
        let base = Instant::now();
        assert!(watch.evaluate(0.0, true, base).is_empty());
        assert_eq!(
            watch.evaluate(0.0, true, base + Duration::from_secs(11)),
            vec![(AlarmKind::SilentAudio, true)]
        );
        // Sound returns → clears; hard clipping raises after its second.
        let at = base + Duration::from_secs(12);
        assert_eq!(
            watch.evaluate(1.0, true, at),
            vec![(AlarmKind::SilentAudio, false)]
        );
        assert_eq!(
            watch.evaluate(1.0, true, at + Duration::from_secs(2)),
            vec![(AlarmKind::Clipping, true)]
        );
    }

    #[test]
    fn forecast_math_is_free_over_rate() {
        // 8 Mbps video + 192 kbps audio ≈ 1.024 MB/s.
        let rate = recording_bytes_per_sec(8_000, 192, 1);
        assert_eq!(rate, 1_024_000);
        // ~22 minutes of space at that rate.
        let free = rate * 22 * 60;
        assert_eq!(forecast_secs(free, rate), Some(22 * 60));
        assert_eq!(forecast_secs(free, 0), None);
    }

    #[test]
    fn the_lossless_container_forecasts_at_its_real_rate_not_the_wire_bitrate() {
        // 1080p60 .frec: half of raw RGBA ≈ 249 MB/s — the wire formula's
        // ~1 MB/s would forecast hours where minutes remain.
        let frec = recording_write_rate(
            fcap_encode::Container::Frec,
            (1920, 1080),
            60,
            8_000,
            192,
            1,
        );
        assert!(
            frec > 200_000_000,
            "frec rate {frec} must dwarf the wire rate"
        );
        // 50 GB free is ~3–4 minutes at that rate, NOT ~800.
        let secs = forecast_secs(50_000_000_000, frec).unwrap();
        assert!(
            secs < LOW_DISK_FLOOR_SECS,
            "{secs}s must trip the alarm floor"
        );
        // Wire containers keep the bitrate formula.
        let wire =
            recording_write_rate(fcap_encode::Container::Mkv, (1920, 1080), 60, 8_000, 192, 1);
        assert_eq!(wire, recording_bytes_per_sec(8_000, 192, 1));
    }

    #[test]
    fn low_disk_raises_below_the_floor_with_hysteresis() {
        let mut watch = DiskWatch::default();
        // Plenty of room: nothing.
        assert!(watch.evaluate(Some(3600)).is_none());
        // Under 10 min: raises with the minute count.
        let raised = watch.evaluate(Some(9 * 60)).expect("raises");
        assert!(raised.active);
        assert_eq!(raised.minutes_left, Some(9));
        // Same minute again: no re-announce; a new minute re-announces.
        assert!(watch.evaluate(Some(9 * 60 + 20)).is_none());
        assert_eq!(
            watch
                .evaluate(Some(8 * 60))
                .and_then(|dto| dto.minutes_left),
            Some(8)
        );
        // 11 min free is inside the hysteresis band — still active, and 13
        // min clears it.
        assert!(watch.evaluate(Some(11 * 60)).is_some_and(|dto| dto.active));
        let cleared = watch.evaluate(Some(13 * 60)).expect("clears");
        assert!(!cleared.active);
        // Not recording → stays quiet.
        assert!(watch.evaluate(None).is_none());
    }
}
