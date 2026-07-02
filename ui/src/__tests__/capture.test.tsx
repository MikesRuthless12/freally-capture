import { render, screen, waitFor } from "@testing-library/react";
import { act } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";

type PushHandler = (event: { payload: unknown }) => void;
const listeners = new Map<string, PushHandler>();
const invokeCalls: Array<{ cmd: string; args: unknown }> = [];
const mockState = vi.hoisted(() => ({
  os: "windows",
  captureSources: [] as unknown[],
}));

vi.mock("@tauri-apps/api/core", () => ({
  convertFileSrc: (path: string, protocol: string) => `${protocol}://localhost/${path}`,
  invoke: (cmd: string, args?: unknown) => {
    invokeCalls.push({ cmd, args });
    switch (cmd) {
      case "health":
        return Promise.resolve({
          appVersion: "9.9.9",
          os: mockState.os,
          coreOk: true,
          crates: [],
        });
      case "settings_get":
        return Promise.resolve({ language: "en", showStatsDock: true });
      case "capture_list_sources":
        return Promise.resolve(mockState.captureSources);
      case "video_devices_list":
        return Promise.resolve([{ id: "0", name: "Test Cam" }]);
      case "video_device_formats":
        return Promise.resolve([{ width: 1920, height: 1080, fps: 30, fourcc: "MJPEG" }]);
      case "preview_start":
      case "preview_stop":
      case "open_privacy_settings":
        return Promise.resolve(null);
      default:
        return Promise.reject(new Error(`unexpected command ${cmd}`));
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

const DISPLAY = {
  id: "display:\\\\.\\DISPLAY1",
  kind: "display",
  label: "Display 1 — 2560×1440 (primary)",
  width: 2560,
  height: 1440,
};
const PORTAL = {
  id: "portal",
  kind: "portal",
  label: "Screen or window — the system picker chooses",
  width: 0,
  height: 0,
};

async function addDisplaySource() {
  act(() => {
    screen.getByRole("button", { name: /add a source/i }).click();
  });
  act(() => {
    screen.getByRole("menuitem", { name: "Display Capture" }).click();
  });
  const entry = await screen.findByRole("button", { name: /Display 1/ });
  act(() => {
    entry.click();
  });
}

describe("capture sources + preview", () => {
  beforeEach(() => {
    listeners.clear();
    invokeCalls.length = 0;
    mockState.os = "windows";
    mockState.captureSources = [DISPLAY];
  });

  it("adds a display source and starts its preview", async () => {
    render(<App />);
    await addDisplaySource();

    const start = invokeCalls.find((c) => c.cmd === "preview_start");
    expect(start).toBeDefined();
    const args = start!.args as { source: { kind: string; id: string }; sourceKey: string };
    expect(args.source.kind).toBe("display");
    expect(args.source.id).toBe(DISPLAY.id);
    expect(args.sourceKey).toBeTruthy();
    // The rail shows the added card (badge + label form its accessible name).
    expect(screen.getByRole("button", { name: /^Display Display 1/ })).toBeInTheDocument();
  });

  it("shows the live overlay when the preview goes live", async () => {
    render(<App />);
    await addDisplaySource();
    await waitFor(() => expect(listeners.has("preview")).toBe(true));

    const start = invokeCalls.find((c) => c.cmd === "preview_start")!.args as {
      sourceKey: string;
    };
    act(() => {
      listeners.get("preview")!({
        payload: {
          state: "live",
          sourceKey: start.sourceKey,
          label: DISPLAY.label,
          width: 2560,
          height: 1440,
          fps: 59,
        },
      });
    });

    expect(await screen.findByRole("img", { name: /live preview/i })).toBeInTheDocument();
    expect(screen.getByText("2560×1440")).toBeInTheDocument();
    expect(screen.getByText("59 fps")).toBeInTheDocument();
  });

  it("offers the macOS permission deep-link on a permission error", async () => {
    mockState.os = "macos";
    render(<App />);
    await addDisplaySource();
    await waitFor(() => expect(listeners.has("preview")).toBe(true));

    act(() => {
      listeners.get("preview")!({
        payload: {
          state: "error",
          errorCode: "permission",
          errorMessage: "screen capture permission was denied",
        },
      });
    });

    expect(await screen.findByRole("alert")).toHaveTextContent(/permission was denied/i);
    const fix = screen.getByRole("button", { name: /open screen recording settings/i });
    act(() => {
      fix.click();
    });
    await waitFor(() => {
      const call = invokeCalls.find((c) => c.cmd === "open_privacy_settings");
      expect(call?.args).toEqual({ pane: "screenRecording" });
    });
  });

  it("stops the preview when the active source is removed", async () => {
    render(<App />);
    await addDisplaySource();

    act(() => {
      screen.getByRole("button", { name: /remove display 1/i }).click();
    });
    await waitFor(() => {
      expect(invokeCalls.some((c) => c.cmd === "preview_stop")).toBe(true);
    });
  });

  it("is honest about Wayland portal-only capture", async () => {
    mockState.captureSources = [PORTAL];
    render(<App />);
    act(() => {
      screen.getByRole("button", { name: /add a source/i }).click();
    });
    act(() => {
      screen.getByRole("menuitem", { name: "Display Capture" }).click();
    });

    expect(await screen.findByRole("button", { name: /system picker/i })).toBeInTheDocument();
    expect(screen.getByText(/can’t capture globally/i)).toBeInTheDocument();
  });
});
