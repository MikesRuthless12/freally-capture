//! OSC control surface (CAP-N04): TouchOSC-class controllers and lighting
//! desks drive the studio over Open Sound Control — **LAN-only, off by
//! default, never the internet**.
//!
//! Same posture and same vocabulary as everything else here: an OSC address
//! maps onto the **fixed remote-API allowlist**, so OSC can ask for nothing
//! the app's own buttons can't. No file paths, no processes, no new surface.
//!
//! **OSC 1.0 has no authentication, and neither does this** — the protocol is
//! a bare message format with no password or handshake, so on `lan = true`
//! *any* host that can reach the port can drive these commands. That is why it
//! ships off, defaults to loopback (which no other host can reach), and the
//! LAN toggle in Settings warns before it opens `0.0.0.0`. Enable LAN OSC only
//! on a network you trust.
//!
//! `/scene/switch "Live"` · `/mixer/vol "Mic" -6.0` · `/mixer/mute "Mic" 1`
//! `/stream/start` · `/record/start` · `/replay/save` · `/marker/add`
//! `/macro/run "Intro"` · `/transition`
//!
//! The OSC 1.0 wire format is tiny (a null-padded address, a `,tags` string,
//! then big-endian args), so it is parsed here directly — no new dependency,
//! nothing to audit but this file.

use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{AppHandle, Manager};

/// The conventional OSC port (what TouchOSC ships with).
pub const DEFAULT_OSC_PORT: u16 = 9000;
/// An OSC packet is small; anything larger is not ours.
const MAX_PACKET: usize = 4 * 1024;

/// Settings for the OSC surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OscSettings {
    /// Off by default. While off, no socket is bound.
    pub enabled: bool,
    pub port: u16,
    /// Accept packets from the LAN (`0.0.0.0`) instead of loopback only.
    pub lan: bool,
}

impl Default for OscSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            port: DEFAULT_OSC_PORT,
            lan: false,
        }
    }
}

impl OscSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.port < 1024 {
            return Err("the OSC port must be 1024 or above".to_owned());
        }
        Ok(())
    }
}

/// Managed state: the bound socket (when enabled) + change detection.
#[derive(Default)]
pub struct OscState {
    server: Mutex<Option<OscServer>>,
    seen: Mutex<Option<OscSettings>>,
}

struct OscServer {
    shutdown: Arc<AtomicBool>,
}

impl Drop for OscServer {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
    }
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// One decoded OSC message: its address and the arguments we understand
/// (strings, ints, floats — the only types a control surface sends).
#[derive(Debug, Clone, PartialEq)]
pub struct OscMessage {
    pub address: String,
    pub args: Vec<OscArg>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OscArg {
    Str(String),
    Int(i32),
    Float(f32),
}

impl OscArg {
    fn as_f32(&self) -> Option<f32> {
        match self {
            OscArg::Float(value) => Some(*value),
            OscArg::Int(value) => Some(*value as f32),
            OscArg::Str(_) => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            OscArg::Str(value) => Some(value),
            _ => None,
        }
    }

