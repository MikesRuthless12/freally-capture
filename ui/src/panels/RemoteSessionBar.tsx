import { useEffect, useRef, useState, useSyncExternalStore } from "react";

import { useT } from "../i18n/t";
import { studioSetItemSlot } from "../api/commands";
import type { ItemId, SceneId } from "../api/types";
import { POSITION_PRESETS } from "../lib/positions";
import { guestButton, guestCanToggle } from "../remote/mute";
import type { GuestQos } from "../remote/qos";
import {
  spikeGetState,
  type GuestView,
  spikeAutoGrid,
  spikeJoinFromLink,
  spikeRemoveGuest,
  spikeRequestCenter,
  spikeSeatGuest,
  spikeSendCue,
  spikeSetAllowCenter,
  spikeSetAutoGrid,
  spikeSetGreenRoomDefault,
  spikeSetHostGate,
  spikeSetJoinPrefill,
  spikeShareScreen,
  spikeStop,
  spikeSubscribe,
  spikeToggleSelfMute,
} from "../remote/spike";

/** Canned host→guest cues (CAP-N55): the i18n key (used for both the button
 * label and the localized cue text) and an optional countdown (seconds). */
const CANNED_CUES: { key: string; seconds: number | null }[] = [
  { key: "cue-thirty", seconds: 30 },
  { key: "cue-wrap", seconds: null },
  { key: "cue-next", seconds: null },
  { key: "cue-speak", seconds: null },
];

/**
 * A persistent bar for a live Remote Guest session — always on the main UI, so
 * the mute + moderation controls don't vanish when the setup dialog closes.
 * The host sees a row per connected guest (green room, QoS, cues, moderation);
 * the guest sees its own controls. Renders nothing when no session is active.
 */
export function RemoteSessionBar() {
  const t = useT();
  const session = useSyncExternalStore(spikeSubscribe, spikeGetState);

  if (!session.active) {
    // TASK-R2: a clicked freally:// invite waits here for explicit consent.
    if (!session.joinPrefill) return null;
    return (
      <div className="flex shrink-0 items-center gap-3 rounded-xl border border-havoc-accent/30 bg-havoc-accent/10 px-4 py-1.5 text-xs">
        <span className="font-semibold text-havoc-text">{t("remote-invite-received")}</span>
        <span className="min-w-0 flex-1 truncate font-mono text-[10px] text-havoc-muted">
          {session.joinPrefill}
        </span>
        <button
          type="button"
          onClick={() => spikeJoinFromLink(session.joinPrefill ?? "")}
          className="shrink-0 rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25"
        >
          {t("remote-join-with-webcam")}
        </button>
        <button
          type="button"
          onClick={() => spikeSetJoinPrefill(null)}
          className="shrink-0 rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
        >
          {t("remote-dismiss")}
        </button>
      </div>
    );
  }

  const connected = session.role === "host" ? session.guests.length > 0 : session.gates !== null;

  return (
    <div className="flex shrink-0 flex-col gap-1.5">
      <div className="flex flex-wrap items-center gap-3 rounded-xl border border-havoc-accent/30 bg-havoc-accent/10 px-4 py-1.5 text-xs">
        <span
          className={`h-2 w-2 shrink-0 rounded-full ${
            connected ? "bg-emerald-400" : "bg-amber-300"
          }`}
          aria-hidden
        />
        <span className="font-semibold text-havoc-text">
          {session.role === "host"
            ? t("remote-hosting-count", { count: session.guests.length })
            : t("remote-you-are-guest")}
        </span>
        <span className="min-w-0 flex-1 truncate text-havoc-muted">{session.status}</span>
        {session.role === "host" && <HostSessionControls session={session} />}
        {session.role === "guest" && <GuestSelfControls session={session} />}
        <button
          type="button"
          onClick={() => spikeStop()}
          className="shrink-0 rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-red-400/50 hover:text-red-300"
        >
          {session.role === "host" ? t("remote-end-session") : t("remote-leave")}
        </button>
      </div>
      {session.role === "host" &&
        session.guests.map((guest) => <GuestRow key={guest.peerId} guest={guest} />)}
      {session.role === "guest" && session.cue && (
        <CueBanner
          key={session.cue.at}
          text={session.cue.text}
          seconds={session.cue.seconds}
          at={session.cue.at}
        />
      )}
      {session.role === "guest" && session.greenRoom && (
        <div className="rounded-xl border border-amber-400/40 bg-amber-500/10 px-4 py-1.5 text-[11px] text-amber-200">
          {t("remote-green-room-guest")}
        </div>
      )}
      {session.role === "guest" && session.hostShare && (
        <HostViewPanel stream={session.hostShare} />
      )}
    </div>
  );
}

