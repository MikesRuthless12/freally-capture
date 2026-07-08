import fs from "node:fs";

import { test } from "@playwright/test";

// The visual-smoke gallery: boot the real built UI with the mocked Tauri IPC and
// screenshot every feature panel. Each screenshot is a rendering confirmation.
const DIR = "e2e/screenshots";
fs.mkdirSync(DIR, { recursive: true });

async function boot(page: import("@playwright/test").Page, query = "") {
  await page.addInitScript({ path: "e2e/tauri-mock.js" });
  await page.goto("/" + query);
}

async function studioReady(page: import("@playwright/test").Page) {
  await page.getByRole("button", { name: "Add a source" }).waitFor({ timeout: 15_000 });
  await page.waitForTimeout(400);
}

test("01 — EULA acceptance gate", async ({ page }) => {
  await boot(page, "?eula=0");
  await page.getByRole("button", { name: /I Agree/ }).waitFor({ timeout: 15_000 });
  await page.waitForTimeout(300);
  await page.screenshot({ path: `${DIR}/01-eula-gate.png`, fullPage: false });
});

test("02 — studio shell", async ({ page }) => {
  await boot(page);
  await studioReady(page);
  await page.screenshot({ path: `${DIR}/02-studio-shell.png`, fullPage: false });
});

test("03 — sources: add menu + pickers", async ({ page }) => {
  await boot(page);
  await studioReady(page);
  await page.getByRole("button", { name: "Add a source" }).click();
  await page.waitForTimeout(300);
  await page.screenshot({ path: `${DIR}/03-sources-add-menu.png` });

  // App Audio picker (lists apps making sound).
  await page.getByText(/Application Audio/).click();
  await page.waitForTimeout(500);
  await page.screenshot({ path: `${DIR}/03b-source-app-audio.png` });
});

test("04 — sources: game capture consent", async ({ page }) => {
  await boot(page);
  await studioReady(page);
  await page.getByRole("button", { name: "Add a source" }).click();
  await page.waitForTimeout(300);
  await page.getByText(/Game Capture/).click();
  await page.waitForTimeout(500);
  await page.screenshot({ path: `${DIR}/04-source-game-capture.png` });
});

// The Controls-dock dialogs, opened by their exact button label.
const DIALOGS: Array<[string, RegExp]> = [
  ["10-components-downloads", /Codecs/],
  ["11-recordings", /Files…/],
  ["12-output", /Output…/],
  ["13-stream", /Stream…/],
  ["14-replay", /Replay…/],
  ["15-hotkeys", /Keys…/],
  ["16-scripts", /Scripts…/],
  ["17-docks", /Docks…/],
  ["18-remote", /Remote…/],
  ["19-profiles", /Profiles…/],
  ["20-bug-report", /Report a bug/],
  ["21-updates", /Check for updates/],
];

for (const [file, label] of DIALOGS) {
  test(`dialog — ${file}`, async ({ page }) => {
    await boot(page);
    await studioReady(page);
    await page.getByRole("button", { name: label }).first().click();
    await page.waitForTimeout(700);
    await page.screenshot({ path: `${DIR}/${file}.png` });
  });
}
