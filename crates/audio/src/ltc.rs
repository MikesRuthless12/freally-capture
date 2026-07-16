//! CAP-N47: owned SMPTE 12M **linear timecode** (LTC) — classic audio
//! timecode, fully offline, for syncing external recorders/cameras in post.
//!
//! An LTC frame is 80 bits: BCD time fields + user bits + the sync word
//! `0011 1111 1111 1101` (bits 64–79), transmitted with **biphase-mark**
//! coding: every bit cell starts with a level transition; a `1` adds a
//! second transition mid-cell. The standard is decades old and patent-free —
//! the same expired-technique posture as the rest of the owned engine.
//!
//! The generator renders frames as a ±0.4 square wave at the mixer's sample
//! rate; the decoder measures the distance between zero crossings, classifies
//! half-cells vs full cells against an adaptive period estimate, and hunts
//! the sync word. Round-trip accuracy is ±1 frame by construction (the DoD's
//! bar), verified in tests at 24/25/30 fps.

/// A decoded (or to-be-encoded) LTC time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LtcTime {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub frames: u8,
}

impl LtcTime {
    /// `HH:MM:SS:FF` — the broadcast display form.
    pub fn display(&self) -> String {
        format!(
            "{:02}:{:02}:{:02}:{:02}",
            self.hours, self.minutes, self.seconds, self.frames
        )
    }
}

/// The sync word occupying bits 64–79, LSB-first as transmitted.
const SYNC_WORD: [bool; 16] = [
    false, false, true, true, true, true, true, true, true, true, true, true, true, true, false,
    true,
];

/// Build the 80 transmitted bits for one frame (LSB-first fields, per 12M).
/// User bits are zero; the drop-frame and color-frame flags stay clear (the
/// generator runs real-time / non-drop — said in the UI).
pub fn encode_frame_bits(time: LtcTime) -> [bool; 80] {
    let mut bits = [false; 80];
    let mut put = |at: usize, width: usize, value: u8| {
        for bit in 0..width {
            bits[at + bit] = (value >> bit) & 1 == 1;
        }
    };
    put(0, 4, time.frames % 10); // frame units
    put(8, 2, time.frames / 10); // frame tens
    put(16, 4, time.seconds % 10); // second units
    put(24, 3, time.seconds / 10); // second tens
    put(32, 4, time.minutes % 10); // minute units
    put(40, 3, time.minutes / 10); // minute tens
    put(48, 4, time.hours % 10); // hour units
    put(56, 2, time.hours / 10); // hour tens
    bits[64..80].copy_from_slice(&SYNC_WORD);
    bits
}

/// Decode the time fields out of 80 frame bits (the caller aligned them so
/// bits 64–79 are the sync word).
fn decode_frame_bits(bits: &[bool]) -> LtcTime {
    let get = |at: usize, width: usize| -> u8 {
        let mut value = 0u8;
        for bit in 0..width {
            if bits[at + bit] {
                value |= 1 << bit;
            }
        }
        value
    };
    LtcTime {
        frames: get(0, 4) + 10 * get(8, 2),
        seconds: get(16, 4) + 10 * get(24, 3),
        minutes: get(32, 4) + 10 * get(40, 3),
        hours: get(48, 4) + 10 * get(56, 2),
    }
}

/// Streaming LTC generator: renders successive frames' biphase-mark square
/// wave into 10 ms mixer blocks, advancing the timecode frame by frame.
pub struct LtcGenerator {
    sample_rate: u32,
    fps: u32,
    time: LtcTime,
    /// The current frame's bits and the render position inside it.
    bits: [bool; 80],
    bit_index: usize,
    /// Sample position inside the current bit cell (fractional cells carry).
    cell_pos: f64,
    /// The next rendered sample begins a cell — biphase-mark transitions the
    /// level there. Set at construction and on every cell rollover.
    cell_start: bool,
    /// The output level (biphase-mark flips it on every cell start + on 1s).
    level: f32,
}

/// The generator's output amplitude — hot enough to decode anywhere, far
/// from clipping.
const LTC_LEVEL: f32 = 0.4;

