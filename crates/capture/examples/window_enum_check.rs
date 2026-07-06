//! `window_enum_check` — the window-enumeration CI probe.
//!
//! The `window-enum` workflow opens a couple of known windows (minimizing one)
//! on each OS, then runs this probe: it calls [`fcap_capture::list_sources`] and
//! asserts the expected window titles are listed — proving the source picker
//! sees *every* open window, minimized included, on the platforms the author
//! can't run locally (macOS, Linux/X11). Windows is verified locally.
//!
//! Usage: `window_enum_check [--expect SUBSTR]... [--min-windows N]`
//! Exit codes:
//!   0  every `--expect` matched a listed window (and `--min-windows` was met)
//!   1  a window was missing — a real enumeration regression
//!   3  enumeration itself errored (e.g. the macOS Screen-Recording permission
//!      is absent on the runner) — an environment limit, not a code defect; the
//!      caller decides whether to treat it as fatal.

use std::process::ExitCode;

use fcap_capture::{list_sources, SourceKind};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut expects: Vec<String> = Vec::new();
    let mut min_windows = 0usize;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--expect" => {
                if let Some(v) = args.get(i + 1) {
                    expects.push(v.clone());
                }
                i += 1;
            }
            "--min-windows" => {
                if let Some(v) = args.get(i + 1) {
                    min_windows = v.parse().unwrap_or(0);
                }
                i += 1;
            }
            other => eprintln!("window_enum_check: ignoring unknown arg {other:?}"),
        }
        i += 1;
    }

    let sources = match list_sources() {
        Ok(sources) => sources,
        Err(err) => {
            eprintln!("window_enum_check: enumeration errored: {err}");
            return ExitCode::from(3);
        }
    };

    let windows: Vec<_> = sources
        .iter()
        .filter(|s| s.kind == SourceKind::Window)
        .collect();

    println!(
        "window_enum_check: {} window source(s) listed:",
        windows.len()
    );
    for w in &windows {
        println!("  · {} ({}×{})", w.label, w.width, w.height);
    }
    println!(
        "window_enum_check: (plus {} display/portal source(s))",
        sources.len() - windows.len()
    );

    let mut ok = true;
    for want in &expects {
        let needle = want.to_lowercase();
        let found = windows
            .iter()
            .any(|w| w.label.to_lowercase().contains(&needle));
        println!(
            "  expect {want:?}: {}",
            if found { "FOUND" } else { "MISSING" }
        );
        if !found {
            ok = false;
        }
    }
    if windows.len() < min_windows {
        eprintln!(
            "window_enum_check: only {} window(s) listed, expected at least {min_windows}",
            windows.len()
        );
        ok = false;
    }

    if ok {
        println!("window_enum_check: PASS");
        ExitCode::SUCCESS
    } else {
        eprintln!("window_enum_check: FAIL — an expected window was not listed");
        ExitCode::FAILURE
    }
}
