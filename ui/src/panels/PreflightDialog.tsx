import { useEffect, useMemo, useState } from "react";

import {
  encodersList,
  preflightDisk,
  replayArm,
  settingsGet,
  settingsSet,
  studioGet,
} from "../api/commands";
import { onAudio, onProgram, onReplay } from "../api/events";
import type {
  AudioLevelsPayload,
  EncoderCatalog,
  ProgramStatus,
  ReplayStatus,
  Settings,
  StudioDto,
} from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

type ItemTone = "green" | "red" | "warn";

type Item = {
  key: string;
  tone: ItemTone;
  /** Blocking items decide "all green"; the rest are honest nudges. */
  blocking: boolean;
  detail?: string;
  fix?: { label: string; run: () => void };
};

/** A linear peak that counts as "metering" (≈ −60 dBFS). */
const METERING_PEAK = 0.001;
/** The disk forecast (minutes) below which the item goes red — the CAP-M10
 * clear threshold, so pre-flight and the live alarm agree. */
const DISK_FLOOR_MIN = 12;

/**
 * The go-live pre-flight checklist (CAP-M09): stream key, encoder probe,
 * source health, disk forecast as blocking items; mic/desktop-audio metering
 * and the replay buffer as honest non-blocking nudges (a screen-only show
 * without a mic must never be held hostage). "Hold Go Live until all green"
 * removes the "Go Live anyway" escape hatch.
 */
