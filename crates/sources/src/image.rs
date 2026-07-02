//! The Image source: decode a still image file to an RGBA frame.
//!
//! Also serves the filter chain — LUT strips aside, the Image Mask filter's
//! mask files come through [`load_image_rgba`] too.

use std::path::Path;

use fcap_capture::Frame;

use crate::static_source::{check_dimension, rgba_frame, StaticSourceError};

/// Decode `path` (PNG/JPEG/BMP/GIF/WebP/TIFF…) into a straight-alpha RGBA
/// frame. Animated formats contribute their first frame — animation is Media
/// territory, and the picker says so.
pub fn load_image_rgba(path: &Path) -> Result<Frame, StaticSourceError> {
    let display = path.display().to_string();
    let bytes = std::fs::read(path).map_err(|err| StaticSourceError::Io {
        path: display.clone(),
        message: err.to_string(),
    })?;
    let decoded = image::load_from_memory(&bytes).map_err(|err| StaticSourceError::Decode {
        path: display.clone(),
        message: err.to_string(),
    })?;
    let rgba = decoded.to_rgba8();
    check_dimension("image width", rgba.width())?;
    check_dimension("image height", rgba.height())?;
    let (width, height) = (rgba.width(), rgba.height());
    Ok(rgba_frame(width, height, rgba.into_raw()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_png(tag: &str, pixels: &[[u8; 4]], width: u32, height: u32) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "fcap-image-test-{}-{nanos}-{tag}.png",
            std::process::id()
        ));
        let mut buffer = image::RgbaImage::new(width, height);
        for (index, px) in pixels.iter().enumerate() {
            let x = index as u32 % width;
            let y = index as u32 / width;
            buffer.put_pixel(x, y, image::Rgba(*px));
        }
        buffer.save(&path).expect("write test png");
        path
    }

    #[test]
    fn decodes_pixels_and_alpha() {
        let path = temp_png(
            "decode",
            &[
                [255, 0, 0, 255],
                [0, 255, 0, 128],
                [0, 0, 255, 255],
                [0, 0, 0, 0],
            ],
            2,
            2,
        );
        let frame = load_image_rgba(&path).expect("load");
        assert_eq!((frame.width, frame.height), (2, 2));
        assert_eq!(&frame.data[0..4], &[255, 0, 0, 255]);
        assert_eq!(&frame.data[4..8], &[0, 255, 0, 128], "alpha survives");
        assert_eq!(&frame.data[12..16], &[0, 0, 0, 0]);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn missing_files_error_with_the_path() {
        let err = load_image_rgba(Path::new("Z:/definitely/not/here.png")).unwrap_err();
        let message = err.to_string();
        assert!(
            message.contains("not/here.png"),
            "names the file: {message}"
        );
    }

    #[test]
    fn garbage_bytes_error_as_decode() {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "fcap-image-test-{}-{nanos}-garbage.png",
            std::process::id()
        ));
        std::fs::write(&path, b"this is not an image").expect("write");
        assert!(matches!(
            load_image_rgba(&path),
            Err(StaticSourceError::Decode { .. })
        ));
        let _ = std::fs::remove_file(&path);
    }
}
