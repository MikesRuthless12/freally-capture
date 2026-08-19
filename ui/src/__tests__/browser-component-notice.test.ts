import { describe, expect, it } from "vitest";

import type { CefStatus } from "../api/types";
import { browserComponentNotice } from "../lib/browserUrl";

describe("browserComponentNotice", () => {
  it("says nothing while the status is unknown or the runtime is ready", () => {
    expect(browserComponentNotice(null)).toBeNull();
    expect(
      browserComponentNotice({ state: "ready", version: "151.0.5", path: "/cache/cef" }),
    ).toBeNull();
  });

  it("points at Components only when there is something to install", () => {
    expect(browserComponentNotice({ state: "missing", supported: true })).toEqual({
      key: "sources-browser-component-missing",
    });
  });

  it("is honest when CEF publishes no build for the platform", () => {
    // Nothing to install here, so "install it from Components" would send the
    // operator on an errand that cannot succeed.
    expect(browserComponentNotice({ state: "missing", supported: false })).toEqual({
      key: "sources-browser-component-unsupported",
    });
    expect(
      browserComponentNotice({ state: "error", message: "no build", supported: false }),
    ).toEqual({ key: "sources-browser-component-unsupported" });
  });

  it("reports an install failure with its reason", () => {
    expect(
      browserComponentNotice({ state: "error", message: "checksum mismatch", supported: true }),
    ).toEqual({ key: "sources-browser-component-error", message: "checksum mismatch" });
  });

  it("distinguishes every mid-install state from 'not installed'", () => {
    const midInstall: CefStatus[] = [
      { state: "resolving" },
      { state: "downloading", receivedBytes: 10, totalBytes: 100, bytesPerSec: 5 },
      { state: "verifying" },
      { state: "extracting" },
    ];
    for (const status of midInstall) {
      expect(browserComponentNotice(status)).toEqual({
        key: "sources-browser-component-installing",
      });
    }
  });
});
