import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { act } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import type { Settings, ThemeMode } from "../api/types";
import { SettingsDialog } from "../panels/Settings";
import { applyTheme } from "../theme/theme";

const mockState = vi.hoisted(() => ({
  failSettingsSet: false,
  /** What `settingsGet()` returns — the store's truth the dialog seeds from. */
  stored: null as Settings | null,
}));

const setCalls: Settings[] = [];

vi.mock("../api/commands", () => ({
  settingsGet: () => Promise.resolve(mockState.stored),
  settingsSet: (next: Settings) => {
    setCalls.push(next);
    return mockState.failSettingsSet
      ? Promise.reject(new Error("disk full"))
      : Promise.resolve(null);
  },
  // The Streaming/Output/Network panes probe these on mount.
  encodersList: () => Promise.resolve({ encoders: [] }),
  ffmpegStatus: () => Promise.resolve({ state: "ready", version: "7" }),
  panelUrl: () => Promise.resolve(null),
  linkUrl: () => Promise.resolve(null),
}));

vi.mock("../api/events", () => ({ onFfmpeg: () => Promise.resolve(() => undefined) }));

vi.mock("../lib/modal", () => ({ pushModal: () => () => undefined }));

/** `applyTheme` writes to `documentElement`; harmless in jsdom, but noisy. */
vi.mock("../theme/theme", async (importOriginal) => ({
  ...(await importOriginal<typeof import("../theme/theme")>()),
  applyTheme: vi.fn(),
}));

/** A full Settings, so every category pane can render from the draft. */
const settings = (mode: ThemeMode): Settings => ({
  language: "en",
  showStatsDock: true,
  monitorDevice: null,
  mixerLayout: "horizontal",
  theme: { mode, accent: "#4a9eff" },
  alignment: { smartGuides: true, safeAreas: false, rulers: false },
  accessibility: {
    meterPreset: "default",
    meterLow: "#22c55e",
    meterMid: "#eab308",
    meterHigh: "#ef4444",
  },
  completedOnboarding: true,
  recording: {
    container: "frec",
    encoderId: "auto",
    rateControl: { mode: "cqp", bitrateKbps: 8000, cq: 23 },
    preset: "balanced",
    keyframeSec: 2,
    fps: 60,
    audioBitrateKbps: 160,
    tracksMask: 1,
    folder: "",
    filenamePrefix: "Freally",
    template: "{prefix} {date} {time}",
    replayTemplate: "Replay {date} {time}",
    stillTemplate: "Still {date} {time}",
    replayFolder: "",
    stillFolder: "",
    counter: 0,
    splitMinutes: 0,
    recordVertical: false,
    outputWidth: 0,
    outputHeight: 0,
  },
  remote: { turnUrl: "", turnUsername: "", turnCredential: "" },
  stream: { targets: [], autoRecord: false, preflightHold: false },
  replay: { seconds: 30, bitrateKbps: 6000, audioBitrateKbps: 160, fps: 60, track: 1 },
  transition: {
    kind: "fade",
    durationMs: 300,
    lumaImage: "",
    stingerPath: "",
    stingerCutMs: 0,
    stingerMatte: "none",
    stingerDuckDb: 0,
  },
  hotkeys: {
    record: null,
    goLive: null,
    transition: null,
    saveReplay: null,
    addMarker: null,
    still: null,
    panic: null,
    timerToggle: null,
    timerReset: null,
    zoom100: null,
    zoom150: null,
    zoom200: null,
    splitTimerSplit: null,
    splitTimerUndo: null,
    splitTimerSkip: null,
    splitTimerReset: null,
    playlistNext: null,
    playlistPrevious: null,
    replayRoll: null,
  },
  panicSlate: { color: "#10141a", image: "" },
  remoteControl: { enabled: false, port: 4455, lan: false, password: "" },
  browserDocks: [],
  scripts: [],
  webPanel: { enabled: false, port: 4457, lan: false, password: "" },
  osc: { enabled: false, port: 9000, lan: false },
  link: { enabled: false, port: 9720, name: "", key: "" },
});

