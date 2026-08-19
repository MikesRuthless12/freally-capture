# Virtual-camera source component — CAP-N76 (Windows)

**Status: normative for the app↔source contract.** The app side lives in
`fcap-vcam-win` (`MfVirtualCamera`) + `fcap_stream::vcam::transport`; the source
DLL is the signed component this document specifies.

## Why a separate, registered DLL

Windows 11's `MFCreateVirtualCamera` does not stream frames from the calling
process. It wires a **registered frame-server media source** — a COM DLL the OS
`frameserver.exe` loads — into a camera that Zoom/Meet/Discord/OBS enumerate.
So the composed program cannot appear as a camera without a DLL that:

1. is registered under `HKCR\CLSID\{VCAM_SOURCE_CLSID}\InprocServer32` (the CLSID
   is fixed in `fcap-vcam-win`), and
2. implements `IMFMediaSource` + one `IMFMediaStream`, serving frames it reads
   from this app.

Because registration touches machine state and the DLL runs inside a system
host, it is distributed as a **signed, on-demand component** — the same trust
model as the CEF host binary: never bundled, installed explicitly, and its
absence is surfaced honestly (`available()` is false; the picker says "install
the Virtual Camera component"). No silent driver installs, ever.

## The frame contract (already built + tested)

`fcap_stream::vcam::transport` is the shared-memory format both sides use, and
it is fully unit-tested app-side today:

- Named region `Local\freally-vcam-<pid>` (the app's pid), created by the app.
- A 32-byte little-endian `FrameHeader` (`FVC1`, version, width, height,
  `bytes_per_slot`, `ready_slot`, `sequence`, `flags`) followed by **two** RGBA
  slots of `width*height*4` bytes each.
- The app writes the idle slot then publishes it by bumping `sequence` and
  pointing `ready_slot` at it; the source always reads `ready_slot`. Latest-wins,
  lock-free — a slow source never stalls the compositor, a mid-read source gets
  the previous frame, never a torn one.
- `flags & FLAG_STREAMING` tells the source whether the app is live; cleared on
  stop so the source shows no signal rather than a frozen last frame.

## What the source DLL does (the component milestone)

1. On activation, read the app pid from its configuration attribute store and
   open `Local\freally-vcam-<pid>`.
2. Expose one video stream at the header's geometry, RGB32/NV12 as the frame
   server negotiates (convert from the transport's RGBA).
3. Each `RequestSample`: read `ready_slot`, hand the frame server that buffer,
   stamped with a monotonic time. When `FLAG_STREAMING` is clear, serve the
   "no signal" slate.
4. Build + sign per the release pipeline; register on component install
   (per-user, `MFVirtualCameraAccess_CurrentUser` — no admin), deregister on
   removal.

## Lifecycle guarantees (enforced app-side today)

- `MfVirtualCamera::stop` calls `IMFVirtualCamera::Stop` **and** `Remove`, and
  `Drop` calls `stop` — a dropped or crashed session leaves no zombie device.
- Session lifetime (`MFVirtualCameraLifetime_Session`) means the camera never
  outlives the app process even if teardown is skipped.
- `available()` is a live registry probe, not a hardcoded flag: the UI reflects
  the real installed state.
