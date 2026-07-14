//! CAP-N12 — the **Freally Link** wire protocol: one Freally Capture
//! instance shares its program (video + master audio) with another over the
//! operator's own network. Both ends are this app, so the protocol is owned
//! end to end — hand-rolled like OSC, VISCA and the web panel; no new
//! dependency, no ffmpeg on either side.
//!
//! **v1 ships motion-JPEG over TCP, stated plainly.** The owned `.frec`
//! codec is *lossless* (FLZ + prediction) and leans on inter-frame deltas —
//! standalone lossless frames of real program content run tens of megabytes
//! each, which no LAN sustains at frame rate. The studio already JPEG-encodes
//! program frames for previews and the passthrough monitor, so the link
//! reuses that path at quality 80: ~a few hundred KB per 1080p frame, fine
//! on wired LAN or good Wi-Fi. Audio is uncompressed interleaved-stereo
//! f32 at 48 kHz (the mixer's native block format) — bandwidth is trivial
//! next to the video.
//!
//! ## Stream framing (TCP)
//!
//! The sender writes the 5-byte magic `FLNK1` once, then length-prefixed
//! frames:
//!
//! ```text
//! [u8 kind] [u32-le payload length] [payload]
//!   kind 0 = video: [u32-le width] [u32-le height] [JPEG bytes]
//!   kind 1 = audio: interleaved stereo f32-le samples (10 ms blocks)
//!   kind 2 = hello: [u8 version] [u8 flags (bit0 = busy)] [UTF-8 name]
//! ```
//!
//! The first frame after the magic is always a hello. A `busy` hello means
//! the sender already has its one allowed receiver — the caller backs off
//! politely. A version other than [`PROTOCOL_VERSION`] is refused honestly
//! (update both apps) instead of guessing at a dialect.
//!
//! ## Discovery packets (DNS wire format on a Freally multicast port)
//!
//! Discovery speaks standard DNS-SD-shaped packets — a PTR question for
//! `_freally-link._tcp.local`, answered with PTR + SRV + TXT + A — encoded
//! and decoded by the pure functions below. **Transport is NOT genuine mDNS
//! on port 5353**: Windows' own resolver holds UDP 5353 and a second plain
//! `std::net` bind fails (verified on the dev machine — sharing it needs
//! `SO_REUSEADDR`, which std cannot set, and pulling a socket dep just for
//! that fails the no-new-deps bar). Both ends of a link are Freally by
//! definition, so discovery instead multicasts the same DNS packets to
//! [`DISCOVERY_GROUP`]:[`DISCOVERY_PORT`] — multicast-local by construction,
//! zero-config in practice, and invisible to (and unbothered by) real mDNS
//! stacks. Manual host:port entry always works without discovery.

use std::io::Read;
use std::net::Ipv4Addr;

/// Stream magic, written once per TCP connection.
pub const MAGIC: &[u8; 5] = b"FLNK1";
/// The protocol this build speaks. Bump on any wire change.
pub const PROTOCOL_VERSION: u8 = 1;

/// Frame kinds.
pub const FRAME_VIDEO: u8 = 0;
pub const FRAME_AUDIO: u8 = 1;
pub const FRAME_HELLO: u8 = 2;
/// Receiver → sender, FIRST frame: the pairing key. The sender streams
/// nothing until it matches (Freally Link carries the program picture —
/// an open port would let any LAN peer watch the show).
pub const FRAME_JOIN: u8 = 3;

/// Per-kind payload caps — a corrupt or hostile length prefix must never
/// drive an allocation. Video covers a q80 JPEG of a 16k-wide canvas with
/// slack; audio covers a full second of stereo f32.
pub const MAX_VIDEO_BYTES: u32 = 64 * 1024 * 1024;
pub const MAX_AUDIO_BYTES: u32 = 48_000 * 2 * 4;
pub const MAX_HELLO_BYTES: u32 = 4 * 1024;
pub const MAX_JOIN_BYTES: u32 = 256;

/// The advertised DNS-SD service name.
pub const SERVICE: &str = "_freally-link._tcp.local";
/// Discovery multicast group (the mDNS group — the packets are DNS-shaped;
/// see the module docs for why the *port* is ours, not 5353).
pub const DISCOVERY_GROUP: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 251);
/// Discovery UDP port (one above the default stream port, 9720).
pub const DISCOVERY_PORT: u16 = 9721;

