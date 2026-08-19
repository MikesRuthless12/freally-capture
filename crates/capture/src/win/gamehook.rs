//! CAP-N78 — the app side of the game-capture hook.
//!
//! Opens the control block and shared texture an injected `freally-game-hook`
//! publishes for a game process, copies each new frame into a CPU staging
//! texture, and feeds the standard latest-wins [`CaptureSession`] channel — the
//! same shape `wgc` and `dxgi` already deliver.
//!
//! **This never injects anything.** Injection is the injector's job, behind the
//! per-title consent gate; this module only reads what an already-hooked game
//! publishes. A game that is not hooked is reported honestly so the caller can
//! fall back to WGC window capture, which stays the recommended path.
//!
//! AUDITED `unsafe`: D3D11 staging-texture map/unmap only.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use freally_game_hook::protocol::{
    control_name, texture_name, ControlBlock, FLAG_GEOMETRY_CHANGED, FLAG_PRODUCER_ALIVE,
    KEY_CONSUMER, KEY_PRODUCER,
};
use freally_game_hook::win::shmem::ControlMapping;
use freally_game_hook::win::texture::{create_device, SharedTexture};
use windows::Win32::Graphics::Direct3D11::{
    ID3D11Device, ID3D11DeviceContext, ID3D11Texture2D, D3D11_CPU_ACCESS_READ,
    D3D11_MAPPED_SUBRESOURCE, D3D11_MAP_READ, D3D11_TEXTURE2D_DESC, D3D11_USAGE_STAGING,
};
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT, DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_B8G8R8A8_UNORM_SRGB,
    DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_R8G8B8A8_UNORM_SRGB, DXGI_SAMPLE_DESC,
};

use crate::{frame_channel, CaptureError, CaptureSession, Frame, PixelFormat};

/// How long to wait for a hooked game to publish its first frame before saying
/// so. A game that is alt-tabbed or still loading simply is not presenting —
/// the caller reports that honestly rather than treating it as a failure.
const FIRST_FRAME_TIMEOUT: Duration = Duration::from_secs(5);

/// Poll interval while waiting for the next present. Short enough to track a
/// 240 Hz title, cheap enough to idle on.
const POLL: Duration = Duration::from_micros(500);

/// How often to ask the OS whether the hooked game is still running.
///
/// The producer cannot reliably announce its own death: a game that is killed
/// or crashes — the common case — never runs `DLL_PROCESS_DETACH`, and the last
/// control block it wrote survives in our own view of the mapping with
/// `FLAG_PRODUCER_ALIVE` still set. Without asking the OS the pump would spin
/// on the final frame forever and the studio would never auto-recover.
const LIVENESS_CHECK: Duration = Duration::from_millis(250);

/// A handle to the hooked game, used only to answer "is it still alive?".
#[allow(dead_code)]
struct GameProcess(windows::Win32::Foundation::HANDLE);

impl GameProcess {
    fn open(pid: u32) -> Option<Self> {
        use windows::Win32::System::Threading::{
            OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SYNCHRONIZE,
        };
        // SAFETY: a query-only handle to another process; closed on drop.
        let handle = unsafe {
            OpenProcess(
                PROCESS_SYNCHRONIZE | PROCESS_QUERY_LIMITED_INFORMATION,
                false,
                pid,
            )
        }
        .ok()?;
        (!handle.is_invalid()).then_some(Self(handle))
    }

    /// `false` once the process has exited.
    fn is_alive(&self) -> bool {
        use windows::Win32::Foundation::WAIT_TIMEOUT;
        use windows::Win32::System::Threading::WaitForSingleObject;
        // SAFETY: waiting with a zero timeout on our own process handle; the
        // process object is signalled exactly when it exits.
        unsafe { WaitForSingleObject(self.0, 0) == WAIT_TIMEOUT }
    }
}

impl Drop for GameProcess {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            // SAFETY: closed exactly once, on the handle this type opened.
            unsafe {
                let _ = windows::Win32::Foundation::CloseHandle(self.0);
            }
        }
    }
}

