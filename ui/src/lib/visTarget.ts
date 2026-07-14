import type { SourceId, VisTargetKind } from "../api/types";

/** CAP-N15: the visualizer target as one <select> value — "master",
 * "track:N", or "source:ID". */
export function visTargetKey(settings: {
  target: VisTargetKind;
  track: number;
  source?: SourceId | null;
}): string {
  if (settings.target === "master") return "master";
  if (settings.target === "track") return `track:${settings.track}`;
  return `source:${settings.source ?? ""}`;
}

/** The inverse of `visTargetKey`. `track`/`source` are null when the key
 * names another target kind (the caller keeps its current value). */
export function parseVisTarget(key: string): {
  target: VisTargetKind;
  track: number | null;
  source: SourceId | null;
} {
  if (key.startsWith("track:"))
    return { target: "track", track: Number(key.slice("track:".length)), source: null };
  if (key.startsWith("source:"))
    return { target: "source", track: null, source: key.slice("source:".length) };
  return { target: "master", track: null, source: null };
}
