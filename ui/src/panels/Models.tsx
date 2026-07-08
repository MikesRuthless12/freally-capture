import { useEffect, useState } from "react";

import {
  ffmpegCancel,
  ffmpegInstall,
  ffmpegRemove,
  ffmpegStatus,
  integrationsStatus,
} from "../api/commands";
import { onFfmpeg } from "../api/events";
import type { FfmpegStatus, IntegrationsStatus } from "../api/types";
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

          {status?.state === "downloading" && (
            <div className="flex flex-col gap-2">
              <div className="flex items-baseline justify-between">
                <span>Downloading…</span>
                <span className="text-havoc-muted">
                  {formatMb(status.receivedBytes)}
                  {status.totalBytes ? ` of ${formatMb(status.totalBytes)}` : ""} ·{" "}
                  {formatRate(status.bytesPerSec)}
                </span>
              </div>
              <div className="h-1.5 overflow-hidden rounded-full bg-white/10">
                <div
                  className="h-full rounded-full bg-gradient-to-r from-havoc-accent to-havoc-accent-2 transition-[width]"
                  style={{
                    width: status.totalBytes
                      ? `${Math.min(100, (status.receivedBytes / status.totalBytes) * 100).toFixed(1)}%`
                      : "100%",
                  }}
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
          )}

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

        <IntegrationsSection />
      </div>
    </PickerShell>
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
