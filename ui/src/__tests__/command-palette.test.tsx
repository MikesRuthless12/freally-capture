import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { CommandPalette } from "../components/CommandPalette";
import { filterCommands, matches, type Command } from "../lib/commands";

vi.mock("../lib/modal", () => ({ pushModal: () => () => undefined }));

const command = (id: string, label: string, group = "Action", keywords?: string): Command => ({
  id,
  label,
  group,
  keywords,
  run: vi.fn(),
});

describe("matches", () => {
  it("matches a subsequence, not just a substring", () => {
    expect(matches("srcw", "Source: Window Capture")).toBe(true);
    expect(matches("window", "Source: Window Capture")).toBe(true);
    expect(matches("zzz", "Source: Window Capture")).toBe(false);
  });

  it("is case-insensitive and ignores spaces in the query", () => {
    expect(matches("WIN CAP", "window capture")).toBe(true);
    expect(matches("  ", "anything")).toBe(true);
  });

  it("respects order — the characters must appear left to right", () => {
    expect(matches("ba", "ab")).toBe(false);
    expect(matches("ab", "ab")).toBe(true);
  });

  it("an empty query matches everything", () => {
    expect(matches("", "anything")).toBe(true);
  });
});

describe("filterCommands", () => {
  const commands = [
    command("a", "Save replay"),
    command("b", "Drop a chapter marker"),
    command("c", "Main scene", "Scene"),
  ];

  it("returns everything for a blank query", () => {
    expect(filterCommands(commands, "  ")).toHaveLength(3);
  });

  it("searches the group and the keywords, not just the label", () => {
    expect(filterCommands(commands, "scene").map((c) => c.id)).toEqual(["c"]);
    const withKeywords = [command("d", "Webcam", "Source", "camera video")];
    expect(filterCommands(withKeywords, "camera")).toHaveLength(1);
  });

  it("does not mutate the input", () => {
    const original = [...commands];
    filterCommands(commands, "x");
    expect(commands).toEqual(original);
  });
});

describe("CommandPalette", () => {
  const setup = (commands: Command[] = [command("a", "Alpha"), command("b", "Beta")]) => {
    const onClose = vi.fn();
    render(<CommandPalette commands={commands} onClose={onClose} />);
    const input = screen.getByRole("combobox");
    return { onClose, input, commands };
  };

  it("focuses the search field so you can type immediately", () => {
    const { input } = setup();
    expect(document.activeElement).toBe(input);
  });

  /**
   * A search-and-pick surface is a combobox over a listbox. The input keeps
   * focus and names the highlighted row through `aria-activedescendant`, so a
   * screen reader announces the selection without focus ever leaving the field.
   */
  it("wires the combobox/listbox pattern a screen reader expects", () => {
    const { input } = setup();
    expect(input).toHaveAttribute("aria-controls", "command-palette-list");
    expect(input).toHaveAttribute("aria-activedescendant", "command-a");

    const list = screen.getByRole("listbox");
    expect(list).toHaveAttribute("id", "command-palette-list");
    expect(screen.getAllByRole("option")).toHaveLength(2);
    expect(screen.getAllByRole("option")[0]).toHaveAttribute("aria-selected", "true");
  });

  it("arrows move the highlight and wrap around", () => {
    const { input } = setup();
    fireEvent.keyDown(input, { key: "ArrowDown" });
    expect(input).toHaveAttribute("aria-activedescendant", "command-b");

    fireEvent.keyDown(input, { key: "ArrowDown" });
    expect(input, "wraps to the first").toHaveAttribute("aria-activedescendant", "command-a");

    fireEvent.keyDown(input, { key: "ArrowUp" });
    expect(input, "wraps to the last").toHaveAttribute("aria-activedescendant", "command-b");
  });

  it("Enter runs the highlighted command and closes", () => {
    const { onClose, input, commands } = setup();
    fireEvent.keyDown(input, { key: "ArrowDown" });
    fireEvent.keyDown(input, { key: "Enter" });

    expect(commands[1].run).toHaveBeenCalledTimes(1);
    expect(commands[0].run).not.toHaveBeenCalled();
    expect(onClose).toHaveBeenCalled();
  });

  it("Escape closes without running anything", () => {
    const { onClose, input, commands } = setup();
    fireEvent.keyDown(input, { key: "Escape" });
    expect(onClose).toHaveBeenCalled();
    expect(commands[0].run).not.toHaveBeenCalled();
  });

  /**
   * Typing resets the highlight to the first result. Without that, a highlight
   * left at index 3 would run a command the user never saw once the list shrank.
   */
  it("typing filters and resets the highlight to the first result", () => {
    const { input } = setup([command("a", "Alpha"), command("b", "Beta"), command("c", "Gamma")]);
    fireEvent.keyDown(input, { key: "ArrowDown" });
    fireEvent.keyDown(input, { key: "ArrowDown" });
    expect(input).toHaveAttribute("aria-activedescendant", "command-c");

    fireEvent.change(input, { target: { value: "bet" } });
    expect(screen.getAllByRole("option")).toHaveLength(1);
    expect(input).toHaveAttribute("aria-activedescendant", "command-b");
  });

  it("Enter on an empty result set does nothing", () => {
    const { onClose, input, commands } = setup();
    fireEvent.change(input, { target: { value: "zzzzz" } });
    expect(screen.queryByRole("listbox")).not.toBeInTheDocument();

    fireEvent.keyDown(input, { key: "Enter" });
    expect(commands[0].run).not.toHaveBeenCalled();
    expect(onClose).not.toHaveBeenCalled();
  });

  /** Same containing-block trap as `PickerShell` — the docks carry backdrop-blur. */
  it("portals out of a filtered ancestor", () => {
    const onClose = vi.fn();
    const { container } = render(
      <div className="backdrop-blur">
        <CommandPalette commands={[command("a", "Alpha")]} onClose={onClose} />
      </div>,
    );
    const dialog = screen.getByRole("dialog", { name: /command palette/i });
    expect(container.contains(dialog)).toBe(false);
    expect(document.body.contains(dialog)).toBe(true);
  });
});