/// Largest discovery packet either side builds or parses.
pub const MAX_DISCOVERY_BYTES: usize = 1024;

// ---------------------------------------------------------------------------
// Stream framing
// ---------------------------------------------------------------------------

/// Encode one frame: `[kind][u32-le len][payload]`.
pub fn encode_frame(kind: u8, payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(5 + payload.len());
    out.push(kind);
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
    out
}

/// Write one frame as header-then-payload — the same bytes as
/// [`encode_frame`] with no intermediate copy, for the sender's hot loop
/// where the payload is a whole JPEG program frame.
pub fn write_frame(
    writer: &mut impl std::io::Write,
    kind: u8,
    payload: &[u8],
) -> std::io::Result<()> {
    let mut header = [0u8; 5];
    header[0] = kind;
    header[1..5].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    writer.write_all(&header)?;
    writer.write_all(payload)
}

fn cap_for(kind: u8) -> u32 {
    match kind {
        FRAME_VIDEO => MAX_VIDEO_BYTES,
        FRAME_AUDIO => MAX_AUDIO_BYTES,
        FRAME_HELLO => MAX_HELLO_BYTES,
        FRAME_JOIN => MAX_JOIN_BYTES,
        _ => 0,
    }
}

/// Read one frame off the stream. Errors on an unknown kind or a payload
/// past its cap — the version gate makes both protocol errors, and a
/// protocol error means reconnect, never a guess.
pub fn read_frame(reader: &mut impl Read) -> std::io::Result<(u8, Vec<u8>)> {
    let mut head = [0u8; 5];
    reader.read_exact(&mut head)?;
    let kind = head[0];
    let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]);
    let cap = cap_for(kind);
    if cap == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("unknown link frame kind {kind}"),
        ));
    }
    if len > cap {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("link frame of {len} bytes exceeds the {cap}-byte cap"),
        ));
    }
    let mut payload = vec![0u8; len as usize];
    reader.read_exact(&mut payload)?;
    Ok((kind, payload))
}

/// Incremental frame parser for the receiving side. Sockets read in
/// arbitrary chunks under a short poll timeout, and a blocking `read_exact`
/// that times out mid-frame would desync the stream — so the receiver feeds
/// whatever arrived and takes whole frames back out. Bounded: the buffer
/// never grows past one maximal frame, and a bad magic / kind / length is a
/// hard error (the caller reconnects; it never guesses at resync).
#[derive(Default)]
pub struct FrameAccumulator {
    buf: Vec<u8>,
    magic_seen: bool,
}

impl FrameAccumulator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Feed freshly-read bytes; returns every frame completed by them.
    pub fn feed(&mut self, bytes: &[u8]) -> std::io::Result<Vec<(u8, Vec<u8>)>> {
        self.buf.extend_from_slice(bytes);
        let mut frames = Vec::new();
        if !self.magic_seen {
            if self.buf.len() < MAGIC.len() {
                return Ok(frames);
            }
            if &self.buf[..MAGIC.len()] != MAGIC {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "not a Freally Link stream",
                ));
            }
            self.buf.drain(..MAGIC.len());
            self.magic_seen = true;
        }
        while self.buf.len() >= 5 {
            let kind = self.buf[0];
            let len = u32::from_le_bytes([self.buf[1], self.buf[2], self.buf[3], self.buf[4]]);
            let cap = cap_for(kind);
            if cap == 0 || len > cap {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("bad link frame (kind {kind}, {len} bytes)"),
                ));
            }
            let total = 5 + len as usize;
            if self.buf.len() < total {
                break;
            }
            let payload = self.buf[5..total].to_vec();
            self.buf.drain(..total);
            frames.push((kind, payload));
        }
        Ok(frames)
    }
}

/// Read and check the stream magic.
pub fn read_magic(reader: &mut impl Read) -> std::io::Result<()> {
    let mut magic = [0u8; 5];
    reader.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "not a Freally Link stream",
        ));
    }
    Ok(())
}

/// The hello frame both sides exchange semantics through (sender → receiver).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hello {
    pub version: u8,
    /// The sender already has its one allowed receiver.
    pub busy: bool,
    /// The sender's display name (what discovery advertised).
    pub name: String,
    /// The join key did not match — the receiver says so and stops, rather
    /// than reconnecting into a wall. (Sent instead of the program, so a
    /// wrong key never sees a single frame.)
    pub denied: bool,
}

