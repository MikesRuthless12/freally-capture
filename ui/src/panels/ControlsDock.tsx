import { Panel } from "../components/Panel";

const buttonBase =
  "w-full rounded-lg border px-3 py-2 text-left text-xs font-medium transition-colors disabled:cursor-not-allowed disabled:opacity-50";

/** The Controls dock — Start Recording / Go Live / Virtual Camera land in 0.55–0.70. */
export function ControlsDock() {
  return (
    <Panel title="Controls">
      <div className="flex flex-col gap-2">
        <button
          type="button"
          disabled
          title="Recording arrives in 0.55.0"
          className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-text`}
        >
          ● Start Recording
        </button>
        <button
          type="button"
          disabled
          title="Streaming arrives with the studio MVP (0.70.0)"
          className={`${buttonBase} border-havoc-accent/40 bg-gradient-to-r from-havoc-accent/20 to-havoc-accent-2/20 text-havoc-text`}
        >
          ⦿ Go Live
        </button>
        <button
          type="button"
          disabled
          title="The virtual camera arrives with the studio MVP (0.70.0)"
          className={`${buttonBase} border-white/10 bg-white/[0.04] text-havoc-text`}
        >
          ⌁ Start Virtual Camera
        </button>
      </div>
    </Panel>
  );
}
