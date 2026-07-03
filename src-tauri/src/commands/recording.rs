//! The recording + encoder command surface (Phase 4).
//!
//! `encoders_list` feeds the encoder picker: the detected hardware encoders
//! (probe honesty: *offered* until the on-demand ffmpeg component confirms
//! them) plus the always-available x264 fallback. The recording session
//! commands land with the recorder (P4.3/P4.5).

use fcap_encode::Catalog;

/// Detect the encoder catalog (GPU enumeration + per-OS rules). Async — the
/// first wgpu enumeration can take a moment and must not block the UI.
#[tauri::command]
pub async fn encoders_list() -> Result<Catalog, String> {
    tauri::async_runtime::spawn_blocking(Catalog::detect)
        .await
        .map_err(|err| format!("encoder detection task failed: {err}"))
}