/// Encode a hello payload. Flags ride one byte: bit 0 = busy, bit 1 = denied.
pub fn encode_hello(hello: &Hello) -> Vec<u8> {
    let mut out = Vec::with_capacity(2 + hello.name.len());
    out.push(hello.version);
    out.push(u8::from(hello.busy) | (u8::from(hello.denied) << 1));
    out.extend_from_slice(hello.name.as_bytes());
    out
}

/// Encode a join payload (receiver → sender): the pairing key, as typed.
pub fn encode_join(key: &str) -> Vec<u8> {
    key.as_bytes().to_vec()
}

/// Decode a join payload (`None` = not valid UTF-8 — a hostile probe).
pub fn decode_join(payload: &[u8]) -> Option<String> {
    std::str::from_utf8(payload).ok().map(str::to_owned)
}

/// Constant-time key comparison — a byte-by-byte `==` on a network-facing
/// secret leaks its prefix through timing (the web panel's rule, same fix).
pub fn key_matches(expected: &str, offered: &str) -> bool {
    let (expected, offered) = (expected.as_bytes(), offered.as_bytes());
    if expected.len() != offered.len() {
        return false;
    }
    let mut diff = 0u8;
    for (a, b) in expected.iter().zip(offered) {
        diff |= a ^ b;
    }
    diff == 0
}

/// Decode a hello payload (`None` = malformed).
pub fn decode_hello(payload: &[u8]) -> Option<Hello> {
    if payload.len() < 2 {
        return None;
    }
    Some(Hello {
        version: payload[0],
        busy: payload[1] & 1 == 1,
        denied: payload[1] & 2 == 2,
        name: String::from_utf8_lossy(&payload[2..]).into_owned(),
    })
}

/// Encode a video payload: `[u32-le width][u32-le height][JPEG]`.
pub fn encode_video_payload(width: u32, height: u32, jpeg: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + jpeg.len());
    out.extend_from_slice(&width.to_le_bytes());
    out.extend_from_slice(&height.to_le_bytes());
    out.extend_from_slice(jpeg);
    out
}

/// Decode a video payload (`None` = malformed or absurd dimensions).
pub fn decode_video_payload(payload: &[u8]) -> Option<(u32, u32, &[u8])> {
    if payload.len() < 8 {
        return None;
    }
    let width = u32::from_le_bytes(payload[0..4].try_into().ok()?);
    let height = u32::from_le_bytes(payload[4..8].try_into().ok()?);
    if width == 0 || height == 0 || width > 16_384 || height > 16_384 {
        return None;
    }
    Some((width, height, &payload[8..]))
}

/// Encode an audio payload: interleaved stereo f32-le samples.
pub fn encode_audio_payload(samples: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(samples.len() * 4);
    for sample in samples {
        out.extend_from_slice(&sample.to_le_bytes());
    }
    out
}

/// Decode an audio payload (`None` = not a whole number of stereo f32 pairs).
pub fn decode_audio_payload(payload: &[u8]) -> Option<Vec<f32>> {
    if payload.len() % 8 != 0 {
        return None;
    }
    Some(
        payload
            .chunks_exact(4)
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect(),
    )
}

// ---------------------------------------------------------------------------
// Discovery packets (DNS wire format), pure encode/decode
// ---------------------------------------------------------------------------

/// One discovered Freally Link output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkPeer {
    /// Display name (from the TXT record, falling back to the instance label).
    pub name: String,
    /// IPv4 address, dotted (from the A record).
    pub host: String,
    /// TCP stream port (from the SRV record).
    pub port: u16,
}

const TYPE_A: u16 = 1;
const TYPE_PTR: u16 = 12;
const TYPE_TXT: u16 = 16;
const TYPE_SRV: u16 = 33;
const TYPE_ANY: u16 = 255;
const CLASS_IN: u16 = 1;

fn push_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_be_bytes());
}

/// Write a DNS name as uncompressed labels. Labels past 63 bytes are
/// truncated (a display name can't break the packet).
fn push_name(out: &mut Vec<u8>, name: &str) {
    for label in name.split('.').filter(|label| !label.is_empty()) {
        let bytes = label.as_bytes();
        let len = bytes.len().min(63);
        out.push(len as u8);
        out.extend_from_slice(&bytes[..len]);
    }
    out.push(0);
}

