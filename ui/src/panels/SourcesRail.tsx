import { useCallback, useEffect, useRef, useState, useSyncExternalStore } from "react";
import { open } from "@tauri-apps/plugin-dialog";

import {
  appAudioApps,
  audioInputDevices,
  audioLoopbackDevices,
  captureListSources,
  captureWindowThumbnail,
  gameCaptureStatus,
  linkDiscover,
  openPrivacySettings,
  settingsGet,
  settingsSet,
  studioAddItem,
  studioApplyLayout,
  studioCreateGroup,
  studioSetAudioMonitor,
  studioSetAudioMuted,
  studioSetBackdropSplit,
  studioMediaPaused,
  studioMediaSetPaused,
  studioRenameSource,
  studioRetrySource,
  hdrToneMapSet,
  studioAddLinkedWindow,
  studioSetCenterView,
  studioZoomFollow,
  studioZoomGet,
  studioSetFocus,
  studioSetItemOutputVisible,
  studioSetGroupVisible,
  studioSetSceneBackdrop,
  studioUngroup,
  videoDeviceFormats,
  videoDevicesList,
} from "../api/commands";
import type {
  AppAudioList,
  AudioDevice,
  AudioLevelsPayload,
  CaptureSource,
  Collection,
  Corner,
  CornerSlot,
  GameCaptureStatus,
  InputLayout,
  ItemId,
  CountdownSlate,
  LinkPeer,
  ProgramStatus,
  Scene,
  SceneId,
  Settings,
  SourceId,
  SourceSettings,
  TimerMode,
  ReplaySpeed,
  SplitComparison,
  TitleAnimation,
  TitleLayer,
  VideoDevice,
  VideoFormat,
  VisStyle,
} from "../api/types";
import { CORNERS, kindHasAudio } from "../api/types";
import { useT } from "../i18n/t";
import { ClockSelect } from "../components/ClockSelect";
import { EmptyHint, Panel } from "../components/Panel";
import { LanIngestFields, type LanIngestValue } from "../components/LanIngestFields";
import { NumberField } from "../components/NumberField";
import { PickerShell } from "../components/PickerShell";
import { QrSvg } from "../components/QrSvg";
import { SocialBarFields, type SocialBarValue } from "../components/SocialBarFields";
import { hexToRgba } from "../lib/color";
import { LAN_DEFAULT_PORTS, lanPassphraseUsable } from "../lib/lanIngest";
import {
  AUDIO_EXTS,
  INPUT_LAYOUTS,
  newSocialRow,
  REPLAY_SPEEDS,
  SLATE_GRADIENT_FROM,
  SLATE_GRADIENT_TO,
  SLATE_SOLID,
  VIS_STYLES,
} from "../lib/sourceOptions";
import { titleTextLayer } from "../lib/titleLayers";
import { parseVisTarget } from "../lib/visTarget";
import { useDismiss } from "../lib/useDismiss";
import {
  spikeGetState,
  spikeHost,
  spikeJoin,
  spikeSetInviteTtl,
  spikeSetMic,
  spikeSetSpeaker,
  spikeStop,
  spikeSubscribe,
} from "../remote/spike";
import {
  listRemoteAudioDevices,
  onDeviceChange,
  type RemoteAudioDevices,
  startMicTest,
} from "../remote/devices";
import { joinTargetFromInput, parseInviteInput, webJoinLink } from "../remote/invite";

type SourcesRailProps = {
  collection: Collection | null;
  scene: Scene | null;
  program: ProgramStatus | null;
  audio: AudioLevelsPayload | null;
  os?: string;
  selectedItem: ItemId | null;
  onSelect: (item: ItemId | null) => void;
  onAdd: (settings: SourceSettings, name?: string) => void;
  onAddExisting: (source: SourceId) => void;
  onRemove: (item: ItemId) => void;
  onMove: (item: ItemId, toIndex: number) => void;
  onSetVisible: (item: ItemId, visible: boolean) => void;
  onSetLocked: (item: ItemId, locked: boolean) => void;
  onOpenFilters: (item: ItemId) => void;
  onOpenProperties: (source: SourceId) => void;
};

type PickerMode =
  | "display"
  | "window"
  | "webcam"
  | "image"
  | "media"
  | "remoteGuest"
  | "color"
  | "text"
  | "nestedScene"
  | "slideshow"
  | "chatOverlay"
  | "audioInput"
  | "audioOutput"
  | "appAudio"
  | "backgroundMusic"
  | "gameCapture"
  | "testSignal"
  | "timer"
  | "startingSoon"
  | "systemStats"
  | "audioVisualizer"
  | "splitTimer"
  | "inputOverlay"
  | "playlist"
  | "replayPlayback"
  | "lanIngest"
  | "title"
  | "socialBar"
  | "freallyLink"
  | "existing";

// Values are i18n keys, resolved with `t(...)` at each render site so a
// language switch repaints them.
const KIND_BADGE: Record<string, string> = {
  display: "sources-badge-display",
  window: "sources-badge-window",
  portal: "sources-badge-portal",
  videoDevice: "sources-badge-camera",
  image: "sources-badge-image",
  media: "sources-badge-media",
  remoteGuest: "sources-badge-guest",
  color: "sources-badge-color",
  text: "sources-badge-text",
  nestedScene: "sources-badge-scene",
  slideshow: "sources-badge-slides",
  chatOverlay: "sources-badge-chat",
  audioInput: "sources-badge-audio-in",
  audioOutput: "sources-badge-audio-out",
  appAudio: "sources-badge-app-audio",
  testBars: "sources-badge-test-bars",
  testGrid: "sources-badge-test-grid",
  testSweep: "sources-badge-test-sweep",
  testTone: "sources-badge-test-tone",
  testFlashBeep: "sources-badge-test-sync",
  timer: "sources-badge-timer",
  systemStats: "sources-badge-stats",
  audioVisualizer: "sources-badge-visualizer",
  splitTimer: "sources-badge-splits",
  inputOverlay: "sources-badge-input",
  playlist: "sources-badge-playlist",
  replayPlayback: "sources-badge-replay",
  lanIngest: "sources-badge-lan-ingest",
  title: "sources-badge-title",
  socialBar: "sources-badge-social",
  freallyLink: "sources-badge-link",
};

// Values are i18n keys (see KIND_BADGE).
const ADD_MENU: Array<[PickerMode, string]> = [
  ["display", "sources-add-display"],
  ["window", "sources-add-window"],
  ["gameCapture", "sources-add-game"],
  ["webcam", "sources-add-webcam"],
  ["image", "sources-add-image"],
  ["media", "sources-add-media"],
  ["playlist", "sources-add-playlist"],
  ["replayPlayback", "sources-add-replay"],
  ["remoteGuest", "sources-add-remote-guest"],
  ["lanIngest", "sources-add-lan-ingest"],
  ["freallyLink", "sources-add-freally-link"],
  ["color", "sources-add-color"],
  ["text", "sources-add-text"],
  ["title", "sources-add-title"],
  ["socialBar", "sources-add-social-bar"],
  ["timer", "sources-add-timer"],
  ["startingSoon", "sources-add-starting-soon"],
  ["systemStats", "sources-add-system-stats"],
  ["audioVisualizer", "sources-add-visualizer"],
  ["splitTimer", "sources-add-split-timer"],
  ["inputOverlay", "sources-add-input-overlay"],
  ["nestedScene", "sources-add-nested-scene"],
  ["slideshow", "sources-add-slideshow"],
  ["chatOverlay", "sources-add-chat-overlay"],
  ["testSignal", "sources-add-test-signal"],
  ["audioInput", "sources-add-audio-input"],
  ["audioOutput", "sources-add-audio-output"],
  ["appAudio", "sources-add-app-audio"],
  ["backgroundMusic", "sources-add-background-music"],
  ["existing", "sources-add-existing"],
];

/**
 * The Sources rail: the active scene's items, top of the stack first.
 * Model order is bottom-first (index = z), so the list renders reversed.
 */
