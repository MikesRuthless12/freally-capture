//! The Color source: a solid RGBA block at a chosen size.

use fcap_capture::Frame;

use crate::static_source::{check_dimension, rgba_frame, StaticSourceError};

/// Render a solid color block. `rgba` is straight (unpremultiplied) alpha —
/// the compositor blends it as such.
pub fn solid_color_frame(
    rgba: [u8; 4],
    width: u32,
    height: u32,
) -> Result<Frame, StaticSourceError> {
    check_dimension("color source width", width)?;
    check_dimension("color source height", height)?;
    let mut data = Vec::with_capacity(width as usize * height as usize * 4);
    for _ in 0..width as usize * height as usize {
        data.extend_from_slice(&rgba);
    }
    Ok(rgba_frame(width, height, data))
}

#[cfg(test)]
mod tests {
    use super::*;
    use fcap_capture::PixelFormat;

    #[test]
    fn fills_the_requested_size() {
        let frame = solid_color_frame([10, 20, 30, 128], 4, 2).expect("render");
        assert_eq!(frame.width, 4);
        assert_eq!(frame.height, 2);
        assert_eq!(frame.format, PixelFormat::Rgba8);
        assert!(frame.data.chunks(4).all(|px| px == [10, 20, 30, 128]));
    }

    #[test]
    fn rejects_degenerate_sizes() {
        assert!(solid_color_frame([0, 0, 0, 255], 0, 4).is_err());
        assert!(solid_color_frame([0, 0, 0, 255], 4, 99_999).is_err());
    }
}