/// A discovery question: one PTR query for [`SERVICE`].
pub fn encode_query() -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
    push_u16(&mut out, 0); // ID (mDNS convention: 0)
    push_u16(&mut out, 0); // flags: standard query
    push_u16(&mut out, 1); // QDCOUNT
    push_u16(&mut out, 0); // ANCOUNT
    push_u16(&mut out, 0); // NSCOUNT
    push_u16(&mut out, 0); // ARCOUNT
    push_name(&mut out, SERVICE);
    push_u16(&mut out, TYPE_PTR);
    push_u16(&mut out, CLASS_IN);
    out
}

/// Whether `packet` is a query asking for [`SERVICE`] (PTR or ANY) — what
/// the announcer answers. Anything else is silently ignored.
pub fn query_wants_service(packet: &[u8]) -> bool {
    if packet.len() < 12 || packet[2] & 0x80 != 0 {
        return false; // truncated, or a response (QR bit set)
    }
    let questions = u16::from_be_bytes([packet[4], packet[5]]);
    let mut offset = 12usize;
    for _ in 0..questions {
        let Some((name, next)) = read_name(packet, offset) else {
            return false;
        };
        if packet.len() < next + 4 {
            return false;
        }
        let qtype = u16::from_be_bytes([packet[next], packet[next + 1]]);
        if name.eq_ignore_ascii_case(SERVICE) && (qtype == TYPE_PTR || qtype == TYPE_ANY) {
            return true;
        }
        offset = next + 4;
    }
    false
}

/// The announcer's answer: PTR → instance, SRV (port), TXT (`name=`), A (ip).
/// Names are written in full (no compression) — parsers still must *read*
/// compressed names, but we never need to emit them.
pub fn encode_response(name: &str, port: u16, ip: Ipv4Addr) -> Vec<u8> {
    // The instance label rides inside a DNS name, so dots would split it
    // into labels — flatten them (the TXT record carries the exact name).
    let label: String = name
        .chars()
        .map(|c| if c == '.' { '-' } else { c })
        .collect();
    let label = if label.is_empty() { "Freally" } else { &label };
    let instance = format!("{label}.{SERVICE}");
    let target = format!("{label}.local");

    let mut out = Vec::with_capacity(256);
    push_u16(&mut out, 0); // ID
    push_u16(&mut out, 0x8400); // flags: response + authoritative
    push_u16(&mut out, 0); // QDCOUNT
    push_u16(&mut out, 4); // ANCOUNT
    push_u16(&mut out, 0); // NSCOUNT
    push_u16(&mut out, 0); // ARCOUNT

    let push_record = |out: &mut Vec<u8>, owner: &str, rtype: u16, rdata: &[u8]| {
        push_name(out, owner);
        push_u16(out, rtype);
        push_u16(out, CLASS_IN);
        out.extend_from_slice(&120u32.to_be_bytes()); // TTL: 2 minutes
        push_u16(out, rdata.len() as u16);
        out.extend_from_slice(rdata);
    };

    // PTR: service → instance.
    let mut ptr = Vec::new();
    push_name(&mut ptr, &instance);
    push_record(&mut out, SERVICE, TYPE_PTR, &ptr);
    // SRV: instance → priority 0, weight 0, port, target host.
    let mut srv = Vec::new();
    push_u16(&mut srv, 0);
    push_u16(&mut srv, 0);
    push_u16(&mut srv, port);
    push_name(&mut srv, &target);
    push_record(&mut out, &instance, TYPE_SRV, &srv);
    // TXT: the exact display name.
    let mut txt = Vec::new();
    let pair = format!("name={name}");
    let bytes = pair.as_bytes();
    let len = bytes.len().min(255);
    txt.push(len as u8);
    txt.extend_from_slice(&bytes[..len]);
    push_record(&mut out, &instance, TYPE_TXT, &txt);
    // A: target host → IPv4.
    push_record(&mut out, &target, TYPE_A, &ip.octets());
    out
}

