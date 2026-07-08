import { useEffect, useMemo, useState } from "react";

import {
  bugReportClearCrash,
  bugReportContext,
  bugReportSimulate,
  bugReportSubmit,
} from "../api/commands";
import type { BugReportContext } from "../api/types";
import { PickerShell } from "../components/PickerShell";

/**
 * Report a bug — opt-in and anonymous (charter: no telemetry, nothing
 * auto-sends). Shows the user the EXACT anonymous report (app/OS + a scrubbed
 * crash from the last run, if any), then lets them submit it via a pre-filled
 * GitHub issue or their email client. The subject is `[Freally Capture] <what
 * went wrong>` so a report is instantly attributable. No server, no shipped
 * credentials — the user still clicks send.
 */
export function BugReportDialog({ onClose }: { onClose: () => void }) {
  const [ctx, setCtx] = useState<BugReportContext | null>(null);
  const [description, setDescription] = useState("");
  const [includeCrash, setIncludeCrash] = useState(true);
  const [copied, setCopied] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = () => {
    bugReportContext()
      .then(setCtx)
      .catch((err) => setError(String(err)));
  };

  useEffect(load, []);

  const preview = useMemo(() => {
    if (!ctx) return "";
    const parts = [
      "### What happened",
      description.trim() || "(no description provided)",
      "",
      "### Anonymous diagnostics (no personal data)",
      "```",
      `From: Freally Capture`,
      ctx.diagnostics.trimEnd(),
    ];
    if (includeCrash && ctx.pendingCrash) {
      parts.push("", "--- crash excerpt ---", ctx.pendingCrash.trimEnd());
    }
    parts.push("```");
    return parts.join("\n");
  }, [ctx, description, includeCrash]);

  const submit = (target: "github" | "email") => {
    setError(null);
    bugReportSubmit(target, description, includeCrash && !!ctx?.pendingCrash).catch((err) =>
      setError(String(err)),
    );
  };

  const copy = () => {
    navigator.clipboard
      .writeText(preview)
      .then(() => {
        setCopied(true);
        window.setTimeout(() => setCopied(false), 1500);
      })
      .catch(() => setError("couldn't copy — select the text and copy manually"));
  };

  const dismissCrash = () => {
    bugReportClearCrash()
      .then(load)
      .catch((err) => setError(String(err)));
  };

  return (
    <PickerShell title="Report a bug" onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-[11px] leading-snug text-havoc-muted">
          Reports are <strong>anonymous</strong> and <strong>opt-in</strong> — nothing is sent
          automatically. You&apos;ll review the exact text below, then submit it via a pre-filled
          GitHub issue or your email app. No personal data (your home path and username are
          redacted); no account, no server.
        </p>

        {ctx?.pendingCrash && (
          <div className="rounded-lg border border-amber-400/40 bg-amber-400/[0.07] px-2.5 py-2">
            <p className="m-0 text-[11px] text-amber-200">
              Freally Capture closed unexpectedly on a previous run — the anonymous crash details
              are included below. Reporting them helps fix it fast.
            </p>
          </div>
        )}

        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          What were you doing when it happened? (optional)
          <textarea
            value={description}
            onChange={(event) => setDescription(event.target.value)}
            rows={3}
            placeholder="e.g. the preview froze when I added a second webcam"
            className="rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60"
          />
        </label>

        {ctx?.pendingCrash && (
          <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
            <input
              type="checkbox"
              checked={includeCrash}
              onChange={(event) => setIncludeCrash(event.target.checked)}
            />
            Include the anonymous crash details from the last run
          </label>
        )}

        <div className="flex flex-col gap-1">
          <span className="text-[10px] tracking-wide text-havoc-muted uppercase">
            Exactly what will be sent
          </span>
          <pre className="m-0 max-h-48 overflow-auto rounded-md border border-white/10 bg-black/30 px-2 py-1.5 font-mono text-[10px] leading-snug break-words whitespace-pre-wrap text-havoc-muted">
            {preview}
          </pre>
        </div>

        <div className="flex flex-wrap gap-2">
          <button
            type="button"
            onClick={() => submit("github")}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            Open GitHub issue
          </button>
          <button
            type="button"
            onClick={() => submit("email")}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            Send email
          </button>
          <button
            type="button"
            onClick={copy}
            className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
          >
            {copied ? "Copied ✓" : "Copy report"}
          </button>
          {ctx?.pendingCrash && (
            <button
              type="button"
              onClick={dismissCrash}
              className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:border-red-400/50 hover:text-red-300"
            >
              Dismiss crash
            </button>
          )}
        </div>

        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}

        <button
          type="button"
          onClick={() =>
            bugReportSimulate()
              .then(load)
              .catch(() => undefined)
          }
          className="self-start text-[10px] text-havoc-muted underline decoration-dotted hover:text-havoc-text"
          title="Writes a harmless sample crash report so you can test the crash-report flow without crashing the app"
        >
          Simulate a crash report (for testing)
        </button>
      </div>
    </PickerShell>
  );
}
