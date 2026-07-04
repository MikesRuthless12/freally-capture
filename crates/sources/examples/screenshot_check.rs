//! `screenshot_check` — the CI screenshot-smoke gate.
//!
//! The `screenshot-smoke` workflow launches the release app on Windows/macOS/
//! Linux and captures the screen. Uploading that PNG only *documents* the run;
//! this turns it into an enforced **pass/fail**: it exits non-zero when the
//! capture is missing, too small, or effectively **blank** — a black/uniform
//! frame, i.e. the classic "the webview/preview never painted" failure this
//! milestone keeps hitting (headless WebKitGTK, an unbound swapchain, …). A
//! real studio UI has thousands of distinct colours; a blank frame has a
//! handful, so a distinct-colour count separates the two with a wide margin.
//!
//! Whole-screen for now, matching the capture steps. A preview-*region* assert
//! (inspect the exact GPU rectangle) lands with the native surfaces — NP.4
//! (macOS CAMetalLayer) / NP.5 (Linux Vulkan) — where such a rectangle exists.
//!
//! It lives in `fcap-sources` because that crate already owns `image`, and it
//! is an *example* (not a bin) so it never ships in the app, yet `--all-targets`
//! clippy still compiles it on every OS. Run:
//!   cargo run --release -p fcap-sources --example screenshot_check -- shot.png

use std::collections::HashMap;
use std::process::ExitCode;

/// Reject frames smaller than this on either axis: a real capture is
/// full-screen, so a tiny image means the capture tool itself errored.
const MIN_DIM: u32 = 200;

/// A rendered UI clears this many distinct (5-bit-quantised) colours by a wide
/// margin; a blank/black frame has only a handful.
const MIN_DISTINCT_COLORS: usize = 24;

/// If one colour still covers essentially the whole frame, it's blank
/// regardless of a few stray colours (e.g. a black frame with a mouse cursor).
const MAX_DOMINANT_FRACTION: f64 = 0.995;

fn main() -> ExitCode {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: screenshot_check <screenshot.png>");
        return ExitCode::FAILURE;
    };

    let img = match image::open(&path) {
        Ok(img) => img.to_rgb8(),
        Err(err) => {
            eprintln!("screenshot_check: cannot read {path}: {err}");
            return ExitCode::FAILURE;
        }
    };

    let (w, h) = img.dimensions();
    if w < MIN_DIM || h < MIN_DIM {
        eprintln!(
            "screenshot_check: {path} is {w}x{h}, smaller than {MIN_DIM}px — capture likely failed"
        );
        return ExitCode::FAILURE;
    }

    // Sample on a stride so the scan is bounded (~<=50k pixels) at any size.
    let total = u64::from(w) * u64::from(h);
    let stride = usize::try_from(total / 50_000).unwrap_or(usize::MAX).max(1);

    let mut counts: HashMap<u16, u32> = HashMap::new();
    let mut sampled: u32 = 0;
    for px in img.pixels().step_by(stride) {
        let [r, g, b] = px.0;
        // 5 bits/channel: tolerant of compression noise, still ~32k buckets.
        let key = (u16::from(r >> 3) << 10) | (u16::from(g >> 3) << 5) | u16::from(b >> 3);
        *counts.entry(key).or_insert(0) += 1;
        sampled += 1;
    }

    let distinct = counts.len();
    let dominant = counts.values().copied().max().unwrap_or(0);
    let dominant_frac = f64::from(dominant) / f64::from(sampled.max(1));

    println!(
        "screenshot_check: {path} {w}x{h}, sampled {sampled}, {distinct} distinct colours, dominant {:.2}%",
        dominant_frac * 100.0
    );

    if distinct < MIN_DISTINCT_COLORS || dominant_frac > MAX_DOMINANT_FRACTION {
        eprintln!(
            "screenshot_check: FAIL — frame looks blank ({distinct} distinct < {MIN_DISTINCT_COLORS}, \
             or dominant {:.2}% > {:.2}%). The app never rendered.",
            dominant_frac * 100.0,
            MAX_DOMINANT_FRACTION * 100.0
        );
        return ExitCode::FAILURE;
    }

    println!("screenshot_check: PASS — frame rendered.");
    ExitCode::SUCCESS
}
