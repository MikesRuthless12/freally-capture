import { EmptyHint, Panel } from "../components/Panel";

/** One stat readout tile. */
function Stat({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-lg border border-white/5 bg-white/[0.03] px-2.5 py-1.5">
      <div className="text-[10px] tracking-wider uppercase text-havoc-muted">{label}</div>
      <div className="text-sm font-semibold tabular-nums">{value}</div>
    </div>
  );
}

/** The stats dock — live fps/CPU/health arrive with the core bridge. */
export function StatsDock() {
  return (
    <Panel title="Stats">
      <div className="grid grid-cols-3 gap-2">
        <Stat label="FPS" value="—" />
        <Stat label="CPU" value="—" />
        <Stat label="Dropped" value="—" />
      </div>
      <div className="mt-2">
        <EmptyHint>Waiting for the core bridge…</EmptyHint>
      </div>
    </Panel>
  );
}
