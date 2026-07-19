//! Native text-to-speech fallback for the teleprompter "read aloud" pace preview.
//!
//! Windows and macOS speak from the UI via the WebView's `speechSynthesis`, which
//! uses the OS voices (Windows OneCore/SAPI, macOS AVSpeechSynthesis). Linux
//! WebKitGTK often ships without a speech backend, so there the UI falls back to
//! these commands, which shell out to the user's installed Speech Dispatcher
//! (`spd-say`) or `espeak-ng`. We deliberately do NOT bundle those (GPL) engines —
//! calling the system daemon over the CLI keeps the app's own licensing clean.

/// Speak `text` at `rate` (a Web-Speech-style multiplier, 1.0 = normal pace).
/// Linux only; on other platforms this returns an error so the caller uses the
/// WebView's `speechSynthesis` instead.
#[tauri::command]
pub fn tts_speak(text: String, rate: f32) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        // Clear anything already queued so a new read replaces the old one.
        let _ = Command::new("spd-say").arg("-C").status();
        // speech-dispatcher's rate is -100..=100; map the multiplier around 1.0.
        let spd_rate = (((rate - 1.0) * 50.0).round() as i32).clamp(-100, 100);
        if Command::new("spd-say")
            .args(["-r", &spd_rate.to_string(), "--", &text])
            .spawn()
            .is_ok()
        {
            return Ok(());
        }
        // Fall back to espeak-ng (words-per-minute ~= rate * 175).
        let wpm = ((rate * 175.0).round() as i32).clamp(80, 500);
        Command::new("espeak-ng")
            .args(["-s", &wpm.to_string(), &text])
            .spawn()
            .map(|_| ())
            .map_err(|e| {
                format!("no Linux TTS found (install speech-dispatcher or espeak-ng): {e}")
            })
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (text, rate);
        Err("native TTS is the Linux fallback only; use the WebView speechSynthesis here".into())
    }
}

/// Stop any in-progress native speech (Linux Speech Dispatcher).
#[tauri::command]
pub fn tts_stop() {
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("spd-say").arg("-C").status();
    }
}
