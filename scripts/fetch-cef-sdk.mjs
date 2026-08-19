#!/usr/bin/env node
// Fetch + verify + extract the pinned CEF **standard** SDK (headers +
// Release/libcef.lib) into ./.cef-sdk, for building the browser-host `cef`
// feature. Resolves the newest stable build in the pinned major from the same
// CDN index the app's fetcher uses, so CI never drifts from what ships.
//
// Env: CEF_MAJOR overrides the major (default: PINNED_CEF_MAJOR from
// crates/encode/src/cef.rs). Writes ./.cef-sdk (CEF_ROOT for build.rs).
import { createHash } from "node:crypto";
import {
  createReadStream,
  readFileSync,
  createWriteStream,
  existsSync,
  mkdirSync,
  readdirSync,
  renameSync,
} from "node:fs";
import { get } from "node:https";
import { spawnSync } from "node:child_process";
import { join } from "node:path";

// The pin is declared ONCE, in the app's own fetcher. If this script restated
// it, a bump could land here and not there — and CI would happily build a host
// against a runtime the shipped fetcher will never install (the exit-3 dead end
// the pin exists to prevent). Read it instead. `CEF_MAJOR` still overrides, for
// trying a line out locally.
function pinnedMajor() {
  const src = readFileSync(
    new URL("../crates/encode/src/cef.rs", import.meta.url),
    "utf8",
  );
  const m = src.match(/pub const PINNED_CEF_MAJOR:\s*u32\s*=\s*(\d+)\s*;/);
  if (!m) {
    throw new Error(
      "could not read PINNED_CEF_MAJOR from crates/encode/src/cef.rs — has it been renamed?",
    );
  }
  return Number(m[1]);
}

const MAJOR = Number(process.env.CEF_MAJOR) || pinnedMajor();
const INDEX = "https://cef-builds.spotifycdn.com/index.json";
const BASE = "https://cef-builds.spotifycdn.com/";
const OUT = join(process.cwd(), ".cef-sdk");

const PLATFORM = (() => {
  switch (`${process.platform}:${process.arch}`) {
    case "win32:x64":
      return "windows64";
    case "linux:x64":
      return "linux64";
    case "darwin:arm64":
      return "macosarm64";
    case "darwin:x64":
      return "macosx64";
    default:
      throw new Error(`no CEF platform key for ${process.platform}/${process.arch}`);
  }
})();

function fetchJson(url) {
  return new Promise((resolve, reject) => {
    get(url, (res) => {
      let d = "";
      res.on("data", (c) => (d += c));
      res.on("end", () => resolve(JSON.parse(d)));
    }).on("error", reject);
  });
}

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const file = createWriteStream(dest);
    const go = (u) =>
      get(u, (res) => {
        if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
          res.resume();
          return go(res.headers.location);
        }
        if (res.statusCode !== 200) {
          reject(new Error(`HTTP ${res.statusCode} for ${u}`));
          return;
        }
        res.pipe(file);
        file.on("finish", () => file.close(() => resolve()));
      }).on("error", reject);
    go(url);
  });
}

function sha1(path) {
  return new Promise((resolve, reject) => {
    const hash = createHash("sha1");
    const stream = createReadStream(path);
    stream.on("data", (d) => hash.update(d));
    stream.on("end", () => resolve(hash.digest("hex")));
    stream.on("error", reject);
  });
}

const cefMajor = (v) => parseInt(v.cef_version, 10);

// Extract the .tar.bz2.
//
// Windows ships bsdtar as `tar`, and it fails on CEF's bzip2 archives with
// "Child returned status 128 / Error is not recoverable" even though the
// download is intact (the sha1 verifies immediately before this). 7-Zip is
// present on the GitHub Windows runners and handles it, so try that first and
// fall back to `tar` — which is the right tool everywhere else.
function extract(archive, outDir) {
  const attempts =
    process.platform === "win32"
      ? [
          // 7z cannot do .tar.bz2 in one pass: bz2 → .tar, then untar.
          ["7z", ["x", "-y", `-o${outDir}`, archive]],
          ["7z", ["x", "-y", `-o${outDir}`, join(outDir, "cef.tar")]],
        ]
      : [["tar", ["-xjf", archive, "-C", outDir]]];

  for (const [cmd, args] of attempts) {
    const res = spawnSync(cmd, args, { stdio: "inherit" });
    if (res.status !== 0) {
      throw new Error(
        `extraction failed: ${cmd} exited ${res.status ?? res.signal}` +
          (res.error ? ` (${res.error.message})` : ""),
      );
    }
  }
}

async function main() {
  console.log(`Resolving CEF standard SDK for ${PLATFORM} in the ${MAJOR}.x line…`);
  const index = await fetchJson(INDEX);
  const versions = (index[PLATFORM]?.versions || [])
    .filter((v) => v.channel === "stable" && cefMajor(v) === MAJOR)
    .filter((v) => v.files.some((f) => f.type === "standard" && f.sha1))
    .sort((a, b) => b.cef_version.localeCompare(a.cef_version));
  if (versions.length === 0) throw new Error(`no stable ${MAJOR}.x standard build for ${PLATFORM}`);
  const file = versions[0].files.find((f) => f.type === "standard" && f.sha1);
  const url = BASE + encodeURIComponent(file.name);
  console.log(`  ${file.name} (${(file.size / 1048576).toFixed(0)} MB)`);

  mkdirSync(OUT, { recursive: true });
  const archive = join(OUT, "cef.tar.bz2");
  await download(url, archive);
  const got = await sha1(archive);
  if (got !== file.sha1) {
    throw new Error(`sha1 mismatch: expected ${file.sha1}, got ${got}`);
  }
  console.log("  sha1 verified.");

  extract(archive, OUT);

  // Flatten cef_binary_*/ into .cef-sdk/ so CEF_ROOT points straight at it.
  const dir = readdirSync(OUT).find((n) => n.startsWith("cef_binary_"));
  if (!dir) throw new Error("extracted SDK dir not found");
  for (const sub of ["include", "Release", "Resources"]) {
    const from = join(OUT, dir, sub);
    if (existsSync(from)) renameSync(from, join(OUT, sub));
  }
  console.log(`CEF SDK staged at ${OUT} (set CEF_ROOT to it).`);
}

main().catch((err) => {
  console.error(`fetch-cef-sdk: ${err.message}`);
  process.exit(1);
});
