# Freally Capture — en
#
# Fluent (.ftl). Keys must match en.ftl exactly; `npm run i18n:lint` fails the
# build on any missing or extra key. English is layered under every locale, so a
# key that slips through still renders in English rather than as a raw id.
#
# Do not reorder: keys are grouped as they appear in the UI.


# --- core ---
studio-mode = Studio Mode
toggle-on = on
toggle-off = off
stats = Stats
core-ok = core OK
hide-stats-dock = Hide the stats dock
show-stats-dock = Show the stats dock


# =============================================================
# --- shell ---
# =============================================================
# shell
# Extracted from ui/src/App.tsx, ui/src/panels/PreviewPanel.tsx,
# ui/src/panels/RemoteSessionBar.tsx.
# Reuses existing en.ftl keys (do NOT redefine here): studio-mode, toggle-on,
# toggle-off, stats, core-ok, hide-stats-dock, show-stats-dock.

# --- App shell (App.tsx) ---
app-save-error = Couldn't save settings — the change won't survive a restart.
studio-mode-leave = Leave Studio Mode
studio-mode-enter-title = Studio Mode — edit a preview scene, commit it to the program with a transition
vertical-canvas-title = The second (vertical 9:16) output canvas — recordable and streamable independently
app-version = v{ $version }
core-error = core ERROR
core-unreachable = core unreachable (browser mode)
connecting-to-core = connecting to core…
filters-source-fallback = Source

# --- Program preview (PreviewPanel.tsx) ---
preview-program-label = Program preview
preview-program-output = Program output
preview-canvas-editor = Canvas editor
preview-px-to-edge-label = Pixels to the frame edges
preview-px-to-edge = px to edge L { $left } · T { $top } · R { $right } · B { $bottom }
preview-program-heading = Program
preview-no-gpu = No usable GPU adapter was found — the compositor can't run on this machine.
preview-starting-compositor = Starting the compositor…
preview-empty-scene = This scene is empty — add a source in Sources, then drag, scale, and rotate it right here on the canvas.
preview-fps = { $fps } fps
preview-dropped = { $dropped } dropped

# --- Remote session bar (RemoteSessionBar.tsx) ---
remote-invite-received = Invite link received
remote-join-with-webcam = Join with webcam
remote-dismiss = Dismiss
remote-hosting-guest = Hosting a remote guest
remote-you-are-guest = You're a remote guest
remote-share-view-title = Share your screen to the guest's app (they see your view live)
remote-stop-sharing-view = Stop sharing view
remote-share-my-view = Share my view
remote-allow-center-title = Allow the guest to switch which view holds the center (you stay in control and can switch back any time)
remote-guest-switching = Guest switching:
remote-stop-screen = Stop screen
remote-share-screen = Share screen
remote-share-screen-title-guest = Share your screen with the host (it becomes a source they can center)
remote-center-request-label = Center view request
remote-center = Center
remote-center-cam-title = Ask the host to center your camera
remote-center-my-cam = My cam
remote-center-screen-title = Ask the host to center your shared screen
remote-center-my-screen = My screen
remote-center-host-title = Give the center back to the host's view
remote-center-host-view = Host view
remote-end-session = End session
remote-leave = Leave
remote-host-view-heading = Host view
remote-host-shared-view-label = The host's shared view
remote-guest-position-label = Guest position
remote-guest-label = Guest
remote-put-guest = Put the guest { $position }
remote-remove-title = Remove the guest — they can rejoin with the same link
remote-remove = Remove
remote-ban-title = Ban the guest — blocks them and invalidates the invite link
remote-ban = Ban
remote-guest-self-muted = guest self-muted
remote-unmute-guest = Unmute guest
remote-mute-guest = Mute guest
remote-muted-by-host = Muted by host
remote-unmute-mic = Unmute mic
remote-mute-mic = Mute mic
remote-waiting-for-host = waiting for the host


# =============================================================
# --- sources-rail ---
# =============================================================
# sources-rail

# Generic fallbacks used where a source/video name is missing
sources-fallback-name = source
sources-fallback-video = video
sources-fallback-error = error
sources-kind-unknown = ?
sources-missing-source = (missing source)

# Kind badges (small uppercase tag on each source row)
sources-badge-display = Display
sources-badge-window = Window
sources-badge-portal = Portal
sources-badge-camera = Camera
sources-badge-image = Image
sources-badge-media = Media
sources-badge-guest = Guest
sources-badge-color = Color
sources-badge-text = Text
sources-badge-scene = Scene
sources-badge-slides = Slides
sources-badge-chat = Chat
sources-badge-audio-in = Audio In
sources-badge-audio-out = Audio Out
sources-badge-app-audio = App Audio
sources-badge-test-bars = Bars
sources-badge-test-grid = Grid
sources-badge-test-sweep = Sweep
sources-badge-test-tone = Tone
sources-badge-test-sync = Sync
sources-badge-timer = Timer
sources-badge-stats = Stats
sources-badge-visualizer = Visualizer
sources-badge-splits = Splits
sources-badge-input = Input
sources-badge-playlist = Playlist
sources-badge-replay = Replay
sources-badge-lan-ingest = LAN
sources-badge-title = Title
sources-badge-link = Link

# Add-source menu items
sources-add-display = Display Capture
sources-add-window = Window Capture
sources-add-game = Game Capture (read first)
sources-add-webcam = Video Capture Device
sources-add-image = Image
sources-add-media = Media (video/image file)
sources-add-playlist = Media Playlist (gapless)
sources-add-replay = Instant Replay
sources-add-remote-guest = Remote Guest (P2P spike)
sources-add-lan-ingest = LAN Ingest (SRT/RTMP listener)
sources-add-freally-link = Freally Link (another instance)
sources-add-color = Color
sources-add-text = Text
sources-add-title = Title / Scoreboard
sources-add-timer = Timer / Clock
sources-add-system-stats = Performance Stats (HUD)
sources-add-visualizer = Audio Visualizer
sources-add-split-timer = Speedrun Split Timer
sources-add-input-overlay = Input Overlay (keys/pad)
sources-add-nested-scene = Nested Scene
sources-add-slideshow = Image Slideshow
sources-add-chat-overlay = Live Chat Overlay
sources-add-test-signal = Test Signal
sources-add-audio-input = Audio Input Capture
sources-add-audio-output = Audio Output Capture
sources-add-app-audio = Application Audio (Windows)
sources-add-existing = Existing source…

# Panel header + toolbar buttons
sources-panel-title = Sources
sources-group-title = Group sources — pick two or more items, then Create group; grouped items move and show/hide together
sources-group-aria = Group sources
sources-arrange = Arrange: screen + corners
sources-add-source = Add a source
sources-browser-source-note = Browser Source ships as its own on-demand component milestone (a ~180 MB Chromium engine — never bundled). Today: capture a real browser window with Window Capture + a chroma/color key, or open chat/alerts as a Dock (Controls → Docks).

# Empty state
sources-empty = No sources in this scene — add a Display Capture, Window, Webcam, Image, Color, or Text with “+”. Drag, scale, and rotate them on the canvas; right side buttons reorder the stack.

# Per-row controls
sources-already-in-group = Already in { $name }
sources-pick-for-new-group = Pick for the new group
sources-pick-item-for-group = Pick { $name } for the new group
sources-hide = Hide
sources-show = Show
sources-hide-item = Hide { $name }
sources-show-item = Show { $name }
sources-unfocus-title = Unfocus — restore the layout
sources-focus-title = Focus — fill the canvas (Highlight Speaker)
sources-unfocus-item = Unfocus { $name }
sources-focus-item = Focus { $name }
sources-center-title = Center — make this the shared center view (cams move to the rail)
sources-center-item = Center { $name }
sources-rename-item = Rename { $name }
sources-in-group = In group { $name }

# Row status + retry
sources-retry-error = Retry — { $message }
sources-retry-item = Retry { $name }
sources-status-error = status: error
sources-open-privacy-title = Open the macOS privacy settings for this permission
sources-open-privacy-item = Open privacy settings for { $name }
sources-privacy-settings-button = settings
sources-status-starting = starting…
sources-status-live = live
sources-status-aria = status: { $state }

# Media row pause/resume
sources-media-resume-title = Resume the video (live on the stream)
sources-media-pause-title = Pause the video — hold the frame + go silent, live on the stream
sources-media-resume-item = Resume { $name }
sources-media-pause-item = Pause { $name }

# Hover controls
sources-unlock = Unlock
sources-lock = Lock
sources-unlock-item = Unlock { $name }
sources-lock-item = Lock { $name }
sources-raise-title = Raise in the stack
sources-raise-item = Raise { $name }
sources-lower-title = Lower in the stack
sources-lower-item = Lower { $name }
sources-filters-title = Filters & blend
sources-filters-item = Filters for { $name }
sources-properties-title = Properties
sources-properties-item = Properties of { $name }
sources-remove-title = Remove from this scene
sources-remove-item = Remove { $name }

# Grouping footer
sources-create-group = Create group ({ $count })
sources-cancel = Cancel

# Groups list
sources-groups-aria = Source groups
sources-hide-group = Hide the group
sources-show-group = Show the group
sources-item-count = · { $count } items
sources-ungroup-title = Ungroup — the items stay where they are
sources-ungroup-item = Ungroup { $name }

# Live Chat Overlay picker
sources-chat-title = Add a Live Chat Overlay
sources-chat-youtube-label = YouTube — channel, watch, or live_chat URL (no key, no sign-in)
sources-chat-youtube-placeholder = https://www.youtube.com/@yourchannel  ·  or a watch?v= URL
sources-chat-twitch-label = Twitch — channel name (read anonymously, no account)
sources-chat-twitch-placeholder = yourchannel
sources-chat-kick-label = Kick — channel slug (public endpoint, best-effort)
sources-chat-kick-placeholder = yourchannel
sources-chat-note = Messages appear with a running h:mm:ss AM/PM timestamp on a transparent background (default top-right; drag it anywhere). A chat flood only ages old lines out — it can never stall the stream or the recording. Facebook chat needs your own Graph token and is not implemented yet — it is never required and never gates the platforms above.
sources-chat-add = Add chat overlay
sources-chat-default-name = Live Chat

# Image Slideshow picker
sources-slideshow-title = Add an Image Slideshow
sources-slideshow-empty = No images yet — Browse adds them in order.
sources-slideshow-remove-slide = Remove slide { $number }
sources-slideshow-browse = Browse images…
sources-slideshow-per-slide-label = Per-slide (ms)
sources-slideshow-crossfade-label = Crossfade (ms, 0 = cut)
sources-slideshow-loop-label = Loop (off = hold the last slide)
sources-slideshow-shuffle-label = Shuffle each cycle
sources-slideshow-note = The crossfade blends equal-sized images; different sizes hard-cut at the boundary (no silent rescale).
sources-slideshow-add = Add slideshow ({ $count })

# Nested Scene picker
sources-nested-title = Add a Nested Scene
sources-nested-empty = No other scene to nest — add a second scene first.
sources-nested-scene-name = Scene: { $name }
sources-nested-note = The nested scene renders live at the program canvas size and follows its own edits; transforms, filters, and blend apply to it like any source. Its audio sources join the mix while a scene showing it is the program.

# Display / Window capture picker
sources-capture-display-title = Add a Display Capture
sources-capture-window-title = Add a Window Capture
sources-capture-looking = Looking for sources…
sources-capture-none-displays = Nothing to capture here — no displays were found.
sources-capture-none-windows = Nothing to capture here — no windows were found.
sources-capture-portal-note = On Wayland, the system dialog picks the screen or window — apps can't capture globally there, so that's the honest (and only) path.
sources-capture-window-note = Previews update live. A minimized window shows its last frame (or none) until you restore it.
sources-thumb-no-preview = no preview
sources-thumb-loading = loading…

# Video Capture Device picker
sources-webcam-title = Add a Video Capture Device
sources-webcam-looking = Looking for cameras…
sources-webcam-none = No cameras or capture cards were found.
sources-webcam-format-label = Format
sources-webcam-format-auto-loading = Auto (loading formats…)
sources-webcam-format-auto = Auto (highest resolution)
sources-webcam-card-presets-label = Card presets:
sources-webcam-preset-title = Select the { $label } mode this card advertises
sources-webcam-add = Add camera

# Audio Input / Output capture picker
sources-audio-output-title = Add an Audio Output Capture
sources-audio-input-title = Add an Audio Input Capture
sources-audio-default-output = Default output (what you hear)
sources-audio-default-input = Default input
sources-audio-looking = Looking for audio devices…
sources-audio-none-output = No desktop-audio capture device was found here.
sources-audio-none-input = No microphones or line-ins were found.
sources-audio-input-note = Mixer strips get a VU meter, fader, mute, monitoring, filters (denoise, gate, compressor…), and track assignment. Everything stays on this machine.

