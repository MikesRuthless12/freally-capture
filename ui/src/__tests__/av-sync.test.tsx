import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import type { AudioLevelsPayload, Collection, ProgramStatus, StudioDto } from "../api/types";
import { AvSyncDialog } from "../panels/AvSyncDialog";

vi.mock("../lib/modal", () => ({ pushModal: () => () => undefined }));

const commands = vi.hoisted(() => ({
  auxWindowOpen: vi.fn(() => Promise.resolve()),
  calibrationFinish: vi.fn(() => Promise.resolve({})),
  calibrationStart: vi.fn(() => Promise.resolve()),
  calibrationStatus: vi.fn(() =>
    Promise.resolve({ videoSamples: 0, audioSamples: 0, flashSeen: false, beepHeard: false }),
  ),
  calibrationStop: vi.fn(() => Promise.resolve()),
  listDisplays: vi.fn(() => Promise.resolve([])),
  studioAddItem: vi.fn(() => Promise.resolve({ sourceId: "src-temp", itemId: "item-temp" })),
  studioRemoveItem: vi.fn(() => Promise.resolve()),
  studioSetAudioMonitor: vi.fn(() => Promise.resolve()),
  studioSetAudioSolo: vi.fn(() => Promise.resolve()),
  studioSetAudioSyncOffset: vi.fn(() => Promise.resolve()),
}));
vi.mock("../api/commands", () => commands);

function fixtureCollection(): Collection {
  return {
    formatVersion: 1,
    canvasWidth: 1920,
    canvasHeight: 1080,
    sources: [
      {
        id: "src-cam",
        name: "Face cam",
        kind: "videoDevice",
        deviceId: "cam-0",
        format: null,
        deinterlace: "off",
        fieldOrder: "topFirst",
      },
      {
        id: "src-mic",
        name: "Desk mic",
        kind: "audioInput",
        deviceId: "",
        audio: {
          volumeDb: 0,
          muted: false,
          monitor: "off",
          tracks: 1,
          pan: 0,
          solo: false,
          mono: false,
          syncOffsetMs: 40,
          filters: [],
        },
      },
      // Test signals must not be offered as calibration candidates.
      { id: "src-bars", name: "SMPTE Bars", kind: "testBars", width: 1920, height: 1080 },
      // A camera in the model but NOT running (off-program scene): offering it
      // would guarantee a failed measurement.
      {
        id: "src-cam2",
        name: "Idle cam",
        kind: "videoDevice",
        deviceId: "cam-1",
        format: null,
        deinterlace: "off",
        fieldOrder: "topFirst",
      },
    ],
    scenes: [{ id: "scene-a", name: "Main", items: [] }],
    activeScene: "scene-a",
  } as unknown as Collection;
}

/** Only sources the engine is RUNNING can be measured (CAP-M20 review fix):
 * the camera streams (program event), the mic has an engine strip (audio
 * event). "src-cam2" is live in the model but not running — it must not be
 * offered, or the run would fail with a misleading "never saw the flash". */
function fixtureProgram(): ProgramStatus {
  return {
    state: "running",
    sources: { "src-cam": { state: "live", width: 1280, height: 720, fps: 30 } },
  } as unknown as ProgramStatus;
}

function fixtureAudio(): AudioLevelsPayload {
  return {
    sources: { "src-mic": { state: "live", peak: [0.1, 0.1], rms: [0.05, 0.05], gated: false } },
  } as unknown as AudioLevelsPayload;
}

function renderDialog() {
  return render(
    <AvSyncDialog
      studio={{ collection: fixtureCollection() } as unknown as StudioDto}
      program={fixtureProgram()}
      audio={fixtureAudio()}
      onClose={() => undefined}
    />,
  );
}

describe("AvSyncDialog (CAP-M20)", () => {
  it("offers only RUNNING cameras and mics, and holds Start until both are picked", () => {
    renderDialog();

    const [videoSelect, audioSelect] = screen.getAllByRole("combobox");
    expect(videoSelect).toContainHTML("Face cam");
    expect(videoSelect).not.toContainHTML("SMPTE Bars");
    expect(videoSelect).not.toContainHTML("Desk mic");
    // Not streaming → not measurable → not offered.
    expect(videoSelect).not.toContainHTML("Idle cam");
    expect(audioSelect).toContainHTML("Desk mic");
    expect(audioSelect).not.toContainHTML("Face cam");

    const start = screen.getByRole("button", { name: "Start calibration" });
    expect(start).toBeDisabled();
    fireEvent.change(videoSelect, { target: { value: "src-cam" } });
    fireEvent.change(audioSelect, { target: { value: "src-mic" } });
    expect(start).toBeEnabled();
  });

  it("starting adds the temp pattern full-canvas, monitors it, and arms the probes", async () => {
    renderDialog();

    const [videoSelect, audioSelect] = screen.getAllByRole("combobox");
    fireEvent.change(videoSelect, { target: { value: "src-cam" } });
    fireEvent.change(audioSelect, { target: { value: "src-mic" } });
    fireEvent.click(screen.getByRole("button", { name: "Start calibration" }));

    // The measuring step appears once the start chain resolves.
    expect(await screen.findByText(/Waiting for the camera/)).toBeInTheDocument();
    expect(commands.studioAddItem).toHaveBeenCalledWith("scene-a", {
      kind: "testFlashBeep",
      width: 1920,
      height: 1080,
    });
    expect(commands.studioSetAudioMonitor).toHaveBeenCalledWith("src-temp", "monitorOnly");
    // Soloed too: a PFL solo left on any strip would otherwise evict the
    // pattern from the monitor bus and silence the beep (CAP-M19 interaction).
    expect(commands.studioSetAudioSolo).toHaveBeenCalledWith("src-temp", true);
    expect(commands.calibrationStart).toHaveBeenCalledWith("src-cam", "src-mic");
  });

  it("unmount disarms the probes and removes the temp source", async () => {
    const { unmount } = renderDialog();

    const [videoSelect, audioSelect] = screen.getAllByRole("combobox");
    fireEvent.change(videoSelect, { target: { value: "src-cam" } });
    fireEvent.change(audioSelect, { target: { value: "src-mic" } });
    fireEvent.click(screen.getByRole("button", { name: "Start calibration" }));
    await screen.findByText(/Waiting for the camera/);

    unmount();
    expect(commands.calibrationStop).toHaveBeenCalled();
    expect(commands.studioRemoveItem).toHaveBeenCalledWith("scene-a", "item-temp");
  });
});
