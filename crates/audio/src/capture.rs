//! One capture stream: a cpal input (or Windows WASAPI loopback) converted
//! to the engine's format — f32, stereo, 48 kHz — into a bounded ring the
//! mixer thread drains. The cpal `Stream` is `!Send`, so streams are created
//! and owned by the engine thread; only the ring is shared with the callback.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use parking_lot::Mutex;

use crate::devices::{find_input_device, find_output_device};
use crate::resample::Resampler;
use crate::{AudioError, InputSpec, SAMPLE_RATE};

/// ~0.5 s of stereo at the engine rate — the hard ring cap.
const RING_MAX_SAMPLES: usize = SAMPLE_RATE as usize;

/// The shared landing buffer between a cpal callback and the mixer thread.
///
/// Concurrency: the cpal capture callback ([`CaptureRing::push`]) and the
/// mixer thread ([`pop_into`](CaptureRing::pop_into) / [`trim_to`](CaptureRing::trim_to))
/// share one `parking_lot::Mutex`. The critical sections are tiny and bounded —
/// the mixer pops one 10 ms block (`BLOCK_SAMPLES`) per tick and only drains
/// more on the rare clock-drift trim, and `parking_lot` is uncontended-fast —
/// so the callback's lock wait stays well under the device deadline in
/// practice. A lock-free SPSC ring would remove even that theoretical wait and
/// is the natural upgrade if a lower-latency device buffer ever needs it.
pub struct CaptureRing {
    buf: Mutex<VecDeque<f32>>,
    /// Samples dropped to the cap (capture outpacing the mixer).
    dropped: AtomicU64,
    /// Set by the cpal error callback — the stream is dead.
    broken: AtomicBool,
}

impl CaptureRing {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            buf: Mutex::new(VecDeque::with_capacity(RING_MAX_SAMPLES)),
            dropped: AtomicU64::new(0),
            broken: AtomicBool::new(false),
        })
    }

    fn push(&self, samples: &[f32]) {
        let mut buf = self.buf.lock();
        for &sample in samples {
            if buf.len() >= RING_MAX_SAMPLES {
                buf.pop_front();
                self.dropped.fetch_add(1, Ordering::Relaxed);
            }
            buf.push_back(sample);
        }
    }

    /// Samples currently buffered.
    pub fn len(&self) -> usize {
        self.buf.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Pop up to `samples` into `out` (appended); returns how many came.
    pub fn pop_into(&self, out: &mut Vec<f32>, samples: usize) -> usize {
        let mut buf = self.buf.lock();
        let take = samples.min(buf.len());
        out.extend(buf.drain(..take));
        take
    }

    /// Drop all but the newest `keep` samples (drift resync); returns dropped.
    pub fn trim_to(&self, keep: usize) -> usize {
        let mut buf = self.buf.lock();
        let excess = buf.len().saturating_sub(keep);
        buf.drain(..excess);
        if excess > 0 {
            self.dropped.fetch_add(excess as u64, Ordering::Relaxed);
        }
        excess
    }

    pub fn dropped(&self) -> u64 {
        self.dropped.load(Ordering::Relaxed)
    }

    pub fn is_broken(&self) -> bool {
        self.broken.load(Ordering::Relaxed)
    }
}

/// A running capture session feeding a [`CaptureRing`].
pub struct CaptureStream {
    // Held for its Drop (stops the stream); never touched otherwise.
    _stream: cpal::Stream,
    ring: Arc<CaptureRing>,
    device_name: String,
}

impl CaptureStream {
    pub fn ring(&self) -> &Arc<CaptureRing> {
        &self.ring
    }

    pub fn device_name(&self) -> &str {
        &self.device_name
    }
}

