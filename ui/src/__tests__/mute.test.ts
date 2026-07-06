import { describe, expect, it } from "vitest";

import {
  GATES_CLEAR,
  guestButton,
  guestCanToggle,
  guestToggleSelf,
  hostToggle,
  isAudible,
} from "../remote/mute";

describe("two-gate guest mute", () => {
  it("is audible only when both gates are clear", () => {
    expect(isAudible({ hostGate: false, selfGate: false })).toBe(true);
    expect(isAudible({ hostGate: true, selfGate: false })).toBe(false);
    expect(isAudible({ hostGate: false, selfGate: true })).toBe(false);
    expect(isAudible({ hostGate: true, selfGate: true })).toBe(false);
  });

  it("reads the button: red host-mute wins over yellow self-mute", () => {
    expect(guestButton(GATES_CLEAR)).toBe("live");
    expect(guestButton({ hostGate: false, selfGate: true })).toBe("selfMuted"); // yellow
    expect(guestButton({ hostGate: true, selfGate: false })).toBe("hostMuted"); // red
    expect(guestButton({ hostGate: true, selfGate: true })).toBe("hostMuted"); // red wins
  });

  it("lets the guest self-mute and self-unmute while the host gate is clear", () => {
    let gates = GATES_CLEAR;
    expect(guestCanToggle(gates)).toBe(true);
    gates = guestToggleSelf(gates);
    expect(gates).toEqual({ hostGate: false, selfGate: true });
    gates = guestToggleSelf(gates);
    expect(gates).toEqual({ hostGate: false, selfGate: false });
  });

  it("forbids the guest from self-unmuting a host mute", () => {
    const hostMuted = { hostGate: true, selfGate: false };
    expect(guestCanToggle(hostMuted)).toBe(false);
    // Toggling is a no-op — the host gate can't be cleared by the guest.
    expect(guestToggleSelf(hostMuted)).toEqual(hostMuted);
  });

  it("keeps a guest's self-mute across a host mute + unmute", () => {
    let gates = guestToggleSelf(GATES_CLEAR); // guest self-mutes (yellow)
    expect(guestButton(gates)).toBe("selfMuted");
    gates = hostToggle(gates); // host also mutes → red, self gate preserved underneath
    expect(gates).toEqual({ hostGate: true, selfGate: true });
    expect(guestButton(gates)).toBe("hostMuted");
    gates = hostToggle(gates); // host lifts → back to the guest's own self-mute
    expect(gates).toEqual({ hostGate: false, selfGate: true });
    expect(guestButton(gates)).toBe("selfMuted");
  });

  it("only ever changes the gate its owner controls", () => {
    // host toggle never touches selfGate…
    expect(hostToggle({ hostGate: false, selfGate: true })).toEqual({
      hostGate: true,
      selfGate: true,
    });
    // …and guest toggle never touches hostGate.
    expect(guestToggleSelf({ hostGate: false, selfGate: false })).toEqual({
      hostGate: false,
      selfGate: true,
    });
  });
});
