//! CAP-M15 — Timer & clock sources: the pure time math and formatting.
//!
//! The render loop owns the cadence (it re-renders a timer's text only when
//! the displayed string changes); everything here is clock-free math so it
//! unit-tests without waiting. Five modes share the family: wall clock
//! (strftime format + fixed UTC offset), countdown (to a duration or the
//! next wall time), count-up stopwatch, and time since live / recording.

use std::fmt::Write as _;
use std::time::{Duration, Instant};

use chrono::{DateTime, FixedOffset, Local, TimeZone, Timelike};

/// Strftime render that can never panic: chrono's `Display` for an invalid
/// specifier returns `fmt::Error` (which `to_string()` would turn into a
/// panic) — `write!` catches it and the clock falls back to a plain time.
pub fn render_time<Tz: TimeZone>(at: &DateTime<Tz>, format: &str) -> String
where
    Tz::Offset: std::fmt::Display,
{
    let format = if format.trim().is_empty() {
        "%H:%M:%S"
    } else {
        format
    };
    let mut out = String::new();
    if write!(out, "{}", at.format(format)).is_ok() {
        return out;
    }
    at.format("%H:%M:%S").to_string()
}

/// The zone a wall clock renders in: a fixed UTC offset in minutes, or the
/// machine's local offset when `None`.
pub fn clock_zone(utc_offset_min: Option<i32>) -> FixedOffset {
    utc_offset_min
        .and_then(|minutes| FixedOffset::east_opt(minutes.clamp(-14 * 60, 14 * 60) * 60))
        .unwrap_or_else(|| *Local::now().offset())
}

