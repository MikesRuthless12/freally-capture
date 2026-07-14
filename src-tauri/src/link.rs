//! CAP-N12 — the **Freally Link output**: share this instance's program
//! (video + master audio) with one other Freally Capture on the operator's
//! own network, where it appears as a "Freally Link" source. The wire
//! protocol and the honest v1 codec note (motion-JPEG + raw stereo f32 over
//! TCP) live in `fcap_encode::link`; the receiving source lives in
//! `fcap_sources::link`.
//!
//! Security posture (the web-panel precedent): **off by default** — no port
//! opens until the operator flips the toggle; the Settings UI carries the
//! honest LAN warning (unencrypted; anyone who can reach the port can watch
//! the program). One receiver at a time — extras get a polite "busy" hello.
//! Deliberately NOT IP-range-gated: a lab VLAN can look non-private, so the
//! label states LAN intent, discovery is multicast-local by construction,
//! and nothing here ever dials out on its own.
//!
//! Discovery: the announcer answers DNS-SD-shaped PTR queries for
//! `_freally-link._tcp.local` — on the Freally multicast port, not genuine
//! mDNS 5353 (Windows' resolver owns 5353 and plain `std::net` cannot share
//! it; see `fcap_encode::link`'s module docs for the tested details).

use std::io::Write;
use std::net::{Ipv4Addr, TcpListener, TcpStream, UdpSocket};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use fcap_encode::link as wire;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

/// The stream's default TCP port (discovery is one above, `link::DISCOVERY_PORT`).
pub const DEFAULT_LINK_PORT: u16 = 9720;
/// v1 video quality — the passthrough monitor uses 70 for latency; the link
/// spends a little more on the picture another studio will re-compose.
const JPEG_QUALITY: u8 = 80;
/// How long a stalled receiver may block a write before it is dropped.
const WRITE_TIMEOUT: Duration = Duration::from_secs(5);
/// How long a fresh connection has to present its key before it is dropped.
const JOIN_TIMEOUT_SECS: u64 = 5;

/// Settings for the Freally Link output. Off by default.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkSettings {
    /// Off by default. While off, no port is open and nothing announces.
    pub enabled: bool,
    pub port: u16,
    /// The name discovery advertises ("" = "Freally Capture").
    pub name: String,
    /// The pairing key a receiver must present. Freally Link carries the
    /// **program picture**, so the port is never open to whoever asks: the
    /// output cannot be enabled without a key (validated below), and a
    /// receiver that cannot present it never sees a frame.
    #[serde(default)]
    pub key: String,
}

impl Default for LinkSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            port: DEFAULT_LINK_PORT,
            name: String::new(),
            key: String::new(),
        }
    }
}

/// The shortest key we let an operator arm the output with.
pub const MIN_LINK_KEY: usize = 8;

impl LinkSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.port < 1024 {
            return Err("the Freally Link port must be 1024 or above".to_owned());
        }
        if self.name.len() > 64 || self.name.chars().any(char::is_control) {
            return Err("the Freally Link name is too long or has control characters".to_owned());
        }
        if self.key.len() > 128 || self.key.chars().any(char::is_control) {
            return Err("the Freally Link key is too long or has control characters".to_owned());
        }
        // The gate exists or the output does not: enabling with a blank (or
        // trivial) key would put the program picture on the network for
        // anyone who can reach the port.
        if self.enabled && self.key.trim().chars().count() < MIN_LINK_KEY {
            return Err(format!(
                "Freally Link needs a key of at least {MIN_LINK_KEY} characters before it can be turned on — receivers must present it to watch the program"
            ));
        }
        Ok(())
    }

    fn join_key(&self) -> String {
        self.key.trim().to_owned()
    }

    fn display_name(&self) -> String {
        let trimmed = self.name.trim();
        if trimmed.is_empty() {
            "Freally Capture".to_owned()
        } else {
            trimmed.to_owned()
        }
    }
}

/// What the studio loop hands the sender each readback: the program frame,
/// tight RGBA, shared not copied.
struct ProgramSlot {
    width: u32,
    height: u32,
    data: Arc<Vec<u8>>,
}

/// State shared between the studio loop, the accept loop and client threads.
#[derive(Default)]
struct Shared {
    frame: Mutex<Option<ProgramSlot>>,
    seq: AtomicU64,
    clients: AtomicUsize,
}

