//! The monitor output: plays the monitor mix on a chosen output device.
//! The engine pushes 48 kHz stereo blocks; they are resampled to the device
//! rate engine-side and drained by the cpal output callback (underruns play
//! silence — monitoring never stalls the mixer).

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use parking_lot::Mutex;

use crate::devices::find_output_device;
use crate::resample::Resampler;
use crate::{AudioError, SAMPLE_RATE};

/// ~200 ms of stereo at 48 kHz — bounded so a stalled device never grows RAM.
const QUEUE_MAX_BLOCKS_MS: usize = 200;

struct MonitorShared {
    /// Device-rate stereo samples waiting for the output callback.
    queue: Mutex<VecDeque<f32>>,
    broken: AtomicBool,
}

pub struct MonitorStream {
    _stream: cpal::Stream,
    shared: Arc<MonitorShared>,
    resampler: Resampler,
    scratch: Vec<f32>,
    device_rate: u32,
    device_name: String,
}

impl MonitorStream {
    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    pub fn is_broken(&self) -> bool {
        self.shared.broken.load(Ordering::Relaxed)
    }

    /// Queue one 48 kHz stereo block for playback.
    pub fn push(&mut self, block: &[f32]) {
        self.scratch.clear();
        self.resampler.process(block, &mut self.scratch);
        let cap = self.device_rate as usize * 2 * QUEUE_MAX_BLOCKS_MS / 1_000;
        let mut queue = self.shared.queue.lock();
        for &sample in &self.scratch {
            if queue.len() >= cap {
                queue.pop_front();
            }
            queue.push_back(sample);
        }
    }
}

/// Open the monitor on an output device ("" = the OS default).
pub fn open_monitor(device_id: &str) -> Result<MonitorStream, AudioError> {
    let device = find_output_device(device_id)?;
    let device_name = device.name().unwrap_or_default();
    let supported = device
        .default_output_config()
        .map_err(|err| AudioError::Backend(err.to_string()))?;
    let sample_format = supported.sample_format();
    let config: StreamConfig = supported.config();
    let channels = config.channels as usize;
    if channels == 0 {
        return Err(AudioError::Backend("a zero-channel device".into()));
    }

    let shared = Arc::new(MonitorShared {
        queue: Mutex::new(VecDeque::new()),
        broken: AtomicBool::new(false),
    });

    let on_error = {
        let shared = Arc::clone(&shared);
        move |err: cpal::StreamError| {
            shared.broken.store(true, Ordering::Relaxed);
            eprintln!("audio: monitor stream error: {err}");
        }
    };

    // The callback pops stereo frames and maps them onto the device's
    // channel count (mono devices average L+R; extra channels stay silent).
    let cb_shared = Arc::clone(&shared);
    let fill_f32 = move |data: &mut [f32]| {
        let mut queue = cb_shared.queue.lock();
        for frame in data.chunks_exact_mut(channels) {
            let left = queue.pop_front().unwrap_or(0.0);
            let right = queue.pop_front().unwrap_or(0.0);
            if channels == 1 {
                frame[0] = (left + right) * 0.5;
            } else {
                frame[0] = left;
                frame[1] = right;
                for extra in frame.iter_mut().skip(2) {
                    *extra = 0.0;
                }
            }
        }
    };

    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(
            &config,
            move |data: &mut [f32], _| fill_f32(data),
            on_error,
            None,
        ),
        SampleFormat::I16 => {
            let mut float_buf: Vec<f32> = Vec::new();
            device.build_output_stream(
                &config,
                move |data: &mut [i16], _| {
                    float_buf.clear();
                    float_buf.resize(data.len(), 0.0);
                    fill_f32(&mut float_buf);
                    for (out, &value) in data.iter_mut().zip(&float_buf) {
                        *out = (value.clamp(-1.0, 1.0) * 32_767.0) as i16;
                    }
                },
                on_error,
                None,
            )
        }
        SampleFormat::U16 => {
            let mut float_buf: Vec<f32> = Vec::new();
            device.build_output_stream(
                &config,
                move |data: &mut [u16], _| {
                    float_buf.clear();
                    float_buf.resize(data.len(), 0.0);
                    fill_f32(&mut float_buf);
                    for (out, &value) in data.iter_mut().zip(&float_buf) {
                        *out = ((value.clamp(-1.0, 1.0) * 0.5 + 0.5) * 65_535.0) as u16;
                    }
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

    Ok(MonitorStream {
        _stream: stream,
        shared,
        resampler: Resampler::new(SAMPLE_RATE, config.sample_rate.0),
        scratch: Vec::new(),
        device_rate: config.sample_rate.0,
        device_name,
    })
}
