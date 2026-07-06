//! Durable window identity + re-resolution, shared by every per-OS window
//! capture path (Windows WGC, macOS SCK, Linux X11).
//!
//! A window's OS handle — HWND, `CGWindowID`, X11 XID — is only valid within
//! the session it was picked in; it is stale after an app (or the target app)
//! restart. So a persisted Window Capture stores a [`WindowKey`] alongside the
//! handle, and on start re-binds to the *same* window by matching that key, the
//! way OBS re-attaches a Window Capture on launch. This module owns the encoding
//! (into the opaque source id the scene persists) and the pure matching; the
//! OS modules supply the platform getters + the fast-path handle validation.

/// A window's *durable* identity — the parts that survive a restart (the handle
/// does not). The field roles are OS-neutral so one matcher serves every path:
///
/// | field   | Windows          | macOS               | Linux/X11             |
/// |---------|------------------|---------------------|-----------------------|
/// | `app`   | exe file name    | owning app name     | `WM_CLASS` class      |
/// | `class` | window class     | *(unused)*          | `WM_CLASS` instance   |
/// | `title` | window title     | window title        | `_NET_WM_NAME`        |
///
/// `app` is the anchor: when it is known, a candidate whose `app` differs is
/// never a match (we don't grab an unrelated program). `class` and `title` then
/// refine the score so the closest window among several from the same app wins.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct WindowKey {
    pub app: String,
    pub class: String,
    pub title: String,
}

