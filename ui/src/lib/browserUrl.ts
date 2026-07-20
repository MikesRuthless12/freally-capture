/** CAP-N77: mirrors the Rust `validate_url` rule — http/https only in v1
 * (local files play through the Media/Image sources; the form's hint says so). */
export function browserUrlValid(url: string): boolean {
  // Schemes are case-insensitive (RFC 3986); still an allowlist, fails closed.
  const scheme = url.trim().toLowerCase();
  return scheme.startsWith("http://") || scheme.startsWith("https://");
}
