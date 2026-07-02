//! Per-window capture via Windows.Graphics.Capture (Windows 10 1903+).
//!
//! A free-threaded frame pool delivers frames on WinRT worker threads; the
//! `FrameArrived` handler copies each surface through a CPU staging texture
//! and publishes BGRA [`Frame`]s. The spawning thread just parks until the
//! stop flag (or the window closing) ends the session.
//!
//! AUDITED `unsafe`: WinRT interop (item creation, Direct3D device bridging,
//! surface → texture access) and the mapped-staging read; see `win/mod.rs`.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use windows::core::Interface;
use windows::Foundation::TypedEventHandler;
use windows::Graphics::Capture::{
    Direct3D11CaptureFramePool, GraphicsCaptureItem, GraphicsCaptureSession,
};
use windows::Graphics::DirectX::Direct3D11::IDirect3DDevice;
use windows::Graphics::DirectX::DirectXPixelFormat;
use windows::Graphics::SizeInt32;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D11::{
    ID3D11Device, ID3D11DeviceContext, ID3D11Texture2D, D3D11_BOX, D3D11_CPU_ACCESS_READ,
    D3D11_MAPPED_SUBRESOURCE, D3D11_MAP_READ, D3D11_TEXTURE2D_DESC, D3D11_USAGE_STAGING,
};
use windows::Win32::Graphics::Dxgi::IDXGIDevice;
use windows::Win32::System::WinRT::Direct3D11::{
    CreateDirect3D11DeviceFromDXGIDevice, IDirect3DDxgiInterfaceAccess,
};
use windows::Win32::System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop;
use windows::Win32::System::WinRT::{RoInitialize, RO_INIT_MULTITHREADED};

use crate::{CaptureError, Frame, FrameSender, PixelFormat};

/// Everything the FrameArrived handler needs, serialized behind one lock
/// (WinRT may deliver callbacks from multiple worker threads).
struct CopyContext {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    winrt_device: IDirect3DDevice,
    staging: Option<(u32, u32, ID3D11Texture2D)>,
    pool_size: SizeInt32,
}

// SAFETY: windows-rs interfaces deliberately don't implement Send. These
// D3D11 objects are free-threaded (the device is created *without*
// D3D11_CREATE_DEVICE_SINGLETHREADED, and the WinRT device wrapper is agile);
// the immediate context — the only non-concurrent-safe piece — is only ever
// touched behind the enclosing `Mutex`, one thread at a time.
unsafe impl Send for CopyContext {}

pub(crate) fn run(hwnd_raw: isize, sender: FrameSender, stop: Arc<AtomicBool>) {
    match run_inner(hwnd_raw, &sender, &stop) {
        Ok(()) => sender.close(None),
        Err(err) => sender.close(Some(err)),
    }
}

fn run_inner(hwnd_raw: isize, sender: &FrameSender, stop: &AtomicBool) -> Result<(), CaptureError> {
    // SAFETY: WinRT init for this thread; S_FALSE/RPC_E_CHANGED_MODE just
    // mean "already initialized", which is fine.
    let _ = unsafe { RoInitialize(RO_INIT_MULTITHREADED) };

    if !GraphicsCaptureSession::IsSupported().unwrap_or(false) {
        return Err(CaptureError::Unsupported(
            "window capture needs Windows 10 1903 or newer".into(),
        ));
    }

    let hwnd = HWND(hwnd_raw as *mut core::ffi::c_void);
    let interop = windows::core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>()
        .map_err(|err| CaptureError::Backend(format!("capture interop: {err}")))?;
    // SAFETY: CreateForWindow validates the HWND; a dead window errors here.
    let item: GraphicsCaptureItem = unsafe { interop.CreateForWindow(hwnd) }
        .map_err(|_| CaptureError::NotFound("the window can no longer be captured".into()))?;

    let (device, context) = super::create_d3d_device(None)?;
    let dxgi_device: IDXGIDevice = device
        .cast()
        .map_err(|err| CaptureError::Backend(format!("IDXGIDevice: {err}")))?;
    // SAFETY: standard WinRT interop over a valid DXGI device.
    let inspectable = unsafe { CreateDirect3D11DeviceFromDXGIDevice(&dxgi_device) }
        .map_err(|err| CaptureError::Backend(format!("WinRT D3D device: {err}")))?;
    let winrt_device: IDirect3DDevice = inspectable
        .cast()
        .map_err(|err| CaptureError::Backend(format!("IDirect3DDevice: {err}")))?;

    let size = item
        .Size()
        .map_err(|err| CaptureError::Backend(format!("item size: {err}")))?;
    let pool = Direct3D11CaptureFramePool::CreateFreeThreaded(
        &winrt_device,
        DirectXPixelFormat::B8G8R8A8UIntNormalized,
        2,
        size,
    )
    .map_err(|err| CaptureError::Backend(format!("frame pool: {err}")))?;

    let closed = Arc::new(AtomicBool::new(false));
    let closed_handler = Arc::clone(&closed);
    item.Closed(&TypedEventHandler::new(move |_, _| {
        closed_handler.store(true, Ordering::Relaxed);
        Ok(())
    }))
    .map_err(|err| CaptureError::Backend(format!("Closed handler: {err}")))?;

    let copy_ctx = Arc::new(Mutex::new(CopyContext {
        device,
        context,
        winrt_device,
        staging: None,
        pool_size: size,
    }));
    let handler_ctx = Arc::clone(&copy_ctx);
    let handler_sender = sender.clone();
    pool.FrameArrived(&TypedEventHandler::new(
        move |pool: &Option<Direct3D11CaptureFramePool>, _| {
            let Some(pool) = pool.as_ref() else {
                return Ok(());
            };
            if let Err(err) = on_frame(pool, &handler_ctx, &handler_sender) {
                handler_sender.close(Some(err));
            }
            Ok(())
        },
    ))
    .map_err(|err| CaptureError::Backend(format!("FrameArrived handler: {err}")))?;

    let session = pool
        .CreateCaptureSession(&item)
        .map_err(|err| CaptureError::Backend(format!("capture session: {err}")))?;
    session
        .StartCapture()
        .map_err(|err| CaptureError::Backend(format!("StartCapture: {err}")))?;

    // Frames now flow on WinRT worker threads; park here until told to stop,
    // the window closes, or the consumer goes away.
    while !stop.load(Ordering::Relaxed) && !closed.load(Ordering::Relaxed) && sender.is_open() {
        std::thread::sleep(Duration::from_millis(50));
    }

    let _ = session.Close();
    let _ = pool.Close();
    Ok(())
}

