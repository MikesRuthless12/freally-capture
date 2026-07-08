//! Opt-in, anonymous bug reporting — **charter-clean: no telemetry, nothing
//! auto-sends, no server we run, no credentials shipped.**
//!
//! A panic hook captures a **scrubbed** crash report to a local file; on the
//! next launch the UI offers to report it. The "Report a bug" dialog shows the
//! user the **exact** anonymous report and lets them submit it via a
//! pre-filled **GitHub issue** or their **email client** — both explicit
//! clicks. Diagnostics carry the app version + OS/arch and (optionally) a
//! crash excerpt with the home path + username redacted; never file contents,
//! stream keys, or personal data.

use std::path::PathBuf;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Runtime};

/// This app's name — put in the subject line + body so a report that lands in
/// the shared inbox is instantly attributable to the right Havoc app.
const APP_NAME: &str = "Freally Capture";
/// The project's issue tracker (a pre-filled URL the user submits — no token).
const GITHUB_NEW_ISSUE: &str = "https://github.com/MikesRuthless12/freally-capture/issues/new";
/// Where an emailed report goes (the user's own mail client sends it).
const REPORT_EMAIL: &str = "mythodikalone@gmail.com";
/// Keep pre-filled URLs under browser/mailer length limits — a long report
/// gets its tail trimmed in the URL; "Copy report" carries the full text.
const MAX_GITHUB_BODY: usize = 6000;
const MAX_MAILTO_BODY: usize = 1800;

fn crash_dir() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "Freally", "Freally Capture")
        .map(|dirs| dirs.data_dir().join("crash-reports"))
}

/// Redact the OS user's home path + username from `text` so a report carries
/// no personal identifiers. The report is always shown to the user before it
/// can be sent, so over-redaction is safe and under-redaction is visible.
pub fn scrub(text: &str) -> String {
    let mut out = text.to_string();
    if let Some(dirs) = directories::UserDirs::new() {
        let home = dirs.home_dir().to_string_lossy().to_string();
        if !home.is_empty() {
            out = out.replace(&home, "<home>");
            // Also the bare username, if it's not a trivially-short substring.
            if let Some(name) = std::path::Path::new(&home)
                .file_name()
                .and_then(|n| n.to_str())
            {
                if name.len() >= 3 {
                    out = out.replace(name, "<user>");
                }
            }
        }
    }
    out
}

/// The always-anonymous system line (no personal data).
pub fn diagnostics() -> String {
    format!(
        "App: Freally Capture {}\nOS: {} / {}\n",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH,
    )
}

/// Install the panic hook (call once in setup): a panic writes a scrubbed
/// crash report to the local crash-reports dir, then the previous hook runs.
pub fn install_panic_hook() {
    let previous = std::panic::take_hook();
    // The closure's `info` type is inferred (not named) so this compiles on the
    // 1.80 MSRV — the `PanicHookInfo` type name only stabilized in 1.81.
    std::panic::set_hook(Box::new(move |info| {
        let location = info
            .location()
            .map(|loc| format!("{}:{}", loc.file(), loc.line()))
            .unwrap_or_else(|| "unknown".to_string());
        let message = info
            .payload()
            .downcast_ref::<&str>()
            .map(|s| (*s).to_string())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "(no message)".to_string());
        let backtrace = std::backtrace::Backtrace::force_capture();
        let raw = format!("Panic at {location}\nMessage: {message}\n\nBacktrace:\n{backtrace}\n");
        write_crash(&scrub(&raw));
        previous(info);
    }));
}

fn write_crash(scrubbed: &str) {
    let Some(dir) = crash_dir() else { return };
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let _ = std::fs::write(dir.join(format!("crash-{ts}.txt")), scrubbed);
}

/// The newest pending crash report (already scrubbed), if any.
pub fn pending_crash() -> Option<String> {
    let dir = crash_dir()?;
    let mut newest: Option<(u128, PathBuf)> = None;
    for entry in std::fs::read_dir(&dir).ok()?.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("txt") {
            continue;
        }
        let mtime = entry
            .metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_millis())
            .unwrap_or(0);
        if newest.as_ref().map(|(t, _)| mtime > *t).unwrap_or(true) {
            newest = Some((mtime, path));
        }
    }
    let (_, path) = newest?;
    std::fs::read_to_string(path).ok()
}

/// Delete the pending crash reports (the user dismissed or sent them).
pub fn clear_crashes() {
    if let Some(dir) = crash_dir() {
        let _ = std::fs::remove_dir_all(dir);
    }
}

