import { useEffect, useMemo, useState } from "react";

import { hotkeyAudit, hotkeyCheatsheetSave } from "../api/commands";
import type { HotkeyAuditEntry } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const inputClass =
  "rounded-md border border-white/10 bg-havoc-panel px-2 py-1.5 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/** Machine action tags → i18n labels. Values are keys, resolved per render. */
const ACTION_LABELS: Record<string, string> = {
  record: "hotkey-audit-action-record",
  goLive: "hotkey-audit-action-go-live",
  transition: "hotkey-audit-action-transition",
  saveReplay: "hotkey-audit-action-save-replay",
  addMarker: "hotkey-audit-action-add-marker",
  still: "hotkey-audit-action-still",
  panic: "hotkey-audit-action-panic",
  timerToggle: "hotkey-audit-action-timer-toggle",
  timerReset: "hotkey-audit-action-timer-reset",
  splitTimerSplit: "hotkey-audit-action-split-split",
  splitTimerUndo: "hotkey-audit-action-split-undo",
  splitTimerSkip: "hotkey-audit-action-split-skip",
  splitTimerReset: "hotkey-audit-action-split-reset",
  playlistNext: "hotkey-audit-action-playlist-next",
  playlistPrevious: "hotkey-audit-action-playlist-previous",
  replayRoll: "hotkey-audit-action-replay-roll",
  pushToTalk: "hotkey-audit-action-ptt",
  pushToMute: "hotkey-audit-action-ptm",
};

/** Feature filter tags → i18n labels (see ACTION_LABELS). */
const FEATURE_LABELS: Record<string, string> = {
  recording: "hotkey-audit-feature-recording",
  streaming: "hotkey-audit-feature-streaming",
  studio: "hotkey-audit-feature-studio",
  replay: "hotkey-audit-feature-replay",
  markers: "hotkey-audit-feature-markers",
  stills: "hotkey-audit-feature-stills",
  panic: "hotkey-audit-feature-panic",
  timers: "hotkey-audit-feature-timers",
  splitTimer: "hotkey-audit-feature-split-timer",
  playlist: "hotkey-audit-feature-playlist",
  audio: "hotkey-audit-feature-audio",
};

/**
 * CAP-M14 — the hotkey audit: every binding (global actions + per-source
 * PTT/PTM) in one searchable table with honest conflict signals, plus an
 * exportable Markdown cheat sheet. SettingsHotkeys binds; this documents.
 */
