//! The owned **InnerTube live-chat client**: reads YouTube chat the exact
//! same way YouTube's own web player does — **no user API key, no Google
//! Cloud project, no OAuth, no sign-in, ever** (the hard rule).
//!
//! Flow: resolve the live video from whatever the user pasted (channel /
//! watch / live_chat URL) → fetch the `live_chat` page like a browser →
//! read the **public web `INNERTUBE_API_KEY`** (the constant youtube.com
//! ships to every visitor), the client version, and the first
//! continuation token off the page → poll
//! `youtubei/v1/live_chat/get_live_chat` and render each
//! `addChatItemAction`. Every parse is **defensive** (schema drift skips
//! fields, never panics); an expired continuation **re-resolves from
//! scratch**; failures back off 1 s → 60 s and never take anything else
//! down.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use serde_json::Value;

use super::{interruptible_sleep, ChatSink};

/// The browser identity + client version fallback — pinned in ONE place so
/// an InnerTube bump is a one-line change.
pub(crate) const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";
const CLIENT_VERSION_FALLBACK: &str = "2.20240701.00.00";

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        // Bounded so a detached ingest thread whose overlay was just removed
        // (its `stop` is set but a poll is mid-flight) self-terminates within
        // this window rather than lingering — keeps stop() on the render
        // thread non-blocking while never stacking long-lived pollers.
        .timeout(Duration::from_secs(8))
        .user_agent(USER_AGENT)
        .build()
}

/// The ingest loop: resolve → bootstrap → poll, with backoff around every
/// layer. Returns when the overlay stops.
pub(crate) fn run(input: &str, sink: &ChatSink, stop: &AtomicBool) {
    let agent = agent();
    let mut backoff = Duration::from_secs(1);
    while !stop.load(Ordering::Relaxed) {
        match session(&agent, input, sink, stop) {
            Ok(()) => return, // stopped cleanly
            Err(err) => {
                eprintln!("chat overlay (youtube): {err} — retrying in {backoff:?}");
                interruptible_sleep(backoff, stop);
                backoff = (backoff * 2).min(Duration::from_secs(60));
            }
        }
    }
}

/// One resolved session: bootstrap the page, then poll until stop/expiry.
fn session(
    agent: &ureq::Agent,
    input: &str,
    sink: &ChatSink,
    stop: &AtomicBool,
) -> Result<(), String> {
    let video_id = resolve_video_id(agent, input)?;
    let page = fetch_text(
        agent,
        &format!("https://www.youtube.com/live_chat?is_popout=1&v={video_id}"),
    )?;
    let api_key = extract_str(&page, "\"INNERTUBE_API_KEY\":\"")
        .ok_or("the live_chat page carried no INNERTUBE_API_KEY (not live yet?)")?;
    let client_version = extract_str(&page, "\"INNERTUBE_CONTEXT_CLIENT_VERSION\":\"")
        .or_else(|| extract_str(&page, "\"clientVersion\":\""))
        .unwrap_or_else(|| CLIENT_VERSION_FALLBACK.to_string());
    let mut continuation = extract_str(&page, "\"continuation\":\"")
        .ok_or("no chat continuation on the page — the stream may not be live")?;

    loop {
        if stop.load(Ordering::Relaxed) {
            return Ok(());
        }
        let body = serde_json::json!({
            "context": { "client": { "clientName": "WEB", "clientVersion": client_version } },
            "continuation": continuation,
        });
        let response: Value = agent
            .post(&format!(
                "https://www.youtube.com/youtubei/v1/live_chat/get_live_chat?key={api_key}&prettyPrint=false"
            ))
            .set("content-type", "application/json")
            .send_string(&body.to_string())
            .map_err(|err| format!("get_live_chat failed: {err}"))?
            .into_json()
            .map_err(|err| format!("get_live_chat returned non-JSON: {err}"))?;

        let chat = &response["continuationContents"]["liveChatContinuation"];
        if chat.is_null() {
            return Err("the chat continuation expired — re-resolving".to_string());
        }
        // Messages: every recognized addChatItemAction; unknown kinds skip.
        if let Some(actions) = chat["actions"].as_array() {
            for action in actions {
                let renderer = &action["addChatItemAction"]["item"]["liveChatTextMessageRenderer"];
                if renderer.is_null() {
                    continue;
                }
                let author = renderer["authorName"]["simpleText"]
                    .as_str()
                    .unwrap_or("viewer");
                let text = runs_to_text(&renderer["message"]["runs"]);
                if !text.is_empty() {
                    sink.push("youtube", author, &text);
                }
            }
        }
        // The next continuation + the server's suggested wait.
        let (next, timeout_ms) =
            next_continuation(chat).ok_or("no next continuation — the stream likely ended")?;
        continuation = next;
        interruptible_sleep(Duration::from_millis(timeout_ms.clamp(500, 15_000)), stop);
    }
}

