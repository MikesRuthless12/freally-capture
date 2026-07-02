//! The org.freedesktop.portal.ScreenCast negotiation via `ashpd`.
//!
//! Blocking wrapper around the async portal dance: create a session, ask for
//! monitor-or-window sources (the *system dialog* does the picking — the only
//! model Wayland permits), start, and hand back the PipeWire remote fd + node
//! id that `pw::run` consumes.

use std::os::fd::OwnedFd;

use ashpd::desktop::screencast::{
    CursorMode, OpenPipeWireRemoteOptions, Screencast, SelectSourcesOptions, SourceType,
    StartCastOptions,
};
use ashpd::desktop::{CreateSessionOptions, PersistMode};

use crate::CaptureError;

/// What the portal granted: a PipeWire connection + the video node inside it.
pub(crate) struct PortalStream {
    pub fd: OwnedFd,
    pub node_id: u32,
}

pub(crate) fn open_portal_stream() -> Result<PortalStream, CaptureError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|err| CaptureError::Backend(format!("tokio runtime: {err}")))?;
    runtime.block_on(negotiate())
}

async fn negotiate() -> Result<PortalStream, CaptureError> {
    let proxy = Screencast::new().await.map_err(portal_error)?;
    let session = proxy
        .create_session(CreateSessionOptions::default())
        .await
        .map_err(portal_error)?;
    proxy
        .select_sources(
            &session,
            SelectSourcesOptions::default()
                .set_multiple(false)
                .set_cursor_mode(CursorMode::Embedded)
                .set_sources(SourceType::Monitor | SourceType::Window)
                .set_persist_mode(PersistMode::DoNot),
        )
        .await
        .map_err(portal_error)?
        .response()
        .map_err(portal_error)?;

    // This is where the system picker appears; it resolves when the user
    // chooses a screen/window or cancels.
    let streams = proxy
        .start(&session, None, StartCastOptions::default())
        .await
        .map_err(portal_error)?
        .response()
        .map_err(portal_error)?;

    let stream = streams.streams().first().ok_or(CaptureError::Cancelled)?;
    let node_id = stream.pipe_wire_node_id();

    let fd = proxy
        .open_pipe_wire_remote(&session, OpenPipeWireRemoteOptions::default())
        .await
        .map_err(portal_error)?;

    Ok(PortalStream { fd, node_id })
}

fn portal_error(err: ashpd::Error) -> CaptureError {
    match &err {
        ashpd::Error::Response(ashpd::desktop::ResponseError::Cancelled) => CaptureError::Cancelled,
        ashpd::Error::PortalNotFound(_) => CaptureError::Unsupported(
            "the ScreenCast portal is not available — install xdg-desktop-portal (plus your \
             desktop's backend) to capture on Wayland"
                .into(),
        ),
        _ => CaptureError::Backend(format!("screencast portal: {err}")),
    }
}
