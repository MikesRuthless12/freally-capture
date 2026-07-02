import { EmptyHint, Panel } from "../components/Panel";

/** The Scenes rail — create/rename/reorder scenes (lands with the scene model, 0.40.0). */
export function ScenesRail() {
  return (
    <Panel
      title="Scenes"
      actions={
        <button
          type="button"
          disabled
          title="Scenes arrive with the compositor (0.40.0)"
          className="rounded-md border border-white/10 px-2 py-0.5 text-xs text-havoc-muted opacity-60"
        >
          +
        </button>
      }
    >
      <EmptyHint>
        No scenes yet. Scenes and the scene model land with the compositor (0.40.0).
      </EmptyHint>
    </Panel>
  );
}
