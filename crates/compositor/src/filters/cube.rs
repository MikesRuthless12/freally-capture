//! Adobe/Resolve `.cube` LUT parsing — pure text in, lattice out.
//!
//! Supports the common 3D form (`LUT_3D_SIZE`), with `DOMAIN_MIN`/`DOMAIN_MAX`
//! honored and values clamped to [0, 1] (the compositor samples the LUT as an
//! 8-bit 3D texture, matching how OBS treats LUT images). File reading happens
//! in the app layer — this module never touches the filesystem.

use crate::CompositorError;

/// A parsed 3D LUT: `size³` RGBA entries (alpha = 255), red fastest —
/// exactly the memory order a `wgpu` 3D texture upload wants.
#[derive(Debug)]
pub struct Lut3d {
    pub size: u32,
    /// `size³ * 4` bytes, RGBA.
    pub rgba: Vec<u8>,
}

/// Parse `.cube` text. Errors name the offending line — the UI shows them in
/// the filter's row rather than failing silently.
pub fn parse_cube(text: &str) -> Result<Lut3d, CompositorError> {
    let bad = |line_no: usize, why: &str| {
        CompositorError::BadFrame(format!(".cube line {line_no}: {why}"))
    };

    let mut size: Option<u32> = None;
    let mut domain_min = [0.0f32; 3];
    let mut domain_max = [1.0f32; 3];
    let mut values: Vec<[f32; 3]> = Vec::new();

    for (index, raw) in text.lines().enumerate() {
        let line_no = index + 1;
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut tokens = line.split_whitespace();
        let head = tokens.next().expect("non-empty line has a token");
        match head {
            "TITLE" => {}
            "LUT_1D_SIZE" => {
                return Err(bad(line_no, "1D LUTs are not supported (use a 3D .cube)"));
            }
            "LUT_3D_SIZE" => {
                let n: u32 = tokens
                    .next()
                    .and_then(|token| token.parse().ok())
                    .ok_or_else(|| bad(line_no, "LUT_3D_SIZE needs a number"))?;
                if !(2..=129).contains(&n) {
                    return Err(bad(line_no, "LUT_3D_SIZE must be between 2 and 129"));
                }
                size = Some(n);
            }
            "DOMAIN_MIN" | "DOMAIN_MAX" => {
                let mut triple = [0.0f32; 3];
                for slot in &mut triple {
                    *slot = tokens
                        .next()
                        .and_then(|token| token.parse().ok())
                        .ok_or_else(|| bad(line_no, "domain needs three numbers"))?;
                }
                if head == "DOMAIN_MIN" {
                    domain_min = triple;
                } else {
                    domain_max = triple;
                }
            }
            _ => {
                // A data row: three floats.
                let r: f32 = head
                    .parse()
                    .map_err(|_| bad(line_no, "expected a number or keyword"))?;
                let g: f32 = tokens
                    .next()
                    .and_then(|token| token.parse().ok())
                    .ok_or_else(|| bad(line_no, "data rows need three numbers"))?;
                let b: f32 = tokens
                    .next()
                    .and_then(|token| token.parse().ok())
                    .ok_or_else(|| bad(line_no, "data rows need three numbers"))?;
                values.push([r, g, b]);
            }
        }
    }

    let size =
        size.ok_or_else(|| CompositorError::BadFrame(".cube file has no LUT_3D_SIZE".to_string()))?;
    let expected = (size as usize).pow(3);
    if values.len() != expected {
        return Err(CompositorError::BadFrame(format!(
            ".cube data has {} entries, LUT_3D_SIZE {size} needs {expected}",
            values.len()
        )));
    }

    // Normalize through the domain and quantize to 8-bit.
    let mut rgba = Vec::with_capacity(expected * 4);
    for value in values {
        for channel in 0..3 {
            let lo = domain_min[channel];
            let hi = domain_max[channel];
            let span = (hi - lo).abs().max(f32::EPSILON);
            let normalized = ((value[channel] - lo) / span).clamp(0.0, 1.0);
            rgba.push((normalized * 255.0).round() as u8);
        }
        rgba.push(255);
    }
    Ok(Lut3d { size, rgba })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A generated identity cube of lattice size `n`.
    pub(crate) fn identity_cube(n: u32) -> String {
        let mut text = format!("TITLE \"identity\"\nLUT_3D_SIZE {n}\n");
        let step = 1.0 / (n - 1) as f32;
        for b in 0..n {
            for g in 0..n {
                for r in 0..n {
                    text.push_str(&format!(
                        "{:.6} {:.6} {:.6}\n",
                        r as f32 * step,
                        g as f32 * step,
                        b as f32 * step
                    ));
                }
            }
        }
        text
    }

    #[test]
    fn parses_an_identity_cube() {
        let lut = parse_cube(&identity_cube(3)).expect("parse");
        assert_eq!(lut.size, 3);
        assert_eq!(lut.rgba.len(), 27 * 4);
        // First lattice point is black, last is white, red varies fastest.
        assert_eq!(&lut.rgba[0..4], &[0, 0, 0, 255]);
        assert_eq!(&lut.rgba[4..8], &[128, 0, 0, 255]);
        let last = lut.rgba.len() - 4;
        assert_eq!(&lut.rgba[last..], &[255, 255, 255, 255]);
    }

    #[test]
    fn honors_the_domain() {
        let text = "LUT_3D_SIZE 2\nDOMAIN_MIN 0 0 0\nDOMAIN_MAX 2 2 2\n\
                    0 0 0\n2 0 0\n0 2 0\n2 2 0\n0 0 2\n2 0 2\n0 2 2\n2 2 2\n";
        let lut = parse_cube(text).expect("parse");
        let last = lut.rgba.len() - 4;
        assert_eq!(&lut.rgba[last..], &[255, 255, 255, 255], "2.0 → domain max");
    }

    #[test]
    fn rejects_wrong_entry_counts() {
        let text = "LUT_3D_SIZE 2\n0 0 0\n1 1 1\n";
        assert!(parse_cube(text).is_err());
    }

    #[test]
    fn rejects_1d_luts_honestly() {
        let text = "LUT_1D_SIZE 2\n0 0 0\n1 1 1\n";
        let err = parse_cube(text).unwrap_err();
        assert!(err.to_string().contains("1D"), "says why: {err}");
    }

    #[test]
    fn tolerates_comments_and_blank_lines() {
        let text = "# a comment\nTITLE \"x\"\n\nLUT_3D_SIZE 2\n# data\n\
                    0 0 0\n1 0 0\n0 1 0\n1 1 0\n0 0 1\n1 0 1\n0 1 1\n1 1 1\n";
        assert!(parse_cube(text).is_ok());
    }
}
