/**
 * Remote Guests — invite tokens & links (TASK-R2/R3, the codec half).
 *
 * An invite is an opaque, URL-safe token that carries the host's live PeerJS
 * session id plus a host-chosen expiry. The guest decodes it, checks the
 * expiry, and calls that peer — so a shareable `freally://join?token=…` link
 * (or the future web-join URL) replaces pasting a raw session id.
 *
 * PoC honesty (matches design/remote-guests-p2p.md §10): the token embeds the
 * peer id (unguessable via PeerJS's random UUID) and a TTL that is checked
 * here. **Single-use** invalidation and server-side validation need the
 * server-backed signaling that lands with the productionized flow — this codec
 * is deliberately server-free. Nothing here is a security boundary on its own.
 */
export const INVITE_SCHEME = "freally://join";
export const INVITE_VERSION = 1;

type InvitePayload = { v: number; p: string; exp: number };

// The token is base64url of ASCII JSON (peer ids + our fields are ASCII).
function base64UrlEncode(ascii: string): string {
  return btoa(ascii).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}
function base64UrlDecode(token: string): string {
  const b64 = token.replace(/-/g, "+").replace(/_/g, "/");
  return atob(b64);
}

/** Build an invite token for `peerId` valid for `ttlMinutes` from `now` (ms). */
export function mintInvite(peerId: string, ttlMinutes: number, now: number): string {
  const payload: InvitePayload = { v: INVITE_VERSION, p: peerId, exp: now + ttlMinutes * 60_000 };
  return base64UrlEncode(JSON.stringify(payload));
}

export type InviteResult =
  { ok: true; peerId: string; exp: number } | { ok: false; reason: "expired" | "malformed" };

/** Decode + validate a token against `now` (ms). */
export function resolveInvite(token: string, now: number): InviteResult {
  let payload: unknown;
  try {
    payload = JSON.parse(base64UrlDecode(token.trim()));
  } catch {
    return { ok: false, reason: "malformed" };
  }
  if (
    typeof payload !== "object" ||
    payload === null ||
    (payload as InvitePayload).v !== INVITE_VERSION ||
    typeof (payload as InvitePayload).p !== "string" ||
    !(payload as InvitePayload).p ||
    typeof (payload as InvitePayload).exp !== "number"
  ) {
    return { ok: false, reason: "malformed" };
  }
  const { p: peerId, exp } = payload as InvitePayload;
  if (now > exp) {
    return { ok: false, reason: "expired" };
  }
  return { ok: true, peerId, exp };
}

/** The deep-link form of an invite. */
export function inviteLink(token: string): string {
  return `${INVITE_SCHEME}?token=${token}`;
}

/** The hosted web-join page (TASK-R3): the guest end running in a plain
 * browser — what phone guests reach from the scanned QR, no install. The
 * page (`docs/join.html`) mirrors this module's token format by hand — bump
 * both together. */
export const WEB_JOIN_BASE = "https://mikesruthless12.github.io/freally-capture/join.html";

/** The browser form of an invite — what the QR encodes. */
export function webJoinLink(token: string): string {
  return `${WEB_JOIN_BASE}?token=${token}`;
}

/**
 * Pull a token out of whatever the guest pasted — a full `freally://join?token=…`
 * link, an `https://…/join?token=…` web-join URL, or a bare token. Returns
 * `null` when there's nothing token-shaped.
 */
export function parseInviteInput(input: string): string | null {
  const trimmed = input.trim();
  const match = trimmed.match(/[?&]token=([^&\s]+)/);
  if (match) {
    return decodeURIComponent(match[1]);
  }
  // A bare token is base64url only.
  return /^[A-Za-z0-9_-]+$/.test(trimmed) && trimmed.length >= 8 ? trimmed : null;
}

export type JoinTarget = { peerId: string } | { error: string };

/**
 * Resolve whatever the guest pasted to a peer id to call: a valid invite
 * (link or token) → its peer; an *expired* invite → an honest error; anything
 * else non-empty is treated as a raw session id (back-compat / direct join).
 */
export function joinTargetFromInput(input: string, now: number): JoinTarget {
  const token = parseInviteInput(input);
  if (token) {
    const result = resolveInvite(token, now);
    if (result.ok) {
      return { peerId: result.peerId };
    }
    if (result.reason === "expired") {
      return { error: "This invite has expired — ask the host for a new one." };
    }
    // Malformed as an invite: fall through — it may be a raw session id.
  }
  const raw = input.trim();
  return raw ? { peerId: raw } : { error: "Paste an invite link or a session id." };
}
