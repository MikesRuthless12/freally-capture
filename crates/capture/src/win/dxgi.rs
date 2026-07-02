//! Display capture via DXGI Desktop Duplication.
//!
//! One thread per display: acquire → copy to a CPU staging texture → blend
//! the mouse pointer (duplication delivers the desktop *without* the cursor,
//! plus separate pointer shape/position metadata) → publish as a BGRA
//! [`Frame`]. Handles mode switches / fullscreen transitions by
//! re-duplicating on `DXGI_ERROR_ACCESS_LOST`.
//!
//! AUDITED `unsafe`: D3D11/DXGI COM calls and the mapped-staging read; see
//! the module note in `win/mod.rs`.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use windows::core::Interface;
use windows::Win32::Foundation::E_ACCESSDENIED;
use windows::Win32::Graphics::Direct3D11::{
    ID3D11Device, ID3D11DeviceContext, ID3D11Texture2D, D3D11_CPU_ACCESS_READ,
    D3D11_MAPPED_SUBRESOURCE, D3D11_MAP_READ, D3D11_TEXTURE2D_DESC, D3D11_USAGE_STAGING,
};
use windows::Win32::Graphics::Dxgi::{
    IDXGIOutput1, IDXGIOutputDuplication, IDXGIResource, DXGI_ERROR_ACCESS_LOST,
    DXGI_ERROR_WAIT_TIMEOUT, DXGI_OUTDUPL_FRAME_INFO, DXGI_OUTDUPL_POINTER_SHAPE_INFO,
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR, DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR,
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME,
};

use crate::{CaptureError, Frame, FrameSender, PixelFormat};

/// How long one AcquireNextFrame waits before we re-check the stop flag.
const ACQUIRE_TIMEOUT_MS: u32 = 100;

struct PointerState {
    visible: bool,
    x: i32,
    y: i32,
    shape: Option<PointerShape>,
}

struct PointerShape {
    kind: u32,
    width: u32,
    height: u32,
    pitch: u32,
    hotspot_x: i32,
    hotspot_y: i32,
    data: Vec<u8>,
}

pub(crate) fn run(device_name: &str, sender: FrameSender, stop: Arc<AtomicBool>) {
    match run_inner(device_name, &sender, &stop) {
        Ok(()) => sender.close(None),
        Err(err) => sender.close(Some(err)),
    }
}

/// How long we keep retrying when the display can't be found / duplicated
/// for reasons *other* than a secure desktop before giving up honestly.
const RECOVERY_DEADLINE: Duration = Duration::from_secs(10);
const RECOVERY_RETRY_INTERVAL: Duration = Duration::from_millis(250);

/// A duplication session: everything must live on the *same* adapter, so all
/// three are recreated together on recovery (a display can migrate adapters
/// on hybrid GPUs / re-plugs).
struct Duplication {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    duplication: IDXGIOutputDuplication,
}

fn open_duplication(device_name: &str) -> Result<Duplication, CaptureError> {
    let (adapter, output) = super::find_output_by_name(device_name)?;
    let (device, context) = super::create_d3d_device(Some(&adapter))?;
    let output1: IDXGIOutput1 = output
        .cast()
        .map_err(|err| CaptureError::Backend(format!("IDXGIOutput1: {err}")))?;
    // SAFETY: duplication against the device created on this output's adapter.
    let duplication: IDXGIOutputDuplication =
        unsafe { output1.DuplicateOutput(&device) }.map_err(|err| {
            if err.code() == E_ACCESSDENIED {
                // A secure desktop (UAC prompt, lock screen, Ctrl+Alt+Del) is
                // active — duplication is temporarily forbidden, not broken.
                CaptureError::PermissionDenied
            } else {
                CaptureError::Backend(format!("DuplicateOutput: {err}"))
            }
        })?;
    Ok(Duplication {
        device,
        context,
        duplication,
    })
}