export function HotkeyAuditDialog({ onClose }: { onClose: () => void }) {
  const t = useT();
  const [entries, setEntries] = useState<HotkeyAuditEntry[] | null>(null);
  const [search, setSearch] = useState("");
  const [feature, setFeature] = useState("all");
  const [exported, setExported] = useState<string | null>(null);
  const [fail, setFail] = useState<string | null>(null);

  useEffect(() => {
    let alive = true;
    hotkeyAudit()
      .then((all) => {
        if (alive) setEntries(all);
      })
      .catch((err) => {
        if (alive) setFail(String(err));
      });
    return () => {
      alive = false;
    };
  }, []);

  const actionLabel = (entry: HotkeyAuditEntry) =>
    ACTION_LABELS[entry.action] ? t(ACTION_LABELS[entry.action]) : entry.action;

  const features = useMemo(() => {
    const present = new Set((entries ?? []).map((entry) => entry.feature));
    return Object.keys(FEATURE_LABELS).filter((tag) => present.has(tag));
  }, [entries]);

  const rows = useMemo(() => {
    const needle = search.trim().toLowerCase();
    return (entries ?? []).filter((entry) => {
      if (feature !== "all" && entry.feature !== feature) return false;
      if (!needle) return true;
      return [entry.accelerator, actionLabel(entry), entry.source ?? ""]
        .join(" ")
        .toLowerCase()
        .includes(needle);
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [entries, search, feature, t]);

  const statusOf = (entry: HotkeyAuditEntry): { text: string; tone: string } => {
    if (!entry.valid) return { text: t("hotkey-audit-invalid"), tone: "text-red-400" };
    if (entry.sharedWith > 0)
      return {
        text: t("hotkey-audit-shared", { count: entry.sharedWith + 1 }),
        tone: "text-amber-300",
      };
    if (!entry.registered) return { text: t("hotkey-audit-unregistered"), tone: "text-amber-300" };
    return { text: t("hotkey-audit-ok"), tone: "text-emerald-300" };
  };

  const exportSheet = () => {
    setFail(null);
    const lines: string[] = [`# ${t("hotkey-audit-title")}`, ""];
    for (const tag of features) {
      const group = (entries ?? []).filter((entry) => entry.feature === tag);
      if (group.length === 0) continue;
      lines.push(`## ${t(FEATURE_LABELS[tag])}`, "");
      for (const entry of group) {
        const where = entry.source ? ` — ${entry.source}` : "";
        lines.push(`- \`${entry.accelerator}\` — ${actionLabel(entry)}${where}`);
      }
      lines.push("");
    }
    hotkeyCheatsheetSave(lines.join("\n"))
      .then(setExported)
      .catch((err) => setFail(String(err)));
  };

  return (
    <PickerShell title={t("hotkey-audit-title")} onClose={onClose} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <div className="flex items-end gap-2">
          <label className="flex min-w-0 flex-1 flex-col gap-1 text-[11px] text-havoc-muted">
            {t("hotkey-audit-search")}
            <input
              value={search}
              onChange={(event) => setSearch(event.target.value)}
              className={inputClass}
            />
          </label>
          <label className="flex flex-col gap-1 text-[11px] text-havoc-muted">
            {t("hotkey-audit-filter")}
            <select
              value={feature}
              onChange={(event) => setFeature(event.target.value)}
              className={inputClass}
            >
              <option value="all">{t("hotkey-audit-filter-all")}</option>
              {features.map((tag) => (
                <option key={tag} value={tag}>
                  {t(FEATURE_LABELS[tag])}
                </option>
              ))}
            </select>
          </label>
          <button
            type="button"
            disabled={!entries || entries.length === 0}
            onClick={exportSheet}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
          >
            {t("hotkey-audit-export")}
          </button>
        </div>
        {rows.length === 0 ? (
          <p className="m-0 text-havoc-muted">{t("hotkey-audit-empty")}</p>
        ) : (
          <table className="w-full border-collapse text-left">
            <thead>
              <tr className="text-[10px] uppercase tracking-wide text-havoc-muted">
                <th className="py-1 pr-3 font-semibold">{t("hotkey-audit-col-key")}</th>
                <th className="py-1 pr-3 font-semibold">{t("hotkey-audit-col-action")}</th>
                <th className="py-1 pr-3 font-semibold">{t("hotkey-audit-col-where")}</th>
                <th className="py-1 font-semibold">{t("hotkey-audit-col-status")}</th>
              </tr>
            </thead>
            <tbody>
              {rows.map((entry, index) => {
                const status = statusOf(entry);
                return (
                  <tr
                    key={`${entry.action}-${entry.source ?? ""}-${index}`}
                    className="border-t border-white/5"
                  >
                    <td className="py-1.5 pr-3 font-mono">{entry.accelerator}</td>
                    <td className="py-1.5 pr-3">{actionLabel(entry)}</td>
                    <td className="py-1.5 pr-3 text-havoc-muted">
                      {entry.source ?? t(FEATURE_LABELS[entry.feature] ?? "")}
                    </td>
                    <td className={`py-1.5 ${status.tone}`}>{status.text}</td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        )}
        <p className="m-0 text-[10px] leading-snug text-havoc-muted">{t("hotkey-audit-note")}</p>
        {exported && (
          <p className="m-0 text-emerald-300">{t("hotkey-audit-exported", { path: exported })}</p>
        )}
        {fail && (
          <p role="alert" className="m-0 text-red-400">
            {fail}
          </p>
        )}
      </div>
    </PickerShell>
  );
}
