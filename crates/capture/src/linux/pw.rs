//! Consume the portal's PipeWire video node and publish [`Frame`]s.
//!
//! Follows the canonical pipewire-rs video consumer shape (`examples/
//! streams.rs`): negotiate a raw BGRx/RGBx/BGRA/RGBA format, then copy each
//! dequeued buffer out under the negotiated stride. A 100 ms loop timer
//! watches the stop flag and quits the main loop. All safe Rust — the
//! `pipewire` crate owns the FFI.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use pipew::properties::properties;
use pipew::spa;
use pipewire as pipew;
use spa::param::format::{FormatProperties, MediaSubtype, MediaType};
use spa::param::video::{VideoFormat, VideoInfoRaw};
use spa::pod::{Pod, Value};
use spa::utils::{Direction, Fraction, Rectangle, SpaTypes};

use super::portal::PortalStream;
use crate::{CaptureError, Frame, FrameSender, PixelFormat};

#[derive(Default)]
struct StreamData {
    format: VideoInfoRaw,
    have_format: bool,
}

pub(crate) fn run(portal: PortalStream, sender: FrameSender, stop: Arc<AtomicBool>) {
    match run_inner(portal, &sender, &stop) {
        Ok(()) => sender.close(None),
        Err(err) => sender.close(Some(err)),
    }
}

fn run_inner(
    portal: PortalStream,
    sender: &FrameSender,
    stop: &Arc<AtomicBool>,
) -> Result<(), CaptureError> {
    pipew::init();

    let main_loop = pipew::main_loop::MainLoopRc::new(None)
        .map_err(|err| CaptureError::Backend(format!("pipewire main loop: {err}")))?;
    let context = pipew::context::ContextRc::new(&main_loop, None)
        .map_err(|err| CaptureError::Backend(format!("pipewire context: {err}")))?;
    let core = context
        .connect_fd_rc(portal.fd, None)
        .map_err(|err| CaptureError::Backend(format!("pipewire connect: {err}")))?;

    let stream = pipew::stream::StreamBox::new(
        &core,
        "freally-capture",
        properties! {
            *pipew::keys::MEDIA_TYPE => "Video",
            *pipew::keys::MEDIA_CATEGORY => "Capture",
            *pipew::keys::MEDIA_ROLE => "Screen",
        },
    )
    .map_err(|err| CaptureError::Backend(format!("pipewire stream: {err}")))?;

    let process_sender = sender.clone();
    let _listener = stream
        .add_local_listener_with_user_data(StreamData::default())
        .param_changed(|_, data, id, param| {
            let Some(param) = param else {
                return;
            };
            if id != spa::param::ParamType::Format.as_raw() {
                return;
            }
            let Ok((media_type, media_subtype)) = spa::param::format_utils::parse_format(param)
            else {
                return;
            };
            if media_type != MediaType::Video || media_subtype != MediaSubtype::Raw {
                return;
            }
            data.have_format = data.format.parse(param).is_ok();
        })
        .process(move |stream, data| {
            if !data.have_format {
                return;
            }
            let Some(mut buffer) = stream.dequeue_buffer() else {
                return;
            };
            let datas = buffer.datas_mut();
            let Some(plane) = datas.first_mut() else {
                return;
            };
            let chunk_stride = plane.chunk().stride();
            let Some(bytes) = plane.data() else {
                return;
            };

            let size = data.format.size();
            let (width, height) = (size.width, size.height);
            if width == 0 || height == 0 {
                return;
            }
            let stride = if chunk_stride > 0 {
                chunk_stride as u32
            } else {
                width * 4
            };
            let needed = stride as usize * height as usize;
            if bytes.len() < needed || stride < width * 4 {
                return;
            }

            let (format, opaque_alpha) = match data.format.format() {
                VideoFormat::BGRA => (PixelFormat::Bgra8, false),
                VideoFormat::BGRx => (PixelFormat::Bgra8, true),
                VideoFormat::RGBA => (PixelFormat::Rgba8, false),
                VideoFormat::RGBx => (PixelFormat::Rgba8, true),
                _ => return, // never negotiated — see the EnumFormat pod below
            };

            let mut out = bytes[..needed].to_vec();
            if opaque_alpha {
                // The 'x' byte is undefined — make the frame honestly opaque.
                for px in out.chunks_exact_mut(4) {
                    px[3] = 0xFF;
                }
            }
            process_sender.send(Frame {
                width,
                height,
                stride,
                format,
                data: out,
                captured_at: Instant::now(),
            });
        })
        .register()
        .map_err(|err| CaptureError::Backend(format!("pipewire listener: {err}")))?;

    // Ask for raw video in the 4-byte layouts we can forward without pixel
    // conversion; size/framerate stay ranges so the source side decides.
    let format_object = spa::pod::object!(
        SpaTypes::ObjectParamFormat,
        spa::param::ParamType::EnumFormat,
        spa::pod::property!(FormatProperties::MediaType, Id, MediaType::Video),
        spa::pod::property!(FormatProperties::MediaSubtype, Id, MediaSubtype::Raw),
        spa::pod::property!(
            FormatProperties::VideoFormat,
            Choice,
            Enum,
            Id,
            VideoFormat::BGRx,
            VideoFormat::BGRx,
            VideoFormat::BGRA,
            VideoFormat::RGBx,
            VideoFormat::RGBA
        ),
        spa::pod::property!(
            FormatProperties::VideoSize,
            Choice,
            Range,
            Rectangle,
            Rectangle {
                width: 1280,
                height: 720
            },
            Rectangle {
                width: 1,
                height: 1
            },
            Rectangle {
                width: 16384,
                height: 16384
            }
        ),
        spa::pod::property!(
            FormatProperties::VideoFramerate,
            Choice,
            Range,
            Fraction,
            Fraction { num: 30, denom: 1 },
            Fraction { num: 0, denom: 1 },
            Fraction {
                num: 1000,
                denom: 1
            }
        ),
    );
    let values = spa::pod::serialize::PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &Value::Object(format_object),
    )
    .map_err(|err| CaptureError::Backend(format!("format pod: {err:?}")))?
    .0
    .into_inner();
    let mut params = [Pod::from_bytes(&values)
        .ok_or_else(|| CaptureError::Backend("format pod: invalid bytes".into()))?];

    stream
        .connect(
            Direction::Input,
            Some(portal.node_id),
            pipew::stream::StreamFlags::AUTOCONNECT | pipew::stream::StreamFlags::MAP_BUFFERS,
            &mut params,
        )
        .map_err(|err| CaptureError::Backend(format!("pipewire connect stream: {err}")))?;

    // Poll the stop flag from inside the loop; quit when asked.
    let weak_loop = main_loop.downgrade();
    let stop_watch = Arc::clone(stop);
    let watch_sender = sender.clone();
    let timer = main_loop.loop_().add_timer(move |_expirations| {
        if stop_watch.load(Ordering::Relaxed) || !watch_sender.is_open() {
            if let Some(main_loop) = weak_loop.upgrade() {
                main_loop.quit();
            }
        }
    });
    timer
        .update_timer(
            Some(Duration::from_millis(100)),
            Some(Duration::from_millis(100)),
        )
        .into_result()
        .map_err(|err| CaptureError::Backend(format!("stop timer: {err}")))?;

    main_loop.run();
    Ok(())
}