impl WindowKey {
    pub fn new(app: impl Into<String>, class: impl Into<String>, title: impl Into<String>) -> Self {
        WindowKey {
            app: app.into(),
            class: class.into(),
            title: title.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.app.is_empty() && self.class.is_empty() && self.title.is_empty()
    }
}

/// Encode a window reference — the fast-path handle plus its durable identity —
/// into the opaque id `list_sources` hands out (and the scene persists). The
/// three text fields are base64'd so any title survives the `:`-joined form.
pub(crate) fn encode_window_id(handle: u64, key: &WindowKey) -> String {
    format!(
        "{handle}:{}:{}:{}",
        b64_encode(key.app.as_bytes()),
        b64_encode(key.class.as_bytes()),
        b64_encode(key.title.as_bytes())
    )
}

/// Parse the payload after the per-OS window prefix. Accepts the rich
/// `handle:app:class:title` form and the legacy bare `handle` (no identity —
/// treated as handle-only, preserving pre-rebind behavior for old saved scenes).
pub(crate) fn decode_window_id(raw: &str) -> Option<(u64, WindowKey)> {
    let parts: Vec<&str> = raw.split(':').collect();
    match parts.as_slice() {
        [handle] => Some((handle.parse().ok()?, WindowKey::default())),
        [handle, app, class, title] => Some((
            handle.parse().ok()?,
            WindowKey {
                app: b64_decode(app)?,
                class: b64_decode(class)?,
                title: b64_decode(title)?,
            },
        )),
        _ => None,
    }
}

/// The best live match for `target` among `candidates` (handle + its key), or
/// `None`. Ties go to the first candidate — callers enumerate top of the
/// Z-order down, so that is the topmost window.
pub(crate) fn resolve_best(target: &WindowKey, candidates: &[(u64, WindowKey)]) -> Option<u64> {
    let mut best: Option<(u32, u64)> = None;
    for (handle, key) in candidates {
        if let Some(score) = match_score(target, key) {
            let better = match best {
                Some((top, _)) => score > top,
                None => true,
            };
            if better {
                best = Some((score, *handle));
            }
        }
    }
    best.map(|(_, handle)| handle)
}

/// Whether the window now at a stored handle is still plausibly the one that was
/// picked — the lenient check for the fast path, where the handle/id *already*
/// identified the window uniquely. It rejects only a *positively different*
/// window: a differing known app means the OS recycled the handle into another
/// program. Deliberately looser than [`resolve_best`]'s scoring, which must
/// anchor strictly because it re-picks among many windows — a still-valid id
/// with no app + no class (every macOS window) must not be thrown away here.
pub(crate) fn same_window(stored: &WindowKey, live: &WindowKey) -> bool {
    if !stored.app.is_empty() && !live.app.is_empty() {
        return stored.app.eq_ignore_ascii_case(&live.app);
    }
    // No app on one side to compare on — use the title when both have one.
    if !stored.title.is_empty() && !live.title.is_empty() {
        return stored.title == live.title || loose_title_match(&stored.title, &live.title);
    }
    // Nothing contradicts the handle — trust it.
    true
}

/// Score how well `cand` matches the `target` identity; `None` = not a match.
fn match_score(target: &WindowKey, cand: &WindowKey) -> Option<u32> {
    let app_known = !target.app.is_empty();
    let app_match = app_known && target.app.eq_ignore_ascii_case(&cand.app);
    if app_known && !app_match {
        return None;
    }
    let class_match = !target.class.is_empty() && target.class.eq_ignore_ascii_case(&cand.class);
    let title_exact = !target.title.is_empty() && target.title == cand.title;
    let title_loose = !title_exact && loose_title_match(&target.title, &cand.title);

    if !app_known && !title_exact {
        // Without a known app anchor, the exact title is the only trustworthy
        // signal (macOS windows have no class; some X11 windows lack WM_CLASS),
        // so require it — otherwise we'd risk binding to an arbitrary window.
        // (A matching class, when present, still adds its bonus below.)
        return None;
    }

    let mut score = 0;
    if app_match {
        score += 4;
    }
    if class_match {
        score += 2;
    }
    if title_exact {
        score += 2;
    } else if title_loose {
        score += 1;
    }
    Some(score)
}

/// A soft title match — the shorter title is a prefix or suffix of the longer
/// (both non-empty). Catches document/tab drift that keeps a stable prefix or
/// suffix ("Doc — App" ↔ "Doc — App — edited") without matching on an arbitrary
/// mid-string coincidence, which a plain `contains` would.
fn loose_title_match(a: &str, b: &str) -> bool {
    if a.is_empty() || b.is_empty() {
        return false;
    }
    let (short, long) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    long.starts_with(short) || long.ends_with(short)
}

const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Standard padded base64 of a byte slice (no external dep — a few dozen bytes
/// per id). The alphabet never emits `:`, so the encoded form is `:`-splittable.
fn b64_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);
        let n = (u32::from(chunk[0]) << 16) | (u32::from(b1) << 8) | u32::from(b2);
        out.push(B64[((n >> 18) & 63) as usize] as char);
        out.push(B64[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            B64[((n >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            B64[(n & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

/// Decode standard base64 back to a UTF-8 string (our own data, so it is);
/// `None` on an invalid character or non-UTF-8 result.
fn b64_decode(text: &str) -> Option<String> {
    fn val(c: u8) -> Option<u32> {
        match c {
            b'A'..=b'Z' => Some(u32::from(c - b'A')),
            b'a'..=b'z' => Some(u32::from(c - b'a') + 26),
            b'0'..=b'9' => Some(u32::from(c - b'0') + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let bytes: Vec<u8> = text.bytes().filter(|&c| c != b'=').collect();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    for chunk in bytes.chunks(4) {
        let mut acc = 0u32;
        for &c in chunk {
            acc = (acc << 6) | val(c)?;
        }
        let bits = chunk.len() * 6;
        acc <<= 24 - bits; // left-align the meaningful bits
        for i in 0..bits / 8 {
            out.push((acc >> (16 - i * 8)) as u8);
        }
    }
    String::from_utf8(out).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(app: &str, class: &str, title: &str) -> WindowKey {
        WindowKey::new(app, class, title)
    }

    #[test]
    fn base64_round_trips_including_empty_and_unicode() {
        for s in [
            "",
            "chrome.exe",
            "Notepad",
            "タイトル: 例",
            "a:b|c/d+e=f",
            "🙂",
        ] {
            assert_eq!(b64_decode(&b64_encode(s.as_bytes())).as_deref(), Some(s));
        }
    }

    #[test]
    fn window_id_round_trips_even_with_colons_in_the_title() {
        let k = key("chrome.exe", "Chrome_WidgetWin_1", "GitHub: home — Chrome");
        let (handle, back) = decode_window_id(&encode_window_id(12345, &k)).expect("decodes");
        assert_eq!(handle, 12345);
        assert_eq!(back, k);
    }

    #[test]
    fn legacy_bare_handle_id_still_parses_as_handle_only() {
        let (handle, k) = decode_window_id("98765").expect("legacy id decodes");
        assert_eq!(handle, 98765);
        assert!(k.is_empty(), "no identity travels with a legacy id");
    }

    #[test]
    fn malformed_ids_are_rejected() {
        assert!(decode_window_id("not-a-number").is_none());
        assert!(decode_window_id("1:2:3").is_none(), "wrong field count");
        assert!(decode_window_id("").is_none());
    }

    #[test]
    fn exact_identity_scores_highest() {
        let k = key("app.exe", "AppClass", "Doc — App");
        assert_eq!(match_score(&k, &k), Some(8));
    }

    #[test]
    fn a_different_app_never_matches() {
        let target = key("chrome.exe", "Chrome_WidgetWin_1", "X — Chrome");
        let other = key("firefox.exe", "Chrome_WidgetWin_1", "X — Chrome");
        assert_eq!(match_score(&target, &other), None);
    }

    #[test]
    fn the_app_anchor_is_case_insensitive() {
        let target = key("Chrome.EXE", "C", "T");
        assert_eq!(match_score(&target, &key("chrome.exe", "C", "T")), Some(8));
    }

    #[test]
    fn among_same_app_windows_the_exact_title_wins() {
        let target = key(
            "code.exe",
            "Chrome_WidgetWin_1",
            "main.rs — project — VS Code",
        );
        let class_only = key(
            "code.exe",
            "Chrome_WidgetWin_1",
            "other.rs — project — VS Code",
        );
        let exact = key(
            "code.exe",
            "Chrome_WidgetWin_1",
            "main.rs — project — VS Code",
        );
        let cands = [(10u64, class_only), (20u64, exact)];
        assert_eq!(resolve_best(&target, &cands), Some(20));
    }

    #[test]
    fn re_resolves_to_a_new_handle_after_a_restart() {
        // The stored handle is gone; a freshly-launched window (new handle 42)
        // carries the same identity — that's the one we must bind to.
        let target = key("game.exe", "UnrealWindow", "My Game");
        let cands = [(42u64, key("game.exe", "UnrealWindow", "My Game"))];
        assert_eq!(resolve_best(&target, &cands), Some(42));
    }

    #[test]
    fn a_tie_takes_the_topmost_window() {
        let target = key("app.exe", "C", "T");
        let cands = [
            (1u64, key("app.exe", "C", "T")),
            (2u64, key("app.exe", "C", "T")),
        ];
        assert_eq!(
            resolve_best(&target, &cands),
            Some(1),
            "first enumerated wins a tie"
        );
    }

    #[test]
    fn nothing_matches_an_absent_app() {
        let target = key("app.exe", "C", "T");
        let cands = [(1u64, key("other.exe", "X", "Y"))];
        assert_eq!(resolve_best(&target, &cands), None);
    }

    #[test]
    fn without_a_known_app_the_exact_title_is_the_anchor() {
        // macOS windows have no class and may have no owning-app name; the exact
        // title then carries the match (a matching class is only a bonus).
        let target = key("", "AppClass", "Exact Title");
        assert_eq!(
            match_score(&target, &key("", "AppClass", "Exact Title")),
            Some(4),
            "class bonus + exact title"
        );
        assert_eq!(
            match_score(&target, &key("", "", "Exact Title")),
            Some(2),
            "exact title alone still matches with no class (the macOS case)"
        );
        assert_eq!(
            match_score(&target, &key("", "AppClass", "Different")),
            None,
            "a differing title with no app anchor is not a match"
        );
    }

    #[test]
    fn a_loose_title_breaks_ties_under_the_same_app() {
        let target = key("browser.exe", "Web", "Inbox — Mail");
        let unrelated = key("browser.exe", "Web", "Totally Different");
        let loose = key("browser.exe", "Web", "Inbox — Mail — Extra");
        assert!(match_score(&target, &loose) > match_score(&target, &unrelated));
    }

    #[test]
    fn a_loose_title_is_only_a_prefix_or_suffix_not_any_substring() {
        // "Inbox" is a prefix → loose; a mid-string coincidence is not.
        assert!(loose_title_match("Inbox", "Inbox — Mail"));
        assert!(loose_title_match("— Chrome", "GitHub — Chrome"));
        assert!(!loose_title_match("Mail", "Inbox — Mail — sub"));
    }

    #[test]
    fn same_window_trusts_the_handle_unless_the_app_changed() {
        // Fast-path validation: a matching (or absent) app is trusted even as the
        // title drifts; a different known app means a recycled handle → reject.
        let stored = key("chrome.exe", "Chrome_WidgetWin_1", "GitHub — Chrome");
        assert!(
            same_window(
                &stored,
                &key("chrome.exe", "Chrome_WidgetWin_1", "Gmail — Chrome")
            ),
            "same app, drifted title → still ours"
        );
        assert!(
            !same_window(&stored, &key("notepad.exe", "Notepad", "untitled")),
            "recycled into a different app → reject"
        );
    }

    #[test]
    fn same_window_falls_back_to_title_when_the_app_is_unknown() {
        // Regression guard: a still-valid macOS id (no class, sometimes no app
        // name) must not be thrown away — validate by title instead.
        let stored = key("", "", "My Document");
        assert!(same_window(&stored, &key("", "", "My Document")));
        assert!(!same_window(&stored, &key("", "", "Someone Else")));
    }
}
