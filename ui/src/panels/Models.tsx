import { useEffect, useState } from "react";

import {
  cefCancel,
  cefInstall,
  cefRemove,
  cefStatus,
  ffmpegCancel,
  ffmpegInstall,
  ffmpegRemove,
  ffmpegStatus,
  integrationsStatus,
} from "../api/commands";
import { onCef, onFfmpeg } from "../api/events";
import type { CefStatus, FfmpegStatus, IntegrationsStatus } from "../api/types";
import { PickerShell } from "../components/PickerShell";

function formatMb(bytes: number): string {
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function formatRate(bytesPerSec: number): string {
  return `${(bytesPerSec / (1024 * 1024)).toFixed(1)} MB/s`;
}

/**
 * The clearly-labeled Components panel (mirrors Freally Snipper's Models
 * panel): manages the **on-demand ffmpeg** wire-codec component — the only
 * non-owned, fetched piece of the app. Everything on this panel is honest
 * by charter: what it is, why it exists, where it comes from, and that the
 * owned freally-video path never needs it.
 */
export function ModelsDialog({ onClose }: { onClose: () => void }) {
  const [status, setStatus] = useState<FfmpegStatus | null>(null);
  const [busy, setBusy] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let alive = true;
    ffmpegStatus()
      .then((current) => alive && setStatus(current))
      .catch(() => alive && setStatus(null));
    onFfmpeg((next) => setStatus(next))
      .then((fn) => {
        if (alive) unlisten = fn;
        else fn();
      })
      .catch(() => undefined);
    return () => {
      alive = false;
      unlisten?.();
    };
  }, []);

  const run = async (action: () => Promise<void>) => {
    setBusy(true);
    setActionError(null);
    try {
      await action();
    } catch (error) {
      setActionError(String(error));
    } finally {
      setBusy(false);
    }
  };

  const build =
    status && (status.state === "missing" || status.state === "error") ? status.build : null;

  return (
    <PickerShell title="Components" onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <section className="rounded-lg border border-white/10 bg-white/[0.03] p-3">
          <div className="flex items-baseline justify-between gap-2">
            <h4 className="m-0 text-xs font-semibold">FFmpeg — wire codecs</h4>
            <span className="shrink-0 rounded bg-havoc-accent/15 px-1.5 py-0.5 text-[10px] font-medium tracking-wide text-havoc-accent uppercase">
              Third-party · not bundled
            </span>
          </div>
          <p className="mt-2 mb-0 leading-relaxed text-havoc-muted">
            Freally Capture&apos;s own engine records lossless{" "}
            <span className="text-havoc-text">freally-video (.frec)</span> with nothing extra.
            Recording the wire formats platforms and players expect — H.264/AAC (and HEVC/AV1) in
            mp4/mkv/mov/webm — uses <span className="text-havoc-text">FFmpeg</span>, a separate tool
            this app never ships with: those codecs are patent-encumbered, so it stays optional and
            clearly labeled. It is downloaded on demand from the pinned build below,
            <span className="text-havoc-text"> SHA-256-verified before first use</span>, cached
            per-user, and driven as a separate process. Its license (LGPL/GPL) is its own — see
            THIRD-PARTY-NOTICES.
          </p>
        </section>

        <section className="rounded-lg border border-white/10 bg-white/[0.03] p-3">
          {status === null && <p className="m-0 text-havoc-muted">Checking…</p>}

          {status?.state === "missing" && (
            <div className="flex flex-col gap-2">
              <p className="m-0 text-havoc-muted">
                {build ? (
                  <>
                    Not installed. Available: FFmpeg{" "}
                    <span className="text-havoc-text">{build.version}</span> from{" "}
                    <span className="text-havoc-text">{build.source}</span> (
                    {formatMb(build.sizeBytes)} download).
                  </>
                ) : (
                  <>
                    No FFmpeg build is pinned for this platform yet — wire-codec recording is
                    unavailable here. Lossless freally-video recording is unaffected.
                  </>
                )}
              </p>
              {build && (
                <button
                  type="button"
                  disabled={busy}
                  onClick={() => run(ffmpegInstall)}
                  className="self-start rounded-lg border border-havoc-accent/40 bg-gradient-to-r from-havoc-accent/20 to-havoc-accent-2/20 px-3 py-1.5 font-medium text-havoc-text transition-colors hover:border-havoc-accent/70 disabled:cursor-not-allowed disabled:opacity-50"
                >
                  Download &amp; verify ({formatMb(build.sizeBytes)})
                </button>
              )}
            </div>
          )}

          {status?.state === "downloading" &&
            (() => {
              const total = status.totalBytes ?? 0;
              const pct = total > 0 ? Math.min(100, (status.receivedBytes / total) * 100) : null;
              return (
                <div className="flex flex-col gap-2">
                  <div className="flex items-baseline justify-between gap-2">
                    <span>Downloading…</span>
                    <span className="text-havoc-muted">
                      {pct !== null && (
                        <span className="font-mono text-havoc-text">{pct.toFixed(2)}%</span>
                      )}{" "}
                      · {formatMb(status.receivedBytes)}
                      {status.totalBytes ? ` of ${formatMb(status.totalBytes)}` : ""} ·{" "}
                      {formatRate(status.bytesPerSec)}
                    </span>
                  </div>
                  <div className="h-1.5 overflow-hidden rounded-full bg-white/10">
                    <div
                      className="h-full rounded-full bg-gradient-to-r from-havoc-accent to-havoc-accent-2 transition-[width]"
                      style={{ width: pct !== null ? `${pct.toFixed(2)}%` : "100%" }}
                    />
                  </div>
                  <button
                    type="button"
                    disabled={busy}
                    onClick={() => run(ffmpegCancel)}
                    className="self-start rounded-lg border border-white/10 bg-white/[0.04] px-3 py-1.5 text-havoc-muted transition-colors hover:text-havoc-text disabled:opacity-50"
                  >
                    Cancel
                  </button>
                </div>
              );
            })()}

          {status?.state === "verifying" && (
            <p className="m-0">Verifying the download against the pinned SHA-256…</p>
          )}
          {status?.state === "extracting" && <p className="m-0">Unpacking…</p>}

          {status?.state === "ready" && (
            <div className="flex flex-col gap-2">
              <p className="m-0">
                <span className="mr-1.5 inline-block h-2 w-2 rounded-full bg-emerald-400" />
                Installed &amp; verified —{" "}
                <span className="text-havoc-muted">{status.version}</span>
              </p>
              <p className="m-0 text-[10px] break-all text-havoc-muted">{status.path}</p>
              <button
                type="button"
                disabled={busy}
                onClick={() => run(ffmpegRemove)}
                className="self-start rounded-lg border border-white/10 bg-white/[0.04] px-3 py-1.5 text-havoc-muted transition-colors hover:text-havoc-text disabled:opacity-50"
              >
                Remove
              </button>
            </div>
          )}

          {status?.state === "error" && (
            <div className="flex flex-col gap-2">
              <p className="m-0 text-red-300">{status.message}</p>
              {build && (
                <button
                  type="button"
                  disabled={busy}
                  onClick={() => run(ffmpegInstall)}
                  className="self-start rounded-lg border border-havoc-accent/40 bg-gradient-to-r from-havoc-accent/20 to-havoc-accent-2/20 px-3 py-1.5 font-medium text-havoc-text transition-colors hover:border-havoc-accent/70 disabled:opacity-50"
                >
                  Retry download
                </button>
              )}
            </div>
          )}

          {actionError && <p className="mt-2 mb-0 text-red-300">{actionError}</p>}
        </section>

        <p className="m-0 text-[10px] leading-relaxed text-havoc-muted">
          The download is the only network action on this panel and never starts on its own. A
          failed checksum aborts the install — the app refuses to run bytes it cannot vouch for.
        </p>

        <CefSection />

        <IntegrationsSection />
      </div>
    </PickerShell>
  );
}