# Application Audio picker
sources-appaudio-title = Add Application Audio
sources-appaudio-looking = Looking for apps making sound…
sources-appaudio-none = No apps are making sound right now — start playback in the app, then refresh.
sources-appaudio-refresh = ⟳ Refresh
sources-appaudio-note = Captures exactly that app's audio — its own VU, fader, mute, filters, and track.

# Game Capture picker
sources-game-title = Game Capture
sources-game-checking = Checking…
sources-game-use-portal = Use Screen Capture (Portal)
sources-game-use-window = Use Window Capture instead

# Image picker
sources-image-title = Add an Image
sources-image-file-label = Image file (PNG, JPEG, BMP, GIF, WebP…)
sources-image-add = Add image

# Path field
sources-browse = Browse…

# Media picker
sources-media-title = Add Media
sources-media-file-label = Media file (mp4, mkv, webm, mov, .frec, or an image)
sources-media-loop-label = Loop (restart from the top at the end)
sources-media-note = .frec plays through the owned freally-video codec — nothing to download. The wire formats (mp4/mkv/webm/…) decode through the on-demand FFmpeg component; its audio lands in the mixer as its own strip.
sources-media-add = Add media

# Invite expiry options
sources-ttl-15min = 15 min
sources-ttl-30min = 30 min
sources-ttl-1hour = 1 hour
sources-ttl-1day = 1 day

# Remote Guest form
sources-remote-copy-failed = couldn't copy — select the link and copy manually
sources-remote-join-failed = join failed: { $error }
sources-remote-title = Remote Guest (P2P spike)
sources-remote-host-heading = Host — invite a guest
sources-remote-start-hosting = Start hosting
sources-remote-expires-label = Expires
sources-remote-invite-expiry-aria = Invite expiry
sources-remote-invite-link-aria = Invite link
sources-remote-copied = Copied ✓
sources-remote-copy = Copy
sources-remote-share-note = Share this link (Discord / text / email). It carries your session and expires as set. The guest opens it and joins with their webcam.
sources-remote-qr-note = Scan on a phone to join straight from the browser — camera + mic, no install. The copyable freally:// link above opens in Freally Capture on a machine that has it.
sources-remote-guest-heading = Guest — join with an invite
sources-remote-paste-placeholder = paste the invite link
sources-remote-invite-input-aria = Invite link or session id
sources-remote-join = Join with webcam
sources-remote-session-note = The live session controls (mute, end) stay on the bar at the top of the main window — you can close this dialog.
sources-remote-stop-session = Stop session

# Invite QR
sources-invite-qr-aria = Invite link QR code

# Remote device pickers
sources-devices-output-unavailable = output routing unavailable — playing on the default device
sources-devices-mic-test-failed = mic test failed: { $error }
sources-devices-heading = Session audio devices
sources-devices-microphone-label = Microphone
sources-devices-microphone-aria = Session microphone
sources-devices-system-default = System default
sources-devices-output-label = Output
sources-devices-output-aria = Session audio output
sources-devices-stop-test = Stop test
sources-devices-test = Test — hear yourself
sources-devices-testing-note = talk into the mic — you're hearing the selected devices live
sources-devices-idle-note = loops your mic to the output (headphones avoid feedback)

# TURN relay section
sources-turn-save-failed = couldn't save: { $error }
sources-turn-summary = Network — optional TURN relay (advanced)
sources-turn-note-1 = Sessions connect directly (P2P) — free, no relay needed. If BOTH sides sit behind strict NATs the direct path can fail; a TURN relay you run yourself carries the media then. Skipping this is fine — most connections work direct-only.
sources-turn-note-2 = Free option: Oracle Cloud "Always Free" runs coturn at no cost (note: Oracle asks for a credit card at signup, but the Always-Free shape stays free). Steps: 1) create the free VM, 2) install coturn, 3) open UDP 3478, 4) set a user/password, 5) enter turn:your-vm-ip:3478 + the credentials here. Your credential stays in your local settings file and is never logged.
sources-turn-url-label = TURN URL
sources-turn-url-placeholder = turn:host:3478 (empty = direct only)
sources-turn-url-aria = TURN URL
sources-turn-username-label = Username
sources-turn-username-aria = TURN username
sources-turn-credential-label = Credential
sources-turn-credential-aria = TURN credential
sources-turn-note-3 = The relay engages once all three fields are set (a TURN server requires the credentials) and applies to the next session you start or join. Verify it with a relay-only test call between your own two machines.
sources-turn-settings-unavailable = settings unavailable (browser mode)

# Color picker
sources-color-title = Add a Color
sources-color-label = Color
sources-color-width-label = Width
sources-color-height-label = Height
sources-color-add = Add color
sources-testsignal-title = Add a Test Signal
sources-testsignal-pattern-label = Pattern
sources-testsignal-bars = SMPTE color bars
sources-testsignal-grid = Calibration grid
sources-testsignal-sweep = Motion sweep
sources-testsignal-tone = 1 kHz tone (−20 dBFS)
sources-testsignal-flash-beep = A/V sync flash + beep
sources-testsignal-note = Verify scenes, encoders, projectors, and stream targets with no camera plugged in. The flash + beep pattern drives the A/V sync workbench.
sources-testsignal-add = Add test signal
sources-timer-title = Add a Timer
sources-timer-mode-label = Mode
sources-timer-wall-clock = Wall clock
sources-timer-countdown = Countdown
sources-timer-stopwatch = Stopwatch
sources-timer-since-live = Time since live
sources-timer-since-recording = Time since recording
sources-timer-note = Duration, format, styling and end-of-countdown actions live in the source's Properties.
sources-timer-add = Add timer

# Instant replay picker (CAP-N10)
sources-replay-title = Add an Instant Replay
sources-replay-seconds-label = Roll length (seconds)
sources-replay-speed-label = Speed
sources-replay-speed-full = 100% (with audio)
sources-replay-speed-half = 50% slow-mo (silent)
sources-replay-speed-quarter = 25% slow-mo (silent)
sources-replay-note = Idles transparent until you roll. Arm the replay buffer (Controls dock) and bind the Roll hotkey — a roll snapshots the buffer's last moments and plays them into the program, then clears back to transparency.
sources-replay-add = Add instant replay

# Freally Link picker (CAP-N12)
sources-link-title = Add a Freally Link
sources-link-about = Receives another Freally Capture's program — video and master audio — over your own network. Enable "Freally Link output" on the sending instance first. v1 streams motion-JPEG over TCP: great on wired LAN or good Wi-Fi, honest about bandwidth on weak links.
sources-link-scan = Scan the LAN
sources-link-scanning = Scanning…
sources-link-none = No Freally Link outputs found. Enable "Freally Link output" on the other instance (Controls → LAN panel), or type its address below.
sources-link-host = Address
sources-link-port = Port
sources-link-key = Pairing key
sources-link-key-hint = The key from the sender's "Freally Link output" settings — without it the sender refuses to serve a single frame.
sources-link-add = Add link
properties-link-note = While unconnected the source shows a "connecting" face and retries on its own with backoff — it never freezes on a stale frame. One receiver per sender; a busy sender is retried politely.

# Media playlist picker (CAP-N17)
sources-playlist-title = Add a Media Playlist
sources-playlist-files-label = Files (one per line, played top to bottom)
sources-playlist-browse = Browse…
sources-playlist-loop = Loop
sources-playlist-shuffle = Shuffle (one draw per start; a looping shuffle repeats its order)
sources-playlist-hold-last = Hold the last frame at the end
sources-playlist-note = Plays the whole trimmed list gaplessly through the labeled ffmpeg component (wire formats only — .frec and stills play through Media/Slideshow). Items are all-video or all-audio, never mixed. Per-item trims, cue points, and the "now playing" variable live in the source's Properties.
sources-playlist-add = Add playlist

# Split timer picker (CAP-N18)
sources-splits-title = Add a Split Timer
sources-splits-file-label = LiveSplit .lss file
sources-splits-comparison-label = Compare against
sources-splits-comparison-pb = Personal best
sources-splits-comparison-best = Best segments
sources-splits-comparison-average = Average
sources-splits-note = Imports the file read-only — nothing is ever written back. Bind the global Split / Undo / Skip / Reset keys in Settings → Hotkeys. Process-memory auto-splitters are deliberately not supported.
sources-splits-add = Add split timer

# LAN ingest picker (CAP-N11)
sources-lan-title = Add a LAN Ingest listener
sources-lan-protocol-label = Protocol
sources-lan-protocol-srt = SRT (encryptable — recommended)
sources-lan-protocol-rtmp = RTMP (no authentication)
sources-lan-port-label = Port (1024–65535)
sources-lan-passphrase-label = Passphrase (empty = open)
sources-lan-passphrase-hint = SRT passphrases are 10–79 characters; the sender must use the same one.
sources-lan-open-warning = No passphrase: anyone on this network can feed this source, unencrypted. Set one unless the network is yours alone.
sources-lan-rtmp-warning = RTMP has no authentication — anyone on this network can send to this port. Prefer SRT with a passphrase.
sources-lan-url-label = Point the sender's app at
sources-lan-qr-aria = Ingest URL QR code
sources-lan-note = LAN only: listens on this machine's local address, only while the source exists, and never touches the internet — nothing leaves the machine unless a sender on your network sends first. Decoding rides the labeled ffmpeg component. The canvas shows this URL until a sender connects.
sources-lan-add = Start listening

# Title designer picker (CAP-N16)
sources-title-title = Add a Title
sources-title-template-label = Start from
sources-title-template-lower-third = Lower third (bar + name + subtitle)
sources-title-template-scoreboard = Scoreboard (plate + 4 cells)
sources-title-template-blank = Blank canvas
sources-title-width-label = Canvas width
sources-title-height-label = Canvas height
sources-title-template-name = Name
sources-title-template-subtitle = Title
sources-title-template-home = HOME
sources-title-template-away = AWAY
sources-title-note = Layered text / image / box titles with an animate-in/out pass, composed locally — no browser source. Layers, bindings to files and {"{{"}variables{"}}"}, and the live Fire / edit controls live in the source's Properties.
sources-title-add = Add title

# Input overlay picker (CAP-N13)
sources-input-title = Add an Input Overlay
sources-input-layout-label = Layout
sources-input-layout-wasd = WASD + mouse
sources-input-layout-keyboard = Compact keyboard + mouse
sources-input-layout-gamepad = Gamepad (dual stick)
sources-input-layout-fightstick = Fight stick
sources-input-color-label = Keys
sources-input-accent-label = Pressed
sources-input-privacy-note = Privacy: input is read only while this source is live in a scene, and only the layout's fixed keys are polled — a point-in-time "is it down?" peek, never a hook. Nothing is logged, stored, or sent anywhere, and typed text is never captured.
sources-input-os-note = Keyboard and mouse state is read on Windows only today — other systems draw the caps unpressed (said honestly, never faked). Gamepads work everywhere via the gilrs library; the first connected controller is drawn, and with none found the layout draws unpressed.
sources-input-add = Add input overlay

# Audio visualizer picker (CAP-N15)
sources-visualizer-title = Add an Audio Visualizer
sources-visualizer-style-label = Style
sources-visualizer-style-bars = Spectrum bars
sources-visualizer-style-scope = Oscilloscope
sources-visualizer-style-vu = VU meters
sources-visualizer-target-label = Listens to
sources-visualizer-target-master = Master mix
sources-visualizer-target-track = Track { $n }
sources-visualizer-note = Draws the signal that actually mixes (post-fader) — a muted source visualizes flat, exactly like it sounds. Size, color, bar count, and fall rate live in the source's Properties.
sources-visualizer-add = Add visualizer

# System-stats HUD picker (CAP-N14)
sources-stats-title = Add a Performance HUD
sources-stats-note = Puts the studio's real measured numbers on the program for your viewers — fps, CPU, memory, render time, dropped frames, and live bitrate. Which lines show, plus size and color, live in the source's Properties. GPU usage is not shown because it is not measured.
sources-stats-add = Add stats HUD

# Text picker
sources-text-title = Add Text
sources-text-label = Text
sources-text-default = Text
sources-text-color-label = Color
sources-text-color-aria = Text color
sources-text-size-label = Size (px)
sources-text-note = Font family, alignment, wrapping, and RTL live in the source's Properties. The bundled Noto Sans (incl. Arabic/Hebrew) is the default — identical on every machine.
sources-text-add = Add text

# Existing source picker
sources-existing-title = Add an existing source
sources-existing-empty = No sources exist yet — add one to any scene first. Existing sources are shared: renaming or reconfiguring one updates every scene that shows it.