/// Shown when the chosen game has no hook installed — the honest, actionable
/// case, not an error. One string so the capture path and the UI never drift.
pub const NOT_HOOKED: &str =
    "this game is not hooked — enable Game Capture for it, or add it as a Window Capture";

/// Map a swap-chain format onto the two layouts the compositor consumes.
///
/// HDR back buffers (`R10G10B10A2`, `R16G16B16A16_FLOAT`) deliberately return
/// `None`: silently reinterpreting them as 8-bit would ship visibly wrong
/// colour, and the honest answer routes the user to WGC, which the desktop
/// compositor has already tone-mapped.
fn pixel_format(format: DXGI_FORMAT) -> Option<PixelFormat> {
    match format {
        DXGI_FORMAT_B8G8R8A8_UNORM | DXGI_FORMAT_B8G8R8A8_UNORM_SRGB => Some(PixelFormat::Bgra8),
        DXGI_FORMAT_R8G8B8A8_UNORM | DXGI_FORMAT_R8G8B8A8_UNORM_SRGB => Some(PixelFormat::Rgba8),
        _ => None,
    }
}

/// Is `pid` currently publishing hooked frames?
///
/// Used by the picker to show live state, and by `start_capture` to fail fast
/// with [`NOT_HOOKED`] instead of blocking on a game nobody hooked.
pub fn is_hooked(pid: u32) -> bool {
    match ControlMapping::open(&control_name(pid)) {
        Ok(Some(mapping)) => {
            let block = mapping.read();
            block.is_compatible() && block.flags & FLAG_PRODUCER_ALIVE != 0
        }
        _ => false,
    }
}

/// A CPU-readable copy target sized to the shared texture.
struct Staging {
    texture: ID3D11Texture2D,
    width: u32,
    height: u32,
    format: DXGI_FORMAT,
}

impl Staging {
    fn create(
        device: &ID3D11Device,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
    ) -> Result<Self, String> {
        let desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: format,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_STAGING,
            BindFlags: 0,
            CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
            MiscFlags: 0,
        };
        let mut texture: Option<ID3D11Texture2D> = None;
        // SAFETY: `desc` is fully initialised; the out-param is the documented
        // shape for CreateTexture2D.
        unsafe { device.CreateTexture2D(&desc, None, Some(&mut texture)) }
            .map_err(|err| format!("could not create the staging texture: {err}"))?;
        Ok(Self {
            texture: texture.ok_or("CreateTexture2D returned no texture")?,
            width,
            height,
            format,
        })
    }

    fn matches(&self, width: u32, height: u32, format: DXGI_FORMAT) -> bool {
        self.width == width && self.height == height && self.format == format
    }

    /// Map the staging texture and copy its rows out as a tightly-strided
    /// buffer. The GPU's `RowPitch` is almost always padded, so the rows are
    /// copied one at a time rather than as one block.
    fn read_rows(&self, context: &ID3D11DeviceContext) -> Result<(Vec<u8>, u32), String> {
        // SAFETY: a STAGING texture with CPU_ACCESS_READ is mappable; the
        // matching Unmap runs on every path below.
        let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
        unsafe {
            context.Map(
                &self.texture,
                0,
                D3D11_MAP_READ,
                0,
                Some(&mut mapped as *mut _),
            )
        }
        .map_err(|err| format!("could not map the staging texture: {err}"))?;

        let row_bytes = (self.width as usize) * 4;
        let pitch = mapped.RowPitch as usize;
        let height = self.height as usize;
        let total = row_bytes * height;
        if mapped.pData.is_null() || pitch < row_bytes {
            // SAFETY: paired with the Map above.
            unsafe { context.Unmap(&self.texture, 0) };
            return Err("the mapped staging texture had an unusable layout".into());
        }
        // Every byte below is written, so the zeroing `vec![0u8; n]` would do
        // is pure waste — an 8 MB calloc per frame at 1080p. Reserve instead
        // and set the length once the rows are in.
        let mut data: Vec<u8> = Vec::with_capacity(total);
        let dst = data.as_mut_ptr();
        if pitch == row_bytes {
            // Tightly packed: the whole surface is one contiguous run.
            // SAFETY: `pData` covers `pitch * height` == `total` bytes here,
            // and `dst` has `total` bytes reserved.
            unsafe { std::ptr::copy_nonoverlapping(mapped.pData as *const u8, dst, total) };
        } else {
            for row in 0..height {
                // SAFETY: `pData` covers `pitch * height`; each read stays
                // inside one row, and the destination run is exactly
                // `row_bytes` within the `total` reserved bytes.
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (mapped.pData as *const u8).add(row * pitch),
                        dst.add(row * row_bytes),
                        row_bytes,
                    );
                }
            }
        }
        // SAFETY: both branches above initialise exactly `total` bytes.
        unsafe { data.set_len(total) };
        // SAFETY: paired with the Map above.
        unsafe { context.Unmap(&self.texture, 0) };
        Ok((data, row_bytes as u32))
    }
}

