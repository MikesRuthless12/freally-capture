//! CAP-N12 — the **Freally Link source**: receives another Freally Capture
//! instance's program feed over the owned link protocol (see
//! `fcap_encode::link` for the wire format and the honest v1 codec note:
//! motion-JPEG video + uncompressed stereo f32 audio over TCP).
//!
//! Session shape: one thread connects to the sender, decodes video frames
//! onto the usual latest-wins channel and pushes audio into the mixer's
//! media-hub ring (keyed by the source id, like Media). While unconnected it
//! draws an honest "connecting" face — host, port, animated dots — and
//! retries with exponential backoff; it never freezes on the last frame of
//! a sender that went away. A sender that already has its one receiver says
//! so (busy hello) and this end keeps politely retrying.
//!
//! Nothing here touches the filesystem, and nothing dials anywhere the
//! operator didn't type or pick from the LAN scan.

use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use fcap_capture::{frame_channel, CaptureError, CaptureSession, Frame, FrameSender, PixelFormat};
use fcap_encode::link as wire;

use crate::text::{render_text, TextAlign, TextStyle};

/// The "connecting" face's fixed canvas (scaled by the scene item like any
/// source; the live feed replaces it at the sender's real size).
const FACE_W: u32 = 640;
const FACE_H: u32 = 360;
/// One TCP connect attempt's budget.
const CONNECT_TIMEOUT: Duration = Duration::from_millis(1500);
/// Socket read poll — short so a stop request joins promptly.
const READ_POLL: Duration = Duration::from_millis(150);
/// A connected stream silent this long is treated as gone.
const DEAD_AIR: Duration = Duration::from_secs(10);

/// Reconnect backoff: 500 ms doubling to an 8 s ceiling. Pure — the session
/// sleeps this long (in stop-checked slices) after attempt `attempt` failed.
fn backoff_ms(attempt: u32) -> u64 {
    500u64.saturating_mul(1u64 << attempt.min(4))
}

/// A plausible host: something the user typed or the scan returned. This is
/// a network address by design — it is never treated as a path and never
/// probed on the filesystem.
fn check_host(host: &str) -> Result<(), CaptureError> {
    let trimmed = host.trim();
    if trimmed.is_empty() {
        return Err(CaptureError::Backend(
            "enter the sending instance's address (use Scan the LAN)".into(),
        ));
    }
    if trimmed.len() > 253 || trimmed.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err(CaptureError::Backend(
            "that address does not look valid".into(),
        ));
    }
    Ok(())
}

/// Start the Freally Link receiver session. `key` is the sender's pairing
/// key, presented as the first frame — the sender serves nothing without it.
pub fn start_link(
    id: &str,
    host: &str,
    port: u16,
    key: &str,
) -> Result<CaptureSession, CaptureError> {
    check_host(host)?;
    if port == 0 {
        return Err(CaptureError::Backend("the link port cannot be 0".into()));
    }
    let host = host.trim().to_owned();
    let id = id.to_owned();
    let key = key.to_owned();
    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);
    let join = std::thread::Builder::new()
        .name("fcap-link".into())
        .spawn(move || run(&id, &host, port, &key, &sender, &thread_stop))
        .map_err(|err| CaptureError::Backend(err.to_string()))?;
    Ok(CaptureSession::from_parts(receiver, stop, join))
}

fn stopped(stop: &AtomicBool, sender: &FrameSender) -> bool {
    stop.load(Ordering::Relaxed) || !sender.is_open()
}