# Screen + corners layout
sources-slot-off = Off
sources-slot-center = Center (screen)
sources-slot-top-left = Top-Left
sources-slot-top-right = Top-Right
sources-slot-bottom-left = Bottom-Left
sources-slot-bottom-right = Bottom-Right
sources-layout-title = Arrange: Screen + corners
sources-layout-empty = Add a screen capture and one or more cameras to this scene first, then arrange them here.
sources-layout-note = Put a screen in the center and up to four cameras in the corners — your explainer / podcast layout. Each corner holds a webcam, a captured call window, or a media clip. You can drag any of them on the canvas afterward.
sources-layout-slot-aria = Slot for { $name }
sources-layout-apply = Apply layout


# =============================================================
# --- docks ---
# =============================================================
# docks
# Extracted from ui/src/panels/{ControlsDock,MixerDock,StatsDock,ScenesRail}.tsx
# The Stats panel title reuses the existing `stats` key (not redefined here).

# --- ControlsDock.tsx ---
controls-title = Controls
controls-start-stop-title-stop = Stop and finalize the recording
controls-start-stop-title-start = Record the program feed with the Settings → Output configuration
controls-finalizing = ◌ Finalizing…
controls-stop-recording = ■ Stop Recording
controls-start-recording = ● Start Recording
controls-marker-title = Drop a chapter marker at this moment — it lands in the RECORDING (mkv chapters, or a sidecar file). Platform-side stream markers need platform accounts, which this app never asks for.
controls-marker = ◈ Marker
controls-pause-title-resume = Resume — the file continues as one contiguous timeline
controls-pause-title-pause = Pause — no frames are written; resuming continues the same playable file
controls-resume-recording = ▶ Resume Recording
controls-pause-recording = ⏸ Pause Recording
controls-reactions-label = Reactions (baked into the program)
controls-reactions-title = Float a reaction over the program — recorded AND streamed, so the replay shows the exact moment. Viewers in chat trigger these too (their reaction emoji float automatically); a flood only caps what's on screen.
controls-react = React { $emoji }
controls-virtual-camera-title = The virtual camera needs its own signed driver component per OS (Win11 MFCreateVirtualCamera / Win10 DirectShow / macOS CoreMediaIO extension / Linux v4l2loopback) — it ships as its own milestone. The feed model is ready for it: program, vertical canvas, or a single source, with a paired virtual mic on Windows/Linux (macOS has no virtual-mic API — said honestly).
controls-virtual-camera = ⌁ Start Virtual Camera
controls-files-title = Finished recordings + the remux-to-mp4 action
controls-files = ▤ Files…
controls-output-title = Recording format, encoder, folder, tracks, and splitting
controls-output = ⚙ Output…
controls-stream-title = Go Live target: service, stream key, encoder, bitrate
controls-stream = ⦿ Stream…
controls-codecs-title = The on-demand ffmpeg wire-codec component (clearly labeled, never bundled)
controls-codecs = ⬡ Codecs…
controls-replay-title = Replay buffer length + quality presets
controls-replay = ⟲ Replay…
controls-keys-title = Global hotkeys: record, Go Live, transition, save replay
controls-keys = ⌨ Keys…
controls-scripts-title = Sandboxed Lua scripts: react to go-live/scene/recording events, drive the studio
controls-scripts = ⚡ Scripts…
controls-docks-title = Browser docks: open a chat popout, alerts page, or Companion buttons as a window beside the studio
controls-docks = ⧉ Docks…
controls-remote-title = WebSocket remote API for Stream Deck / Companion controllers (off by default)
controls-remote = ⌁ Remote…
controls-profiles-title = Profiles (settings) + scene collections — switchable snapshots
controls-profiles = ▣ Profiles…
controls-bug-title = Report a bug — anonymous, opt-in (nothing is sent automatically)
controls-bug = 🐞 Report a bug…
controls-updates-title = Check for updates — signed, verified, nothing downloads without a click
controls-updates = ⭳ Check for updates…
controls-saved = Saved: { $path }

# --- MixerDock.tsx ---
mixer-title = Audio Mixer
mixer-monitor-error = monitor: { $error }
mixer-switch-to-horizontal = Switch to horizontal strips
mixer-switch-to-vertical = Switch to vertical strips
mixer-layout-aria-vertical = Mixer layout: vertical — switch to horizontal
mixer-layout-aria-horizontal = Mixer layout: horizontal — switch to vertical
mixer-empty = No audio sources in this scene — add an Audio Input Capture (mic) or Audio Output Capture (desktop audio) with “+” in Sources. Strips get a VU meter, fader, mute, monitoring, filters, and track assignment.
mixer-advanced-title = Audio — { $name }
mixer-loudness-label = Program loudness (LUFS)
mixer-lufs = LUFS
mixer-momentary-title = Momentary loudness (400 ms)
mixer-short-term-title = Short-term loudness (3 s)
mixer-lufs-short = S { $value }
mixer-monitor-label = Monitor
mixer-monitor-device-aria = Monitor output device
mixer-default-output = Default output

# --- StatsDock.tsx (Panel title reuses `stats`) ---
stats-fps = FPS
stats-cpu = CPU
stats-memory = Memory
stats-dropped = Dropped
stats-render = Render
stats-gpu = GPU
stats-gpu-compositing = compositing
stats-gpu-idle = idle
stats-vertical-fps = 9:16 FPS
stats-targets-label = Stream targets
stats-shared-encode = · shared encode
stats-starting = Starting the compositor…

# --- ScenesRail.tsx ---
scenes-title = Scenes
scenes-new-scene-name = Scene
scenes-add = Add a scene
scenes-empty = Connecting to the studio core…
scenes-rename = Rename { $name }
scenes-on-program = On program
scenes-preview = Preview { $name }
scenes-switch-to = Switch to { $name }
scenes-move-up = Move up
scenes-move-up-aria = Move { $name } up
scenes-move-down = Move down
scenes-move-down-aria = Move { $name } down
scenes-last-stays = The last scene stays
scenes-remove = Remove this scene
scenes-remove-aria = Remove { $name }


# =============================================================
# --- components ---
# =============================================================
# components
# Extracted user-visible strings from ui/src/components/*.tsx:
#   ChannelStrip, LiveButton, RecDot, ReplayControls,
#   PropertiesDialog, AudioFiltersDialog, FiltersDialog, PickerShell.
# (Panel.tsx and NumberField.tsx render only caller-supplied props — no literals.)
# Brand names, technical tokens, and Fluent placeables are preserved verbatim.


# --- ChannelStrip.tsx ---
channelstrip-level = Level
channelstrip-monitor-off = Monitor off
channelstrip-monitor-only = Monitor only (not in the mix)
channelstrip-monitor-and-output = Monitor and output
channelstrip-status-error = error
channelstrip-status-live = live
channelstrip-status-waiting-audio = waiting for audio
channelstrip-status = status: { $state }
channelstrip-status-waiting = waiting
channelstrip-mute = Mute
channelstrip-unmute = Unmute
channelstrip-mute-source = Mute { $name }
channelstrip-unmute-source = Unmute { $name }
channelstrip-scene-mix-on = Per-scene mix ON — this strip overrides the global mix for this scene (click to follow the global mix again)
channelstrip-scene-mix-off = Per-scene mix — give this strip its own fader/mute for the current scene
channelstrip-scene-mix-label = Per-scene mix for { $name }
channelstrip-monitor-cycle = { $mode } — click to cycle
channelstrip-monitor-mode = Monitor mode of { $name }: { $mode }
channelstrip-audio-filters-title = Audio filters (denoise, gate, compressor…)
channelstrip-audio-filters-label = Audio filters for { $name }
channelstrip-advanced-title = Sync offset & push-to-talk hotkeys
channelstrip-advanced-label = Advanced audio settings for { $name }
channelstrip-track-assignment = Track assignment
channelstrip-track = Track { $n }
channelstrip-track-assigned = Track { $n } (assigned)
channelstrip-track-label = Track { $n } for { $name }
channelstrip-device-error = device error
channelstrip-audio-device-error = audio device error
channelstrip-volume-label = Volume of { $name } in decibels
channelstrip-ptt-hold = Push-to-talk: hold { $key }
channelstrip-sync-offset = Sync offset (ms, 0–{ $max } — delays this audio)
channelstrip-solo-title = Solo (PFL) — the monitor hears only soloed strips; the program mix is untouched
channelstrip-solo-source = Solo { $name } (PFL)
channelstrip-pan-label = Balance (double-click resets)
channelstrip-pan-aria = Balance of { $name }
channelstrip-mono-label = Downmix to mono
channelstrip-ptt-hotkey = Push-to-talk hotkey (silent unless held)
channelstrip-ptt-placeholder = e.g. Ctrl+Shift+T or F13
channelstrip-ptt-aria = Push-to-talk hotkey
channelstrip-ptm-hotkey = Push-to-mute hotkey (silent while held)
channelstrip-ptm-placeholder = e.g. Ctrl+Shift+M
channelstrip-ptm-aria = Push-to-mute hotkey
channelstrip-hotkeys-note = Hotkeys work while other apps are focused. On Linux/Wayland, global hotkeys may be unavailable — that's a compositor limit, said honestly.
channelstrip-apply = Apply


# --- LiveButton.tsx ---
livebutton-failure-ended = the stream ended
livebutton-title-live = End the stream — every target (a running recording continues)
livebutton-title-offline = Go live to every enabled Settings → Stream target
livebutton-end-stream = ■ End Stream
livebutton-aria-reconnecting = Reconnecting
livebutton-aria-live = Live
livebutton-badge-retry = retry { $n }
livebutton-badge-live = live
livebutton-go-live = ⦿ Go Live


# --- RecDot.tsx ---
recdot-paused-aria = Recording paused
recdot-recording-aria = Recording
recdot-tracks-one = { $count } audio track recording
recdot-tracks-other = { $count } audio tracks recording
recdot-paused = paused


# --- ReplayControls.tsx ---
replaycontrols-saved = Replay saved — { $name }
replaycontrols-failure-stopped = the buffer stopped
replaycontrols-title-disarm = Disarm the replay buffer (drops the un-saved history)
replaycontrols-title-arm = Arm the rolling replay buffer — keeps the last N seconds ready to save (its own lightweight encode; the stream and recording are untouched)
replaycontrols-replay-seconds = ⟲ Replay { $seconds }s
replaycontrols-arm = ⟲ Arm Replay Buffer
replaycontrols-save-title = Save the last N seconds to the recordings folder (also on the Save-Replay hotkey)
replaycontrols-save = ⤓ Save


