//! Opening a `.frec` from the OS — double-click, or "Open with Freally
//! Capture". Capture **records** `.frec`; it doesn't play it (that's Freally
//! Player, coming soon), so an opened `.frec` lands in a small dialog offering
//! to **Export it** to a wire format that plays anywhere. The path arrives as
//! a launch argument (cold start) or via the single-instance forward (a second
//! launch while the app is already running).

use std::sync::{Mutex, OnceLock};

fn pending() -> &'static Mutex<Option<String>> {
    static PENDING: OnceLock<Mutex<Option<String>>> = OnceLock::new();
    PENDING.get_or_init(|| Mutex::new(None))
}

/// Remember a `.frec` path the app was opened with, for the UI to pick up.
pub fn store(path: String) {
    *pending()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(path);
}

/// The first existing `.frec` path in `args` — the OS passes the opened file
/// as a launch argument (the exe path + any flags are skipped by the `.frec`
/// extension + existence check).
pub fn frec_in_args<I: IntoIterator<Item = String>>(args: I) -> Option<String> {
    args.into_iter().find(|arg| {
        let path = std::path::Path::new(arg);
        path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("frec"))
            .unwrap_or(false)
            && path.is_file()
    })
}

/// One-shot pickup of a cold-start opened file (the UI calls this on load,
/// once its `open-frec` listener is registered).
#[tauri::command]
pub fn open_frec_pending() -> Option<String> {
    pending()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .take()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frec_in_args_finds_only_existing_frec_paths() {
        // A made-up .frec path that doesn't exist is ignored (must be a file).
        assert_eq!(
            frec_in_args(vec![
                "freally-capture.exe".to_string(),
                "C:/nope/ghost.frec".to_string(),
            ]),
            None
        );
        // A real temp .frec is found; a non-.frec sibling is skipped.
        let dir = std::env::temp_dir();
        let real = dir.join(format!("fcap-openfile-{}.frec", std::process::id()));
        std::fs::write(&real, b"x").unwrap();
        let other = dir.join(format!("fcap-openfile-{}.txt", std::process::id()));
        std::fs::write(&other, b"x").unwrap();
        let got = frec_in_args(vec![
            other.display().to_string(),
            real.display().to_string(),
        ]);
        assert_eq!(got.as_deref(), Some(real.display().to_string().as_str()));
        let _ = std::fs::remove_file(&real);
        let _ = std::fs::remove_file(&other);
    }
}
