import { useEffect, useState } from "react";

import {
  quickActionDispatch,
  quickActionsGet,
  quickActionsSet,
  soundboardTrigger,
  type QuickAction,
  type QuickActions,
  type QuickActionSpec,
} from "../api/commands";
import { PickerShell } from "../components/PickerShell";
import { EmptyHint } from "../components/Panel";
import { useT } from "../i18n/t";

/** Allowlisted no-argument commands offered directly in the button builder. */
const COMMAND_PRESETS = [
  "transition",
  "startStream",
  "stopStream",
  "startRecording",
  "stopRecording",
  "pauseRecording",
  "addMarker",
  "armReplay",
  "saveReplay",
  "setStudioMode",
] as const;

type BuildKind = (typeof COMMAND_PRESETS)[number] | "scene" | "macro" | "soundboard";

/** Turn the builder's choice into a stored action spec. */
function specFor(kind: BuildKind, value: string): QuickActionSpec {
  switch (kind) {
    case "scene":
      return { kind: "command", command: "setProgramScene", params: { scene: value } };
    case "macro":
      return { kind: "command", command: "runMacro", params: { name: value } };
    case "soundboard":
      return { kind: "soundboard", pad: value };
    default:
      return { kind: "command", command: kind, params: {} };
  }
}

/**
 * CAP-N68: the quick-actions grid — a Stream-Deck-style button board whose
 * buttons fire the same allowlisted studio commands as hotkeys / the LAN panel,
 * or trigger soundboard pads. The config is shared with the CAP-N06 LAN panel
 * (same `quick-actions.json`). Strictly local.
 */
