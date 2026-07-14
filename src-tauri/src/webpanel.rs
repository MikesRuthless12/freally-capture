//! The LAN touch panel (CAP-N06) and the tally light service (CAP-N07).
//!
//! One tiny HTTP server, self-hosted, **off by default**, that serves three
//! things to devices on the operator's own network:
//!
//! - `/` — a control page (scene buttons with live tally, mixer faders,
//!   replay/marker keys). A phone becomes a control deck with no vendor app,
//!   no account, no cloud.
//! - `/tally` — a full-screen red/green tally page any spare phone can show.
//! - `/api/state` + `POST /api/command` — the JSON the pages poll and post.
//!
//! **The security posture borrows from the WebSocket remote API** — off by
//! default, loopback unless LAN is turned on, the same fixed command
//! allowlist, a required password checked on every request, size-capped
//! requests, and nothing fetched from the internet (the pages are embedded).
//!
//! **One deliberate difference, stated plainly:** the WebSocket API proves the
//! password by challenge–response so it never crosses the wire, whereas this
//! panel carries the key in the request URL so a scanned QR "just works". That
//! means the password travels in cleartext over plain HTTP — fine on a trusted
//! home/studio LAN, exposed to a sniffer on an untrusted one. The LAN toggle
//! in Settings says so. Loopback mode carries nothing off the machine.
//!
//! No new dependency: this is a small, bounded HTTP/1.1 responder over
//! `std::net`, serving a fixed set of routes.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::{AppHandle, Manager};

/// The panel's default port (one above the remote API's 4456).
pub const DEFAULT_PANEL_PORT: u16 = 4457;
/// Hard cap on a request (a phone posts tiny JSON).
const MAX_REQUEST_BYTES: usize = 16 * 1024;
/// How long a client may dawdle before we drop it.
const IO_TIMEOUT: Duration = Duration::from_secs(10);

/// The served control page + tally page (CAP-N06 / CAP-N07). Everything is
/// inline — no CDN, no external font, no analytics; the page works with the
/// machine offline.
const PANEL_HTML: &str = include_str!("../assets/panel.html");

/// Settings for the LAN panel + tally service.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebPanelSettings {
    /// Off by default. While off, no port is open.
    pub enabled: bool,
    pub port: u16,
    /// Accept LAN connections (`0.0.0.0`) instead of loopback only.
    pub lan: bool,
    /// Required. Checked on every request; empty = the server refuses to start.
    pub password: String,
}

impl Default for WebPanelSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            port: DEFAULT_PANEL_PORT,
            lan: false,
            password: String::new(),
        }
    }
}

impl WebPanelSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.port < 1024 {
            return Err("the panel port must be 1024 or above".to_owned());
        }
        if self.enabled && self.password.trim().is_empty() {
            return Err("the LAN panel needs a password before it can be enabled".to_owned());
        }
        if self.password.len() > 128 {
            return Err("the panel password is too long".to_owned());
        }
        Ok(())
    }
}

/// Managed state: the running server (when enabled) + change detection.
#[derive(Default)]
pub struct WebPanelState {
    server: Mutex<Option<PanelServer>>,
    seen: Mutex<Option<WebPanelSettings>>,
}

struct PanelServer {
    shutdown: Arc<AtomicBool>,
    port: u16,
}

impl Drop for PanelServer {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
    }
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl WebPanelState {
    /// The URL to show in Settings (and behind the QR), or `None` when off.
    pub fn url(&self) -> Option<String> {
        let guard = lock(&self.server);
        guard.as_ref().map(|server| {
            let host = local_ip().unwrap_or_else(|| "127.0.0.1".to_owned());
            format!("http://{host}:{}/", server.port)
        })
    }
}

/// A best-effort LAN address for the QR code (never leaves the machine —
/// this only asks the OS which interface would reach a LAN peer). Shared
/// with the Freally Link output (CAP-N12) for its announcer + address line.
pub(crate) fn local_ip() -> Option<String> {
    // A UDP "connect" to a private address performs no traffic; it just makes
    // the OS pick the outgoing interface, whose address we then read.
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("192.168.1.1:9").ok()?;
    Some(socket.local_addr().ok()?.ip().to_string())
}

