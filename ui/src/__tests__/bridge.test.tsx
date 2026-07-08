import { render, screen, waitFor } from "@testing-library/react";
import { act } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";

type PushHandler = (event: { payload: unknown }) => void;
const listeners = new Map<string, PushHandler>();
const invokeCalls: Array<{ cmd: string; args: unknown }> = [];
const mockState = vi.hoisted(() => ({ failSettingsSet: false }));

vi.mock("@tauri-apps/api/core", () => ({
  convertFileSrc: (path: string, protocol: string) => `${protocol}://localhost/${path}`,
  invoke: (cmd: string, args?: unknown) => {
    invokeCalls.push({ cmd, args });
    switch (cmd) {
      case "eula_status":
        return Promise.resolve({ version: "test", text: "", accepted: true });
      case "health":
        return Promise.resolve({
          appVersion: "9.9.9",
          os: "windows",
          coreOk: true,
          crates: [{ name: "fcap-scene", version: "9.9.9" }],
        });
      case "settings_get":
        return Promise.resolve({ language: "en", showStatsDock: true });
      case "settings_set":
        return mockState.failSettingsSet
          ? Promise.reject(new Error("disk full"))
          : Promise.resolve(null);
      case "studio_get":
        return Promise.resolve({
          revision: 1,
          collection: {
            formatVersion: 1,
            canvasWidth: 1920,
            canvasHeight: 1080,
            sources: [],
            scenes: [{ id: "scene-1", name: "Scene", items: [] }],
            activeScene: "scene-1",
          },
        });
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

describe("UI ↔ core bridge", () => {
  beforeEach(() => {
    listeners.clear();
    invokeCalls.length = 0;
    mockState.failSettingsSet = false;
  });

  it("renders the health report from the core", async () => {
    render(<App />);
    await waitFor(() => expect(screen.getByText(/v9\.9\.9 · core OK/)).toBeInTheDocument());
  });

  it("renders stats pushed from the core", async () => {
    render(<App />);
    await waitFor(() => expect(listeners.has("stats")).toBe(true));

    act(() => {
      listeners.get("stats")!({
        payload: {
          fps: 60.4,
          cpu: 5.23,
          memoryMb: 412.7,
          dropped: 3,
          renderMs: 1.84,
          placeholder: false,
        },
      });
    });

    await waitFor(() => {
      expect(screen.getByText("60")).toBeInTheDocument();
      expect(screen.getByText("5.2%")).toBeInTheDocument();
      expect(screen.getByText("413 MB")).toBeInTheDocument();
      expect(screen.getByText("3")).toBeInTheDocument();
      expect(screen.getByText("1.8 ms")).toBeInTheDocument();
    });
    // Real data (placeholder: false) → no startup hint inside the dock.
    const dock = screen.getByRole("region", { name: "Stats" });
    expect(dock).not.toHaveTextContent(/starting the compositor/i);
  });

  it("persists the stats-dock toggle through settings_set", async () => {
    render(<App />);
    const toggle = await screen.findByRole("button", { name: /stats on/i });

    act(() => {
      toggle.click();
    });

    await waitFor(() => {
      const call = invokeCalls.find((c) => c.cmd === "settings_set");
      expect(call).toBeDefined();
      expect(call!.args).toEqual({ settings: { language: "en", showStatsDock: false } });
    });
    expect(screen.queryByRole("region", { name: "Stats" })).not.toBeInTheDocument();
  });

  it("rolls the toggle back and surfaces an error when persisting fails", async () => {
    mockState.failSettingsSet = true;
    render(<App />);
    const toggle = await screen.findByRole("button", { name: /stats on/i });

    act(() => {
      toggle.click();
    });

    await waitFor(() => {
      expect(screen.getByRole("alert")).toHaveTextContent(/couldn't save settings/i);
    });
    // Rolled back: the dock is still visible and the toggle still reads "on".
    expect(screen.getByRole("region", { name: "Stats" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /stats on/i })).toBeInTheDocument();
  });
});
