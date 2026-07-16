import { useEffect, useMemo, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

import { settingsSet, soundboardStopAll, soundboardTrigger } from "../api/commands";
import type { Settings, SoundboardPad } from "../api/types";
import { PickerShell } from "./PickerShell";
import { useT } from "../i18n/t";

const fail = (what: string) => (err: unknown) => console.error(`${what} failed:`, err);

/** Does a keydown match a pad accelerator like "Ctrl+Shift+1" / "F13"? */
function matchesAccelerator(event: KeyboardEvent, accel: string): boolean {
  const parts = accel.split("+").map((p) => p.trim().toLowerCase());
  const key = parts[parts.length - 1];
  const wantCtrl =
    parts.includes("ctrl") || parts.includes("control") || parts.includes("commandorcontrol");
  const wantShift = parts.includes("shift");
  const wantAlt = parts.includes("alt");
  const wantMeta = parts.includes("super") || parts.includes("meta") || parts.includes("command");
  if (event.ctrlKey !== (wantCtrl || false)) return false;
  if (event.shiftKey !== wantShift) return false;
  if (event.altKey !== wantAlt) return false;
  if (event.metaKey !== wantMeta) return false;
  const evKey = event.key.toLowerCase();
  const evCode = event.code.toLowerCase().replace("key", "").replace("digit", "");
  return key === evKey || key === evCode;
}

/**
 * CAP-N37 soundboard: a pad grid of local audio clips. Click a pad to fire it
 * (choke groups, loop, per-pad gain/tracks/auto-duck are engine-side). Edit mode
 * assigns clips, hotkeys, and mix options. Pad hotkeys fire while this is open.
 */
export function SoundboardDialog({
  settings,
  onSettingsSaved,
  onClose,
}: {
  settings: Settings;
  onSettingsSaved: (settings: Settings) => void;
  onClose: () => void;
}) {
  const t = useT();
  const [editing, setEditing] = useState(false);
  const [selected, setSelected] = useState<string | null>(null);

  const pads = useMemo(() => settings.soundboard?.pads ?? [], [settings.soundboard]);

  const savePads = (next: SoundboardPad[]) => {
    const nextSettings: Settings = { ...settings, soundboard: { pads: next } };
    settingsSet(nextSettings)
      .then(() => onSettingsSaved(nextSettings))
      .catch(fail("soundboard save"));
  };

  const setPad = (id: string, patch: Partial<SoundboardPad>) =>
    savePads(pads.map((pad) => (pad.id === id ? { ...pad, ...patch } : pad)));

  const addPad = () => {
    const id = crypto.randomUUID();
    savePads([
      ...pads,
      {
        id,
        name: t("soundboard-new-pad"),
        path: "",
        gainDb: 0,
        tracks: 1,
        chokeGroup: 0,
        looping: false,
        autoDuck: false,
      },
    ]);
    setSelected(id);
    setEditing(true);
  };

  const removePad = (id: string) => {
    savePads(pads.filter((pad) => pad.id !== id));
    if (selected === id) setSelected(null);
  };

  const pickClip = async (id: string) => {
    const picked = await open({
      multiple: false,
      filters: [
        {
          name: t("soundboard-audio-files"),
          extensions: ["wav", "mp3", "ogg", "flac", "m4a", "aac", "opus"],
        },
      ],
    });
    if (typeof picked === "string") {
      const name = picked.split(/[\\/]/).pop() ?? picked;
      setPad(id, { path: picked, name: name.replace(/\.[^.]+$/, "") });
    }
  };

  const play = (pad: SoundboardPad) => {
    if (!pad.path) return;
    soundboardTrigger(pad.id).catch(fail("soundboard trigger"));
  };

  // Pad hotkeys fire while the board is open (and no field is focused).
  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      const target = event.target as HTMLElement | null;
      if (target && ["INPUT", "SELECT", "TEXTAREA"].includes(target.tagName)) return;
      for (const pad of pads) {
        if (pad.hotkey && pad.path && matchesAccelerator(event, pad.hotkey)) {
          event.preventDefault();
          play(pad);
          return;
        }
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [pads]);

  const selectedPad = selected ? (pads.find((pad) => pad.id === selected) ?? null) : null;

  return (
    <PickerShell title={t("soundboard-title")} onClose={onClose} wide>
      <div className="flex max-h-[70vh] flex-col gap-3 overflow-y-auto">
        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={addPad}
            className="rounded border border-white/10 px-2 py-0.5 text-[11px] text-havoc-muted transition-colors hover:border-havoc-accent/50 hover:text-havoc-text"
          >
            {t("soundboard-add-pad")}
          </button>
          <button
            type="button"
            onClick={() => soundboardStopAll().catch(fail("soundboard stop-all"))}
            className="rounded border border-white/10 px-2 py-0.5 text-[11px] text-havoc-muted transition-colors hover:border-red-500/60 hover:text-red-400"
          >
            {t("soundboard-stop-all")}
          </button>
          <label className="ml-auto flex items-center gap-1 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={editing}
              onChange={(e) => setEditing(e.target.checked)}
            />
            {t("soundboard-edit")}
          </label>
        </div>

        {pads.length === 0 ? (
          <p className="text-xs text-havoc-muted">{t("soundboard-empty")}</p>
        ) : (
          <div className="grid grid-cols-4 gap-2">
            {pads.map((pad) => (
              <div key={pad.id} className="relative">
                <button
                  type="button"
                  onClick={() => (editing ? setSelected(pad.id) : play(pad))}
                  disabled={!editing && !pad.path}
                  title={pad.path || t("soundboard-no-clip")}
                  className={`flex h-16 w-full flex-col items-center justify-center gap-0.5 rounded-md border px-1 text-center text-[11px] transition-colors ${
                    pad.path
                      ? "border-havoc-accent/40 bg-havoc-accent/10 text-havoc-text hover:border-havoc-accent/70 hover:bg-havoc-accent/20"
                      : "border-white/10 text-havoc-muted"
                  } ${selected === pad.id ? "ring-1 ring-havoc-accent" : ""}`}
                >
                  <span className="line-clamp-2 leading-tight">
                    {pad.name || t("soundboard-new-pad")}
                  </span>
                  {pad.hotkey && <span className="text-[9px] text-havoc-muted">{pad.hotkey}</span>}
                </button>
              </div>
            ))}
          </div>
        )}

        {editing && selectedPad && (
          <PadEditor
            pad={selectedPad}
            onChange={(patch) => setPad(selectedPad.id, patch)}
            onPickClip={() => pickClip(selectedPad.id)}
            onRemove={() => removePad(selectedPad.id)}
          />
        )}
      </div>
    </PickerShell>
  );
}

function PadEditor({
  pad,
  onChange,
  onPickClip,
  onRemove,
}: {
  pad: SoundboardPad;
  onChange: (patch: Partial<SoundboardPad>) => void;
  onPickClip: () => void;
  onRemove: () => void;
}) {
  const t = useT();
  const field =
    "rounded border border-white/10 bg-havoc-panel px-1.5 py-0.5 text-[11px] text-havoc-text";
  return (
    <section className="flex flex-col gap-2 rounded-md border border-white/10 bg-white/[0.02] p-3 text-[11px] text-havoc-muted">
      <label className="flex items-center gap-2">
        {t("soundboard-name")}
        <input
          value={pad.name}
          onChange={(e) => onChange({ name: e.target.value })}
          className={`flex-1 ${field}`}
        />
      </label>
      <div className="flex items-center gap-2">
        <span className="min-w-0 flex-1 truncate text-havoc-text" title={pad.path}>
          {pad.path || t("soundboard-no-clip")}
        </span>
        <button
          type="button"
          onClick={onPickClip}
          className="rounded border border-white/10 px-2 py-0.5 text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
        >
          {t("soundboard-choose-clip")}
        </button>
      </div>
      <div className="flex flex-wrap items-center gap-3">
        <label className="flex items-center gap-1">
          {t("soundboard-gain")}
          <input
            type="number"
            min={-60}
            max={12}
            step={0.5}
            value={pad.gainDb}
            onChange={(e) =>
              onChange({ gainDb: Math.min(12, Math.max(-60, Number(e.target.value))) })
            }
            className={`w-16 ${field}`}
          />
        </label>
        <label className="flex items-center gap-1">
          {t("soundboard-choke")}
          <select
            value={pad.chokeGroup}
            onChange={(e) => onChange({ chokeGroup: Number(e.target.value) })}
            className={field}
          >
            <option value={0}>{t("soundboard-choke-none")}</option>
            {[1, 2, 3, 4, 5, 6, 7, 8].map((g) => (
              <option key={g} value={g}>
                {g}
              </option>
            ))}
          </select>
        </label>
        <label className="flex items-center gap-1">
          <input
            type="checkbox"
            checked={pad.looping}
            onChange={(e) => onChange({ looping: e.target.checked })}
          />
          {t("soundboard-loop")}
        </label>
        <label className="flex items-center gap-1">
          <input
            type="checkbox"
            checked={pad.autoDuck}
            onChange={(e) => onChange({ autoDuck: e.target.checked })}
          />
          {t("soundboard-auto-duck")}
        </label>
      </div>
      <div className="flex flex-wrap items-center gap-3">
        <label className="flex items-center gap-1">
          {t("soundboard-tracks")}
          {[0, 1, 2, 3, 4, 5].map((i) => (
            <button
              key={i}
              type="button"
              onClick={() => onChange({ tracks: pad.tracks ^ (1 << i) })}
              className={`h-5 w-5 rounded border text-[9px] ${
                (pad.tracks & (1 << i)) !== 0
                  ? "border-havoc-accent bg-havoc-accent/30 text-havoc-text"
                  : "border-white/10 text-havoc-muted"
              }`}
            >
              {i + 1}
            </button>
          ))}
        </label>
        <label className="flex items-center gap-1">
          {t("soundboard-hotkey")}
          <input
            value={pad.hotkey ?? ""}
            onChange={(e) => onChange({ hotkey: e.target.value.trim() || null })}
            placeholder={t("soundboard-hotkey-placeholder")}
            className={`w-28 ${field}`}
          />
        </label>
        <button
          type="button"
          onClick={onRemove}
          className="ml-auto rounded border border-white/10 px-2 py-0.5 text-havoc-muted hover:border-red-500/60 hover:text-red-400"
        >
          {t("soundboard-remove")}
        </button>
      </div>
    </section>
  );
}