/**
 * The optional **CEF (browser-source runtime)** component — same on-demand,
 * verified, never-bundled model as ffmpeg. Resolves the newest stable build
 * from the CEF CDN, verifies its SHA-1, extracts it. The browser SOURCE that
 * renders through it is the follow-on milestone; this ships the download.
 */
function CefSection() {
  const [status, setStatus] = useState<CefStatus | null>(null);
  const [busy, setBusy] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);

  useEffect(() => {
    let alive = true;
    let unlisten: (() => void) | undefined;
    cefStatus()
      .then((s) => {
        if (alive) setStatus(s);
      })
      .catch(() => undefined);
    onCef((s) => {
      if (alive) setStatus(s);
    })
      .then((fn) => (alive ? (unlisten = fn) : fn()))
      .catch(() => undefined);
    return () => {
      alive = false;
      unlisten?.();
    };
  }, []);

  const run = (fn: () => Promise<void>) => {
    setBusy(true);
    setActionError(null);
    fn()
      .catch((err) => setActionError(String(err)))
      .finally(() => setBusy(false));
  };

  const dl =
    status?.state === "downloading"
      ? (() => {
          const total = status.totalBytes ?? 0;
          return total > 0 ? Math.min(100, (status.receivedBytes / total) * 100) : null;
        })()
      : null;

  return (
    <section className="rounded-lg border border-white/10 bg-white/[0.03] p-3">
      <div className="flex items-baseline justify-between gap-2">
        <h4 className="m-0 text-xs font-semibold">Browser Source runtime — Chromium (CEF)</h4>
        <span className="shrink-0 rounded bg-havoc-accent/15 px-1.5 py-0.5 text-[10px] font-medium tracking-wide text-havoc-accent uppercase">
          Third-party · not bundled
        </span>
      </div>
      <p className="mt-2 mb-0 leading-relaxed text-havoc-muted">
        Browser sources render web pages (alerts, widgets, overlays) through{" "}
        <span className="text-havoc-text">Chromium Embedded Framework</span> — a ~100&nbsp;MB
        runtime this app never ships with. It downloads on demand from the official CEF build index,
        is
        <span className="text-havoc-text">
          {" "}
          verified against that index&apos;s SHA-1 before anything is unpacked
        </span>
        , and is cached per-user. The browser <em>source</em> that renders through it arrives with
        its own milestone; this installs the runtime it needs.
      </p>

      <div className="mt-2">
        {status === null && <p className="m-0 text-havoc-muted">Checking…</p>}

        {status?.state === "missing" &&
          (status.supported ? (
            <button
              type="button"
              disabled={busy}
              onClick={() => run(cefInstall)}
              className="rounded-lg border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25 disabled:opacity-50"
            >
              Download &amp; install
            </button>
          ) : (
            <p className="m-0 text-havoc-muted">
              CEF publishes no build for this platform — browser sources are unavailable here.
            </p>
          ))}

        {status?.state === "resolving" && (
          <p className="m-0 text-havoc-muted">Resolving the latest stable build…</p>
        )}

        {status?.state === "downloading" && (
          <div className="flex flex-col gap-2">
            <div className="flex items-baseline justify-between gap-2">
              <span>Downloading…</span>
              <span className="text-havoc-muted">
                {dl !== null && <span className="font-mono text-havoc-text">{dl.toFixed(2)}%</span>}{" "}
                · {formatMb(status.receivedBytes)}
                {status.totalBytes ? ` of ${formatMb(status.totalBytes)}` : ""} ·{" "}
                {formatRate(status.bytesPerSec)}
              </span>
            </div>
            <div className="h-1.5 overflow-hidden rounded-full bg-white/10">
              <div
                className="h-full rounded-full bg-gradient-to-r from-havoc-accent to-havoc-accent-2 transition-[width]"
                style={{ width: dl !== null ? `${dl.toFixed(2)}%` : "100%" }}
              />
            </div>
            <button
              type="button"
              disabled={busy}
              onClick={() => run(cefCancel)}
              className="self-start rounded-lg border border-white/10 bg-white/[0.04] px-3 py-1.5 text-havoc-muted transition-colors hover:text-havoc-text disabled:opacity-50"
            >
              Cancel
            </button>
          </div>
        )}

        {status?.state === "verifying" && (
          <p className="m-0">Verifying the download against the index SHA-1…</p>
        )}
        {status?.state === "extracting" && <p className="m-0">Unpacking the runtime…</p>}

        {status?.state === "ready" && (
          <div className="flex flex-col gap-2">
            <p className="m-0 text-emerald-300">
              Installed — CEF <span className="font-mono">{status.version}</span>.
            </p>
            <button
              type="button"
              disabled={busy}
              onClick={() => run(cefRemove)}
              className="self-start rounded-lg border border-white/10 bg-white/[0.04] px-3 py-1.5 text-havoc-muted transition-colors hover:text-havoc-text disabled:opacity-50"
            >
              Remove
            </button>
          </div>
        )}

        {status?.state === "error" && (
          <div className="flex flex-col gap-2">
            <p className="m-0 text-red-300">{status.message}</p>
            {status.supported && (
              <button
                type="button"
                disabled={busy}
                onClick={() => run(cefInstall)}
                className="self-start rounded-lg border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25 disabled:opacity-50"
              >
                Retry
              </button>
            )}
          </div>
        )}

        {actionError && <p className="mt-2 mb-0 text-red-300">{actionError}</p>}
      </div>
    </section>
  );
}

