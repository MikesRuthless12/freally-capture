//! Anonymous Twitch chat over IRC — **no credentials, ever** (the hard
//! rule): Twitch's chat servers accept the well-known `justinfan<digits>`
//! guest nick read-only, exactly what a logged-out twitch.tv visitor gets.
//! Plain TCP (the anonymous read-only feed is public data); reconnects
//! with backoff; PINGs answered; unknown lines skipped, never panicked on.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use super::{interruptible_sleep, ChatSink};

const HOST: &str = "irc.chat.twitch.tv:6667";

pub(crate) fn run(channel: &str, sink: &ChatSink, stop: &AtomicBool) {
    // A Twitch login is `[a-z0-9_]{1,25}` — pin the channel to that charset
    // before it enters the IRC handshake so a hand-edited/imported scene
    // collection can never inject `\r\n`-separated extra IRC commands onto
    // the anonymous session.
    let channel = sanitize_channel(channel);
    if channel.is_empty() {
        eprintln!("chat overlay (twitch): invalid channel name — ingest disabled");
        return;
    }
    let mut backoff = Duration::from_secs(1);
    while !stop.load(Ordering::Relaxed) {
        match session(&channel, sink, stop) {
            Ok(()) => return,
            Err(err) => {
                eprintln!("chat overlay (twitch): {err} — retrying in {backoff:?}");
                interruptible_sleep(backoff, stop);
                backoff = (backoff * 2).min(Duration::from_secs(60));
            }
        }
    }
}

fn session(channel: &str, sink: &ChatSink, stop: &AtomicBool) -> Result<(), String> {
    let mut stream =
        TcpStream::connect(HOST).map_err(|err| format!("connect {HOST} failed: {err}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .map_err(|err| err.to_string())?;
    // The anonymous guest nick — digits vary so parallel overlays never
    // collide on one nick.
    let nick = format!("justinfan{}", 10_000 + (std::process::id() % 80_000));
    stream
        .write_all(format!("NICK {nick}\r\nJOIN #{channel}\r\n").as_bytes())
        .map_err(|err| format!("IRC handshake failed: {err}"))?;

    let mut writer = stream.try_clone().map_err(|err| err.to_string())?;
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    loop {
        if stop.load(Ordering::Relaxed) {
            return Ok(());
        }
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => return Err("the IRC server closed the connection".to_string()),
            Ok(_) => {
                if line.starts_with("PING") {
                    let _ = writer.write_all(b"PONG :tmi.twitch.tv\r\n");
                    continue;
                }
                if let Some((user, text)) = parse_privmsg(&line) {
                    sink.push("twitch", user, text);
                }
            }
            Err(err)
                if err.kind() == std::io::ErrorKind::WouldBlock
                    || err.kind() == std::io::ErrorKind::TimedOut =>
            {
                continue; // read timeout — loop to honor stop
            }
            Err(err) => return Err(format!("IRC read failed: {err}")),
        }
    }
}

/// Keep only a valid Twitch login's characters (`[a-z0-9_]`, ≤25) so the
/// channel can never carry IRC control bytes into the handshake.
fn sanitize_channel(channel: &str) -> String {
    channel
        .trim()
        .trim_start_matches('#')
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .take(25)
        .collect()
}

/// `:nick!nick@nick.tmi.twitch.tv PRIVMSG #chan :message` → (nick, message).
fn parse_privmsg(line: &str) -> Option<(&str, &str)> {
    let line = line.trim_end();
    let rest = line.strip_prefix(':')?;
    let bang = rest.find('!')?;
    let user = &rest[..bang];
    let privmsg_at = rest.find(" PRIVMSG ")?;
    let after = &rest[privmsg_at + " PRIVMSG ".len()..];
    let colon = after.find(" :")?;
    Some((user, &after[colon + 2..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_names_pin_to_the_login_charset() {
        assert_eq!(sanitize_channel("#MikeCam_1"), "mikecam_1");
        // CRLF-injection attempt: everything but the login charset drops, so
        // the `\r\nJOIN` payload can never reach the handshake.
        assert_eq!(sanitize_channel("foo\r\nJOIN #victim"), "foojoinvictim");
        assert_eq!(sanitize_channel("  \r\n  "), "");
        assert!(!sanitize_channel("anychannel").contains(['\r', '\n', ' ']));
    }

    #[test]
    fn privmsg_lines_parse_and_noise_skips() {
        let (user, text) =
            parse_privmsg(":somefan!somefan@somefan.tmi.twitch.tv PRIVMSG #mike :hello world!\r\n")
                .expect("parses");
        assert_eq!(user, "somefan");
        assert_eq!(text, "hello world!");

        assert!(parse_privmsg("PING :tmi.twitch.tv").is_none());
        assert!(parse_privmsg(":tmi.twitch.tv 001 justinfan :Welcome").is_none());
        assert!(parse_privmsg("garbage").is_none());
    }
}