async function open(mode: ThemeMode) {
  const onSettingsSaved = vi.fn();
  const onClose = vi.fn();
  // The dialog seeds its draft from settingsGet(), not the prop — keep the
  // store on the same theme unless a test overrode it deliberately.
  if (mockState.stored?.theme.mode !== mode) {
    mockState.stored = settings(mode);
  }
  render(
    <SettingsDialog
      settings={settings(mode)}
      onSettingsSaved={onSettingsSaved}
      onClose={onClose}
      onOpen={() => undefined}
    />,
  );
  // The draft seeds from a fresh settingsGet() (async). Wait for a SEEDED
  // General-pane control, not just the tab shell — otherwise a synchronous
  // getByLabelText can race the seed and flake under full-suite load.
  await screen.findByRole("tab", { name: "General" });
  await screen.findByLabelText("Show the stats dock");
  return { onSettingsSaved, onClose };
}

const tab = (name: string) => screen.getByRole("tab", { name });

async function show(name: string) {
  await act(async () => {
    tab(name).click();
  });
}

const accent = () => screen.getByLabelText("Accent") as HTMLInputElement;

describe("SettingsDialog", () => {
  beforeEach(() => {
    mockState.failSettingsSet = false;
    mockState.stored = settings("dark");
    setCalls.length = 0;
    vi.mocked(applyTheme).mockClear();
  });

  it("renders the categories as a vertical tablist", async () => {
    await open("dark");
    const list = screen.getByRole("tablist", { name: "Settings categories" });
    expect(list).toHaveAttribute("aria-orientation", "vertical");
    expect(screen.getAllByRole("tab")).toHaveLength(9);
    expect(tab("General")).toHaveAttribute("aria-selected", "true");
  });

  it("moves the active category with the arrow keys", async () => {
    await open("dark");
    await act(async () => {
      fireEvent.keyDown(tab("General"), { key: "ArrowDown" });
    });
    expect(tab("Appearance")).toHaveAttribute("aria-selected", "true");
    await act(async () => {
      fireEvent.keyDown(tab("Appearance"), { key: "ArrowUp" });
    });
    expect(tab("General")).toHaveAttribute("aria-selected", "true");
  });

  /** The dialog edits a DRAFT: nothing may persist until Apply/OK. */
  it("does not save on edit — Apply persists the draft and stays open", async () => {
    const { onSettingsSaved, onClose } = await open("dark");
    await act(async () => {
      screen.getByLabelText("Show the stats dock").click();
    });
    expect(setCalls).toHaveLength(0);

    await act(async () => {
      screen.getByRole("button", { name: "Apply" }).click();
    });
    await waitFor(() => expect(setCalls).toHaveLength(1));
    expect(setCalls[0].showStatsDock).toBe(false);
    expect(onSettingsSaved).toHaveBeenCalled();
    expect(onClose, "Apply must not close the dialog").not.toHaveBeenCalled();
  });

  it("OK applies the draft and closes", async () => {
    const { onClose } = await open("dark");
    await act(async () => {
      screen.getByLabelText("Show the stats dock").click();
    });
    await act(async () => {
      screen.getByRole("button", { name: "OK" }).click();
    });
    await waitFor(() => expect(onClose).toHaveBeenCalled());
    expect(setCalls).toHaveLength(1);
  });

  it("Cancel discards the draft and reverts the live theme preview", async () => {
    const { onClose } = await open("dark");
    await show("Appearance");
    await act(async () => {
      screen.getByRole("radio", { name: "Light" }).click();
    });
    // The preview applied live…
    expect(vi.mocked(applyTheme).mock.calls.at(-1)?.[0]).toMatchObject({ mode: "light" });

    await act(async () => {
      screen.getByRole("button", { name: "Cancel" }).click();
    });
    // …and Cancel put the last APPLIED theme back without saving anything.
    expect(vi.mocked(applyTheme).mock.calls.at(-1)?.[0]).toMatchObject({ mode: "dark" });
    expect(setCalls).toHaveLength(0);
    expect(onClose).toHaveBeenCalled();
  });

  it("keeps the dialog open and shows the error when the save fails", async () => {
    mockState.failSettingsSet = true;
    const { onClose } = await open("dark");
    await act(async () => {
      screen.getByLabelText("Show the stats dock").click();
    });
    await act(async () => {
      screen.getByRole("button", { name: "OK" }).click();
    });
    await waitFor(() => expect(screen.getByText(/disk full/)).toBeInTheDocument());
    expect(onClose, "a failed OK must not close over the error").not.toHaveBeenCalled();
  });

  /**
   * The swatch used to force `mode: "custom"` so it "worked" from any mode. But
   * Custom is dark-based: one nudge of the colour threw a Light user into the
   * dark palette, silently, with no way back but re-picking Light.
   */
  it("disables the accent swatch outside the custom theme", async () => {
    await open("light");
    await show("Appearance");
    expect(accent().disabled, "light must not be able to set an accent").toBe(true);
  });

  it("enables the accent swatch in the custom theme, and never changes the mode", async () => {
    await open("custom");
    await show("Appearance");
    expect(accent().disabled).toBe(false);
    await act(async () => {
      fireEvent.change(accent(), { target: { value: "#ff0000" } });
    });
    await act(async () => {
      screen.getByRole("button", { name: "Apply" }).click();
    });
    await waitFor(() => expect(setCalls).toHaveLength(1));
    expect(setCalls[0].theme).toEqual({ mode: "custom", accent: "#ff0000" });
  });

  /** The old Remote dialog's gate, now enforced per-category by Apply. */
  it("refuses to enable the remote API without a password, in its category", async () => {
    await open("dark");
    await show("Network");
    await act(async () => {
      screen.getByLabelText("Enable the WebSocket remote API").click();
    });
    await act(async () => {
      screen.getByRole("button", { name: "Apply" }).click();
    });
    expect(setCalls, "an invalid draft must not reach the store").toHaveLength(0);
    expect(screen.getByRole("alert")).toHaveTextContent(/password is required/i);
  });

  /** One pool, exclusive allocation: a combo held by one action is not
   * offered to any other; None returns it. Conflicts die at entry. */
  it("removes a picked hotkey from every other row, and None returns it", async () => {
    await open("dark");
    await show("Hotkeys");
    const record = screen.getByLabelText("Start / stop recording") as HTMLSelectElement;
    const goLive = screen.getByLabelText("Go Live / End Stream") as HTMLSelectElement;
    const values = (select: HTMLSelectElement) =>
      Array.from(select.options).map((option) => option.value);

    expect(values(goLive)).toContain("Ctrl+D");
    await act(async () => {
      fireEvent.change(record, { target: { value: "Ctrl+D" } });
    });
    expect(values(goLive), "a held combo must leave the other rows").not.toContain("Ctrl+D");
    expect(values(record), "the holder keeps seeing its own combo").toContain("Ctrl+D");

    await act(async () => {
      fireEvent.change(record, { target: { value: "" } });
    });
    expect(values(goLive), "None returns the combo to the pool").toContain("Ctrl+D");
  });

  /** A saved binding outside the curated pool (hand-edited settings.json, a
   * CAP-N05 chord) is shown honestly on its row, never silently clobbered. */
  it("keeps an out-of-pool binding selectable on its own row", async () => {
    const stored = settings("dark");
    stored.hotkeys.record = "Ctrl+K, 3";
    mockState.stored = stored;
    await open("dark");
    await show("Hotkeys");
    const record = screen.getByLabelText("Start / stop recording") as HTMLSelectElement;
    expect(record.value).toBe("Ctrl+K, 3");
    expect(Array.from(record.options).some((option) => option.value === "Ctrl+K, 3")).toBe(true);
  });

  /** The dialog seeds from the STORE, not the render prop — a stale prop
   * would resurrect settings another dialog saved meanwhile. */
  it("seeds the draft from a fresh settingsGet, not the render prop", async () => {
    mockState.stored = { ...settings("dark"), showStatsDock: false };
    await open("dark");
    const box = screen.getByLabelText("Show the stats dock") as HTMLInputElement;
    expect(box.checked).toBe(false);
  });
});
