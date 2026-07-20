//! CAP-N77 — the **freally-browser-host** executable (protocol v1).
//!
//! Renders `--url` offscreen via the CEF runtime at `--cef` and streams the
//! `FBH1` header + fixed-size RGBA frames on stdout; the app side
//! (`fcap_sources::browser`) pumps them like any decode pipe. The normative
//! contract lives in `design/browser-host-protocol.md`.
//!
//! **This workspace build is the skeleton:** the real CEF OSR backend is
//! compiled ONLY by the CI component build (`--features cef`, linked against
//! the pinned CEF SDK per OS) and published as a release asset the component
//! installer fetches. Built without the feature, the host validates its
//! arguments and exits **3** with an honest one-line reason — exactly what
//! the protocol reserves for "CEF runtime missing/incompatible".

use std::process::ExitCode;

/// The parsed invocation (protocol v1's argument surface).
#[derive(Debug, PartialEq)]
struct Args {
    url: String,
    width: u32,
    height: u32,
    fps: u32,
    transparent: bool,
    cef: String,
}

/// Hand-rolled parsing — the protocol is tiny and the host takes no deps.
fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut url = None;
    let mut width = None;
    let mut height = None;
    let mut fps = None;
    let mut transparent = false;
    let mut cef = None;
    let mut iter = argv.iter();
    while let Some(flag) = iter.next() {
        let mut value = |name: &str| -> Result<String, String> {
            iter.next()
                .cloned()
                .ok_or_else(|| format!("{name} needs a value"))
        };
        match flag.as_str() {
            "--url" => url = Some(value("--url")?),
            "--width" => width = Some(value("--width")?),
            "--height" => height = Some(value("--height")?),
            "--fps" => fps = Some(value("--fps")?),
            "--cef" => cef = Some(value("--cef")?),
            "--transparent" => transparent = true,
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    let parse_u32 = |name: &str, text: Option<String>, min: u32, max: u32| -> Result<u32, String> {
        let text = text.ok_or_else(|| format!("{name} is required"))?;
        let n: u32 = text
            .parse()
            .map_err(|_| format!("{name} must be a number, got {text:?}"))?;
        // max-then-min: u32::clamp panics when min > max, and honest bounds
        // beat a panic in a helper the app supervises.
        Ok(n.max(min).min(max))
    };
    let url = url.ok_or("--url is required")?;
    // Schemes are case-insensitive (RFC 3986); still an allowlist, fails closed.
    let scheme = url.to_ascii_lowercase();
    if !(scheme.starts_with("http://") || scheme.starts_with("https://")) {
        return Err("--url must be http:// or https:// (protocol v1)".into());
    }
    Ok(Args {
        url,
        width: parse_u32("--width", width, 64, 3840)?,
        height: parse_u32("--height", height, 64, 2160)?,
        fps: parse_u32("--fps", fps, 1, 60)?,
        transparent,
        cef: cef.ok_or("--cef is required")?,
    })
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&argv) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("freally-browser-host: {err}");
            return ExitCode::from(2);
        }
    };

    #[cfg(feature = "cef")]
    {
        return backend::run(args);
    }
    #[cfg(not(feature = "cef"))]
    {
        // The skeleton build has no renderer — exit 3 per the protocol so
        // the app reports "component missing/incompatible", never a hang.
        let _ = args;
        eprintln!(
            "freally-browser-host: built without the CEF backend — install the \
             Browser Runtime component build (this binary is the workspace skeleton)"
        );
        ExitCode::from(3)
    }
}

#[cfg(feature = "cef")]
mod backend {
    //! The real OSR renderer, compiled only in the CI component build
    //! against the pinned CEF SDK. Kept as a named seam so the skeleton and
    //! the real build share `main`/argument parsing byte-for-byte.
    use super::Args;
    use std::process::ExitCode;

    pub fn run(_args: Args) -> ExitCode {
        // CI build wires: load libcef from --cef (refuse a mismatched
        // version → exit 3), CefInitialize windowless, create the OSR
        // browser at width×height, OnPaint → stdout writer (FBH1 header
        // once, then fixed-size RGBA frames paced at fps), SIGTERM/stdin
        // close → clean shutdown (exit 0).
        eprintln!("freally-browser-host: CEF backend not implemented yet");
        ExitCode::from(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parses_a_full_invocation() {
        let args = parse_args(&argv(&[
            "--url",
            "https://example.com/overlay",
            "--width",
            "1280",
            "--height",
            "720",
            "--fps",
            "30",
            "--transparent",
            "--cef",
            "/cache/cef/current/dist",
        ]))
        .expect("valid");
        assert_eq!(
            args,
            Args {
                url: "https://example.com/overlay".into(),
                width: 1280,
                height: 720,
                fps: 30,
                transparent: true,
                cef: "/cache/cef/current/dist".into(),
            }
        );
    }

    #[test]
    fn refuses_non_http_urls_missing_args_and_clamps() {
        assert!(parse_args(&argv(&["--url", "file:///x", "--cef", "d"])).is_err());
        assert!(
            parse_args(&argv(&["--url", "https://x"])).is_err(),
            "--cef required"
        );
        assert!(parse_args(&argv(&["--bogus"])).is_err());
        let args = parse_args(&argv(&[
            "--url",
            "https://x",
            "--width",
            "999999",
            "--height",
            "1",
            "--fps",
            "0",
            "--cef",
            "d",
        ]))
        .expect("clamped, not rejected");
        assert_eq!((args.width, args.height, args.fps), (3840, 64, 1));
    }
}