/// Read a (possibly compressed) DNS name at `offset`. Returns the dotted
/// name and the offset just past it in the *original* run of labels.
/// Bounded: at most 16 pointer jumps and 255 name bytes.
fn read_name(packet: &[u8], mut offset: usize) -> Option<(String, usize)> {
    let mut name = String::new();
    let mut jumps = 0usize;
    let mut end_after_jump: Option<usize> = None;
    loop {
        let len = *packet.get(offset)? as usize;
        if len == 0 {
            let end = end_after_jump.unwrap_or(offset + 1);
            return Some((name, end));
        }
        if len & 0xC0 == 0xC0 {
            // Compression pointer: two bytes, target elsewhere in the packet.
            let low = *packet.get(offset + 1)? as usize;
            if end_after_jump.is_none() {
                end_after_jump = Some(offset + 2);
            }
            offset = ((len & 0x3F) << 8) | low;
            jumps += 1;
            if jumps > 16 {
                return None;
            }
            continue;
        }
        if len > 63 {
            return None;
        }
        let label = packet.get(offset + 1..offset + 1 + len)?;
        if !name.is_empty() {
            name.push('.');
        }
        name.push_str(&String::from_utf8_lossy(label));
        if name.len() > 255 {
            return None;
        }
        offset += 1 + len;
    }
}

