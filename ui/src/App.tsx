import { useCallback, useEffect, useMemo, useRef, useState } from "react";

import { initLocale, useT } from "./i18n/t";
import { applyTheme } from "./theme/theme";

import {
  eulaStatus,
  health,
  remotePendingInvite,
  settingsGet,
  settingsSet,
  studioAddExistingSource,
  studioAddItem,
  studioGet,
  studioRemoveItem,
  studioReorderItem,
  studioZoomSet,
  studioSetFocus,
  studioSetItemLocked,
  studioSetStudioMode,
  studioSetItemTransform,
  studioSetItemVisible,
  studioSelectScene,
  studioTransition,
  studioUndo,
  studioRedo,
  studioSetItemTransforms,
  captureStill,
  collectionMissingFiles,
  replaySave,
  recordingAddMarker,
  salvagePending,
  studioPanicSet,
  studioPasteFilters,
} from "./api/commands";
import {
  onAlarm,
  onAudio,
  onEncoderFallback,
  onProgram,
  onQuitGuard,
  onRemoteInvite,
  onStillError,
  onStillSaved,
  onZoomPreset,
  onStudio,
} from "./api/events";
import type {
  Alarm,
  AudioLevelsPayload,
  EncoderFallback,
  EulaStatus,
  Health,
  ItemId,
  ProgramStatus,
  QuitConsequences,
  Settings,
  SourceId,
  SourceSettings,
  StudioDto,
  Transform,
  Scene,
  SceneItem,
  Collection,
} from "./api/types";
import { kindHasAudio } from "./api/types";
import { AudioFiltersDialog } from "./components/AudioFiltersDialog";
import { CommandPalette } from "./components/CommandPalette";
import { EditTransformDialog } from "./components/EditTransformDialog";
import { MenuBar, type AppMenuDialog } from "./components/MenuBar";
import { StatusAnnouncer } from "./components/StatusAnnouncer";
import { clipboardSnapshot, copyFilters, copyTransform } from "./lib/clipboard";
import type { Command } from "./lib/commands";
import { constrainPaste } from "./lib/constrain";
import { effectiveSourceSize } from "./lib/transform";
import { FiltersDialog } from "./components/FiltersDialog";
import { PropertiesDialog } from "./components/PropertiesDialog";
import { spikeSetJoinPrefill } from "./remote/spike";
import { ControlsDock, type ControlsDialogKind } from "./panels/ControlsDock";
import { EulaGate } from "./panels/EulaGate";
import { FirstRunWizard } from "./panels/FirstRunWizard";
import { MixerDock } from "./panels/MixerDock";
import { PreviewPanel } from "./panels/PreviewPanel";
import { RemoteSessionBar } from "./panels/RemoteSessionBar";
import { ScenesRail } from "./panels/ScenesRail";
import { SourcesRail } from "./panels/SourcesRail";
import { StatsDock } from "./panels/StatsDock";
import { StudioPreviewPane } from "./panels/StudioPreviewPane";
import { HistoryDialog } from "./panels/HistoryDialog";
import { MissingFilesDialog } from "./panels/MissingFilesDialog";
import { AvSyncDialog } from "./panels/AvSyncDialog";
import { HotkeyAuditDialog } from "./panels/HotkeyAuditDialog";
import { MultiviewDialog } from "./panels/MultiviewDialog";
import { PanicBanner } from "./panels/PanicBanner";
import { ProjectorDialog } from "./panels/ProjectorDialog";
import { QuitGuardDialog } from "./panels/QuitGuardDialog";
import { SalvageDialog } from "./panels/SalvageDialog";
import { SourceHealthDialog } from "./panels/SourceHealthDialog";
import { VerticalCanvasDialog } from "./panels/VerticalCanvasDialog";

type OpenDialog =
  | { kind: "filters"; itemId: ItemId }
  | { kind: "properties"; sourceId: SourceId }
  | { kind: "audioFilters"; sourceId: SourceId }
  | { kind: "vertical" }
  | { kind: "history" }
  | { kind: "editTransform"; itemId: ItemId }
  | { kind: "multiview" }
  | { kind: "projector" }
  | { kind: "sourceHealth" }
  | { kind: "avSync" }
  | { kind: "hotkeyAudit" }
  | { kind: "missingFiles" }
  | null;

