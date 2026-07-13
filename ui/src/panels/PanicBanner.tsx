import { useState } from "react";

import { studioPanicSet } from "../api/commands";
import { useT } from "../i18n/t";

/**
 * The engaged-panic banner (CAP-M22): program shows the privacy slate,
 * everything is hard-muted, captures are stopped. Restoring is deliberately
 * two-step — one click arms the confirm, a second click restores — so a
 * stray click can never un-panic a live show.
 */
export function PanicBanner() {
  const t = useT();
  const [confirming, setConfirming] = useState(false);

  const restore = () => {
    studioPanicSet(false).catch((err) => console.error("panic restore failed:", err));
    setConfirming(false);
  };

  return (
    <div
      role="alert"
      className="pointer-events-auto fixed top-3 left-1/2 z-50 flex -translate-x-1/2 items-center gap-3 rounded-md border border-red-500/60 bg-red-950/90 px-4 py-2 text-xs text-red-100"
    >
      <span className="font-bold tracking-wide uppercase">{t("panic-banner-title")}</span>
      <span className="text-red-200/90">{t("panic-banner-body")}</span>
      {confirming ? (
        <span className="flex items-center gap-2">
          <span>{t("panic-restore-confirm")}</span>
          <button
            type="button"
            onClick={restore}
            className="rounded-md border border-red-300/60 bg-red-500/20 px-2.5 py-1 font-semibold transition-colors hover:bg-red-500/35"
          >
            {t("panic-restore-yes")}
          </button>
          <button
            type="button"
            onClick={() => setConfirming(false)}
            className="rounded-md border border-red-300/30 px-2.5 py-1 transition-colors hover:border-red-300/60"
          >
            {t("panic-restore-cancel")}
          </button>
        </span>
      ) : (
        <button
          type="button"
          onClick={() => setConfirming(true)}
          className="rounded-md border border-red-300/60 px-2.5 py-1 font-semibold transition-colors hover:bg-red-500/20"
        >
          {t("panic-restore")}
        </button>
      )}
    </div>
  );
}
