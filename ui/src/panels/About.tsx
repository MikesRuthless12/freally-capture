import { useEffect, useState } from "react";

import { buildInfo, openExternal } from "../api/commands";
import type { BuildInfo } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

/**
 * About (TASK-907). Everything derivable from the build comes from the build —
 * version, authors, repository — so this panel can never claim a version the
 * binary isn't. The two dates are consts in `src-tauri/src/buildinfo.rs`.
 *
 * Links open with the OS handler; nothing is fetched here.
 */
export function AboutDialog({ onClose, onCheckUpdates }: AboutDialogProps) {
  const t = useT();
  const [info, setInfo] = useState<BuildInfo | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let alive = true;
    buildInfo()
      .then((next) => {
        if (alive) setInfo(next);
      })
      .catch((err) => {
        if (alive) setError(String(err));
      });
    return () => {
      alive = false;
    };
  }, []);

  return (
    <PickerShell title={t("about-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <div className="flex flex-col gap-0.5">
          <span className="bg-gradient-to-r from-havoc-accent to-havoc-accent-2 bg-clip-text text-base font-bold tracking-wide text-transparent">
            Freally Capture
          </span>
          <span className="text-[11px] text-havoc-muted">{t("about-tagline")}</span>
        </div>

        {error && (
          <p role="alert" className="m-0 text-[11px] text-red-300">
            {error}
          </p>
        )}

        {info && (
          <dl className="m-0 grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-[11px]">
            <Row label={t("about-version")} value={info.version} />
            <Row label={t("about-created-by")} value={info.authors} />
            <Row label={t("about-project-started")} value={info.projectStarted} />
            <Row
              label={t("about-first-stable")}
              value={info.firstStableReleased ?? t("about-first-stable-pending")}
            />
            <Row label={t("about-platform")} value={`${info.os} / ${info.arch}`} />
          </dl>
        )}

        <p className="m-0 text-[11px] leading-snug text-havoc-muted">{t("about-local-first")}</p>

        {info && (
          <div className="flex flex-wrap gap-2">
            <Link href={info.homepage}>{t("about-website")}</Link>
            <Link href={info.issues}>{t("about-issues")}</Link>
            <Link href={`${info.repository}/blob/main/LICENSE`}>{t("about-license")}</Link>
            <Link href={`${info.repository}/blob/main/EULA.md`}>{t("about-eula")}</Link>
            <Link href={`${info.repository}/blob/main/THIRD-PARTY-NOTICES.md`}>
              {t("about-third-party")}
            </Link>
          </div>
        )}

        <div className="flex flex-wrap items-center gap-2 border-t border-white/5 pt-2">
          <button
            type="button"
            onClick={onCheckUpdates}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
          >
            {t("about-check-updates")}
          </button>
          {info && <span className="text-[10px] text-havoc-muted">{info.copyright}</span>}
        </div>
      </div>
    </PickerShell>
  );
}

type AboutDialogProps = {
  onClose: () => void;
  /** Hands off to the existing `UpdatesDialog` rather than duplicating it. */
  onCheckUpdates: () => void;
};

function Row({ label, value }: { label: string; value: string }) {
  return (
    <>
      <dt className="text-havoc-muted">{label}</dt>
      <dd className="m-0 font-mono text-havoc-text">{value}</dd>
    </>
  );
}

/**
 * A `<a target="_blank">` never reaches the OS browser from this Tauri webview
 * (no opener plugin; external navigation is blocked), so hand the URL to the
 * OS opener command instead — the same path the Help menu links use.
 */
function Link({ href, children }: { href: string; children: React.ReactNode }) {
  return (
    <button
      type="button"
      onClick={() => void openExternal(href)}
      className="rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
    >
      {children}
    </button>
  );
}
