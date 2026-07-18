//! CAP-N65: theme editor file I/O — export/import a `.fctheme` (the theme as
//! JSON). The visual editor + WCAG contrast checks live in the UI; this is just
//! the shareable-file layer. Strictly local; validated on both directions so a
//! hand-edited `.fctheme` can never inject a CSS rule.

use std::path::Path;

use crate::settings::ThemeSettings;

/// Write the current theme to `path` as a `.fctheme`.
#[tauri::command]
pub fn theme_export(path: String, theme: ThemeSettings) -> Result<(), String> {
    theme.validate()?;
    let json = serde_json::to_string_pretty(&theme).map_err(|err| err.to_string())?;
    crate::settings::write_atomic(Path::new(&path), &json).map_err(|err| err.to_string())
}

/// Read a `.fctheme` back (validated).
#[tauri::command]
pub fn theme_import(path: String) -> Result<ThemeSettings, String> {
    let text =
        std::fs::read_to_string(&path).map_err(|err| format!("could not read {path:?}: {err}"))?;
    let theme: ThemeSettings =
        serde_json::from_str(&text).map_err(|err| format!("not a valid .fctheme: {err}"))?;
    theme.validate()?;
    Ok(theme)
}