fn run(id: &str, host: &str, port: u16, key: &str, sender: &FrameSender, stop: &AtomicBool) {
    // Hold the mixer ring for the session's lifetime so hide→show rendezvous
    // on the same buffer (the media-hub contract).
    let ring = fcap_audio::media_hub::ring(id);
    let mut attempt: u32 = 0;
    let mut face_tick = 0u32;
    loop {
        if stopped(stop, sender) {
            return;
        }
        sender.send(connecting_face(host, port, face_tick));
        face_tick = face_tick.wrapping_add(1);

        match connect(host, port) {
            Ok(stream) => match serve_stream(stream, sender, stop, &ring, host, port, key) {
                StreamEnd::Fatal(err) => {
                    sender.close(Some(CaptureError::Backend(err)));
                    return;
                }
                StreamEnd::Retry => attempt = 0, // it *was* up — restart fast
                StreamEnd::Stopped => return,
            },
            Err(_) => attempt = attempt.saturating_add(1),
        }

        // Backoff between attempts, in stop-checked slices; the face keeps
        // animating so the operator sees the source is alive and trying.
        let wait = Duration::from_millis(backoff_ms(attempt));
        let deadline = Instant::now() + wait;
        while Instant::now() < deadline {
            if stopped(stop, sender) {
                return;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

fn connect(host: &str, port: u16) -> std::io::Result<TcpStream> {
    // Resolve (an IP literal short-circuits; a LAN hostname asks the OS).
    let addr = (host, port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "no address"))?;
    let stream = TcpStream::connect_timeout(&addr, CONNECT_TIMEOUT)?;
    stream.set_nodelay(true)?;
    stream.set_read_timeout(Some(READ_POLL))?;
    Ok(stream)
}

enum StreamEnd {
    /// Wrong protocol version — reconnecting cannot fix it; error honestly.
    Fatal(String),
    /// Connection lost / busy / dead air — reconnect with backoff.
    Retry,
    Stopped,
}

fn serve_stream(
    mut stream: TcpStream,
    sender: &FrameSender,
    stop: &AtomicBool,
    ring: &fcap_audio::capture::CaptureRing,
    host: &str,
    port: u16,
    key: &str,
) -> StreamEnd {
    // The join comes FIRST — magic, then the pairing key. The sender's gate
    // serves nothing until the key checks out, so this end must speak first.
    if stream.write_all(wire::MAGIC).is_err()
        || stream
            .write_all(&wire::encode_frame(
                wire::FRAME_JOIN,
                &wire::encode_join(key),
            ))
            .is_err()
    {
        return StreamEnd::Retry;
    }
    let mut acc = wire::FrameAccumulator::new();
    let mut chunk = vec![0u8; 64 * 1024];
    let mut hello_seen = false;
    let mut last_data = Instant::now();
    let mut face_tick = 0u32;
    loop {
        if stopped(stop, sender) {
            return StreamEnd::Stopped;
        }
        let read = match stream.read(&mut chunk) {
            Ok(0) => return StreamEnd::Retry, // sender closed
            Ok(read) => read,
            Err(err)
                if err.kind() == std::io::ErrorKind::WouldBlock
                    || err.kind() == std::io::ErrorKind::TimedOut =>
            {
                if last_data.elapsed() > DEAD_AIR {
                    return StreamEnd::Retry;
                }
                // Connected but no picture yet (e.g. the sender is idle):
                // keep the honest face moving.
                if !hello_seen {
                    sender.send(connecting_face(host, port, face_tick));
                    face_tick = face_tick.wrapping_add(1);
                }
                continue;
            }
            Err(_) => return StreamEnd::Retry,
        };
        last_data = Instant::now();
        let frames = match acc.feed(&chunk[..read]) {
            Ok(frames) => frames,
            Err(_) => return StreamEnd::Retry, // bad magic / corrupt framing
        };
        for (kind, payload) in frames {
            if !hello_seen {
                // The first frame must be the hello — anything else is not
                // a Freally Link sender.
                if kind != wire::FRAME_HELLO {
                    return StreamEnd::Retry;
                }
                let Some(hello) = wire::decode_hello(&payload) else {
                    return StreamEnd::Retry;
                };
                if hello.version != wire::PROTOCOL_VERSION {
                    return StreamEnd::Fatal(format!(
                        "the other instance speaks Freally Link v{} (this build speaks v{}) — update both apps",
                        hello.version,
                        wire::PROTOCOL_VERSION
                    ));
                }
                if hello.denied {
                    // The key was refused — reconnecting cannot fix a wrong
                    // key, so stop honestly instead of retrying into a wall.
                    return StreamEnd::Fatal(
                        "the sender refused this pairing key — check the key in the \
                         sender's Freally Link output settings"
                            .to_owned(),
                    );
                }
                if hello.busy {
                    // One receiver at a time, refused politely — retry.
                    return StreamEnd::Retry;
                }
                hello_seen = true;
                continue;
            }
            match kind {
                wire::FRAME_VIDEO => {
                    let Some((_, _, jpeg)) = wire::decode_video_payload(&payload) else {
                        return StreamEnd::Retry;
                    };
                    // Decoded dimensions are the truth (the header is a
                    // sanity bound, checked in the decoder above).
                    match image::load_from_memory_with_format(jpeg, image::ImageFormat::Jpeg) {
                        Ok(decoded) => {
                            let rgba = decoded.to_rgba8();
                            let (width, height) = (rgba.width(), rgba.height());
                            sender.send(Frame {
                                width,
                                height,
                                stride: width * 4,
                                format: PixelFormat::Rgba8,
                                data: rgba.into_raw(),
                                captured_at: Instant::now(),
                            });
                        }
                        Err(_) => return StreamEnd::Retry,
                    }
                }
                wire::FRAME_AUDIO => {
                    let Some(samples) = wire::decode_audio_payload(&payload) else {
                        return StreamEnd::Retry;
                    };
                    ring.push(&samples);
                }
                _ => {} // a second hello is harmless
            }
        }
    }
}

/// The honest "connecting" face: product name, target address, animated
/// dots. Program output stays language-neutral (the timer/HUD precedent).
fn connecting_face(host: &str, port: u16, tick: u32) -> Frame {
    let mut face = vec![0u8; (FACE_W * FACE_H * 4) as usize];
    // A dark translucent slab so the face reads on any backdrop.
    for pixel in face.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[14, 16, 22, 215]);
    }
    let dots = ".".repeat(1 + (tick % 3) as usize);
    let text = format!("Freally Link\n{host}:{port}\n{dots}");
    if let Ok(raster) = render_text(&TextStyle {
        text,
        font_family: None,
        font_file: None,
        size_px: 34.0,
        color: [235, 238, 245, 255],
        align: TextAlign::Center,
        line_spacing: 1.3,
        force_rtl: false,
        wrap_width: Some(FACE_W - 40),
        ..TextStyle::default()
    }) {
        blit_center(&mut face, &raster);
    }
    Frame {
        width: FACE_W,
        height: FACE_H,
        stride: FACE_W * 4,
        format: PixelFormat::Rgba8,
        data: face,
        captured_at: Instant::now(),
    }
}

/// Alpha-over `raster` centered onto the (opaque) face buffer — the shared
/// compositor; the face background is opaque, where both alpha rules agree.
fn blit_center(face: &mut [u8], raster: &Frame) {
    crate::compose::blit_centered(face, FACE_W as usize, FACE_H as usize, raster);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn backoff_doubles_and_caps() {
        assert_eq!(backoff_ms(0), 500);
        assert_eq!(backoff_ms(1), 1000);
        assert_eq!(backoff_ms(2), 2000);
        assert_eq!(backoff_ms(3), 4000);
        assert_eq!(backoff_ms(4), 8000);
        assert_eq!(backoff_ms(5), 8000, "capped");
        assert_eq!(backoff_ms(u32::MAX), 8000, "no overflow");
    }

    #[test]
    fn hosts_are_sanity_checked() {
        assert!(check_host("192.168.1.20").is_ok());
        assert!(check_host("gaming-pc.local").is_ok());
        assert!(check_host("").is_err());
        assert!(check_host("   ").is_err());
        assert!(check_host("two words").is_err());
    }

    #[test]
    fn the_connecting_face_draws_something() {
        let face = connecting_face("10.0.0.5", 9720, 0);
        assert_eq!(face.width, FACE_W);
        assert!(
            face.data.chunks_exact(4).any(|px| px[0] > 100),
            "the face has visible text pixels"
        );
    }

    /// End-to-end over loopback: a fake sender speaks the wire protocol and
    /// the session yields its decoded frame + feeds the audio ring.
    #[test]
    fn a_link_session_receives_video_and_audio() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind loopback");
        let port = listener.local_addr().expect("addr").port();

        let feeder = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept");
            // The receiver speaks first: magic, then the pairing key —
            // exactly what the real sender's gate demands.
            let mut magic = [0u8; 5];
            stream.read_exact(&mut magic).expect("receiver magic");
            assert_eq!(&magic, wire::MAGIC, "the receiver leads with the magic");
            let (kind, payload) = wire::read_frame(&mut stream).expect("join frame");
            assert_eq!(kind, wire::FRAME_JOIN, "the first frame is the join");
            assert_eq!(
                wire::decode_join(&payload).as_deref(),
                Some("sesame-key"),
                "the join carries the source's pairing key"
            );
            stream.write_all(wire::MAGIC).expect("magic");
            let hello = wire::encode_hello(&wire::Hello {
                version: wire::PROTOCOL_VERSION,
                busy: false,
                name: "test sender".into(),
                denied: false,
            });
            stream
                .write_all(&wire::encode_frame(wire::FRAME_HELLO, &hello))
                .expect("hello");
            // A 6×4 solid-color JPEG.
            let rgb = image::RgbImage::from_pixel(6, 4, image::Rgb([200, 40, 40]));
            let mut jpeg = Vec::new();
            image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg, 80)
                .encode_image(&rgb)
                .expect("jpeg");
            stream
                .write_all(&wire::encode_frame(
                    wire::FRAME_VIDEO,
                    &wire::encode_video_payload(6, 4, &jpeg),
                ))
                .expect("video");
            // One 10 ms audio block (480 stereo frames).
            let samples = vec![0.25f32; 960];
            stream
                .write_all(&wire::encode_frame(
                    wire::FRAME_AUDIO,
                    &wire::encode_audio_payload(&samples),
                ))
                .expect("audio");
            // Hold the socket open long enough for the session to drain it.
            std::thread::sleep(Duration::from_millis(500));
        });

        let session = start_link("link-test-session", "127.0.0.1", port, "sesame-key")
            .expect("session starts");
        // Frames arrive: first the connecting face, then the decoded video.
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut got_video = false;
        while Instant::now() < deadline {
            match session.frames().recv_timeout(Duration::from_millis(200)) {
                Ok(Some(frame)) if frame.width == 6 && frame.height == 4 => {
                    let px = &frame.data[0..4];
                    assert!(px[0] > 150 && px[1] < 100, "JPEG round-trips roughly red");
                    got_video = true;
                    break;
                }
                Ok(_) => {}
                Err(_) => break,
            }
        }
        assert!(got_video, "the decoded sender frame arrived");
        // The audio frame follows the video frame on the same socket — give
        // the session's read loop a moment to parse it (asserting the very
        // instant the video landed raced it and flaked).
        let ring = fcap_audio::media_hub::ring("link-test-session");
        let audio_deadline = Instant::now() + Duration::from_secs(2);
        while ring.len() < 960 && Instant::now() < audio_deadline {
            std::thread::sleep(Duration::from_millis(10));
        }
        assert_eq!(ring.len(), 960, "the audio block landed in the mixer ring");
        session.stop();
        feeder.join().expect("feeder thread");
    }

    /// A denied hello (wrong key) is FATAL: the session closes with an
    /// honest error instead of retrying into a wall — reconnecting can
    /// never fix a wrong key, and the wire doc promises we stop.
    #[test]
    fn a_denied_hello_stops_the_session_instead_of_retrying() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind loopback");
        let port = listener.local_addr().expect("addr").port();

        let feeder = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept");
            let mut magic = [0u8; 5];
            stream.read_exact(&mut magic).expect("receiver magic");
            let (kind, _) = wire::read_frame(&mut stream).expect("join frame");
            assert_eq!(kind, wire::FRAME_JOIN);
            stream.write_all(wire::MAGIC).expect("magic");
            let hello = wire::encode_hello(&wire::Hello {
                version: wire::PROTOCOL_VERSION,
                busy: false,
                name: "test sender".into(),
                denied: true,
            });
            stream
                .write_all(&wire::encode_frame(wire::FRAME_HELLO, &hello))
                .expect("denied hello");
            // Hold the socket open: the receiver must stop on the denial
            // itself, not on the close that would follow.
            std::thread::sleep(Duration::from_millis(500));
        });

        let session = start_link("link-denied-session", "127.0.0.1", port, "wrong-key")
            .expect("session starts");
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut refused = None;
        while Instant::now() < deadline {
            match session.frames().recv_timeout(Duration::from_millis(200)) {
                Ok(_) => {} // connecting faces are fine while it dials
                Err(err) => {
                    refused = Some(err.to_string());
                    break;
                }
            }
        }
        let refused = refused.expect("the session closed instead of retrying forever");
        assert!(
            refused.contains("pairing key"),
            "the error names the key as the reason: {refused}"
        );
        session.stop();
        feeder.join().expect("feeder thread");
    }
}
