import { describe, expect, it } from "vitest";

import {
  INVITE_SCHEME,
  inviteLink,
  joinTargetFromInput,
  mintInvite,
  parseInviteInput,
  resolveInvite,
  WEB_JOIN_BASE,
  webJoinLink,
} from "../remote/invite";

const NOW = 1_700_000_000_000;
const PEER = "24cfeff4-d7fa-454d-ab12-d57d47a6aa09";

describe("invite tokens", () => {
  it("round-trips a peer id through mint → resolve", () => {
    const token = mintInvite(PEER, 30, NOW);
    const result = resolveInvite(token, NOW + 60_000);
    expect(result).toEqual({ ok: true, peerId: PEER, exp: NOW + 30 * 60_000 });
  });

  it("rejects an expired token", () => {
    const token = mintInvite(PEER, 10, NOW);
    const result = resolveInvite(token, NOW + 11 * 60_000);
    expect(result).toEqual({ ok: false, reason: "expired" });
  });

  it("accepts a token exactly at its expiry boundary", () => {
    const token = mintInvite(PEER, 10, NOW);
    expect(resolveInvite(token, NOW + 10 * 60_000).ok).toBe(true);
    expect(resolveInvite(token, NOW + 10 * 60_000 + 1).ok).toBe(false);
  });

  it("rejects garbage and tampered tokens as malformed", () => {
    expect(resolveInvite("not-a-token", NOW)).toEqual({ ok: false, reason: "malformed" });
    expect(resolveInvite("", NOW)).toEqual({ ok: false, reason: "malformed" });
    const token = mintInvite(PEER, 30, NOW);
    // Flip a character — the base64url no longer decodes to our JSON shape.
    const tampered = `X${token.slice(1)}`;
    expect(resolveInvite(tampered, NOW).ok).toBe(false);
  });

  it("builds a freally:// deep link", () => {
    const token = mintInvite(PEER, 30, NOW);
    expect(inviteLink(token)).toBe(`${INVITE_SCHEME}?token=${token}`);
  });

  it("builds a web-join link the join flow can round-trip", () => {
    const token = mintInvite(PEER, 30, NOW);
    const link = webJoinLink(token);
    expect(link).toBe(`${WEB_JOIN_BASE}?token=${token}`);
    // A scanned QR pastes back into the app too — same token, same peer.
    expect(parseInviteInput(link)).toBe(token);
    expect(joinTargetFromInput(link, NOW + 1000)).toEqual({ peerId: PEER });
  });
});

describe("parseInviteInput", () => {
  it("extracts the token from a deep link", () => {
    const token = mintInvite(PEER, 30, NOW);
    expect(parseInviteInput(inviteLink(token))).toBe(token);
  });

  it("extracts the token from a web-join URL", () => {
    const token = mintInvite(PEER, 30, NOW);
    expect(parseInviteInput(`https://freally.example/join?token=${token}&x=1`)).toBe(token);
  });

  it("accepts a bare token and rejects noise", () => {
    const token = mintInvite(PEER, 30, NOW);
    expect(parseInviteInput(`  ${token}  `)).toBe(token);
    expect(parseInviteInput("hi")).toBeNull();
    expect(parseInviteInput("")).toBeNull();
  });
});

describe("joinTargetFromInput", () => {
  it("resolves a valid invite link to its peer", () => {
    const link = inviteLink(mintInvite(PEER, 30, NOW));
    expect(joinTargetFromInput(link, NOW + 1000)).toEqual({ peerId: PEER });
  });

  it("surfaces an expired invite as an error", () => {
    const link = inviteLink(mintInvite(PEER, 5, NOW));
    const target = joinTargetFromInput(link, NOW + 6 * 60_000);
    expect("error" in target && target.error).toContain("expired");
  });

  it("treats a raw session id as a direct join target", () => {
    expect(joinTargetFromInput(PEER, NOW)).toEqual({ peerId: PEER });
  });

  it("errors on empty input", () => {
    expect("error" in joinTargetFromInput("   ", NOW)).toBe(true);
  });
});
