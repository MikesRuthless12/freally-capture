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
};

use super::pointer::{blend_shape, PointerShape};
use crate::{CaptureError, Frame, FrameSender, PixelFormat};

/// How long one AcquireNextFrame waits before we re-check the stop flag.
const ACQUIRE_TIMEOUT_MS: u32 = 100;

struct PointerState {
    visible: bool,
    x: i32,
    y: i32,
    shape: Option<PointerShape>,
    /// Bumped whenever a new shape arrives — part of the "did the drawn
    /// cursor change" key for pointer-only frame synthesis.
    generation: u64,
}

impl PointerState {
    /// What the drawn cursor depends on; unchanged key ⇒ no synthesized frame.
    fn key(&self) -> (bool, i32, i32, u64) {
        (self.visible, self.x, self.y, self.generation)
    }
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
        generation: 0,
    };
    // The last desktop image *without* the cursor: pointer-only updates (the
    // cursor gliding over a static desktop) re-blend onto this and publish,
    // so the recorded cursor keeps moving even when nothing else repaints.
    let mut last_base: Option<Frame> = None;
    let mut last_drawn = pointer.key();

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
                last_base = Some(frame.clone());
                blend_pointer(&mut frame, &pointer);
                last_drawn = pointer.key();
                sender.send(frame);
            }
            Ok(None) => {
                // Pointer-only update: synthesize a frame from the last
                // desktop image so the cursor still tracks.
                if pointer.key() != last_drawn {
                    if let Some(base) = last_base.as_ref() {
                        let mut frame = base.clone();
                        frame.captured_at = Instant::now();
                        blend_pointer(&mut frame, &pointer);
                        last_drawn = pointer.key();
                        sender.send(frame);
                    }
                }
            }
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
        pointer.generation += 1;
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
    blend_shape(frame, shape, pointer.x, pointer.y);
}
