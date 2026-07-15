import { useEffect, useState } from "react";

import { releaseNotes } from "../api/commands";
import type { ReleaseNotes } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

/**
 * Help → What's New: the running build's changelog section, shown in-app — the
 * same read-only release-notes view the updater uses, but for the version you
 * already have (no browser trip). The notes are embedded at build time
 * (`release_notes` reads `CHANGELOG.md`), so this is offline and always matches
 * exactly what shipped.
 */
export function WhatsNewDialog({ onClose }: { onClose: () => void }) {
  const t = useT();
  const [notes, setNotes] = useState<ReleaseNotes | null>(null);

  useEffect(() => {
    let alive = true;
    releaseNotes()
      .then((result) => {
        if (alive) setNotes(result);
      })
      .catch(() => {
        if (alive) setNotes({ version: "", notes: null });
      });
    return () => {
      alive = false;
    };
  }, []);

  return (
    <PickerShell title={t("whats-new-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        {notes === null ? (
          <p className="m-0 text-havoc-muted">{t("whats-new-loading")}</p>
        ) : notes.notes ? (
          <>
            <p className="m-0">{t("whats-new-version", { version: notes.version })}</p>
            <textarea
              readOnly
              value={notes.notes}
              rows={16}
              className="m-0 resize-none rounded-md border border-white/10 bg-black/30 px-2 py-1.5 font-mono text-[10px] leading-snug text-havoc-muted outline-none focus:border-havoc-accent/60"
            />
          </>
        ) : (
          <p className="m-0 text-havoc-muted">{t("whats-new-empty")}</p>
        )}
      </div>
    </PickerShell>
  );
}
