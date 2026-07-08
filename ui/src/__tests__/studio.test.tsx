import { render, screen, waitFor } from "@testing-library/react";
import { act } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import type { Collection, ProgramStatus, StudioDto } from "../api/types";

type PushHandler = (event: { payload: unknown }) => void;
const listeners = new Map<string, PushHandler>();
const invokeCalls: Array<{ cmd: string; args: Record<string, unknown> }> = [];

function fixtureCollection(): Collection {
  return {
    formatVersion: 1,
    canvasWidth: 1920,
    canvasHeight: 1080,
    sources: [
      { id: "src-cam", name: "Face cam", kind: "videoDevice", deviceId: "cam-0", format: null },
      {
        id: "src-img",
        name: "Overlay",
        kind: "image",
        path: "C:/art/overlay.png",
      },
    ],
    scenes: [
      {
        id: "scene-a",
        name: "Main",
        items: [
          {
            id: "item-cam",
            source: "src-cam",
            visible: true,
            locked: false,
            blend: "normal",
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
          },
        ],
      },
      { id: "scene-b", name: "Break", items: [] },
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
        return Promise.resolve({ language: "en", showStatsDock: false });
      case "studio_get":
        return Promise.resolve(mockState.dto);
      case "capture_list_sources":
        return Promise.resolve([
          {
            id: "dxgi:0",
            kind: "display",
            label: "Display 1 — 2560×1440",
            width: 2560,
            height: 1440,
          },
        ]);
      case "video_devices_list":
        return Promise.resolve([{ id: "cam-0", name: "Integrated Camera" }]);
      case "video_device_formats":
        return Promise.resolve([]);
      case "studio_add_item":
        return Promise.resolve({ sourceId: "src-new", itemId: "item-new" });
      case "studio_add_filter":
        return Promise.resolve("filter-new");
      default:
        // Mutations resolve; the real backend echoes a studio event.
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

function pushProgram(status: Partial<ProgramStatus>) {
  act(() => {
    listeners.get("program")!({
      payload: {
        state: "running",
        width: 1920,
        height: 1080,
        fps: 60,
        renderMicros: 500,
        adapter: "Test GPU (Vulkan, DiscreteGpu)",
        dropped: 0,
        sources: {},
        ...status,
      },
    });
  });
}

describe("the studio UI", () => {
  beforeEach(() => {
    listeners.clear();
    invokeCalls.length = 0;
    mockState.dto = { revision: 1, collection: fixtureCollection() };
  });

  it("renders scenes and the active scene's items from studio_get", async () => {
    render(<App />);
    expect(await screen.findByRole("button", { name: /^Main/ })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /^Break/ })).toBeInTheDocument();
    expect(screen.getByText("Face cam")).toBeInTheDocument();
    // Scene B's items are not shown while A is active.
    expect(screen.queryByText("Overlay")).not.toBeInTheDocument();
  });

  it("adds and selects scenes through the studio commands", async () => {
    render(<App />);
    const addScene = await screen.findByRole("button", { name: "Add a scene" });
    act(() => addScene.click());
    await waitFor(() =>
      expect(invokeCalls.some((call) => call.cmd === "studio_add_scene")).toBe(true),
    );

    const sceneB = screen.getByRole("button", { name: /^Break/ });
    act(() => sceneB.click());
    await waitFor(() => {
      const call = invokeCalls.find((c) => c.cmd === "studio_select_scene");
      expect(call?.args).toMatchObject({ sceneId: "scene-b" });
    });
  });

  it("adds a display capture through the picker", async () => {
    render(<App />);
    const addSource = await screen.findByRole("button", { name: "Add a source" });
    act(() => addSource.click());
    act(() => screen.getByRole("menuitem", { name: "Display Capture" }).click());

    const entry = await screen.findByRole("button", { name: /Display 1/ });
    act(() => entry.click());

    await waitFor(() => {
      const call = invokeCalls.find((c) => c.cmd === "studio_add_item");
      expect(call).toBeDefined();
      expect(call!.args).toMatchObject({
        sceneId: "scene-a",
        settings: { kind: "display", captureId: "dxgi:0" },
      });
    });
  });

  it("toggles item visibility", async () => {
    render(<App />);
    const eye = await screen.findByRole("button", { name: /hide face cam/i });
    act(() => eye.click());
    await waitFor(() => {
      const call = invokeCalls.find((c) => c.cmd === "studio_set_item_visible");
      expect(call?.args).toMatchObject({ itemId: "item-cam", visible: false });
    });
  });

  it("applies studio events pushed from the core", async () => {
    render(<App />);
    await screen.findByRole("button", { name: /^Main/ });
    const next = { revision: 2, collection: fixtureCollection() };
    next.collection.scenes[0].name = "Renamed live";
    act(() => {
      listeners.get("studio")!({ payload: next });
    });
    expect(await screen.findByRole("button", { name: /^Renamed live/ })).toBeInTheDocument();
  });

  it("shows per-source status and the program footer", async () => {
    render(<App />);
    await screen.findByText("Face cam");
    pushProgram({
      fps: 59,
      sources: { "src-cam": { state: "live", width: 1280, height: 720, fps: 30 } },
    });
    expect(await screen.findByLabelText("status: live")).toBeInTheDocument();
    expect(screen.getByText("59 fps")).toBeInTheDocument();
    expect(screen.getByText("1920×1080")).toBeInTheDocument();
  });

  it("offers retry on an errored source", async () => {
    render(<App />);
    await screen.findByText("Face cam");
    pushProgram({
      sources: {
        "src-cam": { state: "error", errorCode: "backend", errorMessage: "device busy" },
      },
    });
    const retry = await screen.findByRole("button", { name: /retry face cam/i });
    act(() => retry.click());
    await waitFor(() => {
      const call = invokeCalls.find((c) => c.cmd === "studio_retry_source");
      expect(call?.args).toMatchObject({ sourceId: "src-cam" });
    });
  });

  it("reports a GPU-less machine honestly", async () => {
    render(<App />);
    await screen.findByText("Face cam");
    pushProgram({ state: "noGpu", adapter: "no usable GPU adapter was found" });
    expect(await screen.findByRole("alert")).toHaveTextContent(/no usable gpu/i);
  });

  it("opens the filters dialog and adds a filter", async () => {
    render(<App />);
    await screen.findByText("Face cam");
    const filters = screen.getByRole("button", { name: /filters for face cam/i });
    act(() => filters.click());

    expect(await screen.findByRole("dialog", { name: /filters — face cam/i })).toBeInTheDocument();
    act(() => screen.getByRole("button", { name: "+ Add filter" }).click());
    act(() => screen.getByRole("menuitem", { name: "Chroma Key" }).click());

    await waitFor(() => {
      const call = invokeCalls.find((c) => c.cmd === "studio_add_filter");
      expect(call?.args).toMatchObject({
        itemId: "item-cam",
        kind: { type: "chromaKey" },
      });
    });
  });
});