fn on_frame(
    pool: &Direct3D11CaptureFramePool,
    ctx: &Mutex<CopyContext>,
    sender: &FrameSender,
) -> Result<(), CaptureError> {
    let Ok(frame) = pool.TryGetNextFrame() else {
        return Ok(()); // spurious wakeup
    };
    let content = frame
        .ContentSize()
        .map_err(|err| CaptureError::Backend(format!("content size: {err}")))?;
    let surface = frame
        .Surface()
        .map_err(|err| CaptureError::Backend(format!("frame surface: {err}")))?;
    let access: IDirect3DDxgiInterfaceAccess = surface
        .cast()
        .map_err(|err| CaptureError::Backend(format!("surface access: {err}")))?;
    // SAFETY: the interop access yields the surface's backing texture.
    let texture: ID3D11Texture2D = unsafe { access.GetInterface() }
        .map_err(|err| CaptureError::Backend(format!("surface texture: {err}")))?;

    let mut ctx = ctx.lock().expect("wgc copy context poisoned");

    let mut desc = D3D11_TEXTURE2D_DESC::default();
    // SAFETY: GetDesc writes the local out-param.
    unsafe { texture.GetDesc(&mut desc) };
    // The pool texture is pool-sized; only ContentSize holds real pixels.
    let width = (content.Width.max(1) as u32).min(desc.Width);
    let height = (content.Height.max(1) as u32).min(desc.Height);

    ensure_staging(&mut ctx, width, height)?;
    let CopyContext {
        context, staging, ..
    } = &mut *ctx;
    let staging_tex = &staging.as_ref().expect("staging just ensured").2;

    let src_box = D3D11_BOX {
        left: 0,
        top: 0,
        front: 0,
        right: width,
        bottom: height,
        back: 1,
    };
    // SAFETY: same-device resources; the box is clamped to both textures.
    unsafe {
        context.CopySubresourceRegion(staging_tex, 0, 0, 0, 0, &texture, 0, Some(&src_box));
    }

    let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
    // SAFETY: staging is CPU-readable; pointer valid until Unmap.
    unsafe {
        context
            .Map(staging_tex, 0, D3D11_MAP_READ, 0, Some(&mut mapped))
            .map_err(|err| CaptureError::Backend(format!("Map staging: {err}")))?;
    }
    let stride = mapped.RowPitch;
    let len = stride as usize * height as usize;
    // SAFETY: bounds from the driver-reported RowPitch × copied height.
    let data = unsafe { std::slice::from_raw_parts(mapped.pData as *const u8, len) }.to_vec();
    // SAFETY: paired with the successful Map above.
    unsafe { context.Unmap(staging_tex, 0) };

    sender.send(Frame {
        width,
        height,
        stride,
        format: PixelFormat::Bgra8,
        data,
        captured_at: Instant::now(),
    });

    // Track window resizes: recreate the pool at the new content size so the
    // next frames aren't cropped or letterboxed.
    if content.Width > 0
        && content.Height > 0
        && (content.Width != ctx.pool_size.Width || content.Height != ctx.pool_size.Height)
    {
        pool.Recreate(
            &ctx.winrt_device,
            DirectXPixelFormat::B8G8R8A8UIntNormalized,
            2,
            content,
        )
        .map_err(|err| CaptureError::Backend(format!("pool recreate: {err}")))?;
        ctx.pool_size = content;
        ctx.staging = None;
    }
    Ok(())
}

fn ensure_staging(ctx: &mut CopyContext, width: u32, height: u32) -> Result<(), CaptureError> {
    if matches!(&ctx.staging, Some((w, h, _)) if *w == width && *h == height) {
        return Ok(());
    }
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
    // SAFETY: desc fully initialized; out-param is a local.
    unsafe { ctx.device.CreateTexture2D(&desc, None, Some(&mut texture)) }
        .map_err(|err| CaptureError::Backend(format!("staging texture: {err}")))?;
    let texture = texture.ok_or_else(|| CaptureError::Backend("staging texture missing".into()))?;
    ctx.staging = Some((width, height, texture));
    Ok(())
}
