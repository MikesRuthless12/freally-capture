//! The named shared texture: the producer creates it, the consumer opens it by
//! name. Both sides live here so the creation flags and the keyed-mutex
//! handshake can never be specified twice and drift.
//!
//! AUDITED `unsafe`: D3D11/DXGI COM interop only.

use windows::core::{Interface, HSTRING};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Graphics::Direct3D11::{
    ID3D11Device, ID3D11Device1, ID3D11Texture2D, D3D11_BIND_RENDER_TARGET,
    D3D11_BIND_SHADER_RESOURCE, D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX,
    D3D11_RESOURCE_MISC_SHARED_NTHANDLE, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT,
};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT, DXGI_SAMPLE_DESC};
use windows::Win32::Graphics::Dxgi::{IDXGIKeyedMutex, IDXGIResource1, DXGI_SHARED_RESOURCE_READ};

/// How long either side waits for the keyed mutex before giving up on a frame.
///
/// Deliberately tiny. The protocol's stall rule is that the **game must never
/// be delayed** because the capture app is slow or has died holding the key —
/// a skipped frame is always better than a stuttering game.
pub const MUTEX_TIMEOUT_MS: u32 = 4;

/// A shared texture the game presents into.
pub struct SharedTexture {
    pub texture: ID3D11Texture2D,
    pub mutex: IDXGIKeyedMutex,
    pub width: u32,
    pub height: u32,
    pub format: DXGI_FORMAT,
    /// The producer's named NT handle, closed on drop.
    ///
    /// It MUST be owned: the name stays registered for exactly as long as this
    /// handle is open, so leaking it means the next `create` under the same
    /// name fails with `DXGI_ERROR_NAME_ALREADY_EXISTS` — which is every
    /// geometry change (resize, fullscreen toggle, DPI change). `None` on the
    /// consumer side, which opens the name rather than owning it.
    shared_handle: Option<HANDLE>,
}

impl Drop for SharedTexture {
    fn drop(&mut self) {
        if let Some(handle) = self.shared_handle.take() {
            if !handle.is_invalid() {
                // SAFETY: a handle this type created and has not closed; the
                // texture it names is released with the COM pointers above.
                unsafe {
                    let _ = CloseHandle(handle);
                }
            }
        }
    }
}

impl SharedTexture {
    /// Producer side: create the texture and publish it under `name`.
    pub fn create(
        device: &ID3D11Device,
        name: &str,
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
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: (D3D11_BIND_SHADER_RESOURCE.0 | D3D11_BIND_RENDER_TARGET.0) as u32,
            CPUAccessFlags: 0,
            // NTHANDLE + KEYEDMUTEX is what lets a *different* process open this
            // by name and synchronise against the producer.
            MiscFlags: (D3D11_RESOURCE_MISC_SHARED_NTHANDLE.0
                | D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX.0) as u32,
        };
        let mut texture: Option<ID3D11Texture2D> = None;
        // SAFETY: `desc` is fully initialised; the out-param is the documented
        // shape for CreateTexture2D.
        unsafe { device.CreateTexture2D(&desc, None, Some(&mut texture)) }
            .map_err(|err| format!("could not create the shared texture: {err}"))?;
        let texture = texture.ok_or("CreateTexture2D returned no texture")?;

        let resource: IDXGIResource1 = texture
            .cast()
            .map_err(|err| format!("shared texture is not an IDXGIResource1: {err}"))?;
        let wide = HSTRING::from(name);
        // SAFETY: naming the shared handle is what avoids cross-process handle
        // duplication entirely — the consumer opens the same name read-only.
        // Keep the handle: the name lives exactly as long as it is open, so
        // dropping it here would leak the name and make every later `create`
        // under it fail with DXGI_ERROR_NAME_ALREADY_EXISTS.
        let shared_handle =
            unsafe { resource.CreateSharedHandle(None, DXGI_SHARED_RESOURCE_READ.0, &wide) }
                .map_err(|err| format!("could not publish the shared texture: {err}"))?;

