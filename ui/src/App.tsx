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
  studioSetFocus,
  studioSetItemLocked,
  studioSetStudioMode,
  studioSetItemTransform,
  studioSetItemVisible,
  studioSelectScene,
  studioTransition,
  replaySave,
  recordingAddMarker,
} from "./api/commands";
import { onAudio, onProgram, onRemoteInvite, onStudio } from "./api/events";
import type {
  AudioLevelsPayload,
  EulaStatus,
  Health,
  ItemId,
  ProgramStatus,
  Settings,
  SourceId,
  SourceSettings,
  StudioDto,
  Transform,
} from "./api/types";
import { AudioFiltersDialog } from "./components/AudioFiltersDialog";
import { CommandPalette } from "./components/CommandPalette";
import type { Command } from "./lib/commands";
import { FiltersDialog } from "./components/FiltersDialog";
import { PropertiesDialog } from "./components/PropertiesDialog";
import { spikeSetJoinPrefill } from "./remote/spike";
import { ControlsDock } from "./panels/ControlsDock";
import { EulaGate } from "./panels/EulaGate";
import { MixerDock } from "./panels/MixerDock";
import { PreviewPanel } from "./panels/PreviewPanel";
import { RemoteSessionBar } from "./panels/RemoteSessionBar";
import { ScenesRail } from "./panels/ScenesRail";
import { SourcesRail } from "./panels/SourcesRail";
import { StatsDock } from "./panels/StatsDock";
import { StudioPreviewPane } from "./panels/StudioPreviewPane";
import { VerticalCanvasDialog } from "./panels/VerticalCanvasDialog";

type OpenDialog =
  | { kind: "filters"; itemId: ItemId }
  | { kind: "properties"; sourceId: SourceId }
  | { kind: "audioFilters"; sourceId: SourceId }
  | { kind: "vertical" }
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

  const [studio, setStudio] = useState<StudioDto | null>(null);
  const [program, setProgram] = useState<ProgramStatus | null>(null);
  const [audio, setAudio] = useState<AudioLevelsPayload | null>(null);
  const [selectedItem, setSelectedItem] = useState<ItemId | null>(null);
  const [dialog, setDialog] = useState<OpenDialog>(null);
  const [paletteOpen, setPaletteOpen] = useState(false);
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
    };
  }, []);

  const collection = studio?.collection ?? null;
  const studioMode = studio?.studioMode ?? null;
  const activeScene = useMemo(
    () => collection?.scenes.find((scene) => scene.id === collection.activeScene) ?? null,
    [collection],
  );

  // Selection follows reality (derived, not synced): it only counts while it
  // names an item of the active scene.
  const effectiveSelection =
    selectedItem && activeScene?.items.some((item) => item.id === selectedItem)
      ? selectedItem
      : null;

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
        run: () => setSelectedItem(item.id),
      });
    }

    list.push(
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

    return list;
  }, [collection, activeScene, studioMode, t]);

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

  const addItem = useCallback(
    (settings: SourceSettings, name?: string) => {
      if (!activeScene) return;
      studioAddItem(activeScene.id, settings, name)
        .then((added) => setSelectedItem(added.itemId))
        .catch((err) => console.error("add item failed:", err));
    },
    [activeScene],
  );

  const addExisting = useCallback(
    (sourceId: SourceId) => {
      if (!activeScene) return;
      studioAddExistingSource(activeScene.id, sourceId)
        .then((itemId) => setSelectedItem(itemId))
        .catch((err) => console.error("add existing source failed:", err));
    },
    [activeScene],
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
      {paletteOpen && (
        <CommandPalette commands={paletteCommands} onClose={() => setPaletteOpen(false)} />
      )}
      {/* No app title here — the OS titlebar already says "Freally Capture".
          `justify-end` (not `justify-between`) keeps the controls on the right
          now that nothing balances them on the left. */}
      <header className="flex shrink-0 items-center justify-end rounded-xl border border-white/10 bg-white/[0.03] px-4 py-2">
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
                onSelect={setSelectedItem}
                onItemTransform={setItemTransform}
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
            onSelect={setSelectedItem}
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
          <ControlsDock settings={settings} onSettingsSaved={setSettings} />
          {showStats && <StatsDock />}
        </div>
      </main>

      {dialog?.kind === "vertical" && (
        <VerticalCanvasDialog studio={studio} onClose={() => setDialog(null)} />
      )}
      {dialog?.kind === "filters" && activeScene && dialogItem && (
        <FiltersDialog
          sceneId={activeScene.id}
          item={dialogItem}
          sourceName={
            collection?.sources.find((source) => source.id === dialogItem.source)?.name ??
            t("filters-source-fallback")
          }
          onClose={() => setDialog(null)}
        />
      )}
      {dialog?.kind === "properties" && dialogSource && (
        <PropertiesDialog
          source={dialogSource}
          scenes={collection?.scenes.map((entry) => ({ id: entry.id, name: entry.name })) ?? []}
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