/// Managed state: the running server (when enabled) + change detection.
#[derive(Default)]
pub struct LinkState {
    server: Mutex<Option<LinkServer>>,
    seen: Mutex<Option<LinkSettings>>,
    shared: Arc<Shared>,
}

struct LinkServer {
    shutdown: Arc<AtomicBool>,
    port: u16,
}

impl Drop for LinkServer {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
    }
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl LinkState {
    /// Whether the studio loop should hand over program frames — only while
    /// a receiver is actually connected (an idle server costs nothing).
    pub fn wants_frames(&self) -> bool {
        self.shared.clients.load(Ordering::Relaxed) > 0
    }

    /// Latest-wins program frame from the studio loop (per render tick).
    /// The client thread JPEG-encodes at its own pace and skips to newest.
    pub fn push_video(&self, data: Arc<Vec<u8>>, width: u32, height: u32) {
        *lock(&self.shared.frame) = Some(ProgramSlot {
            width,
            height,
            data,
        });
        self.shared.seq.fetch_add(1, Ordering::Relaxed);
    }

    /// The address receivers can type manually, or `None` while off.
    pub fn url(&self) -> Option<String> {
        let guard = lock(&self.server);
        guard.as_ref().map(|server| {
            let host = crate::webpanel::local_ip().unwrap_or_else(|| "127.0.0.1".to_owned());
            format!("{host}:{}", server.port)
        })
    }
}

/// Reconcile the server against settings (called ~1 Hz). Cheap no-op when
/// nothing changed (the web-panel manager pattern).
pub fn reconcile(app: &AppHandle) {
    let settings = app.state::<crate::settings::SettingsStore>().get().link;
    let state = app.state::<LinkState>();
    if lock(&state.seen).as_ref() == Some(&settings) {
        return;
    }
    *lock(&state.server) = None;
    if !settings.enabled {
        *lock(&state.seen) = Some(settings);
        return;
    }
    match start(&settings, Arc::clone(&state.shared)) {
        Ok(server) => {
            println!(
                "link: sharing the program on 0.0.0.0:{} as \"{}\"",
                server.port,
                settings.display_name()
            );
            *lock(&state.server) = Some(server);
            // Commit `seen` only AFTER a successful bind — a failed bind
            // leaves it stale so the next tick retries once the old accept
            // thread (50 ms poll) has released the port.
            *lock(&state.seen) = Some(settings);
        }
        Err(err) => eprintln!("link: could not start (will retry): {err}"),
    }
}

fn start(settings: &LinkSettings, shared: Arc<Shared>) -> Result<LinkServer, String> {
    // 0.0.0.0 on purpose: the entire point is another machine on the LAN.
    // The toggle is off by default and the Settings UI says what this opens.
    let listener = TcpListener::bind(("0.0.0.0", settings.port))
        .map_err(|err| format!("could not bind 0.0.0.0:{}: {err}", settings.port))?;
    let port = listener.local_addr().map_err(|err| err.to_string())?.port();
    listener
        .set_nonblocking(true)
        .map_err(|err| err.to_string())?;

    let shutdown = Arc::new(AtomicBool::new(false));
    let name = settings.display_name();
    let key = settings.join_key();

    // The accept loop: ONE receiver at a time; extras get a busy hello. Every
    // connection must present the pairing key before it is served (see
    // `joined`) — the count is held across the gate so a wrong-key probe
    // cannot be used to sit on the one slot for longer than JOIN_TIMEOUT.
    {
        let shutdown = Arc::clone(&shutdown);
        let shared = Arc::clone(&shared);
        let name = name.clone();
        let key = key.clone();
        std::thread::Builder::new()
            .name("fcap-link-accept".into())
            .spawn(move || {
                while !shutdown.load(Ordering::Relaxed) {
                    match listener.accept() {
                        Ok((stream, _)) => {
                            if shared.clients.fetch_add(1, Ordering::SeqCst) == 0 {
                                let client_shared = Arc::clone(&shared);
                                let client_shutdown = Arc::clone(&shutdown);
                                let client_name = name.clone();
                                let client_key = key.clone();
                                let spawned = std::thread::Builder::new()
                                    .name("fcap-link-client".into())
                                    .spawn(move || {
                                        serve_client(
                                            stream,
                                            &client_shared,
                                            &client_shutdown,
                                            &client_name,
                                            &client_key,
                                        );
                                        client_shared.clients.fetch_sub(1, Ordering::SeqCst);
                                    });
                                if spawned.is_err() {
                                    shared.clients.fetch_sub(1, Ordering::SeqCst);
                                }
                            } else {
                                shared.clients.fetch_sub(1, Ordering::SeqCst);
                                refuse_politely(stream, &name);
                            }
                        }
                        Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                            std::thread::sleep(Duration::from_millis(50));
                        }
                        Err(_) => break,
                    }
                }
            })
            .map_err(|err| err.to_string())?;
    }

    // The discovery announcer. Best-effort: a failed bind (e.g. a second
    // instance on this same machine already announces) leaves the stream
    // server fully usable via a typed host:port — said honestly in the log.
    {
        let shutdown = Arc::clone(&shutdown);
        std::thread::Builder::new()
            .name("fcap-link-announce".into())
            .spawn(move || announce(port, &name, &shutdown))
            .map_err(|err| err.to_string())?;
    }

    Ok(LinkServer { shutdown, port })
}

