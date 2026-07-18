//! CAP-N60: scene-collection **pack** (`.fcappack`) export / import.
//!
//! A pack is a single portable zip that carries a whole layout — the scene
//! graph plus every local asset it references (images, media, LUTs, masks,
//! fonts) — so a collection can be shared as one file with no marketplace and
//! no server (charter: **strictly local**, nothing is sent anywhere). It is the
//! sibling of the diagnostics zip ([`crate::diagnostics`]) and reuses the same
//! `zip` dependency already vendored for it.
//!
//! Layout inside the archive:
//! ```text
//! manifest.json          PackManifest — format, app version, name, asset table
//! collection.json        the Collection, serialized with its ORIGINAL paths
//! assets/<i>-<basename>  each bundled local asset's bytes (deduped by path)
//! ```
//!
//! Export never rewrites the collection: the original paths stay in
//! `collection.json` and the manifest maps each one to its archived copy. On
//! import the assets are extracted next to the new collection and every original
//! path is **relinked** (via [`Collection::relink_file`]) to its extracted copy,
//! so the pack lands playable on a machine that never had the source files.
//! Paths that cannot be bundled (URLs, UNC/remote paths, unreadable files) are
//! recorded as `external` and left as-is — honestly surfaced by the existing
//! missing-files doctor on the target machine rather than silently dropped.

use std::collections::{HashMap, HashSet};
use std::io::{Read, Seek, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime, State};

use fcap_scene::{Collection, FileRefKind};

use crate::commands::studio::is_remote;
use crate::profiles::WorkspaceState;
use crate::studio::StudioState;

/// Bump on a breaking change to the pack layout; additive manifest fields ride
/// `#[serde(default)]` instead.
const PACK_FORMAT: u32 = 1;

/// The archive entry names that are structure, not assets.
const MANIFEST_ENTRY: &str = "manifest.json";
const COLLECTION_ENTRY: &str = "collection.json";
const ASSET_DIR: &str = "assets/";

/// Refuse to import a pack larger than this, and cap any single extracted asset
/// at the same ceiling — a defense against a decompression bomb. A real layout
/// with a few video backgrounds is tens–hundreds of MB; 4 GiB is generous.
const MAX_PACK_BYTES: u64 = 4 * 1024 * 1024 * 1024;

/// One asset the pack carries (or honestly reports it could not carry).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackAsset {
    /// The entry name inside the zip (e.g. `assets/0-logo.png`). Empty when the
    /// asset is `external` (nothing was bundled).
    pub archived: String,
    /// The path as it appears in the collection — the relink key on import.
    pub original: String,
    pub kind: FileRefKind,
    /// The source that references it (for the user-facing asset table).
    pub source_name: String,
    pub bytes: u64,
    /// The file could not be bundled (a URL, a UNC/remote path we refuse to
    /// stat, or an unreadable file). Left untouched on import.
    pub external: bool,
    /// Fonts (and anything that may carry licensing) — flagged so a recipient
    /// knows an attribution obligation may ride along. (DoD: packs flag bundled
    /// fonts/assets needing attribution.)
    pub needs_attribution: bool,
}

/// The pack's self-description (its `manifest.json`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackManifest {
    #[serde(default)]
    pub pack_format: u32,
    /// The app version that wrote the pack (informational; import is tolerant).
    pub app_version: String,
    pub collection_name: String,
    pub created: String,
    #[serde(default)]
    pub assets: Vec<PackAsset>,
}

/// Returned to the UI after an export — a compact, honest account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackExportReport {
    pub path: String,
    pub collection_name: String,
    /// Assets whose bytes were bundled.
    pub bundled: usize,
    /// Assets that could not be bundled (URLs / remote / unreadable).
    pub external: usize,
    /// Bundled assets flagged as possibly needing attribution (fonts, …).
    pub attribution: usize,
    pub total_bytes: u64,
}

/// Returned to the UI after an import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackImportReport {
    /// The (possibly de-duplicated) collection name it landed as.
    pub collection_name: String,
    pub scene_count: usize,
    pub source_count: usize,
    /// Assets extracted and relinked into the new collection.
    pub relinked: usize,
    /// Assets the pack recorded as external — now (likely) missing on this
    /// machine; the missing-files doctor will surface them.
    pub external: usize,
    pub attribution: usize,
}

/// A parsed pack in memory (structure only — assets stay as bytes).
pub struct ParsedPack {
    pub manifest: PackManifest,
    pub collection: Collection,
    /// archived entry name → bytes.
    pub assets: HashMap<String, Vec<u8>>,
}

