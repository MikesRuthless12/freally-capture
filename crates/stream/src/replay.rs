//! The rolling **replay buffer** ring (Phase 6, TASK-603): bookkeeping for
//! a directory of small encoded MPEG-TS segments that a background encoder
//! (the encode crate's `spawn_replay`) keeps appending to.
//!
//! The Phase 5 lesson stands: raw frames are gigabytes per second — the
//! buffer holds **encoded** segments, so N seconds of history costs N
//! seconds of bitrate on disk and nothing more. Pruning keeps the newest
//! segments that cover the configured window (plus the in-progress one);
//! Save picks the tail and the app concat-copies it into a playable file —
//! never touching the live stream or the recording, which run on their own
//! taps and sessions.

use std::path::{Path, PathBuf};

use fcap_encode::mux::{REPLAY_SEGMENT_EXT, REPLAY_SEGMENT_PREFIX};

/// The ring's segment granularity (and keyframe cadence). Two seconds:
/// fine-grained enough that "the last 30 s" is honest, coarse enough that
/// the muxer/filesystem churn stays negligible.
pub const SEGMENT_SEC: u32 = 2;

/// Parse a ring file's segment index (`replay-000000042.ts` → 42).
pub fn segment_index(path: &Path) -> Option<u64> {
    let name = path.file_name()?.to_str()?;
    let stem = name
        .strip_prefix(REPLAY_SEGMENT_PREFIX)?
        .strip_suffix(&format!(".{REPLAY_SEGMENT_EXT}"))?;
    stem.parse().ok()
}

/// The ring's segments, sorted by ascending index (oldest first).
pub fn list_segments(dir: &Path) -> Vec<(u64, PathBuf)> {
    let mut segments: Vec<(u64, PathBuf)> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            segment_index(&path).map(|index| (index, path))
        })
        .collect();
    segments.sort_by_key(|(index, _)| *index);
    segments
}

/// How many segments the ring keeps: enough complete ones to cover
/// `seconds`, +1 because the oldest kept segment only partially overlaps
/// the window, +1 for the in-progress newest.
pub fn keep_count(seconds: u32, segment_sec: u32) -> usize {
    (seconds.max(1)).div_ceil(segment_sec.max(1)) as usize + 2
}

/// Delete the oldest segments beyond `keep` (best-effort; a file the
/// encoder still holds simply survives to the next prune).
pub fn prune(dir: &Path, keep: usize) {
    let segments = list_segments(dir);
    if segments.len() <= keep {
        return;
    }
    let excess = segments.len() - keep;
    for (_, path) in segments.into_iter().take(excess) {
        let _ = std::fs::remove_file(path);
    }
}

/// The segments a save should stitch: the newest run covering `seconds`
/// (including the in-progress newest — MPEG-TS cuts cleanly mid-write).
pub fn pick_for_save(segments: &[(u64, PathBuf)], seconds: u32, segment_sec: u32) -> Vec<PathBuf> {
    let want = (seconds.max(1)).div_ceil(segment_sec.max(1)) as usize + 1;
    let start = segments.len().saturating_sub(want);
    segments[start..]
        .iter()
        .map(|(_, path)| path.clone())
        .collect()
}

/// The next spawn's `-segment_start_number`, continuing after everything
/// already in the ring so a respawned encoder never collides with (or
/// appears older than) surviving segments.
pub fn next_start_number(dir: &Path) -> u64 {
    list_segments(dir)
        .last()
        .map(|(index, _)| index + 1)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "fcap-replay-test-{tag}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).expect("mkdir");
        dir
    }

    fn seed(dir: &Path, indexes: &[u64]) {
        for index in indexes {
            std::fs::write(dir.join(format!("replay-{index:09}.ts")), b"x").expect("write");
        }
    }

    #[test]
    fn indexes_parse_only_from_ring_names() {
        assert_eq!(segment_index(Path::new("replay-000000042.ts")), Some(42));
        assert_eq!(segment_index(Path::new("replay-0.ts")), Some(0));
        assert_eq!(segment_index(Path::new("recording part001.mkv")), None);
        assert_eq!(segment_index(Path::new("replay-abc.ts")), None);
        assert_eq!(segment_index(Path::new("replay-1.mkv")), None);
    }

    #[test]
    fn pruning_keeps_the_newest_ring() {
        let dir = temp_dir("prune");
        seed(&dir, &[0, 1, 2, 3, 4, 5, 6, 7]);
        prune(&dir, 3);
        let left: Vec<u64> = list_segments(&dir).into_iter().map(|(i, _)| i).collect();
        assert_eq!(left, vec![5, 6, 7]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn save_picks_the_tail_covering_the_window() {
        let dir = temp_dir("pick");
        seed(&dir, &[10, 11, 12, 13, 14, 15]);
        let segments = list_segments(&dir);
        // 6 s at 2 s/segment → 3 complete + the in-progress newest = 4.
        let picked = pick_for_save(&segments, 6, 2);
        let names: Vec<String> = picked
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            names,
            vec![
                "replay-000000012.ts",
                "replay-000000013.ts",
                "replay-000000014.ts",
                "replay-000000015.ts"
            ]
        );
        // A short ring saves everything it has rather than erroring.
        assert_eq!(pick_for_save(&segments[..2], 60, 2).len(), 2);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_ring_keeps_enough_for_the_window_and_the_writer() {
        assert_eq!(keep_count(30, 2), 17); // 15 covering + boundary + in-progress
        assert_eq!(keep_count(5, 2), 5);
        assert_eq!(keep_count(1, 2), 3);
    }

    #[test]
    fn respawns_continue_the_numbering() {
        let dir = temp_dir("start");
        assert_eq!(next_start_number(&dir), 0);
        seed(&dir, &[4, 7]);
        assert_eq!(next_start_number(&dir), 8);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
