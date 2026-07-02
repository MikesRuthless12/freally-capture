import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import App from "../App";

describe("studio shell", () => {
  it("renders every studio dock", async () => {
    render(<App />);
    const docks = ["Scenes", "Program preview", "Sources", "Audio Mixer", "Controls"];
    for (const dock of docks) {
      expect(screen.getByRole("region", { name: dock })).toBeInTheDocument();
    }
    // The stats dock waits for settings to settle (no flash of a disabled
    // dock), so query it asynchronously.
    expect(await screen.findByRole("region", { name: "Stats" })).toBeInTheDocument();
  });

  it("keeps the not-yet-implemented controls disabled with honest tooltips", () => {
    render(<App />);
    for (const name of [/start recording/i, /go live/i, /start virtual camera/i]) {
      const button = screen.getByRole("button", { name });
      expect(button).toBeDisabled();
      expect(button).toHaveAttribute("title");
    }
  });
});
