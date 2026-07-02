import { useState } from "react";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/**
 * A numeric input that tolerates being emptied while typing. The naive
 * `Number(value) || fallback` pattern snaps an emptied field straight to the
 * fallback, making values impossible to retype — this keeps a local string
 * draft, commits only parseable in-range numbers, and restores the last
 * committed value on blur.
 */
export function NumberField({
  label,
  value,
  min,
  max,
  step,
  onCommit,
  className = "",
}: {
  label: string;
  value: number;
  min: number;
  max?: number;
  step?: number;
  onCommit: (value: number) => void;
  className?: string;
}) {
  const [draft, setDraft] = useState<string | null>(null);

  const shown = draft ?? String(value);
  const commitIfValid = (text: string) => {
    const parsed = Number(text);
    if (text.trim() !== "" && Number.isFinite(parsed)) {
      const clamped = Math.min(max ?? Number.POSITIVE_INFINITY, Math.max(min, parsed));
      onCommit(clamped);
      return clamped;
    }
    return null;
  };

  return (
    <label className={`flex flex-col gap-1 text-[11px] text-havoc-muted ${className}`}>
      {label}
      <input
        type="number"
        min={min}
        max={max}
        step={step}
        value={shown}
        onChange={(event) => {
          const text = event.target.value;
          setDraft(text);
          commitIfValid(text);
        }}
        onBlur={() => {
          if (draft !== null) commitIfValid(draft);
          // Drop the draft: the field snaps back to the committed value.
          setDraft(null);
        }}
        aria-label={label}
        className={inputClass}
      />
    </label>
  );
}