# --- PropertiesDialog.tsx ---
properties-title = Properties — { $name }
properties-name = Name
properties-cancel = Cancel
properties-apply = Apply
properties-youtube = YouTube — channel / watch / live_chat URL (no key, no sign-in, ever)
properties-twitch = Twitch — channel name (anonymous)
properties-kick = Kick — channel slug (public endpoint)
properties-width-px = Width (px)
properties-lines = Lines
properties-font-px = Font (px)
properties-images = Image files (one path per line, shown in order)
properties-per-slide = Per-slide (ms)
properties-crossfade = Crossfade (ms, 0 = cut)
properties-loop-slideshow = Loop (off = hold the last slide)
properties-shuffle = Shuffle each cycle
properties-nested-scene = Scene this source composes (a scene that already contains this one is rejected)
properties-portal-note = The Wayland ScreenCast portal picks the screen or window in the system dialog every time this source starts — there is nothing to configure here, by design.
properties-appaudio-capturing = Capturing audio from { $exe }
properties-appaudio-exe-fallback = an application
properties-appaudio-pid = · pid { $pid }
properties-appaudio-note = Re-add the source to target a different app (a process id changes when the app restarts).
properties-image-file = Image file
properties-media-file = Media file (mp4, mkv, webm, mov, .frec, or an image)
properties-media-loop = Loop (restart from the top at the end)
properties-media-hwdecode = Hardware decode (falls back to software on its own)
properties-media-note = .frec plays through the owned freally-video codec — nothing to download. Other video formats decode through the on-demand FFmpeg component. The file's audio gets its own mixer strip; the strip's sync offset fine-tunes A/V alignment. A clip with no audio leaves its strip silent.
properties-color = Color
properties-width = Width
properties-height = Height
properties-testtone-note = A continuous 1 kHz sine at −20 dBFS. Level and mute live on its mixer strip; there is nothing else to configure.
properties-timer-format = Time format (strftime)
properties-timer-format-note = e.g. %H:%M:%S (default), %I:%M %p, %A %H:%M — an invalid pattern falls back to %H:%M:%S.
properties-timer-utc = UTC offset (minutes)
properties-timer-utc-placeholder = local time
properties-timer-duration = Duration (seconds)
properties-timer-target = Count down to (HH:MM)
properties-timer-target-note = A wall-clock target runs by itself and repeats daily; leave it empty to use the duration with Start/Pause/Reset.
properties-timer-end = At zero
properties-timer-end-none = Do nothing
properties-timer-end-flash = Flash the timer
properties-timer-end-switch = Switch scene
properties-timer-end-scene = Scene
properties-timer-size = Size (px)
properties-timer-start = Start
properties-timer-pause = Pause
properties-timer-reset = Reset
properties-stats-show-fps = Show FPS
properties-stats-show-cpu = Show CPU
properties-stats-show-memory = Show memory
properties-stats-show-render = Show render time
properties-stats-show-dropped = Show dropped frames
properties-stats-show-bitrate = Show bitrate
properties-stats-size = Size (px)
properties-vis-bands = Bars
properties-vis-decay = Fall rate (dB/s)
properties-vis-peak-hold = Peak-hold markers
properties-vis-missing-source = (missing source)
properties-splits-size = Size (px)
properties-splits-ahead = Ahead
properties-splits-behind = Behind
properties-splits-gold = Gold
properties-splits-split = Split
properties-splits-undo = Undo
properties-splits-skip = Skip
properties-splits-reset = Reset
properties-splits-note = The buttons drive the live timer (the global hotkeys do the same from any app). The run is never written back to the .lss file.
properties-playlist-items = Items (played top to bottom)
properties-playlist-up = Move up
properties-playlist-down = Move down
properties-playlist-remove = Remove item
properties-playlist-in = In (s)
properties-playlist-out = Out (s)
properties-playlist-cues = Cues (s, comma-separated)
properties-playlist-add-item = + Add item
properties-playlist-loop = Loop
properties-playlist-shuffle = Shuffle
properties-playlist-hold-last = Hold last frame
properties-playlist-hw = Hardware decode
properties-playlist-variable = "Now playing" variable (blank = off)
properties-playlist-previous = ⏮ Previous
properties-playlist-next = ⏭ Next
properties-playlist-note = The cue buttons and Next/Previous drive the LIVE playlist; item edits apply on Apply (the playlist restarts). Put {"{{"}yourVariable{"}}"} in a Text source to show the playing item's name.
properties-title-layers = Layers (drawn in order — later rows on top)
properties-title-kind-text = Text
properties-title-kind-image = Image
properties-title-kind-rect = Box
properties-title-x = X
properties-title-y = Y
properties-title-outline = Outline (px)
properties-title-outline-color = Outline
properties-title-shadow = Shadow
properties-title-animation = Animate in/out
properties-title-anim-none = None (cut)
properties-title-anim-fade = Fade
properties-title-anim-slide-left = Slide left
properties-title-anim-slide-up = Slide up
properties-title-anim-wipe = Wipe
properties-title-duration = Duration (ms)
properties-title-fire-in = ▶ Fire in
properties-title-fire-out = ◼ Fire out
properties-title-set-live = Set live
properties-title-set-live-note = Push this text into the LIVE title now — no Apply, no restart
properties-title-up = Move layer up
properties-title-down = Move layer down
properties-title-remove = Remove layer
properties-title-add-text = + Text
properties-title-add-image = + Image
properties-title-add-rect = + Box
properties-title-note = Fire in/out and Set live drive the RUNNING title; layer edits apply on Apply (the title restarts and fires its In pass again). Text cells can bind to a watched file (CSV cell / JSON value / whole file) and interpolate {"{{"}variables{"}}"} — overrides from Set live win over both.
properties-replay-roll = ⏵ Roll replay
properties-replay-note = Roll snapshots the ARMED replay buffer into a clip and plays it at the chosen speed — retimed, never interpolated. Slow motion is silent by design. Scrub and pause work while it plays; at the end the source clears back to transparency.
properties-lan-note = Applying a protocol, port, or passphrase change restarts the listener — the sender must reconnect. The stream is fitted onto a 1920×1080 canvas.
properties-stats-note = The HUD renders compact universal labels (FPS, CPU, MEM, RENDER, DROPPED, BITRATE) straight onto the program; while nothing streams, the bitrate line shows “—”.
properties-text-file = Read from file (path; empty = use the text above)
properties-text-binding = Parse as
properties-text-binding-whole = Whole file
properties-text-binding-csv = CSV cell
properties-text-binding-json = JSON pointer
properties-text-csv-row = Row
properties-text-csv-column = Column
properties-text-csv-column-placeholder = name or number
properties-text-json-pointer = Pointer
properties-text-file-note = The file is re-read within half a second of a change. Atomic writers (temp + rename) are tolerated: the last good value stays on screen through the swap.
avsync-title = A/V Sync Calibration
avsync-intro = Play the built-in flash + beep through your display and speakers, capture it back with the camera and microphone you want aligned, and the workbench measures the gap between them. The loop runs through your screen and speakers, so their own small latencies are included.
avsync-video-label = Camera (video source)
avsync-audio-label = Microphone (audio source)
avsync-pick = Pick a source…
avsync-no-video = Add the camera as a source first — the workbench measures sources, not raw devices.
avsync-no-audio = Add the microphone as an audio source first.
avsync-projector = Fullscreen the program on
avsync-projector-open = Open projector
avsync-projector-window-title = Program — A/V sync
avsync-start-note = Starting adds a temporary "A/V Sync Pattern" source on top of the current scene and plays the beep on your monitor device. Everything is removed when the run ends.
avsync-manual = Sync offset (ms, manual)
avsync-start = Start calibration
avsync-measuring = Measuring for about 12 seconds — point the camera at the flashing program and keep the room steady…
avsync-flash-seen = Camera sees the flash
avsync-flash-waiting = Waiting for the camera to see the flash…
avsync-beep-heard = Microphone hears the beep
avsync-beep-waiting = Waiting for the microphone to hear the beep…
avsync-cancel = Cancel
avsync-result-offset = Video arrives { $offset } ms after the audio.
avsync-result-detail = Measured over { $cycles } cycles, ±{ $jitter } ms.
avsync-negative = The audio already arrives later than the video. Delaying audio cannot fix this direction — if another strip carries this camera's sound, lower its sync offset instead.
avsync-over-cap = The measured gap is beyond the { $max } ms sync-offset cap. A gap that large usually means the wrong source was picked — check the chain and measure again.
avsync-applied = Applied — the microphone's sync offset is now { $offset } ms.
avsync-apply = Apply { $offset } ms to the microphone
avsync-again = Measure again
avsync-close = Close
avsync-error-noFlash = The camera never saw the flash. Point it at the flashing program (fullscreen helps) and make sure the source is live, then measure again.
avsync-error-noBeep = The microphone never heard the beep. Make sure the monitor device is audible and the mic is live (not push-to-talk gated), then measure again.
avsync-error-tooFewCycles = Not enough clean flash/beep cycles were captured. Keep the pattern clearly visible and audible for the whole run.
avsync-error-notThePattern = What was seen or heard does not repeat at the pattern's rhythm — likely room light or noise, not the test signal.
avsync-error-unstable = The cycles disagree too much to trust one number. Steady the camera, reduce room noise, and measure again.
hotkey-audit-title = Hotkey Map
hotkey-audit-search = Search
hotkey-audit-filter = Feature
hotkey-audit-filter-all = All features
hotkey-audit-col-key = Key
hotkey-audit-col-action = Action
hotkey-audit-col-where = Where
hotkey-audit-col-status = Status
hotkey-audit-ok = OK
hotkey-audit-shared = Shared by { $count } bindings
hotkey-audit-unregistered = Not registered with the OS (grabbed elsewhere or unavailable)
hotkey-audit-invalid = Not a valid accelerator
hotkey-audit-empty = No hotkeys are bound yet — bind them in Settings → Hotkeys or on a mixer strip.
hotkey-audit-export = Export cheat sheet
hotkey-audit-exported = Saved to { $path }
hotkey-audit-note = Bind and change keys in Settings → Hotkeys (global actions) and on each mixer strip (push-to-talk / push-to-mute); this table audits and documents them.
hotkey-audit-action-record = Toggle recording
hotkey-audit-action-go-live = Toggle streaming
hotkey-audit-action-transition = Commit transition
hotkey-audit-action-save-replay = Save replay
hotkey-audit-action-add-marker = Add marker
hotkey-audit-action-still = Capture still
hotkey-audit-action-panic = Panic slate
hotkey-audit-action-timer-toggle = Start/pause all timers
hotkey-audit-action-timer-reset = Reset all timers
hotkey-audit-action-split-split = Split (split timer)
hotkey-audit-action-split-undo = Undo split
hotkey-audit-action-split-skip = Skip segment
hotkey-audit-action-split-reset = Reset split timer
hotkey-audit-action-playlist-next = Playlist next
hotkey-audit-action-playlist-previous = Playlist previous
hotkey-audit-action-replay-roll = Roll instant replay
hotkey-audit-action-ptt = Push-to-talk
hotkey-audit-action-ptm = Push-to-mute
hotkey-audit-feature-recording = Recording
hotkey-audit-feature-streaming = Streaming
hotkey-audit-feature-studio = Studio mode
hotkey-audit-feature-replay = Replay
hotkey-audit-feature-markers = Markers
hotkey-audit-feature-stills = Stills
hotkey-audit-feature-panic = Panic
hotkey-audit-feature-timers = Timers
hotkey-audit-feature-split-timer = Split timer
hotkey-audit-feature-playlist = Playlist
hotkey-audit-feature-audio = Audio (per source)
properties-text = Text
properties-font-family = Font family (system; blank = default)
properties-size-px = Size (px)
properties-text-color = Text color
properties-align = Align
properties-align-left = left
properties-align-center = center
properties-align-right = right
properties-line-spacing = Line spacing
properties-wrap-width = Wrap width (px; 0 = off)
properties-force-rtl = Force right-to-left
properties-text-note = Rendering uses real shaping (Arabic joining, ligatures) and bidi line ordering. The bundled Noto Sans family (incl. Arabic/Hebrew) is the default; system families work too. CJK uses system fonts for now.
properties-repick-capturing = Capturing: { $label }
properties-repick-looking = Looking for sources…
properties-repick-none-displays = No displays found to re-pick.
properties-repick-none-windows = No windows found to re-pick.
properties-repick-again = Pick again:
properties-device = Device
properties-video-current-device = (current device)
properties-format = Format
properties-format-auto-loading = Auto (loading formats…)
properties-deinterlace = Deinterlacing
properties-deinterlace-off = Off
properties-deinterlace-discard = Discard (line-double one field)
properties-deinterlace-bob = Bob (alternate fields)
properties-deinterlace-linear = Linear (interpolate)
properties-deinterlace-blend = Blend (average fields)
properties-deinterlace-adaptive = Motion-adaptive (yadif-class)
properties-field-order = Field order
properties-field-order-top = Top field first
properties-field-order-bottom = Bottom field first
properties-deinterlace-note = For interlaced capture-card feeds. Pure CPU, identical on every OS; changing it restarts the device (like a format change).
camera-controls-title = Camera controls
camera-controls-refresh = Refresh
camera-controls-reset = Reset profile
camera-controls-empty = No controls right now — the device must be streaming (add it to a scene first), and some backends report none (notably macOS). This is the honest per-OS state.
camera-controls-note = Changes apply live and save into this device's profile, which reapplies on hotplug and restart.
camera-control-brightness = Brightness
camera-control-contrast = Contrast
camera-control-hue = Hue
camera-control-saturation = Saturation
camera-control-sharpness = Sharpness
camera-control-gamma = Gamma
camera-control-white-balance = White balance
camera-control-backlight = Backlight compensation
camera-control-gain = Gain
camera-control-pan = Pan
camera-control-tilt = Tilt
camera-control-zoom = Zoom
camera-control-exposure = Exposure
camera-control-iris = Iris
camera-control-focus = Focus
properties-format-auto = Auto (highest resolution)
properties-audio-capture-of = Capture the audio of
properties-audio-default-output = Default output (what you hear)
properties-audio-default-input = Default input
properties-audio-default-suffix = (default)
properties-audio-current-device = (current device: { $id })


