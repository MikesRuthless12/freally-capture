# Freally Capture — Privacy Policy

> **DRAFT — NOT YET LEGALLY REVIEWED.** Review by a qualified attorney is required
> before public distribution.

**Software:** Freally Capture · **Contact:** Mike Weaver
&lt;mythodikalone@gmail.com&gt;

## The short version
Freally Capture is **local-first and account-free**: **no accounts, no telemetry,
no analytics, and no cloud restreaming we run.** Your scenes, recordings, audio,
and webcam frames **never leave your computer** unless *you* stream, record,
share, or export them — and when you stream, your broadcast goes **directly** to
the platforms you configured, not through any server of ours.

## What we collect
**Nothing.** The Licensor does not collect, receive, store, or transmit:

- your screen, window, game, webcam, audio, or recorded files;
- your scenes, sources, filters, profiles, or settings;
- your stream keys or service credentials;
- any personal information, identifiers, or usage data.

All of your content and configuration stays on your device, in the folders and
configuration locations you control.

## Network use
Freally Capture works **fully offline** for building scenes, composing, recording
(in the owned `freally-video` lossless format), and playback. It uses the network
**only** for:

- **the stream targets you configure** — when you go live, the Software connects
  **directly** to the streaming services you chose (e.g. Twitch, YouTube, Kick),
  a self-hosted **SRT** ingest, or a **WHIP** (WebRTC) endpoint, and sends your
  broadcast to them, using the stream keys/tokens you entered. When you enable
  more than one target, each publishes **directly** to its platform. There is
  **no restream server we operate**; your video does not pass through us;
- **the remote-guests session you start** (opt-in) — if you invite remote guests,
  their camera/microphone/screen and yours travel **directly between the peers**
  over WebRTC, **encrypted in transit (DTLS-SRTP)** — there is **no media server
  we run**. Only the small connection handshake (peer ids and session
  descriptions, never media) crosses a public signaling broker, and an invite is
  an **expiring, host-chosen-lifetime** link; joining is always an **explicit
  click** — nothing auto-connects. If both sides sit behind strict NATs you may
  *optionally* configure your **own TURN relay**; its credential is stored
  locally, never logged, and used only to reach the relay **you** chose. A
  guest's feed lands in your composition like any local source, so it is part of
  whatever you record or stream — guests take part knowingly, and the host can
  mute, remove, or ban any guest at any time;
- **the live chat overlay you configure** (optional) — if you add a chat overlay
  and point it at a YouTube, Twitch, or Kick channel, the Software reads that
  channel's **public** live chat the same way a logged-out visitor's browser does
  — **no account, developer key, or sign-in is ever required or requested**. It
  sends only the ordinary requests a viewer's browser makes to read public chat;
  it transmits none of your content or personal data to those platforms;
- **an on-demand download** of one optional, non-bundled component that *you*
  trigger — **ffmpeg** (used for the patent-encumbered "wire" codecs the
  streaming platforms and some exports require), fetched from its third-party
  distributor. The Software contains **no AI/ML features and downloads no
  models**;
- **an update check**. Once per launch, and again whenever you open *Check for
  updates…*, the Software fetches a single small file (`latest.json`) from its
  **GitHub releases page** to see whether a newer version exists. This is the one
  request the Software makes without you asking. It is a plain download: **nothing
  about you, your machine, or your content is sent** — no version, no identifier,
  no usage data. GitHub, like any web server, sees the IP address and user-agent
  of the request itself. Nothing is downloaded or installed unless you answer the
  prompt.

Apart from that update check, these transfers move data **to** your machine and are
initiated by your action, and **no personal data or content is sent** as part of
them beyond the standard network request needed to fetch the file. The streaming
services and third-party distributors have their own privacy practices.

## Stream keys and service credentials
Your stream keys are stored **locally** on your device, shown masked in the
interface, and used **only** to connect to the streaming service you are
broadcasting to. They are never transmitted to the Licensor or to anyone other
than that service.

## Microphone, system audio, webcam, and screen
When you enable screen/window/game capture, audio capture, or a webcam for a
recording or stream, that data is used **only** to produce your recording and/or
the broadcast you chose to send. It is **never** transmitted to the Licensor, and
you control whether each source is captured.

## The remote-control API
The optional WebSocket remote-control API is **off by default**. When you enable
it, it binds to your own machine (loopback by default; LAN only if you explicitly
opt in) and is password-protected. It does not send anything to the Licensor.

## Children
Freally Capture is a general-purpose tool, is not directed at children, and
collects no information from anyone.

## Changes
Any change to this policy will be reflected in this document, both in the
application's About panel and in the project repository.

© Mike Weaver — All Rights Reserved.
