/**
 * Typed mirrors of the Rust bridge payloads.
 *
 * Keep in lockstep with `src-tauri/src/commands/mod.rs`,
 * `src-tauri/src/events.rs`, and `src-tauri/src/settings.rs`.
 */

/** One linked core crate, as reported by `health`. */
export type CrateHealth = {
  name: string;
  version: string;
};

/** The `health` command report. */
export type Health = {
  appVersion: string;
  os: string;
  coreOk: boolean;
  crates: CrateHealth[];
};

/** The persisted user settings (`settings.json` in the OS config dir). */
export type Settings = {
  language: string;
  showStatsDock: boolean;
};

/** The `stats` push-event payload (~2 Hz). */
export type StatsPayload = {
  fps: number;
  cpu: number;
  /** True until real sampling lands (P5.5) — the UI labels the data honestly. */
  placeholder: boolean;
};
