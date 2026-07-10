import { fireEvent, render, screen } from "@testing-library/react";
import { useRef } from "react";
import { describe, expect, it, vi } from "vitest";

import { PickerShell } from "../components/PickerShell";
import { useFocusTrap } from "../lib/useFocusTrap";

vi.mock("../lib/modal", () => ({ pushModal: () => () => undefined }));

/**
 * jsdom reports `offsetParent === null` for everything (no layout), so the
 * visibility filter in `focusableIn` would reject every control. Give the
 * elements a non-null `offsetParent` the way a real browser would.
 */
function withLayout() {
  Object.defineProperty(HTMLElement.prototype, "offsetParent", {
    configurable: true,
    get() {
      return this.parentNode;
    },
  });
}
withLayout();

function Trapped({ active = true }: { active?: boolean }) {
  const ref = useRef<HTMLDivElement>(null);
  useFocusTrap(active, ref);
  return (
    <div ref={ref}>
      <button type="button">first</button>
      <button type="button">middle</button>
      <button type="button" disabled>
        disabled
      </button>
      <button type="button">last</button>
    </div>
  );
}

const button = (name: string) => screen.getByRole("button", { name });

describe("useFocusTrap", () => {
  it("moves focus to the first control on mount", () => {
    render(<Trapped />);
    expect(document.activeElement).toBe(button("first"));
  });

  it("Tab from the last control wraps to the first", () => {
    render(<Trapped />);
    button("last").focus();
    fireEvent.keyDown(document, { key: "Tab" });
    expect(document.activeElement).toBe(button("first"));
  });

  it("Shift+Tab from the first control wraps to the last", () => {
    render(<Trapped />);
    button("first").focus();
    fireEvent.keyDown(document, { key: "Tab", shiftKey: true });
    expect(document.activeElement).toBe(button("last"));
  });

  /** A disabled control is not Tab-reachable; the trap must agree. */
  it("skips disabled controls when picking the last stop", () => {
    render(<Trapped />);
    button("middle").focus();
    // "disabled" sits between middle and last, so middle is not the last stop
    // and Tab must be allowed through untouched.
    fireEvent.keyDown(document, { key: "Tab" });
    expect(document.activeElement, "the browser handles this hop").toBe(button("middle"));
  });

  /**
   * A click on the backdrop can drop focus on `<body>`. The next Tab must come
   * back into the dialog, not start walking the studio behind it.
   */
  it("pulls focus back in when it has escaped the container", () => {
    render(<Trapped />);
    (document.activeElement as HTMLElement)?.blur();
    expect(document.activeElement).toBe(document.body);

    fireEvent.keyDown(document, { key: "Tab" });
    expect(document.activeElement).toBe(button("first"));
  });

  it("does nothing while inactive", () => {
    render(
      <>
        <button type="button">outside</button>
        <Trapped active={false} />
      </>,
    );
    button("outside").focus();
    fireEvent.keyDown(document, { key: "Tab" });
    expect(document.activeElement).toBe(button("outside"));
  });

  /** Escape must not dump focus on `<body>` — the next Tab would restart at the top. */
  it("restores focus to whatever had it before", () => {
    render(<button type="button">opener</button>);
    const opener = button("opener");
    opener.focus();

    const { unmount } = render(<Trapped />);
    expect(document.activeElement).toBe(button("first"));

    unmount();
    expect(document.activeElement).toBe(opener);
  });
});

describe("PickerShell focus management", () => {
  it("traps Tab and restores focus when the dialog closes", () => {
    render(<button type="button">opener</button>);
    const opener = button("opener");
    opener.focus();

    const { unmount } = render(
      <PickerShell title="Report a bug" onClose={() => undefined}>
        <button type="button">inside</button>
      </PickerShell>,
    );

    // The header's close button is the first focusable control in the dialog.
    expect(screen.getByRole("dialog", { name: "Report a bug" })).toBeInTheDocument();
    expect(document.activeElement?.tagName).toBe("BUTTON");
    expect(document.activeElement).not.toBe(opener);

    unmount();
    expect(document.activeElement).toBe(opener);
  });
});
