import { useEffect, useMemo, useState } from "react";

import {
  bugReportClearCrash,
  bugReportContext,
  bugReportSubmit,
  diagnosticsExport,
  diagnosticsPreview,
} from "../api/commands";
import type { BugReportContext } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

/**
 * Report a bug — opt-in and anonymous (charter: no telemetry, nothing
 * auto-sends). Shows the user the EXACT anonymous report (app/OS + a scrubbed
 * crash from the last run, if any), then lets them submit it via a pre-filled
 * GitHub issue or their email client. The subject is `[Freally Capture] <what
 * went wrong>` so a report is instantly attributable. No server, no shipped
 * credentials — the user still clicks send.
 */
export function BugReportDialog({ onClose }: { onClose: () => void }) {
  const t = useT();
  const [ctx, setCtx] = useState<BugReportContext | null>(null);
  const [description, setDescription] = useState("");
  const [includeCrash, setIncludeCrash] = useState(true);
  const [copied, setCopied] = useState(false);
  const [error, setError] = useState<string | null>(null);
  // The diagnostics bundle (CAP-M24): show the EXACT content before export.
  const [bundlePreview, setBundlePreview] = useState<string | null>(null);
  const [exportedPath, setExportedPath] = useState<string | null>(null);

  const load = () => {
    bugReportContext()
      .then(setCtx)
      .catch((err) => setError(String(err)));
  };

  useEffect(load, []);

  // Mirrors `compose_body(.., BodyStyle::Plain)` in `bugreport.rs`. The GitHub
  // target sends the same content as Markdown (`###` headings, fenced
  // diagnostics); only the syntax differs, never the information.
  const preview = useMemo(() => {
    if (!ctx) return "";
    const parts = [
      t("bugreport-preview-what-happened"),
      description.trim() || t("bugreport-preview-no-description"),
      "",
      t("bugreport-preview-diagnostics"),
      t("bugreport-preview-from"),
      ctx.diagnostics.trimEnd(),
    ];
    if (includeCrash && ctx.pendingCrash) {
      parts.push("", t("bugreport-preview-crash-excerpt"), ctx.pendingCrash.trimEnd());
    }
    return parts.join("\n");
  }, [ctx, description, includeCrash, t]);

  const submit = (target: "github" | "gmail" | "email") => {
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
      .catch(() => setError(t("bugreport-copy-failed")));
  };

  const dismissCrash = () => {
    bugReportClearCrash()
      .then(load)
      .catch((err) => setError(String(err)));
  };

  return (
    <PickerShell title={t("bugreport-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-[11px] leading-snug text-havoc-muted">{t("bugreport-intro")}</p>

        {ctx?.pendingCrash && (
          <div className="rounded-lg border border-amber-400/40 bg-amber-400/[0.07] px-2.5 py-2">
            <p className="m-0 text-[11px] text-amber-200">{t("bugreport-crash-notice")}</p>
          </div>
        )}

        <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
          {t("bugreport-description-label")}
          <textarea
            value={description}
            onChange={(event) => setDescription(event.target.value)}
            rows={3}
            placeholder={t("bugreport-description-placeholder")}
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
            {t("bugreport-include-crash")}
          </label>
        )}

        <div className="flex flex-col gap-1">
          <span className="text-[10px] tracking-wide text-havoc-muted uppercase">
            {t("bugreport-preview-label")}
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
            {t("bugreport-open-github")}
          </button>
          <button
            type="button"
            onClick={() => submit("gmail")}
            title={t("bugreport-gmail-title")}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            {t("bugreport-compose-gmail")}
          </button>
          <button
            type="button"
            onClick={() => submit("email")}
            title={t("bugreport-email-title")}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            {t("bugreport-send-email")}
          </button>
          <button
            type="button"
            onClick={copy}
            className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
          >
            {copied ? t("bugreport-copied") : t("bugreport-copy-report")}
          </button>
          {ctx?.pendingCrash && (
            <button
              type="button"
              onClick={dismissCrash}
              className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:border-red-400/50 hover:text-red-300"
            >
              {t("bugreport-dismiss-crash")}
            </button>
          )}
        </div>

        {/* CAP-M24 — the redacted diagnostics bundle. Strictly manual: the
            zip lands in Downloads for the user to attach by hand. */}
        <div className="flex flex-col gap-2 border-t border-white/10 pt-2">
          <span className="text-[10px] tracking-wide text-havoc-muted uppercase">
            {t("diag-title")}
          </span>
          <p className="m-0 text-[11px] leading-snug text-havoc-muted">{t("diag-intro")}</p>
          <div className="flex flex-wrap gap-2">
            <button
              type="button"
              onClick={() =>
                bundlePreview !== null
                  ? setBundlePreview(null)
                  : diagnosticsPreview()
                      .then(setBundlePreview)
                      .catch((err) => setError(String(err)))
              }
              className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
            >
              {bundlePreview !== null ? t("diag-hide-preview") : t("diag-preview")}
            </button>
            <button
              type="button"
              onClick={() =>
                diagnosticsExport()
                  .then(setExportedPath)
                  .catch((err) => setError(String(err)))
              }
              className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
            >
              {t("diag-export")}
            </button>
          </div>
          {bundlePreview !== null && (
            <pre className="m-0 max-h-48 overflow-auto rounded-md border border-white/10 bg-black/30 px-2 py-1.5 font-mono text-[10px] leading-snug break-words whitespace-pre-wrap text-havoc-muted">
              {bundlePreview}
            </pre>
          )}
          {exportedPath && (
            <p className="m-0 text-[11px] text-emerald-300">
              {t("diag-exported", { path: exportedPath })}
            </p>
          )}
        </div>

        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}
      </div>
    </PickerShell>
  );
}
