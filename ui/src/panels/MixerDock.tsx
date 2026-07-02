import { EmptyHint, Panel } from "../components/Panel";

/** The Audio Mixer dock — channel strips with VU meters (lands with audio, 0.55.0). */
export function MixerDock() {
  return (
    <Panel title="Audio Mixer">
      <EmptyHint>No audio sources yet. The audio graph and mixer land in 0.55.0.</EmptyHint>
    </Panel>
  );
}
