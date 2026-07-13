//! PTZ camera control (CAP-N08): pan/tilt/zoom over **VISCA-over-IP**, with
//! named presets and per-scene auto-recall — **LAN-only, off by default,
//! never the internet**.
//!
//! The app talks only to the camera IPs the user types in. VISCA is a tiny
//! byte protocol (`0x81 … 0xFF`), so it is encoded here directly — no new
//! dependency, and the whole wire surface is the handful of commands below.
//!
//! Honest scope: this speaks the standard VISCA-over-IP command set that PTZ
//! cameras from the major vendors accept (pan/tilt drive, stop, zoom, preset
//! store/recall). It does **not** try to enumerate a camera's proprietary
//! extras, and it never auto-discovers: a camera exists here only because the
//! operator entered its address.

use std::net::UdpSocket;
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// The VISCA-over-IP port every vendor ships with.
pub const DEFAULT_VISCA_PORT: u16 = 52381;
/// Camera replies are short; we never wait long for one.
const REPLY_TIMEOUT: Duration = Duration::from_millis(400);
/// The most cameras one profile may hold.
pub const MAX_CAMERAS: usize = 16;
/// The most presets one camera may hold (VISCA's own limit is 0–254).
pub const MAX_PRESETS: usize = 32;

/// One PTZ camera the operator entered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtzCamera {
    pub name: String,
    /// The camera's IP or hostname (typed by the user — never discovered).
    pub host: String,
    #[serde(default = "default_visca_port")]
    pub port: u16,
    /// Named presets: what the operator calls the shot, and the VISCA slot.
    #[serde(default)]
    pub presets: Vec<PtzPreset>,
    /// Recall this preset when a scene becomes the program (by scene name).
    /// Empty = no auto-recall.
    #[serde(default)]
    pub scene_recalls: Vec<SceneRecall>,
}

fn default_visca_port() -> u16 {
    DEFAULT_VISCA_PORT
}

/// One named preset (a VISCA memory slot).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtzPreset {
    pub name: String,
    /// VISCA preset slot (0–254).
    pub slot: u8,
}

/// "When scene X goes on program, recall preset slot N."
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneRecall {
    pub scene: String,
    pub slot: u8,
}

/// The persisted PTZ config (CAP-N08). Empty by default — no camera exists
/// until the operator adds one.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PtzSettings {
    pub cameras: Vec<PtzCamera>,
}

impl PtzSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.cameras.len() > MAX_CAMERAS {
            return Err(format!("too many PTZ cameras ({MAX_CAMERAS} max)"));
        }
        for camera in &self.cameras {
            if camera.name.trim().is_empty() || camera.name.len() > 64 {
                return Err("a PTZ camera needs a name (64 chars max)".to_owned());
            }
            if camera.host.trim().is_empty() || camera.host.len() > 255 {
                return Err("a PTZ camera needs an address".to_owned());
            }
            if camera.host.chars().any(char::is_control) {
                return Err("invalid PTZ camera address".to_owned());
            }
            if camera.port < 1024 {
                return Err("the VISCA port must be 1024 or above".to_owned());
            }
            if camera.presets.len() > MAX_PRESETS {
                return Err(format!("too many presets ({MAX_PRESETS} max)"));
            }
            for preset in &camera.presets {
                if preset.name.len() > 64 {
                    return Err("a preset name is too long".to_owned());
                }
                if preset.slot > 254 {
                    return Err("a VISCA preset slot is 0–254".to_owned());
                }
            }
            for recall in &camera.scene_recalls {
                if recall.scene.len() > 64 {
                    return Err("a scene name is too long".to_owned());
                }
            }
        }
        Ok(())
    }
}

/// Which way to drive the head.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PtzMove {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Stop,
}