/** Host-level controls: share view, auto-grid, green-room default. */
function HostSessionControls({ session }: { session: ReturnType<typeof spikeGetState> }) {
  const t = useT();
  return (
    <>
      <button
        type="button"
        title={t("remote-green-room-default-title")}
        onClick={() => spikeSetGreenRoomDefault(!session.greenRoomDefault)}
        aria-pressed={session.greenRoomDefault}
        className={`shrink-0 rounded-md border px-2 py-1 text-[11px] ${
          session.greenRoomDefault
            ? "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text"
            : "border-white/10 text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
        }`}
      >
        {t("remote-green-room-default")}
      </button>
      <button
        type="button"
        title={t("remote-auto-grid-title")}
        onClick={() => spikeSetAutoGrid(!session.autoGridArmed)}
        aria-pressed={session.autoGridArmed}
        className={`shrink-0 rounded-md border px-2 py-1 text-[11px] ${
          session.autoGridArmed
            ? "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text"
            : "border-white/10 text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
        }`}
      >
        {t("remote-auto-grid")}
      </button>
      <button
        type="button"
        title={t("remote-arrange-grid-title")}
        onClick={() => void spikeAutoGrid()}
        className="shrink-0 rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
      >
        {t("remote-arrange-grid")}
      </button>
      <button
        type="button"
        title={t("remote-share-view-title")}
        onClick={() => void spikeShareScreen(!session.sharingScreen)}
        aria-pressed={session.sharingScreen}
        className={`shrink-0 rounded-md border px-2 py-1 text-[11px] ${
          session.sharingScreen
            ? "border-emerald-400/60 bg-emerald-500/20 text-emerald-300"
            : "border-white/10 text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
        }`}
      >
        {session.sharingScreen ? t("remote-stop-sharing-view") : t("remote-share-my-view")}
      </button>
    </>
  );
}

/** Guest-level controls: self mute, share screen, center requests. */
function GuestSelfControls({ session }: { session: ReturnType<typeof spikeGetState> }) {
  const t = useT();
  if (!session.gates) return null;
  return (
    <>
      <SelfMuteButton
        state={guestButton(session.gates)}
        canToggle={guestCanToggle(session.gates)}
      />
      <button
        type="button"
        title={t("remote-share-screen-title-guest")}
        onClick={() => void spikeShareScreen(!session.sharingScreen)}
        aria-pressed={session.sharingScreen}
        className={`shrink-0 rounded-md border px-2 py-1 text-[11px] ${
          session.sharingScreen
            ? "border-emerald-400/60 bg-emerald-500/20 text-emerald-300"
            : "border-white/10 text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
        }`}
      >
        {session.sharingScreen ? t("remote-stop-screen") : t("remote-share-screen")}
      </button>
      {session.allowCenter && (
        <div
          className="flex shrink-0 items-center gap-1"
          role="group"
          aria-label={t("remote-center-request-label")}
        >
          <span className="text-[10px] uppercase tracking-wide text-havoc-muted">
            {t("remote-center")}
          </span>
          <button
            type="button"
            onClick={() => spikeRequestCenter("guestCam")}
            className="rounded border border-white/10 px-1.5 py-0.5 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
          >
            {t("remote-center-my-cam")}
          </button>
          {session.sharingScreen && (
            <button
              type="button"
              onClick={() => spikeRequestCenter("guestScreen")}
              className="rounded border border-white/10 px-1.5 py-0.5 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
            >
              {t("remote-center-my-screen")}
            </button>
          )}
          <button
            type="button"
            onClick={() => spikeRequestCenter("hostView")}
            className="rounded border border-white/10 px-1.5 py-0.5 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
          >
            {t("remote-center-host-view")}
          </button>
        </div>
      )}
    </>
  );
}

