//! Token-based output filenames (CAP-M25): every recording, replay save and
//! still resolves its name through [`resolve_template`] — the single choke
//! point future output kinds (ISO lanes) flow through too. Pure string work,
//! no filesystem; collision suffixing stays in
//! [`crate::recording::unique_recording_path`].

/// Everything a template can reference, gathered by the caller at the moment
/// of naming (so tests inject fixed values).
#[derive(Debug, Clone)]
pub struct TokenContext {
    /// The per-output kind prefix (`{prefix}`): the user's recording prefix,
    /// or `Replay` / `Still`.
    pub prefix: String,
    /// Local date, `%Y-%m-%d`.
    pub date: String,
    /// Local time, `%H-%M-%S`.
    pub time: String,
    /// The active scene's name (user text — sanitized on substitution).
    pub scene: String,
    /// The active profile's name (user text — sanitized on substitution).
    pub profile: String,
    /// The recorded geometry; `{canvas}` renders as `WxH`.
    pub canvas: (u32, u32),
    /// Chapter markers dropped so far this recording session.
    pub marker_count: u32,
    /// The persisted auto-increment counter, already bumped for this event.
    pub counter: u32,
}

/// Every token a template may use (also the settings-save validator's list).
pub const TOKENS: [&str; 8] = [
    "prefix",
    "date",
    "time",
    "scene",
    "profile",
    "canvas",
    "marker-count",
    "counter",
];

/// Longest filename stem we produce; NTFS/APFS/ext4 allow 255 *bytes* per
/// name — 120 chars leaves room for ` (vertical)`, ` (999)`, ` part000` and
/// the extension even when every char is 4-byte UTF-8.
const MAX_STEM_CHARS: usize = 120;

/// Characters no output filename may contain (the union of the platforms'
/// reserved sets — Windows is the strictest).
fn is_reserved(c: char) -> bool {
    c.is_control() || matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')
}