impl LtcGenerator {
    /// Start at `time`, `fps` ∈ {24, 25, 30}.
    pub fn new(sample_rate: u32, fps: u32, time: LtcTime) -> Self {
        Self {
            sample_rate: sample_rate.max(8_000),
            fps: fps.clamp(24, 30),
            time,
            bits: encode_frame_bits(time),
            bit_index: 0,
            cell_pos: 0.0,
            cell_start: true,
            level: LTC_LEVEL,
        }
    }

    /// Advance the timecode one frame (wrapping a 24 h day).
    fn advance(&mut self) {
        let mut t = self.time;
        t.frames += 1;
        if u32::from(t.frames) >= self.fps {
            t.frames = 0;
            t.seconds += 1;
            if t.seconds >= 60 {
                t.seconds = 0;
                t.minutes += 1;
                if t.minutes >= 60 {
                    t.minutes = 0;
                    t.hours = (t.hours + 1) % 24;
                }
            }
        }
        self.time = t;
        self.bits = encode_frame_bits(t);
        self.bit_index = 0;
    }

    /// The timecode currently being transmitted.
    pub fn time(&self) -> LtcTime {
        self.time
    }

    /// Render `frames` stereo f32 frames (interleaved) of LTC into `out`
    /// (both channels carry the same signal — a mono timecode on a stereo
    /// bus, standard practice).
    pub fn render(&mut self, out: &mut [f32]) {
        let samples_per_cell = f64::from(self.sample_rate) / (f64::from(self.fps) * 80.0);
        let half = samples_per_cell / 2.0;
        for frame in out.chunks_exact_mut(2) {
            if self.cell_start {
                // Biphase-mark: every cell begins with a level transition.
                self.level = -self.level;
                self.cell_start = false;
            }
            let mid_crossed = self.cell_pos < half && self.cell_pos + 1.0 >= half;
            if mid_crossed && self.bits[self.bit_index] {
                // A `1` adds a second transition mid-cell.
                self.level = -self.level;
            }
            frame[0] = self.level;
            frame[1] = self.level;
            self.cell_pos += 1.0;
            if self.cell_pos >= samples_per_cell {
                // Fractional carry keeps the long-run cadence exact.
                self.cell_pos -= samples_per_cell;
                self.bit_index += 1;
                if self.bit_index >= 80 {
                    self.advance();
                }
                self.cell_start = true;
            }
        }
    }
}

/// Streaming LTC decoder: feed it sample blocks from any audio input; it
/// yields the most recently decoded frame time.
pub struct LtcDecoder {
    /// Samples since the last zero crossing.
    run: f64,
    last_sign: bool,
    /// Adaptive half-cell period estimate (samples).
    half_period: f64,
    /// Pending short-interval flag (two shorts = one `1` bit).
    pending_half: bool,
    /// The rolling bit window (newest last).
    window: Vec<bool>,
    /// The last successfully decoded time.
    latest: Option<LtcTime>,
}

