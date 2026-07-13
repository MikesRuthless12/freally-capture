import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import type { Collection, ProgramStatus, StudioDto } from "../api/types";
import { SourceHealthDialog } from "../panels/SourceHealthDialog";

vi.mock("../lib/modal", () => ({ pushModal: () => () => undefined }));

const retrySource = vi.hoisted(() => vi.fn(() => Promise.resolve(null)));
vi.mock("../api/commands", () => ({
  studioRetrySource: retrySource,
}));

function fixtureCollection(): Collection {
  return {
    formatVersion: 1,
    canvasWidth: 1920,
    canvasHeight: 1080,
    sources: [
      { id: "src-cam", name: "Face cam", kind: "videoDevice", deviceId: "cam-0", format: null },
      { id: "src-img", name: "Overlay", kind: "image", path: "C:/art/overlay.png" },
      { id: "src-win", name: "Game window", kind: "window", captureId: "win:1", label: "Game" },
    ],
    scenes: [{ id: "scene-a", name: "Main", items: [] }],
    activeScene: "scene-a",
  };
}

function fixtureProgram(): ProgramStatus {
  return {
    state: "running",
    width: 1920,
    height: 1080,
    fps: 60,
    renderMicros: 900,
    adapter: "Test GPU",
    dropped: 0,
    sources: {
      "src-cam": {
        state: "live",
        width: 1280,
        height: 720,
        fps: 30,
        lastFrameMs: 250,
        dropped: 4,
        retries: 2,
      },
      "src-win": {
        state: "error",
        errorCode: "notFound",
        errorMessage: "window went away",
      },
    },
  };
}

function renderDialog(overrides?: { onOpenProperties?: (id: string) => void }) {
  return render(
    <SourceHealthDialog
      studio={{ collection: fixtureCollection() } as unknown as StudioDto}
      program={fixtureProgram()}
      onOpenProperties={overrides?.onOpenProperties ?? (() => undefined)}
      onClose={() => undefined}
    />,
  );
}

describe("SourceHealthDialog (CAP-M13)", () => {
  it("shows one row per collection source with its live health", () => {
    renderDialog();

    // The live capture: state, resolution, fps, staleness, drops, restarts.
    expect(screen.getByText("Face cam")).toBeInTheDocument();
    expect(screen.getByText("Live")).toBeInTheDocument();
    expect(screen.getByText("1280×720")).toBeInTheDocument();
    expect(screen.getByText("30")).toBeInTheDocument();
    expect(screen.getByText("0.3 s")).toBeInTheDocument();
    expect(screen.getByText("4")).toBeInTheDocument();
    expect(screen.getByText("2")).toBeInTheDocument();

    // The errored capture surfaces its message.
    expect(screen.getByText("Error")).toBeInTheDocument();
    expect(screen.getByText("window went away")).toBeInTheDocument();

    // A source the engine isn't running reads inactive — honestly.
    expect(screen.getByText("Inactive")).toBeInTheDocument();
  });

  it("restarts a source through studio_retry_source", () => {
    renderDialog();

    fireEvent.click(screen.getByRole("button", { name: "Retry Game window" }));
    expect(retrySource).toHaveBeenCalledWith("src-win");

    // An inactive source has nothing to restart.
    expect(screen.getByRole("button", { name: "Retry Overlay" })).toBeDisabled();
  });

  it("opens source properties from a row", () => {
    const onOpenProperties = vi.fn();
    renderDialog({ onOpenProperties });

    fireEvent.click(screen.getByRole("button", { name: "Properties of Face cam" }));
    expect(onOpenProperties).toHaveBeenCalledWith("src-cam");
  });
});