/// Message runs → plain text: text runs verbatim; emoji runs become the
/// emoji character when YouTube gives one, else their `:shortcut:`.
fn runs_to_text(runs: &Value) -> String {
    let Some(runs) = runs.as_array() else {
        return String::new();
    };
    let mut text = String::new();
    for run in runs {
        if let Some(piece) = run["text"].as_str() {
            text.push_str(piece);
        } else if !run["emoji"].is_null() {
            let emoji = &run["emoji"];
            let id = emoji["emojiId"].as_str().unwrap_or("");
            if !id.is_empty() && id.chars().count() <= 4 {
                text.push_str(id); // a real unicode emoji
            } else if let Some(shortcut) = emoji["shortcuts"][0].as_str() {
                text.push_str(shortcut); // a channel emote → :shortcut:
            }
        }
    }
    text
}

/// The next poll's continuation token + suggested timeout, wherever this
/// InnerTube build put it (invalidation / timed / reload — drift-tolerant).
fn next_continuation(chat: &Value) -> Option<(String, u64)> {
    let continuations = chat["continuations"].as_array()?;
    for entry in continuations {
        for key in [
            "invalidationContinuationData",
            "timedContinuationData",
            "reloadContinuationData",
            "liveChatReplayContinuationData",
        ] {
            let data = &entry[key];
            if let Some(token) = data["continuation"].as_str() {
                let timeout = data["timeoutMs"].as_u64().unwrap_or(2_000);
                return Some((token.to_string(), timeout));
            }
        }
    }
    None
}

/// Whatever the user pasted → the live video id: a `v=`/youtu.be id
/// directly, else the channel's `/live` page is fetched and scanned.
fn resolve_video_id(agent: &ureq::Agent, input: &str) -> Result<String, String> {
    let input = input.trim();
    if let Some(id) = query_param(input, "v=") {
        return Ok(id);
    }
    if let Some(rest) = input.split("youtu.be/").nth(1) {
        let id: String = rest
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
            .collect();
        if !id.is_empty() {
            return Ok(id);
        }
    }
    // A channel URL (or bare @handle): its /live page names the video. A
    // pasted URL is only fetched after its host is confirmed to be YouTube
    // and its scheme https — so a hand-edited/imported scene collection can
    // never point this client at an arbitrary internal host (SSRF).
    let channel_live = if input.starts_with("http") {
        if !is_youtube_https(input) {
            return Err("the chat URL must be an https youtube.com / youtu.be link".to_string());
        }
        format!("{}/live", input.trim_end_matches('/'))
    } else if let Some(handle) = input.strip_prefix('@') {
        format!("https://www.youtube.com/@{handle}/live")
    } else {
        format!("https://www.youtube.com/@{input}/live")
    };
    let page = fetch_text(agent, &channel_live)?;
    extract_str(&page, "\"videoId\":\"")
        .ok_or_else(|| format!("no live video found at {channel_live} — is the channel live?"))
}

/// Whether `url` is an `https://` link whose host is YouTube's — the gate
/// on the only branch that fetches a user-pasted URL verbatim.
fn is_youtube_https(url: &str) -> bool {
    let Some(rest) = url.strip_prefix("https://") else {
        return false;
    };
    // The authority ends at the first '/', '?' or '#'; drop any userinfo.
    let authority = rest.split(['/', '?', '#']).next().unwrap_or("");
    let host = authority
        .rsplit('@')
        .next()
        .unwrap_or("")
        .split(':')
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    host == "youtube.com"
        || host == "youtu.be"
        || host.ends_with(".youtube.com")
        || host.ends_with(".youtu.be")
}

fn query_param(url: &str, key: &str) -> Option<String> {
    let at = url.find(key)?;
    let id: String = url[at + key.len()..]
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    (!id.is_empty()).then_some(id)
}