/// (Re)open the duplication, waiting out secure desktops (UAC / lock screen —
/// those can last as long as the user leaves them up) and giving transient
/// failures [`RECOVERY_DEADLINE`] to resolve.
fn open_duplication_with_retry(
    device_name: &str,
    stop: &AtomicBool,
    sender: &FrameSender,
) -> Result<Option<Duplication>, CaptureError> {
    let deadline = Instant::now() + RECOVERY_DEADLINE;
    loop {
        if stop.load(Ordering::Relaxed) || !sender.is_open() {
            return Ok(None);
        }
        match open_duplication(device_name) {
            Ok(session) => return Ok(Some(session)),
            // Secure desktop: retry until it goes away (or we're stopped).
            Err(CaptureError::PermissionDenied) => {}
            Err(err) if Instant::now() >= deadline => return Err(err),
            Err(_) => {}
        }
        std::thread::sleep(RECOVERY_RETRY_INTERVAL);
    }
}

fn run_inner(
    device_name: &str,
    sender: &FrameSender,
    stop: &AtomicBool,
) -> Result<(), CaptureError> {
    // The retry wrapper also covers starting *during* a secure desktop
    // (frames begin the moment it clears); bad sources still fail within
    // the recovery deadline.
    let Some(mut session) = open_duplication_with_retry(device_name, stop, sender)? else {
        return Ok(()); // stopped before the first frame
    };
    let mut staging: Option<(u32, u32, ID3D11Texture2D)> = None;
    let mut pointer = PointerState {
        visible: false,
        x: 0,
        y: 0,
        shape: None,
    };

    while !stop.load(Ordering::Relaxed) && sender.is_open() {
        let mut info = DXGI_OUTDUPL_FRAME_INFO::default();
        let mut resource: Option<IDXGIResource> = None;
        // SAFETY: out-params are locals; on success the frame is released
        // below before the next acquire.
        let acquired = unsafe {
            session
                .duplication
                .AcquireNextFrame(ACQUIRE_TIMEOUT_MS, &mut info, &mut resource)
        };

        match acquired {
            Ok(()) => {}
            Err(err) if err.code() == DXGI_ERROR_WAIT_TIMEOUT => continue,
            Err(err) if err.code() == DXGI_ERROR_ACCESS_LOST => {
                // Mode switch, fullscreen-exclusive handoff, secure desktop,
                // or an adapter migration: rebuild device + duplication
                // together (same-adapter rule) and keep waiting out secure
                // desktops instead of killing the session.
                drop(session);
                staging = None;
                std::thread::sleep(Duration::from_millis(100));
                match open_duplication_with_retry(device_name, stop, sender)? {
                    Some(new_session) => {
                        session = new_session;
                        continue;
                    }
                    None => return Ok(()), // stopped while recovering
                }
            }
            Err(err) => {
                return Err(CaptureError::Backend(format!("AcquireNextFrame: {err}")));
            }
        }

        let result = (|| -> Result<Option<Frame>, CaptureError> {
            update_pointer(&session.duplication, &info, &mut pointer);

            // A pointer-only update carries no desktop image.
            let Some(resource) = resource.as_ref() else {
                return Ok(None);
            };
            if info.LastPresentTime == 0 && info.AccumulatedFrames == 0 {
                return Ok(None);
            }
            let texture: ID3D11Texture2D = resource
                .cast()
                .map_err(|err| CaptureError::Backend(format!("frame texture: {err}")))?;
            let mut desc = D3D11_TEXTURE2D_DESC::default();
            // SAFETY: GetDesc writes into the local out-param.
            unsafe { texture.GetDesc(&mut desc) };

            let staging_tex =
                ensure_staging(&session.device, &mut staging, desc.Width, desc.Height)?;
            // SAFETY: same-device resources with identical dimensions/format.
            unsafe { session.context.CopyResource(staging_tex, &texture) };
            Ok(Some(read_staging(
                &session.context,
                staging_tex,
                desc.Width,
                desc.Height,
            )?))
        })();

        // SAFETY: every successful acquire is paired with exactly one release.
        let _ = unsafe { session.duplication.ReleaseFrame() };

        match result {
            Ok(Some(mut frame)) => {
                blend_pointer(&mut frame, &pointer);
                sender.send(frame);
            }
            Ok(None) => {}
            Err(err) => return Err(err),
        }
    }
    Ok(())
}

