# Remote Guests (P2P / WebRTC) — Design Doc

Status: **Design / not yet built.** This is the phase spec for opt-in remote
collaboration — inviting another person (host + up to three guests) into a
Freally Capture session from another city or country, so their camera and
screen land in the corner slots of the local layout and get composited,
recorded, and streamed like any other source.

This document is the source of truth for that work. No engine code lands until
the token/security contract here is settled.

---

## 1. Why this is a deliberate decision (charter impact)

Freally Capture's founding charter is **local-first**: no accounts, no
telemetry, no cloud; the only outbound calls are the user's own stream
targets, the optional hash-verified FFmpeg download, and the update check.

Remote Guests is the app's **first networked collaboration surface**. It is
therefore:

- **Opt-in and off by default.** The core app stays 100% local for everyone
  who never invites a remote guest. Nothing about this feature runs — or
  reaches the network — until the user explicitly starts a remote session.
- **Clearly labeled.** When a session touches the network (signaling broker,
  a guest's media), the UI says so plainly.
- **Media-P2P.** The heavy traffic (video/audio) flows **peer-to-peer**,
  directly between host and guest — it does **not** pass through any server we
  run. Only tiny signaling metadata touches a broker.

This is a **charter amendment**, not a violation — but it must be made
consciously, documented, and security-reviewed. The amendment reads: *"…plus,
only while an opt-in remote session is active, a signaling broker (default: a
free public one; self-hostable) and optionally a user-provided TURN relay."*

---

## 2. Roles & topology

- **Host** = the Freally Capture **desktop app** (the studio). It captures,
  composites (wgpu), records (.frec / FFmpeg), and streams. This cannot be a
  web app — browsers can't do OS capture, GPU compositing, hardware encoding
  to files, or FFmpeg. The studio is desktop by nature.
- **Guest** = a lightweight **web join page** that runs in **any browser,
  including mobile.** A guest with no computer opens the invite on their phone,
  grants camera/mic, and becomes a remote source. No install. (This is the
  standard StreamYard/Riverside pattern.)
- **Transport** = **WebRTC**, brokered by **PeerJS** (the same stack the
  `vylos-working` prototype already proved). Media is direct P2P; PeerJS only
  brokers the handshake.

```
  Host (desktop app, PeerJS in webview)  <== WebRTC media (P2P) ==>  Guest (web page, any browser/mobile)
             |                                                              |
             +------------------ signaling (few KB) --------------------+---+
                                          |
                              PeerJS broker (free cloud, or self-hosted)
```

---

## 3. The hard part: the webview ↔ compositor bridge

A chat app shows the remote video in a `<video>` tag and is done. A **studio**
must pull that remote MediaStream **into the production pipeline** — it has to
become a **compositor source**: a GPU texture blended into a corner slot,
recorded into the .frec/mp4, and re-streamed to YouTube/Twitch.

Freally Capture is a Tauri app, so PeerJS/WebRTC runs in its webview exactly
like the browser prototype. The engineering is bridging that webview
MediaStream into the Rust compositor. Two candidate approaches:

- **A — Frame-grab bridge (webview → Rust).** In the webview, pull frames off
  the remote `<video>` (`requestVideoFrameCallback` → draw to a canvas →
  `ImageBitmap`/raw bytes) and hand them to Rust over the Tauri IPC / a shared
  buffer; feed them into the compositor as a new "remote" source kind. Audio
  taps into the mixer the same way. **Pros:** reuses the browser WebRTC/PeerJS
  we already understand; no heavy Rust deps (friendly to cargo-deny). **Cons:**
  a per-frame copy webview→Rust→GPU (latency + CPU); needs an efficient
  transfer path.
- **B — Native WebRTC in Rust (`webrtc-rs`).** Terminate WebRTC natively so the
  remote track lands directly as a source. **Pros:** cleaner compositing path,
  no webview round-trip. **Cons:** large new native dependency tree (cargo-deny
  / audit weight), and we re-implement the PeerJS signaling logic natively.

**Recommendation:** start with **A** for the spike (fastest to prove, reuses
vylos know-how, no dep weight), and only consider B if the frame-grab latency
is unacceptable.

The corner-layout engine (already built — `Collection::apply_layout`,
`pending_slot`, `fit_into_slot`) is the composition half; a remote feed drops
into a corner exactly like a local camera. That work is done and independent.

---

## 4. Invite flow

The **link is the invite.** (A QR code is just a link in a bitmap; for two
laptops it adds friction with no benefit — you can't conveniently scan a code
off your own screen. QR is reserved for the *phone-guest* case below.)

1. **Host mints an invite** — the signaling layer issues a **token** with a
   **host-chosen expiry** (e.g. "expires in 30 min if unused").
2. **Host shares the link** however they like (Discord/email/text).
3. **Guest opens the link**, which resolves one of two ways:
   - **Web join page** (default) — works in any browser, desktop or mobile.
   - **Deep link** — `freally://join?token=…` launches an *installed*
     Freally Capture straight to the join screen, falling back to the web page
     if the app isn't present. (Custom URL scheme; see §8.)
4. **Token validated** server-side (signaling); expired/spent → honest
   "invite expired, ask the host for a new one."
5. **QR code (optional, phone guests only):** the app can render the same
   invite link as a QR so a guest joins from their **phone** — scan → mobile
   web join page → phone becomes a corner cam.

---

## 5. Connection strategy (the layered plan)

The relay need is determined by **NAT/firewall type, not WiFi vs cellular.**
WiFi is behind NAT too; "WiFi-only" is both unenforceable (the browser can't
reliably detect link type) and wrong (it would block cellular connections that
would work, and wouldn't guarantee a WiFi pair connects). So:

1. **Always try direct P2P** via free public **STUN** (Google's). Most
   connections (~80–90%, incl. typical home-to-home) succeed here — **$0, over
   the users' own bandwidth.**
2. **If direct fails** (restrictive/symmetric NAT, CGNAT — common between two
   cellular phones): fail with an **honest message**, and *offer* the opt-in
   TURN setup (§6). No silent hang, no false "WiFi-only" rule.
3. **Surface connection requirements + live quality** (§7).

This unifies the two ideas explored: the free STUN baseline *is* "we don't have
to pay for or run anything" for the majority; opt-in TURN is the escape hatch
for the minority the direct path can't reach.

---

## 6. TURN / reliability — opt-in, user-provided

TURN relays media when direct P2P can't connect, so a TURN server's bandwidth
is a real cost that **cannot** be offloaded onto the users' cellular plans (the
relay is a third machine forwarding the stream). Design so **the author never
pays**:

- **Never required.** The free direct path is always the default.
- **User brings their own relay.** An advanced, opt-in setting where the user
  (realistically the **host** — one relay both peers point at) runs their own
  **coturn** and pastes its address + credentials into the app.
- **Guided setup message box** shown *when a direct connection fails*, stating:
  - **What it's for** — "Your networks couldn't connect directly (common
    between two mobile connections). A relay server fixes this. It's optional
    and you run your own — Freally Capture never charges you or routes your
    video through us."
  - **The options + links:**
    - Oracle Cloud **Always Free** — a genuinely free-forever VM with generous
      egress: <https://www.oracle.com/cloud/free/>
    - Hetzner — ~€4/mo VPS with large included traffic:
      <https://www.hetzner.com/cloud>
  - **Numbered steps** — create the account (⚠️ even the free tiers require a
    **credit card at signup** for identity verification; the free tier does not
    charge), spin up a small VM, install coturn, open the UDP/TCP ports, copy
    the URL + username + credential, paste them into Settings → Remote Guests.
  - **A "Skip — try direct only" button.** Opt-in, never forced.

Free TURN tiers (e.g. Metered Open Relay) can be offered as a zero-setup
starter option, with their GB limits stated honestly.

### 6.1 Verifying a TURN server actually relays

A user-provided TURN server is testable **standalone**, before any app work — a
good early de-risk of the whole Oracle-free-tier path. The test needs the
**TURN URL + username + credential** (not the VM's SSH key — that's only for
installing coturn). Three levels:

- **Canonical (manual, ~30s):** the WebRTC **Trickle ICE** page
  (<https://webrtc.github.io/samples/src/content/peerconnection/trickle-ice/>) —
  paste the URL + username + credential, click "Gather candidates"; a candidate
  of type **`relay`** means the server works. No `relay` candidate → ports,
  credentials, or firewall are misconfigured.
- **Automated:** a headless script (`RTCPeerConnection` with
  `iceTransportPolicy: 'relay'` + the TURN config) that reports whether a relay
  candidate is allocated — runnable in a dev session against real credentials.
- **End-to-end:** once the spike exists, force a host↔guest session through the
  relay (`iceTransportPolicy: 'relay'`) to prove it carries real media.

Treat TURN credentials as **secrets** (never logged/committed); prefer coturn's
time-limited credentials (`use-auth-secret`) over static ones. Oracle's classic
gotcha: the ports must be opened in **both** the Oracle security list/NSG **and**
the VM's own firewall (iptables/ufw).

---

## 7. Connection requirements & live quality

- **Minimum bandwidth guidance, shown up front:** ~1.5 Mbps up/down for 720p,
  ~3 Mbps for 1080p, **per remote person.**
- WebRTC **auto-adapts** bitrate (it degrades quality on weak links rather than
  dropping the call), so also show a **live good / fair / poor indicator** from
  WebRTC's own `getStats()`, plus whether the connection is **direct or
  relayed.**

---

## 8. Custom URL scheme (`freally://`) — the first buildable brick

The deep link that opens an installed app to the join screen. **Safe to build
early** as isolated plumbing, but its **token contract must be pinned here
first** (this doc) before it's wired to anything.

- Register `freally://` via the Tauri deep-link plugin (Windows registry /
  macOS `CFBundleURLTypes` / Linux `.desktop` MimeType), single-instance so a
  second click focuses the running app.
- Parse `freally://join?token=<opaque>`; **validate** the token shape before
  use (treat the whole URL as untrusted input).
- Route to a **join screen** (a stub until signaling exists).
- Testable end-to-end independent of the network: click link → app focuses →
  token received + validated.

New dep (`tauri-plugin-deep-link`) must pass cargo-deny / cargo-audit; vendor
any JS (no CDN) to satisfy CSP.

---

## 9. Cost model (summary)

| Piece            | Who pays | Cost |
|------------------|----------|------|
| Desktop app dist | —        | Free (GitHub Releases) |
| Web join page    | —        | Free (GitHub Pages, static) |
| Signaling broker | author   | **$0** (PeerJS free cloud) or ~$5/mo self-host |
| STUN             | —        | Free (public) |
| Media (direct)   | the two users, own bandwidth | **$0 to author** |
| TURN (fallback)  | **the user who opts in** | $0 on free tiers / Oracle free; bandwidth-bound at scale |

**Author's standing cost to ship this open-source: $0.** Free defaults +
configurable infra; users bring their own relay only if they want the hard
cases.

---

## 10. Security review (required before ship)

This is a networked feature — it must not ship without a focused review:

- **Token auth + expiry** — opaque, unguessable, single-use, host-chosen TTL,
  validated server-side.
- **All external input untrusted** — the `freally://` URL, the join page's
  messages, any data from the guest. Validate/parse defensively (the existing
  `#![forbid(unsafe_code)]` + allocation-capped parsing philosophy applies).
- **Media encryption** — WebRTC is DTLS-SRTP encrypted in transit by default;
  even a TURN relay forwards ciphertext. Document what the broker *can* see
  (metadata, not media).
- **Consent** — explicit camera/mic permission on the guest; explicit "start
  remote session" on the host. Nothing auto-connects.
- **No capability creep** — the join page/guest can never reach the host's
  files, other sources, or the network beyond the session.
- **Supply chain** — vendored PeerJS (no CDN, CSP-safe); every new Rust dep
  through cargo-deny / cargo-audit.
- Runs the builtin security-review pass on the branch before merge.

---

## 11. Deployment

- **Desktop app** → GitHub Releases (as today).
- **Web join page** → GitHub Pages (static — it's just HTML/JS running PeerJS
  in the guest's browser).
- **Signaling** → PeerJS free cloud by default; document self-hosting a
  PeerServer (tiny Node app on a free tier / cheap VPS) for reliability.

---

## 12. Phasing

1. **Transport spike (#14).** Two app instances connect P2P via PeerJS in the
   webview; prove a remote MediaStream reaches the compositor as a corner
   source (approach A). Windows first. Proof-of-concept, not production.
2. **URL scheme brick (§8).** Register `freally://`, parse + validate + route
   to a stub join screen. Tested end-to-end offline.
3. **Signaling + invites.** Token mint/validate/expire; the link + deep-link +
   web-join resolution; QR for phone guests.
4. **The bridge, productionized.** Efficient webview↔compositor frame path;
   audio into the mixer; a proper "remote" source kind; honest per-guest
   status.
5. **TURN opt-in + guided setup** (§6) and **connection-quality UI** (§7).
6. **Bidirectional display handoff** — route which participant's shared screen
   is the centered source; hand it back and forth.
7. **Security review + hardening**, then charter-amendment docs + site copy.

---

## Open questions

- Frame-grab (A) vs native `webrtc-rs` (B) — settle after the spike measures
  real latency.
- Whether the web join page lives in this repo (published via Pages) or a
  sibling repo.
- Exactly where the `freally://` token is minted (self-hosted signaling vs a
  serverless function) for the expiring-link feature.