fn fetch_text(agent: &ureq::Agent, url: &str) -> Result<String, String> {
    agent
        .get(url)
        // The EU consent interstitial would otherwise replace the page.
        .set("cookie", "CONSENT=YES+cb; SOCS=CAI")
        .set("accept-language", "en")
        .call()
        .map_err(|err| format!("GET {url} failed: {err}"))?
        .into_string()
        .map_err(|err| format!("GET {url} unreadable: {err}"))
}

/// The string right after `prefix` up to the closing quote (with `\uXXXX`
/// passed through — tokens/ids are ASCII).
fn extract_str(page: &str, prefix: &str) -> Option<String> {
    let at = page.find(prefix)? + prefix.len();
    let rest = &page[at..];
    let end = rest.find('"')?;
    let value = &rest[..end];
    (!value.is_empty()).then(|| value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_ids_resolve_from_watch_and_short_urls() {
        let agent = agent();
        assert_eq!(
            resolve_video_id(&agent, "https://www.youtube.com/watch?v=abc123XYZ-_").unwrap(),
            "abc123XYZ-_"
        );
        assert_eq!(
            resolve_video_id(&agent, "https://youtu.be/dQw4w9WgXcQ?t=1").unwrap(),
            "dQw4w9WgXcQ"
        );
        assert_eq!(
            resolve_video_id(&agent, "https://www.youtube.com/live_chat?v=xyz789").unwrap(),
            "xyz789"
        );
    }

    #[test]
    fn only_youtube_https_urls_are_fetchable() {
        assert!(is_youtube_https("https://www.youtube.com/@chan/live"));
        assert!(is_youtube_https("https://youtu.be/abc"));
        assert!(is_youtube_https("https://m.youtube.com/watch?v=x"));
        // SSRF attempts: internal hosts, other hosts, http, and userinfo
        // tricks all reject before any request is made.
        assert!(!is_youtube_https("http://169.254.169.254/latest/meta-data"));
        assert!(!is_youtube_https("https://169.254.169.254/live"));
        assert!(!is_youtube_https("https://evil.example/live"));
        assert!(!is_youtube_https("https://youtube.com.evil.example/live"));
        assert!(!is_youtube_https("https://youtube.com@evil.example/live"));
        assert!(!is_youtube_https("file:///etc/passwd"));
    }

    #[test]
    fn a_non_youtube_pasted_url_is_refused_before_any_fetch() {
        let agent = agent();
        let err = resolve_video_id(&agent, "http://169.254.169.254/latest/meta-data")
            .expect_err("internal host rejected");
        assert!(err.contains("youtube.com"));
    }

    #[test]
    fn page_scraping_finds_keys_and_tokens() {
        let page = r#"…"INNERTUBE_API_KEY":"AIzaFakeKey123","continuation":"0ofMyToken=="…"#;
        assert_eq!(
            extract_str(page, "\"INNERTUBE_API_KEY\":\"").unwrap(),
            "AIzaFakeKey123"
        );
        assert_eq!(
            extract_str(page, "\"continuation\":\"").unwrap(),
            "0ofMyToken=="
        );
    }

    #[test]
    fn runs_flatten_text_and_emoji_defensively() {
        let runs = serde_json::json!([
            { "text": "gg " },
            { "emoji": { "emojiId": "🔥", "shortcuts": [":fire:"] } },
            { "emoji": { "emojiId": "UCabc/xyz", "shortcuts": [":customEmote:"] } },
            { "unknownRunKind": { "x": 1 } },
        ]);
        assert_eq!(runs_to_text(&runs), "gg 🔥:customEmote:");
        assert_eq!(runs_to_text(&serde_json::json!(null)), "");
    }

    #[test]
    fn continuations_parse_from_any_known_wrapper() {
        let chat = serde_json::json!({
            "continuations": [
                { "invalidationContinuationData": { "continuation": "tok1", "timeoutMs": 3000 } }
            ]
        });
        assert_eq!(
            next_continuation(&chat).unwrap(),
            ("tok1".to_string(), 3000)
        );
        let drifted = serde_json::json!({ "continuations": [ { "someNewKind": {} } ] });
        assert!(
            next_continuation(&drifted).is_none(),
            "drift skips, never panics"
        );
    }
}
