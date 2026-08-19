import type { CefStatus } from "../api/types";

/** CAP-N77: mirrors the Rust `validate_url` rule — http/https only in v1
 * (local files play through the Media/Image sources; the form's hint says so). */
export function browserUrlValid(url: string): boolean {
  // Schemes are case-insensitive (RFC 3986); still an allowlist, fails closed.
  const scheme = url.trim().toLowerCase();
  return scheme.startsWith("http://") || scheme.startsWith("https://");
}

/**
 * What the Browser picker should say about the runtime component — or `null`
 * when it is ready (or not read yet, so no wrong advice ever flashes).
 *
 * The states are deliberately NOT collapsed into one "missing" banner:
 * "install it from Components" is wrong advice while an install is already
 * running, and wrong again on a platform CEF publishes no build for, where
 * there is nothing to install at all.
 */
export type BrowserComponentNotice =
  | { key: "sources-browser-component-missing" }
  | { key: "sources-browser-component-unsupported" }
  | { key: "sources-browser-component-installing" }
  | { key: "sources-browser-component-error"; message: string };

export function browserComponentNotice(cef: CefStatus | null): BrowserComponentNotice | null {
  if (cef === null) return null;
  switch (cef.state) {
    case "ready":
      return null;
    case "missing":
      return cef.supported
        ? { key: "sources-browser-component-missing" }
        : { key: "sources-browser-component-unsupported" };
    case "error":
      // An unsupported platform reports its reason through `error` too; the
      // honest "no build exists" line beats echoing a fetch failure.
      return cef.supported
        ? { key: "sources-browser-component-error", message: cef.message }
        : { key: "sources-browser-component-unsupported" };
    default:
      // resolving / downloading / verifying / extracting — an install is
      // already under way, so pointing at Components would be wrong.
      return { key: "sources-browser-component-installing" };
  }
}