/// The one place the handshake framing lives: magic, then a hello frame
/// carrying our name and the busy/denied verdict.
fn send_hello(stream: &mut TcpStream, name: &str, busy: bool, denied: bool) -> std::io::Result<()> {
    let hello = wire::encode_hello(&wire::Hello {
        version: wire::PROTOCOL_VERSION,
        busy,
        denied,
        name: name.to_owned(),
    });
    stream.write_all(wire::MAGIC)?;
    wire::write_frame(stream, wire::FRAME_HELLO, &hello)
}

/// A second receiver while one is connected: say busy, close. Never silent.
fn refuse_politely(mut stream: TcpStream, name: &str) {
    let _ = stream.set_write_timeout(Some(Duration::from_secs(2)));
    let _ = send_hello(&mut stream, name, true, false);
}

/// Turn a receiver away with an honest reason and NO program bytes.
fn deny(mut stream: TcpStream, name: &str) {
    let _ = stream.set_write_timeout(Some(Duration::from_secs(2)));
    let _ = send_hello(&mut stream, name, false, true);
}

/// The pairing gate: the receiver must present the key as its FIRST frame,
/// promptly. Freally Link carries the program picture, so an unauthenticated
/// port would let any peer on the network watch the show (the LAN panel's
/// password rule, for the same reason) — nothing is sent until this passes.
/// The compare is constant-time; a wrong key never sees a frame.
fn joined(stream: &mut TcpStream, key: &str) -> bool {
    if stream
        .set_read_timeout(Some(Duration::from_secs(JOIN_TIMEOUT_SECS)))
        .is_err()
    {
        return false;
    }
    if wire::read_magic(stream).is_err() {
        return false;
    }
    let Ok((kind, payload)) = wire::read_frame(stream) else {
        return false;
    };
    if kind != wire::FRAME_JOIN {
        return false;
    }
    let Some(offered) = wire::decode_join(&payload) else {
        return false;
    };
    wire::key_matches(key, &offered)
}

