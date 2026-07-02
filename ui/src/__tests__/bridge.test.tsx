import { render, screen, waitFor } from "@testing-library/react";
import { act } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";

type PushHandler = (event: { payload: unknown }) => void;
const listeners = new Map<string, PushHandler>();
const invokeCalls: Array<{ cmd: string; args: unknown }> = [];

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (cmd: string, args?: unknown) => {
    invokeCalls.push({ cmd, args });
    switch (cmd) {
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

describe("UI ↔ core bridge", () => {
  beforeEach(() => {
    listeners.clear();
    invokeCalls.length = 0;
  });

  it("renders the health report from the core", async () => {
    render(<App />);
    await waitFor(() => expect(screen.getByText(/v9\.9\.9 · core OK/)).toBeInTheDocument());
  });

  it("renders stats pushed from the core", async () => {
    render(<App />);
    await waitFor(() => expect(listeners.has("stats")).toBe(true));

    act(() => {
      listeners.get("stats")!({ payload: { fps: 60.4, cpu: 5.23, placeholder: true } });
    });

    await waitFor(() => {
      expect(screen.getByText("60")).toBeInTheDocument();
      expect(screen.getByText("5.2%")).toBeInTheDocument();
    });
    expect(screen.getByText(/placeholder data/i)).toBeInTheDocument();
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
});
