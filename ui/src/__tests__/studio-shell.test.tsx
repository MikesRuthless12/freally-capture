import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import App from "../App";

describe("studio shell", () => {
  it("renders every studio dock", async () => {
    render(<App />);
    const docks = ["Scenes", "Program preview", "Sources", "Audio Mixer", "Controls"];
    // The first-run EULA gate clears asynchronously (no bridge in jsdom → the
    // status check rejects and the gate fails open), so await the first dock.
    expect(await screen.findByRole("region", { name: docks[0] })).toBeInTheDocument();
    for (const dock of docks.slice(1)) {
      expect(screen.getByRole("region", { name: dock })).toBeInTheDocument();
    }
    // The stats dock waits for settings to settle (no flash of a disabled
    // dock), so query it asynchronously.
    expect(await screen.findByRole("region", { name: "Stats" })).toBeInTheDocument();
  });

  it("keeps the not-yet-implemented controls disabled with honest tooltips", async () => {
    render(<App />);
    // Go Live + the virtual camera arrive with the studio MVP (0.70.0).
    // Recording (Start Recording) is implemented as of 0.55.0.
    // Await the studio past the first-run EULA gate before querying controls.
    await screen.findByRole("button", { name: /go live/i });
    for (const name of [/go live/i, /start virtual camera/i]) {
      const button = screen.getByRole("button", { name });
      expect(button).toBeDisabled();
      expect(button).toHaveAttribute("title");
    }
    expect(screen.getByRole("button", { name: /start recording/i })).toBeInTheDocument();
  });
});