/// One receiver's feed: hello, then audio (contiguous, cursor-read from the
/// CAP-N15 master tap — zero new engine surface) interleaved with the
/// latest program frame, JPEG-encoded here so the render loop never pays
/// for the link.
fn serve_client(
    mut stream: TcpStream,
    shared: &Shared,
    shutdown: &AtomicBool,
    name: &str,
    key: &str,
) {
    if stream.set_nodelay(true).is_err() || stream.set_write_timeout(Some(WRITE_TIMEOUT)).is_err() {
        return;
    }
    // The gate comes FIRST: not one byte of program before the key checks out.
    if !joined(&mut stream, key) {
        deny(stream, name);
        return;
    }
    // Past the gate the socket is write-driven; a lingering read timeout
    // would be meaningless (we never read again) but clear it honestly.
    let _ = stream.set_read_timeout(None);
    if send_hello(&mut stream, name, false, false).is_err() {
        return;
    }

    // Holding the ring subscribes Master with the engine (it publishes only
    // to live targets); the cursor keeps the audio gapless across polls.
    let ring = fcap_audio::vis::ring(&fcap_audio::vis::VisTarget::Master);
    let mut cursor = ring.cursor();
    // Start at the CURRENT seq: the slot may still hold the last frame of a
    // previous session (the studio only pushes while a client is connected),
    // and an old program picture must never flash on a fresh receiver. The
    // studio pushes a fresh frame within a render tick of us counting.
    let mut sent_seq = shared.seq.load(Ordering::Relaxed);

    loop {
        if shutdown.load(Ordering::Relaxed) {
            return;
        }
        // Audio first — small, steady, and the thing that must not gap.
        let (next, samples) = ring.since(cursor);
        cursor = next;
        // 10 ms blocks (480 stereo frames), the mixer's native block size.
        for block in samples.chunks(960) {
            let payload = wire::encode_audio_payload(block);
            if wire::write_frame(&mut stream, wire::FRAME_AUDIO, &payload).is_err() {
                return;
            }
        }
        // Then the newest program frame, if the studio published one.
        let seq = shared.seq.load(Ordering::Relaxed);
        if seq != sent_seq {
            sent_seq = seq;
            let slot = {
                let guard = lock(&shared.frame);
                guard
                    .as_ref()
                    .map(|slot| (slot.width, slot.height, Arc::clone(&slot.data)))
            };
            if let Some((width, height, data)) = slot {
                if let Some(jpeg) = encode_jpeg(width, height, &data) {
                    // Header + payload written straight to the socket — the
                    // payload is a whole program frame, so the intermediate
                    // copies `encode_frame` would make are real bandwidth.
                    let payload = wire::encode_video_payload(width, height, &jpeg);
                    if wire::write_frame(&mut stream, wire::FRAME_VIDEO, &payload).is_err() {
                        return;
                    }
                }
            }
        }
        std::thread::sleep(Duration::from_millis(4));
    }
}

/// Full-resolution RGBA → JPEG at quality 80 — the preview encoder at
/// factor 1 (no downscale). The length guard stays HERE: the frame buffer
/// crossed a thread boundary and a short one must fail, not panic.
fn encode_jpeg(width: u32, height: u32, rgba: &[u8]) -> Option<Vec<u8>> {
    let pixels = width as usize * height as usize;
    if width == 0 || height == 0 || width > 16_384 || height > 16_384 || rgba.len() < pixels * 4 {
        return None;
    }
    crate::studio::encode_program_jpeg(width, height, rgba, width, height, JPEG_QUALITY)
}

/// The discovery responder: answer PTR queries for our service, unicast,
/// to whoever asked. Multicast-local by construction (see the module docs).
fn announce(port: u16, name: &str, shutdown: &AtomicBool) {
    let socket = match UdpSocket::bind(("0.0.0.0", wire::DISCOVERY_PORT)) {
        Ok(socket) => socket,
        Err(err) => {
            eprintln!(
                "link: discovery announcer could not bind UDP {} ({err}) — \
                 receivers can still connect by typed address",
                wire::DISCOVERY_PORT
            );
            return;
        }
    };
    if let Err(err) = socket.join_multicast_v4(&wire::DISCOVERY_GROUP, &Ipv4Addr::UNSPECIFIED) {
        eprintln!("link: discovery multicast join failed ({err})");
        return;
    }
    // Multi-NIC honesty: UNSPECIFIED joins the OS's default multicast
    // interface, which on a box with virtual adapters (WSL, VPN) may not be
    // the physical LAN — observed on the dev machine. Also join on the
    // interface that actually routes to LAN peers (the web panel's
    // UDP-connect trick); a duplicate join just errors and is ignored.
    if let Some(ip) = crate::webpanel::local_ip().and_then(|ip| ip.parse::<Ipv4Addr>().ok()) {
        let _ = socket.join_multicast_v4(&wire::DISCOVERY_GROUP, &ip);
    }
    let _ = socket.set_read_timeout(Some(Duration::from_millis(250)));
    let mut buf = [0u8; wire::MAX_DISCOVERY_BYTES];
    while !shutdown.load(Ordering::Relaxed) {
        match socket.recv_from(&mut buf) {
            Ok((read, src)) => {
                if wire::query_wants_service(&buf[..read]) {
                    let ip = crate::webpanel::local_ip()
                        .and_then(|ip| ip.parse::<Ipv4Addr>().ok())
                        .unwrap_or(Ipv4Addr::LOCALHOST);
                    let response = wire::encode_response(name, port, ip);
                    let _ = socket.send_to(&response, src);
                }
            }
            Err(ref err)
                if err.kind() == std::io::ErrorKind::WouldBlock
                    || err.kind() == std::io::ErrorKind::TimedOut => {}
            Err(_) => return,
        }
    }
}