/** The Freally Capture studio shell: preview + rails + bottom docks. */
export default function App() {
  // Subscribes this tree to language changes; `t` is the same function either way.
  const t = useT();
  const [core, setCore] = useState<Health | null>(null);
  const [coreError, setCoreError] = useState(false);
  // First-run EULA gate: `null` while loading, then the status. Until the
  // current version is accepted, the studio does not render.
  const [eula, setEula] = useState<EulaStatus | null>(null);
  const [settings, setSettings] = useState<Settings | null>(null);
  // The stats dock renders only after settings settle (loaded or failed), so
  // a persisted "off" never flashes visible on launch.
  const [settingsSettled, setSettingsSettled] = useState(false);
  const [saveError, setSaveError] = useState<string | null>(null);
  // A transient still-frame-grab confirmation (CAP-M08).
  const [stillToast, setStillToast] = useState<{ ok: boolean; text: string } | null>(null);
  // Quit guard (CAP-M23): non-null while the confirm is up.
  const [quitGuard, setQuitGuard] = useState<QuitConsequences | null>(null);
  // Salvage prompt (CAP-M11): interrupted recordings from an unclean exit.
  const [salvagePaths, setSalvagePaths] = useState<string[] | null>(null);
  // Encoder failover toast (CAP-M12) — transient; the sticky note lives in
  // the stats dock.
  const [fallbackToast, setFallbackToast] = useState<EncoderFallback | null>(null);
  // Broadcast-safety alarms (CAP-M10): the active set, keyed by kind.
  // Dismissing acknowledges (removes) one; it returns on its next raise.
  const [alarms, setAlarms] = useState<Record<string, Alarm>>({});

  const [studio, setStudio] = useState<StudioDto | null>(null);
  const [program, setProgram] = useState<ProgramStatus | null>(null);
  const [audio, setAudio] = useState<AudioLevelsPayload | null>(null);
  // Multi-selection (CAP-M04 follow-on): an ordered list of item ids; the
  // last-clicked is the "primary" that carries the transform handles. A plain
  // single selection is just a one-element list.
  const [selection, setSelection] = useState<ItemId[]>([]);
  const selectSingle = useCallback((id: ItemId | null) => setSelection(id ? [id] : []), []);
  const toggleSelect = useCallback(
    (id: ItemId) =>
      setSelection((cur) => (cur.includes(id) ? cur.filter((x) => x !== id) : [...cur, id])),
    [],
  );
  const selectMany = useCallback((ids: ItemId[]) => setSelection(ids), []);
  const [dialog, setDialog] = useState<OpenDialog>(null);
  const [paletteOpen, setPaletteOpen] = useState(false);
  // The wizard is dismissible from its own close button, so its visibility is
  // local state seeded from settings — not `!settings.completedOnboarding`
  // directly, which would flicker it back while the save round-trips.
  const [wizardDismissed, setWizardDismissed] = useState(false);
  // Ignore stale event echoes while a drag streams newer transforms.
  const localRevision = useRef(0);

  useEffect(() => {
    let cancelled = false;
    eulaStatus()
      .then((status) => {
        if (!cancelled) setEula(status);
      })
      // No bridge (plain browser / tests): don't block the app on the gate.
      .catch(() => {
        if (!cancelled) setEula({ version: "", text: "", accepted: true });
      });
    health()
      .then((report) => {
        if (!cancelled) setCore(report);
      })
      .catch(() => {
        // Plain browser / test runs have no Tauri core behind the bridge.
        if (!cancelled) setCoreError(true);
      });
    settingsGet()
      .then((loaded) => {
        if (!cancelled) {
          // Language and palette are cosmetic; the settings are not. A throw in
          // either would land in the `.catch` below and leave `settings` null,
          // which silently disables every control that reads it — the studio
          // would look alive and refuse to save anything.
          try {
            // `"auto"` follows the OS, an explicit tag wins. Also stamps
            // <html lang/dir>, which is what actually flips Arabic to RTL.
            initLocale(loaded.language);
            // Paint before the first render, so a light theme never flashes dark.
            applyTheme(loaded.theme);
          } catch (err) {
            console.error("could not apply the language or theme:", err);
          }
          setSettings(loaded);
          setSettingsSettled(true);
        }
      })
      .catch(() => {
        if (!cancelled) setSettingsSettled(true);
      });
    studioGet()
      .then((dto) => {
        if (!cancelled) {
          localRevision.current = dto.revision;
          setStudio(dto);
        }
      })
      .catch(() => undefined);
    const unlistenStudio = onStudio((dto) => {
      if (cancelled) return;
      // Optimistic local edits (drags) may be ahead of the echo.
      if (dto.revision >= localRevision.current) {
        localRevision.current = dto.revision;
        setStudio(dto);
      }
    }).catch(() => undefined);
    const unlistenProgram = onProgram((status) => {
      if (!cancelled) setProgram(status);
    }).catch(() => undefined);
    const unlistenAudio = onAudio((levels) => {
      if (!cancelled) setAudio(levels);
    }).catch(() => undefined);
    // A clicked freally:// invite (OS deep link) → the session bar's join
    // prompt. Held, never auto-joined.
    const unlistenInvite = onRemoteInvite((url) => {
      if (!cancelled) spikeSetJoinPrefill(url);
    }).catch(() => undefined);
    // Still-frame grab (CAP-M08) confirmations.
    const unlistenStillSaved = onStillSaved((path) => {
      if (!cancelled) setStillToast({ ok: true, text: path.split(/[\\/]/).pop() ?? path });
    }).catch(() => undefined);
    const unlistenStillError = onStillError((message) => {
      if (!cancelled) setStillToast({ ok: false, text: message });
    }).catch(() => undefined);
    // Quit guard (CAP-M23): a close attempt while live output runs.
    const unlistenQuitGuard = onQuitGuard((pending) => {
      if (!cancelled) setQuitGuard(pending);
    }).catch(() => undefined);
    // Encoder failover (CAP-M12): the honest mid-session toast.
    const unlistenFallback = onEncoderFallback((fallback) => {
      if (!cancelled) setFallbackToast(fallback);
    }).catch(() => undefined);
    // Broadcast-safety alarms (CAP-M10): keep the active set current.
    const unlistenAlarm = onAlarm((alarm) => {
      if (cancelled) return;
      setAlarms((current) => {
        const next = { ...current };
        if (alarm.active) next[alarm.kind] = alarm;
        else delete next[alarm.kind];
        return next;
      });
    }).catch(() => undefined);
    // A cold-start invite (the link LAUNCHED the app) fired before this
    // listener existed — pick it up once.
    remotePendingInvite()
      .then((url) => {
        if (!cancelled && url) spikeSetJoinPrefill(url);
      })
      .catch(() => undefined);
    return () => {
      cancelled = true;
      void unlistenStudio.then((fn) => fn?.());
      void unlistenProgram.then((fn) => fn?.());
      void unlistenAudio.then((fn) => fn?.());
      void unlistenInvite.then((fn) => fn?.());
      void unlistenStillSaved.then((fn) => fn?.());
      void unlistenStillError.then((fn) => fn?.());
      void unlistenQuitGuard.then((fn) => fn?.());
      void unlistenFallback.then((fn) => fn?.());
      void unlistenAlarm.then((fn) => fn?.());
    };
  }, []);

  // "Before load" (CAP-M03): once the studio is up and past onboarding, scan the
  // active collection for broken file references and surface the doctor if any.
  // One-shot per session, and never over the wizard or an already-open dialog.
  const checkedMissing = useRef(false);
  useEffect(() => {
    if (checkedMissing.current || !studio || !settingsSettled) return;
    if (settings && !settings.completedOnboarding && !wizardDismissed) return;
    checkedMissing.current = true;
    collectionMissingFiles()
      .then((list) => {
        if (list.length > 0) setDialog((current) => current ?? { kind: "missingFiles" });
      })
      .catch(() => undefined);
  }, [studio, settingsSettled, settings, wizardDismissed]);

  // Salvage prompt (CAP-M11): the previous session ended uncleanly with
  // recordings still being written — offer to repair them. Same one-shot
  // gating as the doctor.
  const checkedSalvage = useRef(false);
  useEffect(() => {
    if (checkedSalvage.current || !studio || !settingsSettled) return;
    if (settings && !settings.completedOnboarding && !wizardDismissed) return;
    checkedSalvage.current = true;
    salvagePending()
      .then((paths) => {
        if (paths.length > 0) setSalvagePaths(paths);
      })
      .catch(() => undefined);
  }, [studio, settingsSettled, settings, wizardDismissed]);

  const collection = studio?.collection ?? null;
  const studioMode = studio?.studioMode ?? null;
  const activeScene = useMemo(
    () => collection?.scenes.find((scene) => scene.id === collection.activeScene) ?? null,
    [collection],
  );

  // Selection follows reality (derived, not synced): it only counts while it
  // names an item of the active scene.
  const selectedItem = selection.length ? selection[selection.length - 1] : null;

  // Punch-in zoom presets (CAP-N71): a hotkey broadcasts the factor; the UI
  // picks the lens target — the selected visible item if it can zoom, else
  // the top-most visible screen capture. Refs keep the resolver current
  // while the event subscription registers once.
  const zoomTargetRef = useRef<{ scene: Scene | null; selected: ItemId | null }>({
    scene: null,
    selected: null,
  });
  const zoomSourcesRef = useRef<Collection["sources"]>([]);
  useEffect(() => {
    zoomTargetRef.current = { scene: activeScene, selected: selectedItem };
    zoomSourcesRef.current = collection?.sources ?? [];
  });
  useEffect(() => {
    let cancelled = false;
    const unlisten = onZoomPreset((factor) => {
      if (cancelled) return;
      const { scene, selected } = zoomTargetRef.current;
      if (!scene) return;
      const kindOf = (item: SceneItem) =>
        zoomSourcesRef.current.find((source) => source.id === item.source)?.kind ?? "";
      const audioKinds = new Set(["audioInput", "audioOutput", "appAudio", "testTone"]);
      const screenKinds = new Set(["display", "window", "portal"]);
      let target = scene.items.find(
        (item) =>
          item.id === selected && !item.backdrop && item.visible && !audioKinds.has(kindOf(item)),
      );
      if (!target) {
        for (let index = scene.items.length - 1; index >= 0; index -= 1) {
          const item = scene.items[index];
          if (!item.backdrop && item.visible && screenKinds.has(kindOf(item))) {
            target = item;
            break;
          }
        }
      }
      if (target) {
        studioZoomSet(target.id, factor).catch((err) => console.error("zoom preset failed:", err));
      }
    }).catch(() => undefined);
    return () => {
      cancelled = true;
      void unlisten.then((fn) => fn?.());
    };
  }, []);
  const effectiveSelection =
    selectedItem && activeScene?.items.some((item) => item.id === selectedItem)
      ? selectedItem
      : null;
  // The multi-selection that actually exists in the active scene (order kept).
  const effectiveSelectionList = useMemo(
    () => selection.filter((id) => activeScene?.items.some((item) => item.id === id)),
    [selection, activeScene],
  );

  // The selected item's live object — the Edit menu's copy/paste actions and
  // their gating read from it.
  const selectedSceneItem = useMemo(
    () =>
      effectiveSelection
        ? (activeScene?.items.find((item) => item.id === effectiveSelection) ?? null)
        : null,
    [effectiveSelection, activeScene],
  );

  // Menu-bar seams: the Controls dock parks its dialog opener in this ref
  // while mounted; App-owned dialogs open through `openAppDialog`.
  const controlsOpener = useRef<((kind: ControlsDialogKind) => void) | null>(null);
  const openControlsDialog = useCallback((kind: ControlsDialogKind) => {
    controlsOpener.current?.(kind);
  }, []);
  const openAppDialog = useCallback((kind: AppMenuDialog) => {
    setDialog({ kind });
  }, []);

  // Edit-menu actions (CAP-M05 parity): the same clipboard + apply paths the
  // Edit Transform and Filters dialogs use, dispatched from the menu bar.
  const menuCopyTransform = useCallback(() => {
    if (selectedSceneItem) copyTransform(selectedSceneItem.transform);
  }, [selectedSceneItem]);
  const menuPasteTransform = useCallback(() => {
    const clip = clipboardSnapshot().transform;
    if (!clip || !activeScene || !selectedSceneItem) return;
    const status = program?.sources[selectedSceneItem.source];
    const source =
      status?.width && status?.height
        ? effectiveSourceSize(status.width, status.height, selectedSceneItem.filters)
        : null;
    const canvas = { w: collection?.canvasWidth ?? 1920, h: collection?.canvasHeight ?? 1080 };
    // The dialog's Paste and this share `constrainPaste` so they can't diverge.
    studioSetItemTransform(
      activeScene.id,
      selectedSceneItem.id,
      constrainPaste(clip, source, canvas),
    ).catch((err) => console.error("paste transform failed:", err));
  }, [activeScene, selectedSceneItem, program, collection]);
  const menuCopyFilters = useCallback(() => {
    if (selectedSceneItem && selectedSceneItem.filters.length > 0)
      copyFilters(selectedSceneItem.filters);
  }, [selectedSceneItem]);
  const menuPasteFilters = useCallback(() => {
    const filters = clipboardSnapshot().filters;
    if (!filters?.length || !activeScene || !selectedSceneItem) return;
    studioPasteFilters(activeScene.id, selectedSceneItem.id, filters).catch((err) =>
      console.error("paste filters failed:", err),
    );
  }, [activeScene, selectedSceneItem]);
  const menuEditTransform = useCallback(() => {
    if (effectiveSelection) setDialog({ kind: "editTransform", itemId: effectiveSelection });
  }, [effectiveSelection]);

  /**
   * What the palette can reach (TASK-904). Scenes and sources come from the live
   * collection, so the list is always the studio's truth rather than a snapshot.
   * Labels are translated here — the palette never calls `t` on caller strings.
   */
  const paletteCommands = useMemo<Command[]>(() => {
    const list: Command[] = [];

    for (const scene of collection?.scenes ?? []) {
      list.push({
        id: `scene-${scene.id}`,
        group: t("palette-group-scenes"),
        label: scene.name,
        keywords: "scene switch",
        run: () => {
          studioSelectScene(scene.id).catch((err) => console.error("scene switch failed:", err));
        },
      });
    }

    for (const item of activeScene?.items ?? []) {
      const source = collection?.sources.find((candidate) => candidate.id === item.source);
      list.push({
        id: `item-${item.id}`,
        group: t("palette-group-sources"),
        label: source?.name ?? item.id,
        keywords: "source select",
        run: () => selectSingle(item.id),
      });
    }

    list.push(
      {
        id: "action-undo",
        group: t("palette-group-actions"),
        label: t("palette-undo"),
        keywords: "undo revert history",
        run: () => {
          studioUndo().catch((err) => console.error("undo failed:", err));
        },
      },
      {
        id: "action-redo",
        group: t("palette-group-actions"),
        label: t("palette-redo"),
        keywords: "redo history",
        run: () => {
          studioRedo().catch((err) => console.error("redo failed:", err));
        },
      },
      {
        id: "action-history",
        group: t("palette-group-actions"),
        label: t("palette-edit-history"),
        keywords: "history undo redo edits",
        run: () => setDialog({ kind: "history" }),
      },
      {
        id: "action-multiview",
        group: t("palette-group-actions"),
        label: t("palette-multiview"),
        keywords: "multiview monitor grid tally scenes",
        run: () => setDialog({ kind: "multiview" }),
      },
      {
        id: "action-projector",
        group: t("palette-group-actions"),
        label: t("palette-projector"),
        keywords: "projector fullscreen display monitor output",
        run: () => setDialog({ kind: "projector" }),
      },
      {
        id: "action-source-health",
        group: t("palette-group-actions"),
        label: t("palette-source-health"),
        keywords: "source health dashboard fps dropped frames restart capture status",
        run: () => setDialog({ kind: "sourceHealth" }),
      },
      {
        id: "action-av-sync",
        group: t("palette-group-actions"),
        label: t("palette-av-sync"),
        keywords: "av sync calibration offset audio video align flash beep lip sync latency",
        run: () => setDialog({ kind: "avSync" }),
      },
      {
        id: "action-hotkey-audit",
        group: t("palette-group-actions"),
        label: t("palette-hotkey-audit"),
        keywords: "hotkey map audit conflicts cheat sheet bindings keyboard shortcuts",
        run: () => setDialog({ kind: "hotkeyAudit" }),
      },
      {
        id: "action-panic",
        group: t("palette-group-actions"),
        label: t("palette-panic"),
        keywords: "panic privacy slate mute emergency hide sensitive",
        run: () => {
          studioPanicSet(true).catch((err) => console.error("panic failed:", err));
        },
      },
      {
        id: "action-doctor",
        group: t("palette-group-actions"),
        label: t("palette-doctor"),
        keywords: "missing files doctor relink media images broken paths locate",
        run: () => setDialog({ kind: "missingFiles" }),
      },
      {
        id: "action-still",
        group: t("palette-group-actions"),
        label: t("palette-still"),
        keywords: "still screenshot png grab capture frame",
        run: () => {
          captureStill({ kind: "program" }).catch((err) =>
            console.error("still grab failed:", err),
          );
        },
      },
      {
        id: "action-studio-mode",
        group: t("palette-group-actions"),
        label: studioMode ? t("studio-mode-leave") : t("studio-mode"),
        run: () => {
          studioSetStudioMode(!studioMode).catch((err) =>
            console.error("studio mode toggle failed:", err),
          );
        },
      },
      {
        id: "action-transition",
        group: t("palette-group-actions"),
        label: t("palette-transition"),
        run: () => {
          studioTransition().catch((err) => console.error("transition failed:", err));
        },
      },
      {
        id: "action-save-replay",
        group: t("palette-group-actions"),
        label: t("palette-save-replay"),
        run: () => {
          replaySave().catch((err) => console.error("save replay failed:", err));
        },
      },
      {
        id: "action-add-marker",
        group: t("palette-group-actions"),
        label: t("palette-add-marker"),
        run: () => {
          recordingAddMarker().catch((err) => console.error("add marker failed:", err));
        },
      },
      {
        id: "action-vertical",
        group: t("palette-group-actions"),
        label: t("palette-vertical-canvas"),
        run: () => setDialog({ kind: "vertical" }),
      },
    );

    if (effectiveSelection) {
      list.push({
        id: "action-edit-transform",
        group: t("palette-group-actions"),
        label: t("palette-edit-transform"),
        keywords: "transform position size rotation crop",
        run: () => setDialog({ kind: "editTransform", itemId: effectiveSelection }),
      });
    }

    return list;
  }, [collection, activeScene, studioMode, effectiveSelection, selectSingle, t]);

  // Highlight Speaker keyboard toggle: "F" focuses the selected item (fills
  // the canvas) or, when a focus is active, restores the layout — never while
  // typing in a field.
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "f" && event.key !== "F") return;
      if (event.ctrlKey || event.metaKey || event.altKey) return;
      const target = event.target as HTMLElement | null;
      if (target && (target.tagName === "INPUT" || target.tagName === "TEXTAREA")) return;
      if (!activeScene) return;
      const focused = activeScene.focus?.item ?? null;
      const next = focused ? null : effectiveSelection;
      if (!focused && !next) return;
      event.preventDefault();
      studioSetFocus(activeScene.id, next).catch((err) =>
        console.error("focus toggle failed:", err),
      );
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [activeScene, effectiveSelection]);

  // Ctrl/Cmd-K opens the palette (TASK-904). Unlike the "F" shortcut this DOES
  // fire while a field has focus — that is the point of a command palette — so
  // it only guards against the browser's own find-in-page binding.
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "k" && event.key !== "K") return;
      if (!event.ctrlKey && !event.metaKey) return;
      if (event.altKey || event.shiftKey) return;
      event.preventDefault();
      setPaletteOpen((open) => !open);
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, []);

  // Undo / redo (CAP-M01). Ctrl/Cmd+Z undoes; Ctrl/Cmd+Shift+Z or Ctrl+Y
  // redoes. Suppressed while a text field is focused so the browser's own
  // text undo keeps working inside a rename/properties box.
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      const key = event.key.toLowerCase();
      if (key !== "z" && key !== "y") return;
      if (!event.ctrlKey && !event.metaKey) return;
      if (event.altKey) return;
      const target = event.target as HTMLElement | null;
      if (
        target &&
        (target.tagName === "INPUT" || target.tagName === "TEXTAREA" || target.isContentEditable)
      )
        return;
      const redo = key === "y" || event.shiftKey;
      event.preventDefault();
      (redo ? studioRedo() : studioUndo()).catch((err) => console.error("undo/redo failed:", err));
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, []);

  // Ctrl/Cmd+E opens Edit Transform for the selected item (CAP-M05, OBS parity).
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "e" && event.key !== "E") return;
      if (!event.ctrlKey && !event.metaKey) return;
      if (event.altKey || event.shiftKey) return;
      const target = event.target as HTMLElement | null;
      if (
        target &&
        (target.tagName === "INPUT" || target.tagName === "TEXTAREA" || target.isContentEditable)
      )
        return;
      if (!effectiveSelection) return;
      event.preventDefault();
      setDialog({ kind: "editTransform", itemId: effectiveSelection });
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [effectiveSelection]);

  // The still-frame toast fades on its own.
  useEffect(() => {
    if (!stillToast) return;
    const timer = setTimeout(() => setStillToast(null), 3500);
    return () => clearTimeout(timer);
  }, [stillToast]);

  // The encoder-failover toast lingers a little longer — it explains a
  // mid-show change the operator did not initiate (CAP-M12).
  useEffect(() => {
    if (!fallbackToast) return;
    const timer = setTimeout(() => setFallbackToast(null), 8000);
    return () => clearTimeout(timer);
  }, [fallbackToast]);

  const addItem = useCallback(
    (settings: SourceSettings, name?: string) => {
      if (!activeScene) return;
      studioAddItem(activeScene.id, settings, name)
        .then((added) => selectSingle(added.itemId))
        .catch((err) => console.error("add item failed:", err));
    },
    [activeScene, selectSingle],
  );

  const addExisting = useCallback(
    (sourceId: SourceId) => {
      if (!activeScene) return;
      studioAddExistingSource(activeScene.id, sourceId)
        .then((itemId) => selectSingle(itemId))
        .catch((err) => console.error("add existing source failed:", err));
    },
    [activeScene, selectSingle],
  );

  const removeItem = useCallback(
    (itemId: ItemId) => {
      if (!activeScene) return;
      studioRemoveItem(activeScene.id, itemId).catch((err) =>
        console.error("remove item failed:", err),
      );
    },
    [activeScene],
  );

  const moveItem = useCallback(
    (itemId: ItemId, toIndex: number) => {
      if (!activeScene) return;
      studioReorderItem(activeScene.id, itemId, toIndex).catch((err) =>
        console.error("reorder item failed:", err),
      );
    },
    [activeScene],
  );

  const setVisible = useCallback(
    (itemId: ItemId, visible: boolean) => {
      if (!activeScene) return;
      studioSetItemVisible(activeScene.id, itemId, visible).catch((err) =>
        console.error("visibility toggle failed:", err),
      );
    },
    [activeScene],
  );

  const setLocked = useCallback(
    (itemId: ItemId, locked: boolean) => {
      if (!activeScene) return;
      studioSetItemLocked(activeScene.id, itemId, locked).catch((err) =>
        console.error("lock toggle failed:", err),
      );
    },
    [activeScene],
  );

  /** Handle drags: patch locally for instant feedback, then persist. */
  const setItemTransform = useCallback(
    (itemId: ItemId, transform: Transform) => {
      if (!activeScene) return;
      const sceneId = activeScene.id;
      localRevision.current += 1;
      setStudio((current) => {
        if (!current) return current;
        return {
          ...current,
          collection: {
            ...current.collection,
            scenes: current.collection.scenes.map((scene) =>
              scene.id === sceneId
                ? {
                    ...scene,
                    items: scene.items.map((item) =>
                      item.id === itemId ? { ...item, transform, pendingFit: false } : item,
                    ),
                  }
                : scene,
            ),
          },
        };
      });
      studioSetItemTransform(sceneId, itemId, transform).catch((err) =>
        console.error("transform update failed:", err),
      );
    },
    [activeScene],
  );

  // Batch transform for align-to-each-other, distribute, and group drags
  // (CAP-M04 follow-on). Optimistically patches every changed item at once and
  // commits them as a single undo step; `coalesce` folds a streaming group drag.
  const setItemsTransform = useCallback(
    (changes: { item: ItemId; transform: Transform }[], coalesce: boolean) => {
      if (!activeScene || changes.length === 0) return;
      const sceneId = activeScene.id;
      const byId = new Map(changes.map((c) => [c.item, c.transform]));
      localRevision.current += 1;
      setStudio((current) => {
        if (!current) return current;
        return {
          ...current,
          collection: {
            ...current.collection,
            scenes: current.collection.scenes.map((scene) =>
              scene.id === sceneId
                ? {
                    ...scene,
                    items: scene.items.map((item) => {
                      const next = byId.get(item.id);
                      return next ? { ...item, transform: next, pendingFit: false } : item;
                    }),
                  }
                : scene,
            ),
          },
        };
      });
      studioSetItemTransforms(sceneId, changes, coalesce).catch((err) =>
        console.error("batch transform failed:", err),
      );
    },
    [activeScene],
  );

  const showStats = settingsSettled && (settings?.showStatsDock ?? true);

  const toggleStatsDock = () => {
    if (!settings) return;
    const previous = settings;
    const next = { ...settings, showStatsDock: !settings.showStatsDock };
    setSettings(next);
    setSaveError(null);
    settingsSet(next).catch((err) => {
      // Roll back so the UI never claims a state the disk doesn't have.
      setSettings(previous);
      setSaveError(t("app-save-error"));
      console.error("could not persist settings:", err);
    });
  };

  const dialogItem =
    dialog?.kind === "filters"
      ? (activeScene?.items.find((item) => item.id === dialog.itemId) ?? null)
      : null;
  const dialogItemSource = dialogItem
    ? collection?.sources.find((source) => source.id === dialogItem.source)
    : undefined;
  const transformItem =
    dialog?.kind === "editTransform"
      ? (activeScene?.items.find((item) => item.id === dialog.itemId) ?? null)
      : null;
  const dialogSource =
    dialog?.kind === "properties"
      ? (collection?.sources.find((source) => source.id === dialog.sourceId) ?? null)
      : null;
  const dialogAudioSource =
    dialog?.kind === "audioFilters"
      ? (collection?.sources.find((source) => source.id === dialog.sourceId) ?? null)
      : null;

  // First-run gate: hold rendering until the EULA status is known, then block
  // the studio until the current version is accepted.
  if (eula === null) {
    return <div className="h-full w-full bg-havoc-bg" />;
  }
  if (!eula.accepted) {
    return <EulaGate status={eula} onAccepted={() => setEula({ ...eula, accepted: true })} />;
  }

  return (
    <div className="flex h-full flex-col gap-2 p-2">
      {/* Speaks going-live, recording and dropped-frame bursts to a screen reader. */}
      <StatusAnnouncer />
      {/* After the EULA gate, never before: consent precedes onboarding. */}
      {settings && !settings.completedOnboarding && !wizardDismissed && (
        <FirstRunWizard
          settings={settings}
          onSettingsSaved={setSettings}
          activeSceneId={activeScene?.id ?? null}
          onClose={() => setWizardDismissed(true)}
        />
      )}
      {paletteOpen && (
        <CommandPalette commands={paletteCommands} onClose={() => setPaletteOpen(false)} />
      )}
      {/* No app title here — the OS titlebar already says "Freally Capture".
          The menu bar sits on the left (the OBS position); the live-state
          chips keep the right. */}
      <header className="flex shrink-0 items-center justify-between gap-3 rounded-xl border border-white/10 bg-white/[0.03] px-4 py-2">
        <MenuBar
          onOpenControls={openControlsDialog}
          onOpenApp={openAppDialog}
          hasSelection={Boolean(selectedSceneItem)}
          canCopyFilters={(selectedSceneItem?.filters.length ?? 0) > 0}
          onCopyTransform={menuCopyTransform}
          onPasteTransform={menuPasteTransform}
          onCopyFilters={menuCopyFilters}
          onPasteFilters={menuPasteFilters}
          onEditTransform={menuEditTransform}
          statsShown={showStats}
          statsReady={Boolean(settings)}
          onToggleStats={toggleStatsDock}
          onSettingsSaved={setSettings}
        />
        <div className="flex items-center gap-3">
          {saveError && (
            <span role="alert" className="text-xs text-amber-400">
              {saveError}
            </span>
          )}
          <button
            type="button"
            onClick={() =>
              studioSetStudioMode(!studioMode).catch((err) =>
                console.error("studio mode toggle failed:", err),
              )
            }
            disabled={!collection}
            title={studioMode ? t("studio-mode-leave") : t("studio-mode-enter-title")}
            aria-pressed={studioMode !== null}
            className={`rounded-md border px-2 py-0.5 text-xs transition-colors disabled:opacity-50 ${
              studioMode
                ? "border-emerald-400/60 bg-emerald-500/15 text-emerald-300"
                : "border-white/10 text-havoc-muted enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text"
            }`}
          >
            {t("studio-mode")} {studioMode ? t("toggle-on") : t("toggle-off")}
          </button>
          <button
            type="button"
            onClick={() => setDialog({ kind: "vertical" })}
            disabled={!collection}
            title={t("vertical-canvas-title")}
            aria-pressed={Boolean(collection?.vertical)}
            className={`rounded-md border px-2 py-0.5 text-xs transition-colors disabled:opacity-50 ${
              collection?.vertical
                ? "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text"
                : "border-white/10 text-havoc-muted enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text"
            }`}
          >
            9:16 {collection?.vertical ? t("toggle-on") : t("toggle-off")}
          </button>
          <button
            type="button"
            onClick={toggleStatsDock}
            disabled={!settings}
            title={showStats ? t("hide-stats-dock") : t("show-stats-dock")}
            className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-50"
          >
            {t("stats")} {showStats ? t("toggle-on") : t("toggle-off")}
          </button>
          <span className="text-xs text-havoc-muted">
            {core
              ? `${t("app-version", { version: core.appVersion })} · ${
                  core.coreOk ? t("core-ok") : t("core-error")
                }`
              : coreError
                ? t("core-unreachable")
                : t("connecting-to-core")}
          </span>
        </div>
      </header>

      <RemoteSessionBar />

      <main className="flex min-h-0 flex-1 flex-col gap-2">
        <div className="grid min-h-0 flex-1 grid-cols-[240px_minmax(0,1fr)_280px] gap-2">
          <ScenesRail collection={collection} previewScene={studioMode?.previewScene ?? null} />
          <div className="flex min-h-0 min-w-0 gap-2">
            {studioMode && (
              <StudioPreviewPane
                settings={settings}
                onSettingsSaved={setSettings}
                transitioning={studioMode.transitioning}
              />
            )}
            <div className="flex min-h-0 min-w-0 flex-1 [&>section]:flex-1">
              <PreviewPanel
                collection={collection}
                scene={activeScene}
                program={program}
                selectedItem={effectiveSelection}
                selectedItems={effectiveSelectionList}
                onSelect={selectSingle}
                onToggleSelect={toggleSelect}
                onSelectMany={selectMany}
                onItemTransform={setItemTransform}
                onItemsTransform={setItemsTransform}
                alignment={
                  settings?.alignment ?? { smartGuides: true, safeAreas: false, rulers: false }
                }
              />
            </div>
          </div>
          <SourcesRail
            collection={collection}
            scene={activeScene}
            program={program}
            audio={audio}
            os={core?.os}
            selectedItem={effectiveSelection}
            onSelect={selectSingle}
            onAdd={addItem}
            onAddExisting={addExisting}
            onRemove={removeItem}
            onMove={moveItem}
            onSetVisible={setVisible}
            onSetLocked={setLocked}
            onOpenFilters={(itemId) => setDialog({ kind: "filters", itemId })}
            onOpenProperties={(sourceId) => setDialog({ kind: "properties", sourceId })}
          />
        </div>
        <div
          className={`grid h-44 shrink-0 gap-2 ${
            showStats ? "grid-cols-[2fr_1fr_1fr]" : "grid-cols-[3fr_1fr]"
          }`}
        >
          <MixerDock
            collection={collection}
            scene={activeScene}
            audio={audio}
            settings={settings}
            onSettingsSaved={setSettings}
            onOpenAudioFilters={(sourceId) => setDialog({ kind: "audioFilters", sourceId })}
          />
          <ControlsDock
            settings={settings}
            sceneNames={(collection?.scenes ?? []).map((scene) => scene.name)}
            onSettingsSaved={setSettings}
            onOpenSourceHealth={() => setDialog({ kind: "sourceHealth" })}
            menuOpenRef={controlsOpener}
          />
          {showStats && <StatsDock />}
        </div>
      </main>

      {dialog?.kind === "vertical" && (
        <VerticalCanvasDialog studio={studio} onClose={() => setDialog(null)} />
      )}
      {dialog?.kind === "history" && (
        <HistoryDialog studio={studio} onClose={() => setDialog(null)} />
      )}
      {dialog?.kind === "multiview" && (
        <MultiviewDialog studio={studio} onClose={() => setDialog(null)} />
      )}
      {dialog?.kind === "projector" && (
        <ProjectorDialog collection={collection} onClose={() => setDialog(null)} />
      )}
      {dialog?.kind === "avSync" && (
        <AvSyncDialog
          studio={studio}
          program={program}
          audio={audio}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog?.kind === "hotkeyAudit" && <HotkeyAuditDialog onClose={() => setDialog(null)} />}
      {dialog?.kind === "sourceHealth" && (
        <SourceHealthDialog
          studio={studio}
          program={program}
          onOpenProperties={(sourceId) => setDialog({ kind: "properties", sourceId })}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog?.kind === "missingFiles" && <MissingFilesDialog onClose={() => setDialog(null)} />}
      {quitGuard && <QuitGuardDialog pending={quitGuard} onClose={() => setQuitGuard(null)} />}
      {studio?.panic && <PanicBanner />}
      {salvagePaths && <SalvageDialog paths={salvagePaths} onClose={() => setSalvagePaths(null)} />}
      {stillToast && (
        <div
          role="status"
          className={`pointer-events-none fixed bottom-4 left-1/2 z-50 -translate-x-1/2 rounded-md border bg-black/80 px-3 py-1.5 text-xs ${
            stillToast.ok ? "border-white/10 text-havoc-text" : "border-red-500/50 text-red-300"
          }`}
        >
          {stillToast.ok
            ? t("still-saved-toast", { name: stillToast.text })
            : t("still-failed-toast", { error: stillToast.text })}
        </div>
      )}
      {Object.keys(alarms).length > 0 && (
        <div className="pointer-events-none fixed top-3 left-1/2 z-40 flex -translate-x-1/2 flex-col items-center gap-1">
          {Object.values(alarms).map((alarm) => (
            <div
              key={alarm.kind}
              role="alert"
              className="pointer-events-auto flex items-center gap-2 rounded-md border border-amber-400/50 bg-black/85 px-3 py-1.5 text-xs text-amber-200"
            >
              <span>
                {alarm.kind === "lowDisk"
                  ? t("alarm-lowDisk", { minutes: alarm.minutesLeft ?? 0 })
                  : t(`alarm-${alarm.kind}`)}
              </span>
              <button
                type="button"
                aria-label={t("alarm-dismiss")}
                onClick={() =>
                  setAlarms((current) => {
                    const next = { ...current };
                    delete next[alarm.kind];
                    return next;
                  })
                }
                className="rounded px-1 text-amber-200/80 transition-colors hover:text-amber-100"
              >
                ×
              </button>
            </div>
          ))}
        </div>
      )}
      {fallbackToast && (
        <div
          role="status"
          className="pointer-events-none fixed bottom-12 left-1/2 z-50 -translate-x-1/2 rounded-md border border-amber-400/40 bg-black/80 px-3 py-1.5 text-xs text-amber-200"
        >
          {t(
            fallbackToast.scope === "stream" ? "fallback-toast-stream" : "fallback-toast-recording",
            { from: fallbackToast.from, to: fallbackToast.to },
          )}
        </div>
      )}
      {dialog?.kind === "editTransform" && activeScene && transformItem && (
        <EditTransformDialog
          sceneId={activeScene.id}
          item={transformItem}
          sourceName={
            collection?.sources.find((source) => source.id === transformItem.source)?.name ??
            t("filters-source-fallback")
          }
          program={program}
          canvasW={collection?.canvasWidth ?? 1920}
          canvasH={collection?.canvasHeight ?? 1080}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog?.kind === "filters" && activeScene && dialogItem && (
        <FiltersDialog
          sceneId={activeScene.id}
          item={dialogItem}
          sourceName={dialogItemSource?.name ?? t("filters-source-fallback")}
          source={dialogItemSource}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog?.kind === "properties" && dialogSource && (
        <PropertiesDialog
          source={dialogSource}
          scenes={collection?.scenes.map((entry) => ({ id: entry.id, name: entry.name })) ?? []}
          audioSources={
            collection?.sources
              .filter((entry) => kindHasAudio(entry.kind))
              .map((entry) => ({ id: entry.id, name: entry.name })) ?? []
          }
          onClose={() => setDialog(null)}
        />
      )}
      {dialog?.kind === "audioFilters" && dialogAudioSource && (
        <AudioFiltersDialog
          source={dialogAudioSource}
          collection={collection}
          onClose={() => setDialog(null)}
        />
      )}
    </div>
  );
}