/** One connected guest (host view): green room, QoS, mute, seats, cues, kick. */
function GuestRow({ guest }: { guest: GuestView }) {
  const t = useT();
  return (
    <div className="flex flex-wrap items-center gap-2 rounded-xl border border-white/10 bg-black/40 px-3 py-1.5 text-xs">
      <span className="font-semibold text-havoc-text">{guest.label}</span>
      {guest.qos && <QosBadge qos={guest.qos} />}
      <span className="min-w-0 flex-1 truncate text-havoc-muted">{guest.status}</span>

      {guest.greenRoom ? (
        <GreenRoomMonitor guest={guest} />
      ) : (
        <>
          <HostMuteButton peerId={guest.peerId} gates={guest.gates} />
          {guest.itemId && guest.sceneId && (
            <GuestPositionButtons sceneId={guest.sceneId} itemId={guest.itemId} />
          )}
          <button
            type="button"
            title={t("remote-allow-center-title")}
            onClick={() => spikeSetAllowCenter(guest.peerId, !guest.allowCenter)}
            aria-pressed={guest.allowCenter}
            className={`shrink-0 rounded-md border px-2 py-1 text-[11px] ${
              guest.allowCenter
                ? "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text"
                : "border-white/10 text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
            }`}
          >
            {t("remote-guest-switching")} {guest.allowCenter ? t("toggle-on") : t("toggle-off")}
          </button>
          <CueButtons peerId={guest.peerId} />
        </>
      )}
      <HostModerationButtons peerId={guest.peerId} />
    </div>
  );
}

/** CAP-N54: the private green-room monitor — the guest's incoming cam + a
 * tech-check (cam/mic present) and a one-click Seat-on-air. */
function GreenRoomMonitor({ guest }: { guest: GuestView }) {
  const t = useT();
  const videoRef = useRef<HTMLVideoElement>(null);
  useEffect(() => {
    const element = videoRef.current;
    if (!element || !guest.stream) return;
    element.srcObject = guest.stream;
    void element.play().catch(() => {});
    return () => {
      element.srcObject = null;
    };
  }, [guest.stream]);
  const hasCam = (guest.stream?.getVideoTracks().length ?? 0) > 0;
  const hasMic = (guest.stream?.getAudioTracks().length ?? 0) > 0;
  return (
    <div className="flex items-center gap-2">
      <video
        ref={videoRef}
        muted
        playsInline
        aria-label={t("remote-green-room-monitor")}
        className="h-12 w-20 rounded bg-black object-cover"
      />
      <div className="flex flex-col text-[10px] text-havoc-muted">
        <span className={hasCam ? "text-emerald-300" : "text-red-300"}>
          {hasCam ? t("remote-tech-cam-ok") : t("remote-tech-cam-no")}
        </span>
        <span className={hasMic ? "text-emerald-300" : "text-red-300"}>
          {hasMic ? t("remote-tech-mic-ok") : t("remote-tech-mic-no")}
        </span>
      </div>
      <button
        type="button"
        onClick={() => spikeSeatGuest(guest.peerId)}
        className="rounded-md border border-emerald-400/60 bg-emerald-500/20 px-2.5 py-1 text-[11px] font-semibold text-emerald-200 hover:bg-emerald-500/30"
      >
        {t("remote-seat-on-air")}
      </button>
    </div>
  );
}

