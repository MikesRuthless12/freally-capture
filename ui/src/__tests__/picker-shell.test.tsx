import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { PickerShell } from "../components/PickerShell";

vi.mock("../lib/modal", () => ({ pushModal: () => () => undefined }));

describe("PickerShell", () => {
  /**
   * Every dialog is opened from inside a `Panel`, and `Panel` carries
   * `backdrop-blur`. A `backdrop-filter` ancestor becomes the containing block
   * for `position: fixed`, which used to centre the overlay inside the dock's
   * own box (~312x176 for the Controls dock) and push the dialog's buttons off
   * the bottom of the window. Portalling to `document.body` is what stops an
   * ancestor's filter/transform/containment from capturing the overlay.
   */
  it("portals out of a filtered ancestor rather than nesting inside it", () => {
    const { container } = render(
      <div className="backdrop-blur" data-testid="dock">
        <PickerShell title="Report a bug" onClose={() => undefined}>
          <button type="button">Open GitHub issue</button>
        </PickerShell>
      </div>,
    );

    const dialog = screen.getByRole("dialog", { name: "Report a bug" });
    const dock = screen.getByTestId("dock");

    expect(dock.contains(dialog)).toBe(false);
    expect(container.contains(dialog)).toBe(false);
    expect(document.body.contains(dialog)).toBe(true);

    // The overlay still covers the viewport, and the actions are reachable.
    expect(dialog.parentElement?.className).toContain("fixed");
    expect(dialog.parentElement?.className).toContain("inset-0");
    expect(screen.getByRole("button", { name: "Open GitHub issue" })).toBeInTheDocument();
  });

  it("marks the dialog as modal for screen readers", () => {
    render(
      <PickerShell title="Settings" onClose={() => undefined}>
        <p>body</p>
      </PickerShell>,
    );
    expect(screen.getByRole("dialog", { name: "Settings" })).toHaveAttribute("aria-modal", "true");
  });
});