export function SourcesRail({
  collection,
  scene,
  program,
  audio,
  os,
  selectedItem,
  onSelect,
  onAdd,
  onAddExisting,
  onRemove,
  onMove,
  onSetVisible,
  onSetLocked,
  onOpenFilters,
  onOpenProperties,
}: SourcesRailProps) {
  const t = useT();
  const [menuOpen, setMenuOpen] = useState(false);
  // Wraps the + button *and* its menu, so clicking the button dismisses via its
  // own toggle rather than via the outside-click handler (which would re-open it).
  const addMenuRef = useRef<HTMLDivElement>(null);
  useDismiss(menuOpen, addMenuRef, () => setMenuOpen(false));
  const [picker, setPicker] = useState<PickerMode | null>(null);
  // Grouping (TASK-605): while non-null, rows show pick-boxes; "Create
  // group" bundles the picked items so they move/show/hide together.
  const [groupPick, setGroupPick] = useState<ItemId[] | null>(null);
  const [showLayout, setShowLayout] = useState(false);
  const [renaming, setRenaming] = useState<{ source: SourceId; draft: string } | null>(null);
  // Paused state of embedded Media sources (videos), keyed by source id — the
  // streamer pauses/resumes a video live on the broadcast.
  const [mediaPaused, setMediaPaused] = useState<Record<string, boolean>>({});
  // Punch-in follow-pan (CAP-N71): per-item cursor-follow state, hydrated
  // from the engine so a webview reload doesn't lie about active lenses.
  const [zoomFollow, setZoomFollow] = useState<Record<string, boolean>>({});
  // CAP-N74: the display source whose HDR tone-map dialog is open.
  const [hdrFor, setHdrFor] = useState<{ captureId: string; name: string } | null>(null);
  // Keep the pause buttons synced with the backend for the current Media
  // sources (e.g. after a reload — a video may already be paused).
  const mediaIdsKey = (collection?.sources ?? [])
    .filter((source) => source.kind === "media")
    .map((source) => source.id)
    .join(",");
  useEffect(() => {
    if (!mediaIdsKey) return;
    let alive = true;
    for (const id of mediaIdsKey.split(",")) {
      studioMediaPaused(id as SourceId)
        .then((paused) => alive && setMediaPaused((prev) => ({ ...prev, [id]: paused })))
        .catch(() => undefined);
    }
    return () => {
      alive = false;
    };
  }, [mediaIdsKey]);

  // Same hydration for follow-pan lenses on the scene's items.
  const followIdsKey = (scene?.items ?? [])
    .filter((item) => !item.backdrop)
    .map((item) => item.id)
    .join(",");
  useEffect(() => {
    if (!followIdsKey) return;
    let alive = true;
    for (const id of followIdsKey.split(",")) {
      studioZoomGet(id as ItemId)
        .then((lens) => alive && lens && setZoomFollow((prev) => ({ ...prev, [id]: lens.follow })))
        .catch(() => undefined);
    }
    return () => {
      alive = false;
    };
  }, [followIdsKey]);

  const items = scene?.items ?? [];
  const topFirst = [...items].reverse();
  const sourceOf = (id: SourceId) => collection?.sources.find((source) => source.id === id);
  const groups = scene?.groups ?? [];
  const groupOf = (id: ItemId) => groups.find((group) => group.items.includes(id));

  const createGroup = () => {
    if (!scene || !groupPick || groupPick.length < 2) return;
    studioCreateGroup(scene.id, "", groupPick).catch((err) =>
      console.error("group create failed:", err),
    );
    setGroupPick(null);
  };

  const commitRename = () => {
    if (!renaming) return;
    const { source, draft } = renaming;
    setRenaming(null);
    if (draft.trim()) {
      studioRenameSource(source, draft.trim()).catch((err) =>
        console.error("source rename failed:", err),
      );
    }
  };

  const openPicker = (mode: PickerMode) => {
    setMenuOpen(false);
    setPicker(mode);
  };

  const pick = (settings: SourceSettings, name?: string) => {
    setPicker(null);
    onAdd(settings, name);
  };

  return (
    <Panel
      title={t("sources-panel-title")}
      actions={
        <div className="flex items-center gap-1">
          <button
            type="button"
            disabled={!scene}
            onClick={() => setGroupPick((picking) => (picking === null ? [] : null))}
            title={t("sources-group-title")}
            aria-label={t("sources-group-aria")}
            aria-pressed={groupPick !== null}
            className={`rounded-md border px-2 py-0.5 text-xs transition-colors disabled:opacity-60 ${
              groupPick !== null
                ? "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text"
                : "border-white/10 text-havoc-muted enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text"
            }`}
          >
            ⊞
          </button>
          <button
            type="button"
            disabled={!scene}
            onClick={() => setShowLayout(true)}
            title={t("sources-arrange")}
            aria-label={t("sources-arrange")}
            className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-60"
          >
            ▦
          </button>
          <div className="relative" ref={addMenuRef}>
            <button
              type="button"
              disabled={!scene}
              onClick={() => setMenuOpen((open) => !open)}
              title={t("sources-add-source")}
              aria-label={t("sources-add-source")}
              aria-haspopup="menu"
              aria-expanded={menuOpen}
              className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted transition-colors enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-60"
            >
              +
            </button>
            {menuOpen && (
              <div
                role="menu"
                aria-label={t("sources-add-source")}
                // Scrolls within its own panel: the rail's stacking context
                // caps the menu, so a taller list would poke past the panel,
                // render UNDER the docks, and eat clicks (Playwright caught
                // this when the CAP-M15/M21 entries lengthened the menu).
                className="absolute right-0 z-20 mt-1 max-h-72 w-48 overflow-y-auto rounded-lg border border-white/10 bg-havoc-panel p-1 shadow-xl"
              >
                {ADD_MENU.map(([mode, label]) => (
                  <button
                    key={mode}
                    type="button"
                    role="menuitem"
                    onClick={() => openPicker(mode)}
                    className="block w-full rounded-md px-2 py-1.5 text-left text-xs text-havoc-text hover:bg-white/5"
                  >
                    {t(label)}
                  </button>
                ))}
                <p className="m-0 border-t border-white/5 px-2 py-1.5 text-[10px] leading-snug text-havoc-muted">
                  {t("sources-browser-source-note")}
                </p>
              </div>
            )}
          </div>
        </div>
      }
    >
      {topFirst.length === 0 ? (
        <EmptyHint>{t("sources-empty")}</EmptyHint>
      ) : (
        <ul className="m-0 flex list-none flex-col gap-1.5 p-0">
          {topFirst.map((item) => {
            const modelIndex = items.findIndex((candidate) => candidate.id === item.id);
            const source = sourceOf(item.source);
            // Audio-*only* sources report through the `audio` event; every
            // video source (incl. Media, which also has audio but is
            // video-primary) reports its pipeline state — errors, retry —
            // through `program`. Same status shape, one dot.
            const audioOnly =
              source?.kind === "audioInput" ||
              source?.kind === "audioOutput" ||
              source?.kind === "appAudio";
            const status = audioOnly ? audio?.sources[item.source] : program?.sources[item.source];
            const isSelected = item.id === selectedItem;
            const isRenaming = renaming?.source === item.source;
            const isFocused = scene?.focus?.item === item.id;
            const itemGroup = groupOf(item.id);
            return (
              <li key={item.id}>
                <div
                  className={`group flex flex-col-reverse gap-1 rounded-lg border px-1.5 py-1.5 ${
                    isSelected
                      ? "border-havoc-accent/50 bg-havoc-accent/10"
                      : "border-white/10 bg-white/[0.02]"
                  }`}
                >
                  <div className="flex min-w-0 items-center gap-1">
                    {groupPick !== null && (
                      <input
                        type="checkbox"
                        checked={groupPick.includes(item.id)}
                        disabled={Boolean(itemGroup)}
                        title={
                          itemGroup
                            ? t("sources-already-in-group", { name: itemGroup.name })
                            : t("sources-pick-for-new-group")
                        }
                        aria-label={t("sources-pick-item-for-group", {
                          name: source?.name ?? t("sources-fallback-name"),
                        })}
                        onChange={(event) =>
                          setGroupPick((picked) =>
                            picked === null
                              ? picked
                              : event.target.checked
                                ? [...picked, item.id]
                                : picked.filter((id) => id !== item.id),
                          )
                        }
                      />
                    )}
                    <button
                      type="button"
                      onClick={() => onSetVisible(item.id, !item.visible)}
                      title={item.visible ? t("sources-hide") : t("sources-show")}
                      aria-label={
                        item.visible
                          ? t("sources-hide-item", {
                              name: source?.name ?? t("sources-fallback-name"),
                            })
                          : t("sources-show-item", {
                              name: source?.name ?? t("sources-fallback-name"),
                            })
                      }
                      aria-pressed={item.visible}
                      className={`shrink-0 rounded px-1 text-xs ${
                        item.visible ? "text-havoc-text" : "text-havoc-muted opacity-50"
                      }`}
                    >
                      {item.visible ? "👁" : "–"}
                    </button>
                    <button
                      type="button"
                      disabled={Boolean(item.backdrop)}
                      onClick={() => {
                        if (!scene) return;
                        studioSetFocus(scene.id, isFocused ? null : item.id).catch((err) =>
                          console.error("focus toggle failed:", err),
                        );
                      }}
                      title={
                        item.backdrop
                          ? t("sources-backdrop-pinned")
                          : isFocused
                            ? t("sources-unfocus-title")
                            : t("sources-focus-title")
                      }
                      aria-label={
                        isFocused
                          ? t("sources-unfocus-item", {
                              name: source?.name ?? t("sources-fallback-name"),
                            })
                          : t("sources-focus-item", {
                              name: source?.name ?? t("sources-fallback-name"),
                            })
                      }
                      aria-pressed={isFocused}
                      className={`shrink-0 rounded px-1 text-xs ${
                        isFocused
                          ? "text-havoc-accent"
                          : "text-havoc-muted opacity-60 hover:opacity-100"
                      }`}
                    >
                      ⛶
                    </button>
                    {!audioOnly && !item.backdrop && (
                      <button
                        type="button"
                        onClick={() => {
                          if (!scene) return;
                          studioSetCenterView(scene.id, item.id).catch((err) =>
                            console.error("center view failed:", err),
                          );
                        }}
                        title={t("sources-center-title")}
                        aria-label={t("sources-center-item", {
                          name: source?.name ?? t("sources-fallback-name"),
                        })}
                        className="shrink-0 rounded px-1 text-xs text-havoc-muted opacity-60 hover:opacity-100"
                      >
                        ◉
                      </button>
                    )}
                    {(source?.kind === "display" || source?.kind === "window") &&
                      !item.backdrop && (
                        <button
                          type="button"
                          onClick={() => {
                            const next = !zoomFollow[item.id];
                            setZoomFollow((state) => ({ ...state, [item.id]: next }));
                            studioZoomFollow(item.id, next).catch((err) =>
                              console.error("zoom follow failed:", err),
                            );
                          }}
                          title={t("sources-follow-title")}
                          aria-label={t("sources-follow-item", {
                            name: source?.name ?? t("sources-fallback-name"),
                          })}
                          aria-pressed={Boolean(zoomFollow[item.id])}
                          className={`shrink-0 rounded px-1 text-xs ${
                            zoomFollow[item.id]
                              ? "text-havoc-accent"
                              : "text-havoc-muted opacity-60 hover:opacity-100"
                          }`}
                        >
                          🎯
                        </button>
                      )}
                    {isRenaming ? (
                      <input
                        autoFocus
                        value={renaming.draft}
                        onChange={(event) =>
                          setRenaming({ source: item.source, draft: event.target.value })
                        }
                        onBlur={commitRename}
                        onKeyDown={(event) => {
                          if (event.key === "Enter") commitRename();
                          if (event.key === "Escape") setRenaming(null);
                        }}
                        aria-label={t("sources-rename-item", {
                          name: source?.name ?? t("sources-fallback-name"),
                        })}
                        className="min-w-0 flex-1 rounded border border-havoc-accent/50 bg-transparent px-1 text-xs text-havoc-text outline-none"
                      />
                    ) : (
                      <button
                        type="button"
                        onClick={() => onSelect(item.id)}
                        onDoubleClick={() =>
                          source && setRenaming({ source: source.id, draft: source.name })
                        }
                        title={source?.name}
                        className="flex min-w-0 flex-1 items-center gap-1.5 truncate text-left text-xs text-havoc-text"
                      >
                        <span className="rounded bg-white/10 px-1 py-px text-[9px] text-havoc-muted uppercase">
                          {t(KIND_BADGE[source?.kind ?? ""] ?? "sources-kind-unknown")}
                        </span>
                        {itemGroup && (
                          <span
                            title={t("sources-in-group", { name: itemGroup.name })}
                            className="rounded bg-havoc-accent/15 px-1 py-px text-[9px] text-havoc-accent"
                          >
                            ⊞
                          </span>
                        )}
                        {item.backdrop && (
                          <span
                            title={t("sources-backdrop-badge")}
                            className="rounded bg-havoc-accent/15 px-1 py-px text-[9px] text-havoc-accent"
                          >
                            🖼
                          </span>
                        )}
                        <span className="truncate">
                          {source?.name ?? t("sources-missing-source")}
                        </span>
                      </button>
                    )}
                    {status && status.state === "error" ? (
                      <span className="flex shrink-0 items-center gap-1">
                        <button
                          type="button"
                          onClick={() =>
                            studioRetrySource(item.source).catch((err) =>
                              console.error("source retry failed:", err),
                            )
                          }
                          title={t("sources-retry-error", {
                            message: status.errorMessage ?? t("sources-fallback-error"),
                          })}
                          aria-label={t("sources-retry-item", {
                            name: source?.name ?? t("sources-fallback-name"),
                          })}
                          className="flex items-center gap-1 rounded px-1 text-[10px] text-red-400 hover:text-red-300"
                        >
                          <span
                            aria-label={t("sources-status-error")}
                            className="h-1.5 w-1.5 rounded-full bg-red-400"
                          />
                          ↻
                        </button>
                        {status.errorCode === "permission" && os === "macos" && (
                          <button
                            type="button"
                            onClick={() =>
                              void openPrivacySettings(
                                source?.kind === "videoDevice" ? "camera" : "screenRecording",
                              )
                            }
                            title={t("sources-open-privacy-title")}
                            aria-label={t("sources-open-privacy-item", {
                              name: source?.name ?? t("sources-fallback-name"),
                            })}
                            className="rounded border border-red-400/40 px-1 text-[9px] text-red-300 hover:border-red-300"
                          >
                            {t("sources-privacy-settings-button")}
                          </button>
                        )}
                      </span>
                    ) : status ? (
                      <span
                        title={
                          status.state !== "live"
                            ? t("sources-status-starting")
                            : "width" in status && status.width
                              ? `${status.width}×${status.height}${status.fps ? ` @ ${status.fps}` : ""}`
                              : t("sources-status-live")
                        }
                        aria-label={t("sources-status-aria", { state: status.state })}
                        className={`h-1.5 w-1.5 shrink-0 rounded-full ${
                          status.state === "live" ? "bg-emerald-400" : "bg-amber-300"
                        }`}
                      />
                    ) : null}
                    {source?.kind === "media" && (
                      <button
                        type="button"
                        onClick={() => {
                          const next = !mediaPaused[item.source];
                          setMediaPaused((prev) => ({ ...prev, [item.source]: next }));
                          studioMediaSetPaused(item.source, next).catch((err) =>
                            console.error("media pause failed:", err),
                          );
                        }}
                        title={
                          mediaPaused[item.source]
                            ? t("sources-media-resume-title")
                            : t("sources-media-pause-title")
                        }
                        aria-label={
                          mediaPaused[item.source]
                            ? t("sources-media-resume-item", {
                                name: source?.name ?? t("sources-fallback-video"),
                              })
                            : t("sources-media-pause-item", {
                                name: source?.name ?? t("sources-fallback-video"),
                              })
                        }
                        aria-pressed={Boolean(mediaPaused[item.source])}
                        className={`shrink-0 rounded px-1 text-[11px] ${
                          mediaPaused[item.source]
                            ? "text-amber-300"
                            : "text-havoc-muted hover:text-havoc-text"
                        }`}
                      >
                        {mediaPaused[item.source] ? "▶" : "⏸"}
                      </button>
                    )}
                    {status != null &&
                      "hdr" in status &&
                      status.hdr === true &&
                      source?.kind === "display" && (
                        <button
                          type="button"
                          onClick={() =>
                            source.kind === "display" &&
                            setHdrFor({ captureId: source.captureId, name: source.name })
                          }
                          title={t("sources-hdr-title")}
                          aria-label={t("sources-hdr-item", {
                            name: source?.name ?? t("sources-fallback-name"),
                          })}
                          className="shrink-0 rounded bg-amber-400/15 px-1 py-px text-[9px] font-semibold text-amber-300"
                        >
                          HDR
                        </button>
                      )}
                  </div>
                  {/* The action strip — rendered ON TOP of the source line
                      (flex-col-reverse) like a window's control row: always
                      visible, right-aligned, wraps if it ever runs out of
                      width so nothing is ever clipped by the rail. */}
                  <div className="flex flex-wrap items-center justify-end gap-0.5">
                    {/* CAP-N53 per-output visibility: LIVE/REC toggles, struck
                      through + amber while an output is off. */}
                    {(
                      [
                        { key: "stream", caption: "LIVE", on: item.onStream ?? true },
                        { key: "record", caption: "REC", on: item.onRecord ?? true },
                      ] as const
                    ).map(({ key, caption, on }) => (
                      <button
                        key={key}
                        type="button"
                        onClick={() => {
                          if (!scene) return;
                          studioSetItemOutputVisible(
                            scene.id,
                            item.id,
                            key === "stream" ? !on : (item.onStream ?? true),
                            key === "record" ? !on : (item.onRecord ?? true),
                          ).catch((err) => console.error("output visibility failed:", err));
                        }}
                        title={on ? t(`sources-${key}-hide`) : t(`sources-${key}-show`)}
                        aria-label={t(`sources-${key}-${on ? "hide" : "show"}-item`, {
                          name: source?.name ?? t("sources-fallback-name"),
                        })}
                        aria-pressed={on}
                        className={`inline-block shrink-0 rounded px-1 py-px text-[9px] font-semibold ${
                          on
                            ? "text-havoc-muted hover:text-havoc-text"
                            : "text-amber-300 line-through"
                        }`}
                      >
                        {caption}
                      </button>
                    ))}
                    <span className="flex shrink-0 items-center">
                      <button
                        type="button"
                        onClick={() => onSetLocked(item.id, !item.locked)}
                        title={item.locked ? t("sources-unlock") : t("sources-lock")}
                        aria-label={
                          item.locked
                            ? t("sources-unlock-item", {
                                name: source?.name ?? t("sources-fallback-name"),
                              })
                            : t("sources-lock-item", {
                                name: source?.name ?? t("sources-fallback-name"),
                              })
                        }
                        aria-pressed={item.locked}
                        className={`rounded px-1 text-[10px] ${
                          item.locked ? "text-amber-300" : "text-havoc-muted hover:text-havoc-text"
                        }`}
                      >
                        {item.locked ? "🔒" : "🔓"}
                      </button>
                      <button
                        type="button"
                        disabled={modelIndex === items.length - 1 || Boolean(item.backdrop)}
                        onClick={() => onMove(item.id, modelIndex + 1)}
                        title={
                          item.backdrop ? t("sources-backdrop-pinned") : t("sources-raise-title")
                        }
                        aria-label={t("sources-raise-item", {
                          name: source?.name ?? t("sources-fallback-name"),
                        })}
                        className="rounded px-1 text-[10px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
                      >
                        ▲
                      </button>
                      <button
                        type="button"
                        disabled={
                          modelIndex === 0 ||
                          Boolean(item.backdrop) ||
                          Boolean(items[modelIndex - 1]?.backdrop)
                        }
                        onClick={() => onMove(item.id, modelIndex - 1)}
                        title={
                          item.backdrop ? t("sources-backdrop-pinned") : t("sources-lower-title")
                        }
                        aria-label={t("sources-lower-item", {
                          name: source?.name ?? t("sources-fallback-name"),
                        })}
                        className="rounded px-1 text-[10px] text-havoc-muted enabled:hover:text-havoc-text disabled:opacity-40"
                      >
                        ▼
                      </button>
                      <button
                        type="button"
                        onClick={() => onOpenFilters(item.id)}
                        title={t("sources-filters-title")}
                        aria-label={t("sources-filters-item", {
                          name: source?.name ?? t("sources-fallback-name"),
                        })}
                        className="rounded px-1 text-[10px] text-havoc-muted hover:text-havoc-text"
                      >
                        ƒ
                      </button>
                      <button
                        type="button"
                        onClick={() => onOpenProperties(item.source)}
                        title={t("sources-properties-title")}
                        aria-label={t("sources-properties-item", {
                          name: source?.name ?? t("sources-fallback-name"),
                        })}
                        className="rounded px-1 text-[10px] text-havoc-muted hover:text-havoc-text"
                      >
                        ⚙
                      </button>
                      <button
                        type="button"
                        onClick={() => onAddExisting(item.source)}
                        title={t("sources-clone-title")}
                        aria-label={t("sources-clone-item", {
                          name: source?.name ?? t("sources-fallback-name"),
                        })}
                        className="rounded px-1 text-[10px] text-havoc-muted hover:text-havoc-text"
                      >
                        ⧉
                      </button>
                    </span>
                    {/* Remove — last in the top action strip. */}
                    <button
                      type="button"
                      onClick={() => onRemove(item.id)}
                      title={t("sources-remove-title")}
                      aria-label={t("sources-remove-item", {
                        name: source?.name ?? t("sources-fallback-name"),
                      })}
                      className="shrink-0 rounded px-1 text-xs text-havoc-muted hover:text-red-400"
                    >
                      ×
                    </button>
                  </div>
                </div>
              </li>
            );
          })}
        </ul>
      )}

      {hdrFor && (
        <HdrToneMapDialog
          captureId={hdrFor.captureId}
          name={hdrFor.name}
          onClose={() => setHdrFor(null)}
        />
      )}

      {picker === "display" || picker === "window" ? (
        <CapturePicker
          mode={picker}
          os={os}
          onClose={() => setPicker(null)}
          onPick={pick}
          onPickLinked={(captureId, label) => {
            setPicker(null);
            if (!scene) return;
            studioAddLinkedWindow(scene.id, captureId, label).catch((err) =>
              console.error("linked window add failed:", err),
            );
          }}
        />
      ) : picker === "webcam" ? (
        <WebcamPicker onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "audioInput" || picker === "audioOutput" ? (
        <AudioPicker mode={picker} onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "appAudio" ? (
        <AppAudioPicker onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "gameCapture" ? (
        <GameCapturePicker
          onClose={() => setPicker(null)}
          onUseWindowCapture={() => setPicker("window")}
        />
      ) : picker === "image" ? (
        <ImageForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "media" ? (
        <MediaForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "remoteGuest" && scene ? (
        <RemoteGuestForm sceneId={scene.id} onClose={() => setPicker(null)} />
      ) : picker === "color" ? (
        <ColorForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "text" ? (
        <TextForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "chatOverlay" ? (
        <ChatOverlayForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "slideshow" ? (
        <SlideshowForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "testSignal" ? (
        <TestSignalForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "timer" ? (
        <TimerForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "startingSoon" ? (
        <StartingSoonForm
          onClose={() => setPicker(null)}
          onPick={pick}
          sceneId={scene?.id ?? null}
        />
      ) : picker === "backgroundMusic" ? (
        <BackgroundMusicForm onClose={() => setPicker(null)} scene={scene} />
      ) : picker === "systemStats" ? (
        <SystemStatsForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "audioVisualizer" ? (
        <VisualizerForm
          sources={(collection?.sources ?? [])
            .filter((source) => kindHasAudio(source.kind))
            .map((source) => ({ id: source.id, name: source.name }))}
          onClose={() => setPicker(null)}
          onPick={pick}
        />
      ) : picker === "splitTimer" ? (
        <SplitTimerForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "title" ? (
        <TitleForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "socialBar" ? (
        <SocialBarForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "inputOverlay" ? (
        <InputOverlayForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "playlist" ? (
        <PlaylistForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "replayPlayback" ? (
        <ReplayPlaybackForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "lanIngest" ? (
        <LanIngestForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "freallyLink" ? (
        <FreallyLinkForm onClose={() => setPicker(null)} onPick={pick} />
      ) : picker === "nestedScene" ? (
        <NestedSceneForm
          collection={collection}
          currentScene={scene?.id ?? null}
          onClose={() => setPicker(null)}
          onPick={pick}
        />
      ) : picker === "existing" ? (
        <ExistingPicker
          collection={collection}
          onClose={() => setPicker(null)}
          onPick={(sourceId) => {
            setPicker(null);
            onAddExisting(sourceId);
          }}
        />
      ) : null}
      {groupPick !== null && (
        <div className="mt-2 flex items-center gap-2">
          <button
            type="button"
            disabled={groupPick.length < 2}
            onClick={createGroup}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2 py-1 text-[11px] font-semibold text-havoc-text disabled:opacity-50"
          >
            {t("sources-create-group", { count: groupPick.length })}
          </button>
          <button
            type="button"
            onClick={() => setGroupPick(null)}
            className="rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:text-havoc-text"
          >
            {t("sources-cancel")}
          </button>
        </div>
      )}
      {groups.length > 0 && (
        <ul
          className="m-0 mt-2 flex list-none flex-col gap-1 p-0"
          aria-label={t("sources-groups-aria")}
        >
          {groups.map((group) => (
            <li
              key={group.id}
              className="flex items-center gap-1.5 rounded-lg border border-white/10 bg-white/[0.02] px-1.5 py-1 text-[11px]"
            >
              <button
                type="button"
                onClick={() => {
                  if (!scene) return;
                  studioSetGroupVisible(scene.id, group.id, !group.visible).catch((err) =>
                    console.error("group visibility failed:", err),
                  );
                }}
                title={group.visible ? t("sources-hide-group") : t("sources-show-group")}
                aria-pressed={group.visible}
                className={`shrink-0 rounded px-1 ${
                  group.visible ? "text-havoc-text" : "text-havoc-muted opacity-50"
                }`}
              >
                {group.visible ? "👁" : "–"}
              </button>
              <span className="min-w-0 flex-1 truncate text-havoc-text">
                ⊞ {group.name}
                <span className="text-havoc-muted">
                  {" "}
                  {t("sources-item-count", { count: group.items.length })}
                </span>
              </span>
              <button
                type="button"
                onClick={() => {
                  if (!scene) return;
                  studioUngroup(scene.id, group.id).catch((err) =>
                    console.error("ungroup failed:", err),
                  );
                }}
                title={t("sources-ungroup-title")}
                aria-label={t("sources-ungroup-item", { name: group.name })}
                className="shrink-0 rounded px-1 text-havoc-muted hover:text-red-300"
              >
                ✕
              </button>
            </li>
          ))}
        </ul>
      )}
      {showLayout && (
        <LayoutPicker collection={collection} scene={scene} onClose={() => setShowLayout(false)} />
      )}
    </Panel>
  );
}

// ---------------------------------------------------------------------------
// Pickers
// ---------------------------------------------------------------------------

/** Live Chat Overlay (TASK-613): a transparent, time-stamped record of the
 * incoming chat. NO API key, developer account, or sign-in — ever: YouTube
 * reads via the owned InnerTube client (exactly like the web player),
 * Twitch via anonymous IRC, Kick via its public endpoint. */
function ChatOverlayForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [youtube, setYoutube] = useState("");
  const [twitch, setTwitch] = useState("");
  const [kick, setKick] = useState("");
  const fieldClass =
    "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";
  const any = Boolean(youtube.trim() || twitch.trim() || kick.trim());

  return (
    <PickerShell title={t("sources-chat-title")} onClose={onClose}>
      <div className="flex flex-col gap-2 text-xs text-havoc-text">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-chat-youtube-label")}
          <input
            value={youtube}
            onChange={(event) => setYoutube(event.target.value)}
            placeholder={t("sources-chat-youtube-placeholder")}
            className={`${fieldClass} font-mono`}
          />
        </label>
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-chat-twitch-label")}
          <input
            value={twitch}
            onChange={(event) => setTwitch(event.target.value)}
            placeholder={t("sources-chat-twitch-placeholder")}
            className={`${fieldClass} font-mono`}
          />
        </label>
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-chat-kick-label")}
          <input
            value={kick}
            onChange={(event) => setKick(event.target.value)}
            placeholder={t("sources-chat-kick-placeholder")}
            className={`${fieldClass} font-mono`}
          />
        </label>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("sources-chat-note")}</p>
        <button
          type="button"
          disabled={!any}
          onClick={() =>
            onPick(
              {
                kind: "chatOverlay",
                youtube: youtube.trim(),
                twitch: twitch.trim(),
                kick: kick.trim(),
                width: 480,
                maxLines: 12,
                fontSize: 22,
              },
              t("sources-chat-default-name"),
            )
          }
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          {t("sources-chat-add")}
        </button>
      </div>
    </PickerShell>
  );
}

/** Image Slideshow (TASK-607): an ordered image set cycling on a timer,
 * with an optional crossfade (equal sizes only — different sizes hard-cut),
 * loop/hold-last and shuffle. */
function SlideshowForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [paths, setPaths] = useState<string[]>([]);
  const [slideMs, setSlideMs] = useState(5000);
  const [transitionMs, setTransitionMs] = useState(300);
  const [loop, setLoop] = useState(true);
  const [shuffle, setShuffle] = useState(false);

  const browse = () => {
    void open({
      multiple: true,
      filters: [
        { name: "Images", extensions: ["png", "jpg", "jpeg", "bmp", "gif", "webp", "tif"] },
      ],
    }).then((picked) => {
      if (Array.isArray(picked)) setPaths((current) => [...current, ...picked]);
      else if (typeof picked === "string") setPaths((current) => [...current, picked]);
    });
  };

  return (
    <PickerShell title={t("sources-slideshow-title")} onClose={onClose}>
      <div className="flex flex-col gap-2 text-xs text-havoc-text">
        {paths.length === 0 ? (
          <EmptyHint>{t("sources-slideshow-empty")}</EmptyHint>
        ) : (
          <ul className="m-0 flex max-h-40 list-none flex-col gap-1 overflow-y-auto p-0">
            {paths.map((path, index) => (
              <li
                key={`${path}-${index}`}
                className="flex items-center gap-1.5 rounded border border-white/10 px-1.5 py-1 text-[11px]"
              >
                <span className="min-w-0 flex-1 truncate" title={path}>
                  {path.split(/[\\/]/).pop()}
                </span>
                <button
                  type="button"
                  onClick={() => setPaths(paths.filter((_, at) => at !== index))}
                  aria-label={t("sources-slideshow-remove-slide", { number: index + 1 })}
                  className="shrink-0 rounded px-1 text-havoc-muted hover:text-red-300"
                >
                  ✕
                </button>
              </li>
            ))}
          </ul>
        )}
        <button
          type="button"
          onClick={browse}
          className="self-start rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:text-havoc-text"
        >
          {t("sources-slideshow-browse")}
        </button>
        <div className="grid grid-cols-2 gap-2">
          <NumberField
            label={t("sources-slideshow-per-slide-label")}
            value={slideMs}
            min={100}
            max={600000}
            step={500}
            onCommit={(value) => setSlideMs(Math.round(value))}
          />
          <NumberField
            label={t("sources-slideshow-crossfade-label")}
            value={transitionMs}
            min={0}
            max={5000}
            step={50}
            onCommit={(value) => setTransitionMs(Math.round(value))}
          />
        </div>
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          <input type="checkbox" checked={loop} onChange={(e) => setLoop(e.target.checked)} />
          {t("sources-slideshow-loop-label")}
        </label>
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          <input type="checkbox" checked={shuffle} onChange={(e) => setShuffle(e.target.checked)} />
          {t("sources-slideshow-shuffle-label")}
        </label>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("sources-slideshow-note")}
        </p>
        <button
          type="button"
          disabled={paths.length === 0}
          onClick={() => onPick({ kind: "slideshow", paths, slideMs, transitionMs, loop, shuffle })}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          {t("sources-slideshow-add", { count: paths.length })}
        </button>
      </div>
    </PickerShell>
  );
}

/** Nested Scene (TASK-605): compose another scene as a source — cycle-safe
 * (a scene that already contains this one is rejected by the model with an
 * honest error). */
function NestedSceneForm({
  collection,
  currentScene,
  onClose,
  onPick,
}: {
  collection: Collection | null;
  currentScene: SceneId | null;
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const candidates = (collection?.scenes ?? []).filter((entry) => entry.id !== currentScene);
  return (
    <PickerShell title={t("sources-nested-title")} onClose={onClose}>
      {candidates.length === 0 ? (
        <EmptyHint>{t("sources-nested-empty")}</EmptyHint>
      ) : (
        <ul className="m-0 flex list-none flex-col gap-1 p-0">
          {candidates.map((entry) => (
            <li key={entry.id}>
              <button
                type="button"
                onClick={() =>
                  onPick(
                    { kind: "nestedScene", scene: entry.id },
                    t("sources-nested-scene-name", { name: entry.name }),
                  )
                }
                className="w-full rounded-lg border border-white/10 bg-white/[0.02] px-2 py-1.5 text-left text-xs text-havoc-text hover:border-havoc-accent/50"
              >
                {entry.name}
                <span className="text-havoc-muted">
                  {" "}
                  {t("sources-item-count", { count: entry.items.length })}
                </span>
              </button>
            </li>
          ))}
        </ul>
      )}
      <p className="m-0 mt-2 text-[10px] leading-snug text-havoc-muted">
        {t("sources-nested-note")}
      </p>
    </PickerShell>
  );
}

function CapturePicker({
  mode,
  os,
  onClose,
  onPick,
  onPickLinked,
}: {
  mode: "display" | "window";
  os?: string;
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
  /** CAP-N73 (Windows): add the window + its app's audio as a linked pair. */
  onPickLinked: (captureId: string, label: string) => void;
}) {
  const t = useT();
  const [entries, setEntries] = useState<CaptureSource[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  // CAP-N73: "also capture this app's audio" — offered on Windows only
  // (per-app audio is Windows-first), default on.
  const [linkAudio, setLinkAudio] = useState(true);
  const canLink = mode === "window" && os === "windows";
  const loadedRef = useRef(false);

  const refresh = useCallback(() => {
    captureListSources()
      .then((all) => {
        // The portal pseudo-source stands in for both modes (Wayland).
        setEntries(all.filter((s) => s.kind === mode || s.kind === "portal"));
        setError(null);
        loadedRef.current = true;
      })
      .catch((err) => {
        // Only surface a failure before the first successful load; ignore
        // transient refresh errors so the last good list stays put.
        if (!loadedRef.current) setError(String(err));
      });
  }, [mode]);

  useEffect(() => {
    refresh();
    // Re-scan while the picker is open so a window you restore (e.g. from the
    // system tray) shows up within a couple seconds without reopening — the ↻
    // button in the header does the same on demand.
    const timer = window.setInterval(refresh, 2000);
    return () => window.clearInterval(timer);
  }, [refresh]);

  const title =
    mode === "display" ? t("sources-capture-display-title") : t("sources-capture-window-title");
  const hasPortal = entries?.some((s) => s.kind === "portal") ?? false;
  // Window mode shows a live thumbnail grid; everything else (displays, and the
  // Wayland portal entry in either mode) stays a text row.
  const windowTiles =
    mode === "window" ? (entries ?? []).filter((entry) => entry.kind === "window") : [];
  const listEntries = (entries ?? []).filter(
    (entry) => !(mode === "window" && entry.kind === "window"),
  );

  return (
    <PickerShell title={title} onClose={onClose} onRefresh={refresh}>
      {error ? (
        <p className="m-0 text-xs text-red-400">{error}</p>
      ) : entries === null ? (
        <p className="m-0 text-xs text-havoc-muted">{t("sources-capture-looking")}</p>
      ) : entries.length === 0 ? (
        <p className="m-0 text-xs text-havoc-muted">
          {mode === "display"
            ? t("sources-capture-none-displays")
            : t("sources-capture-none-windows")}
        </p>
      ) : (
        <>
          {canLink && (
            <label className="mb-2 flex items-center gap-2 text-xs text-havoc-muted">
              <input
                type="checkbox"
                checked={linkAudio}
                onChange={(event) => setLinkAudio(event.target.checked)}
              />
              {t("sources-link-audio")}
            </label>
          )}
          {windowTiles.length > 0 && (
            <div className="grid grid-cols-2 gap-2">
              {windowTiles.map((entry, index) => (
                <WindowThumbTile
                  key={entry.id}
                  entry={entry}
                  index={index}
                  onPick={() =>
                    canLink && linkAudio
                      ? onPickLinked(entry.id, entry.label)
                      : onPick(
                          { kind: "window", captureId: entry.id, label: entry.label },
                          entry.label,
                        )
                  }
                />
              ))}
            </div>
          )}
          {listEntries.length > 0 && (
            <ul
              className={`m-0 flex list-none flex-col gap-1 p-0 ${windowTiles.length > 0 ? "mt-2" : ""}`}
            >
              {listEntries.map((entry) => (
                <li key={entry.id}>
                  <button
                    type="button"
                    onClick={() => {
                      if (entry.kind === "window" && canLink && linkAudio) {
                        onPickLinked(entry.id, entry.label);
                        return;
                      }
                      onPick(
                        entry.kind === "portal"
                          ? { kind: "portal" }
                          : {
                              kind: entry.kind as "display" | "window",
                              captureId: entry.id,
                              label: entry.label,
                            },
                        entry.label,
                      );
                    }}
                    className="w-full truncate rounded-md border border-white/10 px-2 py-1.5 text-left text-xs text-havoc-text hover:border-havoc-accent/50"
                  >
                    {entry.label}
                    {entry.width > 0 && (
                      <span className="ml-1.5 text-havoc-muted">
                        {entry.width}×{entry.height}
                      </span>
                    )}
                  </button>
                </li>
              ))}
            </ul>
          )}
          {hasPortal && (
            <p className="mt-2 mb-0 text-[11px] leading-relaxed text-havoc-muted">
              {t("sources-capture-portal-note")}
            </p>
          )}
          {mode === "window" && windowTiles.length > 0 && (
            <p className="mt-2 mb-0 text-[10px] leading-snug text-havoc-muted">
              {t("sources-capture-window-note")}
            </p>
          )}
        </>
      )}
    </PickerShell>
  );
}

/**
 * One window tile with a *live* preview: it re-requests the thumbnail on a ~1 s
 * timer while the picker is open. Visible windows update; a minimized one keeps
 * its last frame (or shows a placeholder until/unless it's restored).
 */
function WindowThumbTile({
  entry,
  index,
  onPick,
}: {
  entry: CaptureSource;
  index: number;
  onPick: () => void;
}) {
  const t = useT();
  const [thumb, setThumb] = useState<string | null>(null);
  const [tried, setTried] = useState(false);

  useEffect(() => {
    let cancelled = false;
    let inFlight = false;
    let interval: number | undefined;
    const tick = () => {
      if (inFlight) return;
      inFlight = true;
      captureWindowThumbnail(entry.id, 320)
        .then((uri) => {
          if (!cancelled && uri) setThumb(uri);
        })
        .catch(() => {
          // Keep the last good frame; the placeholder covers the "never" case.
        })
        .finally(() => {
          inFlight = false;
          if (!cancelled) setTried(true);
        });
    };
    // Each grab briefly spins up the real capture backend, so stagger the first
    // one by tile index (no burst when the picker opens), then refresh on a
    // gentle interval for a live-ish preview.
    const startDelay = Math.min(index * 250, 1500);
    const kickoff = window.setTimeout(() => {
      tick();
      interval = window.setInterval(tick, 3000);
    }, startDelay);
    return () => {
      cancelled = true;
      window.clearTimeout(kickoff);
      if (interval !== undefined) window.clearInterval(interval);
    };
  }, [entry.id, index]);

  return (
    <button
      type="button"
      onClick={onPick}
      title={entry.label}
      className="group flex flex-col overflow-hidden rounded-md border border-white/10 text-left transition-colors hover:border-havoc-accent/50"
    >
      <div className="flex aspect-video w-full items-center justify-center bg-black/40">
        {thumb ? (
          <img src={thumb} alt="" className="h-full w-full object-contain" />
        ) : (
          <span className="text-[10px] text-havoc-muted">
            {tried ? t("sources-thumb-no-preview") : t("sources-thumb-loading")}
          </span>
        )}
      </div>
      <span className="truncate px-1.5 py-1 text-[11px] text-havoc-text">{entry.label}</span>
    </button>
  );
}

function WebcamPicker({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [devices, setDevices] = useState<VideoDevice[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [selected, setSelected] = useState<VideoDevice | null>(null);
  const [formatsFor, setFormatsFor] = useState<{ deviceId: string; list: VideoFormat[] } | null>(
    null,
  );
  const formatRef = useRef<HTMLSelectElement>(null);

  useEffect(() => {
    let cancelled = false;
    videoDevicesList()
      .then((list) => {
        if (!cancelled) setDevices(list);
      })
      .catch((err) => {
        if (!cancelled) setError(String(err));
      });
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (!selected) return;
    let cancelled = false;
    const deviceId = selected.id;
    videoDeviceFormats(deviceId)
      .then((list) => {
        if (!cancelled) setFormatsFor({ deviceId, list });
      })
      .catch(() => {
        // Format probing is best-effort (a live device can't be probed);
        // "Auto" still works.
        if (!cancelled) setFormatsFor({ deviceId, list: [] });
      });
    return () => {
      cancelled = true;
    };
  }, [selected]);

  const formats = selected && formatsFor?.deviceId === selected.id ? formatsFor.list : null;

  // Capture-card format presets (TASK-607): the common Elgato/AVerMedia
  // modes, offered when the device looks like a card and actually
  // advertises a matching format — never an invented mode.
  const looksLikeCard = /elgato|avermedia|aver media|cam link|live gamer|capture/i.test(
    selected?.name ?? "",
  );
  const cardPresets: Array<[string, number]> = looksLikeCard
    ? (
        [
          ["4K30", [3840, 2160, 30]],
          ["1080p60", [1920, 1080, 60]],
          ["1080p30", [1920, 1080, 30]],
          ["720p60", [1280, 720, 60]],
        ] as Array<[string, [number, number, number]]>
      )
        .map(([label, [w, h, fps]]): [string, number] => [
          label,
          (formats ?? []).findIndex(
            (format) => format.width === w && format.height === h && format.fps === fps,
          ),
        ])
        .filter(([, index]) => index >= 0)
    : [];

  const applyPreset = (index: number) => {
    if (formatRef.current) formatRef.current.value = String(index);
  };

  const add = () => {
    if (!selected) return;
    const index = formatRef.current ? Number(formatRef.current.value) : -1;
    const format = formats && index >= 0 ? formats[index] : null;
    onPick(
      {
        kind: "videoDevice",
        deviceId: selected.id,
        format,
        deinterlace: "off",
        fieldOrder: "topFirst",
      },
      selected.name,
    );
  };

  return (
    <PickerShell title={t("sources-webcam-title")} onClose={onClose}>
      {error ? (
        <p className="m-0 text-xs text-red-400">{error}</p>
      ) : devices === null ? (
        <p className="m-0 text-xs text-havoc-muted">{t("sources-webcam-looking")}</p>
      ) : devices.length === 0 ? (
        <p className="m-0 text-xs text-havoc-muted">{t("sources-webcam-none")}</p>
      ) : (
        <div className="flex flex-col gap-2">
          <ul className="m-0 flex list-none flex-col gap-1 p-0">
            {devices.map((device) => (
              <li key={device.id}>
                <button
                  type="button"
                  onClick={() => setSelected(device)}
                  aria-pressed={selected?.id === device.id}
                  className={`w-full truncate rounded-md border px-2 py-1.5 text-left text-xs text-havoc-text ${
                    selected?.id === device.id
                      ? "border-havoc-accent/60 bg-havoc-accent/10"
                      : "border-white/10 hover:border-havoc-accent/50"
                  }`}
                >
                  {device.name}
                </button>
              </li>
            ))}
          </ul>
          {selected && (
            <>
              <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
                {t("sources-webcam-format-label")}
                <select
                  ref={formatRef}
                  defaultValue={-1}
                  className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text"
                >
                  <option value={-1}>
                    {formats === null
                      ? t("sources-webcam-format-auto-loading")
                      : t("sources-webcam-format-auto")}
                  </option>
                  {(formats ?? []).map((format, index) => (
                    <option
                      key={`${format.width}x${format.height}@${format.fps}-${format.fourcc}`}
                      value={index}
                    >
                      {format.width}×{format.height} @ {format.fps} fps ({format.fourcc})
                    </option>
                  ))}
                </select>
              </label>
              {cardPresets.length > 0 && (
                <div className="flex flex-wrap items-center gap-1.5">
                  <span className="text-[10px] text-havoc-muted">
                    {t("sources-webcam-card-presets-label")}
                  </span>
                  {cardPresets.map(([label, index]) => (
                    <button
                      key={label}
                      type="button"
                      onClick={() => applyPreset(index)}
                      title={t("sources-webcam-preset-title", { label })}
                      className="rounded-md border border-white/10 px-2 py-0.5 text-[11px] text-havoc-muted hover:border-havoc-accent/60 hover:text-havoc-text"
                    >
                      {label}
                    </button>
                  ))}
                </div>
              )}
              <button
                type="button"
                onClick={add}
                className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
              >
                {t("sources-webcam-add")}
              </button>
            </>
          )}
        </div>
      )}
    </PickerShell>
  );
}

function AudioPicker({
  mode,
  onClose,
  onPick,
}: {
  mode: "audioInput" | "audioOutput";
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [devices, setDevices] = useState<AudioDevice[] | null>(null);
  const [guidance, setGuidance] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const isLoopback = mode === "audioOutput";

  useEffect(() => {
    let cancelled = false;
    if (isLoopback) {
      audioLoopbackDevices()
        .then((result) => {
          if (cancelled) return;
          setDevices(result.devices);
          setGuidance(result.guidance ?? null);
        })
        .catch((err) => {
          if (!cancelled) setError(String(err));
        });
    } else {
      audioInputDevices()
        .then((list) => {
          if (!cancelled) setDevices(list);
        })
        .catch((err) => {
          if (!cancelled) setError(String(err));
        });
    }
    return () => {
      cancelled = true;
    };
  }, [isLoopback]);

  const title = isLoopback ? t("sources-audio-output-title") : t("sources-audio-input-title");
  // Windows loopback (no guidance) can capture the default output; elsewhere
  // an explicit monitor/virtual device pick is the honest requirement.
  const offerDefault = !isLoopback || (devices !== null && guidance === null);
  const entries: Array<{ id: string; name: string }> = [
    ...(offerDefault
      ? [
          {
            id: "",
            name: isLoopback ? t("sources-audio-default-output") : t("sources-audio-default-input"),
          },
        ]
      : []),
    ...(devices ?? []),
  ];

  return (
    <PickerShell title={title} onClose={onClose}>
      {error ? (
        <p className="m-0 text-xs text-red-400">{error}</p>
      ) : devices === null ? (
        <p className="m-0 text-xs text-havoc-muted">{t("sources-audio-looking")}</p>
      ) : (
        <div className="flex flex-col gap-2">
          {entries.length === 0 ? (
            <p className="m-0 text-xs text-havoc-muted">
              {isLoopback ? t("sources-audio-none-output") : t("sources-audio-none-input")}
            </p>
          ) : (
            <ul className="m-0 flex list-none flex-col gap-1 p-0">
              {entries.map((device) => (
                <li key={device.id || "(default)"}>
                  <button
                    type="button"
                    onClick={() =>
                      onPick(
                        { kind: mode, deviceId: device.id },
                        device.id === "" ? undefined : device.name,
                      )
                    }
                    className="w-full truncate rounded-md border border-white/10 px-2 py-1.5 text-left text-xs text-havoc-text hover:border-havoc-accent/50"
                  >
                    {device.name}
                  </button>
                </li>
              ))}
            </ul>
          )}
          {guidance && (
            <p className="m-0 rounded-md border border-amber-400/20 bg-amber-400/5 p-2 text-[11px] leading-relaxed text-amber-200/90">
              {guidance}
            </p>
          )}
          {!isLoopback && (
            <p className="m-0 text-[10px] leading-snug text-havoc-muted">
              {t("sources-audio-input-note")}
            </p>
          )}
        </div>
      )}
    </PickerShell>
  );
}

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/**
 * The App Audio picker (TASK-805): pick one running application to capture as
 * its own mixer source. Windows lists apps making sound now (WASAPI process
 * loopback); other OSes show the honest per-OS guidance instead of a fake
 * toggle. A ⟳ refresh re-scans since apps come and go.
 */
function AppAudioPicker({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [list, setList] = useState<AppAudioList | null>(null);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(() => {
    setError(null);
    appAudioApps()
      .then(setList)
      .catch((err) => setError(String(err)));
  }, []);

  useEffect(() => {
    // Initial scan — set state only in the async callback (never synchronously
    // inside the effect); the Refresh button uses `load` (sync setError is fine
    // from an event handler).
    let cancelled = false;
    appAudioApps()
      .then((result) => {
        if (!cancelled) setList(result);
      })
      .catch((err) => {
        if (!cancelled) setError(String(err));
      });
    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <PickerShell title={t("sources-appaudio-title")} onClose={onClose}>
      {error ? (
        <p className="m-0 text-xs text-red-400">{error}</p>
      ) : list === null ? (
        <p className="m-0 text-xs text-havoc-muted">{t("sources-appaudio-looking")}</p>
      ) : (
        <div className="flex flex-col gap-2">
          {list.supported && list.apps.length > 0 ? (
            <ul className="m-0 flex list-none flex-col gap-1 p-0">
              {list.apps.map((app) => (
                <li key={app.pid}>
                  <button
                    type="button"
                    onClick={() =>
                      onPick({ kind: "appAudio", pid: app.pid, exe: app.exe }, app.name)
                    }
                    className="flex w-full items-center justify-between gap-2 truncate rounded-md border border-white/10 px-2 py-1.5 text-left text-xs text-havoc-text hover:border-havoc-accent/50"
                  >
                    <span className="truncate">{app.name}</span>
                    <span className="shrink-0 font-mono text-[10px] text-havoc-muted">
                      {app.exe} · {app.pid}
                    </span>
                  </button>
                </li>
              ))}
            </ul>
          ) : (
            <p className="m-0 rounded-md border border-amber-400/20 bg-amber-400/5 p-2 text-[11px] leading-relaxed text-amber-200/90">
              {list.supported ? t("sources-appaudio-none") : list.guidance}
            </p>
          )}
          <div className="flex items-center justify-between">
            <button
              type="button"
              onClick={load}
              className="rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
            >
              {t("sources-appaudio-refresh")}
            </button>
            {list.supported && (
              <p className="m-0 text-[10px] leading-snug text-havoc-muted">
                {t("sources-appaudio-note")}
              </p>
            )}
          </div>
        </div>
      )}
    </PickerShell>
  );
}

/**
 * Game Capture (TASK-801): honest, never-inject-silently. The injected GPU-API
 * hook is a flagged milestone; this panel shows the anti-cheat/AV risk and
 * routes the user to the working path (Window Capture, or the portal on
 * Wayland). Nothing is injected from here.
 */
function GameCapturePicker({
  onClose,
  onUseWindowCapture,
}: {
  onClose: () => void;
  onUseWindowCapture: () => void;
}) {
  const t = useT();
  const [status, setStatus] = useState<GameCaptureStatus | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let alive = true;
    gameCaptureStatus()
      .then((s) => {
        if (alive) setStatus(s);
      })
      .catch((err) => {
        if (alive) setError(String(err));
      });
    return () => {
      alive = false;
    };
  }, []);

  return (
    <PickerShell title={t("sources-game-title")} onClose={onClose}>
      {error ? (
        <p className="m-0 text-xs text-red-400">{error}</p>
      ) : status === null ? (
        <p className="m-0 text-xs text-havoc-muted">{t("sources-game-checking")}</p>
      ) : (
        <div className="flex flex-col gap-3 text-xs">
          <p className="m-0 rounded-md border border-red-400/25 bg-red-400/5 p-2 leading-relaxed text-red-200/90">
            {status.risk}
          </p>
          <p className="m-0 leading-relaxed text-havoc-muted">{status.guidance}</p>
          <div className="flex flex-wrap gap-2">
            <button
              type="button"
              onClick={onUseWindowCapture}
              className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
            >
              {status.fallback === "portal"
                ? t("sources-game-use-portal")
                : t("sources-game-use-window")}
            </button>
            <button
              type="button"
              onClick={onClose}
              className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
            >
              {t("sources-cancel")}
            </button>
          </div>
        </div>
      )}
    </PickerShell>
  );
}

function ImageForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [path, setPath] = useState("");
  const browse = () =>
    pickFile([{ name: "Images", extensions: ["png", "jpg", "jpeg", "bmp", "gif", "webp"] }]).then(
      (picked) => {
        if (picked) setPath(picked);
      },
    );
  return (
    <PickerShell title={t("sources-image-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-image-file-label")}
          <PathField
            value={path}
            onChange={setPath}
            onBrowse={browse}
            placeholder="C:\art\overlay.png"
          />
        </label>
        <button
          type="button"
          disabled={!path.trim()}
          onClick={() => onPick({ kind: "image", path: path.trim() })}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          {t("sources-image-add")}
        </button>
      </div>
    </PickerShell>
  );
}

/** CAP-N74: one display's HDR→SDR tone-map — operator + paper-white,
 * applied live (the capture retunes on its very next frame). */
function HdrToneMapDialog({
  captureId,
  name,
  onClose,
}: {
  captureId: string;
  name: string;
  onClose: () => void;
}) {
  const t = useT();
  const [operator, setOperator] = useState("clip");
  const [paperWhite, setPaperWhite] = useState(200);
  useEffect(() => {
    let alive = true;
    settingsGet()
      .then((settings) => {
        const saved = settings.hdrToneMap?.[captureId];
        if (alive && saved) {
          setOperator(saved.operator);
          setPaperWhite(saved.paperWhiteNits);
        }
      })
      .catch(() => undefined);
    return () => {
      alive = false;
    };
  }, [captureId]);

  const apply = (nextOperator: string, nextPaperWhite: number) => {
    setOperator(nextOperator);
    setPaperWhite(nextPaperWhite);
    hdrToneMapSet(captureId, nextOperator, nextPaperWhite).catch((err) =>
      console.error("hdr tone-map failed:", err),
    );
  };

  const OPERATORS: Array<[string, string]> = [
    ["clip", "sources-hdr-op-clip"],
    ["maxRgb", "sources-hdr-op-maxrgb"],
    ["reinhard", "sources-hdr-op-reinhard"],
    ["bt2408", "sources-hdr-op-bt2408"],
  ];

  return (
    <PickerShell title={t("sources-hdr-dialog-title", { name })} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-havoc-muted">{t("sources-hdr-hint")}</p>
        {operator === "clip" && (
          <button
            type="button"
            onClick={() => apply("maxRgb", 200)}
            className="self-start rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 font-semibold enabled:hover:bg-havoc-accent/25"
          >
            {t("sources-hdr-enable-suggested")}
          </button>
        )}
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          {t("sources-hdr-operator")}
          <select
            value={operator}
            onChange={(event) => apply(event.target.value, paperWhite)}
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text"
          >
            {OPERATORS.map(([value, key]) => (
              <option key={value} value={value}>
                {t(key)}
              </option>
            ))}
          </select>
        </label>
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          {t("sources-hdr-paper-white")}
          <input
            type="number"
            min={80}
            max={1000}
            step={10}
            value={paperWhite}
            onChange={(event) => {
              const next = Number(event.target.value);
              if (Number.isFinite(next) && next >= 80 && next <= 1000) {
                apply(operator, next);
              } else {
                setPaperWhite(next);
              }
            }}
            className="w-24 rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text"
          />
          <span>{t("sources-hdr-nits")}</span>
        </label>
      </div>
    </PickerShell>
  );
}

/** Open the native file dialog and return the chosen path (null if cancelled
 * or unavailable — the typed path still works as a fallback). */
async function pickFile(
  filters: Array<{ name: string; extensions: string[] }>,
): Promise<string | null> {
  try {
    const picked = await open({ multiple: false, directory: false, filters });
    return typeof picked === "string" ? picked : null;
  } catch (err) {
    console.error("file dialog failed:", err);
    return null;
  }
}

/** A path input paired with a native Browse button (the Tauri file dialog). */
function PathField({
  value,
  onChange,
  onBrowse,
  placeholder,
}: {
  value: string;
  onChange: (next: string) => void;
  onBrowse: () => void;
  placeholder: string;
}) {
  const t = useT();
  return (
    <div className="flex gap-2">
      <input
        value={value}
        onChange={(event) => onChange(event.target.value)}
        placeholder={placeholder}
        className={`${inputClass} min-w-0 flex-1`}
      />
      <button
        type="button"
        onClick={onBrowse}
        className="shrink-0 rounded-md border border-white/10 px-2.5 py-1.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
      >
        {t("sources-browse")}
      </button>
    </div>
  );
}

function MediaForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [path, setPath] = useState("");
  const [loop, setLoop] = useState(false);
  const browse = () =>
    pickFile([
      { name: "Media", extensions: ["mp4", "mkv", "webm", "mov", "frec", "png", "jpg", "jpeg"] },
    ]).then((picked) => {
      if (picked) setPath(picked);
    });
  return (
    <PickerShell title={t("sources-media-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-media-file-label")}
          <PathField
            value={path}
            onChange={setPath}
            onBrowse={browse}
            placeholder="C:\clips\intro.mp4"
          />
        </label>
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={loop}
            onChange={(event) => setLoop(event.target.checked)}
          />
          {t("sources-media-loop-label")}
        </label>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("sources-media-note")}</p>
        <button
          type="button"
          disabled={!path.trim()}
          onClick={() => onPick({ kind: "media", path: path.trim(), loop, hwDecode: true })}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          {t("sources-media-add")}
        </button>
      </div>
    </PickerShell>
  );
}

function ReplayPlaybackForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [seconds, setSeconds] = useState(15);
  const [speed, setSpeed] = useState<ReplaySpeed>("half");
  return (
    <PickerShell title={t("sources-replay-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <NumberField
          label={t("sources-replay-seconds-label")}
          value={seconds}
          min={2}
          max={300}
          onCommit={setSeconds}
        />
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-replay-speed-label")}
          <select
            value={speed}
            onChange={(event) => setSpeed(event.target.value as ReplaySpeed)}
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text"
          >
            {REPLAY_SPEEDS.map(([value, label]) => (
              <option key={value} value={value}>
                {t(label)}
              </option>
            ))}
          </select>
        </label>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("sources-replay-note")}</p>
        <button
          type="button"
          onClick={() => onPick({ kind: "replayPlayback", seconds, speed, hwDecode: true })}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
        >
          {t("sources-replay-add")}
        </button>
      </div>
    </PickerShell>
  );
}

/**
 * Freally Link receiver (CAP-N12): pick a discovered sender or type its
 * address. The scan is user-initiated only — nothing probes the network
 * until the button is pressed, and manual entry always works without it.
 */
function FreallyLinkForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [host, setHost] = useState("");
  const [port, setPort] = useState(9720);
  const [key, setKey] = useState("");
  const [peers, setPeers] = useState<LinkPeer[] | null>(null);
  const [scanning, setScanning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const scan = () => {
    setScanning(true);
    setError(null);
    linkDiscover()
      .then((found) => setPeers(found))
      .catch((err) => setError(String(err)))
      .finally(() => setScanning(false));
  };

  const keyReady = key.trim().length > 0;
  const add = (pickedHost: string, pickedPort: number, label: string) =>
    onPick(
      { kind: "freallyLink", host: pickedHost, port: pickedPort, label, key: key.trim() },
      label || undefined,
    );

  return (
    <PickerShell title={t("sources-link-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <p className="m-0 text-[11px] leading-snug text-havoc-muted">{t("sources-link-about")}</p>
        <button
          type="button"
          disabled={scanning}
          onClick={scan}
          className="self-start rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted enabled:hover:border-havoc-accent/50 enabled:hover:text-havoc-text disabled:opacity-60"
        >
          {scanning ? t("sources-link-scanning") : t("sources-link-scan")}
        </button>
        {error && <p className="m-0 text-xs text-red-400">{error}</p>}
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-link-key")}
          <input
            value={key}
            onChange={(event) => setKey(event.target.value)}
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 font-mono text-xs text-havoc-text"
          />
        </label>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("sources-link-key-hint")}
        </p>
        {peers !== null &&
          (peers.length > 0 ? (
            <ul className="m-0 flex list-none flex-col gap-1 p-0">
              {peers.map((peer) => (
                <li key={`${peer.host}:${peer.port}`}>
                  <button
                    type="button"
                    disabled={!keyReady}
                    onClick={() => add(peer.host, peer.port, peer.name)}
                    className="flex w-full items-center justify-between gap-2 truncate rounded-md border border-white/10 px-2 py-1.5 text-left text-xs text-havoc-text enabled:hover:border-havoc-accent/50 disabled:opacity-50"
                  >
                    <span className="truncate">{peer.name}</span>
                    <span className="shrink-0 font-mono text-[10px] text-havoc-muted">
                      {peer.host}:{peer.port}
                    </span>
                  </button>
                </li>
              ))}
            </ul>
          ) : (
            <p className="m-0 rounded-md border border-amber-400/20 bg-amber-400/5 p-2 text-[11px] leading-relaxed text-amber-200/90">
              {t("sources-link-none")}
            </p>
          ))}
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-link-host")}
          <input
            value={host}
            onChange={(event) => setHost(event.target.value)}
            placeholder="192.168.1.20"
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 font-mono text-xs text-havoc-text"
          />
        </label>
        <NumberField
          label={t("sources-link-port")}
          value={port}
          min={1}
          max={65535}
          onCommit={setPort}
        />
        <button
          type="button"
          disabled={!host.trim() || !keyReady}
          onClick={() => add(host.trim(), port, `${host.trim()}:${port}`)}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          {t("sources-link-add")}
        </button>
      </div>
    </PickerShell>
  );
}

function PlaylistForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [paths, setPaths] = useState("");
  const [loop, setLoop] = useState(true);
  const [shuffle, setShuffle] = useState(false);
  const [holdLast, setHoldLast] = useState(true);
  const items = paths
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean);
  const browse = () =>
    pickFile([
      { name: "Media", extensions: ["mp4", "mkv", "webm", "mov", "mp3", "wav", "flac", "m4a"] },
    ]).then((picked) => {
      if (picked) setPaths((current) => (current.trim() ? `${current.trim()}\n${picked}` : picked));
    });
  return (
    <PickerShell title={t("sources-playlist-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-playlist-files-label")}
          <textarea
            value={paths}
            onChange={(event) => setPaths(event.target.value)}
            rows={5}
            placeholder={"C:\\vt\\intro.mp4\nC:\\vt\\loop.mp4"}
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 font-mono text-xs text-havoc-text"
          />
        </label>
        <button
          type="button"
          onClick={browse}
          className="self-start rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:text-havoc-text"
        >
          {t("sources-playlist-browse")}
        </button>
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={loop}
            onChange={(event) => setLoop(event.target.checked)}
          />
          {t("sources-playlist-loop")}
        </label>
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={shuffle}
            onChange={(event) => setShuffle(event.target.checked)}
          />
          {t("sources-playlist-shuffle")}
        </label>
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={holdLast}
            onChange={(event) => setHoldLast(event.target.checked)}
          />
          {t("sources-playlist-hold-last")}
        </label>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("sources-playlist-note")}
        </p>
        <button
          type="button"
          disabled={items.length === 0}
          onClick={() =>
            onPick({
              kind: "playlist",
              items: items.map((path) => ({ path, in: 0, out: 0, cues: [] })),
              loop,
              shuffle,
              holdLast,
              hwDecode: true,
              nowPlayingVariable: "",
              // A manual Media Playlist shows its track-list on the canvas.
              hiddenFace: false,
            })
          }
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          {t("sources-playlist-add")}
        </button>
      </div>
    </PickerShell>
  );
}

function SplitTimerForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [path, setPath] = useState("");
  const [comparison, setComparison] = useState<SplitComparison>("personalBest");
  const browse = () =>
    pickFile([{ name: "LiveSplit splits", extensions: ["lss"] }]).then((picked) => {
      if (picked) setPath(picked);
    });
  return (
    <PickerShell title={t("sources-splits-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-splits-file-label")}
          <PathField
            value={path}
            onChange={setPath}
            onBrowse={browse}
            placeholder="C:\runs\any-percent.lss"
          />
        </label>
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-splits-comparison-label")}
          <select
            value={comparison}
            onChange={(event) => setComparison(event.target.value as SplitComparison)}
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text"
          >
            <option value="personalBest">{t("sources-splits-comparison-pb")}</option>
            <option value="bestSegments">{t("sources-splits-comparison-best")}</option>
            <option value="average">{t("sources-splits-comparison-average")}</option>
          </select>
        </label>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("sources-splits-note")}</p>
        <button
          type="button"
          disabled={!path.trim()}
          onClick={() =>
            onPick({
              kind: "splitTimer",
              path: path.trim(),
              comparison,
              width: 420,
              height: 380,
              sizePx: 18,
              color: { r: 255, g: 255, b: 255, a: 255 },
              ahead: { r: 34, g: 197, b: 94, a: 255 },
              behind: { r: 239, g: 68, b: 68, a: 255 },
              gold: { r: 251, g: 191, b: 36, a: 255 },
            })
          }
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          {t("sources-splits-add")}
        </button>
      </div>
    </PickerShell>
  );
}

/** Lower-third starter: an accent bar + name + subtitle, scaled to the canvas. */
function lowerThirdLayers(w: number, h: number, t: (key: string) => string): TitleLayer[] {
  return [
    {
      kind: "rect",
      x: Math.round(w * 0.03),
      y: Math.round(h * 0.78),
      width: Math.round(w * 0.42),
      height: Math.round(h * 0.13),
      color: { r: 74, g: 158, b: 255, a: 230 },
    },
    titleTextLayer({
      x: Math.round(w * 0.046),
      y: Math.round(h * 0.792),
      text: t("sources-title-template-name"),
      sizePx: Math.round(h * 0.052),
      shadow: true,
    }),
    titleTextLayer({
      x: Math.round(w * 0.046),
      y: Math.round(h * 0.856),
      text: t("sources-title-template-subtitle"),
      sizePx: Math.round(h * 0.032),
      color: { r: 255, g: 255, b: 255, a: 220 },
    }),
  ];
}

/** Scoreboard starter: a top plate + 4 cells (two names, two scores). */
function scoreboardLayers(w: number, h: number, t: (key: string) => string): TitleLayer[] {
  const y = Math.round(h * 0.055);
  return [
    {
      kind: "rect",
      x: Math.round(w * 0.29),
      y: Math.round(h * 0.037),
      width: Math.round(w * 0.42),
      height: Math.round(h * 0.085),
      color: { r: 12, g: 16, b: 28, a: 230 },
    },
    titleTextLayer({
      x: Math.round(w * 0.305),
      y,
      text: t("sources-title-template-home"),
      sizePx: Math.round(h * 0.042),
    }),
    titleTextLayer({
      x: Math.round(w * 0.46),
      y,
      text: "0",
      sizePx: Math.round(h * 0.048),
      outlinePx: 2,
    }),
    titleTextLayer({
      x: Math.round(w * 0.525),
      y,
      text: "0",
      sizePx: Math.round(h * 0.048),
      outlinePx: 2,
    }),
    titleTextLayer({
      x: Math.round(w * 0.6),
      y,
      text: t("sources-title-template-away"),
      sizePx: Math.round(h * 0.042),
    }),
  ];
}

function TitleForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [width, setWidth] = useState(1920);
  const [height, setHeight] = useState(1080);
  const [template, setTemplate] = useState<"lowerThird" | "scoreboard" | "blank">("lowerThird");
  const add = () => {
    const layers: TitleLayer[] =
      template === "lowerThird"
        ? lowerThirdLayers(width, height, t)
        : template === "scoreboard"
          ? scoreboardLayers(width, height, t)
          : [];
    const animation: TitleAnimation =
      template === "lowerThird" ? "slideLeft" : template === "scoreboard" ? "fade" : "none";
    onPick({ kind: "title", width, height, layers, animation, durationMs: 400 });
  };
  return (
    <PickerShell title={t("sources-title-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-title-template-label")}
          <select
            value={template}
            onChange={(event) =>
              setTemplate(event.target.value as "lowerThird" | "scoreboard" | "blank")
            }
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text"
          >
            <option value="lowerThird">{t("sources-title-template-lower-third")}</option>
            <option value="scoreboard">{t("sources-title-template-scoreboard")}</option>
            <option value="blank">{t("sources-title-template-blank")}</option>
          </select>
        </label>
        <div className="flex items-end gap-2">
          <NumberField
            label={t("sources-title-width-label")}
            value={width}
            min={16}
            max={16384}
            onCommit={setWidth}
            className="flex-1"
          />
          <NumberField
            label={t("sources-title-height-label")}
            value={height}
            min={16}
            max={16384}
            onCommit={setHeight}
            className="flex-1"
          />
        </div>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("sources-title-note")}</p>
        <button
          type="button"
          onClick={add}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
        >
          {t("sources-title-add")}
        </button>
      </div>
    </PickerShell>
  );
}

/**
 * V1-D: the Social & Channels bar — a generated on-canvas panel listing a
 * creator's social handles (a brand-coloured badge + platform name + handle
 * per row). Fully local: nothing is fetched, no logos are embedded, no files
 * are read. Ships empty-ish (one blank YouTube row) so it draws nothing until
 * the user types a handle; every field stays editable afterwards in Properties.
 */
function SocialBarForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [value, setValue] = useState<SocialBarValue>({
    header: t("sources-social-default-header"),
    rows: [newSocialRow()],
    fontFamily: null,
    sizePx: 32,
    color: { r: 255, g: 255, b: 255, a: 255 },
    background: { r: 10, g: 10, b: 15, a: 184 },
  });
  return (
    <PickerShell title={t("sources-social-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <SocialBarFields value={value} onChange={setValue} />
        <button
          type="button"
          onClick={() => onPick({ kind: "socialBar", ...value })}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
        >
          {t("sources-social-add")}
        </button>
      </div>
    </PickerShell>
  );
}

/**
 * LAN ingest listener setup (CAP-N11): a phone or second PC on the same
 * network feeds the scene over SRT (encryptable — preferred) or RTMP
 * (unauthenticated by protocol). Nothing listens until the source is added;
 * the listener never dials out. The URL + QR point the sender here.
 */
function LanIngestForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [value, setValue] = useState<LanIngestValue>({
    protocol: "srt",
    port: LAN_DEFAULT_PORTS.srt,
    passphrase: "",
  });
  const passUsable = lanPassphraseUsable(value.protocol, value.passphrase);
  return (
    <PickerShell title={t("sources-lan-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <LanIngestFields value={value} onChange={setValue} />
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("sources-lan-note")}</p>
        <button
          type="button"
          disabled={!passUsable}
          onClick={() =>
            onPick({
              kind: "lanIngest",
              ...value,
              passphrase: value.protocol === "srt" ? value.passphrase : "",
            })
          }
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          {t("sources-lan-add")}
        </button>
      </div>
    </PickerShell>
  );
}

/** `[minutes, i18n key]` — the call site renders `t(key)`. */
const INVITE_TTLS: Array<[number, string]> = [
  [15, "sources-ttl-15min"],
  [30, "sources-ttl-30min"],
  [60, "sources-ttl-1hour"],
  [1440, "sources-ttl-1day"],
];

/**
 * Remote Guest setup (TASK-R1 + R2/R3 invites). HOST: start a session and
 * share an expiring invite link; the guest's webcam lands in the scene when
 * they join. GUEST: paste the invite link (or a raw session id) and share this
 * machine's webcam. Media flows P2P (WebRTC); only signaling touches the
 * PeerJS broker — nothing runs until a session is started here. Session state
 * lives in the spike store, so closing this dialog changes nothing; the live
 * controls (mute, stop) sit on the main-UI session bar.
 */
function RemoteGuestForm({ sceneId, onClose }: { sceneId: SceneId; onClose: () => void }) {
  const t = useT();
  const session = useSyncExternalStore(spikeSubscribe, spikeGetState);
  const [hostId, setHostId] = useState("");
  const [ttl, setTtl] = useState(30);
  const [copied, setCopied] = useState(false);
  // Local-only errors (bad invite / copy failure) — the session status itself
  // comes from the store.
  const [formError, setFormError] = useState<string | null>(null);

  const link = session.role === "host" ? session.invite : null;
  // The QR carries the WEB join URL (docs/join.html) so a scanned phone lands
  // in the browser guest — the copyable freally:// link stays for app users.
  const qrToken = link ? parseInviteInput(link) : null;
  const qrLink = qrToken ? webJoinLink(qrToken) : null;
  const startHosting = () => {
    setFormError(null);
    void spikeHost(sceneId, ttl);
  };
  const changeTtl = (minutes: number) => {
    setTtl(minutes);
    spikeSetInviteTtl(minutes); // no-op unless hosting
  };

  const copyLink = () => {
    if (!link) return;
    navigator.clipboard
      .writeText(link)
      .then(() => {
        setCopied(true);
        window.setTimeout(() => setCopied(false), 1500);
      })
      .catch(() => setFormError(t("sources-remote-copy-failed")));
  };
  const join = () => {
    const target = joinTargetFromInput(hostId, Date.now());
    if ("error" in target) {
      setFormError(target.error);
      return;
    }
    setFormError(null);
    spikeJoin(target.peerId).catch((err) =>
      setFormError(t("sources-remote-join-failed", { error: String(err) })),
    );
  };

  return (
    <PickerShell title={t("sources-remote-title")} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <div className="flex flex-col gap-1.5">
          <p className="m-0 text-[11px] font-semibold uppercase tracking-wide text-havoc-muted">
            {t("sources-remote-host-heading")}
          </p>
          <div className="flex items-center gap-2">
            <button
              type="button"
              onClick={startHosting}
              className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
            >
              {t("sources-remote-start-hosting")}
            </button>
            <label className="flex items-center gap-1 text-[11px] text-havoc-muted">
              {t("sources-remote-expires-label")}
              <select
                value={ttl}
                onChange={(event) => changeTtl(Number(event.target.value))}
                aria-label={t("sources-remote-invite-expiry-aria")}
                className="rounded border border-white/10 bg-havoc-panel px-1.5 py-0.5 text-[11px] text-havoc-text"
              >
                {INVITE_TTLS.map(([minutes, label]) => (
                  <option key={minutes} value={minutes}>
                    {t(label)}
                  </option>
                ))}
              </select>
            </label>
          </div>
          {link && (
            <>
              <div className="flex gap-2">
                <input
                  readOnly
                  value={link}
                  onFocus={(event) => event.target.select()}
                  aria-label={t("sources-remote-invite-link-aria")}
                  className="min-w-0 flex-1 rounded border border-white/10 bg-black/30 px-2 py-1 font-mono text-[10px] text-havoc-text"
                />
                <button
                  type="button"
                  onClick={copyLink}
                  className="shrink-0 rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
                >
                  {copied ? t("sources-remote-copied") : t("sources-remote-copy")}
                </button>
              </div>
              <p className="m-0 text-[10px] leading-snug text-havoc-muted">
                {t("sources-remote-share-note")}
              </p>
              <div className="flex items-center gap-2">
                <QrSvg link={qrLink ?? link} />
                <p className="m-0 text-[10px] leading-snug text-havoc-muted">
                  {t("sources-remote-qr-note")}
                </p>
              </div>
            </>
          )}
        </div>
        <RemoteDevicePickers micId={session.micId} speakerId={session.speakerId} />
        <TurnRelaySection />
        <div className="flex flex-col gap-1.5 border-t border-white/5 pt-2">
          <p className="m-0 text-[11px] font-semibold uppercase tracking-wide text-havoc-muted">
            {t("sources-remote-guest-heading")}
          </p>
          <div className="flex gap-2">
            <input
              value={hostId}
              onChange={(event) => {
                setHostId(event.target.value);
                setFormError(null);
              }}
              placeholder={t("sources-remote-paste-placeholder")}
              aria-label={t("sources-remote-invite-input-aria")}
              className="min-w-0 flex-1 rounded border border-white/10 bg-black/30 px-2 py-1 font-mono text-[11px] text-havoc-text"
            />
            <button
              type="button"
              disabled={!hostId.trim()}
              onClick={join}
              className="shrink-0 rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25 disabled:opacity-60"
            >
              {t("sources-remote-join")}
            </button>
          </div>
        </div>
        {session.active && (
          <p className="m-0 border-t border-white/5 pt-2 text-[10px] leading-snug text-havoc-muted">
            {t("sources-remote-session-note")}
          </p>
        )}
        <div className="flex items-center justify-between gap-2 border-t border-white/5 pt-2">
          <p
            className={`m-0 flex-1 text-[11px] leading-snug ${
              formError ? "text-amber-300" : "text-havoc-muted"
            }`}
          >
            {formError ?? session.status}
          </p>
          <button
            type="button"
            onClick={() => {
              spikeStop();
              setFormError(null);
            }}
            className="rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
          >
            {t("sources-remote-stop-session")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}

/**
 * The session's audio devices — which microphone THIS machine sends (host
 * talkback / guest mic) and which output device the other side's audio plays
 * through — plus a self-test loop: talk and hear yourself before going live.
 * Selection applies live mid-session (replaceTrack / setSinkId) and persists.
 */
function RemoteDevicePickers({
  micId,
  speakerId,
}: {
  micId: string | null;
  speakerId: string | null;
}) {
  const t = useT();
  const [devices, setDevices] = useState<RemoteAudioDevices>({ inputs: [], outputs: [] });
  const [testNote, setTestNote] = useState<string | null>(null);
  const [testing, setTesting] = useState(false);
  const testStopRef = useRef<(() => void) | null>(null);

  useEffect(() => {
    let cancelled = false;
    const refresh = () => {
      listRemoteAudioDevices()
        .then((found) => {
          if (!cancelled) setDevices(found);
        })
        .catch(() => {});
    };
    refresh();
    const unlisten = onDeviceChange(refresh);
    return () => {
      cancelled = true;
      unlisten();
    };
  }, []);

  const stopTest = useCallback(() => {
    testStopRef.current?.();
    testStopRef.current = null;
    setTesting(false);
  }, []);
  // Closing the dialog releases the test mic.
  useEffect(() => stopTest, [stopTest]);

  const startTest = (mic: string | null, speaker: string | null) => {
    setTestNote(null);
    startMicTest(mic, speaker)
      .then(({ stop, sink }) => {
        testStopRef.current?.(); // a switch mid-test replaces the loop
        testStopRef.current = stop;
        setTesting(true);
        if (sink !== "ok") setTestNote(t("sources-devices-output-unavailable"));
        // The permission grant unlocks real device labels — refresh.
        listRemoteAudioDevices()
          .then(setDevices)
          .catch(() => {});
      })
      .catch((err) => {
        setTesting(false);
        setTestNote(t("sources-devices-mic-test-failed", { error: String(err) }));
      });
  };

  const pickMic = (next: string | null) => {
    void spikeSetMic(next);
    if (testStopRef.current) startTest(next, speakerId);
  };
  const pickSpeaker = (next: string | null) => {
    void spikeSetSpeaker(next);
    if (testStopRef.current) startTest(micId, next);
  };

  return (
    <div className="flex flex-col gap-1.5 border-t border-white/5 pt-2">
      <p className="m-0 text-[11px] font-semibold uppercase tracking-wide text-havoc-muted">
        {t("sources-devices-heading")}
      </p>
      <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
        <span className="w-20 shrink-0">{t("sources-devices-microphone-label")}</span>
        <select
          value={micId ?? ""}
          onChange={(event) => pickMic(event.target.value || null)}
          aria-label={t("sources-devices-microphone-aria")}
          className="min-w-0 flex-1 rounded border border-white/10 bg-havoc-panel px-1.5 py-0.5 text-[11px] text-havoc-text"
        >
          <option value="">{t("sources-devices-system-default")}</option>
          {devices.inputs.map((device) => (
            <option key={device.deviceId} value={device.deviceId}>
              {device.label}
            </option>
          ))}
        </select>
      </label>
      <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
        <span className="w-20 shrink-0">{t("sources-devices-output-label")}</span>
        <select
          value={speakerId ?? ""}
          onChange={(event) => pickSpeaker(event.target.value || null)}
          aria-label={t("sources-devices-output-aria")}
          className="min-w-0 flex-1 rounded border border-white/10 bg-havoc-panel px-1.5 py-0.5 text-[11px] text-havoc-text"
        >
          <option value="">{t("sources-devices-system-default")}</option>
          {devices.outputs.map((device) => (
            <option key={device.deviceId} value={device.deviceId}>
              {device.label}
            </option>
          ))}
        </select>
      </label>
      <div className="flex items-center gap-2">
        <button
          type="button"
          onClick={() => (testing ? stopTest() : startTest(micId, speakerId))}
          aria-pressed={testing}
          className={`shrink-0 rounded-md border px-3 py-1 text-[11px] font-semibold ${
            testing
              ? "border-emerald-400/60 bg-emerald-500/20 text-emerald-300"
              : "border-white/10 text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
          }`}
        >
          {testing ? t("sources-devices-stop-test") : t("sources-devices-test")}
        </button>
        <span className="text-[10px] leading-snug text-havoc-muted">
          {testing ? t("sources-devices-testing-note") : t("sources-devices-idle-note")}
        </span>
      </div>
      {testNote && <p className="m-0 text-[10px] leading-snug text-amber-300">{testNote}</p>}
    </div>
  );
}

/**
 * TASK-R5: the opt-in TURN relay — the user's OWN relay (e.g. Oracle Cloud
 * Always Free coturn), never author-run infrastructure. Direct P2P (STUN) is
 * the free default and most sessions never need this; it exists for the
 * both-sides-behind-strict-NAT case. The credential is a secret: stored in
 * the local settings file only, never logged, redacted from diagnostics.
 */
function TurnRelaySection() {
  const t = useT();
  const [settings, setSettings] = useState<Settings | null>(null);
  const [note, setNote] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    settingsGet()
      .then((loaded) => {
        if (!cancelled) setSettings(loaded);
      })
      .catch(() => {});
    return () => {
      cancelled = true;
    };
  }, []);

  const save = (patch: Partial<Settings["remote"]>) => {
    if (!settings) return;
    const next = { ...settings, remote: { ...settings.remote, ...patch } };
    setSettings(next);
    setNote(null);
    settingsSet(next).catch((err) =>
      setNote(t("sources-turn-save-failed", { error: String(err) })),
    );
  };

  return (
    <details className="border-t border-white/5 pt-2">
      <summary className="cursor-pointer text-[11px] font-semibold uppercase tracking-wide text-havoc-muted">
        {t("sources-turn-summary")}
      </summary>
      <div className="mt-1.5 flex flex-col gap-1.5">
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("sources-turn-note-1")}</p>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("sources-turn-note-2")}</p>
        {settings ? (
          <>
            <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
              <span className="w-20 shrink-0">{t("sources-turn-url-label")}</span>
              <input
                value={settings.remote.turnUrl}
                onChange={(event) => save({ turnUrl: event.target.value })}
                placeholder={t("sources-turn-url-placeholder")}
                aria-label={t("sources-turn-url-aria")}
                className="min-w-0 flex-1 rounded border border-white/10 bg-black/30 px-2 py-1 font-mono text-[10px] text-havoc-text"
              />
            </label>
            <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
              <span className="w-20 shrink-0">{t("sources-turn-username-label")}</span>
              <input
                value={settings.remote.turnUsername}
                onChange={(event) => save({ turnUsername: event.target.value })}
                aria-label={t("sources-turn-username-aria")}
                className="min-w-0 flex-1 rounded border border-white/10 bg-black/30 px-2 py-1 font-mono text-[10px] text-havoc-text"
              />
            </label>
            <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
              <span className="w-20 shrink-0">{t("sources-turn-credential-label")}</span>
              <input
                type="password"
                value={settings.remote.turnCredential}
                onChange={(event) => save({ turnCredential: event.target.value })}
                aria-label={t("sources-turn-credential-aria")}
                className="min-w-0 flex-1 rounded border border-white/10 bg-black/30 px-2 py-1 font-mono text-[10px] text-havoc-text"
              />
            </label>
            <p className="m-0 text-[10px] leading-snug text-havoc-muted">
              {t("sources-turn-note-3")}
            </p>
          </>
        ) : (
          <p className="m-0 text-[10px] text-havoc-muted">
            {t("sources-turn-settings-unavailable")}
          </p>
        )}
        {note && <p className="m-0 text-[10px] text-amber-300">{note}</p>}
      </div>
    </details>
  );
}

function ColorForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [hex, setHex] = useState("#4a9eff");
  const [width, setWidth] = useState(1920);
  const [height, setHeight] = useState(1080);
  return (
    <PickerShell title={t("sources-color-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          {t("sources-color-label")}
          <input
            type="color"
            value={hex}
            onChange={(event) => setHex(event.target.value)}
            aria-label={t("sources-color-label")}
            className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
          />
        </label>
        <div className="flex gap-2">
          <NumberField
            label={t("sources-color-width-label")}
            value={width}
            min={1}
            max={16384}
            onCommit={setWidth}
            className="flex-1"
          />
          <NumberField
            label={t("sources-color-height-label")}
            value={height}
            min={1}
            max={16384}
            onCommit={setHeight}
            className="flex-1"
          />
        </div>
        <button
          type="button"
          onClick={() => onPick({ kind: "color", color: hexToRgba(hex), width, height })}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
        >
          {t("sources-color-add")}
        </button>
      </div>
    </PickerShell>
  );
}

/** The five CAP-M15 timer faces, in menu order. Values are i18n keys. */
const TIMER_MODES: Array<[TimerMode, string]> = [
  ["wallClock", "sources-timer-wall-clock"],
  ["countdown", "sources-timer-countdown"],
  ["stopwatch", "sources-timer-stopwatch"],
  ["sinceLive", "sources-timer-since-live"],
  ["sinceRecording", "sources-timer-since-recording"],
];

function TimerForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [mode, setMode] = useState<TimerMode>("wallClock");
  return (
    <PickerShell title={t("sources-timer-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-timer-mode-label")}
          <select
            value={mode}
            onChange={(event) => setMode(event.target.value as TimerMode)}
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text"
          >
            {TIMER_MODES.map(([value, label]) => (
              <option key={value} value={value}>
                {t(label)}
              </option>
            ))}
          </select>
        </label>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("sources-timer-note")}</p>
        <button
          type="button"
          onClick={() =>
            onPick({
              kind: "timer",
              mode,
              format: "",
              utcOffsetMin: null,
              countdownMs: 5 * 60 * 1000,
              target: "",
              endAction: "none",
              endScene: null,
              fontFamily: null,
              fontFile: null,
              sizePx: 96,
              color: { r: 255, g: 255, b: 255, a: 255 },
              message: "",
              slate: null,
            })
          }
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
        >
          {t("sources-timer-add")}
        </button>
      </div>
    </PickerShell>
  );
}

/** Add looping background music: an invisible (no on-canvas card),
 * optionally shuffled audio Playlist, auto-set to Monitor and Output so it
 * is heard in preview AND captured to the recording/stream. `muteSource`
 * (the scene's video backdrop, when present) is muted so only one
 * background sound ever plays. Failure logs, never throws. */
function addBackgroundMusic(
  sceneId: SceneId,
  paths: string[],
  name: string,
  options?: { shuffle?: boolean; muteSource?: SourceId | null },
): Promise<void> {
  return studioAddItem(
    sceneId,
    {
      kind: "playlist",
      items: paths.map((path) => ({ path, in: 0, out: 0, cues: [] })),
      loop: true,
      shuffle: options?.shuffle ?? false,
      holdLast: false,
      hwDecode: true,
      nowPlayingVariable: "",
      hiddenFace: true,
    },
    name,
  )
    .then((added) => studioSetAudioMonitor(added.sourceId, "monitorAndOutput"))
    .then(() => (options?.muteSource ? studioSetAudioMuted(options.muteSource, true) : undefined))
    .catch((err: unknown) => console.error("add background music failed:", err));
}

/** V1-C: a one-click "Starting Soon" pre-show slate — a full-canvas countdown
 * (a message above a big number) that flashes red and holds at zero, so the
 * host clears it and cuts to live by hand. It is a countdown Timer in slate
 * mode; every field stays editable afterwards in Properties. */
function StartingSoonForm({
  onClose,
  onPick,
  sceneId,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
  /** The active scene — a GIF/video background is set as its looping Backdrop. */
  sceneId: SceneId | null;
}) {
  const t = useT();
  const [message, setMessage] = useState(t("sources-starting-soon-default"));
  // Count down TO a wall-clock time (e.g. 8:00 PM) — the Timer's `target`,
  // edited on the shared 12-hour ClockSelect (default noon).
  const [target, setTarget] = useState("12:00");
  const [background, setBackground] = useState<"gradient" | "solid" | "file" | "transparent">(
    "gradient",
  );
  const [filePath, setFilePath] = useState("");
  const [musicPath, setMusicPath] = useState("");

  // A GIF/video can't be painted into the static slate face — it rides the
  // scene's (looping) Backdrop instead, with the slate left transparent.
  const isAnimated = (path: string) => /\.(gif|mp4|mkv|webm|mov|frec)$/i.test(path);

  const chooseMusic = async () => {
    try {
      const picked = await open({
        multiple: false,
        directory: false,
        filters: [
          {
            name: t("sources-starting-soon-music"),
            extensions: AUDIO_EXTS,
          },
        ],
      });
      if (typeof picked === "string") setMusicPath(picked);
    } catch (err) {
      console.error("music pick failed:", err);
    }
  };

  const chooseFile = async () => {
    try {
      const picked = await open({
        multiple: false,
        directory: false,
        filters: [
          {
            name: t("backdrop-filter-all"),
            extensions: [
              "png",
              "jpg",
              "jpeg",
              "bmp",
              "webp",
              "tif",
              "tiff",
              "gif",
              "mp4",
              "mkv",
              "webm",
              "mov",
              "frec",
            ],
          },
        ],
      });
      if (typeof picked === "string") setFilePath(picked);
    } catch (err) {
      // Cancelled, or the file went away between the dialog and here — no throw.
      console.error("background file pick failed:", err);
    }
  };

  const slateFor = (): CountdownSlate => {
    if (background === "solid") return { kind: "solid", color: SLATE_SOLID };
    if (background === "transparent") return { kind: "transparent" };
    if (background === "file")
      // A still image embeds; a GIF/video shows through a transparent slate (it
      // rides the scene Backdrop set on Add).
      return filePath && !isAnimated(filePath)
        ? { kind: "image", path: filePath }
        : { kind: "transparent" };
    return { kind: "gradient", from: SLATE_GRADIENT_FROM, to: SLATE_GRADIENT_TO };
  };

  const add = () => {
    const settings: SourceSettings = {
      kind: "timer",
      mode: "countdown",
      format: "",
      utcOffsetMin: null,
      countdownMs: 5 * 60 * 1000,
      target,
      endAction: "flash",
      endScene: null,
      fontFamily: null,
      fontFile: null,
      sizePx: 200,
      color: { r: 255, g: 255, b: 255, a: 255 },
      message,
      slate: slateFor(),
    };
    // Optional background music through the shared helper (invisible looping
    // playlist, monitored). The backdrop added below starts muted server-side,
    // so no extra mute is needed here.
    if (musicPath.trim() && sceneId) {
      void addBackgroundMusic(sceneId, [musicPath.trim()], t("sources-starting-soon-music-name"));
    }
    // A GIF/video background rides the scene Backdrop, contain-fit ("fit") so the
    // WHOLE thing shows, centred. A bad path just skips the backdrop — no crash.
    if (background === "file" && filePath && isAnimated(filePath) && sceneId) {
      studioSetSceneBackdrop(sceneId, filePath)
        .then(() => studioSetBackdropSplit(sceneId, "fit"))
        .catch((err: unknown) => console.error("set backdrop failed:", err))
        .finally(() => onPick(settings, t("sources-starting-soon-title")));
    } else {
      onPick(settings, t("sources-starting-soon-title"));
    }
  };

  return (
    <PickerShell title={t("sources-starting-soon-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-starting-soon-message")}
          <input
            value={message}
            onChange={(event) => setMessage(event.target.value)}
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text"
          />
        </label>
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-starting-soon-start-at")}
          <ClockSelect
            value={target}
            onChange={setTarget}
            selectClass="flex-1 rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text"
          />
        </label>
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-starting-soon-background")}
          <select
            value={background}
            onChange={(event) => setBackground(event.target.value as typeof background)}
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text"
          >
            <option value="gradient">{t("sources-slate-gradient")}</option>
            <option value="solid">{t("sources-slate-solid")}</option>
            <option value="file">{t("sources-slate-file")}</option>
            <option value="transparent">{t("sources-slate-transparent")}</option>
          </select>
        </label>
        {background === "file" && (
          <div className="flex items-center gap-2">
            <button
              type="button"
              onClick={() => void chooseFile()}
              className="shrink-0 rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2.5 py-1 text-[11px] text-havoc-text hover:bg-havoc-accent/25"
            >
              {t("sources-slate-browse")}
            </button>
            <span className="min-w-0 flex-1 truncate text-[10px] text-havoc-muted" title={filePath}>
              {filePath || t("backdrop-none")}
            </span>
          </div>
        )}
        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={() => void chooseMusic()}
            className="shrink-0 rounded-md border border-white/10 px-2.5 py-1 text-[11px] text-havoc-muted hover:text-havoc-text"
          >
            {t("sources-starting-soon-music")}
          </button>
          <span className="min-w-0 flex-1 truncate text-[10px] text-havoc-muted" title={musicPath}>
            {musicPath || t("backdrop-none")}
          </span>
          {musicPath && (
            <button
              type="button"
              onClick={() => setMusicPath("")}
              aria-label={t("backdrop-remove")}
              className="shrink-0 rounded px-1 text-[11px] text-havoc-muted hover:text-red-400"
            >
              ×
            </button>
          )}
        </div>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("sources-starting-soon-note")}
        </p>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("sources-slate-media-note")}
        </p>
        <button
          type="button"
          onClick={add}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
        >
          {t("sources-starting-soon-add")}
        </button>
      </div>
    </PickerShell>
  );
}

/** A findable "Background Music" source: a looping audio file auto-set to
 * Monitor and Output (heard in preview AND captured to the recording/stream).
 * Works in any scene — countdown or not — so music isn't stuck in the Starting
 * Soon add flow. */
function BackgroundMusicForm({ onClose, scene }: { onClose: () => void; scene: Scene | null }) {
  const t = useT();
  const [paths, setPaths] = useState<string[]>([]);
  const [shuffle, setShuffle] = useState(false);
  const choose = async () => {
    try {
      const picked = await open({
        multiple: true,
        directory: false,
        filters: [
          {
            name: t("sources-starting-soon-music"),
            extensions: AUDIO_EXTS,
          },
        ],
      });
      const next = Array.isArray(picked) ? picked : typeof picked === "string" ? [picked] : [];
      if (next.length) setPaths((prev) => [...prev, ...next]);
    } catch (err) {
      console.error("music pick failed:", err);
    }
  };
  const add = () => {
    if (paths.length === 0 || !scene) {
      onClose();
      return;
    }
    // Only one background audio at a time: mute the video backdrop (if any) so
    // the music plays, not both stacked.
    const backdropSource = scene.items.find((item) => item.backdrop)?.source ?? null;
    void addBackgroundMusic(scene.id, paths, t("sources-starting-soon-music-name"), {
      shuffle,
      muteSource: backdropSource,
    }).finally(onClose);
  };
  return (
    <PickerShell title={t("sources-add-background-music")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={() => void choose()}
            className="shrink-0 rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2.5 py-1 text-[11px] text-havoc-text hover:bg-havoc-accent/25"
          >
            {t("sources-slate-browse")}
          </button>
          <span className="min-w-0 flex-1 truncate text-[10px] text-havoc-muted">
            {paths.length
              ? t("sources-background-music-count", { n: paths.length })
              : t("backdrop-none")}
          </span>
          {paths.length > 0 && (
            <button
              type="button"
              onClick={() => setPaths([])}
              aria-label={t("backdrop-remove")}
              className="shrink-0 rounded px-1 text-[11px] text-havoc-muted hover:text-red-400"
            >
              ×
            </button>
          )}
        </div>
        {paths.length > 0 && (
          <ul className="m-0 max-h-24 list-none overflow-y-auto rounded border border-white/10 p-1">
            {paths.map((p, index) => (
              <li
                key={`${p}-${index}`}
                className="truncate px-1 py-0.5 text-[10px] text-havoc-muted"
                title={p}
              >
                {p.split(/[\\/]/).pop()}
              </li>
            ))}
          </ul>
        )}
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={shuffle}
            onChange={(event) => setShuffle(event.target.checked)}
          />
          {t("sources-background-music-shuffle")}
        </label>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("sources-background-music-note")}
        </p>
        <button
          type="button"
          disabled={paths.length === 0}
          onClick={add}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          {t("sources-starting-soon-add")}
        </button>
      </div>
    </PickerShell>
  );
}

function SystemStatsForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  return (
    <PickerShell title={t("sources-stats-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("sources-stats-note")}</p>
        <button
          type="button"
          onClick={() =>
            onPick({
              kind: "systemStats",
              showFps: true,
              showCpu: true,
              showMemory: true,
              showRenderMs: true,
              showDropped: true,
              showBitrate: true,
              showTimecode: false,
              fontFamily: null,
              fontFile: null,
              sizePx: 28,
              color: { r: 255, g: 255, b: 255, a: 255 },
            })
          }
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
        >
          {t("sources-stats-add")}
        </button>
      </div>
    </PickerShell>
  );
}

function VisualizerForm({
  sources,
  onClose,
  onPick,
}: {
  sources: Array<{ id: string; name: string }>;
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [style, setStyle] = useState<VisStyle>("bars");
  const [target, setTarget] = useState("master");
  const [classic, setClassic] = useState(false);
  return (
    <PickerShell title={t("sources-visualizer-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-visualizer-style-label")}
          <select
            value={style}
            onChange={(event) => setStyle(event.target.value as VisStyle)}
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text"
          >
            {VIS_STYLES.map(([value, label]) => (
              <option key={value} value={value}>
                {t(label)}
              </option>
            ))}
          </select>
        </label>
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-visualizer-target-label")}
          <select
            value={target}
            onChange={(event) => setTarget(event.target.value)}
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text"
          >
            <option value="master">{t("sources-visualizer-target-master")}</option>
            {[1, 2, 3, 4, 5, 6].map((n) => (
              <option key={n} value={`track:${n}`}>
                {t("sources-visualizer-target-track", { n })}
              </option>
            ))}
            {sources.map((source) => (
              <option key={source.id} value={`source:${source.id}`}>
                {source.name}
              </option>
            ))}
          </select>
        </label>
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={classic}
            onChange={(event) => setClassic(event.target.checked)}
          />
          {t("sources-visualizer-classic")}
        </label>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("sources-visualizer-note")}
        </p>
        <button
          type="button"
          onClick={() => {
            const parsed = parseVisTarget(target);
            onPick({
              kind: "audioVisualizer",
              style,
              target: parsed.target,
              track: parsed.track ?? 1,
              source: parsed.source,
              width: 800,
              height: 240,
              bands: 48,
              color: { r: 74, g: 158, b: 255, a: 255 },
              peakHold: true,
              decay: 30,
              classic,
            });
          }}
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
        >
          {t("sources-visualizer-add")}
        </button>
      </div>
    </PickerShell>
  );
}

function InputOverlayForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [layout, setLayout] = useState<InputLayout>("wasd");
  const [colorHex, setColorHex] = useState("#ffffff");
  const [accentHex, setAccentHex] = useState("#4a9eff");
  return (
    <PickerShell title={t("sources-input-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-input-layout-label")}
          <select
            value={layout}
            onChange={(event) => setLayout(event.target.value as InputLayout)}
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text"
          >
            {INPUT_LAYOUTS.map(([value, label]) => (
              <option key={value} value={value}>
                {t(label)}
              </option>
            ))}
          </select>
        </label>
        <div className="flex items-end gap-3">
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            {t("sources-input-color-label")}
            <input
              type="color"
              value={colorHex}
              onChange={(event) => setColorHex(event.target.value)}
              aria-label={t("sources-input-color-label")}
              className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
            />
          </label>
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            {t("sources-input-accent-label")}
            <input
              type="color"
              value={accentHex}
              onChange={(event) => setAccentHex(event.target.value)}
              aria-label={t("sources-input-accent-label")}
              className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
            />
          </label>
        </div>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("sources-input-privacy-note")}
        </p>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("sources-input-os-note")}
        </p>
        <button
          type="button"
          onClick={() =>
            onPick({
              kind: "inputOverlay",
              layout,
              color: hexToRgba(colorHex),
              accent: hexToRgba(accentHex),
            })
          }
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
        >
          {t("sources-input-add")}
        </button>
      </div>
    </PickerShell>
  );
}

const TEST_PATTERNS: Array<
  ["testBars" | "testGrid" | "testSweep" | "testTone" | "testFlashBeep", string]
> = [
  ["testBars", "sources-testsignal-bars"],
  ["testGrid", "sources-testsignal-grid"],
  ["testSweep", "sources-testsignal-sweep"],
  ["testTone", "sources-testsignal-tone"],
  ["testFlashBeep", "sources-testsignal-flash-beep"],
];

function TestSignalForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [pattern, setPattern] = useState<(typeof TEST_PATTERNS)[number][0]>("testBars");
  const [width, setWidth] = useState(1920);
  const [height, setHeight] = useState(1080);
  return (
    <PickerShell title={t("sources-testsignal-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-testsignal-pattern-label")}
          <select
            value={pattern}
            onChange={(event) => setPattern(event.target.value as typeof pattern)}
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text"
          >
            {TEST_PATTERNS.map(([kind, label]) => (
              <option key={kind} value={kind}>
                {t(label)}
              </option>
            ))}
          </select>
        </label>
        {pattern !== "testTone" && (
          <div className="flex gap-2">
            <NumberField
              label={t("sources-color-width-label")}
              value={width}
              min={1}
              max={16384}
              onCommit={setWidth}
              className="flex-1"
            />
            <NumberField
              label={t("sources-color-height-label")}
              value={height}
              min={1}
              max={16384}
              onCommit={setHeight}
              className="flex-1"
            />
          </div>
        )}
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">
          {t("sources-testsignal-note")}
        </p>
        <button
          type="button"
          onClick={() =>
            onPick(pattern === "testTone" ? { kind: "testTone" } : { kind: pattern, width, height })
          }
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
        >
          {t("sources-testsignal-add")}
        </button>
      </div>
    </PickerShell>
  );
}

function TextForm({
  onClose,
  onPick,
}: {
  onClose: () => void;
  onPick: (settings: SourceSettings, name?: string) => void;
}) {
  const t = useT();
  const [text, setText] = useState(t("sources-text-default"));
  const [hex, setHex] = useState("#ffffff");
  const [size, setSize] = useState(72);
  return (
    <PickerShell title={t("sources-text-title")} onClose={onClose}>
      <div className="flex flex-col gap-2">
        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("sources-text-label")}
          <textarea
            value={text}
            onChange={(event) => setText(event.target.value)}
            rows={3}
            className={inputClass}
          />
        </label>
        <div className="flex items-end gap-2">
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            {t("sources-text-color-label")}
            <input
              type="color"
              value={hex}
              onChange={(event) => setHex(event.target.value)}
              aria-label={t("sources-text-color-aria")}
              className="h-7 w-12 cursor-pointer rounded border border-white/10 bg-transparent"
            />
          </label>
          <NumberField
            label={t("sources-text-size-label")}
            value={size}
            min={4}
            max={512}
            onCommit={setSize}
            className="flex-1"
          />
        </div>
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("sources-text-note")}</p>
        <button
          type="button"
          disabled={!text.trim()}
          onClick={() =>
            onPick(
              {
                kind: "text",
                text,
                fontFamily: null,
                fontFile: null,
                sizePx: size,
                color: hexToRgba(hex),
                align: "left",
                lineSpacing: 1.0,
                forceRtl: false,
                wrapWidth: null,
                sourceFile: "",
                binding: "whole",
                csvRow: 1,
                csvColumn: "",
                jsonPointer: "",
              },
              text.length > 24 ? `${text.slice(0, 24)}…` : text,
            )
          }
          className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          {t("sources-text-add")}
        </button>
      </div>
    </PickerShell>
  );
}

function ExistingPicker({
  collection,
  onClose,
  onPick,
}: {
  collection: Collection | null;
  onClose: () => void;
  onPick: (source: SourceId) => void;
}) {
  const t = useT();
  const sources = collection?.sources ?? [];
  return (
    <PickerShell title={t("sources-existing-title")} onClose={onClose}>
      {sources.length === 0 ? (
        <p className="m-0 text-xs text-havoc-muted">{t("sources-existing-empty")}</p>
      ) : (
        <ul className="m-0 flex list-none flex-col gap-1 p-0">
          {sources.map((source) => (
            <li key={source.id}>
              <button
                type="button"
                onClick={() => onPick(source.id)}
                className="flex w-full items-center gap-1.5 truncate rounded-md border border-white/10 px-2 py-1.5 text-left text-xs text-havoc-text hover:border-havoc-accent/50"
              >
                <span className="rounded bg-white/10 px-1 py-px text-[9px] text-havoc-muted uppercase">
                  {t(KIND_BADGE[source.kind] ?? "sources-kind-unknown")}
                </span>
                <span className="truncate">{source.name}</span>
              </button>
            </li>
          ))}
        </ul>
      )}
    </PickerShell>
  );
}

/** A source's assigned slot in the screen-plus-corners layout. */
type LayoutChoice = "off" | "center" | Corner;

/** `[value, i18n key]` — the call site renders `t(key)`. */
const SLOT_OPTIONS: Array<[LayoutChoice, string]> = [
  ["off", "sources-slot-off"],
  ["center", "sources-slot-center"],
  ["topLeft", "sources-slot-top-left"],
  ["topRight", "sources-slot-top-right"],
  ["bottomLeft", "sources-slot-bottom-left"],
  ["bottomRight", "sources-slot-bottom-right"],
];

/**
 * Arrange the scene as a centered screen with up to four corner cameras — the
 * explainer / podcast layout. Screen-kind sources auto-seat to the center,
 * cameras fill the corners; the user can reassign any of them (and drag on the
 * canvas afterward). Audio-only sources are skipped — they don't compose.
 */
function LayoutPicker({
  collection,
  scene,
  onClose,
}: {
  collection: Collection | null;
  scene: Scene | null;
  onClose: () => void;
}) {
  const t = useT();
  const sourceOf = (id: SourceId) => collection?.sources.find((source) => source.id === id);
  const visual = (scene?.items ?? []).filter((item) => {
    const kind = sourceOf(item.source)?.kind;
    return kind !== "audioInput" && kind !== "audioOutput" && kind !== "appAudio";
  });

  const [choice, setChoice] = useState<Record<string, LayoutChoice>>(() => {
    const map: Record<string, LayoutChoice> = {};
    let centerTaken = false;
    let cornerIdx = 0;
    for (const item of visual) {
      const kind = sourceOf(item.source)?.kind;
      if (!centerTaken && (kind === "display" || kind === "window" || kind === "portal")) {
        map[item.id] = "center";
        centerTaken = true;
      } else if ((kind === "videoDevice" || kind === "media") && cornerIdx < CORNERS.length) {
        map[item.id] = CORNERS[cornerIdx];
        cornerIdx += 1;
      } else {
        map[item.id] = "off";
      }
    }
    return map;
  });

  const apply = () => {
    if (!scene) return;
    // Dedupe by slot — the first source assigned to a slot wins it.
    let center: ItemId | null = null;
    const taken = new Set<Corner>();
    const corners: CornerSlot[] = [];
    for (const item of visual) {
      const slot = choice[item.id] ?? "off";
      if (slot === "off") continue;
      if (slot === "center") {
        center ??= item.id;
      } else if (!taken.has(slot)) {
        taken.add(slot);
        corners.push({ itemId: item.id, corner: slot });
      }
    }
    studioApplyLayout(scene.id, center, corners).catch((err) =>
      console.error("apply layout failed:", err),
    );
    onClose();
  };

  return (
    <PickerShell title={t("sources-layout-title")} onClose={onClose}>
      {visual.length === 0 ? (
        <p className="m-0 text-xs text-havoc-muted">{t("sources-layout-empty")}</p>
      ) : (
        <div className="flex flex-col gap-2">
          <p className="m-0 text-[11px] leading-relaxed text-havoc-muted">
            {t("sources-layout-note")}
          </p>
          <ul className="m-0 flex list-none flex-col gap-1 p-0">
            {visual.map((item) => {
              const source = sourceOf(item.source);
              return (
                <li key={item.id} className="flex items-center gap-2">
                  <span className="rounded bg-white/10 px-1 py-px text-[9px] text-havoc-muted uppercase">
                    {t(KIND_BADGE[source?.kind ?? ""] ?? "sources-kind-unknown")}
                  </span>
                  <span className="min-w-0 flex-1 truncate text-xs text-havoc-text">
                    {source?.name ?? t("sources-missing-source")}
                  </span>
                  <select
                    value={choice[item.id] ?? "off"}
                    onChange={(event) =>
                      setChoice((prev) => ({
                        ...prev,
                        [item.id]: event.target.value as LayoutChoice,
                      }))
                    }
                    aria-label={t("sources-layout-slot-aria", {
                      name: source?.name ?? t("sources-fallback-name"),
                    })}
                    className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text"
                  >
                    {SLOT_OPTIONS.map(([value, label]) => (
                      <option key={value} value={value}>
                        {t(label)}
                      </option>
                    ))}
                  </select>
                </li>
              );
            })}
          </ul>
          <button
            type="button"
            onClick={apply}
            className="self-end rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            {t("sources-layout-apply")}
          </button>
        </div>
      )}
    </PickerShell>
  );
}