/// The VISCA pan/tilt drive payload (speeds clamp to the protocol's range:
/// pan 0x01–0x18, tilt 0x01–0x14).
pub fn visca_move(direction: PtzMove, pan_speed: u8, tilt_speed: u8) -> Vec<u8> {
    let pan = pan_speed.clamp(1, 0x18);
    let tilt = tilt_speed.clamp(1, 0x14);
    let (a, b) = match direction {
        PtzMove::Up => (0x03, 0x01),
        PtzMove::Down => (0x03, 0x02),
        PtzMove::Left => (0x01, 0x03),
        PtzMove::Right => (0x02, 0x03),
        PtzMove::UpLeft => (0x01, 0x01),
        PtzMove::UpRight => (0x02, 0x01),
        PtzMove::DownLeft => (0x01, 0x02),
        PtzMove::DownRight => (0x02, 0x02),
        PtzMove::Stop => (0x03, 0x03),
    };
    vec![0x81, 0x01, 0x06, 0x01, pan, tilt, a, b, 0xFF]
}

/// The VISCA zoom payload. `speed` 0 stops; positive zooms in (tele),
/// negative zooms out (wide); magnitude 1–7.
pub fn visca_zoom(speed: i8) -> Vec<u8> {
    let payload = match speed {
        0 => 0x00,                                    // stop
        s if s > 0 => 0x20 | (s.min(7) as u8 & 0x07), // tele, variable
        s => 0x30 | ((-s).min(7) as u8 & 0x07),       // wide, variable
    };
    vec![0x81, 0x01, 0x04, 0x07, payload, 0xFF]
}

/// The VISCA preset recall payload.
pub fn visca_preset_recall(slot: u8) -> Vec<u8> {
    vec![0x81, 0x01, 0x04, 0x3F, 0x02, slot.min(254), 0xFF]
}

/// The VISCA preset store payload.
pub fn visca_preset_store(slot: u8) -> Vec<u8> {
    vec![0x81, 0x01, 0x04, 0x3F, 0x01, slot.min(254), 0xFF]
}

/// Wrap a VISCA payload in the VISCA-over-IP header (payload type 0x0100,
/// the length, and a rolling sequence number).
fn visca_ip_frame(payload: &[u8], sequence: u32) -> Vec<u8> {
    let mut frame = Vec::with_capacity(8 + payload.len());
    frame.extend_from_slice(&[0x01, 0x00]); // payload type: VISCA command
    frame.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    frame.extend_from_slice(&sequence.to_be_bytes());
    frame.extend_from_slice(payload);
    frame
}

/// Send one VISCA command to a camera. Fire-and-forget with a short wait for
/// the ack: a PTZ head that is slow to answer must never stall the studio.
pub fn send(host: &str, port: u16, payload: &[u8], sequence: u32) -> Result<(), String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|err| err.to_string())?;
    socket
        .set_read_timeout(Some(REPLY_TIMEOUT))
        .map_err(|err| err.to_string())?;
    let frame = visca_ip_frame(payload, sequence);
    socket
        .send_to(&frame, (host, port))
        .map_err(|err| format!("could not reach {host}:{port}: {err}"))?;
    // The ack is optional for us — read it if it comes, ignore a timeout.
    let mut buf = [0u8; 32];
    let _ = socket.recv_from(&mut buf);
    Ok(())
}