export function PreflightDialog({
  onProceed,
  onClose,
  onOpenStreamSettings,
  onOpenComponents,
  onOpenSourceHealth,
  onSettingsSaved,
}: {
  onProceed: () => void;
  onClose: () => void;
  onOpenStreamSettings: () => void;
  onOpenComponents: () => void;
  onOpenSourceHealth: () => void;
  /** Keeps App's settings copy current — another panel's save would
   * otherwise write a stale preflightHold back. */
  onSettingsSaved: (next: Settings) => void;
}) {
  const t = useT();
  const [settings, setSettings] = useState<Settings | null>(null);
  const [studio, setStudio] = useState<StudioDto | null>(null);
  const [catalog, setCatalog] = useState<EncoderCatalog | null>(null);
  const [diskMinutes, setDiskMinutes] = useState<number | null | "loading">("loading");
  const [program, setProgram] = useState<ProgramStatus | null>(null);
  const [audio, setAudio] = useState<AudioLevelsPayload | null>(null);
  const [replay, setReplay] = useState<ReplayStatus | null>(null);

  useEffect(() => {
    let alive = true;
    const cleanups: Array<() => void> = [];
    settingsGet()
      .then((loaded) => alive && setSettings(loaded))
      .catch(() => undefined);
    studioGet()
      .then((dto) => alive && setStudio(dto))
      .catch(() => undefined);
    encodersList()
      .then((found) => alive && setCatalog(found))
      .catch(() => alive && setCatalog(null));
    preflightDisk()
      .then((minutes) => alive && setDiskMinutes(minutes))
      .catch(() => alive && setDiskMinutes(null));
    for (const subscribe of [
      onProgram((status) => setProgram(status)),
      onAudio((levels) => setAudio(levels)),
      onReplay((status) => setReplay(status)),
    ]) {
      subscribe
        .then((fn) => {
          if (alive) cleanups.push(fn);
          else fn();
        })
        .catch(() => undefined);
    }
    return () => {
      alive = false;
      cleanups.forEach((fn) => fn());
    };
  }, []);

  const items = useMemo<Item[]>(() => {
    const list: Item[] = [];

    // 1. Targets: at least one enabled, each with its key/URL set.
    const targets = settings?.stream.targets.filter((target) => target.enabled) ?? [];
    const keyed = targets.every((target) => {
      const urlAuth = target.service === "srt" || target.service === "whip";
      return urlAuth ? target.ingestUrl.trim() !== "" : target.streamKey.trim() !== "";
    });
    list.push({
      key: "targets",
      tone: settings === null ? "warn" : targets.length > 0 && keyed ? "green" : "red",
      blocking: true,
      detail: t("preflight-targets-detail", { count: targets.length }),
      fix: { label: t("preflight-fix-stream"), run: onOpenStreamSettings },
    });

    // 2. Encoder probe: something usable in the catalog.
    const usable = catalog?.encoders.some((encoder) => encoder.verified !== false) ?? false;
    list.push({
      key: "encoder",
      tone: catalog === null ? "warn" : usable ? "green" : "red",
      blocking: true,
      fix: { label: t("preflight-fix-components"), run: onOpenComponents },
    });

    // 3. Source health (CAP-M13 data): nothing errored.
    const errored = Object.values(program?.sources ?? {}).filter(
      (source) => source.state === "error",
    ).length;
    list.push({
      key: "sources",
      tone: program === null ? "warn" : errored === 0 ? "green" : "red",
      blocking: true,
      detail: errored > 0 ? t("preflight-sources-detail", { count: errored }) : undefined,
      fix: { label: t("preflight-fix-sources"), run: onOpenSourceHealth },
    });

    // 4. Disk forecast (CAP-M10 math, same threshold as the live alarm).
    list.push({
      key: "disk",
      tone:
        diskMinutes === "loading" || diskMinutes === null
          ? "warn"
          : diskMinutes >= DISK_FLOOR_MIN
            ? "green"
            : "red",
      blocking: true,
      detail:
        typeof diskMinutes === "number"
          ? t("preflight-disk-detail", { minutes: diskMinutes })
          : undefined,
    });

    // 5/6. Mic + desktop audio metering — non-blocking nudges.
    const meterFor = (kind: "audioInput" | "audioOutput"): ItemTone => {
      const ids = (studio?.collection.sources ?? [])
        .filter((source) => source.kind === kind)
        .map((source) => source.id);
      if (ids.length === 0) return "warn"; // no such source — say so, never block
      const metering = ids.some((id) => {
        const levels = audio?.sources?.[id];
        return (
          levels?.state === "live" &&
          !levels.gated &&
          Math.max(levels.peak[0], levels.peak[1]) > METERING_PEAK
        );
      });
      return metering ? "green" : "red";
    };
    list.push({ key: "mic", tone: meterFor("audioInput"), blocking: false });
    list.push({ key: "desktopAudio", tone: meterFor("audioOutput"), blocking: false });

    // 7. Replay buffer — non-blocking; one click arms it.
    list.push({
      key: "replay",
      tone: replay?.armed ? "green" : "warn",
      blocking: false,
      fix: replay?.armed
        ? undefined
        : {
            label: t("preflight-fix-replay"),
            run: () => {
              replayArm().catch((err) => console.error("replay arm failed:", err));
            },
          },
    });

    return list;
  }, [
    settings,
    catalog,
    program,
    audio,
    replay,
    studio,
    diskMinutes,
    t,
    onOpenStreamSettings,
    onOpenComponents,
    onOpenSourceHealth,
  ]);

  // Only a CONFIRMED failure (red) blocks. An amber unknown — the forecast
  // unreadable, statuses still loading — must never turn the hold into a
  // lockout with no path to go live.
  const allGreen = items.every((item) => !item.blocking || item.tone !== "red");
  const hold = settings?.stream.preflightHold ?? false;

  const setHold = (next: boolean) => {
    if (!settings) return;
    const updated: Settings = {
      ...settings,
      stream: { ...settings.stream, preflightHold: next },
    };
    setSettings(updated);
    settingsSet(updated)
      .then(() => onSettingsSaved(updated))
      .catch((err) => console.error("preflight hold save failed:", err));
  };

  const dot: Record<ItemTone, string> = {
    green: "bg-emerald-500",
    red: "bg-red-500",
    warn: "bg-amber-400",
  };

  return (
    <PickerShell title={t("preflight-title")} onClose={onClose}>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        <p className="m-0 text-[11px] leading-snug text-havoc-muted">{t("preflight-intro")}</p>
        <ul className="m-0 flex list-none flex-col gap-1.5 p-0">
          {items.map((item) => (
            <li
              key={item.key}
              className="flex items-center gap-2 rounded-md border border-white/10 px-2.5 py-1.5"
            >
              <span
                aria-hidden
                className={`inline-block h-2 w-2 shrink-0 rounded-full ${dot[item.tone]}`}
              />
              <span className="min-w-0 flex-1">
                <span className="block">{t(`preflight-item-${item.key}`)}</span>
                {item.detail && (
                  <span className="block truncate text-[10px] text-havoc-muted">{item.detail}</span>
                )}
              </span>
              {!item.blocking && item.tone !== "green" && (
                <span className="shrink-0 text-[10px] text-havoc-muted">
                  {t("preflight-optional")}
                </span>
              )}
              {item.fix && item.tone !== "green" && (
                <button
                  type="button"
                  onClick={item.fix.run}
                  className="shrink-0 rounded border border-white/10 px-2 py-0.5 text-[10px] transition-colors hover:border-havoc-accent/60"
                >
                  {item.fix.label}
                </button>
              )}
            </li>
          ))}
        </ul>
        <label className="flex items-center gap-2 text-[11px] text-havoc-muted">
          <input
            type="checkbox"
            checked={hold}
            onChange={(event) => setHold(event.target.checked)}
          />
          {t("preflight-hold")}
        </label>
        <div className="flex justify-end gap-2">
          <button
            type="button"
            onClick={onClose}
            className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:text-havoc-text"
          >
            {t("preflight-cancel")}
          </button>
          {!allGreen && !hold && (
            <button
              type="button"
              onClick={onProceed}
              className="rounded-md border border-amber-400/50 bg-amber-400/10 px-3 py-1.5 text-xs text-amber-200 transition-colors hover:bg-amber-400/20"
            >
              {t("preflight-go-anyway")}
            </button>
          )}
          <button
            type="button"
            onClick={onProceed}
            disabled={!allGreen}
            className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text transition-colors hover:bg-havoc-accent/25 disabled:cursor-not-allowed disabled:opacity-50"
          >
            {t("preflight-go-live")}
          </button>
        </div>
      </div>
    </PickerShell>
  );
}
