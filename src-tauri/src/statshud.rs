//! CAP-N14: the system-stats overlay's face text.
//!
//! Pure formatting — the render loop feeds it the measured numbers and
//! repaints only when the resulting text changes. Labels are terse and
//! language-neutral: like the timer faces, this is program output for
//! viewers, not UI chrome, so it is not localized.

/// The numbers one HUD face can show. Everything here is measured — the
/// HUD never guesses (which is why there is no GPU% field).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct StatsNumbers {
    /// Composed frames per second (the program render rate).
    pub fps: u32,
    /// This process's CPU usage, percent of the whole machine.
    pub cpu_percent: f32,
    /// This process's resident memory, MiB.
    pub memory_mb: f32,
    /// Mean GPU compose time per frame, milliseconds.
    pub render_ms: f32,
    /// Frames the capture pipeline dropped since the session began.
    pub dropped: u64,
    /// Live publish bitrate summed across targets; `None` = not streaming.
    pub bitrate_kbps: Option<u32>,
}

/// Which lines the face shows (mirrors the source's `show_*` settings).
#[derive(Debug, Clone, Copy)]
pub struct HudToggles {
    pub fps: bool,
    pub cpu: bool,
    pub memory: bool,
    pub render_ms: bool,
    pub dropped: bool,
    pub bitrate: bool,
}

/// One line per enabled stat, rounded so the text (and therefore the
/// repaint) only moves at the samplers' cadence. All lines off renders a
/// single dash — a face must never be zero-sized.
pub fn format_hud(numbers: &StatsNumbers, show: &HudToggles) -> String {
    let mut lines: Vec<String> = Vec::new();
    if show.fps {
        lines.push(format!("FPS {}", numbers.fps));
    }
    if show.cpu {
        lines.push(format!("CPU {:.0}%", numbers.cpu_percent));
    }
    if show.memory {
        lines.push(format!("MEM {:.0} MB", numbers.memory_mb));
    }
    if show.render_ms {
        lines.push(format!("RENDER {:.1} ms", numbers.render_ms));
    }
    if show.dropped {
        lines.push(format!("DROPPED {}", numbers.dropped));
    }
    if show.bitrate {
        match numbers.bitrate_kbps {
            Some(kbps) => lines.push(format!("BITRATE {kbps} kbps")),
            None => lines.push("BITRATE —".to_string()),
        }
    }
    if lines.is_empty() {
        return "—".to_string();
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: HudToggles = HudToggles {
        fps: true,
        cpu: true,
        memory: true,
        render_ms: true,
        dropped: true,
        bitrate: true,
    };

    fn numbers() -> StatsNumbers {
        StatsNumbers {
            fps: 60,
            cpu_percent: 12.4,
            memory_mb: 541.6,
            render_ms: 3.13,
            dropped: 2,
            bitrate_kbps: Some(6000),
        }
    }

    #[test]
    fn every_line_renders_rounded() {
        let face = format_hud(&numbers(), &ALL);
        assert_eq!(
            face,
            "FPS 60\nCPU 12%\nMEM 542 MB\nRENDER 3.1 ms\nDROPPED 2\nBITRATE 6000 kbps"
        );
    }

    #[test]
    fn off_air_bitrate_shows_a_dash_not_a_zero() {
        let mut n = numbers();
        n.bitrate_kbps = None;
        let face = format_hud(
            &n,
            &HudToggles {
                fps: false,
                cpu: false,
                memory: false,
                render_ms: false,
                dropped: false,
                bitrate: true,
            },
        );
        assert_eq!(face, "BITRATE —");
    }

    #[test]
    fn subset_keeps_only_enabled_lines() {
        let face = format_hud(
            &numbers(),
            &HudToggles {
                fps: true,
                cpu: false,
                memory: false,
                render_ms: false,
                dropped: true,
                bitrate: false,
            },
        );
        assert_eq!(face, "FPS 60\nDROPPED 2");
    }

    #[test]
    fn all_lines_off_never_yields_an_empty_face() {
        let face = format_hud(
            &numbers(),
            &HudToggles {
                fps: false,
                cpu: false,
                memory: false,
                render_ms: false,
                dropped: false,
                bitrate: false,
            },
        );
        assert_eq!(face, "—");
    }

    #[test]
    fn rounding_pins_the_repaint_cadence_to_the_samplers() {
        // Sub-percent CPU jitter and sub-0.05 ms render jitter must not
        // change the text — the face repaints only on visible movement.
        let a = format_hud(&numbers(), &ALL);
        let mut wiggled = numbers();
        wiggled.cpu_percent = 12.1;
        wiggled.render_ms = 3.12;
        let b = format_hud(&wiggled, &ALL);
        assert_eq!(a, b);
    }
}
