//! What the **About** page shows (TASK-907).
//!
//! Everything here that *can* come from the build comes from the build:
//! `CARGO_PKG_VERSION`, `CARGO_PKG_AUTHORS`, `CARGO_PKG_REPOSITORY` are baked in
//! by cargo, so the About page can never drift from what actually shipped. The
//! two dates cannot be derived — the first commit is not in the binary and the
//! 1.0.0 release has not happened — so they are consts, kept beside each other.

use serde::Serialize;

/// The repo's first commit, as an ISO date. Not derivable at build time: `git`
/// is not guaranteed present, and a release tarball has no history at all.
const PROJECT_STARTED: &str = "2026-06-30";

/// Filled in at the 1.0.0 cut, and not before. `None` renders as "not yet" —
/// an About page claiming a stable release that has not happened is worse than
/// one admitting it is pre-1.0.
const FIRST_STABLE_RELEASED: Option<&str> = None;

const HOMEPAGE: &str = "https://mikesruthless12.github.io/freally-capture/";

/// Everything the About panel renders. Links are opened by the OS, never fetched.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
    /// The running build's version — the same string the updater compares.
    pub version: String,
    /// `Mike Weaver <mythodikalone@gmail.com>`, straight from `CARGO_PKG_AUTHORS`.
    pub authors: String,
    pub project_started: String,
    /// `None` until 1.0.0 ships.
    pub first_stable_released: Option<String>,
    pub copyright: String,
    pub homepage: String,
    pub repository: String,
    pub issues: String,
    /// `windows` / `macos` / `linux`, and the CPU architecture.
    pub os: String,
    pub arch: String,
    /// Rust's own version is not available at runtime without a build script,
    /// so this is the target triple, which is.
    pub target: String,
}

#[tauri::command]
pub fn build_info() -> BuildInfo {
    let repository = env!("CARGO_PKG_REPOSITORY").to_string();
    BuildInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        authors: env!("CARGO_PKG_AUTHORS").to_string(),
        project_started: PROJECT_STARTED.to_string(),
        first_stable_released: FIRST_STABLE_RELEASED.map(str::to_string),
        copyright: format!("© 2026 {} — All Rights Reserved", author_name()),
        homepage: HOMEPAGE.to_string(),
        issues: format!("{repository}/issues"),
        repository,
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        target: format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS),
    }
}

/// The full changelog, embedded at build time so **What's New** can show the
/// running version's notes offline — no network, and always exactly the notes
/// that shipped in this binary.
const CHANGELOG: &str = include_str!("../../CHANGELOG.md");

/// The running version and its changelog section, for the What's-New dialog.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseNotes {
    pub version: String,
    /// The `## [version]` section of `CHANGELOG.md`, as Markdown. `None` if the
    /// running version has no section yet (a dev build ahead of the changelog).
    pub notes: Option<String>,
}

/// What **Help → What's New** shows: this build's changelog, read in-app instead
/// of opening the web changelog in a browser.
#[tauri::command]
pub fn release_notes() -> ReleaseNotes {
    let version = env!("CARGO_PKG_VERSION");
    ReleaseNotes {
        version: version.to_string(),
        notes: changelog_section(CHANGELOG, version).map(str::to_string),
    }
}

/// Extract the `## [version] …` section from a Keep-a-Changelog document: from
/// its heading through to just before the next `## [` heading (or the end).
fn changelog_section<'a>(changelog: &'a str, version: &str) -> Option<&'a str> {
    let heading = format!("## [{version}]");
    let start = changelog.find(&heading)?;
    let body = &changelog[start..];
    let end = body[heading.len()..]
        .find("\n## [")
        .map(|offset| heading.len() + offset)
        .unwrap_or(body.len());
    Some(body[..end].trim_end())
}

/// `Mike Weaver <mail@…>` → `Mike Weaver`. Cargo joins multiple authors with
/// `:`, so take the first; the copyright line names a person, not a list.
fn author_name() -> &'static str {
    env!("CARGO_PKG_AUTHORS")
        .split(':')
        .next()
        .unwrap_or("")
        .split('<')
        .next()
        .unwrap_or("")
        .trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn author_name_drops_the_email_and_extra_authors() {
        assert_eq!(author_name(), "Mike Weaver");
    }

    #[test]
    fn build_info_reports_the_running_build() {
        let info = build_info();
        assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
        assert!(info.repository.starts_with("https://github.com/"));
        assert!(info.issues.ends_with("/issues"));
        assert!(info.copyright.contains("Mike Weaver"));
        assert!(
            !info.copyright.contains('<'),
            "no email in the copyright line"
        );
    }

    #[test]
    fn changelog_section_extracts_one_version() {
        let md = "# Changelog\n\n## [Unreleased]\n\n- wip\n\n## [0.301.0] — 2026-07-14 (X)\n\n### Added\n- a thing\n\n## [0.300.0] — 2026-07-13\n\n- older\n";
        let section = changelog_section(md, "0.301.0").expect("section");
        assert!(section.starts_with("## [0.301.0]"));
        assert!(section.contains("a thing"));
        assert!(!section.contains("older"), "stops at the next heading");
        assert!(!section.contains("wip"), "does not bleed from Unreleased");
        assert!(changelog_section(md, "9.9.9").is_none());
    }

    /// The running build must have a changelog section — a release with no notes
    /// is one nobody can read. This also guards the version bump: it fails until
    /// `CHANGELOG.md` gains the new version's section.
    #[test]
    fn running_version_has_release_notes() {
        assert!(
            release_notes().notes.is_some(),
            "CHANGELOG.md is missing a section for {}",
            env!("CARGO_PKG_VERSION")
        );
    }

    /// The About page must not claim a stable release before one exists. When
    /// 1.0.0 ships, set `FIRST_STABLE_RELEASED` and this test changes with it.
    #[test]
    fn first_stable_is_unset_until_one_dot_oh() {
        let pre_release = env!("CARGO_PKG_VERSION").starts_with("0.");
        assert_eq!(
            FIRST_STABLE_RELEASED.is_none(),
            pre_release,
            "FIRST_STABLE_RELEASED must be filled in at the 1.0.0 cut, and only then"
        );
    }
}