        let mutex: IDXGIKeyedMutex = texture
            .cast()
            .map_err(|err| format!("shared texture has no keyed mutex: {err}"))?;
        Ok(Self {
            texture,
            mutex,
            width,
            height,
            format,
            shared_handle: Some(shared_handle),
        })
    }

    /// Consumer side: open a texture the producer published under `name`.
    pub fn open(device: &ID3D11Device, name: &str) -> Result<Self, String> {
        let device1: ID3D11Device1 = device
            .cast()
            .map_err(|err| format!("D3D11.1 is required to open shared textures: {err}"))?;
        let wide = HSTRING::from(name);
        // SAFETY: opening an existing named shared resource; the call fails
        // cleanly (Err) when the producer has not published one.
        let texture: ID3D11Texture2D =
            unsafe { device1.OpenSharedResourceByName(&wide, DXGI_SHARED_RESOURCE_READ.0) }
                .map_err(|err| format!("could not open the shared texture: {err}"))?;

        let mut desc = D3D11_TEXTURE2D_DESC::default();
        // SAFETY: GetDesc fills the struct; no ownership transfer.
        unsafe { texture.GetDesc(&mut desc) };
        let mutex: IDXGIKeyedMutex = texture
            .cast()
            .map_err(|err| format!("shared texture has no keyed mutex: {err}"))?;
        Ok(Self {
            texture,
            mutex,
            width: desc.Width,
            height: desc.Height,
            format: desc.Format,
            // The consumer opened the name; it does not own it.
            shared_handle: None,
        })
    }

    /// Take the keyed mutex at `key`, run `body`, then release at `release_key`.
    ///
    /// Returns `Ok(None)` when the lock was not free within [`MUTEX_TIMEOUT_MS`]
    /// — the caller skips the frame. That is the normal, non-error path when the
    /// other side is mid-copy.
    pub fn with_lock<T>(
        &self,
        key: u64,
        release_key: u64,
        body: impl FnOnce() -> T,
    ) -> Result<Option<T>, String> {
        // The raw vtable, deliberately: the safe wrapper is `HRESULT::ok()`,
        // which only fails on the severity bit — and WAIT_TIMEOUT (0x102) has
        // it CLEAR. Through the wrapper a timed-out acquire is indistinguishable
        // from a successful one, so the producer would happily paint into a
        // texture the consumer is reading. Inspect the HRESULT ourselves.
        //
        // SAFETY: the mutex belongs to this texture; every acquire below has a
        // matching release on the success paths, and the skip path took nothing.
        let hr = unsafe {
            (Interface::vtable(&self.mutex).AcquireSync)(
                Interface::as_raw(&self.mutex),
                key,
                MUTEX_TIMEOUT_MS,
            )
        };
        match hr.0 {
            // S_OK: ours.
            0 => {}
            // WAIT_ABANDONED: the previous owner died holding the key. DXGI
            // still grants us ownership, so this is an acquire, not a failure —
            // exactly the case the protocol's stall rule exists for.
            0x0000_0080 => {}
            // WAIT_TIMEOUT: someone else has it this frame. Skip, never block.
            0x0000_0102 => return Ok(None),
            _ => return Err(format!("could not acquire the shared texture: {hr:?}")),
        }
        let out = body();
        // SAFETY: paired with the AcquireSync above.
        unsafe { self.mutex.ReleaseSync(release_key) }
            .map_err(|err| format!("could not release the shared texture: {err}"))?;
        Ok(Some(out))
    }
}

/// Create a plain hardware D3D11 device. Used by the consumer (the producer
/// borrows the game's own device) and by the tests.
pub fn create_device() -> Result<ID3D11Device, String> {
    use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_HARDWARE;
    use windows::Win32::Graphics::Direct3D11::{
        D3D11CreateDevice, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION,
    };
    let mut device: Option<ID3D11Device> = None;
    // SAFETY: standard device creation; every out-param is an Option we own.
    unsafe {
        D3D11CreateDevice(
            None::<&windows::Win32::Graphics::Dxgi::IDXGIAdapter>,
            D3D_DRIVER_TYPE_HARDWARE,
            windows::Win32::Foundation::HMODULE::default(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            None,
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            None,
        )
    }
    .map_err(|err| format!("could not create a D3D11 device: {err}"))?;
    device.ok_or_else(|| "D3D11CreateDevice returned no device".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{texture_name, KEY_CONSUMER, KEY_PRODUCER};
    use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;

    #[test]
    fn a_named_shared_texture_round_trips_between_devices() {
        // Needs a real GPU; skip cleanly on a headless runner rather than
        // failing CI for a missing adapter.
        let Ok(producer_device) = create_device() else {
            eprintln!("no D3D11 hardware device — skipping");
            return;
        };
        let name = texture_name(std::process::id());
        let produced = SharedTexture::create(
            &producer_device,
            &name,
            1920,
            1080,
            DXGI_FORMAT_B8G8R8A8_UNORM,
        )
        .expect("create the shared texture");

        // A SEPARATE device, exactly like the app opening a game's texture.
        let consumer_device = create_device().expect("second device");
        let opened = SharedTexture::open(&consumer_device, &name).expect("open by name");

        assert_eq!((opened.width, opened.height), (1920, 1080));
        assert_eq!(opened.format, DXGI_FORMAT_B8G8R8A8_UNORM);
        assert_eq!(
            (produced.width, produced.height),
            (opened.width, opened.height)
        );
    }

    #[test]
    fn the_keyed_mutex_hands_off_between_producer_and_consumer() {
        let Ok(device) = create_device() else {
            eprintln!("no D3D11 hardware device — skipping");
            return;
        };
        let name = format!("{}-handoff", texture_name(std::process::id()));
        let tex = SharedTexture::create(&device, &name, 64, 64, DXGI_FORMAT_B8G8R8A8_UNORM)
            .expect("create");

        // The producer starts owning key 0 and hands off as key 1.
        let wrote = tex
            .with_lock(KEY_PRODUCER, KEY_CONSUMER, || "painted")
            .expect("no lock error");
        assert_eq!(wrote, Some("painted"));

        // Now only the consumer key opens it; the producer key must time out
        // rather than deadlock the game.
        let consumed = tex
            .with_lock(KEY_CONSUMER, KEY_PRODUCER, || "copied")
            .expect("no lock error");
        assert_eq!(consumed, Some("copied"));

        // Back to the producer's turn — and a consumer acquire now finds it
        // taken and skips, which is the documented non-error path.
        let skipped = tex
            .with_lock(KEY_CONSUMER, KEY_PRODUCER, || "should not run")
            .expect("a busy mutex is not an error");
        assert_eq!(skipped, None, "a contended frame is skipped, never blocked");
    }
}