/** CAP-N56: a green/amber/red roll-up with the key numbers + a sparkline. */
function QosBadge({ qos }: { qos: GuestQos }) {
  const t = useT();
  const color =
    qos.level === "good"
      ? "text-emerald-300 border-emerald-400/40"
      : qos.level === "fair"
        ? "text-amber-300 border-amber-400/40"
        : "text-red-300 border-red-400/40";
  const parts: string[] = [];
  if (qos.rttMs !== null) parts.push(`${Math.round(qos.rttMs)}ms`);
  if (qos.lossPct !== null) parts.push(`${qos.lossPct.toFixed(1)}%`);
  if (qos.fps !== null) parts.push(`${Math.round(qos.fps)}fps`);
  // The full detail (received resolution + jitter) lives in the tooltip.
  const detail = [
    t(`remote-qos-${qos.level}`),
    qos.width && qos.height ? `${qos.width}×${qos.height}` : null,
    qos.jitterMs !== null ? `jitter ${Math.round(qos.jitterMs)}ms` : null,
  ]
    .filter(Boolean)
    .join(" · ");
  return (
    <span
      className={`flex shrink-0 items-center gap-1.5 rounded-md border px-1.5 py-0.5 font-mono text-[10px] ${color}`}
      title={detail}
    >
      <Sparkline values={qos.history} />
      {parts.join(" · ")}
    </span>
  );
}

/** A tiny 0–100 sparkline of recent quality scores. */
function Sparkline({ values }: { values: number[] }) {
  if (values.length < 2) return null;
  const w = 36;
  const h = 12;
  const step = w / (values.length - 1);
  const points = values
    .map(
      (v, i) =>
        `${(i * step).toFixed(1)},${(h - (Math.max(0, Math.min(100, v)) / 100) * h).toFixed(1)}`,
    )
    .join(" ");
  return (
    <svg width={w} height={h} viewBox={`0 0 ${w} ${h}`} aria-hidden="true">
      <polyline points={points} fill="none" stroke="currentColor" strokeWidth={1} />
    </svg>
  );
}

/** CAP-N55: the canned-cue buttons for one guest. */
function CueButtons({ peerId }: { peerId: string }) {
  const t = useT();
  return (
    <div
      className="flex shrink-0 items-center gap-1"
      role="group"
      aria-label={t("remote-cues-label")}
    >
      <span className="text-[10px] uppercase tracking-wide text-havoc-muted">
        {t("remote-cue")}
      </span>
      {CANNED_CUES.map((cue) => (
        <button
          key={cue.key}
          type="button"
          onClick={() => spikeSendCue(peerId, t(`remote-${cue.key}`), cue.seconds)}
          className="rounded border border-white/10 px-1.5 py-0.5 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
        >
          {t(`remote-${cue.key}`)}
        </button>
      ))}
    </div>
  );
}

/** The cue the guest received (CAP-N55), with a live countdown when timed. */
function CueBanner({ text, seconds, at }: { text: string; seconds: number | null; at: number }) {
  const [left, setLeft] = useState<number | null>(seconds);
  useEffect(() => {
    // Keyed by `at` in the parent, so this only mounts for a fresh cue; a
    // null-seconds cue keeps its initial null and never ticks.
    if (seconds === null) return;
    const tick = () => {
      setLeft(Math.max(0, Math.ceil(seconds - (Date.now() - at) / 1000)));
    };
    tick();
    const timer = window.setInterval(tick, 250);
    return () => window.clearInterval(timer);
  }, [seconds, at]);
  return (
    <div className="flex items-center gap-3 rounded-xl border border-havoc-accent/50 bg-havoc-accent/15 px-4 py-2 text-sm font-semibold text-havoc-text">
      <span className="text-[10px] uppercase tracking-widest text-havoc-accent">CUE</span>
      <span className="flex-1">{text}</span>
      {left !== null && (
        <span className="font-mono text-lg tabular-nums text-havoc-accent">{left}s</span>
      )}
    </div>
  );
}

/** The host's shared view, rendered live in the guest's app (TASK-R6). */
function HostViewPanel({ stream }: { stream: MediaStream }) {
  const t = useT();
  const videoRef = useRef<HTMLVideoElement>(null);
  useEffect(() => {
    const element = videoRef.current;
    if (!element) return;
    element.srcObject = stream;
    void element.play().catch(() => {});
    return () => {
      element.srcObject = null;
    };
  }, [stream]);
  return (
    <div className="rounded-xl border border-white/10 bg-black/60 p-1.5">
      <p className="m-0 px-1 pb-1 text-[10px] uppercase tracking-wide text-havoc-muted">
        {t("remote-host-view-heading")}
      </p>
      <video
        ref={videoRef}
        muted
        playsInline
        aria-label={t("remote-host-shared-view-label")}
        className="max-h-56 w-full rounded-lg bg-black object-contain"
      />
    </div>
  );
}

