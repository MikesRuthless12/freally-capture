//! The WebSocket remote-control API (Phase 7, TASK-701) — the transport,
//! auth, and protocol half. The app supplies the actual command surface via
//! [`RemoteHandler`]; this module never touches the studio (or any file)
//! itself, so the API's reach is exactly the allowlist the app implements.
//!
//! **Security posture (charter):** off by default — the server only exists
//! while the user has enabled it in Settings, and stopping it closes the
//! port. Binds `127.0.0.1` unless the user explicitly opts into LAN. A
//! **password is required** to enable it, and the wire never carries it:
//! auth is a challenge–response (the client proves knowledge of
//! `sha256(password + salt)` by answering a per-connection random challenge),
//! verified with a constant-time compare. Unauthenticated connections get
//! one shot at `auth` within a short deadline, then the socket closes.
//!
//! ## Protocol (JSON text messages)
//!
//! ```text
//! server → {"op":"hello","rpcVersion":1,"challenge":"…","salt":"…"}
//! client → {"op":"auth","auth":hex(sha256(hex(sha256(password+salt))+challenge))}
//! server → {"op":"authOk"} | {"op":"authFail","error":"…"} (then close)
//! client → {"op":"request","id":"…","command":"…","params":{…}}
//! server → {"op":"response","id":"…","ok":true,"data":…} |
//!          {"op":"response","id":"…","ok":false,"error":"…"}
//! server → {"op":"event","name":"…","data":…}          (push, after auth)
//! ```

use std::io::ErrorKind;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tungstenite::protocol::WebSocketConfig;
use tungstenite::Message;

/// The default port (one above obs-websocket's 4455 — side-by-side friendly).
pub const DEFAULT_REMOTE_PORT: u16 = 4456;
/// Protocol version, sent in `hello`.
pub const RPC_VERSION: u32 = 1;
/// Hard cap on an inbound message (a controller sends tiny JSON).
const MAX_MESSAGE_BYTES: usize = 64 * 1024;
/// How long an unauthenticated connection may exist.
const AUTH_DEADLINE: Duration = Duration::from_secs(10);
/// The idle read-timeout tick — how often shutdown + queued events are seen.
const TICK: Duration = Duration::from_millis(50);

/// The app-provided command surface. Implementations validate the command
/// name against their own allowlist and must never take a file path to read —
/// "cannot read arbitrary files" is a design guarantee of the surface.
pub trait RemoteHandler: Send + Sync + 'static {
    /// Execute `command` with `params`; `Ok` data (may be `Value::Null`) or a
    /// human-readable error. Runs on the requesting client's thread.
    fn handle(&self, command: &str, params: &Value) -> Result<Value, String>;
}

/// A running remote-control server. Dropping (or [`RemoteServer::stop`])
/// shuts it down — the port closes and every client socket ends.
pub struct RemoteServer {
    shutdown: Arc<AtomicBool>,
    clients: Arc<Mutex<Vec<mpsc::Sender<String>>>>,
    port: u16,
    lan: bool,
}

