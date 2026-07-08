//! Floating reactions (Phase 6, TASK-614): viewer reaction emoji rise and
//! fade **baked into the program** — recorded AND streamed (unlike
//! preview-only chrome), so a replay shows the exact moment people
//! reacted.
//!
//! Sources: the in-app reaction bar (charter-clean, no external API) and
//! the platform chat riding TASK-613's **no-key** InnerTube/IRC/Kick
//! ingest (emoji spotted in chat spawn reactions — no API key or account,
//! honoring the same hard rule). Robustness: a **bounded pending queue**,
//! a **capped spawn rate with burst coalescing**, and the compositor's
//! hard particle pool — a reaction flood can never stall or break the
//! stream, the recording, or the overlay; it only caps what's on screen.

use std::sync::{Arc, Mutex};

use fcap_compositor::ReactionDraw;

/// The reaction set the bar offers and chat is scanned for. (Variation
/// selectors are stripped before matching/rasterizing.)
pub const REACTION_EMOJI: [&str; 6] = ["❤", "🔥", "💯", "👏", "😂", "🎉"];

/// The sprite tint per emoji — the rasterizer draws system-emoji outlines
/// monochrome (we own no color-emoji renderer; said honestly), so each
/// gets its natural color.
pub fn tint_of(emoji: &str) -> [u8; 4] {
    match emoji {
        "❤" => [235, 64, 82, 255],
        "🔥" => [255, 122, 36, 255],
        "💯" => [235, 64, 52, 255],
        "👏" => [240, 190, 90, 255],
        "😂" => [250, 204, 60, 255],
        "🎉" => [120, 180, 255, 255],
        _ => [255, 255, 255, 255],
    }
}

/// Pending reactions cap — pushes beyond it drop (coalescing by honesty:
/// the screen could never show them anyway).
const QUEUE_CAP: usize = 128;
/// Most particles spawned per tick (bursts coalesce into these).
const SPAWN_PER_TICK: usize = 6;
/// The live particle pool (mirrors the compositor's draw cap).
const POOL: usize = fcap_compositor::REACTION_POOL;

/// Managed state: the bounded pending queue the bar / chat / remote push
/// into; the studio loop drains it into particles.
pub struct ReactionState {
    queue: Arc<Mutex<Vec<String>>>,
}

impl ReactionState {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// The shared handle ingest closures capture (chat threads).
    pub fn queue_handle(&self) -> Arc<Mutex<Vec<String>>> {
        Arc::clone(&self.queue)
    }

    /// Push one reaction (whitelisted; floods drop beyond the cap).
    pub fn push(&self, emoji: &str) -> Result<(), String> {
        push_into(&self.queue, emoji)
    }

    /// Take everything pending (the studio loop's per-tick drain).
    pub fn drain(&self) -> Vec<String> {
        std::mem::take(
            &mut *self
                .queue
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
        )
    }
}

impl Default for ReactionState {
    fn default() -> Self {
        Self::new()
    }
}

/// The shared push (also used by the chat tap without the managed state).
pub fn push_into(queue: &Mutex<Vec<String>>, emoji: &str) -> Result<(), String> {
    let clean: String = emoji.chars().filter(|c| *c != '\u{FE0F}').collect();
    let Some(canonical) = REACTION_EMOJI.iter().find(|known| ***known == clean) else {
        return Err("unknown reaction".to_string());
    };
    let mut queue = queue
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if queue.len() < QUEUE_CAP {
        queue.push((*canonical).to_string());
    }
    Ok(())
}

/// One live particle: everything needed to place it at any `now`.
#[derive(Debug, Clone)]
pub struct Particle {
    pub sprite: String,
    /// Horizontal column, 0..1 of the canvas width.
    pub column: f32,
    pub born_s: f32,
    pub duration_s: f32,
    /// Wobble phase so parallel hearts don't march in lockstep.
    pub phase: f32,
    /// Base sprite size, canvas px.
    pub size: f32,
}

/// A tiny deterministic LCG — no rand dependency for confetti.
pub struct Lcg(pub u64);

impl Lcg {
    fn next_f32(&mut self) -> f32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 40) as f32) / ((1u64 << 24) as f32)
    }
}

