//! Post-record **remux**: rewrap an mkv recording as mp4 — stream copy, no
//! re-encode, no quality change — through the labeled ffmpeg component.
//! (mkv is the crash-tolerant recording default; mp4 is what editors and
//! phones expect. Remux gives both without paying for a second encode.)
//! Also the replay buffer's **concat-copy** (Phase 6): stitch the ring's
//! TS segments into one playable file, again without re-encoding.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::ffmpeg::{command, run_with_timeout, Ffmpeg};

/// The input's video codec name ("h264", "hevc", "av1", …), parsed from the
/// `-i` banner — HEVC needs the `hvc1` sample-entry tag in mp4 or Apple
/// players refuse it.
fn probe_video_codec(ffmpeg: &Ffmpeg, input: &Path) -> Option<String> {
    let mut cmd = command(ffmpeg);
    cmd.args(["-hide_banner", "-i"]).arg(input);
    // No output mapped: ffmpeg prints the banner and exits non-zero — the
    // stderr is what we came for.
    let output = run_with_timeout(cmd, Duration::from_secs(30)).ok()?;
    let banner = String::from_utf8_lossy(&output.stderr);
    for line in banner.lines() {
        if let Some(at) = line.find("Video: ") {
            let codec: String = line[at + 7..]
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect();
            if !codec.is_empty() {
                return Some(codec);
            }
        }
    }
    None
}

/// `name.mkv` → a non-colliding sibling `name.mp4` / `name (2).mp4` / ….
fn unique_mp4_sibling(input: &Path) -> PathBuf {
    let stem = input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("recording");
    let first = input.with_file_name(format!("{stem}.mp4"));
    if !first.exists() {
        return first;
    }
    for index in 2..1000 {
        let candidate = input.with_file_name(format!("{stem} ({index}).mp4"));
        if !candidate.exists() {
            return candidate;
        }
    }
    input.with_file_name(format!("{stem}.remux.mp4"))
}

/// Remux `input` (mkv) to a sibling mp4: `-map 0 -c copy`, faststart, and
/// the `hvc1` tag when the video is HEVC. Blocking — minutes on very long
/// recordings (still pure I/O; nothing is re-encoded).
pub fn remux_to_mp4(ffmpeg: &Ffmpeg, input: &Path) -> Result<PathBuf, String> {
    if input.extension().and_then(|ext| ext.to_str()) != Some("mkv") {
        return Err("remux takes an mkv recording".to_string());
    }
    if !input.is_file() {
        return Err(format!("{} is not a file", input.display()));
    }
    let output = unique_mp4_sibling(input);
    let codec = probe_video_codec(ffmpeg, input);

    let mut cmd = command(ffmpeg);
    cmd.args(["-hide_banner", "-v", "error", "-y", "-i"])
        .arg(input);
    cmd.args(["-map", "0", "-c", "copy"]);
    if codec.as_deref() == Some("hevc") {
        cmd.args(["-tag:v", "hvc1"]);
    }
    cmd.args(["-movflags", "+faststart", "-f", "mp4"])
        .arg(&output);

    let result = run_with_timeout(cmd, Duration::from_secs(30 * 60))?;
    if !result.status.success() {
        let _ = std::fs::remove_file(&output);
        let stderr = String::from_utf8_lossy(&result.stderr);
        return Err(format!(
            "remux failed ({}): {}",
            result.status,
            stderr.trim().chars().take(300).collect::<String>()
        ));
    }
    Ok(output)
}

/// One `file '…'` line for the concat demuxer's list, with the only escape
/// it honors (a quote closes, `\'` re-opens: `'` → `'\''`).
fn concat_list_line(path: &Path) -> String {
    let text = path.to_string_lossy().replace('\'', "'\\''");
    format!("file '{text}'\n")
}

/// Stitch `segments` (MPEG-TS, same codec parameters — the replay ring's
/// output) into `out` with `-c copy`: no re-encode, sub-second even for
/// minutes of buffer. The newest segment may still be mid-write; TS cuts
/// cleanly so the copy simply takes what has been flushed.
pub fn concat_copy(ffmpeg: &Ffmpeg, segments: &[PathBuf], out: &Path) -> Result<(), String> {
    if segments.is_empty() {
        return Err("the replay buffer is still empty — nothing to save yet".to_string());
    }
    let list_path = out.with_extension("concat.txt");
    {
        let mut list = std::fs::File::create(&list_path)
            .map_err(|err| format!("could not write the concat list: {err}"))?;
        for segment in segments {
            list.write_all(concat_list_line(segment).as_bytes())
                .map_err(|err| format!("could not write the concat list: {err}"))?;
        }
    }
    let mut cmd = command(ffmpeg);
    cmd.args([
        "-hide_banner",
        "-v",
        "error",
        "-y",
        "-f",
        "concat",
        "-safe",
        "0",
        "-i",
    ])
    .arg(&list_path);
    cmd.args(["-map", "0", "-c", "copy"]).arg(out);
    let result = run_with_timeout(cmd, Duration::from_secs(10 * 60));
    let _ = std::fs::remove_file(&list_path);
    let result = result?;
    if !result.status.success() {
        let _ = std::fs::remove_file(out);
        let stderr = String::from_utf8_lossy(&result.stderr);
        return Err(format!(
            "replay save failed ({}): {}",
            result.status,
            stderr.trim().chars().take(300).collect::<String>()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_list_lines_escape_quotes() {
        assert_eq!(
            concat_list_line(Path::new("C:/tmp/replay-000000001.ts")),
            "file 'C:/tmp/replay-000000001.ts'\n"
        );
        assert_eq!(
            concat_list_line(Path::new("/tmp/it's here.ts")),
            "file '/tmp/it'\\''s here.ts'\n"
        );
    }

    #[test]
    fn concat_of_nothing_is_refused() {
        let fake = Ffmpeg {
            path: PathBuf::from("ffmpeg-not-real"),
            version: "test".to_string(),
        };
        let err = concat_copy(&fake, &[], Path::new("out.mkv")).unwrap_err();
        assert!(err.contains("empty"));
    }

    #[test]
    fn sibling_names_avoid_collisions() {
        let dir = std::env::temp_dir().join(format!(
            "fcap-remux-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let input = dir.join("take.mkv");
        std::fs::write(&input, b"x").expect("write");

        let first = unique_mp4_sibling(&input);
        assert_eq!(first.file_name().unwrap(), "take.mp4");
        std::fs::write(&first, b"x").expect("write");
        let second = unique_mp4_sibling(&input);
        assert_eq!(second.file_name().unwrap(), "take (2).mp4");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn non_mkv_inputs_are_refused() {
        let fake = Ffmpeg {
            path: PathBuf::from("ffmpeg-not-real"),
            version: "test".to_string(),
        };
        let err = remux_to_mp4(&fake, Path::new("clip.mp4")).unwrap_err();
        assert!(err.contains("mkv"));
    }
}