/// Resolve `template` against `ctx` into a safe filename stem (no extension).
/// Unknown `{…}` groups pass through literally — [`template_tokens_valid`]
/// rejects them at settings-save time, but resolution itself never fails.
pub fn resolve_template(template: &str, ctx: &TokenContext) -> String {
    let mut out = String::with_capacity(template.len() + 16);
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        match after.find('}') {
            Some(close) => {
                let name = &after[..close];
                match token_value(name, ctx) {
                    Some(value) => out.push_str(&value),
                    // Unknown token: keep it literally (braces are legal
                    // filename chars everywhere).
                    None => {
                        out.push('{');
                        out.push_str(name);
                        out.push('}');
                    }
                }
                rest = &after[close + 1..];
            }
            // Unclosed `{` — keep the tail literally.
            None => {
                out.push_str(&rest[open..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    let stem = sanitize_stem(&out);
    if stem.is_empty() {
        // User text (scene/profile) can sanitize away entirely; fall back to
        // the classic pre-template name so the output always has one.
        sanitize_stem(&format!("{} {} {}", ctx.prefix, ctx.date, ctx.time))
    } else {
        stem
    }
}

fn token_value(name: &str, ctx: &TokenContext) -> Option<String> {
    Some(match name {
        "prefix" => ctx.prefix.clone(),
        "date" => ctx.date.clone(),
        "time" => ctx.time.clone(),
        "scene" => ctx.scene.clone(),
        "profile" => ctx.profile.clone(),
        "canvas" => format!("{}x{}", ctx.canvas.0, ctx.canvas.1),
        "marker-count" => ctx.marker_count.to_string(),
        "counter" => format!("{:04}", ctx.counter),
        _ => return None,
    })
}

/// Make a resolved name filesystem-safe: drop reserved characters, collapse
/// whitespace runs, bound the length **by chars** (a byte truncate can split
/// a UTF-8 scalar and abort the process under `panic = "abort"`), and trim
/// trailing dots/spaces (Windows rejects them).
fn sanitize_stem(raw: &str) -> String {
    let cleaned: String = raw.chars().filter(|&c| !is_reserved(c)).collect();
    let collapsed = cleaned.split_whitespace().collect::<Vec<_>>().join(" ");
    let bounded: String = collapsed.chars().take(MAX_STEM_CHARS).collect();
    bounded.trim_end_matches(['.', ' ']).to_string()
}

/// Settings-save validation: every `{…}` group must name a known token and
/// every `{` must close — catches template typos where the user can fix
/// them instead of silently landing braces in filenames.
pub fn template_tokens_valid(template: &str) -> Result<(), String> {
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        let after = &rest[open + 1..];
        let Some(close) = after.find('}') else {
            return Err("unclosed { — tokens look like {date}".to_owned());
        };
        let name = &after[..close];
        if !TOKENS.contains(&name) {
            return Err(format!(
                "unknown token {{{name}}} — valid tokens: {}",
                TOKENS
                    .iter()
                    .map(|t| format!("{{{t}}}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        rest = &after[close + 1..];
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> TokenContext {
        TokenContext {
            prefix: "Freally Capture".to_owned(),
            date: "2026-07-12".to_owned(),
            time: "14-30-05".to_owned(),
            scene: "Main Scene".to_owned(),
            profile: "Default".to_owned(),
            canvas: (1920, 1080),
            marker_count: 3,
            counter: 7,
        }
    }

    #[test]
    fn default_template_reproduces_the_classic_name() {
        assert_eq!(
            resolve_template("{prefix} {date} {time}", &ctx()),
            "Freally Capture 2026-07-12 14-30-05"
        );
    }

    #[test]
    fn every_token_resolves() {
        assert_eq!(
            resolve_template(
                "{prefix}|{date}|{time}|{scene}|{profile}|{canvas}|{marker-count}|{counter}",
                &ctx()
            ),
            // The `|` separators are reserved chars and sanitize away.
            "Freally Capture2026-07-1214-30-05Main SceneDefault1920x108030007"
        );
    }

    #[test]
    fn canvas_marker_and_counter_render_their_formats() {
        assert_eq!(
            resolve_template("{canvas} m{marker-count} c{counter}", &ctx()),
            "1920x1080 m3 c0007"
        );
    }

    #[test]
    fn scene_names_are_sanitized_not_trusted() {
        let mut c = ctx();
        c.scene = "a/b\\c:d*e?f\"g<h>i|j".to_owned();
        assert_eq!(resolve_template("{scene}", &c), "abcdefghij");
    }

    #[test]
    fn unknown_tokens_pass_through_literally() {
        assert_eq!(resolve_template("x {nope} y", &ctx()), "x {nope} y");
    }

    #[test]
    fn unclosed_brace_is_kept_literally() {
        assert_eq!(resolve_template("take {date", &ctx()), "take {date");
    }

    #[test]
    fn truncation_counts_chars_not_bytes() {
        let mut c = ctx();
        // 300 four-byte scalars: a byte-boundary truncate would panic.
        c.scene = "🎥".repeat(300);
        let stem = resolve_template("{scene}", &c);
        assert_eq!(stem.chars().count(), MAX_STEM_CHARS);
    }

    #[test]
    fn empty_resolution_falls_back_to_the_classic_name() {
        let mut c = ctx();
        c.scene = "///".to_owned();
        assert_eq!(
            resolve_template("{scene}", &c),
            "Freally Capture 2026-07-12 14-30-05"
        );
    }

    #[test]
    fn whitespace_collapses_and_windows_tail_is_trimmed() {
        assert_eq!(resolve_template("  a   b .", &ctx()), "a b");
    }

    #[test]
    fn validator_accepts_known_tokens_and_plain_text() {
        assert!(template_tokens_valid("{prefix} {date} {time}").is_ok());
        assert!(template_tokens_valid("plain name").is_ok());
        assert!(
            template_tokens_valid("{scene} {profile} {canvas} {marker-count} {counter}").is_ok()
        );
    }

    #[test]
    fn validator_rejects_typos_and_unclosed_braces() {
        assert!(template_tokens_valid("{dat}").is_err());
        assert!(template_tokens_valid("oops {").is_err());
    }
}