/// Parse a discovery response into peers. Tolerant by design: records may
/// arrive in any order, names may be compressed, and anything that doesn't
/// assemble into (SRV port + A address) is skipped rather than failed.
pub fn parse_response(packet: &[u8]) -> Vec<LinkPeer> {
    if packet.len() < 12 || packet[2] & 0x80 == 0 {
        return Vec::new(); // truncated, or a query
    }
    let questions = u16::from_be_bytes([packet[4], packet[5]]) as usize;
    let answers = u16::from_be_bytes([packet[6], packet[7]]) as usize;

    // Skip the question section.
    let mut offset = 12usize;
    for _ in 0..questions {
        let Some((_, next)) = read_name(packet, offset) else {
            return Vec::new();
        };
        offset = next + 4;
    }

    // Collect the records we care about.
    #[derive(Default)]
    struct Instance {
        srv: Option<(u16, String)>, // port + target host name
        txt_name: Option<String>,
    }
    let mut instances: std::collections::HashMap<String, Instance> =
        std::collections::HashMap::new();
    let mut hosts: std::collections::HashMap<String, Ipv4Addr> = std::collections::HashMap::new();

    for _ in 0..answers.min(32) {
        let Some((owner, next)) = read_name(packet, offset) else {
            break;
        };
        if packet.len() < next + 10 {
            break;
        }
        let rtype = u16::from_be_bytes([packet[next], packet[next + 1]]);
        let rdlen = u16::from_be_bytes([packet[next + 8], packet[next + 9]]) as usize;
        let rdata_at = next + 10;
        let Some(rdata) = packet.get(rdata_at..rdata_at + rdlen) else {
            break;
        };
        match rtype {
            TYPE_SRV if rdlen >= 7 => {
                let port = u16::from_be_bytes([rdata[4], rdata[5]]);
                if let Some((target, _)) = read_name(packet, rdata_at + 6) {
                    instances.entry(owner.to_ascii_lowercase()).or_default().srv =
                        Some((port, target.to_ascii_lowercase()));
                }
            }
            TYPE_TXT => {
                // Concatenated length-prefixed strings; we want `name=`.
                let mut at = 0usize;
                while at < rdata.len() {
                    let len = rdata[at] as usize;
                    let Some(chunk) = rdata.get(at + 1..at + 1 + len) else {
                        break;
                    };
                    if let Some(value) = String::from_utf8_lossy(chunk).strip_prefix("name=") {
                        instances
                            .entry(owner.to_ascii_lowercase())
                            .or_default()
                            .txt_name = Some(value.to_owned());
                    }
                    at += 1 + len;
                }
            }
            TYPE_A if rdlen == 4 => {
                hosts.insert(
                    owner.to_ascii_lowercase(),
                    Ipv4Addr::new(rdata[0], rdata[1], rdata[2], rdata[3]),
                );
            }
            _ => {}
        }
        offset = rdata_at + rdlen;
    }

    let mut peers: Vec<LinkPeer> = instances
        .into_iter()
        .filter_map(|(instance, record)| {
            let (port, target) = record.srv?;
            let ip = hosts.get(&target)?;
            // "<label>._freally-link._tcp.local" → "<label>" as the fallback.
            let label = instance.split('.').next().unwrap_or(&instance);
            Some(LinkPeer {
                name: record.txt_name.unwrap_or_else(|| label.to_owned()),
                host: ip.to_string(),
                port,
            })
        })
        .collect();
    peers.sort_by(|a, b| a.name.cmp(&b.name));
    peers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_round_trip_and_refuse_nonsense() {
        let payload = vec![7u8; 1234];
        let bytes = encode_frame(FRAME_VIDEO, &payload);
        let mut cursor = std::io::Cursor::new(&bytes);
        let (kind, back) = read_frame(&mut cursor).expect("frame reads back");
        assert_eq!(kind, FRAME_VIDEO);
        assert_eq!(back, payload);

        // An unknown kind is a protocol error, not a guess.
        let bogus = encode_frame(9, b"x");
        assert!(read_frame(&mut std::io::Cursor::new(&bogus)).is_err());

        // A length past the per-kind cap never allocates.
        let mut oversized = vec![FRAME_AUDIO];
        oversized.extend_from_slice(&(MAX_AUDIO_BYTES + 1).to_le_bytes());
        assert!(read_frame(&mut std::io::Cursor::new(&oversized)).is_err());
    }

    #[test]
    fn magic_gates_the_stream() {
        assert!(read_magic(&mut std::io::Cursor::new(b"FLNK1rest")).is_ok());
        assert!(read_magic(&mut std::io::Cursor::new(b"HTTP/1.1 ")).is_err());
    }

    #[test]
    fn the_accumulator_reassembles_arbitrary_chunking() {
        // Magic + hello + video, delivered one byte at a time.
        let mut stream = Vec::new();
        stream.extend_from_slice(MAGIC);
        stream.extend_from_slice(&encode_frame(
            FRAME_HELLO,
            &encode_hello(&Hello {
                version: PROTOCOL_VERSION,
                busy: false,
                name: "x".into(),
                denied: false,
            }),
        ));
        stream.extend_from_slice(&encode_frame(
            FRAME_VIDEO,
            &encode_video_payload(2, 2, b"jj"),
        ));

        let mut acc = FrameAccumulator::new();
        let mut frames = Vec::new();
        for byte in &stream {
            frames.extend(acc.feed(std::slice::from_ref(byte)).expect("clean feed"));
        }
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].0, FRAME_HELLO);
        assert_eq!(frames[1].0, FRAME_VIDEO);

        // A wrong magic is a hard error, never a resync guess.
        let mut bad = FrameAccumulator::new();
        assert!(bad.feed(b"HTTP/1.1 200").is_err());
    }

    #[test]
    fn hello_round_trips_and_the_version_gate_holds() {
        let hello = Hello {
            version: PROTOCOL_VERSION,
            busy: false,
            name: "Studio PC".into(),
            denied: false,
        };
        let back = decode_hello(&encode_hello(&hello)).expect("hello decodes");
        assert_eq!(back, hello);

        let busy = decode_hello(&encode_hello(&Hello {
            version: PROTOCOL_VERSION,
            busy: true,
            name: String::new(),
            denied: true,
        }))
        .expect("busy hello decodes");
        assert!(busy.busy);
        assert!(busy.denied, "busy and denied pack into one flag byte");

        // A future version decodes (so the receiver can *name* the mismatch)
        // but is not this build's version — the caller refuses it.
        let future = decode_hello(&encode_hello(&Hello {
            version: 9,
            busy: false,
            name: "Newer".into(),
            denied: false,
        }))
        .expect("future hello still decodes");
        assert_ne!(future.version, PROTOCOL_VERSION);
        assert!(decode_hello(&[1]).is_none(), "truncated hello is refused");
    }

    #[test]
    fn video_and_audio_payloads_round_trip() {
        let encoded = encode_video_payload(1920, 1080, b"jfif");
        let (w, h, jpeg) = decode_video_payload(&encoded).expect("video");
        assert_eq!((w, h), (1920, 1080));
        assert_eq!(jpeg, b"jfif");
        assert!(
            decode_video_payload(&encode_video_payload(0, 1080, b"")).is_none(),
            "zero-size frames are refused"
        );

        let samples = vec![0.25f32, -0.5, 1.0, 0.0];
        let back = decode_audio_payload(&encode_audio_payload(&samples)).expect("audio");
        assert_eq!(back, samples);
        assert!(
            decode_audio_payload(&[0u8; 6]).is_none(),
            "a ragged payload is refused"
        );
    }

    #[test]
    fn a_query_asks_for_the_service_and_the_responder_recognizes_it() {
        let query = encode_query();
        assert!(query_wants_service(&query));
        // Our own response is not a query.
        let response = encode_response("Studio", 9720, Ipv4Addr::new(192, 168, 1, 20));
        assert!(!query_wants_service(&response));
        // A query for something else is ignored.
        let mut other = Vec::new();
        push_u16(&mut other, 0);
        push_u16(&mut other, 0);
        push_u16(&mut other, 1);
        push_u16(&mut other, 0);
        push_u16(&mut other, 0);
        push_u16(&mut other, 0);
        push_name(&mut other, "_printer._tcp.local");
        push_u16(&mut other, TYPE_PTR);
        push_u16(&mut other, CLASS_IN);
        assert!(!query_wants_service(&other));
    }

    #[test]
    fn responses_round_trip_through_the_parser() {
        let response = encode_response("Gaming PC", 9720, Ipv4Addr::new(10, 0, 0, 42));
        let peers = parse_response(&response);
        assert_eq!(
            peers,
            vec![LinkPeer {
                name: "Gaming PC".into(),
                host: "10.0.0.42".into(),
                port: 9720,
            }]
        );
        // A dotted display name flattens in the DNS labels but survives in TXT.
        let dotted = encode_response("mike.studio", 9999, Ipv4Addr::new(192, 168, 0, 7));
        let peers = parse_response(&dotted);
        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].name, "mike.studio");
        assert_eq!(peers[0].port, 9999);
    }

    #[test]
    fn compressed_names_parse() {
        // Hand-built response using a compression pointer for the SRV owner
        // and target — real mDNS stacks emit these, so the reader must
        // follow pointers even though our own encoder never writes them.
        let mut packet = Vec::new();
        push_u16(&mut packet, 0);
        push_u16(&mut packet, 0x8400);
        push_u16(&mut packet, 0); // QD
        push_u16(&mut packet, 3); // AN
        push_u16(&mut packet, 0);
        push_u16(&mut packet, 0);
        // Answer 1: PTR, full owner name at offset 12.
        let service_at = packet.len() as u16;
        push_name(&mut packet, SERVICE);
        push_u16(&mut packet, TYPE_PTR);
        push_u16(&mut packet, CLASS_IN);
        packet.extend_from_slice(&120u32.to_be_bytes());
        // PTR rdata: "Box" + pointer back to the service name.
        let mut ptr_rdata = vec![3, b'B', b'o', b'x'];
        ptr_rdata.push(0xC0);
        ptr_rdata.push(service_at as u8);
        push_u16(&mut packet, ptr_rdata.len() as u16);
        let instance_ptr_at = packet.len() as u16; // "Box.<service>" starts here
        packet.extend_from_slice(&ptr_rdata);
        // Answer 2: SRV owned by a pointer to the instance name.
        packet.push(0xC0);
        packet.push(instance_ptr_at as u8);
        push_u16(&mut packet, TYPE_SRV);
        push_u16(&mut packet, CLASS_IN);
        packet.extend_from_slice(&120u32.to_be_bytes());
        let mut srv = Vec::new();
        push_u16(&mut srv, 0);
        push_u16(&mut srv, 0);
        push_u16(&mut srv, 4321);
        let target_at: u16;
        {
            // target: "box.local", written in full inside the rdata.
            let mut t = Vec::new();
            push_name(&mut t, "box.local");
            target_at = (packet.len() + 2 + srv.len()) as u16;
            srv.extend_from_slice(&t);
        }
        push_u16(&mut packet, srv.len() as u16);
        packet.extend_from_slice(&srv);
        // Answer 3: A owned by a pointer to the SRV target.
        packet.push(0xC0);
        packet.push(target_at as u8);
        push_u16(&mut packet, TYPE_A);
        push_u16(&mut packet, CLASS_IN);
        packet.extend_from_slice(&120u32.to_be_bytes());
        push_u16(&mut packet, 4);
        packet.extend_from_slice(&[172, 16, 0, 9]);

        let peers = parse_response(&packet);
        assert_eq!(
            peers,
            vec![LinkPeer {
                name: "box".into(),
                host: "172.16.0.9".into(),
                port: 4321,
            }]
        );
    }
}