/// `H:MM:SS` above an hour, `M:SS` below — the broadcast-timer shape.
pub fn format_hms(duration: Duration) -> String {
    let total = duration.as_secs();
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    let seconds = total % 60;
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

/// Parse a `"HH:MM"` wall-clock countdown target (24-hour).
pub fn parse_wall_target(text: &str) -> Option<(u32, u32)> {
    let (hours, minutes) = text.trim().split_once(':')?;
    let hours: u32 = hours.parse().ok()?;
    let minutes: u32 = minutes.parse().ok()?;
    (hours < 24 && minutes < 60).then_some((hours, minutes))
}

/// Whether `now` sits inside the wall target's exact first second — the
/// countdown's fire moment (the render loop's fire-once guard debounces the
/// second's worth of ticks).
pub fn wall_target_hit<Tz: TimeZone>(now: &DateTime<Tz>, hours: u32, minutes: u32) -> bool {
    now.hour() == hours && now.minute() == minutes && now.second() == 0
}

/// Milliseconds until the NEXT occurrence of `HH:MM` local time (today if
/// still ahead, else tomorrow).
pub fn remaining_to_wall<Tz: TimeZone>(now: &DateTime<Tz>, hours: u32, minutes: u32) -> Duration {
    let seconds_today =
        i64::from(now.hour()) * 3600 + i64::from(now.minute()) * 60 + i64::from(now.second());
    let target = i64::from(hours) * 3600 + i64::from(minutes) * 60;
    let mut delta = target - seconds_today;
    if delta <= 0 {
        delta += 24 * 3600;
    }
    Duration::from_secs(delta as u64)
}

/// One countdown/stopwatch's run state (command → render-loop; not
/// persisted — a relaunch resets timers, honestly). End-of-countdown
/// fire-once tracking is the render loop's own (loop-local), not run state.
#[derive(Debug, Clone, Copy, Default)]
pub struct TimerRun {
    running: bool,
    accumulated: Duration,
    started_at: Option<Instant>,
    /// Bumped on every reset. The render loop watches this to re-arm a
    /// latched wall countdown (and clear its flash) — a reset never bumps
    /// the model revision, so this is how the loop learns about it.
    resets: u32,
}

impl TimerRun {
    /// Elapsed run time as of `now`.
    pub fn elapsed(&self, now: Instant) -> Duration {
        let live = self
            .started_at
            .map(|at| now.saturating_duration_since(at))
            .unwrap_or_default();
        self.accumulated + if self.running { live } else { Duration::ZERO }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn start(&mut self, now: Instant) {
        if !self.running {
            self.running = true;
            self.started_at = Some(now);
        }
    }

    pub fn pause(&mut self, now: Instant) {
        if self.running {
            self.accumulated = self.elapsed(now);
            self.running = false;
            self.started_at = None;
        }
    }

    pub fn toggle(&mut self, now: Instant) {
        if self.running {
            self.pause(now);
        } else {
            self.start(now);
        }
    }

    pub fn reset(&mut self) {
        let resets = self.resets.wrapping_add(1);
        *self = TimerRun {
            resets,
            ..TimerRun::default()
        };
    }

    /// How many times this timer has been reset — the render loop's re-arm
    /// signal (see [`TimerRun::reset`]).
    pub fn resets(&self) -> u32 {
        self.resets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strftime_renders_and_bad_patterns_fall_back() {
        let at = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2026, 7, 12, 21, 5, 9)
            .unwrap();
        assert_eq!(render_time(&at, "%H:%M:%S"), "21:05:09");
        assert_eq!(render_time(&at, "%H.%M"), "21.05");
        assert_eq!(render_time(&at, ""), "21:05:09");
        // `%Q` is not a chrono specifier — must not panic, must show a time.
        assert_eq!(render_time(&at, "%Q"), "21:05:09");
    }

    #[test]
    fn fixed_offsets_shift_the_clock() {
        let utc = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2026, 7, 12, 21, 0, 0)
            .unwrap();
        let tokyo = utc.with_timezone(&clock_zone(Some(9 * 60)));
        assert_eq!(render_time(&tokyo, "%H:%M"), "06:00");
        // Out-of-range offsets clamp instead of erroring.
        let clamped = utc.with_timezone(&clock_zone(Some(99_999)));
        assert_eq!(render_time(&clamped, "%H:%M"), "11:00");
    }

    #[test]
    fn hms_is_broadcast_shaped() {
        assert_eq!(format_hms(Duration::from_secs(5)), "0:05");
        assert_eq!(format_hms(Duration::from_secs(90)), "1:30");
        assert_eq!(format_hms(Duration::from_secs(3_600)), "1:00:00");
        assert_eq!(format_hms(Duration::from_secs(7_265)), "2:01:05");
    }

    #[test]
    fn wall_targets_parse_strictly() {
        assert_eq!(parse_wall_target("19:30"), Some((19, 30)));
        assert_eq!(parse_wall_target(" 07:05 "), Some((7, 5)));
        assert_eq!(parse_wall_target("24:00"), None);
        assert_eq!(parse_wall_target("12:60"), None);
        assert_eq!(parse_wall_target("noon"), None);
        assert_eq!(parse_wall_target(""), None);
    }

    #[test]
    fn the_fire_moment_is_the_target_second_only() {
        let zone = FixedOffset::east_opt(0).unwrap();
        let hit = zone.with_ymd_and_hms(2026, 7, 12, 19, 30, 0).unwrap();
        assert!(wall_target_hit(&hit, 19, 30));
        let late = zone.with_ymd_and_hms(2026, 7, 12, 19, 30, 1).unwrap();
        assert!(!wall_target_hit(&late, 19, 30));
        let early = zone.with_ymd_and_hms(2026, 7, 12, 19, 29, 59).unwrap();
        assert!(!wall_target_hit(&early, 19, 30));
    }

    #[test]
    fn wall_countdown_targets_the_next_occurrence() {
        let at = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2026, 7, 12, 21, 0, 0)
            .unwrap();
        assert_eq!(
            remaining_to_wall(&at, 21, 30),
            Duration::from_secs(30 * 60),
            "later today"
        );
        assert_eq!(
            remaining_to_wall(&at, 20, 0),
            Duration::from_secs(23 * 3600),
            "already passed → tomorrow"
        );
        assert_eq!(
            remaining_to_wall(&at, 21, 0),
            Duration::from_secs(24 * 3600),
            "exactly now → the next one"
        );
    }

    #[test]
    fn a_timer_run_accumulates_across_pauses() {
        let t0 = Instant::now();
        let mut run = TimerRun::default();
        assert_eq!(run.elapsed(t0), Duration::ZERO);

        run.start(t0);
        let t1 = t0 + Duration::from_secs(10);
        assert_eq!(run.elapsed(t1), Duration::from_secs(10));

        run.pause(t1);
        let t2 = t1 + Duration::from_secs(100);
        assert_eq!(run.elapsed(t2), Duration::from_secs(10), "paused holds");

        run.toggle(t2); // resume
        let t3 = t2 + Duration::from_secs(5);
        assert_eq!(run.elapsed(t3), Duration::from_secs(15));

        run.reset();
        assert_eq!(run.elapsed(t3), Duration::ZERO);
        assert!(!run.is_running());
        assert_eq!(run.resets(), 1, "a reset is visible to the render loop");
        run.reset();
        assert_eq!(run.resets(), 2);
    }

    #[test]
    fn double_start_does_not_rewind() {
        let t0 = Instant::now();
        let mut run = TimerRun::default();
        run.start(t0);
        run.start(t0 + Duration::from_secs(5)); // no-op while running
        assert_eq!(
            run.elapsed(t0 + Duration::from_secs(8)),
            Duration::from_secs(8)
        );
    }
}
