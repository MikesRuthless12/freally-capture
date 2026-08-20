# Local-only security & code-review tooling

Everything here runs **on your machine**. Nothing uploads source, and nothing
needs an account. Where a tool fetches data (a vulnerability database, a rule
pack) that is called out explicitly, along with how to run it offline afterwards.

Written 2026-08-19 while auditing the whole codebase section by section; results
filled in 2026-08-20 after actually running each one.

---

## What the repo already runs

| Check                     | Covers                                   | Where          |
| ------------------------- | ---------------------------------------- | -------------- |
| `clippy -D warnings`      | Rust lints, many correctness             | `ci:local`, CI |
| `cargo-deny`              | dependency licences + RustSec advisories | `ci:local`, CI |
| `cargo-audit`             | RustSec advisories                       | `ci:local`, CI |
| `eslint`                  | TS/React lints                           | `ci:local`, CI |
| `i18n:lint`, `theme:lint` | catalog parity, theme tokens             | `ci:local`, CI |

That is a good baseline, but note what it does **not** do: `cargo-audit` and
`cargo-deny` only look at _dependencies_. Nothing in the list analyses this
project's own Rust for memory-safety or logic defects.

---

## Recommended additions, in value order

### 1. In-tree fuzz sweeps — **added, no install required**

The highest-yield addition, and it needs no new tool. Every parser that takes
untrusted bytes now has a deterministic malformed-input sweep in the normal test
suite, so it runs on all three OSes in CI:

- `crates/encode/src/flz.rs` → `malformed_streams_never_panic_or_overrun`
  (single-byte mutations at every offset, every truncation, lying decoded
  lengths, pure noise)
- `crates/capture/src/window_match.rs` → `decoding_an_arbitrary_capture_id_never_panics`

**Why these two first:** `flz::decompress_into` writes through a cursor into a
caller-owned slice and trusts the input stream for every length and
back-reference; capture ids are persisted in scene collections, so they come
back from `.fcappack` files, OBS imports and hand-edited JSON.

**Still worth adding the same treatment to:**
`freally_video::read_index` / the `.frec` chunk reader, `scene::obs_import`,
`compositor::filters::cube::parse_cube`, and `encode::link`'s frame accumulator.

### 2. `cargo-fuzz` — deeper, coverage-guided (needs nightly) — **harness committed**

The sweeps above are cheap and deterministic; libFuzzer explores far further.

`fuzz/` holds the harness (`fuzz/Cargo.toml`, `fuzz/fuzz_targets/`); the corpus
and crash artifacts are gitignored. It is excluded from the workspace, so a
stable-toolchain `cargo test --workspace` never tries to build it.

```bash
rustup toolchain install nightly
cargo install cargo-fuzz
cd fuzz && cargo +nightly fuzz run flz_decompress -- -max_total_time=300
```

Fully local once installed. **Run it in the Linux container** — libFuzzer is
best supported there.

**Result (2026-08-20):** 77,625,230 executions in 181 s against
`flz::decompress_into`, **zero crashes**. Worth knowing because that function
was rewritten during this audit to decode straight into a caller-owned slice
(removing a full-frame allocation and copy per decoded frame) — exactly the kind
of change that turns a bounds bug into an out-of-bounds write.

### 3. Semgrep OSS — the TypeScript/React half — **run, clean**

`ui/` is ~46k lines and only eslint looks at it.

**Result (2026-08-20):** 74 rules over 166 TS/TSX files, **0 findings**. Run in
Docker (`semgrep/semgrep:latest`) so nothing is installed on the host:

```bash
docker run --rm -v "$PWD:/src" semgrep/semgrep:latest \n  semgrep scan --config p/typescript --config p/react --metrics=off /src/ui/src
```

```bash
pipx install semgrep        # or: python -m pip install semgrep
semgrep scan --config p/typescript --config p/react --config p/secrets ui/src
```

**Offline note:** `--config auto` and the `p/…` packs fetch rules from the
registry on first use. To stay fully offline, vendor the rule YAML once and
point `--config` at the local directory. Semgrep's Rust support is thin — treat
this as a UI-side tool, not a whole-repo one.

### 4. Trivy — secrets + npm dependencies — **run, clean**

**Result (2026-08-20):** 0 secrets and 0 HIGH/CRITICAL vulnerabilities across
`Cargo.lock` and `package-lock.json`. Also runs in Docker:

```bash
docker run --rm -v "$PWD:/src" -v trivy-cache:/root/.cache/trivy \n  aquasec/trivy:latest fs --scanners vuln,secret \n  --skip-dirs node_modules,target,.git /src
```

Local binary; downloads its vulnerability DB once, then `--offline-scan` works
from the cache. Overlaps `cargo-audit` for Rust deps, but adds **secret
scanning** and covers `package-lock.json`, which nothing currently does.

---

## Deliberately NOT recommended here

### Miri — skip it

Miri is the standard advice for a codebase with `unsafe`, and it is the wrong
call for this one. All twelve library crates already `deny`/`forbid`
unsafe_code:

```
appaudio  audio  capture  compositor  encode  ndi
plugin    preview  scene   script     sources  stream
```

The `unsafe` that exists lives entirely in the per-OS FFI modules — Win32/COM,
ScreenCaptureKit, PipeWire, wgpu surface creation — and **Miri cannot execute
FFI**. It would run over code the compiler already proves safe and find nothing.
Revisit only if a pure-Rust `unsafe` block ever lands.

### Cloud-backed scanners — out of scope by request

CodeQL-in-Actions, the GitHub MCP server (code scanning / Dependabot / secret
scanning alerts), Snyk and similar are genuinely good and free for public
repos, but they run server-side. Excluded here on purpose.

> The CodeQL **CLI** is the exception: it builds its database and runs queries
> entirely locally, and is free for open-source. It is the one cloud-branded
> tool that fits a local-only rule — worth revisiting if the fuzzing above
> stops finding things.

---

## Where the real risk is concentrated

For anyone picking up this file later — the audit found the dangerous surface is
not evenly spread:

1. **`crates/game-hook` and `crates/vcam-source`** run inside _someone else's
   process_ (a game; an OS frame-server service). A defect there crashes a third
   party. Both ship unsigned.
2. **`crates/browser-host`** renders arbitrary web pages, and a page URL can
   arrive from an imported scene collection.
3. **User WGSL shaders** (`FilterKind::UserShader`) are compiled and run from
   whatever a `.fcappack` contains.
4. **Every `.fcappack` / OBS import** is attacker-influenceable input to the
   scene model, and `Collection::sanitize` is the one chokepoint that repairs it.
