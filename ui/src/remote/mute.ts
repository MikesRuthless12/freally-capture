/**
 * Remote Guests — the two-gate guest mute (TASK-R7).
 *
 * A guest's audio passes only when BOTH gates are clear:
 *   - the **host gate** — only the host sets/clears it;
 *   - the **self gate** — only the guest sets/clears it.
 *
 * A host-muted guest cannot unmute themselves (they wait for the host to lift
 * it) but may still self-mute / self-unmute whenever the host gate is clear.
 * The guest's button reads the state: red = muted by host (locked), yellow =
 * self-muted (can self-unmute), normal = live. This module is the pure state
 * machine; the data channel + UI + audio gating wire onto it.
 */
export type GateState = { hostGate: boolean; selfGate: boolean };

export const GATES_CLEAR: GateState = { hostGate: false, selfGate: false };

/** The guest is heard only when neither gate mutes them. */
export function isAudible(gates: GateState): boolean {
  return !gates.hostGate && !gates.selfGate;
}

/** How the guest's own mute button reads. */
export type GuestButton = "live" | "selfMuted" | "hostMuted";

export function guestButton(gates: GateState): GuestButton {
  if (gates.hostGate) return "hostMuted"; // red — locked until the host lifts it
  if (gates.selfGate) return "selfMuted"; // yellow — the guest can self-unmute
  return "live";
}

/** Whether the guest may act on their own mute button right now. */
export function guestCanToggle(gates: GateState): boolean {
  return !gates.hostGate;
}

/**
 * The guest toggles ONLY their self gate, and only while the host gate is
 * clear — a host mute cannot be self-cleared.
 */
export function guestToggleSelf(gates: GateState): GateState {
  if (gates.hostGate) return gates;
  return { ...gates, selfGate: !gates.selfGate };
}

/** The host toggles ONLY their own gate (never the guest's self gate). */
export function hostToggle(gates: GateState): GateState {
  return { ...gates, hostGate: !gates.hostGate };
}