export function QuickActionsDialog({ onClose }: { onClose: () => void }) {
  const t = useT();
  const [config, setConfig] = useState<QuickActions | null>(null);
  const [page, setPage] = useState(0);
  const [editing, setEditing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [label, setLabel] = useState("");
  const [color, setColor] = useState("#4a9eff");
  const [buildKind, setBuildKind] = useState<BuildKind>("transition");
  const [buildValue, setBuildValue] = useState("");

  useEffect(() => {
    quickActionsGet()
      .then((c) => setConfig(c.pages.length ? c : { pages: [{ name: "Page 1", buttons: [] }] }))
      .catch((err) => setError(String(err)));
  }, []);

  const persist = (next: QuickActions) => {
    const prev = config;
    setConfig(next);
    quickActionsSet(next).catch((err) => {
      setError(String(err));
      // The backend rejected the save (e.g. over the button/page cap) — roll the
      // UI back so it never shows an unsaved grid as if it persisted.
      setConfig(prev);
    });
  };

  const run = (action: QuickActionSpec) => {
    setError(null);
    (action.kind === "soundboard"
      ? soundboardTrigger(action.pad)
      : quickActionDispatch(action.command, action.params ?? {})
    ).catch((err) => setError(String(err)));
  };

  const pages = config?.pages ?? [];
  const current = pages[page];
  const needsValue = buildKind === "scene" || buildKind === "macro" || buildKind === "soundboard";

  const addButton = () => {
    if (!current || !label.trim() || (needsValue && !buildValue.trim())) return;
    const button: QuickAction = {
      label: label.trim(),
      color,
      action: specFor(buildKind, buildValue.trim()),
    };
    persist({
      pages: pages.map((p, i) => (i === page ? { ...p, buttons: [...p.buttons, button] } : p)),
    });
    setLabel("");
    setBuildValue("");
  };

  const removeButton = (index: number) =>
    persist({
      pages: pages.map((p, i) =>
        i === page ? { ...p, buttons: p.buttons.filter((_, j) => j !== index) } : p,
      ),
    });

  const addPage = () => {
    const next = { pages: [...pages, { name: `Page ${pages.length + 1}`, buttons: [] }] };
    persist(next);
    setPage(next.pages.length - 1);
  };

  const deletePage = () => {
    if (pages.length <= 1) return;
    persist({ pages: pages.filter((_, i) => i !== page) });
    setPage(0);
  };

  return (
    <PickerShell title={t("quick-actions-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <div className="flex flex-wrap items-center gap-1.5">
          {pages.map((p, i) => (
            <button
              key={i}
              type="button"
              onClick={() => setPage(i)}
              className={`rounded-md border px-2 py-1 text-[11px] ${
                i === page
                  ? "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text"
                  : "border-white/10 text-havoc-muted hover:text-havoc-text"
              }`}
            >
              {p.name}
            </button>
          ))}
          {editing && (
            <button
              type="button"
              onClick={addPage}
              className="rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:text-havoc-text"
            >
              + {t("quick-actions-add-page")}
            </button>
          )}
          <button
            type="button"
            onClick={() => setEditing((e) => !e)}
            className={`ml-auto rounded-md border px-2 py-1 text-[11px] ${
              editing
                ? "border-havoc-accent/60 text-havoc-accent"
                : "border-white/10 text-havoc-muted hover:text-havoc-text"
            }`}
          >
            {editing ? t("quick-actions-done") : t("quick-actions-edit")}
          </button>
        </div>

        {current && current.buttons.length === 0 ? (
          <EmptyHint>{t("quick-actions-empty")}</EmptyHint>
        ) : (
          <div className="grid grid-cols-4 gap-2">
            {current?.buttons.map((button, index) => (
              <div key={index} className="relative">
                <button
                  type="button"
                  onClick={() => run(button.action)}
                  style={button.color ? { borderColor: button.color } : undefined}
                  className="flex h-16 w-full items-center justify-center rounded-md border-2 bg-white/[0.03] px-1 text-center text-[11px] text-havoc-text hover:bg-white/5"
                >
                  {button.label}
                </button>
                {editing && (
                  <button
                    type="button"
                    onClick={() => removeButton(index)}
                    aria-label={t("quick-actions-remove", { name: button.label })}
                    className="absolute -top-1.5 -right-1.5 rounded-full bg-havoc-panel px-1 text-xs text-havoc-muted hover:text-red-400"
                  >
                    ×
                  </button>
                )}
              </div>
            ))}
          </div>
        )}

        {editing && (
          <div className="flex flex-col gap-2 rounded-md border border-white/10 bg-white/[0.02] p-3">
            <div className="flex flex-wrap items-center gap-2">
              <input
                value={label}
                onChange={(e) => setLabel(e.target.value)}
                placeholder={t("quick-actions-label")}
                className="min-w-0 flex-1 rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs outline-none focus:border-havoc-accent/60"
              />
              <input
                type="color"
                value={color}
                onChange={(e) => setColor(e.target.value)}
                aria-label={t("quick-actions-color")}
                className="h-7 w-9 shrink-0 cursor-pointer rounded border border-white/10 bg-transparent p-0"
              />
            </div>
            <div className="flex flex-wrap items-center gap-2">
              <select
                value={buildKind}
                onChange={(e) => setBuildKind(e.target.value as BuildKind)}
                className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text outline-none focus:border-havoc-accent/60"
              >
                {COMMAND_PRESETS.map((c) => (
                  <option key={c} value={c}>
                    {t(`quick-actions-cmd-${c}`)}
                  </option>
                ))}
                <option value="scene">{t("quick-actions-cmd-scene")}</option>
                <option value="macro">{t("quick-actions-cmd-macro")}</option>
                <option value="soundboard">{t("quick-actions-cmd-soundboard")}</option>
              </select>
              {needsValue && (
                <input
                  value={buildValue}
                  onChange={(e) => setBuildValue(e.target.value)}
                  placeholder={t(`quick-actions-value-${buildKind}`)}
                  className="min-w-0 flex-1 rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs outline-none focus:border-havoc-accent/60"
                />
              )}
              <button
                type="button"
                disabled={!label.trim() || (needsValue && !buildValue.trim())}
                onClick={addButton}
                className="shrink-0 rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2.5 py-1 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
              >
                {t("quick-actions-add")}
              </button>
            </div>
            {pages.length > 1 && (
              <button
                type="button"
                onClick={deletePage}
                className="self-start rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:text-red-400"
              >
                {t("quick-actions-delete-page", { name: current?.name ?? "" })}
              </button>
            )}
            <p className="m-0 text-[10px] leading-snug text-havoc-muted">
              {t("quick-actions-hint")}
            </p>
          </div>
        )}

        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}
      </div>
    </PickerShell>
  );
}
