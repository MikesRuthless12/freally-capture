//! Mouse-pointer shapes + software blending, shared by the Windows capture
//! paths. Desktop duplication delivers the desktop *without* the cursor (its
//! shape/position arrive out-of-band), and Windows.Graphics.Capture only
//! composites the cursor into frames of the *focused* window — so both paths
//! draw the cursor themselves with the blending below, the way OBS does.
//!
//! AUDITED `unsafe`: user32/gdi32 cursor + bitmap reads (GetCursorInfo,
//! GetIconInfo, DrawIconEx onto our own DIB sections) and window-rect queries;
//! see the module note in `win/mod.rs`.

use std::collections::HashMap;
use std::time::Instant;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{COLORREF, HWND, POINT, RECT, SIZE};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};
use windows::Win32::Graphics::Dxgi::{
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR, DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR,
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME,
};
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, CreateDIBSection, CreateFontW, DeleteDC, DeleteObject, GdiFlush,
    GetBitmapBits, GetObjectW, GetTextExtentPoint32W, SelectObject, SetBkMode, SetTextColor,
    TextOutW, ANTIALIASED_QUALITY, BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
    CLIP_DEFAULT_PRECIS, DEFAULT_CHARSET, DIB_RGB_COLORS, OUT_DEFAULT_PRECIS, TRANSPARENT,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VIRTUAL_KEY, VK_BACK, VK_CONTROL, VK_DELETE, VK_DOWN, VK_ESCAPE, VK_LBUTTON,
    VK_LEFT, VK_LWIN, VK_MENU, VK_RBUTTON, VK_RETURN, VK_RIGHT, VK_SHIFT, VK_SPACE, VK_TAB, VK_UP,
};
use windows::Win32::UI::WindowsAndMessaging::{
    DrawIconEx, GetAncestor, GetCursorInfo, GetIconInfo, GetWindowRect, WindowFromPoint,
    CURSORINFO, CURSOR_SHOWING, DI_NORMAL, GA_ROOT, HICON, ICONINFO,
};

use crate::cursorfx::{self, CursorFxConfig, FxState, KeyBadge};
use crate::Frame;

pub(crate) const SHAPE_COLOR: u32 = DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR.0 as u32;
pub(crate) const SHAPE_MASKED_COLOR: u32 = DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR.0 as u32;
pub(crate) const SHAPE_MONOCHROME: u32 = DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME.0 as u32;

/// One pointer image, in the DXGI shape vocabulary (COLOR = straight-alpha
/// BGRA; MASKED_COLOR = color + XOR mask in the alpha byte; MONOCHROME =
/// 1-bpp AND mask over 1-bpp XOR mask, stacked vertically).
pub(crate) struct PointerShape {
    pub kind: u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub hotspot_x: i32,
    pub hotspot_y: i32,
    pub data: Vec<u8>,
}

/// Draw `shape` into the BGRA frame with its hotspot at (`x`, `y`).
pub(crate) fn blend_shape(frame: &mut Frame, shape: &PointerShape, x: i32, y: i32) {
    let origin_x = x - shape.hotspot_x;
    let origin_y = y - shape.hotspot_y;
    match shape.kind {
        k if k == SHAPE_COLOR => blend_color(frame, shape, origin_x, origin_y, false),
        k if k == SHAPE_MASKED_COLOR => blend_color(frame, shape, origin_x, origin_y, true),
        k if k == SHAPE_MONOCHROME => blend_monochrome(frame, shape, origin_x, origin_y),
        _ => {}
    }
}

