import { EmptyHint, Panel } from "../components/Panel";

/** The Sources rail — add/arrange sources (first sources land with capture, 0.25.0). */
export function SourcesRail() {
  return (
    <Panel
      title="Sources"
      actions={
        <button
          type="button"
          disabled
          title="Display Capture and Webcam arrive with capture (0.25.0)"
          className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted opacity-60"
        >
          +
        </button>
      }
    >
      <EmptyHint>
        No sources yet. Display Capture and Webcam arrive with capture (0.25.0); the full source set
        follows with the compositor (0.40.0).
      </EmptyHint>
    </Panel>
  );
}