fn ensure_staging<'t>(
    device: &ID3D11Device,
    staging: &'t mut Option<(u32, u32, ID3D11Texture2D)>,
    width: u32,
    height: u32,
) -> Result<&'t ID3D11Texture2D, CaptureError> {
    let needs_new = !matches!(staging, Some((w, h, _)) if *w == width && *h == height);
    if needs_new {
        let desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_STAGING,
            BindFlags: 0,
            CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
            MiscFlags: 0,
        };
        let mut texture: Option<ID3D11Texture2D> = None;
        // SAFETY: desc is fully initialized; the out-param is a local.
        unsafe { device.CreateTexture2D(&desc, None, Some(&mut texture)) }
            .map_err(|err| CaptureError::Backend(format!("staging texture: {err}")))?;
        let texture =
            texture.ok_or_else(|| CaptureError::Backend("staging texture missing".into()))?;
        *staging = Some((width, height, texture));
    }
    Ok(&staging.as_ref().expect("staging just ensured").2)
}

fn read_staging(
    context: &ID3D11DeviceContext,
    staging: &ID3D11Texture2D,
    width: u32,
    height: u32,
) -> Result<Frame, CaptureError> {
    let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
    // SAFETY: staging was created D3D11_USAGE_STAGING + CPU_ACCESS_READ; the
    // mapped pointer is valid for RowPitch × height bytes until Unmap.
    unsafe {
        context
            .Map(staging, 0, D3D11_MAP_READ, 0, Some(&mut mapped))
            .map_err(|err| CaptureError::Backend(format!("Map staging: {err}")))?;
    }
    let stride = mapped.RowPitch;
    let len = stride as usize * height as usize;
    // SAFETY: bounds derived from the driver-reported RowPitch and the
    // texture height; copied out before Unmap invalidates the pointer.
    let data = unsafe { std::slice::from_raw_parts(mapped.pData as *const u8, len) }.to_vec();
    // SAFETY: paired with the successful Map above.
    unsafe { context.Unmap(staging, 0) };
    Ok(Frame {
        width,
        height,
        stride,
        format: PixelFormat::Bgra8,
        data,
        captured_at: Instant::now(),
    })
}

// ---------------------------------------------------------------------------
// Mouse pointer (duplication reports it out-of-band)
// ---------------------------------------------------------------------------

fn update_pointer(
    duplication: &IDXGIOutputDuplication,
    info: &DXGI_OUTDUPL_FRAME_INFO,
    pointer: &mut PointerState,
) {
    if info.LastMouseUpdateTime != 0 {
        pointer.visible = info.PointerPosition.Visible.as_bool();
        pointer.x = info.PointerPosition.Position.x;
        pointer.y = info.PointerPosition.Position.y;
    }
    if info.PointerShapeBufferSize == 0 {
        return;
    }
    let mut buffer = vec![0u8; info.PointerShapeBufferSize as usize];
    let mut required = 0u32;
    let mut shape_info = DXGI_OUTDUPL_POINTER_SHAPE_INFO::default();
    // SAFETY: buffer is sized from PointerShapeBufferSize as the API requires.
    let ok = unsafe {
        duplication.GetFramePointerShape(
            buffer.len() as u32,
            buffer.as_mut_ptr() as *mut core::ffi::c_void,
            &mut required,
            &mut shape_info,
        )
    };
    if ok.is_ok() {
        pointer.shape = Some(PointerShape {
            kind: shape_info.Type,
            width: shape_info.Width,
            height: shape_info.Height,
            pitch: shape_info.Pitch,
            hotspot_x: shape_info.HotSpot.x,
            hotspot_y: shape_info.HotSpot.y,
            data: buffer,
        });
    }
}

/// Draw the cached pointer shape into the BGRA frame (duplicated desktop
/// images never include the cursor — without this, display capture looks
/// broken to anyone streaming).
fn blend_pointer(frame: &mut Frame, pointer: &PointerState) {
    let Some(shape) = pointer.shape.as_ref() else {
        return;
    };
    if !pointer.visible {
        return;
    }
    let origin_x = pointer.x - shape.hotspot_x;
    let origin_y = pointer.y - shape.hotspot_y;

    match shape.kind {
        k if k == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR.0 as u32 => {
            blend_color(frame, shape, origin_x, origin_y, false);
        }
        k if k == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR.0 as u32 => {
            blend_color(frame, shape, origin_x, origin_y, true);
        }
        k if k == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME.0 as u32 => {
            blend_monochrome(frame, shape, origin_x, origin_y);
        }
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