# --- AudioFiltersDialog.tsx ---
audiofilters-name-gain = Gain
audiofilters-name-noise-gate = Noise Gate
audiofilters-name-compressor = Compressor
audiofilters-name-limiter = Limiter
audiofilters-name-eq = 3-Band EQ
audiofilters-name-denoise = Denoise
audiofilters-name-ducking = Ducking
audiofilters-title = Audio filters — { $name }
audiofilters-chain-header = Filter chain (top runs first, before the fader)
audiofilters-add = + Add filter
audiofilters-add-menu = Add an audio filter
audiofilters-empty = No filters yet — denoise a mic (classic DSP, no ML), gate the room, tame peaks with the compressor, or duck music under your voice.
audiofilters-enable = Enable { $name }
audiofilters-run-earlier = Run earlier
audiofilters-move-up = Move { $name } up
audiofilters-run-later = Run later
audiofilters-move-down = Move { $name } down
audiofilters-remove-title = Remove filter
audiofilters-remove = Remove { $name }
audiofilters-gain-db = Gain (dB)
audiofilters-open-db = Open at (dB)
audiofilters-close-db = Close at (dB)
audiofilters-attack-ms = Attack (ms)
audiofilters-hold-ms = Hold (ms)
audiofilters-release-ms = Release (ms)
audiofilters-ratio = Ratio (:1)
audiofilters-threshold-db = Threshold (dB)
audiofilters-output-gain-db = Output gain (dB)
audiofilters-ceiling-db = Ceiling (dB)
audiofilters-low-db = Low (dB)
audiofilters-mid-db = Mid (dB)
audiofilters-high-db = High (dB)
audiofilters-strength = Strength
audiofilters-denoise-note = Owned classic-DSP spectral suppression — steady noise (fans, hiss) drops while speech passes. No ML, no models, per the charter.
audiofilters-duck-under = Duck under
audiofilters-ducking-trigger = Ducking trigger source
audiofilters-pick-trigger = (pick a trigger — e.g. your mic)
audiofilters-trigger-at-db = Trigger at (dB)
audiofilters-duck-by-db = Duck by (dB)


# --- FiltersDialog.tsx ---
filters-name-chroma-key = Chroma Key
filters-name-color-key = Color Key
filters-name-luma-key = Luma Key
filters-name-render-delay = Render Delay
filters-name-color-correction = Color Correction
filters-name-lut = Apply LUT
filters-name-blur = Blur
filters-name-mask = Image Mask
filters-name-sharpen = Sharpen
filters-name-scroll = Scroll
filters-name-crop = Crop
filters-title = Filters — { $name }
filters-blend-mode = Blend mode
filters-chain-header = Filter chain (top runs first)
filters-add = + Add filter
filters-add-menu = Add a filter
filters-empty = No filters yet — chroma key a webcam, color-correct a capture, or scroll a ticker.
filters-enable = Enable { $name }
filters-run-earlier = Run earlier
filters-move-up = Move { $name } up
filters-run-later = Run later
filters-move-down = Move { $name } down
filters-remove-title = Remove filter
filters-remove = Remove { $name }
filters-key-color-rgb = Key color (any color, RGB distance)
filters-similarity = Similarity
filters-smoothness = Smoothness
filters-luma-min = Luma min (darker keys out)
filters-luma-max = Luma max (brighter keys out)
filters-delay = Delay (ms — video only, e.g. to sync with audio; capped at 500)
filters-key-color = Key color
filters-spill = Spill
filters-gamma = Gamma
filters-brightness = Brightness
filters-contrast = Contrast
filters-saturation = Saturation
filters-hue-shift = Hue shift
filters-opacity = Opacity
filters-cube-file = .cube file
filters-amount = Amount
filters-radius = Radius
filters-mask-image = Mask image
filters-mask-mode = Mode
filters-mask-alpha = alpha
filters-mask-luma = luma
filters-mask-invert = invert
filters-speed-x = Speed X (px/s)
filters-speed-y = Speed Y (px/s)
filters-crop-left = left
filters-crop-top = top
filters-crop-right = right
filters-crop-bottom = bottom
filters-crop-aria = crop { $side }
filters-cursorfx-header = Cursor effects
filters-cursorfx-hint = On Windows (which draws the cursor itself) these are painted straight into the capture, so they show up in recordings and streams. macOS and Linux composite the cursor OS-side, so these effects are Windows-only. Changes apply live.
filters-cursorfx-halo = Cursor halo
filters-cursorfx-halo-color = Color
filters-cursorfx-halo-radius = Radius (px)
filters-cursorfx-ripples = Click ripples
filters-cursorfx-left-color = Left click
filters-cursorfx-right-color = Right click
filters-cursorfx-keystrokes = Keystroke ghosting
filters-cursorfx-keystrokes-hint = Shows a fixed key set (letters, digits, modifiers, arrows) near the cursor while held. Keys are read only while this is on, drawn straight into the frame, and never stored or logged.


# --- PickerShell.tsx ---
pickershell-refresh-aria = Refresh
pickershell-refresh-title = Refresh the list
pickershell-close = Close


# =============================================================
# --- dialogs ---
# =============================================================
# dialogs
# Extracted user-visible strings from the dialog panels:
#   BugReport, Updates, Models, Recordings, OpenedFrec,
#   VerticalCanvasDialog, EulaGate.
# Brand names, technical tokens, and Fluent placeables are preserved verbatim.


# --- BugReport.tsx ---
bugreport-title = Report a bug
bugreport-intro = Reports are anonymous and opt-in — nothing is sent automatically. You'll review the exact text below, then submit it via a pre-filled GitHub issue or your email app. No personal data (your home path and username are redacted); no account, no server.
bugreport-crash-notice = Freally Capture closed unexpectedly on a previous run — the anonymous crash details are included below. Reporting them helps fix it fast.
bugreport-description-label = What were you doing when it happened? (optional)
bugreport-description-placeholder = e.g. the preview froze when I added a second webcam
bugreport-include-crash = Include the anonymous crash details from the last run
bugreport-preview-label = Exactly what will be sent
bugreport-open-github = Open GitHub issue
bugreport-gmail-title = Opens Gmail's compose window in your browser, pre-filled. Signed out? Google shows its login screen first.
bugreport-compose-gmail = Compose in Gmail
bugreport-email-title = Opens a draft in whatever mail app this PC uses by default (Outlook, Thunderbird, Mail…)
bugreport-send-email = Send email
bugreport-copied = Copied ✓
bugreport-copy-report = Copy report
bugreport-dismiss-crash = Dismiss crash
bugreport-copy-failed = couldn't copy — select the text and copy manually
# Composed report preview rendered into the <pre> block
bugreport-preview-what-happened = WHAT HAPPENED
bugreport-preview-no-description = (no description provided)
bugreport-preview-diagnostics = ANONYMOUS DIAGNOSTICS (no personal data)
bugreport-preview-from = From: Freally Capture
bugreport-preview-crash-excerpt = --- crash excerpt ---


# --- Updates.tsx ---
updates-title = Software update
updates-checking = Checking for updates…
updates-uptodate = You're on the latest version.
updates-check-again = Check again
updates-available = Version { $version } is available
updates-current-version = (you have { $current })
updates-release-notes-label = Version { $version } — Release notes
updates-confirm = Do you want to update now? The download is verified against the bundled signing key before it's applied. Freally Capture closes, the installer runs, and the new version reopens by itself.
updates-yes-update-now = Yes, update now
updates-no-not-now = No, not now
updates-downloading = Downloading { $version }…
updates-starting = starting…
updates-installed = Update installed.
updates-restart-now = Restart now
updates-restart-later = Restart later
updates-try-again = Try again


# --- Models.tsx ---
models-title = Components
models-ffmpeg-heading = FFmpeg — wire codecs
models-badge-third-party = Third-party · not bundled
models-ffmpeg-desc = Freally Capture's own engine records lossless freally-video (.frec) with nothing extra. Recording the wire formats platforms and players expect — H.264/AAC (and HEVC/AV1) in mp4/mkv/mov/webm — uses FFmpeg, a separate tool this app never ships with: those codecs are patent-encumbered, so it stays optional and clearly labeled. It is downloaded on demand from the pinned build below, SHA-256-verified before first use, cached per-user, and driven as a separate process. Its license (LGPL/GPL) is its own — see THIRD-PARTY-NOTICES.
models-checking = Checking…
models-ffmpeg-not-installed = Not installed. Available: FFmpeg { $version } from { $source } ({ $size } download).
models-ffmpeg-none-pinned = No FFmpeg build is pinned for this platform yet — wire-codec recording is unavailable here. Lossless freally-video recording is unaffected.
models-ffmpeg-download-verify = Download & verify ({ $size })
models-downloading = Downloading…
models-download-of = of
models-cancel = Cancel
models-ffmpeg-verifying = Verifying the download against the pinned SHA-256…
models-ffmpeg-extracting = Unpacking…
models-ffmpeg-ready = Installed & verified — { $version }
models-remove = Remove
models-ffmpeg-retry = Retry download
models-network-note = The download is the only network action on this panel and never starts on its own. A failed checksum aborts the install — the app refuses to run bytes it cannot vouch for.
models-cef-heading = Browser Source runtime — Chromium (CEF)
models-cef-desc = Browser sources render web pages (alerts, widgets, overlays) through Chromium Embedded Framework — a ~100 MB runtime this app never ships with. It downloads on demand from the official CEF build index, is verified against that index's SHA-1 before anything is unpacked, and is cached per-user. The browser source that renders through it arrives with its own milestone; this installs the runtime it needs.
models-cef-download-install = Download & install
models-cef-unsupported = CEF publishes no build for this platform — browser sources are unavailable here.
models-cef-resolving = Resolving the latest stable build…
models-cef-verifying = Verifying the download against the index SHA-1…
models-cef-extracting = Unpacking the runtime…
models-cef-ready = Installed — CEF { $version }.
models-cef-retry = Retry
models-integrations-heading = Optional integrations
models-badge-never-bundled = Never bundled
models-ndi-detected = Detected
models-ndi-not-installed = Not installed
models-vst-available = Available
models-vst-not-available = Not available


# --- Recordings.tsx ---
recordings-title = Recordings
recordings-loading = Reading the folder…
recordings-empty = No recordings yet — Start Recording writes into the folder set in Output.
recordings-frec-label = owned lossless (freally-video)
recordings-remux-title = Rewrap as mp4 — stream copy, no re-encode, no quality change (needs the FFmpeg component)
recordings-remuxing = Remuxing…
recordings-remux-to-mp4 = Remux to MP4
recordings-export-mp4-title = Decode the owned .frec and re-encode to MP4 (H.264/AAC) so it plays in any player — needs the FFmpeg component
recordings-exporting = Exporting…
recordings-export-mp4 = Export → MP4
recordings-export-mkv-title = Decode the owned .frec and re-encode to MKV so it plays in any player
recordings-starting = starting…
recordings-frames = { $done } / { $total } frames
recordings-cancel = Cancel
recordings-export-cancelled = Export cancelled.
recordings-exported-to = Exported to { $path }
recordings-remuxed-to = Remuxed to { $path }


# --- OpenedFrec.tsx ---
openfrec-title = Open .frec recording
openfrec-desc = Freally Capture records the owned lossless .frec format — it doesn't play it. Freally Player will play .frec directly when it's released. For now, export it to MP4/MKV and it plays in any player (VLC, your OS player, anything).
openfrec-exported-to = Exported to { $path }
openfrec-exporting = Exporting…
openfrec-starting = starting…
openfrec-export-mp4 = Export → MP4
openfrec-export-mkv = Export → MKV


# --- VerticalCanvasDialog.tsx ---
vertical-title = Vertical canvas (9:16)
vertical-enable = Enable the second canvas — recordable and streamable independently of the program
vertical-scene-label = Scene this canvas composes
vertical-width = Width
vertical-height = Height
vertical-preview-alt = Vertical canvas preview
vertical-note = Item positions are pixel-true across canvases: select this scene in the Scenes rail to arrange it while this preview shows the vertical result. Stream targets pick this canvas in ⦿ Stream…; Settings → Output can record it alongside the main file.
vertical-close = Close


# --- EulaGate.tsx ---
eula-title = Freally Capture — License Agreement
eula-version = v{ $version }
eula-intro = Please read and accept this agreement to use Freally Capture. In short: it's a neutral tool, and you are solely responsible for what you capture, record, and broadcast — and for having the rights to it.
eula-thanks = Thanks for reading.
eula-scroll-hint = Scroll to the end to continue.
eula-decline = Decline & Quit
eula-agree = I Agree


# =============================================================
# --- settings ---
# =============================================================
# settings

