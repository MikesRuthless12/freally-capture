import type { IngestProtocol } from "../api/types";

/** CAP-N11: the default listen port per protocol (SRT 9710, RTMP 1935). */
export const LAN_DEFAULT_PORTS: Record<IngestProtocol, number> = { srt: 9710, rtmp: 1935 };

/** CAP-N11: the URL a sender on the LAN dials (mirrors laningest.rs
 * `connect_url`). The SRT passphrase rides the query percent-encoded. */
export function lanIngestUrl(
  protocol: IngestProtocol,
  host: string,
  port: number,
  passphrase: string,
): string {
  if (protocol === "rtmp") return `rtmp://${host}:${port}/live`;
  const query = passphrase ? `?passphrase=${encodeURIComponent(passphrase)}` : "";
  return `srt://${host}:${port}${query}`;
}

/** SRT passphrases are 10–79 characters by spec (libsrt refuses others);
 * empty = an open listener (warned, not blocked). */
export function lanPassphraseUsable(protocol: IngestProtocol, passphrase: string): boolean {
  if (protocol !== "srt") return true;
  const chars = [...passphrase].length;
  return chars === 0 || (chars >= 10 && chars <= 79);
}
