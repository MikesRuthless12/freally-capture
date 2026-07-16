import { useEffect, useState } from "react";

import { audioClapPlugins } from "../api/commands";
import { PickerShell } from "./PickerShell";
import { useT } from "../i18n/t";

type Plugin = { path: string; name: string; format: string };

/**
 * CAP-N33 audio plugins: discovery of the user's installed CLAP + VST3 plugins
 * (both MIT-licensed, $0-clean). Live hosting runs each in a crash-isolated
 * process with its own GUI — that integration is in progress, so this panel is
 * the honest boundary: it lists what's installed and states the plan plainly.
 */
export function PluginsDialog({ onClose }: { onClose: () => void }) {
  const t = useT();
  const [plugins, setPlugins] = useState<Plugin[] | null>(null);
  const [status, setStatus] = useState("");
  const [nonce, setNonce] = useState(0);

  // Re-scan on mount and whenever the refresh button bumps `nonce`. The fetch
  // is async, so state only updates on resolution (no sync setState in-effect).
  useEffect(() => {
    let alive = true;
    audioClapPlugins()
      .then((result) => {
        if (!alive) return;
        setPlugins(result.plugins);
        setStatus(result.status);
      })
      .catch(() => alive && setPlugins([]));
    return () => {
      alive = false;
    };
  }, [nonce]);

  const scan = () => {
    setPlugins(null);
    setNonce((n) => n + 1);
  };

  const badge = (format: string) =>
    format === "vst3" ? "VST3" : format === "clap" ? "CLAP" : format.toUpperCase();

  return (
    <PickerShell title={t("plugins-title")} onClose={onClose} onRefresh={scan} wide>
      <div className="flex max-h-[70vh] flex-col gap-3 overflow-y-auto text-xs text-havoc-text">
        {status && <p className="m-0 text-[11px] leading-snug text-havoc-muted">{status}</p>}
        {plugins === null && <p className="m-0 text-havoc-muted">{t("plugins-scanning")}</p>}
        {plugins?.length === 0 && <p className="m-0 text-havoc-muted">{t("plugins-none")}</p>}
        {plugins && plugins.length > 0 && (
          <ul className="m-0 flex list-none flex-col gap-1 p-0">
            {plugins.map((plugin) => (
              <li
                key={`${plugin.format}:${plugin.path}`}
                className="flex items-center gap-2 rounded-md border border-white/10 bg-white/[0.02] px-2 py-1.5"
                title={plugin.path}
              >
                <span
                  className={`shrink-0 rounded px-1.5 py-0.5 text-[9px] font-semibold ${
                    plugin.format === "vst3"
                      ? "bg-havoc-accent-2/20 text-havoc-accent-2"
                      : "bg-havoc-accent/20 text-havoc-accent"
                  }`}
                >
                  {badge(plugin.format)}
                </span>
                <span className="min-w-0 flex-1 truncate">{plugin.name}</span>
              </li>
            ))}
          </ul>
        )}
      </div>
    </PickerShell>
  );
}
