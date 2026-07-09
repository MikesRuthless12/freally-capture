import { fireEvent, render, screen } from "@testing-library/react";
import { useRef, useState } from "react";
import { describe, expect, it, vi } from "vitest";

import { useDismiss } from "../lib/useDismiss";

/** A trigger + popover wired exactly like the Sources rail's "+" menu. */
function Menu() {
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);
  useDismiss(open, ref, () => setOpen(false));
  return (
    <div>
      <div ref={ref}>
        <button type="button" onClick={() => setOpen((o) => !o)}>
          Add a source
        </button>
        {open && (
          <div role="menu" aria-label="Add a source">
            <button type="button" role="menuitem">
              Display Capture
            </button>
          </div>
        )}
      </div>
      <button type="button">Outside</button>
    </div>
  );
}

const menu = () => screen.queryByRole("menu", { name: "Add a source" });

/**
 * jsdom has no `PointerEvent` constructor, and the hook only reads `.target`,
 * so a bubbling MouseEvent named "pointerdown" exercises the real listener.
 * The browser always fires pointerdown *before* click — these helpers preserve
 * that order, which is the whole point of listening on pointerdown.
 */
const pointerDown = (el: Element) =>
  fireEvent(el, new MouseEvent("pointerdown", { bubbles: true, cancelable: true }));

const pressAndClick = (el: Element) => {
  pointerDown(el);
  fireEvent.click(el);
};

describe("useDismiss", () => {
  it("closes the menu when the pointer goes down outside it", () => {
    render(<Menu />);
    pressAndClick(screen.getByRole("button", { name: "Add a source" }));
    expect(menu()).toBeInTheDocument();

    pressAndClick(screen.getByRole("button", { name: "Outside" }));
    expect(menu()).not.toBeInTheDocument();
  });

  it("closes on Escape", () => {
    render(<Menu />);
    pressAndClick(screen.getByRole("button", { name: "Add a source" }));
    expect(menu()).toBeInTheDocument();

    fireEvent.keyDown(document, { key: "Escape" });
    expect(menu()).not.toBeInTheDocument();
  });

  /**
   * The trap: if the ref wrapped only the popover, a click on the trigger would
   * fire the outside-pointerdown handler (close) and then the trigger's own
   * onClick (toggle back open) — leaving the menu stuck open forever.
   */
  it("lets the trigger toggle the menu shut rather than reopening it", () => {
    render(<Menu />);
    const trigger = screen.getByRole("button", { name: "Add a source" });

    pressAndClick(trigger);
    expect(menu()).toBeInTheDocument();

    pressAndClick(trigger);
    expect(menu()).not.toBeInTheDocument();
  });

  it("keeps the menu open when the pointer goes down inside it", () => {
    render(<Menu />);
    pressAndClick(screen.getByRole("button", { name: "Add a source" }));

    pointerDown(screen.getByRole("menuitem", { name: "Display Capture" }));
    expect(menu()).toBeInTheDocument();
  });

  it("does not listen while the menu is closed", () => {
    render(<Menu />);
    // No menu, no crash, no state churn from stray clicks.
    pressAndClick(screen.getByRole("button", { name: "Outside" }));
    fireEvent.keyDown(document, { key: "Escape" });
    expect(menu()).not.toBeInTheDocument();
  });

  /**
   * These menus live inside a `PickerShell`, which closes the whole dialog on
   * Escape via a bubble-phase listener on `window`. One Escape must close only
   * the menu; the dialog needs a second press.
   */
  it("stops Escape from also closing an enclosing dialog", () => {
    const outer = vi.fn();
    window.addEventListener("keydown", outer);
    try {
      render(<Menu />);
      pressAndClick(screen.getByRole("button", { name: "Add a source" }));

      fireEvent.keyDown(document, { key: "Escape" });
      expect(menu()).not.toBeInTheDocument();
      expect(outer).not.toHaveBeenCalled();

      // Menu closed → the hook unsubscribes → the dialog sees the next Escape.
      fireEvent.keyDown(document, { key: "Escape" });
      expect(outer).toHaveBeenCalledTimes(1);
    } finally {
      window.removeEventListener("keydown", outer);
    }
  });

  it("never swallows keys other than Escape", () => {
    const outer = vi.fn();
    window.addEventListener("keydown", outer);
    try {
      render(<Menu />);
      pressAndClick(screen.getByRole("button", { name: "Add a source" }));
      fireEvent.keyDown(document, { key: "a" });
      expect(menu()).toBeInTheDocument();
      expect(outer).toHaveBeenCalledTimes(1);
    } finally {
      window.removeEventListener("keydown", outer);
    }
  });
});
