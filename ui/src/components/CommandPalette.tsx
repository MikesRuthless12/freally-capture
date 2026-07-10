import { useEffect, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { filterCommands, type Command } from "../lib/commands";
import { pushModal } from "../lib/modal";
import { useT } from "../i18n/t";

type CommandPaletteProps = {
  commands: Command[];
  onClose: () => void;
};

/**
 * The Ctrl/Cmd-K palette (TASK-904). Search and jump to any scene, source, or
 * action without leaving the keyboard.
 *
 * Portalled to `document.body` for the same reason `PickerShell` is: every dock
 * carries `backdrop-blur`, which makes it the containing block for
 * `position: fixed`, and the overlay would centre itself inside a dock.
 *
 * Wired as a `combobox` over a `listbox`, which is what a screen reader expects
 * of a search-and-pick surface: the input keeps focus and owns the selection via
 * `aria-activedescendant`, so arrow keys move the highlight without ever moving
 * focus off the field.
 */
export function CommandPalette({ commands, onClose }: CommandPaletteProps) {
  const t = useT();
  // Query and highlight move together: typing resets the highlight in the SAME
  // render. Kept as one object rather than a `setActive(0)` effect, which would
  // paint the old highlight for a frame and trip react-hooks/set-state-in-effect.
  const [search, setSearch] = useState({ query: "", active: 0 });
  const { query, active } = search;
  const listRef = useRef<HTMLUListElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const results = useMemo(() => filterCommands(commands, query), [commands, query]);

  // A shrinking result list must not leave the highlight past its end.
  const selected = Math.min(active, Math.max(results.length - 1, 0));

  useEffect(() => pushModal(), []);
  useEffect(() => inputRef.current?.focus(), []);

  // Keep the highlighted row visible while arrowing through a long list.
  // Optional-called: scrolling is a nicety and jsdom does not implement it — a
  // missing `scrollIntoView` must never break the palette's keyboard contract.
  useEffect(() => {
    const list = listRef.current;
    const row = list?.children[selected] as HTMLElement | undefined;
    row?.scrollIntoView?.({ block: "nearest" });
  }, [selected]);

  const onKeyDown = (event: React.KeyboardEvent) => {
    switch (event.key) {
      case "ArrowDown":
        event.preventDefault();
        setSearch((s) => ({ ...s, active: results.length ? (s.active + 1) % results.length : 0 }));
        break;
      case "ArrowUp":
        event.preventDefault();
        setSearch((s) => ({
          ...s,
          active: results.length ? (s.active - 1 + results.length) % results.length : 0,
        }));
        break;
      case "Home":
        event.preventDefault();
        setSearch((s) => ({ ...s, active: 0 }));
        break;
      case "End":
        event.preventDefault();
        setSearch((s) => ({ ...s, active: Math.max(results.length - 1, 0) }));
        break;
      case "Enter": {
        event.preventDefault();
        const command = results[selected];
        if (command) {
          // Close first: a command that opens a dialog must not fight the palette
          // for the modal stack.
          onClose();
          command.run();
        }
        break;
      }
      case "Escape":
        event.preventDefault();
        onClose();
        break;
      default:
        break;
    }
  };

  return createPortal(
    <div
      className="fixed inset-0 z-40 flex items-start justify-center bg-black/60 p-6 pt-[12vh]"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget) onClose();
      }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label={t("palette-title")}
        className="flex max-h-[60vh] w-[34rem] max-w-full flex-col overflow-hidden rounded-xl border border-white/10 bg-havoc-panel shadow-2xl"
      >
        <input
          ref={inputRef}
          type="text"
          role="combobox"
          aria-expanded={true}
          aria-controls="command-palette-list"
          aria-activedescendant={results[selected] ? `command-${results[selected].id}` : undefined}
          aria-autocomplete="list"
          aria-label={t("palette-search")}
          placeholder={t("palette-placeholder")}
          value={query}
          onChange={(event) => setSearch({ query: event.target.value, active: 0 })}
          onKeyDown={onKeyDown}
          className="w-full border-b border-white/5 bg-transparent px-4 py-3 text-sm text-havoc-text outline-none placeholder:text-havoc-muted"
        />

        {results.length === 0 ? (
          <p className="m-0 px-4 py-6 text-center text-xs text-havoc-muted">
            {t("palette-no-results", { query })}
          </p>
        ) : (
          <ul
            id="command-palette-list"
            ref={listRef}
            role="listbox"
            aria-label={t("palette-title")}
            className="m-0 min-h-0 flex-1 list-none overflow-auto p-1"
          >
            {results.map((command, index) => (
              <li
                key={command.id}
                id={`command-${command.id}`}
                role="option"
                aria-selected={index === selected}
                onMouseMove={() => setSearch((s) => ({ ...s, active: index }))}
                onClick={() => {
                  onClose();
                  command.run();
                }}
                className={`flex cursor-pointer items-baseline gap-2 rounded-md px-3 py-2 text-xs ${
                  index === selected ? "bg-havoc-accent/20 text-havoc-text" : "text-havoc-muted"
                }`}
              >
                <span className="shrink-0 text-[10px] tracking-wide text-havoc-muted uppercase">
                  {command.group}
                </span>
                <span className="truncate text-havoc-text">{command.label}</span>
              </li>
            ))}
          </ul>
        )}

        <p className="m-0 border-t border-white/5 px-4 py-1.5 text-[10px] text-havoc-muted">
          {t("palette-hint")}
        </p>
      </div>
    </div>,
    document.body,
  );
}
