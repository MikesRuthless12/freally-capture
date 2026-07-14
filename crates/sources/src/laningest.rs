//! CAP-N11 — the **LAN ingest source**: a built-in SRT/RTMP *listener* so a
//! phone or second PC on the same network can feed a scene (any free mobile
//! SRT/RTMP camera app, another encoder, a drone controller).
//!
//! **LAN honesty:** nothing listens until the source is added, the listener
//! stops when it is removed, and this module **never dials out** — the bind
//! is local, the connect URL the UI shows is built by the caller from its
//! local-IP probe, and no traffic leaves the machine unless a sender on the
//! network sends first. SRT can encrypt with a passphrase (preferred); RTMP
//! has no authentication in the protocol — the pickers say both plainly.
//!
//! **Why one process:** ffmpeg IS the listener, and two processes cannot
//! bind one listen port — so a single labeled-ffmpeg child accepts the
//! connection, decodes, scales+pads every stream to one fixed canvas (the
//! playlist's fit+pad precedent; no probe roundtrip burns the sender's
//! connection), and interleaves rawvideo RGBA + f32 audio as a streamed AVI
//! on stdout. The owned demuxer below walks the RIFF chunks: `00dc`/`00db`
//! frames go to the compositor, `01wb` samples go to the media-hub ring
//! (the source's own mixer strip, like Media).
//!
//! **Lifecycle:** no sender yet → the pipe yields nothing, so the session
//! shows a "waiting" face with the connect URL; the sender disconnecting
//! makes ffmpeg exit → face again + a fresh listener (auto-relisten) until
//! the source is removed. A listener that keeps dying *without ever
//! receiving* (a busy port) errors honestly instead of spinning forever.

use std::collections::HashMap;
use std::io::Read;
use std::process::Child;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame, FrameSender, PixelFormat};
use fcap_encode::decode;

use crate::media::read_exact_or_end;
use crate::text::{render_text, TextAlign, TextStyle};

/// The normalized pipe geometry: every sender scales+pads into this canvas
/// so ONE listener serves the session and the pipe never changes frame size
/// (the waiting face is the same size for the same reason).
pub const CANVAS_W: u32 = 1920;
pub const CANVAS_H: u32 = 1080;

/// The waiting face's background — an opaque dark placard so the URL stays
/// legible over anything.
const FACE_BG: [u8; 4] = [22, 24, 29, 255];

/// How long a listener must survive to count as healthy; three instant
/// deaths without a single frame = an honest error (usually a busy port).
const INSTANT_EXIT: Duration = Duration::from_secs(2);
const MAX_STRIKES: u32 = 3;

/// Which protocol the listener speaks (mirrors `fcap_scene::IngestProtocol`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IngestProtocol {
    Srt,
    Rtmp,
}

impl IngestProtocol {
    fn label(self) -> &'static str {
        match self {
            IngestProtocol::Srt => "SRT",
            IngestProtocol::Rtmp => "RTMP",
        }
    }
}

#[derive(Debug, Clone)]
pub struct LanIngestConfig {
    pub protocol: IngestProtocol,
    pub port: u16,
    /// SRT only; empty = an open, unencrypted listener.
    pub passphrase: String,
    /// The reachable URL the waiting face shows. Built by the CALLER from
    /// its local-IP probe — this module never asks the network anything.
    pub connect_url: String,
}

// ---------------------------------------------------------------------------
// URL / argument builders (pure — the tests pin them)
// ---------------------------------------------------------------------------

/// The ffmpeg LISTEN input: the URL plus the extra args that go before
/// `-i`. The SRT passphrase travels as the `-passphrase` option — never
/// URL-encoded into the bind URL, so no escaping questions on this side.
pub fn listener_input(
    protocol: IngestProtocol,
    port: u16,
    passphrase: &str,
) -> (String, Vec<String>) {
    match protocol {
        IngestProtocol::Srt => {
            let url = format!("srt://0.0.0.0:{port}?mode=listener");
            let mut args = Vec::new();
            if !passphrase.is_empty() {
                args.push("-passphrase".to_string());
                args.push(passphrase.to_string());
            }
            (url, args)
        }
        IngestProtocol::Rtmp => (
            format!("rtmp://0.0.0.0:{port}/live"),
            vec!["-listen".to_string(), "1".to_string()],
        ),
    }
}

