import { CentralPanel, type PanelHost } from "@freally/central-panel";

import { openExternal } from "../api/commands";
import { PickerShell } from "../components/PickerShell";
import { useLocale, useT } from "../i18n/t";

/**
 * "More Freally apps" — the Central-inside panel (Central's FC-50), vendored
 * from the freally-central submodule. The grid, live release data, install
 * detection, and the whole verified download → silent-install flow are the
 * exact code Freally Central ships; this dialog only provides our shell:
 * i18n (the fcp-* catalogs ride in via bundle.ts), the external-link opener,
 * and the PickerShell chrome. No reveal-in-folder here — the panel hides that
 * action when the host doesn't provide it.
 */
const HOST: PanelHost = { openExternal };

export function MoreAppsDialog({ onClose }: { onClose: () => void }) {
  const t = useT();
  const locale = useLocale();
  return (
    <PickerShell title={t("moreapps-title")} onClose={onClose} sidebar>
      <div className="min-h-0 flex-1 overflow-auto p-4">
        <CentralPanel t={t} locale={locale} host={HOST} />
      </div>
    </PickerShell>
  );
}
