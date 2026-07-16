//! An owned streaming WAV writer (CAP-N38 audio-only recording) — 16-bit PCM,
//! the most universally-playable format. Nothing fetched, no ffmpeg: the header
//! is written with placeholder sizes and patched on finalize, so a crash still
//! leaves a file whose header can be repaired from its length.

use std::fs::File;
use std::io::{self, BufWriter, Seek, SeekFrom, Write};
use std::path::Path;

/// A streaming 16-bit PCM WAV writer. Interleaved samples for `channels`.
pub struct WavWriter {
    file: BufWriter<File>,
    data_bytes: u32,
}

impl WavWriter {
    /// Create `path` and write the 44-byte header (sizes patched on finalize).
    pub fn create(path: &Path, channels: u16, sample_rate: u32) -> io::Result<Self> {
        let mut file = BufWriter::new(File::create(path)?);
        let byte_rate = sample_rate * channels as u32 * 2;
        let block_align = channels * 2;
        file.write_all(b"RIFF")?;
        file.write_all(&0u32.to_le_bytes())?; // RIFF chunk size — patched
        file.write_all(b"WAVE")?;
        file.write_all(b"fmt ")?;
        file.write_all(&16u32.to_le_bytes())?; // fmt chunk size
        file.write_all(&1u16.to_le_bytes())?; // PCM
        file.write_all(&channels.to_le_bytes())?;
        file.write_all(&sample_rate.to_le_bytes())?;
        file.write_all(&byte_rate.to_le_bytes())?;
        file.write_all(&block_align.to_le_bytes())?;
        file.write_all(&16u16.to_le_bytes())?; // bits per sample
        file.write_all(b"data")?;
        file.write_all(&0u32.to_le_bytes())?; // data chunk size — patched
        Ok(Self {
            file,
            data_bytes: 0,
        })
    }

    /// Append interleaved f32 samples (clamped to 16-bit PCM).
    pub fn write_f32(&mut self, samples: &[f32]) -> io::Result<()> {
        let mut bytes = Vec::with_capacity(samples.len() * 2);
        for &sample in samples {
            let clamped = (sample.clamp(-1.0, 1.0) * 32_767.0) as i16;
            bytes.extend_from_slice(&clamped.to_le_bytes());
        }
        self.file.write_all(&bytes)?;
        self.data_bytes = self.data_bytes.saturating_add(bytes.len() as u32);
        Ok(())
    }

    /// Patch the RIFF + data sizes and flush.
    pub fn finalize(mut self) -> io::Result<()> {
        self.file.flush()?;
        let riff_size = 36u32.saturating_add(self.data_bytes);
        self.file.seek(SeekFrom::Start(4))?;
        self.file.write_all(&riff_size.to_le_bytes())?;
        self.file.seek(SeekFrom::Start(40))?;
        self.file.write_all(&self.data_bytes.to_le_bytes())?;
        self.file.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_a_playable_header_and_data() {
        let path = std::env::temp_dir().join(format!(
            "fcap-wav-{}-{}.wav",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut writer = WavWriter::create(&path, 2, 48_000).unwrap();
        // 480 stereo frames = 960 samples = 1920 data bytes.
        writer.write_f32(&vec![0.5f32; 960]).unwrap();
        writer.finalize().unwrap();

        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(&bytes[0..4], b"RIFF");
        assert_eq!(&bytes[8..12], b"WAVE");
        assert_eq!(&bytes[36..40], b"data");
        let data_size = u32::from_le_bytes([bytes[40], bytes[41], bytes[42], bytes[43]]);
        assert_eq!(data_size, 1_920, "480 frames × 2ch × 2 bytes");
        let riff_size = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        assert_eq!(riff_size as usize, bytes.len() - 8);
        // Sample 0.5 → 16383/16384-ish.
        let first = i16::from_le_bytes([bytes[44], bytes[45]]);
        assert!((first - 16_383).abs() <= 1);
        let _ = std::fs::remove_file(&path);
    }
}