/// What a sender on the LAN dials — shown next to the QR in the pickers.
/// The SRT passphrase rides the query percent-encoded (senders' URL parsers
/// decode it; verified against an ffmpeg caller).
pub fn connect_url(protocol: IngestProtocol, host: &str, port: u16, passphrase: &str) -> String {
    match protocol {
        IngestProtocol::Srt if !passphrase.is_empty() => {
            format!(
                "srt://{host}:{port}?passphrase={}",
                encode_query(passphrase)
            )
        }
        IngestProtocol::Srt => format!("srt://{host}:{port}"),
        IngestProtocol::Rtmp => format!("rtmp://{host}:{port}/live"),
    }
}

/// Percent-encode a URL query value (RFC 3986: unreserved stay literal).
fn encode_query(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                encoded.push(byte as char)
            }
            other => encoded.push_str(&format!("%{other:02X}")),
        }
    }
    encoded
}

/// Ports below 1024 need elevation and collide with system services.
pub fn validate_port(port: u16) -> Result<(), String> {
    if port < 1024 {
        return Err(format!("port {port} is reserved — use 1024–65535"));
    }
    Ok(())
}

/// SRT passphrases are 10–79 characters **by spec** — libsrt refuses others
/// with a cryptic handshake error, so say it readably up front. Empty is
/// allowed (an open listener; the UI warns).
pub fn validate_passphrase(passphrase: &str) -> Result<(), String> {
    let len = passphrase.chars().count();
    if len != 0 && !(10..=79).contains(&len) {
        return Err("SRT passphrases must be 10–79 characters (or empty for open)".into());
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// The "sender connected" registry (the studio's runtime-state truth)
// ---------------------------------------------------------------------------

/// Keyed by source id. A frame arriving proves nothing here — the waiting
/// face is a frame too — so the session publishes whether a sender is
/// actually feeding it, and the studio mirrors that into the runtime state.
fn registry() -> &'static Mutex<HashMap<String, bool>> {
    static REG: OnceLock<Mutex<HashMap<String, bool>>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Is a sender currently feeding this source? (`false` = waiting face.)
pub fn receiving(id: &str) -> bool {
    registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(id)
        .copied()
        .unwrap_or(false)
}

fn set_receiving(id: &str, on: bool) {
    registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .insert(id.to_string(), on);
}

fn clear_receiving(id: &str) {
    registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .remove(id);
}

// ---------------------------------------------------------------------------
// The session
// ---------------------------------------------------------------------------

/// Start the LAN ingest listener session. `hub_id` keys the mixer-side
/// audio ring (the source id, like Media).
pub fn start_lan_ingest(
    hub_id: &str,
    config: LanIngestConfig,
) -> Result<CaptureSession, CaptureError> {
    let Some(ffmpeg) = fcap_encode::ffmpeg::installed() else {
        return Err(CaptureError::Backend(
            "LAN ingest needs the ffmpeg component — install it from Components".into(),
        ));
    };
    validate_port(config.port).map_err(CaptureError::Backend)?;
    if config.protocol == IngestProtocol::Srt {
        validate_passphrase(&config.passphrase).map_err(CaptureError::Backend)?;
    }

    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let hub_id = hub_id.to_string();
    let join = std::thread::Builder::new()
        .name("fcap-lan-ingest".into())
        .spawn(move || run_listener(ffmpeg, config, hub_id, sender, thread_stop))
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

/// The listen loop: waiting face → one listener stretch → face again →
/// re-listen, until the source is removed (or the port proves unusable).
fn run_listener(
    ffmpeg: fcap_encode::ffmpeg::Ffmpeg,
    config: LanIngestConfig,
    hub_id: String,
    sender: FrameSender,
    stop: Arc<AtomicBool>,
) {
    let face = waiting_face(config.protocol.label(), &config.connect_url);
    let (url, extra_args) = listener_input(config.protocol, config.port, &config.passphrase);
    let mut strikes = 0u32;
    'listen: loop {
        if stop.load(Ordering::Relaxed) || !sender.is_open() {
            break;
        }
        set_receiving(&hub_id, false);
        sender.send(face.clone());
        let child =
            match decode::spawn_url_av_decoder(&ffmpeg, &url, &extra_args, CANVAS_W, CANVAS_H) {
                Ok(child) => child,
                Err(err) => {
                    clear_receiving(&hub_id);
                    sender.close(Some(CaptureError::Backend(err)));
                    return;
                }
            };
        let born = Instant::now();
        let (got_frames, stderr_tail) = pump_stretch(child, &hub_id, &sender, &stop);
        set_receiving(&hub_id, false);
        if stop.load(Ordering::Relaxed) || !sender.is_open() {
            break;
        }
        if got_frames {
            strikes = 0;
        } else if stderr_tail.contains("does not contain any stream") {
            // The sender connected but delivered no video — re-listening
            // cannot fix an audio-only feed, so say the real cause instead
            // of blaming the port (or silently re-dropping the sender).
            clear_receiving(&hub_id);
            sender.close(Some(CaptureError::Backend(format!(
                "the sender delivered no video stream — {} ingest needs video \
                 (audio-only feeds are not supported)",
                config.protocol.label()
            ))));
            return;
        } else if born.elapsed() < INSTANT_EXIT {
            // Died immediately without a single frame — the port may be
            // taken, or ffmpeg said why. Error out honestly, never spin.
            strikes += 1;
            if strikes >= MAX_STRIKES {
                clear_receiving(&hub_id);
                let cause = if stderr_tail.is_empty() {
                    format!("is port {} already in use?", config.port)
                } else {
                    stderr_tail
                };
                sender.close(Some(CaptureError::Backend(format!(
                    "the {} listener keeps stopping before any sender connects — {cause}",
                    config.protocol.label()
                ))));
                return;
            }
        }
        // Re-listen at a walking pace (never a spin), stop-checked.
        for _ in 0..10 {
            if stop.load(Ordering::Relaxed) || !sender.is_open() {
                break 'listen;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    clear_receiving(&hub_id);
    sender.close(None);
}

/// One listener stretch: demux the child's AVI stdout until it ends (sender
/// disconnected / never connected / studio stop). Returns whether any video
/// frame arrived. The watchdog kills the child on stop so the blocking pipe
/// read always unblocks (the playlist's pattern).
fn pump_stretch(
    mut child: Child,
    hub_id: &str,
    sender: &FrameSender,
    stop: &Arc<AtomicBool>,
) -> (bool, String) {
    let stderr_pipe = child.stderr.take();
    let Some(mut stdout) = child.stdout.take() else {
        let _ = child.kill();
        let _ = child.wait();
        return (false, String::new());
    };

    // ffmpeg's parting words name the real failure (e.g. a sender that
    // delivered no video stream) — drain them so the error can be honest.
    // EOF arrives when the child dies, so the drain always finishes.
    let stderr_tail = Arc::new(Mutex::new(String::new()));
    let stderr_drain = stderr_pipe.and_then(|mut pipe| {
        let tail = Arc::clone(&stderr_tail);
        std::thread::Builder::new()
            .name("fcap-lan-ingest-stderr".into())
            .spawn(move || {
                let mut all = String::new();
                let _ = pipe.read_to_string(&mut all);
                let mut last: Vec<&str> = all.lines().rev().take(3).collect();
                last.reverse();
                let mut joined = last.join(" | ");
                if joined.len() > 300 {
                    let mut cut = 300;
                    while !joined.is_char_boundary(cut) {
                        cut -= 1;
                    }
                    joined.truncate(cut);
                }
                *tail
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner) = joined;
            })
            .ok()
    });

    let (kill_tx, watchdog) =
        crate::media::spawn_kill_watchdog("fcap-lan-ingest-watchdog", stop, vec![child]);

    let ring = fcap_audio::media_hub::ring(hub_id);
    ring.clear();
    let frame_bytes = (CANVAS_W * CANVAS_H * 4) as usize;
    let mut got_frames = false;
    loop {
        if stop.load(Ordering::Relaxed) || !sender.is_open() {
            break;
        }
        match next_event(&mut stdout, frame_bytes) {
            AviEvent::Video(data) => {
                if !got_frames {
                    got_frames = true;
                    set_receiving(hub_id, true);
                }
                sender.send(Frame {
                    width: CANVAS_W,
                    height: CANVAS_H,
                    stride: CANVAS_W * 4,
                    format: PixelFormat::Rgba8,
                    data,
                    captured_at: Instant::now(),
                });
            }
            AviEvent::Audio(samples) => {
                ring.push(&samples);
            }
            AviEvent::End => break,
        }
    }

    let _ = kill_tx.send(());
    if let Some(handle) = watchdog {
        let _ = handle.join();
    }
    if let Some(handle) = stderr_drain {
        let _ = handle.join();
    }
    let tail = stderr_tail
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone();
    (got_frames, tail)
}

// ---------------------------------------------------------------------------
// The owned AVI demuxer (RIFF chunk walk — layout pinned by the tests)
// ---------------------------------------------------------------------------

/// Audio chunks larger than this are corrupt, not audio (bounded allocs).
const MAX_AUDIO_CHUNK: u32 = 4 * 1024 * 1024;

enum AviEvent {
    /// One full RGBA frame (exactly canvas-sized).
    Video(Vec<u8>),
    /// Interleaved stereo f32 samples at 48 kHz.
    Audio(Vec<f32>),
    /// EOF / broken pipe / corrupt stream — the stretch is over.
    End,
}

/// Walk the RIFF stream to the next payload chunk. `RIFF`/`LIST` headers
/// descend (4 extra type bytes); `00dc`/`00db` is a video frame and must be
/// exactly `frame_bytes` (rawvideo of a fixed geometry — anything else is
/// corruption); `01wb` is f32le audio; everything else (headers, `JUNK`,
/// indexes) skips. Chunks pad to even length.
fn next_event(reader: &mut impl Read, frame_bytes: usize) -> AviEvent {
    loop {
        let mut header = [0u8; 8];
        if !read_exact_or_end(reader, &mut header) {
            return AviEvent::End;
        }
        let id = [header[0], header[1], header[2], header[3]];
        if &id == b"RIFF" || &id == b"LIST" {
            // Descend: consume the 4-byte list type, then keep walking.
            let mut kind = [0u8; 4];
            if !read_exact_or_end(reader, &mut kind) {
                return AviEvent::End;
            }
            continue;
        }
        let size = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
        let padded = size as u64 + (size as u64 & 1);
        match &id {
            b"00dc" | b"00db" => {
                if size as usize != frame_bytes {
                    return AviEvent::End; // a fixed-geometry pipe never resizes
                }
                let mut data = vec![0u8; frame_bytes];
                if !read_exact_or_end(reader, &mut data) {
                    return AviEvent::End;
                }
                skip(reader, padded - size as u64);
                return AviEvent::Video(data);
            }
            b"01wb" => {
                if size > MAX_AUDIO_CHUNK {
                    return AviEvent::End;
                }
                let mut bytes = vec![0u8; size as usize];
                if !read_exact_or_end(reader, &mut bytes) {
                    return AviEvent::End;
                }
                skip(reader, padded - size as u64);
                // Whole stereo f32 frames only (8 bytes); a torn tail drops.
                let mut samples = Vec::new();
                crate::media::f32_samples_into(&bytes, &mut samples);
                if samples.is_empty() {
                    continue;
                }
                return AviEvent::Audio(samples);
            }
            _ => {
                if !skip(reader, padded) {
                    return AviEvent::End;
                }
            }
        }
    }
}

/// Consume `len` bytes; `false` on EOF.
fn skip(reader: &mut impl Read, len: u64) -> bool {
    if len == 0 {
        return true;
    }
    std::io::copy(&mut reader.take(len), &mut std::io::sink()).is_ok_and(|taken| taken == len)
}

// ---------------------------------------------------------------------------
// The waiting face
// ---------------------------------------------------------------------------

/// The face shown while no sender is connected: the protocol + the
/// reachable connect URL on an opaque dark placard, full canvas size so the
/// stream arriving later never changes the pipe geometry. Program output —
/// language-neutral where possible, like the timer faces.
fn waiting_face(protocol_label: &str, connect_url: &str) -> Frame {
    let text = format!("{protocol_label}\n{connect_url}");
    let raster = render_text(&TextStyle {
        text,
        font_family: None,
        font_file: None,
        size_px: 52.0,
        color: [255, 255, 255, 255],
        align: TextAlign::Center,
        line_spacing: 1.5,
        force_rtl: false,
        wrap_width: Some(CANVAS_W.saturating_sub(160)),
        ..TextStyle::default()
    })
    .ok();
    compose_face(raster)
}

/// Blend the text raster (straight alpha) centered onto the dark canvas.
fn compose_face(raster: Option<Frame>) -> Frame {
    let mut data = Vec::with_capacity((CANVAS_W * CANVAS_H * 4) as usize);
    for _ in 0..(CANVAS_W * CANVAS_H) {
        data.extend_from_slice(&FACE_BG);
    }
    if let Some(raster) = raster {
        crate::compose::blit_centered(&mut data, CANVAS_W as usize, CANVAS_H as usize, &raster);
    }
    Frame {
        width: CANVAS_W,
        height: CANVAS_H,
        stride: CANVAS_W * 4,
        format: PixelFormat::Rgba8,
        data,
        captured_at: Instant::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_urls_embed_the_escaped_passphrase() {
        assert_eq!(
            connect_url(IngestProtocol::Srt, "192.168.1.23", 9710, ""),
            "srt://192.168.1.23:9710"
        );
        assert_eq!(
            connect_url(
                IngestProtocol::Srt,
                "192.168.1.23",
                9710,
                "correct horse battery"
            ),
            "srt://192.168.1.23:9710?passphrase=correct%20horse%20battery"
        );
        // Query-hostile characters must not survive literally.
        assert_eq!(
            connect_url(IngestProtocol::Srt, "10.0.0.5", 9000, "p&ss=100%?"),
            "srt://10.0.0.5:9000?passphrase=p%26ss%3D100%25%3F"
        );
        // RTMP has no passphrase in the protocol — the URL never grows one.
        assert_eq!(
            connect_url(IngestProtocol::Rtmp, "10.0.0.5", 1935, "ignored anyway"),
            "rtmp://10.0.0.5:1935/live"
        );
    }

    #[test]
    fn listener_inputs_bind_all_interfaces_and_pass_the_passphrase_as_an_option() {
        let (url, args) = listener_input(IngestProtocol::Srt, 9710, "");
        assert_eq!(url, "srt://0.0.0.0:9710?mode=listener");
        assert!(args.is_empty(), "no passphrase, no option");

        let (url, args) = listener_input(IngestProtocol::Srt, 9710, "correct horse battery");
        assert_eq!(url, "srt://0.0.0.0:9710?mode=listener");
        // The option form dodges URL escaping on the bind side entirely.
        assert_eq!(args, vec!["-passphrase", "correct horse battery"]);

        let (url, args) = listener_input(IngestProtocol::Rtmp, 1935, "ignored");
        assert_eq!(url, "rtmp://0.0.0.0:1935/live");
        assert_eq!(args, vec!["-listen", "1"]);
    }

    #[test]
    fn ports_and_passphrases_validate_readably() {
        assert!(validate_port(1023).is_err(), "reserved range");
        assert!(validate_port(1024).is_ok());
        assert!(validate_port(65535).is_ok());
        assert!(validate_passphrase("").is_ok(), "empty = open (warned)");
        assert!(validate_passphrase("too short").is_err(), "9 chars");
        assert!(validate_passphrase("just right").is_ok(), "10 chars");
        assert!(validate_passphrase(&"x".repeat(79)).is_ok());
        assert!(validate_passphrase(&"x".repeat(80)).is_err());
    }

    #[test]
    fn avi_chunks_demux_into_frames_and_samples() {
        // A miniature stream in the exact shape ffmpeg writes (verified
        // against the pinned build): RIFF/LIST headers descend, junk skips
        // (with odd-length padding), 00dc is a fixed-size frame, 01wb is
        // f32le audio.
        let frame_bytes = 4 * 2 * 4; // a 4×2 RGBA "canvas"
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
        bytes.extend_from_slice(b"AVI ");
        bytes.extend_from_slice(b"LIST");
        bytes.extend_from_slice(&100u32.to_le_bytes()); // list sizes are ignored
        bytes.extend_from_slice(b"hdrl");
        bytes.extend_from_slice(b"avih");
        bytes.extend_from_slice(&3u32.to_le_bytes()); // odd → 1 pad byte
        bytes.extend_from_slice(&[1, 2, 3, 0]);
        bytes.extend_from_slice(b"LIST");
        bytes.extend_from_slice(&100u32.to_le_bytes());
        bytes.extend_from_slice(b"movi");
        bytes.extend_from_slice(b"00dc");
        bytes.extend_from_slice(&(frame_bytes as u32).to_le_bytes());
        bytes.extend_from_slice(&vec![0xAB; frame_bytes]);
        bytes.extend_from_slice(b"01wb");
        bytes.extend_from_slice(&16u32.to_le_bytes()); // 2 stereo f32 frames
        for value in [0.5f32, -0.5, 0.25, -0.25] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes.extend_from_slice(b"00dc");
        bytes.extend_from_slice(&(frame_bytes as u32).to_le_bytes());
        bytes.extend_from_slice(&vec![0xCD; frame_bytes]);

        let mut reader = std::io::Cursor::new(bytes);
        match next_event(&mut reader, frame_bytes) {
            AviEvent::Video(data) => assert!(data.iter().all(|byte| *byte == 0xAB)),
            _ => panic!("expected the first video frame"),
        }
        match next_event(&mut reader, frame_bytes) {
            AviEvent::Audio(samples) => assert_eq!(samples, vec![0.5, -0.5, 0.25, -0.25]),
            _ => panic!("expected the audio chunk"),
        }
        match next_event(&mut reader, frame_bytes) {
            AviEvent::Video(data) => assert!(data.iter().all(|byte| *byte == 0xCD)),
            _ => panic!("expected the second video frame"),
        }
        assert!(matches!(
            next_event(&mut reader, frame_bytes),
            AviEvent::End
        ));
    }

    #[test]
    fn a_resized_video_chunk_ends_the_stretch_instead_of_desyncing() {
        // The pipe is fixed-geometry by construction — a mismatched frame
        // size means corruption, and reading on would garble everything.
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"00dc");
        bytes.extend_from_slice(&12u32.to_le_bytes());
        bytes.extend_from_slice(&[0u8; 12]);
        let mut reader = std::io::Cursor::new(bytes);
        assert!(matches!(next_event(&mut reader, 32), AviEvent::End));
    }

    #[test]
    fn the_waiting_face_is_canvas_sized_with_legible_text() {
        let face = waiting_face("SRT", "srt://192.168.1.23:9710");
        assert_eq!((face.width, face.height), (CANVAS_W, CANVAS_H));
        assert_eq!(face.data.len(), (CANVAS_W * CANVAS_H * 4) as usize);
        // Corners are the opaque placard (no accidental transparency)…
        assert_eq!(&face.data[0..4], &FACE_BG);
        assert_eq!(face.data[3], 255);
        // …and some pixel differs from the background: text was drawn.
        assert!(
            face.data.chunks_exact(4).any(|pixel| pixel != FACE_BG),
            "the face must actually render the URL"
        );
    }

    #[test]
    fn the_receiving_registry_tracks_sessions_honestly() {
        assert!(!receiving("lan-test-a"), "unknown = not receiving");
        set_receiving("lan-test-a", true);
        assert!(receiving("lan-test-a"));
        set_receiving("lan-test-a", false);
        assert!(!receiving("lan-test-a"));
        clear_receiving("lan-test-a");
        assert!(!receiving("lan-test-a"));
    }
}