/// Keep only characters that are safe in a zip entry / filename; collapse the
/// rest to `_`. Never returns an empty string.
fn safe_basename(path: &str) -> String {
    // Take the last path component, splitting on either separator (a Windows
    // path can appear on a POSIX host and vice-versa).
    let base = path.rsplit(['/', '\\']).next().unwrap_or(path);
    let cleaned: String = base
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let trimmed = cleaned.trim_matches('.');
    if trimmed.is_empty() {
        "asset".to_string()
    } else {
        trimmed.chars().take(80).collect()
    }
}

/// Fonts carry attribution obligations far more often than a user's own capture
/// media; flag them so a recipient is warned.
fn needs_attribution(kind: FileRefKind) -> bool {
    matches!(kind, FileRefKind::Font)
}

/// Build a pack into `writer` from `collection`. `load` reads an asset's bytes
/// by its (local) path; injecting it keeps this pure and unit-testable without
/// touching the filesystem. A path that `is_remote` (URL / UNC) is never passed
/// to `load` — it is recorded as `external`.
pub fn write_pack<W: Write + Seek>(
    writer: W,
    collection: &Collection,
    collection_name: &str,
    created: &str,
    mut load: impl FnMut(&str) -> std::io::Result<Vec<u8>>,
) -> Result<PackExportReport, String> {
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    let mut zip = zip::ZipWriter::new(writer);

    // The collection first, with its original paths preserved verbatim.
    let collection_json =
        serde_json::to_string_pretty(collection).map_err(|err| format!("serialize: {err}"))?;
    zip.start_file(COLLECTION_ENTRY, options)
        .map_err(|err| format!("zip: {err}"))?;
    zip.write_all(collection_json.as_bytes())
        .map_err(|err| format!("zip: {err}"))?;

    // Enumerate every referenced path once (dedup preserves first-seen order so
    // the archive is deterministic for a given collection).
    let mut assets: Vec<PackAsset> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut total_bytes: u64 = 0;
    let mut bundled = 0usize;
    let mut external = 0usize;
    let mut attribution = 0usize;

    for file_ref in collection.file_refs() {
        if !seen.insert(file_ref.path.clone()) {
            continue; // already bundled under an earlier reference
        }
        let attribution_flag = needs_attribution(file_ref.kind);
        // A URL or a UNC/remote path is never read (statting a UNC path leaks an
        // NTLM hash — see the render-loop guard). Record it as external.
        if is_remote(&file_ref.path) {
            external += 1;
            assets.push(PackAsset {
                archived: String::new(),
                original: file_ref.path,
                kind: file_ref.kind,
                source_name: file_ref.source_name,
                bytes: 0,
                external: true,
                needs_attribution: attribution_flag,
            });
            continue;
        }
        match load(&file_ref.path) {
            Ok(bytes) => {
                let entry = format!(
                    "{ASSET_DIR}{}-{}",
                    assets.len(),
                    safe_basename(&file_ref.path)
                );
                zip.start_file(&entry, options)
                    .map_err(|err| format!("zip: {err}"))?;
                zip.write_all(&bytes).map_err(|err| format!("zip: {err}"))?;
                total_bytes += bytes.len() as u64;
                bundled += 1;
                if attribution_flag {
                    attribution += 1;
                }
                assets.push(PackAsset {
                    archived: entry,
                    original: file_ref.path,
                    kind: file_ref.kind,
                    source_name: file_ref.source_name,
                    bytes: bytes.len() as u64,
                    external: false,
                    needs_attribution: attribution_flag,
                });
            }
            Err(_) => {
                external += 1;
                assets.push(PackAsset {
                    archived: String::new(),
                    original: file_ref.path,
                    kind: file_ref.kind,
                    source_name: file_ref.source_name,
                    bytes: 0,
                    external: true,
                    needs_attribution: attribution_flag,
                });
            }
        }
    }

    let manifest = PackManifest {
        pack_format: PACK_FORMAT,
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        collection_name: collection_name.to_string(),
        created: created.to_string(),
        assets,
    };
    let manifest_json =
        serde_json::to_string_pretty(&manifest).map_err(|err| format!("serialize: {err}"))?;
    zip.start_file(MANIFEST_ENTRY, options)
        .map_err(|err| format!("zip: {err}"))?;
    zip.write_all(manifest_json.as_bytes())
        .map_err(|err| format!("zip: {err}"))?;
    zip.finish().map_err(|err| format!("zip: {err}"))?;

    Ok(PackExportReport {
        path: String::new(), // filled in by the command
        collection_name: manifest.collection_name,
        bundled,
        external,
        attribution,
        total_bytes,
    })
}