/// Percent-encode a query component (RFC 3986 unreserved kept verbatim).
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for byte in s.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

fn truncate_chars(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max).collect();
    out.push_str("\n… (truncated — use “Copy report” for the full text)");
    out
}

/// A pre-filled GitHub "new issue" URL (the user submits it while signed in —
/// no token, no server).
pub fn github_url(title: &str, body: &str) -> String {
    format!(
        "{GITHUB_NEW_ISSUE}?labels=bug&title={}&body={}",
        urlencode(&truncate_chars(title, 200)),
        urlencode(&truncate_chars(body, MAX_GITHUB_BODY)),
    )
}

/// A pre-filled `mailto:` URL (the user's own mail client sends it).
pub fn mailto_url(subject: &str, body: &str) -> String {
    format!(
        "mailto:{REPORT_EMAIL}?subject={}&body={}",
        urlencode(&truncate_chars(subject, 200)),
        urlencode(&truncate_chars(body, MAX_MAILTO_BODY)),
    )
}

/// Open an https/mailto URL with the OS default handler. The URL is one we
/// built (validated scheme, no control chars) and passed as a single argv
/// entry — no shell.
fn open_url(url: &str) -> Result<(), String> {
    if !(url.starts_with("https://") || url.starts_with("mailto:")) {
        return Err("refusing to open a non-https/mailto URL".into());
    }
    if url.chars().any(char::is_control) {
        return Err("invalid URL".into());
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("rundll32")
            .args(["url.dll,FileProtocolHandler", url])
            .spawn()
            .map_err(|err| format!("could not open the link: {err}"))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .map_err(|err| format!("could not open the link: {err}"))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .map_err(|err| format!("could not open the link: {err}"))?;
    }
    Ok(())
}

/// A short one-line summary of the error for the subject: the crash's panic
/// message if there was one, else the first line of the user's description,
/// else a generic label. Kept to one line, bounded length.
fn error_summary(crash: Option<&str>, description: &str) -> String {
    let from_crash = crash.and_then(|c| {
        c.lines()
            .find_map(|line| line.strip_prefix("Message: "))
            .map(str::to_string)
    });
    let raw = from_crash
        .filter(|s| !s.trim().is_empty())
        .or_else(|| {
            description
                .lines()
                .map(str::trim)
                .find(|line| !line.is_empty())
                .map(str::to_string)
        })
        .unwrap_or_else(|| {
            if crash.is_some() {
                "crash report".to_string()
            } else {
                "bug report".to_string()
            }
        });
    // One line, bounded — the rest lives in the body.
    let one_line: String = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    if one_line.chars().count() > 80 {
        format!("{}…", one_line.chars().take(80).collect::<String>())
    } else {
        one_line
    }
}

/// The subject line: `[<App>] <error summary>` — which app + what went wrong.
fn subject(crash: Option<&str>, description: &str) -> String {
    format!("[{APP_NAME}] {}", error_summary(crash, description))
}

/// Build the full report body from the user's note + diagnostics (+ crash).
fn compose_body(description: &str, crash: Option<&str>) -> String {
    let mut body = String::new();
    body.push_str("### What happened\n");
    body.push_str(if description.trim().is_empty() {
        "(no description provided)"
    } else {
        description.trim()
    });
    body.push_str("\n\n### Anonymous diagnostics (no personal data)\n```\n");
    body.push_str(&format!("From: {APP_NAME}\n"));
    body.push_str(&diagnostics());
    if let Some(crash) = crash {
        body.push_str("\n--- crash excerpt ---\n");
        body.push_str(crash);
    }
    body.push_str("\n```\n");
    // Belt-and-suspenders: scrub the whole assembled body once more.
    scrub(&body)
}

// --- Tauri commands --------------------------------------------------------

/// What the "Report a bug" dialog shows: the anonymous system info + any
/// pending crash from the last run (already scrubbed). Nothing is sent here.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BugReportContextDto {
    pub app_version: String,
    pub os: String,
    pub arch: String,
    pub diagnostics: String,
    /// The scrubbed crash text from the previous run, if the app crashed.
    pub pending_crash: Option<String>,
}

#[tauri::command]
pub fn bug_report_context() -> BugReportContextDto {
    BugReportContextDto {
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        diagnostics: diagnostics(),
        pending_crash: pending_crash(),
    }
}

