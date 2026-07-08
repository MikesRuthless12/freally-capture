# Auto-updater & release signing (TASK-803)

Freally Capture ships a **self-hosted, signed auto-updater**. There is no update
server and no third-party service: the app checks a static `latest.json` published
on the GitHub Releases page and verifies every download against a public key baked
into the binary. An unsigned or tampered package is refused by the updater plugin
before it is ever applied.

## How it works

- **App side** — `tauri-plugin-updater` (`src-tauri/src/main.rs`), configured in
  `src-tauri/tauri.conf.json` → `plugins.updater`:
  - `pubkey` — the minisign **public** key (safe to commit; it's public).
  - `endpoints` — `…/releases/latest/download/latest.json` (the latest published,
    non-draft release wins).
  - UI: **⭳ Check for updates…** in the Controls dock (`ui/src/panels/Updates.tsx`).
    Nothing downloads without an explicit click; the app restarts to finish via
    `tauri-plugin-process`.
  - `bundle.createUpdaterArtifacts: true` makes the bundler emit the signed update
    package (`.sig`) beside each installer.

- **Release side** — `.github/workflows/release.yml`:
  - Each build job signs its artifact (when `TAURI_SIGNING_PRIVATE_KEY` is set) and
    uploads a space-free `update-<platform>-…` asset plus a `latest.json` fragment.
  - The release job merges the fragments into `latest.json` and attaches it to the
    draft release alongside every installer.

## One-time setup (required to ship updates)

The updater's security model **requires** signing, so a signing key must exist. One
was generated for this repo:

- **Public key** — already in `tauri.conf.json` (`plugins.updater.pubkey`).
- **Private key** — written to `.tauri/updater.key` (gitignored, **never commit**).

Set the private key as a repository secret so CI can sign releases:

```sh
# from the repo root, with the GitHub CLI authenticated:
gh secret set TAURI_SIGNING_PRIVATE_KEY < .tauri/updater.key
# the generated key has no password, so this secret is left unset:
#   TAURI_SIGNING_PRIVATE_KEY_PASSWORD
```

To rotate the key, run `npx tauri signer generate -w .tauri/updater.key -f`, paste
the new public key into `tauri.conf.json`, and re-set the secret. Losing the private
key means you can no longer sign updates for the installed base — keep a backup.

> Because `createUpdaterArtifacts` is on, a **full `tauri build`** (local or CI) needs
> `TAURI_SIGNING_PRIVATE_KEY` in the environment. `cargo build`/`test`/`clippy` are
> unaffected. Locally: `TAURI_SIGNING_PRIVATE_KEY="$(cat .tauri/updater.key)" npm run tauri -- build`.

## Code signing (installers) — dormant until certs exist

Installer code signing is **paid-cert territory** and stays off by default:

- **Windows** — MSI/NSIS ship unsigned; SmartScreen reputation clears with downloads.
  Wire an Authenticode cert via the standard Tauri `bundle.windows` signing config
  when one is purchased.
- **macOS** — ad-hoc signed (`APPLE_SIGNING_IDENTITY: "-"`); first launch is
  right-click → Open. The workflow already reads `APPLE_SIGNING_IDENTITY`,
  `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, `APPLE_ID`, `APPLE_PASSWORD`,
  `APPLE_TEAM_ID` from secrets — set them (a paid Developer ID cert) and the build
  switches to full Developer-ID signing + notarization with no workflow change.

## First-release checklist (proves the updater end to end)

The updater's signing and manifest are built and unit-safe, but the exact release
asset URLs can only be confirmed by a real multi-OS release run:

1. Tag a release; let `release.yml` build + draft.
2. Confirm `latest.json` and each `update-*` asset are attached to the draft.
3. Publish the draft, then open `…/releases/latest/download/latest.json` and check
   each platform URL resolves (HTTP 200).
4. Install the previous version, bump, and run **⭳ Check for updates…** to confirm
   the download verifies and installs.
