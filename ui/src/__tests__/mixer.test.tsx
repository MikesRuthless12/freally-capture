import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { act } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import type { AudioLevelsPayload, Collection, StudioDto } from "../api/types";

type PushHandler = (event: { payload: unknown }) => void;
const listeners = new Map<string, PushHandler>();
const invokeCalls: Array<{ cmd: string; args: Record<string, unknown> }> = [];

/** A scene with a mic (with one denoise filter) + desktop audio + a webcam. */
function fixtureCollection(): Collection {
  const item = (id: string, source: string) => ({
    id,
    source,
    visible: true,
    locked: false,
    blend: "normal" as const,
    transform: {
      x: 960,
      y: 540,
      scaleX: 1,
      scaleY: 1,
      rotation: 0,
      crop: { left: 0, top: 0, right: 0, bottom: 0 },
    },
    pendingFit: false,
    filters: [],
  });
  return {
    formatVersion: 1,
    canvasWidth: 1920,
    canvasHeight: 1080,
    sources: [
      {
        id: "src-mic",
        name: "USB Mic",
        kind: "audioInput",
        deviceId: "usb-mic",
        audio: {
          volumeDb: 0,
          muted: false,
          monitor: "off",
          tracks: 0b1,
          syncOffsetMs: 0,
          pushToTalk: null,
          pushToMute: null,
          filters: [{ id: "af-denoise", enabled: true, type: "denoise", strength: 0.5 }],
        },
      },
      {
        id: "src-desktop",
        name: "Desktop Audio",
        kind: "audioOutput",
        deviceId: "",
        audio: {
          volumeDb: -6,
          muted: true,
          monitor: "off",
          tracks: 0b11,
          syncOffsetMs: 0,
          filters: [],
        },
      },
      { id: "src-cam", name: "Face cam", kind: "videoDevice", deviceId: "cam-0", format: null },
    ],
    scenes: [
      {
        id: "scene-a",
        name: "Main",
        items: [
          item("item-mic", "src-mic"),
          item("item-desktop", "src-desktop"),
          item("item-cam", "src-cam"),
        ],
      },
    ],
    activeScene: "scene-a",
  };
}

const mockState = vi.hoisted(() => ({
  dto: null as unknown as StudioDto,
}));

vi.mock("@tauri-apps/api/core", () => ({
  convertFileSrc: (path: string, protocol: string) => `${protocol}://localhost/${path}`,
  invoke: (cmd: string, args?: Record<string, unknown>) => {
    invokeCalls.push({ cmd, args: args ?? {} });
    switch (cmd) {
      case "eula_status":
        return Promise.resolve({ version: "test", text: "", accepted: true });
      case "health":
        return Promise.resolve({
          appVersion: "9.9.9",
          os: "windows",
          coreOk: true,
          crates: [],
        });
      case "settings_get":
        return Promise.resolve({ language: "en", showStatsDock: false, monitorDevice: null });
      case "studio_get":
        return Promise.resolve(mockState.dto);
      case "audio_input_devices":
        return Promise.resolve([{ id: "usb-mic", name: "USB Mic", isDefault: true }]);
      case "audio_output_devices":
        return Promise.resolve([{ id: "spk", name: "Speakers", isDefault: true }]);
      case "audio_loopback_devices":
        return Promise.resolve({
          devices: [{ id: "spk", name: "Speakers", isDefault: true }],
        });
      case "studio_add_audio_filter":
        return Promise.resolve("af-new");
      default:
        return Promise.resolve(null);
    }
  },
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: (name: string, handler: PushHandler) => {
    listeners.set(name, handler);
    return Promise.resolve(() => listeners.delete(name));
  },
}));

import App from "../App";

function pushAudio(payload: Partial<AudioLevelsPayload>) {
  act(() => {
    listeners.get("audio")!({
      payload: {
        sources: {},
        master: { peak: [0, 0], rms: [0, 0] },
        lufs: {},
        dropped: 0,
        ...payload,
      },
    });
  });
}

async function renderApp() {
  mockState.dto = { revision: 1, collection: fixtureCollection() };
  render(<App />);
  // The name shows in both the Sources rail and the mixer strip.
  await waitFor(() => expect(screen.getAllByText("USB Mic").length).toBeGreaterThan(0));
}

beforeEach(() => {
  listeners.clear();
  invokeCalls.length = 0;
});

