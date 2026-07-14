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
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT, DXGI_FORMAT_R16G16B16A16_FLOAT};
use windows::Win32::Graphics::Dxgi::{
    IDXGIOutput1, IDXGIOutputDuplication, IDXGIResource, DXGI_ERROR_ACCESS_LOST,
    DXGI_ERROR_WAIT_TIMEOUT, DXGI_OUTDUPL_FRAME_INFO, DXGI_OUTDUPL_POINTER_SHAPE_INFO,
};

use super::pointer::{blend_shape, fx_draw, fx_tick, KeyGhost, PointerShape};
use crate::cursorfx::{self, CursorFxConfig, FxState};
use crate::{CaptureError, Frame, FrameSender, PixelFormat};

/// How long one AcquireNextFrame waits before we re-check the stop flag.
const ACQUIRE_TIMEOUT_MS: u32 = 100;
/// The faster acquire wait while cursor effects sample input (CAP-N19):
/// clicks and key edges arrive between desktop updates, and ripples animate
/// at this cadence over a perfectly still desktop.
const FX_ACQUIRE_TIMEOUT_MS: u32 = 16;

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
    let mut staging: Option<(u32, u32, DXGI_FORMAT, ID3D11Texture2D)> = None;
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
    // Cursor effects (CAP-N19): the registry key matches the scene's capture
    // id, and the state/label cache live for the session.
    let fx_id = format!("display:{device_name}");
    let mut fx = FxState::new();
    let mut ghost = KeyGhost::new();

    while !stop.load(Ordering::Relaxed) && sender.is_open() {
        // The config read is live (the tone-map precedent): a retune applies
        // on the very next frame. While effects sample input, acquire on a
        // short leash — clicks and key edges arrive between desktop updates,
        // and duplication never wakes for them.
        let fx_config = cursorfx::cursor_fx_for(&fx_id);
        let wants_input = fx_config
            .as_ref()
            .is_some_and(|c| c.ripples || c.keystrokes);
        let timeout = if wants_input {
            FX_ACQUIRE_TIMEOUT_MS
        } else {
            ACQUIRE_TIMEOUT_MS
        };

        let mut info = DXGI_OUTDUPL_FRAME_INFO::default();
        let mut resource: Option<IDXGIResource> = None;
        // SAFETY: out-params are locals; on success the frame is released
        // below before the next acquire.
        let acquired = unsafe {
            session
                .duplication
                .AcquireNextFrame(timeout, &mut info, &mut resource)
        };

        let got_update = match acquired {
            Ok(()) => true,
            // A timeout means "no desktop change" — the effects may still
            // need a synthesized frame, so fall through instead of looping.
            Err(err) if err.code() == DXGI_ERROR_WAIT_TIMEOUT => false,
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
        };

        let result: Result<Option<Frame>, CaptureError> = if got_update {
            let copied = (|| -> Result<Option<Frame>, CaptureError> {
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

                let staging_tex = ensure_staging(
                    &session.device,
                    &mut staging,
                    desc.Width,
                    desc.Height,
                    desc.Format,
                )?;
                // SAFETY: same-device resources with identical dimensions/format.
                unsafe { session.context.CopyResource(staging_tex, &texture) };
                Ok(Some(read_staging(
                    &session.context,
                    staging_tex,
                    desc.Width,
                    desc.Height,
                    desc.Format,
                    device_name,
                )?))
            })();
            // SAFETY: every successful acquire is paired with exactly one release.
            let _ = unsafe { session.duplication.ReleaseFrame() };
            copied
        } else {
            Ok(None)
        };

        // One effects tick per loop pass — press edges spawn ripples, dead
        // ones age out, key badges follow the held set (CAP-N19). Sampling
        // happens ONLY while a config enables it.
        let now = Instant::now();
        let fx_dirty = fx_tick(
            &mut fx,
            fx_config.as_ref(),
            pointer.x,
            pointer.y,
            pointer.visible,
            now,
        );

        match result {
            Ok(Some(mut frame)) => {
                last_base = Some(frame.clone());
                draw_overlays(
                    &mut frame,
                    &pointer,
                    &fx,
                    &mut ghost,
                    fx_config.as_ref(),
                    now,
                );
                last_drawn = pointer.key();
                sender.send(frame);
            }
            Ok(None) => {
                // Pointer-only update (or an effects-only change): synthesize
                // a frame from the last desktop image so the cursor — and any
                // animating ripple — still tracks.
                if pointer.key() != last_drawn || fx_dirty {
                    if let Some(base) = last_base.as_ref() {
                        let mut frame = base.clone();
                        frame.captured_at = now;
                        draw_overlays(
                            &mut frame,
                            &pointer,
                            &fx,
                            &mut ghost,
                            fx_config.as_ref(),
                            now,
                        );
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

/// Blend the cursor and any enabled cursor effects (CAP-N19) into a frame
/// about to be published.
fn draw_overlays(
    frame: &mut Frame,
    pointer: &PointerState,
    fx: &FxState,
    ghost: &mut KeyGhost,
    config: Option<&CursorFxConfig>,
    now: Instant,
) {
    blend_pointer(frame, pointer);
    if let Some(config) = config {
        fx_draw(
            frame,
            fx,
            ghost,
            config,
            pointer.x,
            pointer.y,
            pointer.visible,
            now,
        );
    }
}

fn ensure_staging<'t>(
    device: &ID3D11Device,
    staging: &'t mut Option<(u32, u32, DXGI_FORMAT, ID3D11Texture2D)>,
    width: u32,
    height: u32,
    format: DXGI_FORMAT,
) -> Result<&'t ID3D11Texture2D, CaptureError> {
    // Keyed on format too: toggling Windows HDR mid-session flips the
    // desktop between BGRA8 and FP16, and the staging must follow.
    let needs_new =
        !matches!(staging, Some((w, h, f, _)) if *w == width && *h == height && *f == format);
    if needs_new {
        let desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: format,
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
        *staging = Some((width, height, format, texture));
    }
    Ok(&staging.as_ref().expect("staging just ensured").3)
}

fn read_staging(
    context: &ID3D11DeviceContext,
    staging: &ID3D11Texture2D,
    width: u32,
    height: u32,
    format: DXGI_FORMAT,
    device_name: &str,
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
    let raw = unsafe { std::slice::from_raw_parts(mapped.pData as *const u8, len) };
    let frame = if format == DXGI_FORMAT_R16G16B16A16_FLOAT {
        // An HDR desktop (CAP-N74): scRGB FP16 → SDR BGRA8 through the
        // configured tone-map, row by row, so everything downstream keeps
        // seeing ordinary 8-bit frames. The registry read is live — a
        // paper-white change applies on the very next frame.
        let config = crate::tonemap::tone_map_for(&format!("display:{device_name}"));
        let mut data = vec![0u8; width as usize * height as usize * 4];
        for y in 0..height as usize {
            let src = &raw[y * stride as usize..][..width as usize * 8];
            let dst = &mut data[y * width as usize * 4..][..width as usize * 4];
            crate::tonemap::convert_row_fp16_to_bgra(src, dst, width as usize, &config);
        }
        Frame {
            width,
            height,
            stride: width * 4,
            format: PixelFormat::Bgra8,
            data,
            captured_at: Instant::now(),
        }
    } else {
        Frame {
            width,
            height,
            stride,
            format: PixelFormat::Bgra8,
            data: raw.to_vec(),
            captured_at: Instant::now(),
        }
    };
    // SAFETY: paired with the successful Map above.
    unsafe { context.Unmap(staging, 0) };
    Ok(frame)
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