/**
 * Optional integrations (TASK-804), read-only: **NDI** lights up when the free
 * Vizrt runtime is installed (never bundled); **VST** is scoped off for
 * licensing reasons, pointing at the built-in filters. Detection runs off the
 * UI thread; nothing here bundles or downloads anything.
 */
function IntegrationsSection() {
  const [status, setStatus] = useState<IntegrationsStatus | null>(null);

  useEffect(() => {
    let alive = true;
    integrationsStatus()
      .then((s) => {
        if (alive) setStatus(s);
      })
      .catch(() => undefined);
    return () => {
      alive = false;
    };
  }, []);

  return (
    <section className="rounded-lg border border-white/10 bg-white/[0.03] p-3">
      <div className="flex items-baseline justify-between gap-2">
        <h4 className="m-0 text-xs font-semibold">Optional integrations</h4>
        <span className="shrink-0 rounded bg-havoc-accent/15 px-1.5 py-0.5 text-[10px] font-medium tracking-wide text-havoc-accent uppercase">
          Never bundled
        </span>
      </div>
      {status === null ? (
        <p className="mt-2 mb-0 text-havoc-muted">Checking…</p>
      ) : (
        <div className="mt-2 flex flex-col gap-2">
          <div className="flex flex-col gap-0.5">
            <div className="flex items-baseline gap-2">
              <span className="font-semibold text-havoc-text">NDI</span>
              {status.ndiAvailable ? (
                <span className="rounded bg-emerald-400/15 px-1.5 py-0.5 text-[10px] font-medium text-emerald-300">
                  Detected{status.ndiVersion ? ` · ${status.ndiVersion}` : ""}
                </span>
              ) : (
                <span className="rounded bg-white/10 px-1.5 py-0.5 text-[10px] font-medium text-havoc-muted">
                  Not installed
                </span>
              )}
            </div>
            {!status.ndiAvailable && status.ndiGuidance && (
              <p className="m-0 leading-relaxed text-havoc-muted">{status.ndiGuidance}</p>
            )}
          </div>
          <div className="flex flex-col gap-0.5">
            <div className="flex items-baseline gap-2">
              <span className="font-semibold text-havoc-text">VST2/3</span>
              <span className="rounded bg-white/10 px-1.5 py-0.5 text-[10px] font-medium text-havoc-muted">
                {status.vstAvailable ? "Available" : "Not available"}
              </span>
            </div>
            <p className="m-0 leading-relaxed text-havoc-muted">{status.vstStatus}</p>
          </div>
        </div>
      )}
    </section>
  );
}
