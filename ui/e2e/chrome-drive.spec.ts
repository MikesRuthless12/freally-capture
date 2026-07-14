import fs from "node:fs";

import { expect, test } from "@playwright/test";

// Drive the new OBS-style chrome (menu bar + Settings modal) end-to-end against
// the mocked Tauri IPC — a temporary DoD driver, not a committed gallery test.
const DIR = "e2e/screenshots";
fs.mkdirSync(DIR, { recursive: true });

async function boot(page: import("@playwright/test").Page) {
  await page.addInitScript({ path: "e2e/tauri-mock.js" });
  await page.goto("/");
  await page.getByRole("button", { name: "Add a source" }).waitFor({ timeout: 15_000 });
  await page.waitForTimeout(400);
}

test("menu bar renders and File menu opens", async ({ page }) => {
  await boot(page);
  const menubar = page.getByRole("menubar");
  await expect(menubar).toBeVisible();
  await page.getByRole("menuitem", { name: /^File$/ }).click();
  await page.waitForTimeout(300);
  await page.screenshot({ path: `${DIR}/chrome-01-file-menu.png` });
  // A menu should be showing with Settings in it.
  await expect(page.getByRole("menuitem", { name: /Settings/ }).first()).toBeVisible();
});

test("File → Settings opens the sidebar modal, Apply disabled until change", async ({ page }) => {
  await boot(page);
  await page.getByRole("menuitem", { name: /^File$/ }).click();
  await page
    .getByRole("menuitem", { name: /Settings/ })
    .first()
    .click();
  await page.waitForTimeout(500);
  await page.screenshot({ path: `${DIR}/chrome-02-settings-open.png` });
  // Sidebar categories present.
  await expect(page.getByRole("tab", { name: /General/ })).toBeVisible();
  await expect(page.getByRole("tab", { name: /Hotkeys/ })).toBeVisible();
  await expect(page.getByRole("tab", { name: /Accessibility/ })).toBeVisible();
  // Apply starts disabled (no edits yet).
  const apply = page.getByRole("button", { name: /^Apply$/ });
  await expect(apply).toBeDisabled();
});

test("Hotkeys category: comboboxes, exclusion", async ({ page }) => {
  await boot(page);
  await page.getByRole("menuitem", { name: /^File$/ }).click();
  await page
    .getByRole("menuitem", { name: /Settings/ })
    .first()
    .click();
  await page.getByRole("tab", { name: /Hotkeys/ }).click();
  await page.waitForTimeout(400);
  await page.screenshot({ path: `${DIR}/chrome-03-hotkeys.png` });
  // Every hotkey row is a <select> (no free-text input).
  const selects = page.locator('[role="tabpanel"] select');
  await expect(await selects.count()).toBeGreaterThan(5);
});

test("Accessibility category renders meter colors", async ({ page }) => {
  await boot(page);
  await page.getByRole("menuitem", { name: /^File$/ }).click();
  await page
    .getByRole("menuitem", { name: /Settings/ })
    .first()
    .click();
  await page.getByRole("tab", { name: /Accessibility/ }).click();
  await page.waitForTimeout(400);
  await page.screenshot({ path: `${DIR}/chrome-04-accessibility.png` });
});

test("Tools and Help menus open", async ({ page }) => {
  await boot(page);
  await page.getByRole("menuitem", { name: /^Tools$/ }).click();
  await page.waitForTimeout(250);
  await page.screenshot({ path: `${DIR}/chrome-05-tools-menu.png` });
  await page.getByRole("menuitem", { name: /^Help$/ }).click();
  await page.waitForTimeout(250);
  await page.screenshot({ path: `${DIR}/chrome-06-help-menu.png` });
});
