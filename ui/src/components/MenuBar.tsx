import { useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { getCurrentWindow } from "@tauri-apps/api/window";

import {
  captureStill,
  collectionSwitch,
  collectionsList,
  openExternal,
  profileSwitch,
  profilesList,
  revealAppFolder,
  studioRedo,
  studioUndo,
  type NamedList,
} from "../api/commands";
import type { Settings } from "../api/types";
import { useT } from "../i18n/t";
import { useClipboard } from "../lib/clipboard";
import { DISCORD_URL, HELP_URL, WEBSITE_URL, type MenuDef, type MenuEntry } from "../lib/menuDefs";
import { pushModal } from "../lib/modal";
import type { ControlsDialogKind } from "../panels/ControlsDock";

const fail = (what: string) => (err: unknown) => console.error(`${what} failed:`, err);
const noop = () => undefined;
const sep: MenuEntry = { kind: "separator" };

/** App-owned dialogs the menus can open (the same set the palette reaches). */
export type AppMenuDialog =
  | "history"
  | "multiview"
  | "projector"
  | "downstream"
  | "transitionRules"
  | "sourceHealth"
  | "avSync"
  | "hotkeyAudit"
  | "missingFiles";

type MenuBarProps = {
  /** Opens one of the Controls dock's dialogs (the registration seam). */
  onOpenControls: (kind: ControlsDialogKind) => void;
  /** Opens one of App's own dialogs. */
  onOpenApp: (kind: AppMenuDialog) => void;
  /** An item of the active scene is selected — gates the Edit rows. */
  hasSelection: boolean;
  /** The selected item has at least one filter to copy. */
  canCopyFilters: boolean;
  onCopyTransform: () => void;
  onPasteTransform: () => void;
  onCopyFilters: () => void;
  onPasteFilters: () => void;
  onEditTransform: () => void;
  /** The stats-dock checkbox — shared with the header chip. */
  statsShown: boolean;
  statsReady: boolean;
  onToggleStats: () => void;
  /** A profile switch resolves to the new settings — App must adopt them. */
  onSettingsSaved: (next: Settings) => void;
};

/**
 * The OBS-style in-app menu bar. A pure dispatch layer: every row reuses a
 * handler the palette, the Controls dock, or a dialog already owns.
 *
 * The open dropdown portals to `document.body` (dock `backdrop-blur` makes any
 * dock a containing block for `position: fixed` — the PickerShell trap) and
 * registers with `pushModal()` so the native GPU preview never paints over it.
 * Keyboard model per the ARIA menubar pattern: Left/Right between menus,
 * Down/Up within one, Home/End, Escape closes back to the trigger. `useDismiss`
 * is not reusable here — its single ref cannot span a portalled panel — so the
 * same capture-phase pointerdown/Escape listeners are wired against both refs.
 */
export function MenuBar({
  onOpenControls,
  onOpenApp,
  hasSelection,
  canCopyFilters,
  onCopyTransform,
  onPasteTransform,
  onCopyFilters,
  onPasteFilters,
  onEditTransform,
  statsShown,
  statsReady,
  onToggleStats,
  onSettingsSaved,
}: MenuBarProps) {
  const t = useT();
  const clipboard = useClipboard();
  // Open menu + roving focus travel together, like the palette's query/active:
  // switching menus must reset the focused row in the same render. `focus` is
  // `null` (stay on the trigger), a row index, or -1 for "last row".
  const [state, setState] = useState<{ open: string | null; focus: number | null }>({
    open: null,
    focus: null,
  });
  const [anchor, setAnchor] = useState<{ left: number; top: number } | null>(null);
  // Loaded lazily each time the Profile / Scene Collection menu opens.
  const [profiles, setProfiles] = useState<NamedList | null>(null);
  const [collections, setCollections] = useState<NamedList | null>(null);
  const [fullscreen, setFullscreen] = useState(() => Boolean(document.fullscreenElement));
  const [barFocus, setBarFocus] = useState(0);
  const barRef = useRef<HTMLDivElement>(null);
  const panelRef = useRef<HTMLDivElement>(null);
  const triggerRefs = useRef<(HTMLButtonElement | null)[]>([]);
  const itemRefs = useRef<(HTMLElement | null)[]>([]);

  useEffect(() => {
    const sync = () => setFullscreen(Boolean(document.fullscreenElement));
    document.addEventListener("fullscreenchange", sync);
    return () => document.removeEventListener("fullscreenchange", sync);
  }, []);

  const toggleFullscreen = useCallback(() => {
    if (document.fullscreenElement) {
      document.exitFullscreen?.().catch(noop);
    } else {
      document.documentElement.requestFullscreen?.().catch(noop);
    }
  }, []);

  const menus = useMemo<MenuDef[]>(() => {
    const stats: MenuEntry = {
      kind: "check",
      id: "stats",
      label: t("menu-stats-dock"),
      checked: statsShown,
      disabled: !statsReady,
      onSelect: onToggleStats,
    };
    const profileRows: MenuEntry[] = (profiles?.names ?? []).map((name) => ({
      kind: "radio",
      id: `profile-${name}`,
      label: name,
      checked: name === profiles?.active,
      onSelect: () => {
        if (name === profiles?.active) return;
        profileSwitch(name).then(onSettingsSaved).catch(fail("profile switch"));
      },
    }));
    const collectionRows: MenuEntry[] = (collections?.names ?? []).map((name) => ({
      kind: "radio",
      id: `collection-${name}`,
      label: name,
      checked: name === collections?.active,
      onSelect: () => {
        if (name === collections?.active) return;
        collectionSwitch(name).catch(fail("collection switch"));
      },
    }));
    return [
      {
        id: "file",
        label: t("menu-file"),
        entries: [
          {
            kind: "item",
            id: "show-recordings",
            label: t("menu-file-show-recordings"),
            onSelect: () => {
              revealAppFolder("recordings").catch(fail("reveal recordings"));
            },
          },
          {
            kind: "item",
            id: "remux",
            label: t("menu-file-remux"),
            onSelect: () => onOpenControls("recordings"),
          },
          sep,
          {
            kind: "item",
            id: "settings",
            label: t("menu-file-settings"),
            onSelect: () => onOpenControls("settings"),
          },
          {
            kind: "item",
            id: "settings-folder",
            label: t("menu-file-show-settings-folder"),
            onSelect: () => {
              revealAppFolder("settings").catch(fail("reveal settings"));
            },
          },
          sep,
          {
            kind: "item",
            id: "exit",
            label: t("menu-file-exit"),
            // Routes through the existing quit guard (CAP-M23) like the OS
            // close button does.
            onSelect: () => {
              getCurrentWindow().close().catch(fail("close window"));
            },
          },
        ],
      },
      {
        id: "edit",
        label: t("menu-edit"),
        entries: [
          {
            kind: "item",
            id: "undo",
            label: t("menu-edit-undo"),
            shortcut: "Ctrl+Z",
            onSelect: () => {
              studioUndo().catch(fail("undo"));
            },
          },
          {
            kind: "item",
            id: "redo",
            label: t("menu-edit-redo"),
            shortcut: "Ctrl+Shift+Z",
            onSelect: () => {
              studioRedo().catch(fail("redo"));
            },
          },
          {
            kind: "item",
            id: "history",
            label: t("menu-edit-history"),
            onSelect: () => onOpenApp("history"),
          },
          sep,
          {
            kind: "item",
            id: "copy-transform",
            label: t("menu-edit-copy-transform"),
            disabled: !hasSelection,
            onSelect: onCopyTransform,
          },
          {
            kind: "item",
            id: "paste-transform",
            label: t("menu-edit-paste-transform"),
            disabled: !hasSelection || !clipboard.transform,
            onSelect: onPasteTransform,
          },
          {
            kind: "item",
            id: "copy-filters",
            label: t("menu-edit-copy-filters"),
            disabled: !canCopyFilters,
            onSelect: onCopyFilters,
          },
          {
            kind: "item",
            id: "paste-filters",
            label: t("menu-edit-paste-filters"),
            disabled: !hasSelection || !clipboard.filters?.length,
            onSelect: onPasteFilters,
          },
          sep,
          {
            kind: "item",
            id: "transform",
            label: t("menu-edit-transform"),
            disabled: !hasSelection,
            onSelect: onEditTransform,
          },
          sep,
          // No preview-lock backend yet — honest gray, not a dead toggle.
          {
            kind: "item",
            id: "lock-preview",
            label: t("menu-edit-lock-preview"),
            disabled: true,
            onSelect: noop,
          },
        ],
      },
      {
        id: "view",
        label: t("menu-view"),
        entries: [
          {
            kind: "check",
            id: "fullscreen",
            label: t("menu-view-fullscreen"),
            checked: fullscreen,
            onSelect: toggleFullscreen,
          },
          sep,
          stats,
          sep,
          {
            kind: "item",
            id: "multiview",
            label: t("menu-view-multiview"),
            onSelect: () => onOpenApp("multiview"),
          },
          {
            kind: "item",
            id: "projector",
            label: t("menu-view-projectors"),
            onSelect: () => onOpenApp("projector"),
          },
          {
            kind: "item",
            id: "source-health",
            label: t("menu-view-source-health"),
            onSelect: () => onOpenApp("sourceHealth"),
          },
          sep,
          {
            kind: "item",
            id: "still",
            label: t("menu-view-still"),
            onSelect: () => {
              captureStill({ kind: "program" }).catch(fail("still grab"));
            },
          },
        ],
      },
      {
        id: "docks",
        label: t("menu-docks"),
        entries: [
          {
            kind: "item",
            id: "browser",
            label: t("menu-docks-browser"),
            onSelect: () => onOpenControls("docks"),
          },
          sep,
          stats,
          sep,
          // The docks are a fixed grid today — no lock/reset to offer yet.
          { kind: "item", id: "lock", label: t("menu-docks-lock"), disabled: true, onSelect: noop },
          {
            kind: "item",
            id: "reset",
            label: t("menu-docks-reset"),
            disabled: true,
            onSelect: noop,
          },
        ],
      },
      {
        id: "profile",
        label: t("menu-profile"),
        entries: [
          {
            kind: "item",
            id: "manage",
            label: t("menu-profile-manage"),
            onSelect: () => onOpenControls("workspace"),
          },
          ...(profileRows.length ? [sep, ...profileRows] : []),
          sep,
          // No rename/remove/import/export backends exist yet.
          { kind: "item", id: "rename", label: t("menu-rename"), disabled: true, onSelect: noop },
          { kind: "item", id: "remove", label: t("menu-remove"), disabled: true, onSelect: noop },
          { kind: "item", id: "import", label: t("menu-import"), disabled: true, onSelect: noop },
          { kind: "item", id: "export", label: t("menu-export"), disabled: true, onSelect: noop },
        ],
      },
      {
        id: "collection",
        label: t("menu-collection"),
        entries: [
          {
            kind: "item",
            id: "manage",
            label: t("menu-collection-manage"),
            onSelect: () => onOpenControls("workspace"),
          },
          {
            kind: "item",
            id: "import-obs",
            label: t("menu-collection-import-obs"),
            onSelect: () => onOpenControls("workspace"),
          },
          {
            kind: "item",
            id: "missing",
            label: t("menu-collection-missing"),
            onSelect: () => onOpenApp("missingFiles"),
          },
          ...(collectionRows.length ? [sep, ...collectionRows] : []),
          sep,
          { kind: "item", id: "rename", label: t("menu-rename"), disabled: true, onSelect: noop },
          { kind: "item", id: "remove", label: t("menu-remove"), disabled: true, onSelect: noop },
          { kind: "item", id: "export", label: t("menu-export"), disabled: true, onSelect: noop },
        ],
      },
      {
        id: "tools",
        label: t("menu-tools"),
        entries: [
          // The wizard only runs on first launch; no re-run path exists yet.
          {
            kind: "item",
            id: "wizard",
            label: t("menu-tools-wizard"),
            disabled: true,
            title: t("menu-tools-wizard-title"),
            onSelect: noop,
          },
          {
            kind: "item",
            id: "automation",
            label: t("menu-tools-automation"),
            onSelect: () => onOpenControls("automation"),
          },
          {
            kind: "item",
            id: "rundown",
            label: t("menu-tools-rundown"),
            onSelect: () => onOpenControls("rundown"),
          },
          {
            kind: "item",
            id: "hotkeys",
            label: t("menu-tools-hotkeys"),
            onSelect: () => onOpenApp("hotkeyAudit"),
          },
          {
            kind: "item",
            id: "av-sync",
            label: t("menu-tools-av-sync"),
            onSelect: () => onOpenApp("avSync"),
          },
          {
            kind: "item",
            id: "scripts",
            label: t("menu-tools-scripts"),
            onSelect: () => onOpenControls("scripts"),
          },
          {
            kind: "item",
            id: "components",
            label: t("menu-tools-components"),
            onSelect: () => onOpenControls("components"),
          },
          {
            kind: "item",
            id: "midi",
            label: t("menu-tools-midi"),
            onSelect: () => onOpenControls("midi"),
          },
          {
            kind: "item",
            id: "ptz",
            label: t("menu-tools-ptz"),
            onSelect: () => onOpenControls("ptz"),
          },
          {
            kind: "item",
            id: "downstream",
            label: t("menu-tools-downstream"),
            onSelect: () => onOpenApp("downstream"),
          },
          {
            kind: "item",
            id: "transition-rules",
            label: t("menu-tools-transition-rules"),
            onSelect: () => onOpenApp("transitionRules"),
          },
          sep,
          {
            kind: "item",
            id: "remote",
            label: t("menu-tools-remote"),
            onSelect: () => onOpenControls("remote"),
          },
          {
            kind: "item",
            id: "panel",
            label: t("menu-tools-panel"),
            onSelect: () => onOpenControls("panel"),
          },
        ],
      },
      {
        id: "help",
        label: t("menu-help"),
        entries: [
          { kind: "link", id: "portal", label: t("menu-help-portal"), href: HELP_URL },
          { kind: "link", id: "website", label: t("menu-help-website"), href: WEBSITE_URL },
          DISCORD_URL
            ? { kind: "link", id: "discord", label: t("menu-help-discord"), href: DISCORD_URL }
            : {
                kind: "item",
                id: "discord",
                label: t("menu-help-discord"),
                disabled: true,
                onSelect: noop,
              },
          sep,
          {
            kind: "item",
            id: "bug",
            label: t("menu-help-bug"),
            onSelect: () => onOpenControls("bug"),
          },
          {
            kind: "item",
            id: "updates",
            label: t("menu-help-updates"),
            onSelect: () => onOpenControls("updates"),
          },
          sep,
          {
            kind: "item",
            id: "whats-new",
            label: t("menu-help-whats-new"),
            onSelect: () => onOpenControls("whatsnew"),
          },
          {
            kind: "item",
            id: "about",
            label: t("menu-help-about"),
            onSelect: () => onOpenControls("about"),
          },
          sep,
          {
            kind: "item",
            id: "more-apps",
            label: t("menu-help-more-apps"),
            onSelect: () => onOpenControls("moreapps"),
          },
        ],
      },
    ];
  }, [
    t,
    clipboard,
    hasSelection,
    canCopyFilters,
    statsShown,
    statsReady,
    fullscreen,
    profiles,
    collections,
    toggleFullscreen,
    onOpenControls,
    onOpenApp,
    onCopyTransform,
    onPasteTransform,
    onCopyFilters,
    onPasteFilters,
    onEditTransform,
    onToggleStats,
    onSettingsSaved,
  ]);

  const openIdx = menus.findIndex((menu) => menu.id === state.open);
  const openMenu = openIdx >= 0 ? menus[openIdx] : null;
  const focusable = openMenu ? openMenu.entries.filter((entry) => entry.kind !== "separator") : [];
  const menuOpen = openMenu !== null;

  // Lazy loads: the lists are fetched when their menu OPENS, never on mount.
  useEffect(() => {
    if (state.open !== "profile") return;
    let alive = true;
    profilesList()
      .then((list) => {
        if (alive) setProfiles(list);
      })
      .catch(noop);
    return () => {
      alive = false;
    };
  }, [state.open]);
  useEffect(() => {
    if (state.open !== "collection") return;
    let alive = true;
    collectionsList()
      .then((list) => {
        if (alive) setCollections(list);
      })
      .catch(noop);
    return () => {
      alive = false;
    };
  }, [state.open]);

  // Hide the native GPU preview while any menu is open — the dropdown may
  // overlap its region, and the native child window paints over the webview.
  useEffect(() => {
    if (!menuOpen) return;
    return pushModal();
  }, [menuOpen]);

  const openMenuAt = (index: number, focus: number | null) => {
    const wrapped = (index + menus.length) % menus.length;
    const trigger = triggerRefs.current[wrapped];
    const menu = menus[wrapped];
    if (!trigger || !menu) return;
    const rect = trigger.getBoundingClientRect();
    setAnchor({ left: rect.left, top: rect.bottom + 4 });
    setState({ open: menu.id, focus });
    setBarFocus(wrapped);
  };

  // The latest close, held in a ref (the useDismiss pattern) so the document
  // listeners below subscribe once per open, not once per render.
  const closeRef = useRef<(refocus: boolean) => void>(noop);
  useEffect(() => {
    closeRef.current = (refocus: boolean) => {
      if (refocus && openIdx >= 0) triggerRefs.current[openIdx]?.focus();
      setState({ open: null, focus: null });
    };
  });
  const close = (refocus: boolean) => closeRef.current(refocus);

  // Dismissal, mirroring `useDismiss`: capture-phase pointerdown outside the
  // bar AND the portalled panel (one ref cannot span a portal), and Escape —
  // stopped in capture so an underlying dialog doesn't also close.
  useEffect(() => {
    if (!menuOpen) return;
    const onPointerDown = (event: PointerEvent) => {
      const target = event.target;
      if (!(target instanceof Node)) return;
      if (barRef.current?.contains(target) || panelRef.current?.contains(target)) return;
      closeRef.current(false);
    };
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Escape") return;
      event.stopPropagation();
      closeRef.current(true);
    };
    document.addEventListener("pointerdown", onPointerDown, true);
    document.addEventListener("keydown", onKeyDown, true);
    return () => {
      document.removeEventListener("pointerdown", onPointerDown, true);
      document.removeEventListener("keydown", onKeyDown, true);
    };
  }, [menuOpen]);

  // Roving focus inside the open menu.
  useEffect(() => {
    if (!menuOpen || state.focus === null) return;
    const index = state.focus === -1 ? focusable.length - 1 : state.focus;
    itemRefs.current[index]?.focus();
  }, [menuOpen, state.focus, focusable.length]);

  // Keep the dropdown on-screen: a right-edge menu must not spill past the
  // window. Measured after paint because the width isn't known before it —
  // and a ResizeObserver re-clamps when the Profile / Scene Collection rows
  // arrive lazily and widen the panel (a one-shot measure missed them). Reset
  // to the anchor first so a menu that doesn't spill clears a prior menu's
  // offset on the shared panel element.
  useLayoutEffect(() => {
    const panel = panelRef.current;
    if (!panel || !anchor) return;
    const clamp = () => {
      panel.style.left = `${anchor.left}px`;
      const spill = anchor.left + panel.offsetWidth + 8 - window.innerWidth;
      if (spill > 0) panel.style.left = `${Math.max(8, anchor.left - spill)}px`;
    };
    clamp();
    const observer = new ResizeObserver(clamp);
    observer.observe(panel);
    return () => observer.disconnect();
  }, [anchor, state.open]);

  const activate = (entry: MenuEntry) => {
    if (entry.kind === "separator") return;
    if (entry.kind !== "link" && entry.kind !== "radio" && entry.disabled) return;
    // Close first (the palette's rule): a dialog the action opens must not
    // fight the menu for focus or the modal stack.
    close(true);
    // This Tauri webview never follows an `<a target="_blank">` out to the OS
    // browser (no opener plugin, external nav is blocked), so link rows go dead
    // unless we hand the URL to the OS opener command explicitly.
    if (entry.kind === "link") {
      void openExternal(entry.href);
      return;
    }
    entry.onSelect();
  };

  const onTriggerKeyDown = (event: React.KeyboardEvent, index: number) => {
    const step = (delta: number) => {
      const next = (index + delta + menus.length) % menus.length;
      if (menuOpen) {
        openMenuAt(next, 0);
      } else {
        setBarFocus(next);
        triggerRefs.current[next]?.focus();
      }
    };
    switch (event.key) {
      case "ArrowRight":
        event.preventDefault();
        step(1);
        break;
      case "ArrowLeft":
        event.preventDefault();
        step(-1);
        break;
      case "ArrowDown":
      case "Enter":
      case " ":
        event.preventDefault();
        openMenuAt(index, 0);
        break;
      case "ArrowUp":
        event.preventDefault();
        openMenuAt(index, -1);
        break;
      default:
        break;
    }
  };

  const onPanelKeyDown = (event: React.KeyboardEvent) => {
    const count = focusable.length;
    const resolve = (focus: number) => (focus === -1 ? count - 1 : focus);
    switch (event.key) {
      case "ArrowDown":
        event.preventDefault();
        setState((s) => ({
          ...s,
          focus: count ? (s.focus === null ? 0 : (resolve(s.focus) + 1) % count) : null,
        }));
        break;
      case "ArrowUp":
        event.preventDefault();
        setState((s) => ({
          ...s,
          focus: count
            ? s.focus === null
              ? count - 1
              : (resolve(s.focus) - 1 + count) % count
            : null,
        }));
        break;
      case "Home":
        event.preventDefault();
        setState((s) => ({ ...s, focus: 0 }));
        break;
      case "End":
        event.preventDefault();
        setState((s) => ({ ...s, focus: count - 1 }));
        break;
      case "ArrowRight":
        event.preventDefault();
        openMenuAt(openIdx + 1, 0);
        break;
      case "ArrowLeft":
        event.preventDefault();
        openMenuAt(openIdx - 1, 0);
        break;
      case " ": {
        // Anchors don't activate on Space; buttons re-fire on keyup. One
        // explicit click covers both without double-running.
        event.preventDefault();
        (event.target as HTMLElement).click();
        break;
      }
      case "Tab":
        // Per the menubar pattern, Tab leaves the menu — let the default move
        // focus, just close behind it.
        close(false);
        break;
      default:
        break;
    }
  };

  const itemClass = (disabled: boolean) =>
    `flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-xs no-underline outline-none focus:bg-havoc-accent/20 ${
      disabled ? "cursor-default text-havoc-muted opacity-50" : "text-havoc-text"
    }`;

  // The check/radio glyph column is always rendered so labels line up.
  const glyph = (entry: MenuEntry) => {
    if (entry.kind === "check" || entry.kind === "radio") return entry.checked ? "✓" : "";
    return "";
  };

  // Rows carry their roving-focus index (separators have none), computed with
  // a plain loop — no counter reassignment inside the JSX map.
  const rows: { entry: MenuEntry; refIndex: number }[] = [];
  let nextFocus = 0;
  for (const entry of openMenu?.entries ?? []) {
    if (entry.kind === "separator") {
      rows.push({ entry, refIndex: -1 });
    } else {
      rows.push({ entry, refIndex: nextFocus });
      nextFocus += 1;
    }
  }

  return (
    <div ref={barRef} role="menubar" aria-label={t("menu-bar-label")} className="flex items-center">
      {menus.map((menu, index) => (
        <button
          key={menu.id}
          type="button"
          ref={(el) => {
            triggerRefs.current[index] = el;
          }}
          role="menuitem"
          aria-haspopup="menu"
          aria-expanded={state.open === menu.id}
          tabIndex={index === barFocus ? 0 : -1}
          onClick={() => (state.open === menu.id ? close(false) : openMenuAt(index, null))}
          onPointerEnter={() => {
            if (menuOpen && state.open !== menu.id) openMenuAt(index, null);
          }}
          onFocus={() => setBarFocus(index)}
          onKeyDown={(event) => onTriggerKeyDown(event, index)}
          className={`rounded-md px-2 py-0.5 text-xs transition-colors ${
            state.open === menu.id
              ? "bg-white/10 text-havoc-text"
              : "text-havoc-muted hover:text-havoc-text"
          }`}
        >
          {menu.label}
        </button>
      ))}
      {openMenu &&
        anchor &&
        createPortal(
          <div
            ref={panelRef}
            role="menu"
            aria-label={openMenu.label}
            style={{ left: anchor.left, top: anchor.top }}
            onKeyDown={onPanelKeyDown}
            className="fixed z-50 max-h-[70vh] min-w-56 overflow-y-auto rounded-lg border border-white/10 bg-havoc-panel p-1 shadow-2xl"
          >
            {rows.map(({ entry, refIndex }, index) => {
              if (entry.kind === "separator") {
                return (
                  <div key={`sep-${index}`} role="separator" className="my-1 h-px bg-white/10" />
                );
              }
              const disabled =
                entry.kind !== "link" && entry.kind !== "radio" && Boolean(entry.disabled);
              const setRef = (el: HTMLElement | null) => {
                itemRefs.current[refIndex] = el;
              };
              const onPointerEnter = () => setState((s) => ({ ...s, focus: refIndex }));
              if (entry.kind === "link") {
                return (
                  <a
                    key={entry.id}
                    ref={setRef}
                    role="menuitem"
                    href={entry.href}
                    rel="noreferrer"
                    tabIndex={-1}
                    onClick={(event) => {
                      // The webview won't navigate here anyway; keep the href
                      // for context but route opening through the OS opener.
                      event.preventDefault();
                      activate(entry);
                    }}
                    onPointerEnter={onPointerEnter}
                    className={itemClass(false)}
                  >
                    <span aria-hidden="true" className="w-4 shrink-0" />
                    <span className="truncate">{entry.label}</span>
                  </a>
                );
              }
              return (
                <button
                  key={entry.id}
                  type="button"
                  ref={setRef}
                  role={
                    entry.kind === "check"
                      ? "menuitemcheckbox"
                      : entry.kind === "radio"
                        ? "menuitemradio"
                        : "menuitem"
                  }
                  aria-checked={
                    entry.kind === "check" || entry.kind === "radio" ? entry.checked : undefined
                  }
                  aria-disabled={disabled || undefined}
                  tabIndex={-1}
                  title={entry.kind === "item" ? entry.title : undefined}
                  onClick={() => activate(entry)}
                  onPointerEnter={onPointerEnter}
                  className={itemClass(disabled)}
                >
                  <span aria-hidden="true" className="w-4 shrink-0 text-havoc-accent">
                    {glyph(entry)}
                  </span>
                  <span className="truncate">{entry.label}</span>
                  {entry.kind === "item" && entry.shortcut && (
                    <span className="ml-auto pl-6 text-[10px] text-havoc-muted">
                      {entry.shortcut}
                    </span>
                  )}
                </button>
              );
            })}
          </div>,
          document.body,
        )}
    </div>
  );
}