/// Read a pack from `reader`: its manifest, its collection (parsed but NOT yet
/// sanitized — the caller sanitizes after relinking), and every bundled asset's
/// bytes. Bounds each entry at [`MAX_PACK_BYTES`] against a decompression bomb.
pub fn read_pack<R: Read + Seek>(reader: R) -> Result<ParsedPack, String> {
    let mut archive = zip::ZipArchive::new(reader).map_err(|err| format!("not a pack: {err}"))?;

    let read_entry = |archive: &mut zip::ZipArchive<R>, name: &str| -> Result<String, String> {
        let file = archive
            .by_name(name)
            .map_err(|_| format!("the pack is missing {name}"))?;
        let mut text = String::new();
        file.take(MAX_PACK_BYTES)
            .read_to_string(&mut text)
            .map_err(|err| format!("read {name}: {err}"))?;
        Ok(text)
    };

    let manifest_text = read_entry(&mut archive, MANIFEST_ENTRY)?;
    let manifest: PackManifest =
        serde_json::from_str(&manifest_text).map_err(|err| format!("bad manifest: {err}"))?;
    let collection_text = read_entry(&mut archive, COLLECTION_ENTRY)?;
    let collection: Collection = serde_json::from_str(&collection_text)
        .map_err(|err| format!("bad collection in pack: {err}"))?;

    // Pull the bytes for every asset the manifest says is bundled.
    let mut assets: HashMap<String, Vec<u8>> = HashMap::new();
    for asset in &manifest.assets {
        if asset.external || asset.archived.is_empty() {
            continue;
        }
        let file = archive.by_name(&asset.archived).map_err(|_| {
            format!(
                "the pack references {} but it is not inside",
                asset.archived
            )
        })?;
        let mut bytes = Vec::new();
        file.take(MAX_PACK_BYTES)
            .read_to_end(&mut bytes)
            .map_err(|err| format!("read {}: {err}", asset.archived))?;
        assets.insert(asset.archived.clone(), bytes);
    }

    Ok(ParsedPack {
        manifest,
        collection,
        assets,
    })
}

/// Extract every bundled asset under `asset_dir` and relink the collection's
/// original paths to the extracted copies. Returns how many references were
/// relinked. `asset_dir` is created if needed.
fn extract_and_relink(
    manifest: &PackManifest,
    assets: &HashMap<String, Vec<u8>>,
    collection: &mut Collection,
    asset_dir: &Path,
) -> Result<usize, String> {
    if assets.is_empty() {
        return Ok(0);
    }
    std::fs::create_dir_all(asset_dir)
        .map_err(|err| format!("could not create {}: {err}", asset_dir.display()))?;
    let mut relinked = 0usize;
    for asset in &manifest.assets {
        if asset.external || asset.archived.is_empty() {
            continue;
        }
        let Some(bytes) = assets.get(&asset.archived) else {
            continue;
        };
        // The on-disk name is a SANITIZED basename of the archived entry — never
        // the raw name. A pack is a shared/untrusted file, and a zip entry name
        // may legally contain `..`, separators, or an absolute/drive-letter path;
        // joining that verbatim would let a crafted manifest escape `asset_dir`
        // and write anywhere (zip-slip → e.g. the Startup folder → code exec).
        // `safe_basename` strips every separator and `..`, so `out` is always a
        // direct child of `asset_dir`. The archived name still embeds a unique
        // index, so legitimate assets keep distinct filenames. Mirrors the
        // sanitization `write_pack` already applies on export.
        let file_name = safe_basename(&asset.archived);
        let out = asset_dir.join(&file_name);
        std::fs::write(&out, bytes)
            .map_err(|err| format!("could not write {}: {err}", out.display()))?;
        let new_path = out.to_string_lossy().to_string();
        relinked += collection.relink_file(&asset.original, &new_path);
    }
    Ok(relinked)
}

// ---------------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------------

/// Export the **active** scene collection to `dest` as a `.fcappack`. Runs off
/// the UI thread; reads the user's own asset files (guarded so a URL/UNC path is
/// bundled as *external*, never statted). `dest` is a user-picked save path.
#[tauri::command]
pub async fn pack_export<R: Runtime>(
    app: AppHandle<R>,
    dest: String,
) -> Result<PackExportReport, String> {
    let collection = app
        .state::<StudioState>()
        .with_collection(|collection| collection.clone());
    let collection_name = app.state::<WorkspaceState>().collection_name();
    let created = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    tauri::async_runtime::spawn_blocking(move || {
        let file = std::fs::File::create(&dest)
            .map_err(|err| format!("could not create {dest}: {err}"))?;
        let mut report = write_pack(
            std::io::BufWriter::new(file),
            &collection,
            &collection_name,
            &created,
            |path| std::fs::read(path),
        )?;
        report.path = dest.clone();
        println!(
            "pack: exported {:?} → {} ({} bundled, {} external)",
            report.collection_name, dest, report.bundled, report.external
        );
        Ok(report)
    })
    .await
    .map_err(|err| format!("pack export task failed: {err}"))?
}

