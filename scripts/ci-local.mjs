#!/usr/bin/env node
// Local CI — run the SAME checks as .github/workflows/ci.yml before pushing.
//
// Mirrors the CI jobs (this repo is an npm workspace: the root package.json
// proxies every UI script to the `ui` workspace, so all UI commands run from
// the repo root — no cd into ui/):
//   Rust: cargo fmt --check · clippy -D warnings · test  (+ cargo-deny /
//         cargo-audit when installed)
//   UI:   prettier (format:check) · eslint · i18n:lint · theme:lint · vitest
//         (test:ui) · build (tsc --noEmit && vite build) · Playwright e2e
//   Tauri: debug compile smoke  (npm run tauri -- build --debug --no-bundle)
//
// Speed: the Rust lane and the UI lane share no toolchain and no build lock, so
// they run CONCURRENTLY, and the four read-only UI lints (prettier/eslint/i18n/
// theme) run as one parallel batch. The Tauri debug build runs LAST, alone —
// it both compiles Rust (so it wants the warm cargo target the Rust lane leaves)
// and writes ui/dist via its beforeBuildCommand (so it must not race the UI
// lane's own `build`). Pass --serial to force the old one-at-a-time behaviour.
//
// Every check still runs even if an earlier one fails, and one summary prints at
// the end, so a single pass surfaces all problems. Exits non-zero if anything
// failed, so it's safe to gate a push on it.
//
// Usage:  node scripts/ci-local.mjs [--no-e2e] [--no-tauri-build] [--rust-only] [--ui-only] [--serial] [--install]
//   --no-e2e         skip the Playwright e2e step (fast inner-loop)
//   --no-tauri-build skip the Tauri debug compile (slow; ~cargo build of the app)
//   --rust-only      run only the Rust + Tauri-build checks
//   --ui-only        run only the UI checks
//   --serial         run every step sequentially (no lanes) — for clean logs
//   --install        (re)install deps first: npm ci + playwright chromium
import { spawn, spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const uiDir = join(repoRoot, "ui");

const args = new Set(process.argv.slice(2));
const noE2e = args.has("--no-e2e");
const noTauriBuild = args.has("--no-tauri-build");
const rustOnly = args.has("--rust-only");
const uiOnly = args.has("--ui-only");
const serial = args.has("--serial");
const doInstall = args.has("--install");

// Pass the whole probe as one shell string (not an args array) — with shell:true
// an args array triggers a Node deprecation warning and isn't escaped anyway.
function have(commandLine) {
  return spawnSync(commandLine, { stdio: "ignore", shell: true }).status === 0;
}

const hasRust =
  existsSync(join(repoRoot, "Cargo.toml")) || existsSync(join(repoRoot, "src-tauri", "Cargo.toml"));
const hasUi = existsSync(join(uiDir, "package.json"));

// Run one step, buffering its output so concurrent lanes don't interleave. The
// buffered output is printed under a header the moment the step finishes.
function run({ name, cmd, cwd }) {
  return new Promise((resolve) => {
    const started = process.hrtime.bigint();
    const child = spawn(cmd, { cwd, shell: true });
    let out = "";
    const collect = (buf) => {
      out += buf.toString();
    };
    child.stdout.on("data", collect);
    child.stderr.on("data", collect);
    child.on("close", (status) => {
      const secs = Number((process.hrtime.bigint() - started) / 1_000_000n) / 1000;
      const ok = status === 0;
      const bar = "─".repeat(Math.max(0, 56 - name.length));
      const where = cwd === repoRoot ? "." : "ui";
      console.log(`\n${ok ? "✓" : "✗"} ${name} ${bar}  ${secs.toFixed(1)}s`);
      console.log(`  $ ${cmd}  (in ${where})`);
      if (out.trim()) console.log(out.replace(/\n$/, ""));
      resolve({ name, ok, secs });
    });
  });
}

// Run a list of steps one after another, stopping the *lane* early only if a
// step fails? No — we want every check to run, so a lane runs all its steps
// regardless, collecting every result.
async function lane(steps) {
  const results = [];
  for (const s of steps) results.push(await run(s));
  return results;
}

// ---- Assemble the lanes ---------------------------------------------------

const rustSteps = [];
if (!uiOnly && hasRust) {
  rustSteps.push({ name: "rust: fmt", cmd: "cargo fmt --all -- --check", cwd: repoRoot });
  if (existsSync(join(repoRoot, "deny.toml")) && have("cargo deny --version")) {
    rustSteps.push({ name: "rust: cargo-deny", cmd: "cargo deny check", cwd: repoRoot });
  } else {
    console.log("• note: cargo-deny not installed (or no deny.toml) — skipping (CI runs it).");
  }
  if (have("cargo audit --version")) {
    rustSteps.push({
      name: "rust: cargo-audit",
      cmd: "cargo audit --ignore RUSTSEC-2026-0194 --ignore RUSTSEC-2026-0195",
      cwd: repoRoot,
    });
  } else {
    console.log("• note: cargo-audit not installed — skipping (CI runs it).");
  }
  rustSteps.push({
    name: "rust: clippy",
    cmd: "cargo clippy --workspace --all-targets -- -D warnings",
    cwd: repoRoot,
  });
  rustSteps.push({ name: "rust: test", cmd: "cargo test --workspace", cwd: repoRoot });
}

// The UI lane: the four read-only lints run as one parallel batch, then vitest
// and the build, then e2e (which needs the build).
const uiLintSteps = [];
const uiSeqSteps = [];
if (!rustOnly && hasUi) {
  uiLintSteps.push(
    { name: "ui: format:check", cmd: "npm run format:check", cwd: repoRoot },
    { name: "ui: lint", cmd: "npm run lint", cwd: repoRoot },
    { name: "ui: i18n:lint", cmd: "npm run i18n:lint", cwd: repoRoot },
    { name: "ui: theme:lint", cmd: "npm run theme:lint", cwd: repoRoot },
  );
  // build runs `tsc --noEmit` first, so there's no separate typecheck (as in CI).
  uiSeqSteps.push(
    { name: "ui: test:ui", cmd: "npm run test:ui", cwd: repoRoot },
    { name: "ui: build", cmd: "npm run build", cwd: repoRoot },
  );
  if (!noE2e) {
    uiSeqSteps.push({ name: "ui: e2e", cmd: "npm run test:e2e", cwd: repoRoot });
  } else {
    console.log("• note: --no-e2e — skipping Playwright e2e (CI runs it).");
  }
}

const tauriStep =
  !uiOnly && hasRust && hasUi && !noTauriBuild
    ? { name: "tauri: debug build", cmd: "npm run tauri -- build --debug --no-bundle", cwd: repoRoot }
    : null;
if (noTauriBuild) {
  console.log("• note: --no-tauri-build — skipping Tauri debug compile (CI runs it per-OS).");
}

async function installDeps() {
  if (!doInstall || !hasUi) return [];
  return lane([
    { name: "deps: npm ci", cmd: "npm ci", cwd: repoRoot },
    { name: "deps: playwright chromium", cmd: "npx playwright install --with-deps chromium", cwd: uiDir },
  ]);
}

// ---- Execute --------------------------------------------------------------

const totalSteps =
  rustSteps.length + uiLintSteps.length + uiSeqSteps.length + (tauriStep ? 1 : 0);
if (totalSteps === 0) {
  console.error("ci-local: nothing to run (no Rust/UI detected, or filtered out).");
  process.exit(1);
}

const wallStart = process.hrtime.bigint();
let results = [];

// Installs (if any) must finish before the lanes that use the deps.
results = results.concat(await installDeps());

if (serial) {
  // One at a time — cleanest logs, slowest wall-clock.
  results = results.concat(await lane(rustSteps));
  results = results.concat(await lane(uiLintSteps));
  results = results.concat(await lane(uiSeqSteps));
  if (tauriStep) results = results.concat(await lane([tauriStep]));
} else {
  console.log(
    `\n▶ Running the Rust lane (${rustSteps.length}) and UI lane ` +
      `(${uiLintSteps.length} lint + ${uiSeqSteps.length} seq) concurrently…`,
  );
  // Rust lane ∥ UI lane. UI lane = parallel lints, then sequential test/build/e2e.
  const uiLane = (async () => {
    const lintResults = await Promise.all(uiLintSteps.map(run));
    const seqResults = await lane(uiSeqSteps);
    return [...lintResults, ...seqResults];
  })();
  const [rustResults, uiResults] = await Promise.all([lane(rustSteps), uiLane]);
  results = results.concat(rustResults, uiResults);

  // Tauri last: wants the Rust lane's warm cargo target, and must not race the
  // UI lane's build over ui/dist. Both lanes are done here, so it runs alone.
  if (tauriStep) results = results.concat(await run(tauriStep));
}

const wallSecs = Number((process.hrtime.bigint() - wallStart) / 1_000_000n) / 1000;

console.log("\n" + "═".repeat(64));
console.log("  Local CI summary" + (serial ? " (serial)" : " (parallel lanes)"));
console.log("═".repeat(64));
let failed = 0;
for (const r of results) {
  const mark = r.ok ? "✓ pass" : "✗ FAIL";
  console.log(`  ${mark}  ${r.name.padEnd(24)} ${r.secs.toFixed(1)}s`);
  if (!r.ok) failed++;
}
console.log("─".repeat(64));
console.log(`  wall-clock: ${wallSecs.toFixed(1)}s across ${results.length} checks`);
console.log("═".repeat(64));

if (failed > 0) {
  console.error(`\n✗ ${failed} check(s) failed — fix before pushing.`);
  process.exit(1);
}
console.log("\n✓ All checks passed — matches CI. Safe to push.");