/// Submit a report: build the anonymous body from the user's note (+ the crash
/// excerpt if `include_crash`) and open it via `target` = `"github"` |
/// `"email"`. This only opens a pre-filled page/mail draft — the user still
/// clicks send. Nothing leaves the machine automatically.
#[tauri::command]
pub fn bug_report_submit(
    target: String,
    description: String,
    include_crash: bool,
) -> Result<(), String> {
    let crash = if include_crash {
        pending_crash()
    } else {
        None
    };
    // Subject: [Freally Capture] <what went wrong> — the app + the error.
    let subject = subject(crash.as_deref(), &description);
    let body = compose_body(&description, crash.as_deref());
    let url = match target.as_str() {
        "github" => github_url(&subject, &body),
        "email" => mailto_url(&subject, &body),
        other => return Err(format!("unknown report target: {other}")),
    };
    open_url(&url)
}

/// Dismiss + delete the pending crash report(s).
#[tauri::command]
pub fn bug_report_clear_crash() {
    clear_crashes();
}

/// Write a harmless sample crash report so the "we found a crash — report it?"
/// flow can be **tested** without actually crashing the app. The UI exposes
/// this behind a small "simulate a crash report" affordance.
#[tauri::command]
pub fn bug_report_simulate<R: Runtime>(app: AppHandle<R>) {
    write_crash(&scrub(
        "Panic at src/example.rs:42\nMessage: this is a SAMPLE crash report for testing the \
         opt-in bug-report flow — no real crash occurred.\n\nBacktrace:\n(sample)\n",
    ));
    // Nudge the UI to re-check for a pending crash.
    let _ = app.emit("bug-report-crash-detected", ());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrub_redacts_home_and_username() {
        // Whatever the real home is, scrubbing it must remove it.
        if let Some(dirs) = directories::UserDirs::new() {
            let home = dirs.home_dir().to_string_lossy().to_string();
            if !home.is_empty() {
                let text = format!("error opening {home}/Videos/secret.frec");
                let scrubbed = scrub(&text);
                assert!(!scrubbed.contains(&home), "home path must be redacted");
            }
        }
    }

    #[test]
    fn urlencode_escapes_unsafe_and_keeps_unreserved() {
        assert_eq!(urlencode("a b&c"), "a%20b%26c");
        assert_eq!(urlencode("Aa0-_.~"), "Aa0-_.~");
        assert_eq!(urlencode("líne\n"), "l%C3%ADne%0A");
    }

    #[test]
    fn github_and_mailto_urls_are_wellformed_and_bounded() {
        let long = "x".repeat(20_000);
        let gh = github_url("Bug report", &long);
        assert!(gh.starts_with("https://github.com/"));
        assert!(gh.contains("labels=bug"));
        // Truncation kept the URL far under the 20k input.
        assert!(gh.len() < 10_000, "github url must be bounded");

        let mail = mailto_url("Bug report", &long);
        assert!(mail.starts_with("mailto:mythodikalone@gmail.com?"));
        assert!(mail.len() < 4_000, "mailto url must be bounded");
    }

    #[test]
    fn open_url_refuses_non_http_mailto() {
        assert!(open_url("file:///etc/passwd").is_err());
        assert!(open_url("javascript:alert(1)").is_err());
        assert!(open_url("https://ok.example/\u{7}").is_err());
    }

    #[test]
    fn compose_body_is_scrubbed_and_labeled() {
        let body = compose_body("it broke", Some("crash text"));
        assert!(body.contains("What happened"));
        assert!(body.contains("Anonymous diagnostics"));
        assert!(body.contains("Freally Capture"));
        assert!(body.contains("From: Freally Capture"));
    }

    #[test]
    fn subject_names_the_app_and_the_error() {
        // A crash → the panic message rides the subject.
        let crash = "Panic at src/x.rs:1\nMessage: index out of bounds\nBacktrace:\n";
        let s = subject(Some(crash), "");
        assert_eq!(s, "[Freally Capture] index out of bounds");

        // A manual report → the description's first line.
        let s2 = subject(None, "audio cuts out when I switch scenes\nmore detail");
        assert_eq!(s2, "[Freally Capture] audio cuts out when I switch scenes");

        // Nothing useful → a generic, still app-tagged subject.
        assert_eq!(subject(None, "   "), "[Freally Capture] bug report");

        // Long summaries are bounded to one line.
        let long = format!("Message: {}", "x ".repeat(200));
        let s3 = subject(Some(&format!("Panic\n{long}")), "");
        assert!(s3.chars().count() <= "[Freally Capture] ".len() + 81);
    }
}