/// Start reading hooked frames from the game process `pid`.
///
/// Fails immediately with [`NOT_HOOKED`] when nothing is publishing, so the
/// caller can fall back to WGC rather than staring at a black source.
pub fn start_game_hook_capture(pid: u32) -> Result<CaptureSession, CaptureError> {
    let control = ControlMapping::open(&control_name(pid))
        .map_err(CaptureError::Backend)?
        .ok_or_else(|| CaptureError::Backend(NOT_HOOKED.into()))?;
    if !control.read().is_compatible() {
        return Err(CaptureError::Backend(
            "the injected game hook is a different protocol version — reinstall Freally Capture"
                .into(),
        ));
    }

    let device = create_device().map_err(CaptureError::Backend)?;
    // SAFETY: GetImmediateContext returns this device's context.
    let context = unsafe { device.GetImmediateContext() }
        .map_err(|err| CaptureError::Backend(format!("no D3D11 context: {err}")))?;

    let (sender, receiver) = frame_channel();
    let stop = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop);

    let join = std::thread::Builder::new()
        .name("fcap-gamehook".into())
        .spawn(move || {
            pump(pid, control, device, context, sender, thread_stop);
        })
        .map_err(|err| CaptureError::Backend(err.to_string()))?;

    Ok(CaptureSession::from_parts(receiver, stop, join))
}

