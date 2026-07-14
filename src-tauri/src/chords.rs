//! Hotkey chords & layers (CAP-N05): two-stroke leader sequences
//! (`Ctrl+K, 3`) and sticky layers, so a small keyboard drives many actions
//! without collisions.
//!
//! **Chords.** A chord accelerator is `"<leader>, <follower>"`. Only the
//! **leader** is globally registered at rest — the follower is a bare key
//! (`3`), and permanently claiming that from every other app would be
//! hostile. Pressing the leader arms the chord: the follower set is
//! registered for a short window, the next matching stroke runs its macro,
//! and the followers are released again (on match, on any other stroke, or
//! on timeout). So a bare key is only ever claimed for the ~1.5 s a chord is
//! actually pending.
//!
//! **Layers.** A macro may be scoped to a layer; only macros on the *active*
//! layer fire. Layer 0 is the base layer. **Honest scope:** a true
//! *hold*-to-shift layer is not reachable through the OS global-shortcut API
//! (it reports presses of registered accelerators, not raw key state), so
//! layers here are **sticky**: a layer key switches the active layer, and it
//! stays until switched back. The UI says exactly that instead of pretending.

use std::time::{Duration, Instant};

/// How long a chord stays armed after its leader (then the followers are
/// released — a half-typed chord never leaves a bare key claimed).
pub const CHORD_WINDOW: Duration = Duration::from_millis(1_500);

/// One parsed chord accelerator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    /// The globally-registered first stroke (e.g. `Ctrl+K`).
    pub leader: String,
    /// The second stroke (e.g. `3`) — registered only while armed.
    pub follower: String,
}

/// Parse `"Ctrl+K, 3"` into its two strokes. `None` when the text is a plain
/// (single-stroke) accelerator, or malformed — a malformed chord is never
/// silently treated as a plain key.
pub fn parse_chord(text: &str) -> Option<Chord> {
    let (leader, follower) = text.split_once(',')?;
    let leader = leader.trim();
    let follower = follower.trim();
    if leader.is_empty() || follower.is_empty() || follower.contains(',') {
        return None;
    }
    Some(Chord {
        leader: leader.to_owned(),
        follower: follower.to_owned(),
    })
}

/// Whether `text` looks like a chord at all (used to route registration).
pub fn is_chord(text: &str) -> bool {
    text.contains(',')
}

/// The chord/layer runtime state.
#[derive(Debug, Default)]
pub struct ChordState {
    /// The leader that is currently armed, and when it was pressed.
    armed: Option<(String, Instant)>,
    /// The active layer (0 = base).
    layer: u8,
}

impl ChordState {
    /// Arm a chord: the follower set becomes live for [`CHORD_WINDOW`].
    pub fn arm(&mut self, leader: &str, now: Instant) {
        self.armed = Some((leader.to_owned(), now));
    }

    /// The armed leader, if the window has not expired.
    pub fn armed_leader(&mut self, now: Instant) -> Option<String> {
        match &self.armed {
            Some((leader, at)) if now.duration_since(*at) <= CHORD_WINDOW => Some(leader.clone()),
            Some(_) => {
                self.armed = None; // expired: release the followers
                None
            }
            None => None,
        }
    }

    /// Disarm (a follower matched, a stray stroke arrived, or we timed out).
    pub fn disarm(&mut self) {
        self.armed = None;
    }

    /// Whether a chord is currently armed (the loop's cheap check).
    pub fn is_armed(&mut self, now: Instant) -> bool {
        self.armed_leader(now).is_some()
    }

    pub fn layer(&self) -> u8 {
        self.layer
    }

    /// Switch the active layer (sticky — see the module docs).
    pub fn set_layer(&mut self, layer: u8) {
        self.layer = layer;
    }

    /// Whether a binding on `layer` may fire right now. A binding with no
    /// layer (`None`) is global: it fires on every layer, which is what an
    /// operator expects of Record or the panic key.
    pub fn fires_on_layer(&self, layer: Option<u8>) -> bool {
        match layer {
            None => true,
            Some(target) => target == self.layer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chords_parse_and_plain_keys_do_not() {
        assert_eq!(
            parse_chord("Ctrl+K, 3"),
            Some(Chord {
                leader: "Ctrl+K".to_owned(),
                follower: "3".to_owned(),
            })
        );
        // Whitespace is forgiving.
        assert_eq!(
            parse_chord("Ctrl+K,3").map(|chord| chord.follower),
            Some("3".to_owned())
        );
        // A plain accelerator is not a chord.
        assert_eq!(parse_chord("Ctrl+Shift+R"), None);
        assert!(!is_chord("Ctrl+Shift+R"));
        assert!(is_chord("Ctrl+K, 3"));
        // Malformed chords are rejected, never silently mangled.
        assert_eq!(parse_chord("Ctrl+K,"), None);
        assert_eq!(parse_chord(", 3"), None);
        assert_eq!(parse_chord("A, B, C"), None, "only two strokes");
    }

    #[test]
    fn an_armed_chord_expires_and_releases_its_followers() {
        let mut state = ChordState::default();
        let start = Instant::now();
        assert!(!state.is_armed(start), "nothing armed at rest");

        state.arm("Ctrl+K", start);
        assert_eq!(state.armed_leader(start).as_deref(), Some("Ctrl+K"));
        // Still armed just inside the window…
        let inside = start + CHORD_WINDOW - Duration::from_millis(1);
        assert!(state.is_armed(inside));
        // …and released once it lapses (so a bare `3` stops being claimed).
        let outside = start + CHORD_WINDOW + Duration::from_millis(1);
        assert!(!state.is_armed(outside));
        assert!(state.armed_leader(outside).is_none());

        // A matched follower disarms immediately.
        state.arm("Ctrl+K", outside);
        state.disarm();
        assert!(!state.is_armed(outside));
    }

    #[test]
    fn layers_gate_bindings_and_unlayered_keys_always_fire() {
        let mut state = ChordState::default();
        assert_eq!(state.layer(), 0, "the base layer is 0");
        // An unlayered binding (Record, Panic…) fires on every layer.
        assert!(state.fires_on_layer(None));
        // A layered binding fires only on its own layer.
        assert!(state.fires_on_layer(Some(0)));
        assert!(!state.fires_on_layer(Some(1)));

        state.set_layer(1);
        assert_eq!(state.layer(), 1);
        assert!(state.fires_on_layer(None), "global keys still fire");
        assert!(state.fires_on_layer(Some(1)));
        assert!(!state.fires_on_layer(Some(0)));
    }
}