/// Drain `pending` into the pool: at most [`SPAWN_PER_TICK`] per call, the
/// pool hard-capped at [`POOL`] — the flood contract.
pub fn spawn(particles: &mut Vec<Particle>, pending: Vec<String>, now_s: f32, rng: &mut Lcg) {
    for sprite in pending.into_iter().take(SPAWN_PER_TICK) {
        if particles.len() >= POOL {
            return; // the pool is the honest ceiling
        }
        particles.push(Particle {
            sprite,
            column: 0.08 + rng.next_f32() * 0.84,
            born_s: now_s,
            duration_s: 2.6 + rng.next_f32() * 1.2,
            phase: rng.next_f32() * std::f32::consts::TAU,
            size: 44.0 + rng.next_f32() * 28.0,
        });
    }
}

/// Advance the pool to `now`: expired particles leave; the rest map to
/// draws — rising, wobbling, fading in over the first 10% and out over
/// the last 30%.
pub fn step(particles: &mut Vec<Particle>, now_s: f32, canvas: (u32, u32)) -> Vec<ReactionDraw> {
    particles.retain(|particle| now_s - particle.born_s < particle.duration_s);
    let (width, height) = (canvas.0 as f32, canvas.1 as f32);
    particles
        .iter()
        .map(|particle| {
            let progress =
                ((now_s - particle.born_s) / particle.duration_s.max(1e-3)).clamp(0.0, 1.0);
            let alpha_in = (progress / 0.10).clamp(0.0, 1.0);
            let alpha_out = ((1.0 - progress) / 0.30).clamp(0.0, 1.0);
            ReactionDraw {
                sprite: particle.sprite.clone(),
                x: width * particle.column
                    + (progress * 5.0 + particle.phase).sin() * width * 0.015,
                y: height * (0.92 - 0.78 * progress),
                size: particle.size * (0.9 + 0.25 * progress),
                alpha: alpha_in.min(alpha_out),
            }
        })
        .collect()
}

/// Emoji spotted in a chat message (max 3) — TASK-613's ingest feeds this.
pub fn reactions_in_chat(text: &str) -> Vec<&'static str> {
    let clean: String = text.chars().filter(|c| *c != '\u{FE0F}').collect();
    let mut found = Vec::new();
    for emoji in REACTION_EMOJI {
        for _ in clean.matches(emoji) {
            if found.len() >= 3 {
                return found;
            }
            found.push(emoji);
        }
    }
    found
}

// -- commands -----------------------------------------------------------------

/// The in-app reaction button (host side; the remote-guest page rides the
/// same surface through the host).
#[tauri::command]
pub fn studio_send_reaction(
    state: tauri::State<'_, ReactionState>,
    emoji: String,
) -> Result<(), String> {
    state.push(&emoji)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pushes_whitelist_and_bound() {
        let state = ReactionState::new();
        state.push("❤️").expect("variation selector strips");
        state.push("🔥").expect("known emoji");
        assert!(state.push("🍕").is_err(), "unknown reactions are refused");
        for _ in 0..500 {
            let _ = state.push("👏");
        }
        assert!(
            state.drain().len() <= QUEUE_CAP,
            "the queue never grows past its cap"
        );
    }

    #[test]
    fn a_flood_caps_the_pool_and_never_grows_past_it() {
        let mut particles = Vec::new();
        let mut rng = Lcg(7);
        for tick in 0..100 {
            let burst: Vec<String> = std::iter::repeat("❤".to_string()).take(50).collect();
            spawn(&mut particles, burst, tick as f32 * 0.016, &mut rng);
            assert!(particles.len() <= POOL, "pool cap holds under flood");
        }
        assert_eq!(particles.len(), POOL);
    }

    #[test]
    fn particles_rise_fade_and_expire() {
        let mut particles = Vec::new();
        let mut rng = Lcg(3);
        spawn(&mut particles, vec!["🔥".to_string()], 10.0, &mut rng);
        let early = step(&mut particles, 10.05, (1920, 1080));
        let later = step(&mut particles, 11.5, (1920, 1080));
        assert_eq!(early.len(), 1);
        assert!(later[0].y < early[0].y, "it rises");
        assert!(early[0].alpha <= 1.0 && later[0].alpha > 0.0);
        // Past its duration it leaves the pool.
        let gone = step(&mut particles, 20.0, (1920, 1080));
        assert!(gone.is_empty());
        assert!(particles.is_empty());
    }

    #[test]
    fn chat_text_maps_to_known_reactions_only() {
        assert_eq!(reactions_in_chat("gg ❤️❤️🔥 wow"), vec!["❤", "❤", "🔥"]);
        assert_eq!(
            reactions_in_chat("❤❤❤❤❤").len(),
            3,
            "per-message coalescing"
        );
        assert!(reactions_in_chat("plain text 🍕").is_empty());
    }
}