impl RemoteServer {
    /// Bind + start serving. `port` 0 binds an ephemeral port (tests);
    /// `lan` false binds loopback only. Fails without a non-empty password.
    pub fn start(
        port: u16,
        lan: bool,
        password: &str,
        handler: Arc<dyn RemoteHandler>,
    ) -> Result<Self, String> {
        if password.trim().is_empty() {
            return Err("the remote API requires a password — set one in Settings".into());
        }
        let host = if lan { "0.0.0.0" } else { "127.0.0.1" };
        let listener = TcpListener::bind((host, port))
            .map_err(|err| format!("could not bind {host}:{port}: {err}"))?;
        let port = listener
            .local_addr()
            .map_err(|err| format!("no local addr: {err}"))?
            .port();
        listener
            .set_nonblocking(true)
            .map_err(|err| format!("nonblocking listener: {err}"))?;

        let shutdown = Arc::new(AtomicBool::new(false));
        let clients: Arc<Mutex<Vec<mpsc::Sender<String>>>> = Arc::new(Mutex::new(Vec::new()));
        let password = password.to_owned();
        {
            let shutdown = Arc::clone(&shutdown);
            let clients = Arc::clone(&clients);
            thread::Builder::new()
                .name("fcap-remote-accept".into())
                .spawn(move || {
                    while !shutdown.load(Ordering::Relaxed) {
                        match listener.accept() {
                            Ok((stream, _addr)) => {
                                let shutdown = Arc::clone(&shutdown);
                                let clients = Arc::clone(&clients);
                                let handler = Arc::clone(&handler);
                                let password = password.clone();
                                let spawned = thread::Builder::new()
                                    .name("fcap-remote-client".into())
                                    .spawn(move || {
                                        serve_client(stream, &password, handler, clients, shutdown)
                                    });
                                if let Err(err) = spawned {
                                    eprintln!("remote: client thread spawn failed: {err}");
                                }
                            }
                            Err(err) if err.kind() == ErrorKind::WouldBlock => {
                                thread::sleep(Duration::from_millis(100));
                            }
                            Err(err) => {
                                eprintln!("remote: accept error: {err}");
                                thread::sleep(Duration::from_millis(250));
                            }
                        }
                    }
                    // The listener drops here — the port is closed.
                })
                .map_err(|err| format!("accept thread: {err}"))?;
        }
        Ok(Self {
            shutdown,
            clients,
            port,
            lan,
        })
    }

    /// The bound port (resolves an ephemeral bind).
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Whether the server accepts LAN connections (vs loopback only).
    pub fn lan(&self) -> bool {
        self.lan
    }

    /// Push an event to every authenticated client (drops dead ones).
    pub fn publish_event(&self, name: &str, data: Value) {
        let text = json!({ "op": "event", "name": name, "data": data }).to_string();
        let mut clients = lock(&self.clients);
        clients.retain(|sender| sender.send(text.clone()).is_ok());
    }

    /// Stop serving: the port closes and every client connection ends.
    pub fn stop(&self) {
        self.shutdown.store(true, Ordering::Relaxed);
        lock(&self.clients).clear();
    }
}