# --- SettingsOutput.tsx ---
output-title = Settings — Output
output-loading = Settings are still loading…
output-container-frec = freally-video (.frec) — lossless, owned, nothing to download
output-container-mkv = MKV — crash-tolerant; remux to mp4 later
output-container-mp4 = MP4 — plays everywhere
output-container-mov = MOV
output-container-webm = WebM (AV1 + Opus)
output-preset-lossless-label = Lossless
output-preset-lossless-title = The owned freally-video codec — bit-exact, no download
output-preset-high-label = High quality
output-preset-high-title = MP4, best-detected encoder, near-lossless CQ 16, Quality preset
output-preset-balanced-label = Balanced
output-preset-balanced-title = MKV, best-detected encoder, CQ 23, Balanced preset
output-recording-format = Recording format
output-ffmpeg-warning = This format needs the FFmpeg component (wire codecs — not bundled). Lossless .frec needs nothing.
output-install = Install…
output-recordings-folder = Recordings folder
output-folder-placeholder = OS Videos folder
output-filename-prefix = Filename prefix
output-recording-template = Recording filename
output-replay-template = Replay filename
output-still-template = Still filename
output-template-tokens = Tokens: {"{prefix}"}, {"{date}"}, {"{time}"}, {"{scene}"}, {"{profile}"}, {"{canvas}"}, {"{marker-count}"}, {"{counter}"}
output-replay-folder = Replay folder
output-still-folder = Still folder
output-same-folder-placeholder = Recordings folder
output-frame-rate = Frame rate
output-fps-option = { $fps } fps
output-split-every = Split every (minutes, 0 = off)
output-output-width = Output width (0 = canvas; wire formats only)
output-output-height = Output height (0 = canvas)
output-record-vertical = Also record the vertical canvas (a parallel “… (vertical)” file; needs the 9:16 canvas enabled)
output-audio-tracks = Audio tracks
output-recorded-tracks-group = Recorded tracks
output-track-last-one = At least one track must record
output-record-track-on = Record track { $index }: on
output-record-track-off = Record track { $index }: off
output-encoder-heading = Encoder
output-video-encoder = Video encoder
output-encoder-auto = Auto — best detected (H.264)
output-encoder-unavailable = — unavailable here
output-preset = Preset
output-preset-quality = Quality
output-preset-balanced-option = Balanced
output-preset-performance = Performance
output-rate-control = Rate control
output-rc-cqp = CQP (constant quality)
output-rc-cbr = CBR (constant bitrate)
output-rc-vbr = VBR (variable bitrate)
output-cq = CQ (0–51, lower = better)
output-bitrate = Bitrate (kbps)
output-keyframe = Keyframe interval (s)
output-audio-bitrate = Audio bitrate (kbps / track)
output-presets = Presets:

# --- SettingsStream.tsx ---
stream-title = Settings — Stream
stream-target-enabled = Target { $index } enabled
stream-target = Target { $index }
stream-remove = Remove
stream-service = Service
stream-canvas = Canvas
stream-canvas-main = Main (program)
stream-canvas-vertical = Vertical (9:16 — enable it in the studio)
stream-ingest-srt = SRT ingest URL
stream-ingest-whip = WHIP endpoint URL
stream-ingest-url = Ingest URL
stream-ingest-override = (override — empty = the service preset)
stream-key-srt = streamid (optional — appended as ?streamid=…; treated as a secret)
stream-key-whip = Bearer token (optional — sent as the Authorization header; a secret)
stream-key-custom = Stream key (from your server — treated as a secret)
stream-key-service = Stream key (from your creator dashboard — treated as a secret)
stream-key-aria = Stream key { $index }
stream-key-hide = Hide
stream-key-show = Show
stream-encoder = Encoder (H.264 — what RTMP, SRT and WHIP all carry)
stream-encoder-auto = Auto — the best detected H.264 encoder
stream-encoder-unavailable = (unavailable here)
stream-video-bitrate = Video bitrate (kbps, CBR)
stream-audio-bitrate = Audio bitrate (kbps)
stream-fps = FPS
stream-keyframe = Keyframe interval (s)
stream-audio-track = Audio track (1–6)
stream-output-width = Output width (0 = canvas)
stream-output-height = Output height (0 = canvas)
stream-add-target = + Add target
stream-go-live-note = Go Live publishes to every enabled target at once, direct to each platform. Targets with identical encoder settings share a single encode.
stream-auto-record = Start recording when I go live (the recording still stops independently)
stream-ffmpeg-note-before = Streaming wire codecs run through the labeled on-demand ffmpeg component —
stream-ffmpeg-note-link = manage it here
stream-ffmpeg-note-after = . The local recording keeps running no matter what the stream does.
stream-cancel = Cancel
stream-save = Save

# --- SettingsReplay.tsx ---
replay-title = Settings — Replay Buffer
replay-length-15s = 15 s
replay-length-30s = 30 s
replay-length-1min = 1 min
replay-length-2min = 2 min
replay-length-5min = 5 min
replay-quality-low = Low (3 Mbps)
replay-quality-standard = Standard (6 Mbps)
replay-quality-high = High (12 Mbps)
replay-length-presets = Length presets
replay-quality-presets = Quality presets
replay-length-seconds = Length (seconds)
replay-video-bitrate = Video bitrate (kbps)
replay-fps = FPS
replay-audio-track = Audio track (1–6)
replay-note = While armed, the buffer runs its own lightweight encode into a bounded on-disk ring — about { $mb } MB at these settings. Saving stitches the ring without re-encoding and never touches the stream or the recording. Changes apply the next time you arm.
replay-cancel = Cancel
replay-save = Save

# --- SettingsRemote.tsx ---
remote-title = Settings — Remote Control
remote-enable = Enable the WebSocket remote API
remote-password = Password (required — controllers authenticate with it)
remote-password-placeholder = a password for your controllers
remote-password-hide = Hide
remote-password-show = Show
remote-port = Port
remote-allow-lan = Allow LAN connections (default is this machine only)
remote-note = Off = the port is closed. On = a password-protected WebSocket on 127.0.0.1 (or your LAN when opted in) that can switch scenes, run the transition, start/stop the stream and recording, save replays, and set mutes/volumes — the same actions as the UI, nothing more. It cannot read files. Treat the password like any credential; prefer this-machine-only unless you specifically control from another device.
remote-password-required = A password is required to enable the remote API.
remote-cancel = Cancel
remote-save = Save

# --- SettingsHotkeys.tsx ---
hotkeys-title = Settings — Hotkeys
hotkeys-record = Start / stop recording
hotkeys-record-placeholder = e.g. Ctrl+Shift+R
hotkeys-go-live = Go Live / End Stream
hotkeys-go-live-placeholder = e.g. Ctrl+Shift+L
hotkeys-transition = Studio-Mode Transition
hotkeys-transition-placeholder = e.g. Ctrl+Shift+T or F13
hotkeys-save-replay = Save Replay (last N seconds)
hotkeys-save-replay-placeholder = e.g. Ctrl+Shift+S
hotkeys-add-marker = Drop a chapter marker (recording)
hotkeys-add-marker-placeholder = e.g. Ctrl+Shift+K
hotkeys-note = Hotkeys are global — they fire while other apps are focused. Blank = unbound. Mixer push-to-talk/mute keys live on each strip's ⋯ menu. On Linux/Wayland, global hotkeys may be unavailable (a compositor limit) — the buttons keep working.
hotkeys-cancel = Cancel
hotkeys-save = Save

# --- WorkspaceDialog.tsx ---
workspace-title = Profiles & Scene Collections
workspace-profiles = Profiles
workspace-profiles-hint = A profile is your settings — stream target, output, hotkeys. Switch per show or per platform.
workspace-collections = Scene collections
workspace-collections-hint = A collection is your scenes + sources. Create duplicates the current one as a starting point.
workspace-active = Active
workspace-switch-to = Switch to { $name }
workspace-active-marker = ● active
workspace-new-name-placeholder = new name…
workspace-new-name-label = New { $title } name
workspace-create = Create

# --- OBS import (CAP-M02) ---
workspace-import-obs = Import from OBS…
workspace-import-obs-hint = Bring in an OBS scene collection (its scenes.json). Your current collection is saved first.
workspace-import-busy = Importing…
workspace-import-title = Imported "{ $name }"
workspace-import-summary = { $scenes } scenes · { $sources } sources · { $items } items
workspace-import-dismiss = Dismiss
workspace-import-clean = Everything came across cleanly.
workspace-import-geometry-caveat = Item sizes and positions are fitted from OBS's layout — review each scene, and re-pick any capture device.
workspace-import-notes-title = Imported with notes
workspace-import-skipped-title = Not imported
import-note-needsReselect = Re-pick the device/monitor/window
import-note-gameCaptureAsWindow = Game Capture → Window Capture
import-note-referencesFile = Check the file path
import-note-filterDropped = Some filters weren't supported
import-note-geometryApproximated = Position/size approximated
import-skip-unsupportedKind = No equivalent source type
import-skip-group = Groups aren't supported yet

# --- Missing-file doctor (CAP-M03) ---
palette-doctor = Relink missing files…
doctor-title = Missing files
doctor-scanning = Scanning…
doctor-all-good = Every referenced file resolves. Nothing to relink.
doctor-intro = { $count } referenced files can't be found on this computer. Point each to its new location — every scene that uses it is fixed at once.
doctor-relinked = Relinked { $count } references.
doctor-uses = used { $count }×
doctor-locate = Locate…
doctor-locate-folder = Locate in folder…
doctor-locate-folder-hint = Pick a folder; each missing file is matched by name and repointed.
doctor-kind-image = image
doctor-kind-media = media
doctor-kind-slideshow = slideshow
doctor-kind-font = font
doctor-kind-lut = LUT
doctor-kind-mask = mask
history-relinkFiles = Relink files

# --- ScriptsDialog.tsx ---
scripts-title = Scripts (Lua)
scripts-empty = No scripts yet — add a .lua file. See scripts/sample.lua for the API: react to go-live/scene/recording events and drive the same commands as the remote API.
scripts-enable = Enable { $path }
scripts-remove = Remove { $path }
scripts-path-label = Script path
scripts-add = Add
scripts-note = Scripts run sandboxed — no file or OS access; they can only call the same studio commands as the remote API (switch scenes, transition, record/stream/replay, mutes). A script error is logged and contained. Changes apply within a second.
scripts-error-not-lua = Point at a .lua file.

# --- BrowserDock.tsx ---
browser-dock-title = Browser docks
browser-dock-empty = No docks yet — add a chat popout, an alerts page, or your Companion web buttons.
browser-dock-open = Open
browser-dock-remove = Remove { $name }
browser-dock-name-placeholder = name (e.g. Twitch Chat)
browser-dock-name-label = Dock name
browser-dock-url-label = Dock URL
browser-dock-note = A dock opens as its own window you can place beside the studio. The page gets no access to the app — it just renders. http(s) URLs only; docks open only when you click Open.
browser-dock-error-name = Name the dock (e.g. Twitch Chat).
browser-dock-error-url = A dock URL must start with http:// or https://.

# --- studio-preview-pane ---
studio-preview-label = Studio Mode preview
studio-preview-heading = Preview
studio-preview-hint = click a scene to load it here
studio-preview-empty = The preview will appear here.
studio-preview-mirrors = mirrors program
studio-preview-transition-select = Transition
studio-preview-duration = Transition duration (ms)
studio-preview-commit-title = Commit Preview → Program through the transition (the audience sees it)
studio-preview-transitioning = Transitioning…
studio-preview-transition-button = Transition ⇄
studio-preview-luma-placeholder = grayscale wipe image (png/jpg)
studio-preview-luma-label = Luma wipe image
studio-preview-browse = Browse…
studio-preview-filter-images = Images
studio-preview-filter-video = Video
studio-preview-stinger-placeholder = stinger video (ProRes 4444 .mov keeps its alpha)
studio-preview-stinger-label = Stinger video file
studio-preview-stinger-cut-label = Stinger cut point (ms)
studio-preview-stinger-cut-title = When the scene swap lands under the stinger (ms into the transition)

# --- transition kinds (rendered from TRANSITION_KINDS in api/types.ts) ---
transition-kind-cut = Cut
transition-kind-fade = Fade
transition-kind-slide-left = Slide ←
transition-kind-slide-right = Slide →
transition-kind-slide-up = Slide ↑
transition-kind-slide-down = Slide ↓
transition-kind-swipe-left = Swipe ←
transition-kind-swipe-right = Swipe →
transition-kind-luma-linear = Luma wipe (linear)
transition-kind-luma-radial = Luma wipe (radial)
transition-kind-luma-horizontal = Luma wipe (horizontal)
transition-kind-luma-diamond = Luma wipe (diamond)
transition-kind-luma-clock = Luma wipe (clock)
transition-kind-image = Image wipe (custom)
transition-kind-stinger = Stinger (video)

# --- stream services (rendered from STREAM_SERVICES in api/types.ts) ---
stream-service-twitch = Twitch
stream-service-youtube = YouTube
stream-service-kick = Kick
stream-service-facebook = Facebook
stream-service-trovo = Trovo
stream-service-custom = Custom (RTMP/RTMPS)
stream-service-srt = SRT (self-hosted)
stream-service-whip = WHIP (WebRTC)

# --- about (TASK-907) ---
about-title = About
about-tagline = Record and stream like a studio — no accounts, no cloud.
about-version = Version
about-created-by = Created by
about-project-started = Project started
about-first-stable = First stable release
about-first-stable-pending = Not yet — 1.0.0 is in progress
about-platform = Platform
about-local-first = Freally Capture runs entirely on your machine. No accounts, no telemetry, no cloud — the only thing that leaves your computer is the stream you chose to send.
about-website = Website
about-issues = Report an issue
about-license = License
about-eula = EULA
about-third-party = Third-party notices
about-check-updates = Check for updates…