/// Import a `.fcappack` as a NEW scene collection and switch to it (the outgoing
/// collection is saved first — never silently lost, exactly like the OBS
/// import). `path` is a user-picked file — read, never written.
#[tauri::command]
pub async fn pack_import<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, WorkspaceState>,
    path: String,
) -> Result<PackImportReport, String> {
    let size = std::fs::metadata(&path)
        .map_err(|err| format!("could not read {path:?}: {err}"))?
        .len();
    if size > MAX_PACK_BYTES {
        return Err(format!(
            "that pack is too large to import ({} MB)",
            size / (1024 * 1024)
        ));
    }
    let base = state.base()?.to_path_buf();
    state.subdir("collections"); // ensure the dir exists

    // Parse off the caller's stack (spawn_blocking): the file read + unzip.
    let path_for_read = path.clone();
    let mut pack = tauri::async_runtime::spawn_blocking(move || {
        let file = std::fs::File::open(&path_for_read)
            .map_err(|err| format!("could not open {path_for_read:?}: {err}"))?;
        read_pack(std::io::BufReader::new(file))
    })
    .await
    .map_err(|err| format!("pack import task failed: {err}"))??;

    let name = crate::profiles::make_import_name(&base, &pack.manifest.collection_name);
    let asset_dir = base.join("collections").join("assets").join(&name);
    let relinked = extract_and_relink(
        &pack.manifest,
        &pack.assets,
        &mut pack.collection,
        &asset_dir,
    )?;
    pack.collection.sanitize();

    let external = pack.manifest.assets.iter().filter(|a| a.external).count();
    let attribution = pack
        .manifest
        .assets
        .iter()
        .filter(|a| a.needs_attribution && !a.external)
        .count();
    let report = PackImportReport {
        collection_name: name.clone(),
        scene_count: pack.collection.scenes.len(),
        source_count: pack.collection.sources.len(),
        relinked,
        external,
        attribution,
    };

    // Write the new collection to its own file, then switch to it (which saves
    // the outgoing collection first).
    let load = crate::studio::collection_file(&base, &name);
    let json = serde_json::to_string_pretty(&pack.collection)
        .map_err(|err| format!("serialize: {err}"))?;
    crate::settings::write_atomic(&load, &json).map_err(|err| err.to_string())?;

    let current = state.collection_name();
    let save_as = crate::studio::collection_file(&base, &current);
    app.state::<StudioState>()
        .switch_collection_file(&app, save_as, load, false)?;
    state.set_active_collection(name);

    println!(
        "pack: imported {:?} ({} scenes, {} relinked, {} external)",
        report.collection_name, report.scene_count, report.relinked, report.external
    );
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fcap_scene::{Rgba, Source, SourceSettings};
    use std::io::Cursor;

    /// A collection whose only source is a color (no external files): a pack
    /// round-trip must reproduce the scene graph BIT-IDENTICALLY (the DoD's
    /// export→import equality). We compare the parsed collection to the source.
    #[test]
    fn pack_roundtrip_no_assets_is_bit_identical() {
        let mut collection = Collection::new();
        let scene = collection.active_scene().id;
        let color = Source::new(
            "Backdrop",
            SourceSettings::Color {
                color: Rgba::new(10, 20, 30, 255),
                width: 1920,
                height: 1080,
            },
        );
        collection
            .add_item_with_new_source(scene, color)
            .expect("seed color");

        let mut buf = Cursor::new(Vec::new());
        let report = write_pack(&mut buf, &collection, "Show", "t", |_| {
            panic!("no asset should be read for a color-only collection")
        })
        .expect("export");
        assert_eq!(report.bundled, 0);
        assert_eq!(report.external, 0);

        buf.set_position(0);
        let parsed = read_pack(buf).expect("import");
        assert_eq!(
            parsed.collection, collection,
            "the scene graph must round-trip bit-identically"
        );
        assert_eq!(parsed.manifest.collection_name, "Show");
        assert_eq!(parsed.manifest.pack_format, PACK_FORMAT);
    }

    /// An image asset is bundled by bytes and, on import, relinked to its
    /// extracted copy — every other field of the graph unchanged.
    #[test]
    fn pack_bundles_and_relinks_an_asset() {
        let mut collection = Collection::new();
        let scene = collection.active_scene().id;
        let logo = Source::new(
            "Logo",
            SourceSettings::Image {
                path: "C:/media/logo.png".to_string(),
            },
        );
        let (source_id, _item) = collection
            .add_item_with_new_source(scene, logo)
            .expect("seed image");

        let mut buf = Cursor::new(Vec::new());
        let report = write_pack(&mut buf, &collection, "Show", "t", |path| {
            assert_eq!(path, "C:/media/logo.png");
            Ok(b"PNGBYTES".to_vec())
        })
        .expect("export");
        assert_eq!(report.bundled, 1);
        assert_eq!(report.total_bytes, 8);

        buf.set_position(0);
        let parsed = read_pack(buf).expect("import");
        // The manifest recorded exactly one bundled asset with its bytes present.
        assert_eq!(parsed.manifest.assets.len(), 1);
        let archived = parsed.manifest.assets[0].archived.clone();
        assert_eq!(
            parsed.assets.get(&archived).map(Vec::as_slice),
            Some(&b"PNGBYTES"[..])
        );

        let dir = std::env::temp_dir().join(format!("fcap-pack-relink-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let mut relinked_collection = parsed.collection.clone();
        let n = extract_and_relink(
            &parsed.manifest,
            &parsed.assets,
            &mut relinked_collection,
            &dir,
        )
        .expect("relink");
        assert_eq!(n, 1);
        let path = match &relinked_collection.source(source_id).unwrap().settings {
            SourceSettings::Image { path } => path.clone(),
            other => panic!("expected an image source, got {other:?}"),
        };
        assert!(
            path.contains("logo.png"),
            "relinked into the extract dir: {path}"
        );
        assert_ne!(
            path, "C:/media/logo.png",
            "the path must have been rewritten"
        );
        assert_eq!(std::fs::read(&path).expect("extracted bytes"), b"PNGBYTES");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// SECURITY (zip-slip): a crafted pack manifest whose `archived` name tries
    /// to climb out of the extract dir (or is an absolute path) must be reduced
    /// to a safe basename INSIDE `asset_dir` — never written to an arbitrary
    /// location such as the Windows Startup folder.
    #[test]
    fn extract_neutralizes_zip_slip_archived_names() {
        let dir = std::env::temp_dir().join(format!("fcap-pack-slip-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        // Two escape attempts: a relative climb and (Windows) an absolute path.
        let evil_rel = "assets/../../../../../../evil-pack-payload.bat";
        let evil_abs = "C:/evil-pack-abs.bat";
        let mut assets = HashMap::new();
        assets.insert(evil_rel.to_string(), b"payload".to_vec());
        assets.insert(evil_abs.to_string(), b"payload".to_vec());
        let mk = |archived: &str| PackAsset {
            archived: archived.to_string(),
            original: format!("C:/media/{archived}"),
            kind: FileRefKind::Image,
            source_name: "S".into(),
            bytes: 7,
            external: false,
            needs_attribution: false,
        };
        let manifest = PackManifest {
            pack_format: PACK_FORMAT,
            app_version: "x".into(),
            collection_name: "Evil".into(),
            created: "t".into(),
            assets: vec![mk(evil_rel), mk(evil_abs)],
        };
        let mut collection = Collection::new();
        extract_and_relink(&manifest, &assets, &mut collection, &dir).expect("extract");

        // Nothing escaped above the extract dir, and no absolute-path write landed.
        let escaped = dir.parent().unwrap().join("evil-pack-payload.bat");
        assert!(
            !escaped.exists(),
            "zip-slip escaped to {}",
            escaped.display()
        );
        assert!(
            !Path::new("C:/evil-pack-abs.bat").exists(),
            "absolute-path write escaped"
        );
        // Both payloads landed as sanitized basenames inside the extract dir.
        assert!(dir.join("evil-pack-payload.bat").exists());
        assert!(dir.join("evil-pack-abs.bat").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A URL asset is never read (no NTLM/HTTP probe) and is recorded external.
    #[test]
    fn remote_asset_is_external_never_read() {
        let mut collection = Collection::new();
        let scene = collection.active_scene().id;
        let remote = Source::new(
            "Web",
            SourceSettings::Image {
                path: "https://example.com/a.png".to_string(),
            },
        );
        collection
            .add_item_with_new_source(scene, remote)
            .expect("seed remote");

        let mut buf = Cursor::new(Vec::new());
        let report = write_pack(&mut buf, &collection, "Show", "t", |_| {
            panic!("a remote path must never be read")
        })
        .expect("export");
        assert_eq!(report.bundled, 0);
        assert_eq!(report.external, 1);
    }
}
