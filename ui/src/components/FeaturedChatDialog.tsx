import { useEffect, useState } from "react";

import { chatFeature, chatFeatured, chatRecent, settingsSet } from "../api/commands";
import type { ChatLine, Settings } from "../api/types";
import { PickerShell } from "./PickerShell";
import { useT } from "../i18n/t";

const fail = (what: string) => (err: unknown) => console.error(`${what} failed:`, err);

/** How often the picker refreshes while open (the feed is chat-rate). */
const REFRESH_MS = 2000;

const TAGS: Record<string, string> = {
  youtube: "YT",
  twitch: "TW",
  kick: "KI",
};

/**
 * V1-E: the featured chat message picker — pin any recent chat line to the
 * program as a styled bottom banner, and style it. The pin is deliberately
 * transient (it clears on restart); only the colors persist in Settings.
 */
export function FeaturedChatDialog({
  settings,
  onSettingsSaved,
  onClose,
}: {
  settings: Settings;
  onSettingsSaved: (next: Settings) => void;
  onClose: () => void;
}) {
  const t = useT();
  const [lines, setLines] = useState<ChatLine[]>([]);
  const [pinned, setPinned] = useState<ChatLine | null>(null);

  // The feed + the active pin, refreshed on a slow tick while open.
  useEffect(() => {
    let alive = true;
    const refresh = () => {
      chatRecent(50)
        .then((next) => alive && setLines(next))
        .catch(() => undefined);
      chatFeatured()
        .then((next) => alive && setPinned(next))
        .catch(() => undefined);
    };
    refresh();
    const timer = window.setInterval(refresh, REFRESH_MS);
    return () => {
      alive = false;
      window.clearInterval(timer);
    };
  }, []);

  const pin = (line: ChatLine | null) => {
    chatFeature(line)
      .then(() => setPinned(line))
      .catch(fail("featured-chat pin"));
  };

  const banner = settings.featuredBanner ?? { bg: "#101a2a", text: "#ffffff" };
  const saveColor = (patch: Partial<Settings["featuredBanner"]>) => {
    const next: Settings = { ...settings, featuredBanner: { ...banner, ...patch } };
    settingsSet(next)
      .then(() => {
        onSettingsSaved(next);
        // The studio re-renders the banner only when the pin changes — re-pin
        // the same message so an on-air banner repaints in the new colors.
        if (pinned) return chatFeature(pinned);
      })
      .catch(fail("featured-banner colors save"));
  };

  const tag = (platform: string) => TAGS[platform] ?? "—";
  const when = (atUnixMs: number) => new Date(atUnixMs).toLocaleTimeString();

  return (
    <PickerShell title={t("featured-chat-title")} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-havoc-muted">{t("featured-chat-intro")}</p>

        {pinned && (
          <div
            className="flex items-center justify-between gap-2 rounded-lg border border-havoc-accent/50 bg-havoc-accent/10 px-2.5 py-2"
            role="status"
          >
            <span className="min-w-0 truncate">
              <span className="mr-1.5 font-bold text-havoc-accent">
                {t("featured-chat-pinned")}
              </span>
              <strong>{pinned.username}</strong>: {pinned.text}
            </span>
            <button
              type="button"
              onClick={() => pin(null)}
              className="shrink-0 rounded-md border border-white/10 px-2 py-1 text-xs text-havoc-muted transition-colors hover:border-red-400/60 hover:text-red-300"
            >
              {t("featured-chat-clear")}
            </button>
          </div>
        )}

        {lines.length === 0 ? (
          <p className="m-0 rounded-lg border border-white/10 bg-white/[0.02] px-3 py-4 text-center text-havoc-muted">
            {t("featured-chat-empty")}
          </p>
        ) : (
          <ul className="m-0 flex max-h-72 list-none flex-col gap-1 overflow-y-auto p-0">
            {[...lines].reverse().map((line, index) => (
              <li
                key={`${line.atUnixMs}-${index}`}
                className="flex items-center gap-2 rounded-lg border border-white/10 bg-white/[0.02] px-2.5 py-1.5"
              >
                <span className="shrink-0 rounded border border-white/15 px-1 text-[9px] font-bold tracking-wider text-havoc-muted">
                  {tag(line.platform)}
                </span>
                <span className="min-w-0 flex-1 truncate">
                  <strong>{line.username}</strong>: {line.text}
                </span>
                <span className="shrink-0 text-[10px] tabular-nums text-havoc-muted">
                  {when(line.atUnixMs)}
                </span>
                <button
                  type="button"
                  onClick={() => pin(line)}
                  className="shrink-0 rounded-md border border-havoc-accent/40 px-2 py-0.5 text-xs text-havoc-accent transition-colors hover:border-havoc-accent/80"
                >
                  {t("featured-chat-pin")}
                </button>
              </li>
            ))}
          </ul>
        )}

        <div className="flex items-center gap-4">
          <label className="flex items-center gap-2">
            {t("featured-chat-bg")}
            <input
              type="color"
              value={banner.bg}
              onChange={(event) => saveColor({ bg: event.target.value })}
              className="h-6 w-10 cursor-pointer rounded border border-white/10 bg-transparent"
            />
          </label>
          <label className="flex items-center gap-2">
            {t("featured-chat-text")}
            <input
              type="color"
              value={banner.text}
              onChange={(event) => saveColor({ text: event.target.value })}
              className="h-6 w-10 cursor-pointer rounded border border-white/10 bg-transparent"
            />
          </label>
        </div>

        <p className="m-0 text-[10px] text-havoc-muted">{t("featured-chat-note")}</p>
      </div>
    </PickerShell>
  );
}