/// Per-scene preset auto-recall (CAP-N08): the program scene changed — recall
/// every camera preset bound to the new scene. Called from the studio loop's
/// 1 Hz tick; a no-op when no camera has a scene binding.
pub fn on_scene<R: tauri::Runtime>(app: &tauri::AppHandle<R>, scene: &str) {
    use tauri::Manager;
    let settings = app.state::<crate::settings::SettingsStore>().get().ptz;
    for camera in settings.cameras {
        let Some(recall) = camera
            .scene_recalls
            .iter()
            .find(|recall| recall.scene == scene)
        else {
            continue;
        };
        let payload = visca_preset_recall(recall.slot);
        let (host, port) = (camera.host.clone(), camera.port);
        let name = camera.name.clone();
        // Off the loop thread: a slow head must never stall the studio.
        std::thread::spawn(move || {
            if let Err(err) = send(&host, port, &payload, 0) {
                eprintln!("ptz: {name}: preset recall failed: {err}");
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visca_move_payloads_match_the_protocol() {
        // The canonical "pan left at speed 8, tilt speed 8" command.
        assert_eq!(
            visca_move(PtzMove::Left, 8, 8),
            vec![0x81, 0x01, 0x06, 0x01, 0x08, 0x08, 0x01, 0x03, 0xFF]
        );
        // Stop is the same command with both axes at 0x03.
        assert_eq!(
            visca_move(PtzMove::Stop, 8, 8)[6..8],
            [0x03, 0x03],
            "stop drives neither axis"
        );
        // Speeds clamp into VISCA's ranges rather than corrupting the frame.
        let fast = visca_move(PtzMove::Right, 0xFF, 0xFF);
        assert_eq!(fast[4], 0x18, "pan speed clamps to 0x18");
        assert_eq!(fast[5], 0x14, "tilt speed clamps to 0x14");
        let slow = visca_move(PtzMove::Right, 0, 0);
        assert_eq!((slow[4], slow[5]), (1, 1), "zero clamps up to 1");
        // Every payload is a well-formed VISCA frame.
        for direction in [
            PtzMove::Up,
            PtzMove::Down,
            PtzMove::Left,
            PtzMove::Right,
            PtzMove::UpLeft,
            PtzMove::UpRight,
            PtzMove::DownLeft,
            PtzMove::DownRight,
            PtzMove::Stop,
        ] {
            let payload = visca_move(direction, 5, 5);
            assert_eq!(payload[0], 0x81, "address byte");
            assert_eq!(*payload.last().expect("non-empty"), 0xFF, "terminator");
        }
    }

    #[test]
    fn zoom_and_presets_encode_correctly() {
        assert_eq!(visca_zoom(0), vec![0x81, 0x01, 0x04, 0x07, 0x00, 0xFF]);
        assert_eq!(visca_zoom(3)[4], 0x23, "tele at speed 3");
        assert_eq!(visca_zoom(-3)[4], 0x33, "wide at speed 3");
        assert_eq!(visca_zoom(99)[4], 0x27, "speed clamps to 7");
        assert_eq!(visca_zoom(-99)[4], 0x37);

        assert_eq!(
            visca_preset_recall(4),
            vec![0x81, 0x01, 0x04, 0x3F, 0x02, 0x04, 0xFF]
        );
        assert_eq!(
            visca_preset_store(4),
            vec![0x81, 0x01, 0x04, 0x3F, 0x01, 0x04, 0xFF]
        );
        assert_eq!(visca_preset_recall(255)[5], 254, "slot clamps to 254");
    }

    #[test]
    fn the_ip_frame_carries_type_length_and_sequence() {
        let payload = visca_zoom(0);
        let frame = visca_ip_frame(&payload, 7);
        assert_eq!(&frame[0..2], &[0x01, 0x00], "VISCA command payload type");
        assert_eq!(
            u16::from_be_bytes([frame[2], frame[3]]) as usize,
            payload.len(),
            "the length field matches the payload"
        );
        assert_eq!(
            u32::from_be_bytes([frame[4], frame[5], frame[6], frame[7]]),
            7
        );
        assert_eq!(&frame[8..], &payload[..]);
    }

    #[test]
    fn settings_are_empty_by_default_and_bounded() {
        let settings = PtzSettings::default();
        assert!(
            settings.cameras.is_empty(),
            "no camera until one is entered"
        );
        assert!(settings.validate().is_ok());

        let nameless = PtzSettings {
            cameras: vec![PtzCamera {
                name: String::new(),
                host: "192.168.1.50".to_owned(),
                port: DEFAULT_VISCA_PORT,
                presets: Vec::new(),
                scene_recalls: Vec::new(),
            }],
        };
        assert!(nameless.validate().is_err());

        let hostless = PtzSettings {
            cameras: vec![PtzCamera {
                name: "Cam".to_owned(),
                host: "  ".to_owned(),
                port: DEFAULT_VISCA_PORT,
                presets: Vec::new(),
                scene_recalls: Vec::new(),
            }],
        };
        assert!(hostless.validate().is_err(), "a camera needs an address");

        let ok = PtzSettings {
            cameras: vec![PtzCamera {
                name: "Wide".to_owned(),
                host: "192.168.1.50".to_owned(),
                port: DEFAULT_VISCA_PORT,
                presets: vec![PtzPreset {
                    name: "Two-shot".to_owned(),
                    slot: 3,
                }],
                scene_recalls: vec![SceneRecall {
                    scene: "Interview".to_owned(),
                    slot: 3,
                }],
            }],
        };
        assert!(ok.validate().is_ok());
    }
}
