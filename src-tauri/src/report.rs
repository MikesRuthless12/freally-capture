//! CAP-N51 stream session report: on session end, an **optional, local**
//! HTML + Markdown summary written next to the recording — uptime, average
//! and peak bitrate per target, drop percentages, the reconnect log,
//! markers, alarms, and the recording paths. Nothing is uploaded anywhere;
//! sharing it is the user's call alone.
//!
//! Rendering is pure (fixture-testable); only `write_session_report` walks
//! the filesystem.

use std::path::PathBuf;

use tauri::{AppHandle, Manager, Runtime};

use crate::forensic::SessionData;

/// A per-target rollup computed from the session's 1 Hz samples.
struct TargetSummary {
    label: String,
    avg_kbps: u32,
    peak_kbps: u32,
    reconnects: u32,
    frames_dropped: u64,
    /// Share of samples spent NOT publishing ("reconnecting"/"failed"), as
    /// an honest percentage of the session.
    down_pct: f32,
}

fn summarize_targets(session: &SessionData) -> Vec<TargetSummary> {
    let mut ids: Vec<usize> = Vec::new();
    for sample in &session.samples {
        for target in &sample.targets {
            if !ids.contains(&target.id) {
                ids.push(target.id);
            }
        }
    }
    ids.into_iter()
        .filter_map(|id| {
            let mut label = String::new();
            let (mut sum, mut peak, mut n, mut down) = (0u64, 0u32, 0u32, 0u32);
            let (mut reconnects, mut frames_dropped) = (0u32, 0u64);
            for sample in &session.samples {
                if let Some(target) = sample.targets.iter().find(|target| target.id == id) {
                    label = target.label.clone();
                    n += 1;
                    sum += u64::from(target.kbps);
                    peak = peak.max(target.kbps);
                    if target.state != "live" {
                        down += 1;
                    }
                    reconnects = reconnects.max(target.reconnects);
                    frames_dropped = frames_dropped.max(target.frames_dropped);
                }
            }
            (n > 0).then(|| TargetSummary {
                label,
                avg_kbps: (sum / u64::from(n)) as u32,
                peak_kbps: peak,
                reconnects,
                frames_dropped,
                down_pct: down as f32 * 100.0 / n as f32,
            })
        })
        .collect()
}

fn format_duration(ms: u64) -> String {
    let total = ms / 1000;
    format!(
        "{}:{:02}:{:02}",
        total / 3600,
        (total / 60) % 60,
        total % 60
    )
}

