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
//! `--expect-magenta` (used by the Linux job) switches to a stronger, surface-
//! specific gate: assert the seeded smoke scene's magenta (255,0,255) covers a
//! real fraction of the frame — direct proof NP.5's native Vulkan surface
//! painted, independent of whatever headless WebKitGTK does. The seeded colour
//! + forced region live in `studio::seed_smoke_scene`.
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

/// `--expect-magenta`: the seeded smoke scene paints a magenta (255,0,255) box
/// that covers several percent of the frame. Require a wide-margin floor so a
/// non-painting surface (0% magenta) fails hard, while a later tweak to the box
/// size doesn't trip it.
const MIN_MAGENTA_FRACTION: f64 = 0.005;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let expect_magenta = args.iter().any(|a| a.as_str() == "--expect-magenta");
    let Some(path) = args.iter().find(|a| !a.starts_with("--")).cloned() else {
        eprintln!("usage: screenshot_check <screenshot.png> [--expect-magenta]");
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
    let mut magenta: u32 = 0;
    let mut sampled: u32 = 0;
    for px in img.pixels().step_by(stride) {
        let [r, g, b] = px.0;
        // Tolerant magenta test (high R + B, low G): matches the seeded box but
        // not the UI's blues/cyans (low R or high G) or its whites/greys.
        if r >= 200 && g <= 60 && b >= 200 {
            magenta += 1;
        }
        // 5 bits/channel: tolerant of compression noise, still ~32k buckets.
        let key = (u16::from(r >> 3) << 10) | (u16::from(g >> 3) << 5) | u16::from(b >> 3);
        *counts.entry(key).or_insert(0) += 1;
        sampled += 1;
    }

    let distinct = counts.len();
    let dominant = counts.values().copied().max().unwrap_or(0);
    let dominant_frac = f64::from(dominant) / f64::from(sampled.max(1));

    // Linux: the native Vulkan surface is the thing under test, so gate on the
    // seeded magenta actually being present rather than the generic not-blank
    // heuristic (which would also depend on headless WebKitGTK). Magenta present
    // is itself proof the frame isn't blank.
    if expect_magenta {
        let magenta_frac = f64::from(magenta) / f64::from(sampled.max(1));
        println!(
            "screenshot_check: {path} {w}x{h}, sampled {sampled}, magenta {:.2}%",
            magenta_frac * 100.0
        );
        if magenta_frac < MIN_MAGENTA_FRACTION {
            eprintln!(
                "screenshot_check: FAIL — native surface did not paint: magenta {:.3}% < {:.3}%. \
                 NP.5's Vulkan surface never presented the seeded scene.",
                magenta_frac * 100.0,
                MIN_MAGENTA_FRACTION * 100.0
            );
            return ExitCode::FAILURE;
        }
        println!("screenshot_check: PASS — native surface painted (magenta present).");
        return ExitCode::SUCCESS;
    }

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