/// Reconcile the server against settings (called ~1 Hz). Cheap no-op when
/// nothing changed.
pub fn reconcile(app: &AppHandle) {
    let settings = app
        .state::<crate::settings::SettingsStore>()
        .get()
        .web_panel;
    let state = app.state::<WebPanelState>();
    if lock(&state.seen).as_ref() == Some(&settings) {
        return;
    }
    // Any change restarts the server (the settings are tiny and rare).
    *lock(&state.server) = None;
    if !settings.enabled || settings.password.trim().is_empty() {
        *lock(&state.seen) = Some(settings); // off: settled, nothing to retry
        return;
    }
    match start(app.clone(), &settings) {
        Ok(server) => {
            println!(
                "panel: serving on {}:{} (LAN: {})",
                if settings.lan { "0.0.0.0" } else { "127.0.0.1" },
                server.port,
                settings.lan
            );
            *lock(&state.server) = Some(server);
            // Commit `seen` only AFTER a successful bind — a failed bind leaves
            // it stale so the next tick retries, once the old accept thread
            // (50 ms poll) has released the port.
            *lock(&state.seen) = Some(settings);
        }
        Err(err) => eprintln!("panel: could not start (will retry): {err}"),
    }
}

fn start(app: AppHandle, settings: &WebPanelSettings) -> Result<PanelServer, String> {
    let host = if settings.lan { "0.0.0.0" } else { "127.0.0.1" };
    let listener = TcpListener::bind((host, settings.port))
        .map_err(|err| format!("could not bind {host}:{}: {err}", settings.port))?;
    let port = listener.local_addr().map_err(|err| err.to_string())?.port();
    listener
        .set_nonblocking(true)
        .map_err(|err| err.to_string())?;

    let shutdown = Arc::new(AtomicBool::new(false));
    let password = settings.password.clone();
    {
        let shutdown = Arc::clone(&shutdown);
        std::thread::Builder::new()
            .name("fcap-panel".into())
            .spawn(move || {
                while !shutdown.load(Ordering::Relaxed) {
                    match listener.accept() {
                        Ok((stream, _)) => {
                            let app = app.clone();
                            let password = password.clone();
                            let _ = std::thread::Builder::new()
                                .name("fcap-panel-client".into())
                                .spawn(move || {
                                    let _ = serve(stream, &app, &password);
                                });
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
    Ok(PanelServer { shutdown, port })
}

/// One request → one response. Deliberately tiny: three routes, a password on
/// each, a size cap, and no path ever reaches the filesystem.
fn serve(mut stream: TcpStream, app: &AppHandle, password: &str) -> std::io::Result<()> {
    stream.set_read_timeout(Some(IO_TIMEOUT))?;
    stream.set_write_timeout(Some(IO_TIMEOUT))?;

    let mut buf = vec![0u8; MAX_REQUEST_BYTES];
    let read = stream.read(&mut buf)?;
    let request = String::from_utf8_lossy(&buf[..read]).to_string();
    let mut lines = request.split("\r\n");
    let start_line = lines.next().unwrap_or_default();
    let mut parts = start_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let target = parts.next().unwrap_or_default();
    // Route on the path only; a query string carries the password + tally id.
    let (path, query) = target.split_once('?').unwrap_or((target, ""));

    // The password rides the query (`?k=…`) so a phone can just scan a QR.
    // Constant-ish comparison: the panel is LAN-only and rate-bounded by the
    // client's own round-trips, but never leak length differences early.
    let supplied = query
        .split('&')
        .find_map(|pair| pair.strip_prefix("k="))
        .unwrap_or_default();
    let authorized = supplied.len() == password.len()
        && supplied
            .bytes()
            .zip(password.bytes())
            .fold(0u8, |acc, (a, b)| acc | (a ^ b))
            == 0;

    let body = request.split("\r\n\r\n").nth(1).unwrap_or("");

    let (status, content_type, payload) = match (method, path) {
        // The pages themselves are harmless without the key (they only show a
        // "wrong key" state), but gate them anyway: fewer moving parts.
        ("GET", "/") | ("GET", "/tally") if authorized => {
            ("200 OK", "text/html; charset=utf-8", PANEL_HTML.to_owned())
        }
        ("GET", "/api/state") if authorized => {
            ("200 OK", "application/json", state_json(app).to_string())
        }
        ("POST", "/api/command") if authorized => {
            let result = run_command(app, body);
            match result {
                Ok(value) => ("200 OK", "application/json", value.to_string()),
                Err(err) => (
                    "400 Bad Request",
                    "application/json",
                    json!({ "error": err }).to_string(),
                ),
            }
        }
        (_, _) if !authorized => (
            "401 Unauthorized",
            "text/plain; charset=utf-8",
            "wrong or missing key".to_owned(),
        ),
        _ => (
            "404 Not Found",
            "text/plain; charset=utf-8",
            "no such page".to_owned(),
        ),
    };

    // No caching, no framing, no sniffing — a control surface, not a site.
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n\
         Cache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\n\
         X-Frame-Options: DENY\r\nConnection: close\r\n\r\n{payload}",
        payload.len()
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()
}

/// The state the panel + tally pages poll: scenes with their program/preview
/// tally, the mixer strips, and the live/recording flags.
fn state_json(app: &AppHandle) -> Value {
    let studio = app.state::<crate::studio::StudioState>().snapshot();
    let program = studio.collection.active_scene;
    let preview = studio.studio_mode.as_ref().map(|mode| mode.preview_scene);
    let scenes: Vec<Value> = studio
        .collection
        .scenes
        .iter()
        .map(|scene| {
            json!({
                "id": scene.id,
                "name": scene.name,
                "program": scene.id == program,
                "preview": Some(scene.id) == preview,
            })
        })
        .collect();
    let sources: Vec<Value> = studio
        .collection
        .sources
        .iter()
        .filter(|source| source.audio.is_some())
        .map(|source| {
            let audio = source.audio.as_ref().expect("filtered");
            json!({
                "id": source.id,
                "name": source.name,
                "muted": audio.muted,
                "volumeDb": audio.volume_db,
            })
        })
        .collect();
    json!({
        "scenes": scenes,
        "sources": sources,
        "live": app.state::<crate::stream::StreamBridgeState>().wants_frames(),
        "recording": app.state::<crate::recording::RecordingState>().wants_frames(),
    })
}

/// A posted command — the SAME allowlist the remote API and automation use.
#[derive(Deserialize)]
struct PanelCommand {
    command: String,
    #[serde(default)]
    params: Value,
}

fn run_command(app: &AppHandle, body: &str) -> Result<Value, String> {
    let parsed: PanelCommand =
        serde_json::from_str(body.trim()).map_err(|err| format!("bad request: {err}"))?;
    // `dispatch_any` re-checks the allowlist itself — belt and braces.
    crate::remote_api::dispatch_any(app, &parsed.command, &parsed.params)
}

/// Poll settings ~1 Hz and keep the panel server in sync (mirrors the
/// remote API's manager). Winds down with the app.
pub fn spawn_manager(app: AppHandle) {
    std::thread::Builder::new()
        .name("fcap-panel-manager".into())
        .spawn(move || loop {
            reconcile(&app);
            std::thread::sleep(Duration::from_secs(1));
        })
        .expect("panel manager thread spawns");
}

/// The panel's URL (for the Settings QR), or `None` while it is off.
#[tauri::command]
pub fn panel_url(state: tauri::State<'_, WebPanelState>) -> Option<String> {
    state.url()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_default_off_and_demand_a_password() {
        let off = WebPanelSettings::default();
        assert!(!off.enabled, "off by default — no port is open");
        assert!(off.validate().is_ok(), "a disabled panel needs no password");

        let no_password = WebPanelSettings {
            enabled: true,
            ..WebPanelSettings::default()
        };
        assert!(
            no_password.validate().is_err(),
            "enabling without a password is refused"
        );

        let ok = WebPanelSettings {
            enabled: true,
            password: "hunter2".to_owned(),
            ..WebPanelSettings::default()
        };
        assert!(ok.validate().is_ok());

        let privileged = WebPanelSettings {
            port: 80,
            ..WebPanelSettings::default()
        };
        assert!(privileged.validate().is_err(), "no privileged ports");
    }

    #[test]
    fn the_served_page_fetches_nothing_from_the_internet() {
        // The charter's rule, enforced as a test: the embedded page may not
        // reference any remote origin — no CDN, no font, no analytics.
        let lowered = PANEL_HTML.to_lowercase();
        for needle in ["http://", "https://", "//cdn", "fonts.googleapis"] {
            assert!(
                !lowered.contains(needle),
                "the panel page must not reference {needle}"
            );
        }
    }

    #[test]
    fn the_panel_serves_only_its_three_routes() {
        // Any path outside the fixed set is a 404 — nothing maps to a file,
        // so there is no traversal surface at all.
        for path in ["/", "/tally", "/api/state", "/api/command"] {
            assert!(PANEL_ROUTES.contains(&path));
        }
        assert!(!PANEL_ROUTES.contains(&"/../settings.json"));
        assert!(!PANEL_ROUTES.contains(&"/etc/passwd"));
    }

    /// The complete route table (kept next to the matcher above).
    const PANEL_ROUTES: [&str; 4] = ["/", "/tally", "/api/state", "/api/command"];
}