impl Drop for RemoteServer {
    fn drop(&mut self) {
        self.stop();
    }
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// What a client may send.
#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
enum ClientMsg {
    Auth {
        auth: String,
    },
    Request {
        id: String,
        command: String,
        #[serde(default)]
        params: Value,
    },
}

/// One client connection, on its own thread: hello → auth gate → serve
/// requests + forward pushed events until close/shutdown.
fn serve_client(
    stream: TcpStream,
    password: &str,
    handler: Arc<dyn RemoteHandler>,
    clients: Arc<Mutex<Vec<mpsc::Sender<String>>>>,
    shutdown: Arc<AtomicBool>,
) {
    if stream.set_read_timeout(Some(TICK)).is_err() {
        return;
    }
    let _ = stream.set_nodelay(true);
    let config = WebSocketConfig {
        max_message_size: Some(MAX_MESSAGE_BYTES),
        max_frame_size: Some(MAX_MESSAGE_BYTES),
        ..Default::default()
    };
    let Ok(mut ws) = tungstenite::accept_with_config(stream, Some(config)) else {
        return; // not a WebSocket handshake — drop it
    };

    // Per-connection random challenge + salt (UUID v4 = OS-seeded entropy).
    let challenge = uuid::Uuid::new_v4().to_string();
    let salt = uuid::Uuid::new_v4().to_string();
    let hello = json!({
        "op": "hello",
        "rpcVersion": RPC_VERSION,
        "challenge": challenge,
        "salt": salt,
    });
    if ws.send(Message::Text(hello.to_string())).is_err() {
        return;
    }
    let expected = auth_response(&auth_secret(password, &salt), &challenge);

    let connected_at = Instant::now();
    let mut events: Option<mpsc::Receiver<String>> = None;

    loop {
        if shutdown.load(Ordering::Relaxed) {
            let _ = ws.close(None);
            return;
        }
        if events.is_none() && connected_at.elapsed() > AUTH_DEADLINE {
            let _ = ws.close(None); // never authenticated — done waiting
            return;
        }
        match ws.read() {
            Ok(Message::Text(text)) => {
                let msg: ClientMsg = match serde_json::from_str(&text) {
                    Ok(msg) => msg,
                    Err(_) => {
                        let _ = ws.send(Message::Text(
                            json!({"op":"authFail","error":"unrecognized message"}).to_string(),
                        ));
                        let _ = ws.close(None);
                        return;
                    }
                };
                match msg {
                    ClientMsg::Auth { auth } => {
                        if events.is_some() {
                            continue; // already authenticated — ignore
                        }
                        if verify_digest(&expected, &auth) {
                            let (tx, rx) = mpsc::channel();
                            lock(&clients).push(tx);
                            events = Some(rx);
                            if ws
                                .send(Message::Text(json!({"op":"authOk"}).to_string()))
                                .is_err()
                            {
                                return;
                            }
                        } else {
                            let _ = ws.send(Message::Text(
                                json!({"op":"authFail","error":"authentication failed"})
                                    .to_string(),
                            ));
                            let _ = ws.close(None);
                            return;
                        }
                    }
                    ClientMsg::Request {
                        id,
                        command,
                        params,
                    } => {
                        if events.is_none() {
                            // One strike: a request before auth closes the door.
                            let _ = ws.send(Message::Text(
                                json!({"op":"authFail","error":"not authenticated"}).to_string(),
                            ));
                            let _ = ws.close(None);
                            return;
                        }
                        let reply = match handler.handle(&command, &params) {
                            Ok(data) => json!({"op":"response","id":id,"ok":true,"data":data}),
                            Err(error) => {
                                json!({"op":"response","id":id,"ok":false,"error":error})
                            }
                        };
                        if ws.send(Message::Text(reply.to_string())).is_err() {
                            return;
                        }
                    }
                }
            }
            Ok(Message::Ping(_) | Message::Pong(_) | Message::Binary(_) | Message::Frame(_)) => {}
            Ok(Message::Close(_)) => return,
            Err(tungstenite::Error::Io(err))
                if err.kind() == ErrorKind::WouldBlock || err.kind() == ErrorKind::TimedOut =>
            {
                // Idle tick: forward any pushed events to this client.
                if let Some(rx) = &events {
                    while let Ok(text) = rx.try_recv() {
                        if ws.send(Message::Text(text)).is_err() {
                            return;
                        }
                    }
                }
            }
            Err(_) => return,
        }
    }
}

/// The stored proof of the password: `hex(sha256(password + salt))`. The
/// client computes the same from what the user typed — the password itself
/// never crosses the wire.
pub fn auth_secret(password: &str, salt: &str) -> String {
    hex(&Sha256::digest(format!("{password}{salt}").as_bytes()))
}

/// The handshake answer: `hex(sha256(secret + challenge))`.
pub fn auth_response(secret: &str, challenge: &str) -> String {
    hex(&Sha256::digest(format!("{secret}{challenge}").as_bytes()))
}

/// Constant-time digest compare (both sides are fixed-width hex).
fn verify_digest(expected: &str, presented: &str) -> bool {
    let (a, b) = (expected.as_bytes(), presented.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EchoHandler;
    impl RemoteHandler for EchoHandler {
        fn handle(&self, command: &str, params: &Value) -> Result<Value, String> {
            match command {
                "echo" => Ok(params.clone()),
                other => Err(format!("unknown command: {other}")),
            }
        }
    }

    fn connect(
        port: u16,
    ) -> tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>> {
        let (ws, _) =
            tungstenite::connect(format!("ws://127.0.0.1:{port}")).expect("client connects");
        ws
    }

    fn read_json(
        ws: &mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>,
    ) -> Value {
        loop {
            match ws.read().expect("read") {
                Message::Text(text) => return serde_json::from_str(&text).expect("json"),
                _ => continue,
            }
        }
    }

    #[test]
    fn auth_math_round_trips_and_rejects_wrong_passwords() {
        let secret = auth_secret("hunter2", "salt-1");
        let answer = auth_response(&secret, "challenge-1");
        assert_eq!(
            answer,
            auth_response(&auth_secret("hunter2", "salt-1"), "challenge-1")
        );
        assert_ne!(
            answer,
            auth_response(&auth_secret("hunter3", "salt-1"), "challenge-1")
        );
        assert_ne!(
            answer,
            auth_response(&auth_secret("hunter2", "salt-2"), "challenge-1")
        );
        assert!(verify_digest(&answer, &answer.clone()));
        assert!(!verify_digest(&answer, "deadbeef"));
    }

    #[test]
    fn an_empty_password_refuses_to_start() {
        assert!(RemoteServer::start(0, false, "  ", Arc::new(EchoHandler)).is_err());
    }

    #[test]
    fn full_session_auth_request_event_and_port_close() {
        let server =
            RemoteServer::start(0, false, "pw", Arc::new(EchoHandler)).expect("server starts");
        let port = server.port();

        // Wrong password: authFail and the connection ends.
        {
            let mut ws = connect(port);
            let hello = read_json(&mut ws);
            assert_eq!(hello["op"], "hello");
            let bad = auth_response(
                &auth_secret("wrong", hello["salt"].as_str().unwrap()),
                hello["challenge"].as_str().unwrap(),
            );
            ws.send(Message::Text(json!({"op":"auth","auth":bad}).to_string()))
                .unwrap();
            assert_eq!(read_json(&mut ws)["op"], "authFail");
        }

        // Right password: authOk → request → response → pushed event.
        let mut ws = connect(port);
        let hello = read_json(&mut ws);
        let good = auth_response(
            &auth_secret("pw", hello["salt"].as_str().unwrap()),
            hello["challenge"].as_str().unwrap(),
        );
        ws.send(Message::Text(json!({"op":"auth","auth":good}).to_string()))
            .unwrap();
        assert_eq!(read_json(&mut ws)["op"], "authOk");

        ws.send(Message::Text(
            json!({"op":"request","id":"1","command":"echo","params":{"x":7}}).to_string(),
        ))
        .unwrap();
        let response = read_json(&mut ws);
        assert_eq!(response["op"], "response");
        assert_eq!(response["ok"], true);
        assert_eq!(response["data"]["x"], 7);

        ws.send(Message::Text(
            json!({"op":"request","id":"2","command":"nope"}).to_string(),
        ))
        .unwrap();
        let failed = read_json(&mut ws);
        assert_eq!(failed["ok"], false);

        server.publish_event("state", json!({"recording": true}));
        let event = read_json(&mut ws);
        assert_eq!(event["op"], "event");
        assert_eq!(event["name"], "state");
        assert_eq!(event["data"]["recording"], true);

        // Stop → the port actually closes (disabled = closed, the charter).
        server.stop();
        let deadline = Instant::now() + Duration::from_secs(3);
        while TcpStream::connect(("127.0.0.1", port)).is_ok() {
            assert!(Instant::now() < deadline, "the port stayed open after stop");
            thread::sleep(Duration::from_millis(100));
        }
    }

    #[test]
    fn a_request_before_auth_is_refused_and_closed() {
        let server =
            RemoteServer::start(0, false, "pw", Arc::new(EchoHandler)).expect("server starts");
        let mut ws = connect(server.port());
        let _hello = read_json(&mut ws);
        ws.send(Message::Text(
            json!({"op":"request","id":"1","command":"echo"}).to_string(),
        ))
        .unwrap();
        assert_eq!(read_json(&mut ws)["op"], "authFail");
    }
}
