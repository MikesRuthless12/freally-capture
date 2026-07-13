import { useState } from "react";

import { quitConfirmed, quitGuardCancel } from "../api/commands";
import type { QuitConsequences } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

/**
 * The quit guard (CAP-M23): closing the window while live output runs shows
 * exactly what quitting will do — end the stream, finalize the recording,
 * flush the replay buffer — before the ordered shutdown. Cancel keeps the
 * show running.
 */
export function QuitGuardDialog({
  pending,
  onClose,
}: {
  pending: QuitConsequences;
  onClose: () => void;
}) {
  const t = useT();
  const [quitting, setQuitting] = useState(false);

  const quit = () => {
    setQuitting(true);
    quitConfirmed().catch((err) => {
      console.error("quit failed:", err);
      setQuitting(false);
    });
  };

  const cancel = () => {
    // Disarm the backend prompt so the NEXT close asks again.
    quitGuardCancel().catch(() => undefined);
    onClose();
  };

  const consequences = [
    pending.streaming && t("quit-consequence-stream"),
    pending.recording && t("quit-consequence-recording"),
    pending.replay && t("quit-consequence-replay"),
  ].filter((entry): entry is string => Boolean(entry));

  return (
    <PickerShell title={t("quit-title")} onClose={cancel}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-havoc-muted">{t("quit-body")}</p>
        <ol className="m-0 flex list-decimal flex-col gap-1 pl-5">
          {consequences.map((line) => (
            <li key={line}>{line}</li>
          ))}
        </ol>
        <div className="flex justify-end gap-2">
          <button
            type="button"
            onClick={cancel}
            disabled={quitting}
            className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-text transition-colors hover:border-havoc-accent/60 disabled:opacity-50"
          >
            {t("quit-cancel")}
          </button>
          <button
            type="button"
            onClick={quit}
            disabled={quitting}
            className="rounded-md border border-red-500/50 bg-red-500/10 px-3 py-1.5 text-xs font-semibold text-red-300 transition-colors hover:border-red-400 disabled:opacity-50"
          >
            {quitting ? t("quit-quitting") : t("quit-confirm")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}