/// The Markdown report — the whole truth in a shape a person can diff.
pub fn render_markdown(session: &SessionData) -> String {
    let mut out = String::new();
    let uptime = session
        .ended_t_ms
        .or_else(|| session.samples.last().map(|sample| sample.t_ms))
        .unwrap_or(0);
    out.push_str("# Freally Capture — session report\n\n");
    if session.rehearsal {
        let drill = session
            .simulator
            .as_deref()
            .map(|profile| format!(" (simulator: {profile})"))
            .unwrap_or_default();
        out.push_str(&format!(
            "**REHEARSAL** — this session published to local sinks only; nothing left the machine{drill}.\n\n"
        ));
    }
    out.push_str(&format!("- Uptime: {}\n", format_duration(uptime)));
    let dropped = session.samples.last().map(|s| s.dropped).unwrap_or(0)
        - session.samples.first().map(|s| s.dropped).unwrap_or(0);
    out.push_str(&format!("- Capture frames dropped: {dropped}\n"));
    if session.recording_paths.is_empty() {
        out.push_str("- Recordings: none\n");
    } else {
        out.push_str("- Recordings:\n");
        for path in &session.recording_paths {
            out.push_str(&format!("  - `{path}`\n"));
        }
    }
    out.push('\n');

    let targets = summarize_targets(session);
    if !targets.is_empty() {
        out.push_str("## Targets\n\n");
        out.push_str("| Target | Avg kbps | Peak kbps | Down % | Reconnects | Frames dropped |\n");
        out.push_str("|---|---:|---:|---:|---:|---:|\n");
        for target in &targets {
            out.push_str(&format!(
                "| {} | {} | {} | {:.1}% | {} | {} |\n",
                target.label,
                target.avg_kbps,
                target.peak_kbps,
                target.down_pct,
                target.reconnects,
                target.frames_dropped
            ));
        }
        out.push('\n');
    }

    let mut section = |title: &str, kinds: &[&str]| {
        let rows: Vec<&crate::forensic::ForensicEvent> = session
            .events
            .iter()
            .filter(|event| kinds.contains(&event.kind.as_str()))
            .collect();
        if rows.is_empty() {
            return;
        }
        out.push_str(&format!("## {title}\n\n"));
        for event in rows {
            out.push_str(&format!(
                "- {} — {}\n",
                format_duration(event.t_ms),
                event.label
            ));
        }
        out.push('\n');
    };
    section("Reconnects", &["reconnect", "target"]);
    section("Alarms", &["alarm", "alarm-clear"]);
    section("Markers", &["marker"]);
    section("Scene switches", &["scene"]);
    section("Encoder fallbacks", &["fallback"]);
    out
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// The HTML report: the same content, readable in any browser — dark,
/// dependency-free, self-contained.
pub fn render_html(session: &SessionData) -> String {
    let body = render_markdown(session);
    // A deliberately tiny "renderer": headings, list items and the target
    // table pre-rendered as <pre> keeps this dependency-free and honest.
    format!(
        "<!doctype html>\n<html><head><meta charset=\"utf-8\">\n<title>Freally Capture — session report</title>\n<style>body{{background:#101216;color:#e6e9ef;font:14px/1.5 system-ui,sans-serif;max-width:60rem;margin:2rem auto;padding:0 1rem}}pre{{white-space:pre-wrap;word-break:break-word}}</style>\n</head><body><pre>{}</pre></body></html>\n",
        escape_html(&body)
    )
}

/// Where the report lands: beside the session's first recording when there
/// is one, else in the recordings folder.
fn report_dir<R: Runtime>(app: &AppHandle<R>, session: &SessionData) -> PathBuf {
    session
        .recording_paths
        .first()
        .map(PathBuf::from)
        .and_then(|path| path.parent().map(PathBuf::from))
        .unwrap_or_else(|| {
            let settings = app
                .state::<crate::settings::SettingsStore>()
                .get()
                .recording;
            crate::recording::recordings_folder(&settings)
        })
}

/// Write the `.md` + `.html` pair next to the recording. Failures are
/// logged, never fatal — a report must not be able to break a show's end.
pub fn write_session_report<R: Runtime>(app: &AppHandle<R>, session: &SessionData) {
    let dir = report_dir(app, session);
    // Stamp with the session's wall-clock start so reruns never collide.
    let secs = session.started_unix_ms / 1000;
    let stem = format!("session-report-{secs}");
    let markdown = render_markdown(session);
    let html = render_html(session);
    for (ext, content) in [("md", &markdown), ("html", &html)] {
        let path = dir.join(format!("{stem}.{ext}"));
        match std::fs::write(&path, content) {
            Ok(()) => println!("report: wrote {}", path.display()),
            Err(err) => eprintln!("report: could not write {}: {err}", path.display()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::forensic::{ForensicEvent, ForensicSample, ForensicTarget};

    fn fixture() -> SessionData {
        let target = |kbps: u32, state: &str, reconnects: u32| ForensicTarget {
            id: 0,
            label: "Twitch".to_string(),
            state: state.to_string(),
            kbps,
            reconnects,
            frames_dropped: 12,
        };
        SessionData {
            started_unix_ms: 1_700_000_000_000,
            rehearsal: true,
            simulator: Some("hotelWifi".to_string()),
            samples: vec![
                ForensicSample {
                    t_ms: 0,
                    fps: 60,
                    render_us: 4_000,
                    dropped: 10,
                    frames_behind: 0,
                    targets: vec![target(6_000, "live", 0)],
                },
                ForensicSample {
                    t_ms: 1_000,
                    fps: 60,
                    render_us: 4_100,
                    dropped: 12,
                    frames_behind: 2,
                    targets: vec![target(2_000, "reconnecting", 1)],
                },
                ForensicSample {
                    t_ms: 2_000,
                    fps: 59,
                    render_us: 4_200,
                    dropped: 40,
                    frames_behind: 0,
                    targets: vec![target(4_000, "live", 1)],
                },
            ],
            events: vec![
                ForensicEvent {
                    t_ms: 900,
                    kind: "reconnect".to_string(),
                    label: "Twitch (#1)".to_string(),
                },
                ForensicEvent {
                    t_ms: 1_200,
                    kind: "alarm".to_string(),
                    label: "black frame".to_string(),
                },
                ForensicEvent {
                    t_ms: 1_500,
                    kind: "marker".to_string(),
                    label: "Scene: Intro".to_string(),
                },
            ],
            recording_paths: vec!["D:/Videos/show.mkv".to_string()],
            ended_t_ms: Some(3_000),
        }
    }

    #[test]
    fn markdown_report_carries_every_promised_section() {
        let report = render_markdown(&fixture());
        // Uptime, drops, recordings.
        assert!(report.contains("Uptime: 0:00:03"));
        assert!(report.contains("Capture frames dropped: 30"));
        assert!(report.contains("D:/Videos/show.mkv"));
        // Per-target rollup: avg of 6000/2000/4000 = 4000, peak 6000,
        // down 1 of 3 samples ≈ 33.3%.
        assert!(report.contains("| Twitch | 4000 | 6000 | 33.3% | 1 | 12 |"));
        // Reconnect log, alarms, markers.
        assert!(report.contains("Twitch (#1)"));
        assert!(report.contains("black frame"));
        assert!(report.contains("Scene: Intro"));
        // A rehearsal says so, with the armed drill.
        assert!(report.contains("REHEARSAL"));
        assert!(report.contains("hotelWifi"));
    }

    #[test]
    fn html_report_is_selfcontained_and_escaped() {
        let mut session = fixture();
        session.events.push(ForensicEvent {
            t_ms: 2_500,
            kind: "marker".to_string(),
            label: "<script>alert(1)</script>".to_string(),
        });
        let html = render_html(&session);
        assert!(html.starts_with("<!doctype html>"));
        assert!(!html.contains("<script>alert"), "labels must be escaped");
        assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
        // No external fetches — the CSP-clean, local-only promise.
        assert!(!html.contains("http://") && !html.contains("https://"));
    }
}