/// COLOR: straight-alpha BGRA over-blend. MASKED_COLOR: the alpha byte is a
/// mask — 0 ⇒ opaque color, 0xFF ⇒ XOR with screen (we invert, the standard
/// approximation).
fn blend_color(
    frame: &mut Frame,
    shape: &PointerShape,
    origin_x: i32,
    origin_y: i32,
    masked: bool,
) {
    for row in 0..shape.height {
        let dst_y = origin_y + row as i32;
        if dst_y < 0 || dst_y >= frame.height as i32 {
            continue;
        }
        for col in 0..shape.width {
            let dst_x = origin_x + col as i32;
            if dst_x < 0 || dst_x >= frame.width as i32 {
                continue;
            }
            let src_idx = (row * shape.pitch + col * 4) as usize;
            let Some(px) = shape.data.get(src_idx..src_idx + 4) else {
                continue;
            };
            let dst_idx = dst_y as usize * frame.stride as usize + dst_x as usize * 4;
            let Some(dst) = frame.data.get_mut(dst_idx..dst_idx + 4) else {
                continue;
            };
            if masked {
                if px[3] == 0 {
                    dst[0] = px[0];
                    dst[1] = px[1];
                    dst[2] = px[2];
                } else {
                    // XOR mask: invert the underlying pixel.
                    dst[0] = 255 - dst[0];
                    dst[1] = 255 - dst[1];
                    dst[2] = 255 - dst[2];
                }
            } else {
                let alpha = px[3] as u32;
                if alpha == 0 {
                    continue;
                }
                for c in 0..3 {
                    let src_c = px[c] as u32;
                    let dst_c = dst[c] as u32;
                    dst[c] = ((src_c * alpha + dst_c * (255 - alpha)) / 255) as u8;
                }
            }
        }
    }
}

