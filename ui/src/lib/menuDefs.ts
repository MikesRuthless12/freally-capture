/**
 * The in-app menu bar's row shapes and link targets (OBS-style chrome).
 *
 * Data shapes only — the entries themselves are built inside `MenuBar`, where
 * `t()` and the App-supplied handlers live. Kept out of the component file for
 * the same reason as `lib/commands.ts`: a module exporting both a component
 * and helpers loses Fast Refresh.
 */

/** One row of an open menu. Disabled rows stay focusable (`aria-disabled`). */
export type MenuEntry =
  | {
      kind: "item";
      id: string;
      label: string;
      onSelect: () => void;
      disabled?: boolean;
      /** Plain-text hint (e.g. "Ctrl+Z") — must mirror a binding App registers. */
      shortcut?: string;
      /** Tooltip; used to say honestly why a disabled row is disabled. */
      title?: string;
    }
  | {
      kind: "check";
      id: string;
      label: string;
      checked: boolean;
      onSelect: () => void;
      disabled?: boolean;
    }
  | { kind: "radio"; id: string; label: string; checked: boolean; onSelect: () => void }
  | { kind: "link"; id: string; label: string; href: string }
  | { kind: "separator" };

/** One top-level menu: its trigger label and its rows. */
export type MenuDef = { id: string; label: string; entries: MenuEntry[] };

/** The public site (GitHub Pages) — the same origin the About dialog links to. */
export const SITE_URL = "https://mikesruthless12.github.io/freally-capture";
export const HELP_URL = `${SITE_URL}/documentation.html`;
export const WEBSITE_URL = `${SITE_URL}/index.html`;
export const CHANGELOG_URL = `${SITE_URL}/changelog.html`;

/** No server yet — set the invite URL to enable Help → Join Discord Server. */
export const DISCORD_URL: string | null = null;
