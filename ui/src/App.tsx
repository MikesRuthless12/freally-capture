import { ControlsDock } from "./panels/ControlsDock";
import { MixerDock } from "./panels/MixerDock";
import { PreviewPanel } from "./panels/PreviewPanel";
import { ScenesRail } from "./panels/ScenesRail";
import { SourcesRail } from "./panels/SourcesRail";
import { StatsDock } from "./panels/StatsDock";

/** The Freally Capture studio shell: preview + rails + bottom docks. */
export default function App() {
  return (
    <div className="flex h-full flex-col gap-2 p-2">
      <header className="flex shrink-0 items-center justify-between rounded-xl border border-white/10 bg-white/[0.03] px-4 py-2">
        <span className="bg-gradient-to-r from-havoc-accent to-havoc-accent-2 bg-clip-text text-sm font-bold tracking-wide text-transparent">
          Freally Capture
        </span>
        <span className="text-xs text-havoc-muted">pre-release scaffold</span>
      </header>

      <main className="flex min-h-0 flex-1 flex-col gap-2">
        <div className="grid min-h-0 flex-1 grid-cols-[240px_minmax(0,1fr)_240px] gap-2">
          <ScenesRail />
          <PreviewPanel />
          <SourcesRail />
        </div>
        <div className="grid h-44 shrink-0 grid-cols-[2fr_1fr_1fr] gap-2">
          <MixerDock />
          <ControlsDock />
          <StatsDock />
        </div>
      </main>
    </div>
  );
}
