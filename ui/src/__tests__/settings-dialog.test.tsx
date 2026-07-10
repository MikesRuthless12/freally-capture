import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { act } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import type { Settings, ThemeMode } from "../api/types";
import { SettingsDialog } from "../panels/Settings";

const mockState = vi.hoisted(() => ({
  failSettingsSet: false,
  /** What the store would return on a re-read — i.e. what actually stuck. */
  stored: null as Settings | null,
}));

const setCalls: Settings[] = [];
let getCalls = 0;

vi.mock("../api/commands", () => ({
  settingsSet: (next: Settings) => {
    setCalls.push(next);
    return mockState.failSettingsSet
      ? Promise.reject(new Error("disk full"))
      : Promise.resolve(null);
  },
  settingsGet: () => {
    getCalls += 1;
    return Promise.resolve(mockState.stored);
  },
}));

vi.mock("../lib/modal", () => ({ pushModal: () => () => undefined }));

/** `applyTheme` writes to `documentElement`; harmless in jsdom, but noisy. */
vi.mock("../theme/theme", async (importOriginal) => ({
  ...(await importOriginal<typeof import("../theme/theme")>()),
  applyTheme: vi.fn(),
}));

const settings = (mode: ThemeMode): Settings =>
  ({
    language: "en",
    showStatsDock: true,
    theme: { mode, accent: "#4a9eff" },
  }) as Settings;

function open(mode: ThemeMode, onSettingsSaved = vi.fn()) {
  render(
    <SettingsDialog
      settings={settings(mode)}
      onSettingsSaved={onSettingsSaved}
      onClose={() => undefined}
      onOpen={() => undefined}
    />,
  );
  return onSettingsSaved;
}

const accent = () => screen.getByLabelText("Accent") as HTMLInputElement;

describe("SettingsDialog", () => {
  beforeEach(() => {
    mockState.failSettingsSet = false;
    mockState.stored = null;
    setCalls.length = 0;
    getCalls = 0;
  });

  /**
   * The swatch used to force `mode: "custom"` so it "worked" from any mode. But
   * Custom is dark-based: one nudge of the colour threw a Light user into the
   * dark palette, silently, with no way back but re-picking Light.
   */
  it("disables the accent swatch outside the custom theme", () => {
    open("light");
    expect(accent().disabled, "light must not be able to set an accent").toBe(true);
  });

  it("disables the accent swatch in the dark theme too", () => {
    open("dark");
    expect(accent().disabled).toBe(true);
  });

  it("enables the accent swatch in the custom theme, and never changes the mode", async () => {
    open("custom");
    expect(accent().disabled).toBe(false);

    await act(async () => {
      fireEvent.change(accent(), { target: { value: "#ff0000" } });
    });

    await waitFor(() => expect(setCalls).toHaveLength(1));
    expect(setCalls[0].theme).toEqual({ mode: "custom", accent: "#ff0000" });
  });

  /**
   * The regression this test exists for. On a failed save the dialog used to put
   * back the whole `settings` prop it captured at render. Change the theme, then
   * change the language while that save is still in flight, then let the theme
   * save reject — and the successful language change is silently discarded. The
   * store is the only thing that knows what actually stuck.
   */
  it("re-reads the store when a save fails, rather than restoring a stale snapshot", async () => {
    mockState.failSettingsSet = true;
    // What the backend holds: the language change that *did* succeed.
    mockState.stored = { ...settings("dark"), language: "fr" } as Settings;

    const onSettingsSaved = open("dark");
    await act(async () => {
      screen.getByRole("radio", { name: "Light" }).click();
    });

    await waitFor(() => expect(getCalls).toBe(1));
    // The last thing pushed to the parent is the store's truth, not the stale
    // `settings` prop — so `language: "fr"` survives the failed theme save.
    const last = onSettingsSaved.mock.calls.at(-1)?.[0] as Settings;
    expect(last.language).toBe("fr");
  });

  it("surfaces the failure to the user", async () => {
    mockState.failSettingsSet = true;
    open("dark");
    await act(async () => {
      screen.getByRole("radio", { name: "Light" }).click();
    });
    await waitFor(() => expect(screen.getByText(/disk full/)).toBeInTheDocument());
  });
});
