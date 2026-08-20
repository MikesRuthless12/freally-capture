// A minimal Tauri v2 IPC mock so the REAL built UI renders in a plain browser
// (Playwright) for the visual-smoke gallery. It shims `window.__TAURI_INTERNALS__`
// with an `invoke` that returns canned, valid data per command, and a no-op
// event system so `listen()` resolves. This is UI-render coverage — it does not
// exercise the Rust backend (that's the per-OS `cargo test` suite).
//
// Runs via Playwright addInitScript (before the app bundle loads).
(() => {
  const params = new URLSearchParams(location.search);
  const eulaAccepted = params.get("eula") !== "0";

  const EULA_TEXT =
    "# Freally Capture — End User License Agreement (EULA)\n\n" +
    "By installing or using the Software you agree to this Agreement. You are " +
    "solely responsible for your content, your streams, and how you use the " +
    "Software, including having all necessary rights and complying with all laws " +
    'and platform terms. THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ' +
    "ANY KIND. (Mocked text for the UI gallery.)\n".repeat(6);

  const scene = {
    id: "scene-1",
    name: "Main Scene",
    items: [
      { id: "item-1", source: "src-cam", visible: true, locked: false },
      { id: "item-2", source: "src-mic", visible: true, locked: false },
    ],
  };
  const collection = {
    formatVersion: 1,
    canvasWidth: 1920,
    canvasHeight: 1080,
    activeScene: "scene-1",
    scenes: [scene, { id: "scene-2", name: "Starting Soon", items: [] }],
    sources: [
      { id: "src-cam", name: "Webcam", kind: "videoDevice", deviceId: "cam0" },
      { id: "src-mic", name: "Microphone", kind: "audioInput", deviceId: "" },
    ],
  };

  const settings = {
    language: "en",
    // The gallery photographs the studio, not the first-run wizard. A fresh
    // install shows the wizard over everything; this fixture is an existing user.
    completedOnboarding: true,
    theme: { mode: "dark", accent: "#4a9eff" },
    showStatsDock: true,
    monitorDevice: null,
    mixerLayout: "horizontal",
    // Required slices — Rust `settings_get` always returns these, so the
    // Settings modal reads them without guarding. Keep in sync with the
    // `Settings` type or the modal crashes on open.
    alignment: { smartGuides: true, safeAreas: false, rulers: false },
    accessibility: {
      meterPreset: "default",
      meterLow: "#26f26f",
      meterMid: "#f7f726",
      meterHigh: "#f72626",
    },
    panicSlate: { color: "#000000", image: "" },
    webPanel: { enabled: false, port: 4457, lan: false, password: "" },
    osc: { enabled: false, port: 9000, lan: false },
    link: { enabled: false, port: 9720, name: "", key: "" },
    // These slices are NOT optional in the `Settings` type — the panels read
    // them unconditionally (`rec.isoSources.length`, `stream.simulator.profile`,
    // …), so a partial fixture takes the whole app down rather than degrading.
    // Keep them complete when the type grows.
    featuredBanner: { bg: "#101014", text: "#f2f2f5" },
    recording: {
      container: "frec",
      audioFormat: "wav",
      encoderId: "x264",
      rateControl: "cbr",
      preset: "quality",
      keyframeSec: 2,
      fps: 60,
      audioBitrateKbps: 160,
      tracksMask: 1,
      folder: "C:/Users/demo/Videos",
      filenamePrefix: "freally",
      template: "{prefix} {date}",
      replayTemplate: "{prefix} replay {date}",
      stillTemplate: "{prefix} still {date}",
      replayFolder: "C:/Users/demo/Videos",
      stillFolder: "C:/Users/demo/Pictures",
      counter: 1,
      splitMinutes: 0,
      recordVertical: false,
      outputWidth: 0,
      outputHeight: 0,
      isoSources: [],
      isoPostFilter: false,
      isoContainer: "frec",
      isoEncoderId: "x264",
      alphaFrec: false,
      splitOnScene: false,
      splitOnMarker: false,
      splitOnRundown: false,
      autoMarkers: false,
      pipelineEnabled: false,
      pipeline: [],
      autoExportMp4: false,
    },
    remote: { turnUrl: "", turnUsername: "", turnCredential: "" },
    stream: {
      enabled: false,
      service: "youtube",
      targets: [],
      autoRecord: false,
      preflightHold: true,
      sessionReport: false,
      simulator: {
        profile: "off",
        bandwidthKbps: 0,
        latencyMs: 0,
        jitterMs: 0,
        outageEveryS: 0,
        outageLenS: 0,
      },
    },
    replay: { seconds: 60, bitrateKbps: 12000, audioBitrateKbps: 160, fps: 60, track: 1 },
    transition: {
      kind: "fade",
      durationMs: 300,
      lumaImage: "",
      stingerPath: "",
      stingerCutMs: 0,
      stingerMatte: "none",
      stingerDuckDb: 0,
    },
    hotkeys: {},
    remoteControl: { enabled: false, port: 4456, lan: false, password: "" },
    browserDocks: [{ name: "Twitch Chat", url: "https://twitch.tv/popout/x/chat" }],
    scripts: [{ path: "C:/scripts/go-live.lua", enabled: true }],
    acceptedEulaVersion: eulaAccepted ? "2026-07-08" : null,
  };

  const RESP = {
    health: {
      appVersion: "0.95.0",
      os: "windows",
      coreOk: true,
      crates: [
        "fcap-capture",
        "fcap-sources",
        "fcap-compositor",
        "fcap-scene",
        "fcap-audio",
        "fcap-appaudio",
        "fcap-ndi",
        "fcap-encode",
        "fcap-stream",
      ].map((name) => ({ name, version: "0.95.0" })),
    },
    eula_status: { version: "2026-07-08", text: EULA_TEXT, accepted: eulaAccepted },
    settings_get: settings,
    settings_set: null,
    settings_complete_onboarding: null,
    reveal_app_folder: null,
    // The menu bar lazy-loads these when the Profile / Scene Collection
    // menus open.
    profiles_list: { active: "Default", names: ["Default", "Streaming"] },
    collections_list: { active: "Main", names: ["Main", "Just Chatting"] },
    profile_switch: settings,
    collection_switch: { active: "Main", names: ["Main", "Just Chatting"] },
    studio_get: { revision: 1, collection },
    integrations_status: {
      ndiAvailable: false,
      ndiVersion: null,
      ndiGuidance:
        "NDI output/input needs the NDI runtime, which Freally Capture never bundles. Install the free NDI Tools from ndi.video.",
      vstAvailable: false,
      vstStatus:
        "VST2/3 plugins are not available: the VST2 SDK is no longer licensed by Steinberg and VST3 is GPLv3-or-proprietary. Use the built-in filters.",
    },
    ffmpeg_status: {
      state: "missing",
      build: { version: "8.1.2", source: "gyan.dev", sizeBytes: 106000000 },
    },
    cef_status: { state: "missing", supported: true },
    // CAP-N76: the gallery machine has no registered source DLL, which is the
    // honest common case — the control stays disabled and says why.
    virtual_camera_status: {
      available: false,
      reason:
        "the Freally Capture Virtual Camera source is not registered — install the virtual-camera component",
      running: false,
      error: null,
    },
    app_audio_apps: {
      apps: [
        { pid: 1234, name: "Spotify", exe: "Spotify.exe" },
        { pid: 5678, name: "Chrome", exe: "chrome.exe" },
        { pid: 9012, name: "Game", exe: "game.exe" },
      ],
      supported: true,
      guidance: "Per-app audio needs Windows 10 build 2004 (19041) or newer.",
    },
    game_capture_status: {
      support: "hookPlanned",
      hookPossible: true,
      risk: "Game Capture injects a hook into the game to copy its GPU frames. Anti-cheat can treat that as tampering — in competitive games this may get your account BANNED — and antivirus may flag it. Prefer Window Capture.",
      fallback: "windowCapture",
      guidance:
        "The injected GPU-API hook is a flagged, opt-in milestone. Today, run the game in borderless/windowed mode and add it as a Window Capture.",
    },
    audio_input_devices: [
      { id: "", name: "Default input", isDefault: true },
      { id: "mic-usb", name: "USB Microphone", isDefault: false },
    ],
    audio_output_devices: [
      { id: "", name: "Default output", isDefault: true },
      { id: "spk", name: "Speakers (Realtek)", isDefault: false },
    ],
    audio_loopback_devices: {
      devices: [{ id: "spk", name: "Speakers (Realtek)", isDefault: true }],
      guidance: null,
    },
    capture_list_sources: [
      { id: "disp-1", kind: "display", label: "Display 1 (1920×1080)", width: 1920, height: 1080 },
      { id: "win-1", kind: "window", label: "Notepad", width: 800, height: 600 },
    ],
    capture_window_thumbnail: null,
    video_devices_list: [{ id: "cam0", name: "Integrated Webcam" }],
    video_device_formats: [{ width: 1280, height: 720, fps: 30, fourcc: "MJPG" }],
    // Every command below returns a non-Option value in Rust (`Vec<..>`, a
    // struct, a tagged enum). Left unmocked they resolved to `null` here, and
    // the reading panel dereferenced it — six dialogs took the whole app down
    // (Recordings, Projectors, A/V Sync, Featured Chat, Profiles, Scene
    // Collections) and About silently dropped its version + links block.
    pipeline_status: [],
    audiorec_status: { state: "idle" },
    recordings_list: [
      {
        path: "C:/Users/demo/Videos/show.frec",
        name: "show.frec",
        sizeBytes: 1048576,
        modifiedMs: 0,
        ext: "frec",
        frecAlpha: false,
      },
    ],
    list_displays: [
      { index: 0, name: "Display 1", width: 1920, height: 1080, x: 0, y: 0, isPrimary: true },
      { index: 1, name: "Display 2", width: 2560, height: 1440, x: 1920, y: 0, isPrimary: false },
    ],
    chat_recent: [
      {
        id: "l1",
        platform: "twitch",
        author: "viewer_one",
        text: "hello from chat",
        timestampMs: 0,
      },
    ],
    dock_presets_list: [],
    collection_missing_files: [],
    hotkey_audit: [
      {
        accelerator: "Ctrl+Shift+R",
        action: "record",
        feature: "Recording",
        registered: true,
        sharedWith: 0,
        valid: true,
      },
    ],
    midi_ports: [[], []],
    quick_actions_get: { pages: [] },
    rundown_status: { running: false },
    automation_variables: {},
    release_notes: { version: "1.2.0", notes: "Deep Capture & Web." },
    teleprompter_get: {
      script: "Welcome to the show.",
      speed: 60,
      fontSize: 32,
      mirror: false,
      offset: 0,
      playing: false,
      caesuraSecs: 0,
      countdownSecs: 3,
      countdownRemaining: 0,
    },
    build_info: {
      version: "1.2.0",
      authors: "Mike Weaver",
      projectStarted: "2026-06-30",
      firstStableReleased: "2026-07-19",
      copyright: "© 2026 Havoc Software",
      homepage: "https://mikesruthless12.github.io/freally-capture",
      repository: "https://github.com/MikesRuthless12/freally-capture",
      issues: "https://github.com/MikesRuthless12/freally-capture/issues",
      os: "windows",
      arch: "x86_64",
      target: "x86_64-pc-windows-msvc",
    },
    bug_report_context: {
      appVersion: "0.95.0",
      os: "windows",
      arch: "x86_64",
      diagnostics: "Freally Capture 0.95.0 · windows x86_64",
      githubUrl: "https://github.com/MikesRuthless12/freally-capture/issues/new",
      email: "mythodikalone@gmail.com",
      pendingCrash: null,
    },
  };

  function respond(cmd) {
    if (cmd in RESP) return RESP[cmd];
    // Event plugin: let listen()/emit() resolve to a no-op.
    if (cmd.startsWith("plugin:event|")) return 0;
    return null; // unknown mutation → resolve null (the UI treats echoes via events)
  }

  let cbId = 0;
  window.__TAURI_INTERNALS__ = {
    invoke: (cmd) => Promise.resolve(respond(cmd)),
    transformCallback: (cb) => {
      const id = ++cbId;
      window[`_${id}`] = cb;
      return id;
    },
    convertFileSrc: (path, protocol) => `${protocol || "asset"}://localhost/${path}`,
    metadata: {
      currentWindow: { label: "main" },
      currentWebview: { windowLabel: "main", label: "main" },
    },
    plugins: {},
  };
})();