/// Poll settings ~1 Hz and keep the server in sync (the panel manager's
/// shape). Winds down with the app.
pub fn spawn_manager(app: AppHandle) {
    std::thread::Builder::new()
        .name("fcap-link-manager".into())
        .spawn(move || loop {
            reconcile(&app);
            std::thread::sleep(Duration::from_secs(1));
        })
        .expect("link manager thread spawns");
}

/// One discovered sender, as the picker's scan shows it.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkPeerDto {
    pub name: String,
    pub host: String,
    pub port: u16,
}

/// Scan the LAN for Freally Link outputs (~2 s). User-initiated only — the
/// picker's button; nothing scans in the background.
#[tauri::command]
pub async fn link_discover() -> Result<Vec<LinkPeerDto>, String> {
    tauri::async_runtime::spawn_blocking(|| {
        let socket = UdpSocket::bind(("0.0.0.0", 0)).map_err(|err| err.to_string())?;
        // Loop our own multicast back so an announcer on this same machine
        // is discoverable too (two instances on one PC is a real test rig).
        let _ = socket.set_multicast_loop_v4(true);
        let _ = socket.set_read_timeout(Some(Duration::from_millis(250)));
        let query = wire::encode_query();
        let target = (wire::DISCOVERY_GROUP, wire::DISCOVERY_PORT);
        socket
            .send_to(&query, target)
            .map_err(|err| err.to_string())?;

        let mut peers: Vec<LinkPeerDto> = Vec::new();
        let deadline = Instant::now() + Duration::from_secs(2);
        let mut resent = false;
        let mut buf = [0u8; wire::MAX_DISCOVERY_BYTES];
        while Instant::now() < deadline {
            // One repeat at half time — multicast is best-effort delivery.
            if !resent && deadline - Instant::now() < Duration::from_secs(1) {
                let _ = socket.send_to(&query, target);
                resent = true;
            }
            let Ok((read, _)) = socket.recv_from(&mut buf) else {
                continue;
            };
            for peer in wire::parse_response(&buf[..read]) {
                if !peers
                    .iter()
                    .any(|known| known.host == peer.host && known.port == peer.port)
                {
                    peers.push(LinkPeerDto {
                        name: peer.name,
                        host: peer.host,
                        port: peer.port,
                    });
                }
            }
        }
        peers.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(peers)
    })
    .await
    .map_err(|err| format!("link scan task failed: {err}"))?
}

/// The output's address for the Settings UI, or `None` while off.
#[tauri::command]
pub fn link_url(state: tauri::State<'_, LinkState>) -> Option<String> {
    state.url()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_default_off_and_validate() {
        let off = LinkSettings::default();
        assert!(!off.enabled, "off by default — no port is open");
        assert_eq!(off.port, DEFAULT_LINK_PORT);
        assert!(off.validate().is_ok());

        let privileged = LinkSettings {
            port: 80,
            ..LinkSettings::default()
        };
        assert!(privileged.validate().is_err(), "no privileged ports");

        let long_name = LinkSettings {
            name: "x".repeat(65),
            ..LinkSettings::default()
        };
        assert!(long_name.validate().is_err());
    }

    #[test]
    fn an_idle_state_wants_no_frames() {
        let state = LinkState::default();
        assert!(!state.wants_frames(), "no client → the studio skips us");
        state.push_video(Arc::new(vec![0u8; 16]), 2, 2);
        assert!(
            !state.wants_frames(),
            "a pushed frame alone changes nothing"
        );
    }

    #[test]
    fn jpeg_encode_rejects_short_buffers_and_encodes_real_ones() {
        assert!(encode_jpeg(4, 4, &[0u8; 8]).is_none());
        let jpeg = encode_jpeg(4, 4, &[128u8; 4 * 4 * 4]).expect("encodes");
        assert_eq!(&jpeg[..2], &[0xFF, 0xD8], "JFIF magic");
    }
}