# --- unified settings modal (TASK-906) ---
settings-title = Settings
settings-language-section = Language
settings-language = Interface language
settings-language-system = System default
settings-language-note = A language you pick here is remembered. “System default” follows your operating system. Untranslated text falls back to English.
settings-appearance-section = Appearance
settings-theme = Theme
settings-theme-dark = Dark
settings-theme-light = Light
settings-theme-custom = Custom
settings-accent = Accent
settings-general-section = General
settings-show-stats-dock = Show the stats dock
settings-more-section = More settings
settings-open-output = Recording…
settings-open-stream = Streaming…
settings-open-replay = Replay…
settings-open-hotkeys = Hotkeys…
settings-open-remote = Remote API…
settings-open-about = About…
controls-settings = ⚙ Settings…
controls-settings-title = Language, appearance, and the app-wide preferences

# --- command palette (TASK-904) ---
palette-title = Command palette
palette-search = Search scenes, sources and actions
palette-placeholder = Search scenes, sources, actions…
palette-no-results = Nothing matches “{ $query }”
palette-hint = ↑ ↓ to move · Enter to run · Esc to close
palette-group-scenes = Scene
palette-group-sources = Source
palette-group-actions = Action
palette-transition = Transition Preview → Program
palette-save-replay = Save replay
palette-add-marker = Drop a chapter marker
palette-vertical-canvas = Vertical (9:16) canvas…

# --- first-run wizard (TASK-903 + TASK-905) ---
wizard-title = Welcome to Freally Capture
wizard-welcome = Two quick steps: check what your machine can do, then start a scene. It takes about thirty seconds, and you can change everything later.
wizard-local-first = Nothing here leaves your computer. Freally Capture has no accounts, no telemetry, and no cloud.
wizard-start = Get started
wizard-skip = Skip
wizard-hardware-title = What your machine can do
wizard-probing = Checking your graphics card and processor…
wizard-encoder = Encoder
wizard-canvas = Canvas
wizard-bitrate = Bitrate
wizard-probe-found = Found: { $gpus } · { $cores } physical cores
wizard-no-gpu = no dedicated GPU
wizard-apply = Use these settings
wizard-keep-current = Keep what I have
wizard-template-title = Start with a scene
wizard-template-screen = Capture my screen
wizard-template-screen-note = Adds a Display Capture of your main monitor. The most common place to start.
wizard-template-empty = Start empty
wizard-template-empty-note = An empty scene. Add sources yourself with the + button.
wizard-done = You're set up.
wizard-done-hint = Press Ctrl+K at any time to search scenes, sources and actions. Settings live behind the ⚙ button.
wizard-close = Start streaming

# --- auto-config reasons (rendered by the wizard; keys come from Rust) ---
autoconfig-reason-hardware = Your graphics card can encode video on its own, which leaves the processor free for the rest of the studio.
autoconfig-reason-software = No usable hardware encoder was found, so the processor will encode. That works, it just costs more CPU.
autoconfig-reason-quality-hardware = 1080p at 60 frames per second, at a bitrate every major platform accepts.
autoconfig-reason-quality-software = 30 frames per second, because software encoding at 60 drops frames on most processors.
autoconfig-reason-quality-low-cores = A lower bitrate, because this processor has few cores and software encoding will fight the compositor for them.

# --- screen-reader announcements (TASK-901, aria-live) ---
announce-recording-started = Recording started
announce-recording-paused = Recording paused
announce-recording-stopped = Recording stopped
announce-live-started = You are live
announce-live-ended = Stream ended
announce-reconnecting = Connection lost, reconnecting
announce-stream-failed = Stream failed
announce-frames-dropped = { $count } frames dropped

# CAP-M01 — undo/redo edit history
palette-undo = Undo
palette-redo = Redo
palette-edit-history = Edit history…
history-title = Edit history
history-empty = No edits to undo yet.
history-current = Current state
history-close = Close
history-addScene = Add scene
history-renameScene = Rename scene
history-removeScene = Remove scene
history-reorderScene = Reorder scenes
history-addSource = Add source
history-removeSource = Remove source
history-reorderSource = Reorder sources
history-renameSource = Rename source
history-transformSource = Move source
history-toggleVisibility = Toggle visibility
history-toggleLock = Toggle lock
history-setBlendMode = Change blend mode
history-editSourceProperties = Edit properties
history-applyLayout = Arrange layout
history-moveToSeat = Move to seat
history-groupSources = Group sources
history-ungroupSources = Ungroup sources
history-toggleGroupVisibility = Toggle group
history-setSceneAudio = Scene audio
history-setVerticalCanvas = Vertical canvas
history-addFilter = Add filter
history-removeFilter = Remove filter
history-reorderFilter = Reorder filters
history-editFilter = Edit filter
history-toggleFilter = Toggle filter
history-setVolume = Adjust volume
history-toggleMute = Toggle mute
history-setMonitor = Change monitoring
history-setTracks = Change tracks
history-setSyncOffset = Adjust A/V sync
history-setAudioHotkeys = Audio hotkeys

# CAP-M04 — alignment aids
settings-alignment-section = Alignment aids
settings-smart-guides = Smart guides (snap while dragging)
settings-safe-areas = Safe-area overlays
settings-rulers = Rulers
align-group = Align to canvas
align-left = Align left
align-hcenter = Center horizontally
align-right = Align right
align-top = Align top
align-vcenter = Center vertically
align-bottom = Align bottom

# --- Arrange + custom guides (CAP-M04 follow-on) ---
arrange-group = Align & distribute selected
arrange-left = Align left edges
arrange-hcenter = Align horizontal centers
arrange-right = Align right edges
arrange-top = Align top edges
arrange-vcenter = Align vertical centers
arrange-bottom = Align bottom edges
distribute-h = Distribute horizontally
distribute-v = Distribute vertically
guides-group = Guides
guides-add-v = Add vertical guide
guides-add-h = Add horizontal guide
guides-clear = Clear all guides
history-arrangeItems = Arrange items
history-editGuides = Edit guides

# CAP-M05 — edit transform + copy/paste
transform-title = Edit Transform — { $name }
transform-anchor = Anchor
transform-x = X
transform-y = Y
transform-w = W
transform-h = H
transform-rotation = Rotation
transform-crop = Crop
transform-crop-left = Left
transform-crop-top = Top
transform-crop-right = Right
transform-crop-bottom = Bottom
transform-no-size = Size and crop become available once the source reports its dimensions.
transform-copy = Copy transform
transform-paste = Paste transform
transform-close = Close
filters-copy = Copy filters ({ $count })
filters-paste = Paste filters ({ $count })
palette-edit-transform = Edit transform…
history-pasteFilters = Paste filters

# CAP-M26 — keying workbench
workbench-title = Keying Workbench — { $name }
workbench-mode-keyed = Keyed
workbench-mode-source = Source
workbench-mode-matte = Matte
workbench-mode-split = Split
workbench-eyedropper = Eyedropper
workbench-eyedropper-hint = Click the source to sample the key colour.
workbench-loupe = Loupe
workbench-split = Split
workbench-preview-alt = Keying workbench preview
workbench-tune = Tune
workbench-close = Close

# CAP-M06 — multiview monitor
multiview-title = Multiview
multiview-program = PGM
multiview-preview = PVW
multiview-hint-cut = Click a scene to cut to it.
multiview-hint-stage = Click a scene to stage it in preview.
palette-multiview = Multiview monitor

# CAP-M07 — projectors
projector-title = Open projector
projector-source = Source
projector-target-program = Program
projector-target-preview = Preview
projector-target-scene = Scene…
projector-target-source = Source…
projector-target-multiview = Multiview
projector-which-scene = Which scene
projector-which-source = Which source
projector-none = Nothing to show there
projector-display = Display
projector-windowed = Floating window (this screen)
projector-display-option = Display { $n } — { $w }×{ $h }
projector-primary = (primary)
projector-open = Open
projector-cancel = Cancel
projector-exit-hint = Press Esc to exit
palette-projector = Open projector…

# CAP-M08 — still-frame grab
palette-still = Grab still frame…
still-saved-toast = Still saved: { $name }
still-failed-toast = Still grab failed: { $error }
hotkeys-still = Grab still
hotkeys-still-placeholder = e.g. Ctrl+Shift+P

# CAP-M13 — source health dashboard
palette-source-health = Source health…
palette-av-sync = A/V sync calibration…
palette-hotkey-audit = Hotkey map…
health-title = Source Health
health-col-source = Source
health-col-state = State
health-col-resolution = Resolution
health-col-fps = FPS
health-col-last-frame = Last frame
health-col-dropped = Dropped
health-col-retries = Restarts
health-col-actions = Actions
health-state-live = Live
health-state-waiting = Waiting
health-state-error = Error
health-state-inactive = Inactive
health-restart = Restart
health-properties = Properties
health-empty = This collection has no sources yet.
health-seconds = { $value } s

# CAP-M23 — quit guard + orderly shutdown
quit-title = Quit Freally Capture?
quit-body = Quitting now will safely do the following, in order:
quit-consequence-stream = End the live stream and disconnect from the service.
quit-consequence-recording = Stop the recording and finalize its file(s).
quit-consequence-replay = Shut down the replay buffer — unsaved replay footage is discarded.
quit-confirm = Quit safely
quit-quitting = Shutting down…
quit-cancel = Cancel

# CAP-M11 — crash-safe recording salvage
salvage-title = Recover interrupted recordings?
salvage-body = The last session ended unexpectedly while these recordings were still being written. Repair writes a playable copy next to the original — the original file is never changed.
salvage-repair = Repair
salvage-repairing = Repairing…
salvage-done = Repaired
salvage-repaired = Repaired → { $name }
salvage-failed = Repair failed: { $error }
salvage-dismiss = Not now

# CAP-M12 — mid-session encoder failover
fallback-toast-stream = Encoder fault — switched from { $from } to { $to }. The stream reconnected and stays up.
fallback-toast-recording = Encoder fault — switched from { $from } to { $to }. The recording continues in a new file.
fallback-note = Encoder fell back: { $from } → { $to }

# CAP-M10 — broadcast safety alarms
alarm-silentAudio = Program audio has gone silent
alarm-clipping = Program audio is clipping
alarm-black = Program picture is black
alarm-frozen = Program picture has not changed for a while
alarm-lowDisk = Disk space: about { $minutes } min left at the current bitrate
alarm-dismiss = Dismiss alarm
alarm-cleared = Resolved: { $alarm }

# CAP-M22 — panic button
palette-panic = Panic — cut to privacy slate
panic-banner-title = Panic
panic-banner-body = Program shows the privacy slate; all audio is muted and captures are stopped. Stream and recording stay up.
panic-restore = Restore…
panic-restore-confirm = Restore the program?
panic-restore-yes = Restore
panic-restore-cancel = Cancel
hotkeys-panic = Panic (privacy slate)
hotkeys-panic-placeholder = e.g. Ctrl+Shift+F12
hotkeys-timer-toggle = Start/pause all timers
hotkeys-timer-toggle-placeholder = e.g. Ctrl+Shift+T
hotkeys-timer-reset = Reset all timers
hotkeys-timer-reset-placeholder = e.g. Ctrl+Shift+0
panic-slate-color = Panic slate colour
panic-slate-image = Panic slate image
panic-slate-image-placeholder = Optional image path

# CAP-M24 — redacted diagnostics bundle
diag-title = Diagnostics bundle
diag-intro = Export a redacted .zip (config snapshot, encoder probe, recent stats — secrets, paths and names are never included) to attach to a GitHub issue by hand. Nothing is sent anywhere.
diag-preview = Preview contents
diag-hide-preview = Hide preview
diag-export = Export .zip
diag-exported = Exported: { $path }

# CAP-M09 — go-live pre-flight checklist
preflight-title = Go-live pre-flight
preflight-intro = Every blocking item must be green; the rest are honest nudges.
preflight-item-targets = Stream targets configured (key/URL set)
preflight-item-encoder = A usable encoder is available
preflight-item-sources = All sources healthy
preflight-item-disk = Disk space for the recording
preflight-item-mic = Microphone metering
preflight-item-desktopAudio = Desktop audio metering
preflight-item-replay = Replay buffer armed
preflight-targets-detail = { $count } enabled
preflight-sources-detail = { $count } source(s) errored
preflight-disk-detail = ~{ $minutes } min at the current bitrate
preflight-fix-stream = Stream settings…
preflight-fix-components = Components…
preflight-fix-sources = Source health…
preflight-fix-replay = Arm
preflight-optional = optional
preflight-hold = Hold Go Live until all checks are green
preflight-cancel = Cancel
preflight-go-anyway = Go Live anyway
preflight-go-live = Go Live