/// MONOCHROME: 1-bpp AND mask over 1-bpp XOR mask, stacked vertically
/// (`shape.height` counts both). result = (screen AND and) XOR xor.
fn blend_monochrome(frame: &mut Frame, shape: &PointerShape, origin_x: i32, origin_y: i32) {
    let cursor_height = shape.height / 2;
    for row in 0..cursor_height {
        let dst_y = origin_y + row as i32;
        if dst_y < 0 || dst_y >= frame.height as i32 {
            continue;
        }
        for col in 0..shape.width {
            let dst_x = origin_x + col as i32;
            if dst_x < 0 || dst_x >= frame.width as i32 {
                continue;
            }
            let byte_idx = (row * shape.pitch + col / 8) as usize;
            let xor_byte_idx = ((row + cursor_height) * shape.pitch + col / 8) as usize;
            let bit = 0x80u8 >> (col % 8);
            let and_set = shape
                .data
                .get(byte_idx)
                .map(|b| b & bit != 0)
                .unwrap_or(true);
            let xor_set = shape
                .data
                .get(xor_byte_idx)
                .map(|b| b & bit != 0)
                .unwrap_or(false);
            let dst_idx = dst_y as usize * frame.stride as usize + dst_x as usize * 4;
            let Some(dst) = frame.data.get_mut(dst_idx..dst_idx + 4) else {
                continue;
            };
            match (and_set, xor_set) {
                (true, false) => {} // transparent
                (true, true) => {
                    // Invert the screen pixel.
                    dst[0] = 255 - dst[0];
                    dst[1] = 255 - dst[1];
                    dst[2] = 255 - dst[2];
                }
                (false, false) => {
                    dst[0] = 0;
                    dst[1] = 0;
                    dst[2] = 0;
                }
                (false, true) => {
                    dst[0] = 255;
                    dst[1] = 255;
                    dst[2] = 255;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Live-cursor tracking for window capture (wgc)
// ---------------------------------------------------------------------------

/// Everything that decides what the drawn cursor looks like on one frame —
/// the emit-dedup key: nothing changed ⇒ no synthesized frame.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct CursorKey {
    /// The (visible) cursor is inside the captured window's bounds.
    pub over: bool,
    /// Hotspot position in frame pixels (valid when `over`).
    pub x: i32,
    pub y: i32,
    /// The `HCURSOR` handle value (shape identity).
    pub cursor: isize,
}

impl CursorKey {
    pub(crate) const AWAY: CursorKey = CursorKey {
        over: false,
        x: 0,
        y: 0,
        cursor: 0,
    };
}

/// Samples the global cursor against a window and draws it into frames,
/// caching one rendered [`PointerShape`] per `HCURSOR`.
pub(crate) struct CursorTracker {
    shapes: HashMap<isize, Option<PointerShape>>,
}

impl CursorTracker {
    pub(crate) fn new() -> Self {
        CursorTracker {
            shapes: HashMap::new(),
        }
    }

    /// Where is the cursor relative to `hwnd`'s visible bounds, mapped into a
    /// `frame_w`×`frame_h` frame? `AWAY` when hidden or outside.
    pub(crate) fn sample(hwnd_raw: isize, frame_w: u32, frame_h: u32) -> CursorKey {
        let mut info = CURSORINFO {
            cbSize: std::mem::size_of::<CURSORINFO>() as u32,
            ..Default::default()
        };
        // SAFETY: out-param sized via cbSize as the API requires.
        if unsafe { GetCursorInfo(&mut info) }.is_err() {
            return CursorKey::AWAY;
        }
        if info.flags.0 & CURSOR_SHOWING.0 == 0 || info.hCursor.is_invalid() {
            return CursorKey::AWAY;
        }

        let hwnd = HWND(hwnd_raw as *mut core::ffi::c_void);
        // The DWM extended frame bounds match what WGC captures (no drop
        // shadow); plain GetWindowRect is the fallback.
        let mut rect = RECT::default();
        // SAFETY: writing a RECT-sized out-param for our own live HWND.
        let bounds_ok = unsafe {
            DwmGetWindowAttribute(
                hwnd,
                DWMWA_EXTENDED_FRAME_BOUNDS,
                &mut rect as *mut RECT as *mut core::ffi::c_void,
                std::mem::size_of::<RECT>() as u32,
            )
        }
        .is_ok()
            // SAFETY: same out-param contract.
            || unsafe { GetWindowRect(hwnd, &mut rect) }.is_ok();
        let (rect_w, rect_h) = (
            i64::from(rect.right - rect.left),
            i64::from(rect.bottom - rect.top),
        );
        if !bounds_ok || rect_w <= 0 || rect_h <= 0 {
            return CursorKey::AWAY;
        }

        // Scale window-relative physical coords into frame pixels (they are
        // normally 1:1; a mid-resize frame may briefly differ).
        let x = (i64::from(info.ptScreenPos.x - rect.left) * i64::from(frame_w) / rect_w) as i32;
        let y = (i64::from(info.ptScreenPos.y - rect.top) * i64::from(frame_h) / rect_h) as i32;
        let in_frame = x >= 0 && (x as u32) < frame_w && y >= 0 && (y as u32) < frame_h;
        // Occlusion hit-test (matches OBS): draw the cursor only when the
        // captured window is actually the frontmost window under the pointer.
        // Keying off the bounding rect alone would paint the cursor onto
        // pixels another window covers — Freally itself on top, or any app over
        // the captured window — and onto parts of the window hidden behind
        // another. `WindowFromPoint` returns whatever window owns that screen
        // pixel; only when its top-level root is ours is the cursor really
        // "over" the capture.
        if !(in_frame && Self::cursor_over_window(hwnd, info.ptScreenPos)) {
            // The canonical away key, not a live x/y with `over: false`: the
            // key is the emit-dedup key, so a not-over position change must
            // compare equal to the last not-over sample — else every cursor
            // move anywhere on the desktop (or over an occluding window)
            // synthesizes an undrawn frame at the pump cadence.
            return CursorKey::AWAY;
        }
        CursorKey {
            over: true,
            x,
            y,
            cursor: info.hCursor.0 as isize,
        }
    }

    /// Whether `target`'s top-level window is the frontmost window at screen
    /// point `pt` — the occlusion test behind [`Self::sample`]'s `over`.
    fn cursor_over_window(target: HWND, pt: POINT) -> bool {
        // SAFETY: a plain hit-test + ancestor walk over live window handles.
        unsafe {
            let hit = WindowFromPoint(pt);
            if hit.is_invalid() {
                return false;
            }
            GetAncestor(hit, GA_ROOT).0 == GetAncestor(target, GA_ROOT).0
        }
    }

    /// Draw the sampled cursor into the frame (no-op when not `over` or the
    /// cursor shape cannot be rendered).
    pub(crate) fn blend(&mut self, frame: &mut Frame, key: CursorKey) {
        if !key.over {
            return;
        }
        if self.shapes.len() > 64 {
            self.shapes.clear();
        }
        let shape = self
            .shapes
            .entry(key.cursor)
            .or_insert_with(|| shape_from_hcursor(key.cursor));
        if let Some(shape) = shape {
            blend_shape(frame, shape, key.x, key.y);
        }
    }
}

/// Render an `HCURSOR` to a [`PointerShape`]. Monochrome (AND/XOR) cursors
/// keep their native mask layout so inversion still works; color cursors are
/// rendered twice (black and white background) to recover straight alpha.
fn shape_from_hcursor(hcursor_raw: isize) -> Option<PointerShape> {
    let hicon = HICON(hcursor_raw as *mut core::ffi::c_void);
    let mut info = ICONINFO::default();
    // SAFETY: querying a live cursor handle; the two returned bitmaps are
    // owned by us and released below.
    unsafe { GetIconInfo(hicon, &mut info) }.ok()?;
    let mask_bmp = info.hbmMask;
    let color_bmp = info.hbmColor;
    let cleanup = || {
        // SAFETY: GetIconInfo transferred ownership of these bitmaps to us.
        unsafe {
            if !mask_bmp.is_invalid() {
                let _ = DeleteObject(mask_bmp);
            }
            if !color_bmp.is_invalid() {
                let _ = DeleteObject(color_bmp);
            }
        }
    };

    let mut bm = BITMAP::default();
    // SAFETY: out-param write for a live bitmap handle.
    let got = unsafe {
        GetObjectW(
            mask_bmp,
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bm as *mut BITMAP as *mut core::ffi::c_void),
        )
    };
    if got == 0 || bm.bmWidth <= 0 || bm.bmHeight <= 0 {
        cleanup();
        return None;
    }

    let shape = if color_bmp.is_invalid() {
        // Monochrome: hbmMask is the AND mask stacked over the XOR mask —
        // exactly the MONOCHROME shape layout. GetBitmapBits rows are
        // WORD-aligned.
        let width = bm.bmWidth as u32;
        let full_height = bm.bmHeight as u32; // both halves
        let pitch = width.div_ceil(16) * 2;
        let mut data = vec![0u8; (pitch * full_height) as usize];
        // SAFETY: buffer sized to the bitmap's WORD-aligned 1-bpp rows.
        let copied = unsafe {
            GetBitmapBits(
                mask_bmp,
                data.len() as i32,
                data.as_mut_ptr() as *mut core::ffi::c_void,
            )
        };
        (copied > 0).then_some(PointerShape {
            kind: SHAPE_MONOCHROME,
            width,
            height: full_height,
            pitch,
            hotspot_x: info.xHotspot as i32,
            hotspot_y: info.yHotspot as i32,
            data,
        })
    } else {
        // Color: draw on black and on white, recover straight alpha per
        // channel (white − black = 255 − alpha), color from the black pass.
        let width = bm.bmWidth as u32;
        let height = bm.bmHeight as u32;
        let black = draw_cursor_on(hicon, width, height, 0x00);
        let white = draw_cursor_on(hicon, width, height, 0xFF);
        match (black, white) {
            (Some(black), Some(white)) => {
                let mut data = vec![0u8; (width * height * 4) as usize];
                for i in (0..data.len()).step_by(4) {
                    let alpha =
                        (255 - (i32::from(white[i + 1]) - i32::from(black[i + 1]))).clamp(0, 255);
                    if alpha == 0 {
                        continue;
                    }
                    for c in 0..3 {
                        data[i + c] =
                            ((u32::from(black[i + c]) * 255) / alpha as u32).min(255) as u8;
                    }
                    data[i + 3] = alpha as u8;
                }
                Some(PointerShape {
                    kind: SHAPE_COLOR,
                    width,
                    height,
                    pitch: width * 4,
                    hotspot_x: info.xHotspot as i32,
                    hotspot_y: info.yHotspot as i32,
                    data,
                })
            }
            _ => None,
        }
    };
    cleanup();
    shape
}

/// Draw the cursor onto a `bg`-filled 32-bpp DIB and return its BGRA bytes.
fn draw_cursor_on(hicon: HICON, width: u32, height: u32, bg: u8) -> Option<Vec<u8>> {
    // SAFETY: a private memory DC + DIB section, fully released before return;
    // DrawIconEx only touches that DC.
    unsafe {
        let hdc = CreateCompatibleDC(None);
        if hdc.is_invalid() {
            return None;
        }
        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: -(height as i32), // top-down
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut bits: *mut core::ffi::c_void = std::ptr::null_mut();
        let Ok(dib) = CreateDIBSection(hdc, &bmi, DIB_RGB_COLORS, &mut bits, None, 0) else {
            let _ = DeleteDC(hdc);
            return None;
        };
        if bits.is_null() {
            let _ = DeleteObject(dib);
            let _ = DeleteDC(hdc);
            return None;
        }
        let old = SelectObject(hdc, dib);
        let len = (width * height * 4) as usize;
        let px = std::slice::from_raw_parts_mut(bits as *mut u8, len);
        px.fill(bg);
        let drawn = DrawIconEx(
            hdc,
            0,
            0,
            hicon,
            width as i32,
            height as i32,
            0,
            None,
            DI_NORMAL,
        )
        .is_ok();
        let _ = GdiFlush();
        let out = drawn.then(|| px.to_vec());
        SelectObject(hdc, old);
        let _ = DeleteObject(dib);
        let _ = DeleteDC(hdc);
        out
    }
}

// ---------------------------------------------------------------------------
// Cursor-effects input sampling + keystroke badges (CAP-N19)
// ---------------------------------------------------------------------------

/// The mouse buttons as a `cursorfx::BUTTON_*` bitmask. Sampled only when a
/// capture has a cursor-effects config with ripples on — otherwise nothing
/// ever reads the buttons.
pub(crate) fn sample_buttons() -> u8 {
    let mut mask = 0u8;
    // SAFETY: plain global key-state reads; no memory contract.
    unsafe {
        if GetAsyncKeyState(i32::from(VK_LBUTTON.0)) as u16 & 0x8000 != 0 {
            mask |= cursorfx::BUTTON_LEFT;
        }
        if GetAsyncKeyState(i32::from(VK_RBUTTON.0)) as u16 & 0x8000 != 0 {
            mask |= cursorfx::BUTTON_RIGHT;
        }
    }
    mask
}

/// The FIXED key set keystroke ghosting may show: modifiers, letters, digits,
/// and a few navigation keys — nothing outside this list is ever sampled.
/// What is held gets drawn into the frame and immediately forgotten; nothing
/// is stored or logged (the roadmap's explicit line).
const GHOST_MODIFIERS: [VIRTUAL_KEY; 4] = [VK_CONTROL, VK_SHIFT, VK_MENU, VK_LWIN];
const GHOST_SPECIALS: [VIRTUAL_KEY; 10] = [
    VK_ESCAPE, VK_TAB, VK_RETURN, VK_SPACE, VK_BACK, VK_DELETE, VK_LEFT, VK_UP, VK_RIGHT, VK_DOWN,
];

/// Sample the fixed ghost-key set → held VKs in display order (modifiers
/// first, so a chord reads "Ctrl Shift C"; capped so a mashed keyboard cannot
/// flood the frame with badges).
pub(crate) fn sample_ghost_keys() -> Vec<i32> {
    const MAX_BADGES: usize = 6;
    let mut held: Vec<i32> = Vec::new();
    let modifiers = GHOST_MODIFIERS.iter().map(|vk| i32::from(vk.0));
    let letters = 0x41..=0x5A; // A–Z (VK == ASCII)
    let digits = 0x30..=0x39; // 0–9
    let specials = GHOST_SPECIALS.iter().map(|vk| i32::from(vk.0));
    for vk in modifiers.chain(letters).chain(digits).chain(specials) {
        if held.len() >= MAX_BADGES {
            break;
        }
        // SAFETY: plain global key-state read.
        if unsafe { GetAsyncKeyState(vk) } as u16 & 0x8000 != 0 {
            held.push(vk);
        }
    }
    held
}

/// A fixed-set VK's badge label. `None` for anything outside the set (never
/// sampled anyway — belt and braces).
fn ghost_label(vk: i32) -> Option<&'static str> {
    const LETTERS: [&str; 26] = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];
    const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    match vk {
        0x41..=0x5A => Some(LETTERS[(vk - 0x41) as usize]),
        0x30..=0x39 => Some(DIGITS[(vk - 0x30) as usize]),
        vk if vk == i32::from(VK_CONTROL.0) => Some("Ctrl"),
        vk if vk == i32::from(VK_SHIFT.0) => Some("Shift"),
        vk if vk == i32::from(VK_MENU.0) => Some("Alt"),
        vk if vk == i32::from(VK_LWIN.0) => Some("Win"),
        vk if vk == i32::from(VK_ESCAPE.0) => Some("Esc"),
        vk if vk == i32::from(VK_TAB.0) => Some("Tab"),
        vk if vk == i32::from(VK_RETURN.0) => Some("Enter"),
        vk if vk == i32::from(VK_SPACE.0) => Some("Space"),
        vk if vk == i32::from(VK_BACK.0) => Some("Bksp"),
        vk if vk == i32::from(VK_DELETE.0) => Some("Del"),
        vk if vk == i32::from(VK_LEFT.0) => Some("←"),
        vk if vk == i32::from(VK_UP.0) => Some("↑"),
        vk if vk == i32::from(VK_RIGHT.0) => Some("→"),
        vk if vk == i32::from(VK_DOWN.0) => Some("↓"),
        _ => None,
    }
}

/// Renders and caches key-badge labels (GDI white-on-black → luminance-as-
/// alpha, the `draw_cursor_on` trick) and draws them near the cursor. One per
/// capture session; the fixed key set bounds the cache.
pub(crate) struct KeyGhost {
    cache: HashMap<i32, Option<KeyBadge>>,
}

impl KeyGhost {
    pub(crate) fn new() -> Self {
        KeyGhost {
            cache: HashMap::new(),
        }
    }

    /// One badge per held key, left-to-right just under the cursor hotspot
    /// (the draw fns clip at the frame edges).
    pub(crate) fn draw(&mut self, frame: &mut Frame, keys: &[i32], x: i32, y: i32) {
        let mut at_x = x + 18;
        let at_y = y + 26;
        for &vk in keys {
            let badge = self
                .cache
                .entry(vk)
                .or_insert_with(|| ghost_label(vk).and_then(render_label));
            if let Some(badge) = badge {
                at_x += cursorfx::draw_badge(frame, at_x, at_y, badge);
            }
        }
    }
}

/// Render one badge label to coverage bytes: GDI draws white text on a black
/// 32-bpp DIB, and the green channel IS the anti-aliased coverage.
fn render_label(label: &str) -> Option<KeyBadge> {
    let text: Vec<u16> = label.encode_utf16().collect();
    let face: Vec<u16> = "Segoe UI"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    // SAFETY: a private memory DC + font + DIB section, all released before
    // return; GDI only touches that DC.
    unsafe {
        let hdc = CreateCompatibleDC(None);
        if hdc.is_invalid() {
            return None;
        }
        // Pitch-and-family 0 = DEFAULT_PITCH | FF_DONTCARE.
        let font = CreateFontW(
            -14,
            0,
            0,
            0,
            600, // semibold — readable at badge size
            0,
            0,
            0,
            u32::from(DEFAULT_CHARSET.0),
            u32::from(OUT_DEFAULT_PRECIS.0),
            u32::from(CLIP_DEFAULT_PRECIS.0),
            u32::from(ANTIALIASED_QUALITY.0), // grayscale AA — ClearType would color-fringe the alpha
            0,
            PCWSTR(face.as_ptr()),
        );
        let old_font = (!font.is_invalid()).then(|| SelectObject(hdc, font));
        let release_font_dc = |old_font: Option<_>| {
            if let Some(old) = old_font {
                SelectObject(hdc, old);
            }
            if !font.is_invalid() {
                let _ = DeleteObject(font);
            }
            let _ = DeleteDC(hdc);
        };

        let mut size = SIZE::default();
        if !GetTextExtentPoint32W(hdc, &text, &mut size).as_bool() || size.cx <= 0 || size.cy <= 0 {
            release_font_dc(old_font);
            return None;
        }
        let (width, height) = (size.cx as u32, size.cy as u32);

        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: -(height as i32), // top-down
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut bits: *mut core::ffi::c_void = std::ptr::null_mut();
        let Ok(dib) = CreateDIBSection(hdc, &bmi, DIB_RGB_COLORS, &mut bits, None, 0) else {
            release_font_dc(old_font);
            return None;
        };
        if bits.is_null() {
            let _ = DeleteObject(dib);
            release_font_dc(old_font);
            return None;
        }
        let old_bmp = SelectObject(hdc, dib);
        let len = (width * height * 4) as usize;
        let px = std::slice::from_raw_parts_mut(bits as *mut u8, len);
        px.fill(0);
        let _ = SetBkMode(hdc, TRANSPARENT);
        let _ = SetTextColor(hdc, COLORREF(0x00FF_FFFF));
        let drawn = TextOutW(hdc, 0, 0, &text).as_bool();
        let _ = GdiFlush();
        let out = drawn.then(|| {
            let mut alpha = vec![0u8; (width * height) as usize];
            for (i, slot) in alpha.iter_mut().enumerate() {
                *slot = px[i * 4 + 1]; // green channel of white-on-black
            }
            KeyBadge {
                width,
                height,
                alpha,
            }
        });
        SelectObject(hdc, old_bmp);
        let _ = DeleteObject(dib);
        release_font_dc(old_font);
        out
    }
}

/// One capture tick's cursor-effects input pass: sample ONLY what the config
/// enables (no config = no input reads at all), then advance the ripple /
/// badge state. Returns true when the effects need a repaint even though the
/// capture produced no frame.
pub(crate) fn fx_tick(
    fx: &mut FxState,
    config: Option<&CursorFxConfig>,
    x: i32,
    y: i32,
    over: bool,
    now: Instant,
) -> bool {
    let buttons = if config.is_some_and(|c| c.ripples) {
        sample_buttons()
    } else {
        0
    };
    let keys = if config.is_some_and(|c| c.keystrokes) {
        sample_ghost_keys()
    } else {
        Vec::new()
    };
    fx.tick(config, buttons, &keys, x, y, over, now)
}

/// Draw the enabled effects into a frame the cursor was just blended into
/// (halo + ripples from the shared draw fns, key badges from the GDI cache).
#[allow(clippy::too_many_arguments)] // one call site per capture path; a struct would just rename them
pub(crate) fn fx_draw(
    frame: &mut Frame,
    fx: &FxState,
    ghost: &mut KeyGhost,
    config: &CursorFxConfig,
    x: i32,
    y: i32,
    over: bool,
    now: Instant,
) {
    fx.draw(frame, config, x, y, over, now);
    if config.keystrokes && over {
        ghost.draw(frame, fx.held_keys(), x, y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PixelFormat;

    fn frame(w: u32, h: u32, fill: u8) -> Frame {
        Frame {
            width: w,
            height: h,
            stride: w * 4,
            format: PixelFormat::Bgra8,
            data: vec![fill; (w * h * 4) as usize],
            captured_at: Instant::now(),
        }
    }

    #[test]
    fn color_shape_alpha_blends_and_clips() {
        // A 2×2 fully-red, fully-opaque cursor with hotspot (0,0).
        let shape = PointerShape {
            kind: SHAPE_COLOR,
            width: 2,
            height: 2,
            pitch: 8,
            hotspot_x: 0,
            hotspot_y: 0,
            data: vec![
                0, 0, 255, 255, 0, 0, 255, 255, //
                0, 0, 255, 255, 0, 0, 255, 255,
            ],
        };
        let mut f = frame(4, 4, 0);
        blend_shape(&mut f, &shape, 3, 3); // bottom-right corner: 3 px clipped
        assert_eq!(&f.data[(3 * 16 + 12)..(3 * 16 + 16)], &[0, 0, 255, 0]);
        assert_eq!(&f.data[0..4], &[0, 0, 0, 0], "far corner untouched");

        // 50% alpha over black → half-red.
        let half = PointerShape {
            data: [[0u8, 0, 255, 128]; 4].concat(),
            ..shape
        };
        let mut f = frame(4, 4, 0);
        blend_shape(&mut f, &half, 0, 0);
        assert_eq!(f.data[2], (255 * 128 / 255) as u8);
    }

    #[test]
    fn monochrome_shape_inverts_under_xor() {
        // 8×1 cursor: AND all-set except bit 0; XOR bit 0 and bit 1 set.
        // bit 0: and=0,xor=1 → white. bit 1: and=1,xor=1 → invert.
        let shape = PointerShape {
            kind: SHAPE_MONOCHROME,
            width: 8,
            height: 2, // AND row + XOR row
            pitch: 2,
            hotspot_x: 0,
            hotspot_y: 0,
            data: vec![0b0111_1111, 0, 0b1100_0000, 0],
        };
        let mut f = frame(8, 1, 10);
        blend_shape(&mut f, &shape, 0, 0);
        assert_eq!(&f.data[0..3], &[255, 255, 255], "col 0 forced white");
        assert_eq!(&f.data[4..7], &[245, 245, 245], "col 1 inverted");
        assert_eq!(&f.data[8..11], &[10, 10, 10], "col 2 untouched");
    }

    #[test]
    fn away_key_never_blends() {
        let mut tracker = CursorTracker::new();
        let mut f = frame(4, 4, 7);
        let before = f.data.clone();
        tracker.blend(&mut f, CursorKey::AWAY);
        assert_eq!(f.data, before);
    }
}
