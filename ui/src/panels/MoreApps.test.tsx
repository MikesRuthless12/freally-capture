import { render, screen } from "@testing-library/react";
import { afterEach, beforeEach, expect, test, vi } from "vitest";

import { MoreAppsDialog } from "./MoreApps";

// The embed smoke (Central's FC-50 DoD): the vendored panel renders inside our
// shell, and its fcp-* strings resolve through OUR Fluent runtime — i.e. the
// submodule's catalogs really are loaded into this app's bundles.
beforeEach(() => {
  // Offline: the hosted catalog + GitHub release fetches fail, so the panel
  // falls back to its bundled catalog and hides counts (its honest empty state).
  vi.stubGlobal(
    "fetch",
    vi.fn(() => Promise.reject(new Error("offline"))),
  );
  window.__FC_TEST__ = { platform: { os: "windows", arch: "x86_64" } };
});

afterEach(() => {
  delete window.__FC_TEST__;
  vi.unstubAllGlobals();
});

test("More Freally apps embeds the Central panel, localized via our catalogs", async () => {
  render(<MoreAppsDialog onClose={() => {}} />);

  // Our own chrome key (moreapps-title) titles the dialog.
  expect(screen.getByRole("dialog", { name: "More Freally apps" })).toBeInTheDocument();

  // The bundled catalog's cards render — real panel, no copy of its logic here.
  expect(await screen.findByText("Freally Capture")).toBeInTheDocument();
  expect(screen.getByText("Freally Vault")).toBeInTheDocument();

  // A vendored fcp-* string resolves through Capture's t() — the submodule's
  // 18-locale catalogs are wired into our bundles.
  expect(screen.getByText("Download & install all")).toBeInTheDocument();
});