describe("the audio mixer", () => {
  it("renders a strip per audio source (video sources stay out)", async () => {
    await renderApp();
    const mixer = screen.getByRole("region", { name: "Audio Mixer" });
    expect(mixer).toHaveTextContent("USB Mic");
    expect(mixer).toHaveTextContent("Desktop Audio");
    expect(mixer).not.toHaveTextContent("Face cam");
    // The muted desktop strip shows its fader dB and mute state.
    expect(screen.getByRole("button", { name: "Unmute Desktop Audio" })).toBeInTheDocument();
  });

  it("shows live levels, LUFS, and per-source status from the audio event", async () => {
    await renderApp();
    pushAudio({
      sources: {
        "src-mic": {
          state: "live",
          peak: [0.5, 0.5],
          rms: [0.25, 0.25],
          gated: false,
        },
        "src-desktop": {
          state: "error",
          errorCode: "deviceNotFound",
          errorMessage: "audio device not found: X",
          peak: [0, 0],
          rms: [0, 0],
          gated: true,
        },
      },
      lufs: { momentary: -18.4, shortTerm: -19.1 },
    });
    expect(screen.getByText("-18.4")).toBeInTheDocument();
    expect(screen.getByText(/S -19\.1/)).toBeInTheDocument();
    expect(screen.getByText("audio device not found: X")).toBeInTheDocument();
    const meters = screen.getAllByRole("meter", { name: "Level" });
    expect(meters.length).toBeGreaterThan(0);
  });

  it("mute, fader, and track dots drive the audio commands", async () => {
    await renderApp();

    fireEvent.click(screen.getByRole("button", { name: "Mute USB Mic" }));
    await waitFor(() =>
      expect(invokeCalls).toContainEqual({
        cmd: "studio_set_audio_muted",
        args: { sourceId: "src-mic", muted: true },
      }),
    );

    fireEvent.change(screen.getByRole("slider", { name: "Volume of USB Mic in decibels" }), {
      target: { value: "-12" },
    });
    await waitFor(() =>
      expect(invokeCalls).toContainEqual({
        cmd: "studio_set_audio_volume",
        args: { sourceId: "src-mic", volumeDb: -12 },
      }),
    );

    // Track 2 on the mic: bit 1 joins bit 0.
    fireEvent.click(screen.getByRole("button", { name: "Track 2 for USB Mic" }));
    await waitFor(() =>
      expect(invokeCalls).toContainEqual({
        cmd: "studio_set_audio_tracks",
        args: { sourceId: "src-mic", tracks: 0b11 },
      }),
    );

    // The monitor cycle: off → monitorOnly.
    fireEvent.click(screen.getByRole("button", { name: /Monitor mode of USB Mic/ }));
    await waitFor(() =>
      expect(invokeCalls).toContainEqual({
        cmd: "studio_set_audio_monitor",
        args: { sourceId: "src-mic", monitor: "monitorOnly" },
      }),
    );
  });

  it("opens the audio filters dialog and adds a filter", async () => {
    await renderApp();
    fireEvent.click(screen.getByRole("button", { name: "Audio filters for USB Mic" }));
    expect(
      await screen.findByRole("dialog", { name: "Audio filters — USB Mic" }),
    ).toBeInTheDocument();
    // The existing denoise filter renders with its strength slider.
    expect(screen.getByRole("checkbox", { name: "Enable Denoise" })).toBeChecked();

    fireEvent.click(screen.getByRole("button", { name: "+ Add filter" }));
    fireEvent.click(screen.getByRole("menuitem", { name: "Compressor" }));
    await waitFor(() =>
      expect(
        invokeCalls.some(
          (call) =>
            call.cmd === "studio_add_audio_filter" &&
            call.args.sourceId === "src-mic" &&
            (call.args.kind as { type: string }).type === "compressor",
        ),
      ).toBe(true),
    );
  });

  it("lists audio sources in the add-source menu with device pickers", async () => {
    await renderApp();
    fireEvent.click(screen.getByRole("button", { name: "Add a source" }));
    fireEvent.click(screen.getByRole("menuitem", { name: "Audio Input Capture" }));
    expect(
      await screen.findByRole("dialog", { name: "Add an Audio Input Capture" }),
    ).toBeInTheDocument();
    // The default entry + the enumerated mic.
    expect(screen.getByRole("button", { name: "Default input" })).toBeInTheDocument();
    expect(await screen.findByRole("button", { name: "USB Mic" })).toBeInTheDocument();
  });

  it("audio source rows in the rail show status from the audio event", async () => {
    await renderApp();
    pushAudio({
      sources: {
        "src-mic": { state: "live", peak: [0.1, 0.1], rms: [0.05, 0.05], gated: false },
      },
    });
    const sourcesRail = screen.getByRole("region", { name: "Sources" });
    const liveDots = sourcesRail.querySelectorAll('[aria-label="status: live"]');
    expect(liveDots.length).toBe(1);
  });
});