/** One-click guest seats: top/middle/bottom × left/right. */
function GuestPositionButtons({ sceneId, itemId }: { sceneId: SceneId; itemId: ItemId }) {
  const t = useT();
  return (
    <div
      className="flex shrink-0 items-center gap-1"
      role="group"
      aria-label={t("remote-guest-position-label")}
    >
      {POSITION_PRESETS.map((preset) => (
        <button
          key={preset.key}
          type="button"
          title={t("remote-put-guest", { position: preset.label })}
          aria-label={t("remote-put-guest", { position: preset.label })}
          onClick={() =>
            studioSetItemSlot(sceneId, itemId, preset.slot).catch((err) =>
              console.error("guest position failed:", err),
            )
          }
          className="rounded border border-white/10 px-1.5 py-0.5 text-[11px] leading-none text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
        >
          {preset.glyph}
        </button>
      ))}
    </div>
  );
}

/** TASK-R8 host moderation for one guest: kick or ban (denylist the peer id). */
function HostModerationButtons({ peerId }: { peerId: string }) {
  const t = useT();
  return (
    <div className="flex shrink-0 items-center gap-1">
      <button
        type="button"
        title={t("remote-remove-title")}
        onClick={() => void spikeRemoveGuest(peerId, false)}
        className="rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-amber-400/50 hover:text-amber-300"
      >
        {t("remote-remove")}
      </button>
      <button
        type="button"
        title={t("remote-ban-title")}
        onClick={() => void spikeRemoveGuest(peerId, true)}
        className="rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-red-400/50 hover:text-red-300"
      >
        {t("remote-ban")}
      </button>
    </div>
  );
}

function HostMuteButton({
  peerId,
  gates,
}: {
  peerId: string;
  gates: { hostGate: boolean; selfGate: boolean };
}) {
  const t = useT();
  return (
    <div className="flex shrink-0 items-center gap-2">
      {!gates.hostGate && gates.selfGate && (
        <span className="text-[11px] text-amber-300">{t("remote-guest-self-muted")}</span>
      )}
      <button
        type="button"
        onClick={() => spikeSetHostGate(peerId, !gates.hostGate)}
        aria-pressed={gates.hostGate}
        className={`rounded-md border px-3 py-1 text-xs font-semibold ${
          gates.hostGate
            ? "border-red-400/60 bg-red-500/20 text-red-300"
            : "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text hover:bg-havoc-accent/25"
        }`}
      >
        {gates.hostGate ? t("remote-unmute-guest") : t("remote-mute-guest")}
      </button>
    </div>
  );
}

function SelfMuteButton({
  state,
  canToggle,
}: {
  state: "live" | "selfMuted" | "hostMuted";
  canToggle: boolean;
}) {
  const t = useT();
  const label =
    state === "hostMuted"
      ? t("remote-muted-by-host")
      : state === "selfMuted"
        ? t("remote-unmute-mic")
        : t("remote-mute-mic");
  const className =
    state === "hostMuted"
      ? "border-red-400/60 bg-red-500/20 text-red-300"
      : state === "selfMuted"
        ? "border-amber-400/60 bg-amber-500/20 text-amber-300"
        : "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text hover:bg-havoc-accent/25";
  return (
    <div className="flex shrink-0 items-center gap-2">
      {state === "hostMuted" && (
        <span className="text-[11px] text-havoc-muted">{t("remote-waiting-for-host")}</span>
      )}
      <button
        type="button"
        disabled={!canToggle}
        onClick={() => spikeToggleSelfMute()}
        aria-pressed={state !== "live"}
        className={`rounded-md border px-3 py-1 text-xs font-semibold disabled:cursor-not-allowed ${className}`}
      >
        {label}
      </button>
    </div>
  );
}
