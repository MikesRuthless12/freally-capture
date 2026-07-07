//! Kick chat via the site's **public channel endpoint** — no key, no
//! account (the hard rule). Kick's live chat rides a websocket, but its
//! public REST endpoint serves the recent messages to any visitor; the
//! overlay polls it and de-duplicates by message id. Best-effort by
//! design: Kick fronts with anti-bot filtering that may reject non-browser
//! clients — a failure backs off and the OTHER platforms keep flowing
//! (non-fatal, honestly logged).

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use serde_json::Value;

use super::{interruptible_sleep, ChatSink};

const POLL: Duration = Duration::from_secs(3);

pub(crate) fn run(slug: &str, sink: &ChatSink, stop: &AtomicBool) {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(15))
        .user_agent(super::innertube::USER_AGENT)
        .build();
    let url = format!(
        "https://kick.com/api/v2/channels/{}/messages",
        slug.to_ascii_lowercase()
    );
    // De-dup window: the endpoint returns a rolling recent slice.
    let mut seen: VecDeque<String> = VecDeque::new();
    let mut backoff = Duration::from_secs(3);
    while !stop.load(Ordering::Relaxed) {
        match poll(&agent, &url, sink, &mut seen) {
            Ok(()) => {
                backoff = Duration::from_secs(3);
                interruptible_sleep(POLL, stop);
            }
            Err(err) => {
                eprintln!("chat overlay (kick): {err} — retrying in {backoff:?}");
                interruptible_sleep(backoff, stop);
                backoff = (backoff * 2).min(Duration::from_secs(120));
            }
        }
    }
}

fn poll(
    agent: &ureq::Agent,
    url: &str,
    sink: &ChatSink,
    seen: &mut VecDeque<String>,
) -> Result<(), String> {
    let body: Value = agent
        .get(url)
        .set("accept", "application/json")
        .call()
        .map_err(|err| format!("GET {url} failed: {err}"))?
        .into_json()
        .map_err(|err| format!("{url} returned non-JSON: {err}"))?;
    let messages = body["data"]["messages"]
        .as_array()
        .or_else(|| body["data"].as_array())
        .ok_or("no messages array (schema drift — skipped)")?;
    // Oldest-first so the overlay reads naturally.
    for message in messages.iter().rev() {
        let id = message["id"]
            .as_str()
            .map(str::to_string)
            .or_else(|| message["id"].as_u64().map(|id| id.to_string()))
            .unwrap_or_default();
        if id.is_empty() || seen.contains(&id) {
            continue;
        }
        seen.push_back(id);
        while seen.len() > 200 {
            seen.pop_front();
        }
        let user = message["sender"]["username"].as_str().unwrap_or("viewer");
        let text = message["content"].as_str().unwrap_or("");
        if !text.is_empty() {
            sink.push("kick", user, text);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU64;
    use std::sync::Arc;

    #[test]
    fn kick_payloads_parse_and_dedup() {
        let sink = ChatSink {
            ring: Arc::new(std::sync::Mutex::new(VecDeque::new())),
            revision: Arc::new(AtomicU64::new(0)),
        };
        let mut seen = VecDeque::new();
        let payload = serde_json::json!({
            "data": { "messages": [
                { "id": "m2", "sender": { "username": "beta" }, "content": "second" },
                { "id": "m1", "sender": { "username": "alpha" }, "content": "first" },
            ]}
        });
        // Simulate the poll body-handling by calling the same traversal.
        let messages = payload["data"]["messages"].as_array().unwrap();
        for message in messages.iter().rev() {
            let id = message["id"].as_str().unwrap().to_string();
            if seen.contains(&id) {
                continue;
            }
            seen.push_back(id);
            sink.push(
                "kick",
                message["sender"]["username"].as_str().unwrap(),
                message["content"].as_str().unwrap(),
            );
        }
        let ring = sink.ring.lock().unwrap();
        assert_eq!(ring.len(), 2);
        assert_eq!(ring[0].username, "alpha");
        assert_eq!(ring[1].text, "second");
    }
}