# =============================================================
# --- Scene backdrop (wallpaper) + Flip filter ---
# =============================================================
scenes-backdrop = Backdrop
scenes-backdrop-aria = Backdrop for { $name }
backdrop-title = Backdrop — { $name }
backdrop-hint = A wallpaper pinned behind everything in this scene — an image, an animated GIF, or a looping video. Your capture always sits on top; scroll on the canvas to zoom.
backdrop-choose = Choose image or video…
backdrop-remove = Remove backdrop
backdrop-none = No backdrop set.
backdrop-position = Position
backdrop-split-full = Full canvas
backdrop-split-left = Left half
backdrop-split-right = Right half
backdrop-split-top = Top half
backdrop-split-bottom = Bottom half
backdrop-sync = Start playback when recording starts
backdrop-sync-hint = Holds on the first frame until you record; every take starts the video from the top.
backdrop-preview-play = Preview playback
backdrop-preview-pause = Pause preview
backdrop-filter-all = Backdrops (images & video)
backdrop-filter-images = Images
backdrop-filter-media = Video & GIF
sources-backdrop-badge = Backdrop wallpaper (pinned to the bottom)
sources-backdrop-pinned = The backdrop stays pinned at the bottom
filters-name-flip = Flip
filters-flip-horizontal = Horizontal
filters-flip-vertical = Vertical
history-setSceneBackdrop = Set backdrop
history-setBackdropSplit = Move backdrop
history-setBackdropSync = Backdrop recording sync
backdrop-scrub = Playback position
backdrop-loop = Loop
backdrop-reverse = Play in reverse
backdrop-reverse-hint = Reverse renders a backwards copy once (videos need the ffmpeg component; GIFs reverse instantly) — the first switch can take a while on long files.
filters-scaling = Scaling
filters-scaling-hint = Pixel-perfect modes for retro/pixel content; Integer also snaps the drawn size to whole multiples (the handles show the logical size).
filters-scaling-auto = Smooth
filters-scaling-nearest = Nearest
filters-scaling-integer = Integer (whole ×)
filters-scaling-sharp = Sharp bilinear
history-setScaling = Change scaling
hotkeys-zoom-100 = Zoom: reset (100%)
hotkeys-zoom-150 = Zoom: punch in 150%
hotkeys-zoom-200 = Zoom: punch in 2×
hotkeys-zoom-placeholder = Ctrl+Shift+2
hotkeys-split-split = Split timer: start / split
hotkeys-split-undo = Split timer: undo split
hotkeys-split-skip = Split timer: skip segment
hotkeys-split-reset = Split timer: reset
hotkeys-split-placeholder = e.g. Numpad1
hotkeys-playlist-next = Playlist: next item
hotkeys-playlist-previous = Playlist: previous item
hotkeys-playlist-placeholder = e.g. Ctrl+Alt+Right
hotkeys-replay-roll = Instant replay: roll
hotkeys-replay-roll-placeholder = e.g. Ctrl+Shift+I
sources-follow-title = Follow the cursor while zoomed (Windows; scroll the canvas to zoom)
sources-follow-item = Toggle cursor-follow for { $name }
filters-autocrop = ✂ Auto-crop black bars
filters-autocrop-title = Scan the next frame for letterbox/pillarbox bars and crop them (undoable). Dark scenes are never cropped.
filters-autocrop-follow = Re-check when the resolution changes
history-autoCrop = Auto-crop black bars
sources-link-audio = Also capture this app's audio (linked: hides mute it, removing the window removes it)
history-addLinkedWindow = Add window + linked audio
sources-hdr-title = This display is HDR — open the tone-map (the canvas stays SDR)
sources-hdr-item = HDR tone-map for { $name }
sources-hdr-dialog-title = HDR → SDR — { $name }
sources-hdr-hint = This display outputs HDR. Without a tone-map, highlights clip and the capture looks washed out on the SDR canvas. Changes apply on the next frame.
sources-hdr-enable-suggested = Enable suggested (maxRGB, 200 nits)
sources-hdr-operator = Operator
sources-hdr-op-clip = Clip (off)
sources-hdr-op-maxrgb = maxRGB (hue-preserving)
sources-hdr-op-reinhard = Reinhard
sources-hdr-op-bt2408 = BT.2408 knee (SDR exact)
sources-hdr-paper-white = Paper white
sources-hdr-nits = nits
projector-target-passthrough = Passthrough monitor (low latency)
projector-which-device = Device
projector-passthrough-none = Add a display, window, or capture device first.
projector-passthrough-about = Raw device frames — no scenes, no filters, no compositor. Shows a measured capture→display latency; audio still monitors through the mixer strip.
projector-passthrough-hint = Passthrough — Esc closes
projector-latency = { $ms } ms
projector-latency-measuring = measuring…
controls-automation = Automation
controls-automation-title = Rules, macros & studio variables (CAP-N01/N02)
automation-title = Automation — rules, macros & variables
automation-about = Rules run studio actions when something happens. Actions come from one fixed list — the same commands the remote API exposes — so a rule can never name a file, run a program, or reach the network. Every rule ships off; nothing runs until you enable it.
automation-rules = Rules
automation-add-rule = + Rule
automation-no-rules = No rules yet.
automation-new-rule = New rule
automation-enabled = On
automation-rule-name = Rule name
automation-remove = Remove
automation-when = When
automation-then-run = then run
automation-no-macro = (no macro)
automation-macros = Macros
automation-add-macro = + Macro
automation-no-macros = No macros yet.
automation-new-macro = New macro
automation-macro-name = Macro name
automation-hotkey = Hotkey
automation-hotkey-placeholder = Ctrl+Shift+M
automation-run = Run
automation-add-action = + Action…
automation-add-wait = + Wait
automation-repeat = Repeat
automation-variables = Studio variables
automation-variables-about = Use {"{{"}name{"}}"} in any Text source — it updates live when a macro sets the value.
automation-var-name = name
automation-var-value = value
automation-set-var = Set
automation-trigger-scene = scene switches to
automation-trigger-stream = streaming
automation-trigger-recording = recording
automation-trigger-source-error = a source errors
automation-trigger-audio = audio level
automation-trigger-idle = system idle for
automation-trigger-focus = window focus is
automation-trigger-time = time of day
automation-trigger-file = file changes
automation-scene-name = scene name
automation-starts = starts
automation-stops = stops
automation-any-source = (any source)
automation-source-name = source name
automation-rises-above = rises above
automation-falls-below = falls below
automation-threshold = threshold in dB
automation-idle-seconds = idle seconds
automation-seconds-windows = s (Windows only)
automation-exe = executable name
automation-windows-only = (Windows only)
automation-time = time (HH:MM)
automation-file = file path
controls-rundown = Rundown
controls-rundown-title = The show rundown: a timed scene playlist (CAP-N09)
rundown-title = Show rundown
rundown-about = An ordered list of steps — a scene and how long it holds. Advance by hand, or let it advance itself. Running the rundown switches scenes the ordinary way (undoable); it never edits your scenes.
rundown-start = Start
rundown-next = Next ▸
rundown-stop = Stop
rundown-idle = Not running
rundown-next-up = Next up: { $name }
rundown-last-step = Last step
rundown-auto-advance = Advance automatically when a step's time runs out
rundown-empty = No steps yet.
rundown-add-step = + Step
rundown-new-step = New step
rundown-step-name = Step name
rundown-step-scene = Scene
rundown-stay = (stay on this scene)
rundown-hold = Hold seconds
rundown-seconds = s
rundown-jump = Jump to this step
rundown-move-up = Move up
rundown-move-down = Move down
rundown-remove = Remove step
automation-layer = Layer
automation-layer-hint = Only fires while this hotkey layer is active (blank = every layer). Layers are sticky — a layer key switches layers and stays there (a true hold-to-shift layer isn't available through the OS global-hotkey API).
automation-chord-hint = A plain key (Ctrl+Shift+M) or a two-stroke chord (Ctrl+K, 3). A chord's second key is only claimed while the chord is pending.
controls-panel = LAN panel
controls-panel-title = The LAN touch panel + tally lights (CAP-N06/N07)
panel-title = LAN panel & tally
panel-about = Serve a control page and a full-screen tally page to phones on your own network. Off by default; a password is required; it binds to this machine only unless you enable LAN. The page is built into the app — nothing is fetched from the internet, and it accepts only the same commands the app's own buttons do.
panel-enable = Serve the panel
panel-port = Port
panel-lan = Allow other devices on my network (otherwise this machine only)
panel-password = Password
panel-show = Show
panel-hide = Hide
panel-qr-alt = QR code for the control panel
panel-tally-hint = Tally page (open it on a spare phone; add ?scene=NAME to watch one scene):
panel-off-hint = Enable the panel and set a password to get a link and QR code.
panel-save = Save
link-title = Freally Link output
link-about = Share this instance's program — video and master audio — with ONE other Freally Capture on your own network; it appears there as a "Freally Link" source (two-PC streaming, overflow monitors). Off by default; nothing announces or listens until enabled. v1 streams motion-JPEG + uncompressed audio over TCP — built for wired LAN or good Wi-Fi, never the internet.
link-enable = Share the program on my network
link-name = Instance name
link-key = Pairing key
link-key-hint = At least 8 characters — receivers must enter this key before a single frame is served.
link-lan-warning = ⚠ Receivers must present the pairing key before anything is served, but the stream itself is not encrypted in v1 — use it only on a network you trust.
link-serving = Receivers can find this instance with "Scan the LAN", or add it manually at:
link-off-hint = Enable sharing to open the port and announce this instance to LAN scans.
osc-title = OSC control surface
osc-about = TouchOSC-class controllers and lighting desks. Off by default; LAN-only, never the internet; it accepts only the same commands the app's own buttons do.
osc-enable = Listen for OSC
osc-addresses = /scene/switch "Live" · /transition · /record/start · /stream/start · /replay/save · /marker/add · /macro/run "Intro" · /mixer/vol "Mic" -6.0 · /mixer/mute "Mic" 1
controls-ptz = PTZ
controls-ptz-title = PTZ camera control — VISCA over IP (CAP-N08)
ptz-title = PTZ cameras
ptz-about = Pan, tilt and zoom cameras that speak VISCA over IP. LAN-only: the app talks to a camera only because you typed its address here — nothing is discovered. Hold a pad button to drive the head; releasing stops it.
ptz-camera = Camera
ptz-none = (no cameras yet)
ptz-add-camera = + Camera
ptz-remove-camera = Remove camera
ptz-new-camera = New camera
ptz-camera-name = Camera name
ptz-host = Address
ptz-port = Port
ptz-speed = Speed
ptz-zoom = Zoom
ptz-zoom-in = Zoom in
ptz-zoom-out = Zoom out
ptz-move-up = Tilt up
ptz-move-down = Tilt down
ptz-move-left = Pan left
ptz-move-right = Pan right
ptz-move-upLeft = Up and left
ptz-move-upRight = Up and right
ptz-move-downLeft = Down and left
ptz-move-downRight = Down and right
ptz-move-stop = Stop
ptz-presets = Presets
ptz-preset-name = Preset name
ptz-slot = Slot
ptz-recall = Recall
ptz-store = Store
ptz-add-preset = + Preset
ptz-new-preset = New preset
ptz-remove-preset = Remove preset
ptz-scene-recalls = Per-scene recall
ptz-scene-recalls-about = When a scene goes on program, recall the preset bound to it.
ptz-scene = Scene
ptz-add-recall = + Scene recall
ptz-remove-recall = Remove scene recall
controls-midi = MIDI
controls-midi-title = MIDI control surfaces — learn pads and faders (CAP-N03)
midi-title = MIDI control surface
midi-about = Bind a pad, knob, or fader to a studio action. Press Learn, then touch the control. LED and motor-fader feedback mirrors what the studio is actually doing. No MIDI port is opened until you pick one, and a binding can only name a command the app's own buttons already use.
midi-input = Input
midi-output = Output (feedback)
midi-none = (none)
midi-learn = Learn
midi-learning = Touch a control…
midi-empty = No bindings yet — pick an input, press Learn, and touch a pad.
midi-target = Does
midi-target-action = Action
midi-target-macro = Macro
midi-target-scene = Switch scene
midi-target-volume = Fader
midi-target-mute = Mute
midi-command = Command
midi-macro = Macro
midi-scene = Scene
midi-source = source name
midi-feedback = LED
midi-remove = Remove binding
panel-lan-warning = ⚠ LAN traffic is not encrypted — the password travels in the URL over plain HTTP. Use this only on a network you trust.
osc-lan-warning = ⚠ OSC has no password — any device on your network can send these commands. Use LAN mode only on a network you trust.