    /// A control surface sends `1`/`1.0` for "on" and `0` for "off".
    fn as_bool(&self) -> Option<bool> {
        self.as_f32().map(|value| value >= 0.5)
    }
}

/// Read a null-terminated, 4-byte-aligned OSC string.
fn read_string(data: &[u8], at: &mut usize) -> Option<String> {
    let start = *at;
    let end = data[start..].iter().position(|byte| *byte == 0)? + start;
    let text = std::str::from_utf8(&data[start..end]).ok()?.to_owned();
    // Advance past the null and the alignment padding.
    *at = (end + 4) & !3;
    (*at <= data.len()).then_some(text)
}

fn read_i32(data: &[u8], at: &mut usize) -> Option<i32> {
    let bytes: [u8; 4] = data.get(*at..*at + 4)?.try_into().ok()?;
    *at += 4;
    Some(i32::from_be_bytes(bytes))
}

fn read_f32(data: &[u8], at: &mut usize) -> Option<f32> {
    let bytes: [u8; 4] = data.get(*at..*at + 4)?.try_into().ok()?;
    *at += 4;
    Some(f32::from_be_bytes(bytes))
}

/// Parse one OSC 1.0 message. Bundles and unknown tag types are ignored
/// (returning `None`) rather than guessed at.
pub fn parse_message(data: &[u8]) -> Option<OscMessage> {
    if data.is_empty() || data.len() > MAX_PACKET || data[0] != b'/' {
        return None; // not an OSC message (a `#bundle` starts with '#')
    }
    let mut at = 0usize;
    let address = read_string(data, &mut at)?;
    let mut args = Vec::new();
    // The type-tag string is optional in the wild; without it there are no args.
    if let Some(tags) = read_string(data, &mut at) {
        for tag in tags.strip_prefix(',').unwrap_or("").chars() {
            match tag {
                's' => args.push(OscArg::Str(read_string(data, &mut at)?)),
                'i' => args.push(OscArg::Int(read_i32(data, &mut at)?)),
                'f' => args.push(OscArg::Float(read_f32(data, &mut at)?)),
                'T' => args.push(OscArg::Int(1)), // OSC true
                'F' => args.push(OscArg::Int(0)), // OSC false
                _ => return None,                 // an unknown type: don't guess
            }
        }
    }
    Some(OscMessage { address, args })
}

/// Map an OSC message onto the fixed studio-command allowlist (CAP-N04).
/// `None` = the address is not one we serve (silently ignored, like every
/// OSC device does with addresses it doesn't own).
pub fn to_command(message: &OscMessage) -> Option<(String, serde_json::Value)> {
    let arg = |index: usize| message.args.get(index);
    match message.address.as_str() {
        "/scene/switch" => {
            let scene = arg(0)?.as_str()?;
            Some(("setProgramScene".into(), json!({ "scene": scene })))
        }
        "/scene/preview" => {
            let scene = arg(0)?.as_str()?;
            Some(("setPreviewScene".into(), json!({ "scene": scene })))
        }
        "/transition" => Some(("transition".into(), json!({}))),
        "/stream/start" => Some(("startStream".into(), json!({}))),
        "/stream/stop" => Some(("stopStream".into(), json!({}))),
        "/record/start" => Some(("startRecording".into(), json!({}))),
        "/record/stop" => Some(("stopRecording".into(), json!({}))),
        "/record/pause" => {
            let paused = arg(0)?.as_bool()?;
            Some(("pauseRecording".into(), json!({ "paused": paused })))
        }
        "/marker/add" => Some(("addMarker".into(), json!({}))),
        "/replay/save" => Some(("saveReplay".into(), json!({}))),
        "/replay/arm" => {
            let armed = arg(0)?.as_bool()?;
            Some(("armReplay".into(), json!({ "armed": armed })))
        }
        "/macro/run" => {
            let name = arg(0)?.as_str()?;
            Some(("runMacro".into(), json!({ "name": name })))
        }
        // The mixer addresses take the SOURCE NAME (what the operator sees on
        // the strip); the command layer resolves it.
        "/mixer/vol" => {
            let name = arg(0)?.as_str()?;
            let db = arg(1)?.as_f32()?;
            Some((
                "setAudioVolume".into(),
                json!({ "sourceName": name, "volumeDb": db }),
            ))
        }
        "/mixer/mute" => {
            let name = arg(0)?.as_str()?;
            let muted = arg(1)?.as_bool()?;
            Some((
                "setAudioMuted".into(),
                json!({ "sourceName": name, "muted": muted }),
            ))
        }
        _ => None,
    }
}

/// Reconcile the socket against settings (~1 Hz). Cheap no-op when unchanged.
pub fn reconcile(app: &AppHandle) {
    let settings = app.state::<crate::settings::SettingsStore>().get().osc;
    let state = app.state::<OscState>();
    if lock(&state.seen).as_ref() == Some(&settings) {
        return;
    }
    *lock(&state.server) = None; // any change closes the old socket first
    if !settings.enabled {
        *lock(&state.seen) = Some(settings); // off: settled, nothing to retry
        return;
    }
    match start(app.clone(), &settings) {
        Ok(server) => {
            println!(
                "osc: listening on {}:{} (LAN: {})",
                if settings.lan { "0.0.0.0" } else { "127.0.0.1" },
                settings.port,
                settings.lan
            );
            *lock(&state.server) = Some(server);
            // Commit `seen` only AFTER a successful bind. A failed bind leaves
            // `seen` stale so the next tick retries — by which time the old
            // listener thread (200 ms poll) has released the port.
            *lock(&state.seen) = Some(settings);
        }
        Err(err) => eprintln!("osc: could not start (will retry): {err}"),
    }
}

fn start(app: AppHandle, settings: &OscSettings) -> Result<OscServer, String> {
    let host = if settings.lan { "0.0.0.0" } else { "127.0.0.1" };
    let socket = UdpSocket::bind((host, settings.port))
        .map_err(|err| format!("could not bind {host}:{}: {err}", settings.port))?;
    socket
        .set_read_timeout(Some(Duration::from_millis(200)))
        .map_err(|err| err.to_string())?;

    let shutdown = Arc::new(AtomicBool::new(false));
    {
        let shutdown = Arc::clone(&shutdown);
        std::thread::Builder::new()
            .name("fcap-osc".into())
            .spawn(move || {
                let mut buf = vec![0u8; MAX_PACKET];
                while !shutdown.load(Ordering::Relaxed) {
                    let Ok((read, _from)) = socket.recv_from(&mut buf) else {
                        continue; // timeout: re-check the shutdown flag
                    };
                    let Some(message) = parse_message(&buf[..read]) else {
                        continue; // not ours / malformed: ignore quietly
                    };
                    let Some((command, params)) = to_command(&message) else {
                        continue; // an address we don't serve
                    };
                    // Every command re-checks the allowlist inside dispatch.
                    if let Err(err) = crate::remote_api::dispatch_any(&app, &command, &params) {
                        eprintln!("osc: {} failed: {err}", message.address);
                    }
                }
            })
            .map_err(|err| err.to_string())?;
    }
    Ok(OscServer { shutdown })
}

/// Poll settings ~1 Hz and keep the socket in sync.
pub fn spawn_manager(app: AppHandle) {
    std::thread::Builder::new()
        .name("fcap-osc-manager".into())
        .spawn(move || loop {
            reconcile(&app);
            std::thread::sleep(Duration::from_secs(1));
        })
        .expect("osc manager thread spawns");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a real OSC packet the way TouchOSC would.
    fn packet(address: &str, tags: &str, payload: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let pad = |out: &mut Vec<u8>, text: &str| {
            out.extend_from_slice(text.as_bytes());
            out.push(0);
            while out.len() % 4 != 0 {
                out.push(0);
            }
        };
        pad(&mut out, address);
        pad(&mut out, tags);
        out.extend_from_slice(payload);
        out
    }

    #[test]
    fn a_scene_switch_packet_parses_and_maps() {
        let data = packet("/scene/switch", ",s", &{
            let mut arg = b"Live".to_vec();
            arg.push(0);
            while arg.len() % 4 != 0 {
                arg.push(0);
            }
            arg
        });
        let message = parse_message(&data).expect("parses");
        assert_eq!(message.address, "/scene/switch");
        assert_eq!(message.args, vec![OscArg::Str("Live".to_owned())]);

        let (command, params) = to_command(&message).expect("maps");
        assert_eq!(command, "setProgramScene");
        assert_eq!(params["scene"], "Live");
        // …and the mapped command is on the shared allowlist.
        assert!(crate::remote_api::is_allowed_command(&command));
    }

    #[test]
    fn a_fader_packet_carries_its_float() {
        let mut payload = b"Mic\0".to_vec();
        payload.extend_from_slice(&(-6.0f32).to_be_bytes());
        let data = packet("/mixer/vol", ",sf", &payload);
        let message = parse_message(&data).expect("parses");
        let (command, params) = to_command(&message).expect("maps");
        assert_eq!(command, "setAudioVolume");
        assert_eq!(params["sourceName"], "Mic");
        assert!((params["volumeDb"].as_f64().expect("f64") + 6.0).abs() < 1e-6);
    }

    #[test]
    fn every_mapped_address_lands_on_the_allowlist() {
        // The design guarantee: OSC can ask for nothing the app's own buttons
        // can't. Walk every address we serve and assert it maps into the list.
        let cases: Vec<(&str, Vec<OscArg>)> = vec![
            ("/scene/switch", vec![OscArg::Str("A".into())]),
            ("/scene/preview", vec![OscArg::Str("A".into())]),
            ("/transition", vec![]),
            ("/stream/start", vec![]),
            ("/stream/stop", vec![]),
            ("/record/start", vec![]),
            ("/record/stop", vec![]),
            ("/record/pause", vec![OscArg::Int(1)]),
            ("/marker/add", vec![]),
            ("/replay/save", vec![]),
            ("/replay/arm", vec![OscArg::Int(1)]),
            ("/macro/run", vec![OscArg::Str("M".into())]),
            (
                "/mixer/vol",
                vec![OscArg::Str("Mic".into()), OscArg::Float(-3.0)],
            ),
            (
                "/mixer/mute",
                vec![OscArg::Str("Mic".into()), OscArg::Int(1)],
            ),
        ];
        for (address, args) in cases {
            let message = OscMessage {
                address: address.to_owned(),
                args,
            };
            let (command, _) = to_command(&message).unwrap_or_else(|| panic!("{address} maps"));
            assert!(
                crate::remote_api::is_allowed_command(&command),
                "{address} → {command} must be on the allowlist"
            );
        }
    }

    #[test]
    fn junk_and_unknown_addresses_are_ignored_not_guessed() {
        assert!(parse_message(b"").is_none());
        assert!(
            parse_message(b"#bundle\0").is_none(),
            "bundles are not ours"
        );
        assert!(parse_message(&[0xff, 0xfe, 0x00]).is_none());
        // A well-formed message on an address we don't serve maps to nothing.
        let message = OscMessage {
            address: "/some/other/app".to_owned(),
            args: vec![OscArg::Int(1)],
        };
        assert!(to_command(&message).is_none());
        // A known address with the WRONG argument shape is ignored, not
        // half-applied.
        let bad = OscMessage {
            address: "/mixer/vol".to_owned(),
            args: vec![OscArg::Str("Mic".into())], // missing the level
        };
        assert!(to_command(&bad).is_none());
    }

    #[test]
    fn settings_are_off_by_default_and_loopback_first() {
        let settings = OscSettings::default();
        assert!(!settings.enabled, "off by default");
        assert!(!settings.lan, "loopback unless LAN is asked for");
        assert!(settings.validate().is_ok());
        let privileged = OscSettings {
            port: 80,
            ..OscSettings::default()
        };
        assert!(privileged.validate().is_err());
    }
}
