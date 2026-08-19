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

/// The protocol's scheme allowlist, and **the only** gate for any URL this host
/// loads — the one from argv *and* every navigation the page then asks for.
///
/// Checking argv alone would be a hole: a page served over http can redirect
/// (302, `location.href`, `window.open`, a frame) to `file://` and read local
/// files, or to `javascript:`/`data:` to run script the operator never chose.
/// The initial URL being safe says nothing about where the page goes next.
///
/// **The CEF backend MUST call this from its navigation handler** (`OnBeforeBrowse`
/// and the resource-request path) and refuse anything it rejects — see
/// `design/browser-host-protocol.md`. Keeping one function means the argv rule
/// and the redirect rule can never drift apart.
///
/// Fails closed: schemes are case-insensitive (RFC 3986), and anything that is
/// not plainly http/https is refused — `file://` and its UNC forms (which would
/// stat a network path, the CAP-M16 rule), `javascript:`, `data:`, `blob:`,
/// `about:`, `chrome:`, and any scheme invented later. Local files already play
/// through the Media/Image sources.
pub fn is_allowed_url(url: &str) -> bool {
    let url = url.trim().to_ascii_lowercase();
    url.starts_with("http://") || url.starts_with("https://")
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
    let url = url.ok_or("--url is required")?.trim().to_string();
    if !is_allowed_url(&url) {
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
    // CEF relaunches this exe for its own subprocesses (GPU, renderer, utility)
    // with `--type=` args our parser doesn't understand. cef_execute_process
    // must run BEFORE we parse, so a subprocess exits here without ever reaching
    // the argument check. Returns None only in the main browser process.
    #[cfg(feature = "cef")]
    if let Some(code) = cef_backend::run_subprocess_if_any() {
        return code;
    }

    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&argv) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("freally-browser-host: {err}");
            return ExitCode::from(2);
        }
    };

    // Exactly one of these two blocks survives cfg-stripping, so whichever
    // remains is main's tail expression.
    #[cfg(feature = "cef")]
    {
        cef_backend::run(args)
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

// The real OSR renderer, compiled only under `--features cef` against the pinned
// CEF SDK (linked by build.rs). Skeleton and real build share `main` + argument
// parsing byte-for-byte; only this module differs.
#[cfg(feature = "cef")]
#[path = "cef_backend.rs"]
mod cef_backend;

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

    #[test]
    fn the_allowlist_gates_redirect_targets_not_just_argv() {
        assert!(is_allowed_url("https://example.com/overlay"));
        assert!(is_allowed_url("http://localhost:8080/x"));
        assert!(
            is_allowed_url("  HTTPS://Example.com  "),
            "case + surrounding space"
        );

        // Everything a redirected page could reach for. Each of these is a
        // realistic 302 / location.href target, not a theoretical scheme.
        for hostile in [
            "file:///C:/Windows/win.ini",
            "file:///etc/passwd",
            r"file://\server\share\x.html",
            "javascript:fetch('/steal')",
            "data:text/html,<script>alert(1)</script>",
            "blob:https://example.com/9b1d",
            "about:blank",
            "chrome://settings",
            "devtools://devtools/bundled/inspector.html",
            "ftp://example.com/x",
            "",
            "   ",
            "//example.com/protocol-relative",
            "ht\ttp://example.com",
        ] {
            assert!(!is_allowed_url(hostile), "must fail closed for {hostile:?}");
        }
    }
}