impl LtcDecoder {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            run: 0.0,
            last_sign: false,
            // Seed for 30 fps @ 48 kHz (~20 samples/cell); adapts in a few
            // transitions either way.
            half_period: f64::from(sample_rate.max(8_000)) / (30.0 * 80.0) / 2.0,
            pending_half: false,
            window: Vec::with_capacity(160),
            latest: None,
        }
    }

    /// The most recently decoded timecode, if any signal has locked yet.
    pub fn latest(&self) -> Option<LtcTime> {
        self.latest
    }

    /// Feed interleaved stereo samples (the left channel is read).
    pub fn feed(&mut self, samples: &[f32]) {
        for frame in samples.chunks_exact(2) {
            let sign = frame[0] >= 0.0;
            self.run += 1.0;
            if sign == self.last_sign {
                continue;
            }
            self.last_sign = sign;
            let interval = self.run;
            self.run = 0.0;
            // Ignore sub-audio glitches (shorter than a third of a half).
            if interval < self.half_period / 3.0 {
                continue;
            }
            let is_half = interval < self.half_period * 1.5;
            // Adapt the period estimate toward what we see.
            let seen_half = if is_half { interval } else { interval / 2.0 };
            self.half_period = self.half_period * 0.95 + seen_half * 0.05;
            if is_half {
                if self.pending_half {
                    self.pending_half = false;
                    self.push_bit(true);
                } else {
                    self.pending_half = true;
                }
            } else {
                // A stray half without its partner resyncs as a zero.
                self.pending_half = false;
                self.push_bit(false);
            }
        }
    }

    fn push_bit(&mut self, bit: bool) {
        self.window.push(bit);
        if self.window.len() > 160 {
            self.window.drain(..80);
        }
        let len = self.window.len();
        if len < 80 {
            return;
        }
        // The newest 16 bits matching the sync word close an 80-bit frame.
        if self.window[len - 16..] == SYNC_WORD {
            let frame = &self.window[len - 80..];
            let time = decode_frame_bits(frame);
            // Sanity-bound the fields — noise that happens to contain the
            // sync pattern must not publish a nonsense time.
            if time.hours < 24 && time.minutes < 60 && time.seconds < 60 && time.frames < 30 + 1 {
                self.latest = Some(time);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip(fps: u32, start: LtcTime, seconds: f64) -> (LtcTime, LtcTime) {
        let sample_rate = 48_000u32;
        let mut generator = LtcGenerator::new(sample_rate, fps, start);
        let mut decoder = LtcDecoder::new(sample_rate);
        let total = (f64::from(sample_rate) * seconds) as usize;
        let mut rendered = 0usize;
        while rendered < total {
            let block = 480.min(total - rendered);
            let mut samples = vec![0.0f32; block * 2];
            generator.render(&mut samples);
            decoder.feed(&samples);
            rendered += block;
        }
        (
            generator.time(),
            decoder.latest().expect("the decoder locks within seconds"),
        )
    }

    /// The DoD's bar: generate → read lands within ±1 frame of the last
    /// FULLY TRANSMITTED frame, at every broadcast rate. (The sync word
    /// closes a frame at its end, so the frame still mid-transmission is by
    /// definition not decodable yet — the generator's clock reads one ahead
    /// of the wire.)
    #[test]
    fn generate_read_round_trips_within_one_frame() {
        for fps in [24u32, 25, 30] {
            let start = LtcTime {
                hours: 12,
                minutes: 34,
                seconds: 56,
                frames: 0,
            };
            let (sent, got) = round_trip(fps, start, 2.0);
            let frame_of = |t: LtcTime| -> i64 {
                (i64::from(t.hours) * 3600 + i64::from(t.minutes) * 60 + i64::from(t.seconds))
                    * i64::from(fps)
                    + i64::from(t.frames)
            };
            let last_transmitted = frame_of(sent) - 1;
            let delta = (last_transmitted - frame_of(got)).abs();
            assert!(
                delta <= 1,
                "{fps} fps: last transmitted {} but decoded {} (Δ {delta} frames)",
                last_transmitted,
                got.display()
            );
        }
    }

    /// Field coding: known values survive the bit round-trip exactly.
    #[test]
    fn frame_bits_encode_and_decode_exactly() {
        for time in [
            LtcTime {
                hours: 0,
                minutes: 0,
                seconds: 0,
                frames: 0,
            },
            LtcTime {
                hours: 23,
                minutes: 59,
                seconds: 59,
                frames: 29,
            },
            LtcTime {
                hours: 9,
                minutes: 41,
                seconds: 7,
                frames: 13,
            },
        ] {
            let bits = encode_frame_bits(time);
            assert_eq!(&bits[64..80], &SYNC_WORD, "sync word in place");
            assert_eq!(decode_frame_bits(&bits), time);
        }
    }

    /// The timecode rolls over frame → second → minute → hour → day.
    #[test]
    fn the_generator_clock_rolls_over_cleanly() {
        let mut generator = LtcGenerator::new(
            48_000,
            25,
            LtcTime {
                hours: 23,
                minutes: 59,
                seconds: 59,
                frames: 24,
            },
        );
        generator.advance();
        assert_eq!(
            generator.time(),
            LtcTime {
                hours: 0,
                minutes: 0,
                seconds: 0,
                frames: 0
            }
        );
    }

    /// Silence (or noise without the sync pattern) never publishes a time.
    #[test]
    fn silence_never_decodes() {
        let mut decoder = LtcDecoder::new(48_000);
        decoder.feed(&vec![0.0f32; 48_000]);
        assert_eq!(decoder.latest(), None);
    }
}
