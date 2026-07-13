import { useEffect, useMemo, useState } from "react";

import { midiLearn, midiPorts, settingsSet } from "../api/commands";
import { onMidiLearned } from "../api/events";
import { ALLOWED_COMMANDS } from "../api/types";
import type { MidiBinding, MidiControl, MidiSettings, Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/** "Note 60 ch1" / "CC 7 ch3" — how a learned control reads. */
function controlLabel(control: MidiControl): string {
  return control.kind === "note"
    ? `Note ${control.note} · ch${control.channel + 1}`
    : `CC ${control.cc} · ch${control.channel + 1}`;
}

/**
 * MIDI control surfaces (CAP-N03): MIDI-learn a pad, knob, or fader onto a
 * studio action, with LED / motor-fader feedback so the surface mirrors the
 * studio's real state.
 *
 * No port is opened until one is picked here, and a binding can only name a
 * command from the app's own fixed allowlist.
 */
export function MidiDialog({
  settings,
  sceneNames,
  onSaved,
  onClose,
}: {
  settings: Settings | null;
  sceneNames: string[];
  onSaved: (next: Settings) => void;
  onClose: () => void;
}) {
  const t = useT();
  const [error, setError] = useState<string | null>(null);
  const [ports, setPorts] = useState<{ inputs: string[]; outputs: string[] }>({
    inputs: [],
    outputs: [],
  });
  const [learning, setLearning] = useState(false);

  useEffect(() => {
    midiPorts()
      .then(([inputs, outputs]) => setPorts({ inputs: inputs ?? [], outputs: outputs ?? [] }))
      .catch(() => undefined);
  }, []);

  // Memoized: the learn listener below depends on it, and a fresh object on
  // every render would re-subscribe (and drop the in-flight learn) each time.
  const midi: MidiSettings = useMemo(
    () => settings?.midi ?? { input: "", output: "", bindings: [] },
    [settings],
  );

  // A learned control lands here and becomes a new (unbound) row.
  useEffect(() => {
    let cancelled = false;
    const unlisten = onMidiLearned((control) => {
      if (cancelled || !settings) return;
      setLearning(false);
      const next: Settings = {
        ...settings,
        midi: {
          ...midi,
          bindings: [
            ...midi.bindings,
            {
              control,
              target: { kind: "action", command: "startRecording", params: {} },
              feedback: true,
            },
          ],
        },
      };
      settingsSet(next)
        .then(() => onSaved(next))
        .catch((err) => setError(String(err)));
    }).catch(() => undefined);
    return () => {
      cancelled = true;
      void unlisten.then((fn) => fn?.());
    };
  }, [settings, midi, onSaved]);

  if (!settings) return null;

  const persist = (next: MidiSettings) => {
    const nextSettings = { ...settings, midi: next };
    setError(null);
    settingsSet(nextSettings)
      .then(() => onSaved(nextSettings))
      .catch((err) => setError(String(err)));
  };

  const updateBinding = (index: number, patch: Partial<MidiBinding>) =>
    persist({
      ...midi,
      bindings: midi.bindings.map((binding, at) =>
        at === index ? { ...binding, ...patch } : binding,
      ),
    });

  return (
    <PickerShell title={t("midi-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-havoc-muted">{t("midi-about")}</p>
        {error && <p className="m-0 text-red-400">{error}</p>}

        <div className="flex flex-wrap items-center gap-2">
          <label className="flex items-center gap-2 text-havoc-muted">
            {t("midi-input")}
            <select
              value={midi.input}
              onChange={(event) => persist({ ...midi, input: event.target.value })}
              className={inputClass}
            >
              <option value="">{t("midi-none")}</option>
              {ports.inputs.map((port) => (
                <option key={port} value={port}>
                  {port}
                </option>
              ))}
            </select>
          </label>
          <label className="flex items-center gap-2 text-havoc-muted">
            {t("midi-output")}
            <select
              value={midi.output}
              onChange={(event) => persist({ ...midi, output: event.target.value })}
              className={inputClass}
            >
              <option value="">{t("midi-none")}</option>
              {ports.outputs.map((port) => (
                <option key={port} value={port}>
                  {port}
                </option>
              ))}
            </select>
          </label>
          <button
            type="button"
            disabled={!midi.input}
            onClick={() => {
              const next = !learning;
              setLearning(next);
              midiLearn(next).catch((err) => setError(String(err)));
            }}
            className={`rounded-md border px-3 py-1.5 font-semibold disabled:opacity-50 ${
              learning
                ? "border-emerald-400/60 bg-emerald-500/15 text-emerald-300"
                : "border-havoc-accent/60 bg-havoc-accent/15"
            }`}
          >
            {learning ? t("midi-learning") : t("midi-learn")}
          </button>
        </div>

        {midi.bindings.length === 0 && <p className="m-0 text-havoc-muted">{t("midi-empty")}</p>}

        <ul className="m-0 flex list-none flex-col gap-1.5 p-0">
          {midi.bindings.map((binding, index) => (
            <li
              key={index}
              className="flex flex-wrap items-center gap-2 rounded-lg border border-white/10 p-2"
            >
              <code className="w-32 shrink-0 text-havoc-muted">
                {controlLabel(binding.control)}
              </code>

              <select
                value={binding.target.kind}
                onChange={(event) => {
                  const kind = event.target.value;
                  const target: MidiBinding["target"] =
                    kind === "action"
                      ? { kind: "action", command: "startRecording", params: {} }
                      : kind === "macro"
                        ? { kind: "macro", name: "" }
                        : kind === "scene"
                          ? { kind: "scene", scene: sceneNames[0] ?? "" }
                          : kind === "volume"
                            ? { kind: "volume", source: "" }
                            : { kind: "mute", source: "" };
                  updateBinding(index, { target });
                }}
                aria-label={t("midi-target")}
                className={inputClass}
              >
                <option value="action">{t("midi-target-action")}</option>
                <option value="macro">{t("midi-target-macro")}</option>
                <option value="scene">{t("midi-target-scene")}</option>
                <option value="volume">{t("midi-target-volume")}</option>
                <option value="mute">{t("midi-target-mute")}</option>
              </select>

              {binding.target.kind === "action" && (
                <select
                  value={binding.target.command}
                  onChange={(event) =>
                    updateBinding(index, {
                      target: { kind: "action", command: event.target.value, params: {} },
                    })
                  }
                  aria-label={t("midi-command")}
                  className={inputClass}
                >
                  {ALLOWED_COMMANDS.map((command) => (
                    <option key={command} value={command}>
                      {command}
                    </option>
                  ))}
                </select>
              )}
              {binding.target.kind === "macro" && (
                <select
                  value={binding.target.name}
                  onChange={(event) =>
                    updateBinding(index, { target: { kind: "macro", name: event.target.value } })
                  }
                  aria-label={t("midi-macro")}
                  className={inputClass}
                >
                  <option value="">—</option>
                  {(settings.automation?.macros ?? []).map((entry) => (
                    <option key={entry.name} value={entry.name}>
                      {entry.name}
                    </option>
                  ))}
                </select>
              )}
              {binding.target.kind === "scene" && (
                <select
                  value={binding.target.scene}
                  onChange={(event) =>
                    updateBinding(index, { target: { kind: "scene", scene: event.target.value } })
                  }
                  aria-label={t("midi-scene")}
                  className={inputClass}
                >
                  {sceneNames.map((name) => (
                    <option key={name} value={name}>
                      {name}
                    </option>
                  ))}
                </select>
              )}
              {(binding.target.kind === "volume" || binding.target.kind === "mute") && (
                <input
                  value={binding.target.source}
                  onChange={(event) =>
                    updateBinding(index, {
                      target:
                        binding.target.kind === "volume"
                          ? { kind: "volume", source: event.target.value }
                          : { kind: "mute", source: event.target.value },
                    })
                  }
                  placeholder={t("midi-source")}
                  aria-label={t("midi-source")}
                  className={`${inputClass} w-32`}
                />
              )}

              <label className="flex items-center gap-1 text-havoc-muted">
                <input
                  type="checkbox"
                  checked={binding.feedback}
                  onChange={(event) => updateBinding(index, { feedback: event.target.checked })}
                />
                {t("midi-feedback")}
              </label>

              <button
                type="button"
                onClick={() =>
                  persist({
                    ...midi,
                    bindings: midi.bindings.filter((_, at) => at !== index),
                  })
                }
                aria-label={t("midi-remove")}
                className="ml-auto rounded px-1.5 text-havoc-muted hover:text-red-400"
              >
                ×
              </button>
            </li>
          ))}
        </ul>
      </div>
    </PickerShell>
  );
}