/// The frame pump: wait for `frame_index` to advance, copy, publish.
fn pump(
    pid: u32,
    control: ControlMapping,
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    sender: crate::FrameSender,
    stop: Arc<AtomicBool>,
) {
    let mut shared: Option<SharedTexture> = None;
    let mut staging: Option<Staging> = None;
    let mut last_index = 0u64;
    let started = Instant::now();
    let mut ever_delivered = false;
    let game = GameProcess::open(pid);
    let mut last_liveness = Instant::now();

    while !stop.load(Ordering::Relaxed) && sender.is_open() {
        // Ask the OS whether the game is still there — the control block's
        // FLAG_PRODUCER_ALIVE cannot answer this (see LIVENESS_CHECK).
        if last_liveness.elapsed() >= LIVENESS_CHECK {
            last_liveness = Instant::now();
            if let Some(game) = game.as_ref() {
                if !game.is_alive() {
                    // The game exited. Close the source so the studio's
                    // auto-recovery takes over instead of freezing on the
                    // last frame forever.
                    break;
                }
            }
        }
        let block: ControlBlock = control.read();
        if !block.has_frame() {
            // Producer gone entirely: stop, and let the studio auto-recover.
            if block.is_compatible() && block.flags == 0 && ever_delivered {
                break;
            }
            if !ever_delivered && started.elapsed() > FIRST_FRAME_TIMEOUT {
                // Not an error: the game simply is not presenting yet.
                sender.close(Some(CaptureError::Backend(
                    "the hooked game has not presented a frame yet".into(),
                )));
                return;
            }
            std::thread::sleep(POLL);
            continue;
        }
        if block.frame_index == last_index {
            std::thread::sleep(POLL);
            continue;
        }

        // Geometry change (resize / fullscreen toggle) invalidates both sides.
        let format = DXGI_FORMAT(block.format as i32);
        let stale = shared
            .as_ref()
            .map(|tex| {
                tex.width != block.width || tex.height != block.height || tex.format != format
            })
            .unwrap_or(true);
        if stale || block.flags & FLAG_GEOMETRY_CHANGED != 0 {
            shared = SharedTexture::open(&device, &texture_name(pid)).ok();
            staging = None;
        }
        let Some(tex) = shared.as_ref() else {
            std::thread::sleep(POLL);
            continue;
        };

        let Some(layout) = pixel_format(tex.format) else {
            sender.close(Some(CaptureError::Unsupported(format!(
                "this game presents in an HDR format ({:?}) the hook cannot convert — \
                 use Window Capture, which the desktop compositor tone-maps",
                tex.format
            ))));
            return;
        };

        if staging
            .as_ref()
            .map(|s| !s.matches(tex.width, tex.height, tex.format))
            .unwrap_or(true)
        {
            match Staging::create(&device, tex.width, tex.height, tex.format) {
                Ok(next) => staging = Some(next),
                Err(err) => {
                    sender.close(Some(CaptureError::Backend(err)));
                    return;
                }
            }
        }
        let Some(stage) = staging.as_ref() else {
            continue;
        };

        // Copy out under the consumer key; a contended frame is skipped, never
        // blocked — the game must never stall on us.
        let copied = tex.with_lock(KEY_CONSUMER, KEY_PRODUCER, || {
            // SAFETY: both textures are this device's, same size and format.
            unsafe { context.CopyResource(&stage.texture, &tex.texture) };
        });
        match copied {
            Ok(Some(())) => {}
            Ok(None) => {
                std::thread::sleep(POLL);
                continue;
            }
            Err(err) => {
                sender.close(Some(CaptureError::Backend(err)));
                return;
            }
        }

        match stage.read_rows(&context) {
            Ok((data, stride)) => {
                last_index = block.frame_index;
                ever_delivered = true;
                sender.send(Frame {
                    width: tex.width,
                    height: tex.height,
                    stride,
                    format: layout,
                    data,
                    captured_at: Instant::now(),
                });
            }
            Err(err) => {
                sender.close(Some(CaptureError::Backend(err)));
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_eight_bit_layouts_are_accepted() {
        assert_eq!(
            pixel_format(DXGI_FORMAT_B8G8R8A8_UNORM),
            Some(PixelFormat::Bgra8)
        );
        assert_eq!(
            pixel_format(DXGI_FORMAT_B8G8R8A8_UNORM_SRGB),
            Some(PixelFormat::Bgra8),
            "sRGB is the same byte layout"
        );
        assert_eq!(
            pixel_format(DXGI_FORMAT_R8G8B8A8_UNORM),
            Some(PixelFormat::Rgba8)
        );
    }

    #[test]
    fn hdr_back_buffers_are_refused_rather_than_misread() {
        use windows::Win32::Graphics::Dxgi::Common::{
            DXGI_FORMAT_R10G10B10A2_UNORM, DXGI_FORMAT_R16G16B16A16_FLOAT,
        };
        // Reinterpreting 10-bit or half-float as 8-bit would ship visibly wrong
        // colour; None routes the caller to the tone-mapped WGC path instead.
        assert_eq!(pixel_format(DXGI_FORMAT_R10G10B10A2_UNORM), None);
        assert_eq!(pixel_format(DXGI_FORMAT_R16G16B16A16_FLOAT), None);
    }

    #[test]
    fn an_unhooked_process_is_reported_not_hooked() {
        // A pid nothing has ever hooked: honest false, no panic, no error.
        assert!(!is_hooked(0xDEAD_BEEF));
    }

    #[test]
    fn starting_on_an_unhooked_process_fails_with_the_fallback_message() {
        let Err(err) = start_game_hook_capture(0xDEAD_BEEF) else {
            panic!("nothing is hooked, so this must not start a session");
        };
        let text = format!("{err}");
        assert!(
            text.contains("Window Capture"),
            "the error must route to the working fallback, got: {text}"
        );
    }
}