/// Open a capture per the spec. Loopback resolves per OS: on **Windows** an
/// *output* device opened for WASAPI loopback; elsewhere a monitor/virtual
/// *input* device — with an empty id refused honestly (there is no default
/// loopback to fall back to off-Windows).
pub fn open_capture(spec: &InputSpec) -> Result<CaptureStream, AudioError> {
    match spec {
        InputSpec::Input { device_id } => {
            let device = find_input_device(device_id)?;
            let config = device
                .default_input_config()
                .map_err(|err| AudioError::Backend(err.to_string()))?;
            build(device, config)
        }
        InputSpec::Loopback { device_id } => {
            if cfg!(target_os = "windows") {
                let device = find_output_device(device_id)?;
                // WASAPI loopback: an output device opened as an input; the
                // shared mix format comes from its output config.
                let config = device
                    .default_output_config()
                    .map_err(|err| AudioError::Backend(err.to_string()))?;
                build(device, config)
            } else if device_id.is_empty() {
                Err(AudioError::NoLoopback(loopback_guidance()))
            } else {
                let device = find_input_device(device_id)?;
                let config = device
                    .default_input_config()
                    .map_err(|err| AudioError::Backend(err.to_string()))?;
                build(device, config)
            }
        }
    }
}

/// The per-OS honest "no desktop audio here" message.
pub fn loopback_guidance() -> String {
    if cfg!(target_os = "macos") {
        "macOS has no public system-audio capture — install a virtual audio device \
         (e.g. BlackHole) and pick it in the source's properties."
            .to_string()
    } else {
        "Pick the system's monitor device (e.g. \"Monitor of …\") in the source's \
         properties — PipeWire/PulseAudio expose one per output."
            .to_string()
    }
}

fn build(
    device: cpal::Device,
    supported: cpal::SupportedStreamConfig,
) -> Result<CaptureStream, AudioError> {
    let device_name = device.name().unwrap_or_default();
    let sample_format = supported.sample_format();
    let config: StreamConfig = supported.config();
    let channels = config.channels as usize;
    if channels == 0 {
        return Err(AudioError::Backend("a zero-channel device".into()));
    }
    let ring = CaptureRing::new();

    let on_error = {
        let ring = Arc::clone(&ring);
        move |err: cpal::StreamError| {
            ring.broken.store(true, Ordering::Relaxed);
            eprintln!("audio: capture stream error: {err}");
        }
    };

    // Convert → stereo → 48 kHz → ring, entirely inside the callback.
    let mut resampler = Resampler::new(config.sample_rate.0, SAMPLE_RATE);
    let mut stereo: Vec<f32> = Vec::new();
    let mut resampled: Vec<f32> = Vec::new();
    let push_ring = Arc::clone(&ring);
    let mut handle_f32 = move |data: &[f32]| {
        stereo.clear();
        for frame in data.chunks_exact(channels) {
            let left = frame[0];
            let right = if channels > 1 { frame[1] } else { frame[0] };
            stereo.push(left);
            stereo.push(right);
        }
        resampled.clear();
        resampler.process(&stereo, &mut resampled);
        push_ring.push(&resampled);
    };

    let stream = match sample_format {
        SampleFormat::F32 => device.build_input_stream(
            &config,
            move |data: &[f32], _| handle_f32(data),
            on_error,
            None,
        ),
        SampleFormat::I16 => {
            let mut float_buf: Vec<f32> = Vec::new();
            device.build_input_stream(
                &config,
                move |data: &[i16], _| {
                    float_buf.clear();
                    float_buf.extend(data.iter().map(|&s| s as f32 / 32_768.0));
                    handle_f32(&float_buf);
                },
                on_error,
                None,
            )
        }
        SampleFormat::U16 => {
            let mut float_buf: Vec<f32> = Vec::new();
            device.build_input_stream(
                &config,
                move |data: &[u16], _| {
                    float_buf.clear();
                    float_buf.extend(data.iter().map(|&s| (s as f32 - 32_768.0) / 32_768.0));
                    handle_f32(&float_buf);
                },
                on_error,
                None,
            )
        }
        SampleFormat::I32 => {
            let mut float_buf: Vec<f32> = Vec::new();
            device.build_input_stream(
                &config,
                move |data: &[i32], _| {
                    float_buf.clear();
                    float_buf.extend(data.iter().map(|&s| s as f32 / 2_147_483_648.0));
                    handle_f32(&float_buf);
                },
                on_error,
                None,
            )
        }
        other => {
            return Err(AudioError::Unsupported(format!(
                "sample format {other:?} on {device_name}"
            )))
        }
    }
    .map_err(|err| AudioError::Backend(err.to_string()))?;

    stream
        .play()
        .map_err(|err| AudioError::Backend(err.to_string()))?;

    Ok(CaptureStream {
        _stream: stream,
        ring,
        device_name,
    })
}
