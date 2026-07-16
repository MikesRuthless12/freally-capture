//! CAP-N37 soundboard playback: triggered pads decode a local clip through the
//! labeled ffmpeg component into a media-hub ring keyed by the pad id — exactly
//! the ring the audio engine drains for that pad's transient `Media` source
//! (added by the audio bridge). Choke groups stop siblings; loop repeats; a
//! natural end self-clears so the bridge drops the source.
//!
//! Captured audio still goes nowhere but the mixer, the monitor, and the
//! recording — the clip is a local file the operator chose.

use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use fcap_encode::ffmpeg::Ffmpeg;

/// One pad currently sounding, as the audio bridge needs it: the engine source
/// id (the pad id) plus its mix options.
#[derive(Debug, Clone, PartialEq)]
pub struct ActivePad {
    pub id: String,
    pub gain_db: f32,
    pub tracks: u8,
    pub auto_duck: bool,
}

struct Player {
    id: String,
    gain_db: f32,
    tracks: u8,
    auto_duck: bool,
    choke_group: u8,
    stop: Arc<AtomicBool>,
    /// Set by the decode thread when the clip ends on its own.
    done: Arc<AtomicBool>,
}

/// Tauri-managed soundboard runtime: the pads currently playing.
#[derive(Default)]
pub struct SoundboardState {
    playing: Mutex<HashMap<String, Player>>,
}

impl SoundboardState {
    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<String, Player>> {
        self.playing
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// The pads sounding right now (pruning any that ended). The bridge diffs
    /// this each tick to keep the engine's transient sources + auto-duck in sync.
    pub fn active(&self) -> Vec<ActivePad> {
        let mut playing = self.lock();
        playing.retain(|_, player| !player.done.load(Ordering::Relaxed));
        playing
            .values()
            .map(|player| ActivePad {
                id: player.id.clone(),
                gain_db: player.gain_db,
                tracks: player.tracks,
                auto_duck: player.auto_duck,
            })
            .collect()
    }

    /// Stop the sounding pads in `group` except `except` (group 0 = no choke).
    fn choke(&self, group: u8, except: &str) {
        if group == 0 {
            return;
        }
        let mut playing = self.lock();
        playing.retain(|id, player| {
            let stop = id != except && player.choke_group == group;
            if stop {
                player.stop.store(true, Ordering::Relaxed);
            }
            !stop
        });
    }

    /// Fire a pad: choke its group, (re)start its decoder, and feed its ring.
    #[allow(clippy::too_many_arguments)]
    pub fn trigger(
        &self,
        ffmpeg: &Ffmpeg,
        id: String,
        path: PathBuf,
        looping: bool,
        gain_db: f32,
        tracks: u8,
        choke_group: u8,
        auto_duck: bool,
    ) -> Result<(), String> {
        self.choke(choke_group, &id);
        // Re-trigger restarts the same pad.
        if let Some(previous) = self.lock().remove(&id) {
            previous.stop.store(true, Ordering::Relaxed);
        }

        let mut child = fcap_encode::decode::spawn_audio_decoder(ffmpeg, &path, looping, 0.0)?;
        let mut stdout = child
            .stdout
            .take()
            .ok_or("the soundboard decoder produced no output")?;

        let stop = Arc::new(AtomicBool::new(false));
        let done = Arc::new(AtomicBool::new(false));
        let ring = fcap_audio::media_hub::ring(&id);
        let thread_stop = Arc::clone(&stop);
        let thread_done = Arc::clone(&done);

        std::thread::Builder::new()
            .name("fcap-soundboard-pad".into())
            .spawn(move || {
                // f32le stereo @ 48 kHz (pcm_out) — the engine's native format,
                // no resampling. A pipe read may split a sample, so carry the
                // odd tail bytes to the next read.
                let mut raw = vec![0u8; 16 * 1024];
                let mut carry: Vec<u8> = Vec::new();
                let mut samples: Vec<f32> = Vec::new();
                loop {
                    if thread_stop.load(Ordering::Relaxed) {
                        let _ = child.kill();
                        break;
                    }
                    match stdout.read(&mut raw) {
                        Ok(0) => {
                            thread_done.store(true, Ordering::Relaxed);
                            break;
                        }
                        Ok(read) => {
                            carry.extend_from_slice(&raw[..read]);
                            let whole = carry.len() - (carry.len() % 4);
                            samples.clear();
                            for chunk in carry[..whole].chunks_exact(4) {
                                samples.push(f32::from_le_bytes([
                                    chunk[0], chunk[1], chunk[2], chunk[3],
                                ]));
                            }
                            carry.drain(..whole);
                            if !samples.is_empty() {
                                ring.push(&samples);
                            }
                        }
                        Err(_) => {
                            thread_done.store(true, Ordering::Relaxed);
                            break;
                        }
                    }
                }
                let _ = child.wait();
            })
            .map_err(|err| format!("could not start the soundboard pad thread: {err}"))?;

        self.lock().insert(
            id.clone(),
            Player {
                id,
                gain_db,
                tracks,
                auto_duck,
                choke_group,
                stop,
                done,
            },
        );
        Ok(())
    }

    /// Stop one pad.
    pub fn stop(&self, id: &str) {
        if let Some(player) = self.lock().remove(id) {
            player.stop.store(true, Ordering::Relaxed);
        }
    }

    /// Stop every sounding pad (the dock's panic/stop-all).
    pub fn stop_all(&self) {
        let mut playing = self.lock();
        for player in playing.values() {
            player.stop.store(true, Ordering::Relaxed);
        }
        playing.clear();
    }
}
