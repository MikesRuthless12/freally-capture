import { useEffect, useRef, useSyncExternalStore } from "react";

import { useT } from "../i18n/t";
import { studioSetItemSlot } from "../api/commands";
import type { ItemId, SceneId } from "../api/types";
import { POSITION_PRESETS } from "../lib/positions";
import { guestButton, guestCanToggle } from "../remote/mute";
import {
  spikeGetState,
  spikeJoinFromLink,
  spikeRemoveGuest,
  spikeRequestCenter,
  spikeSetAllowCenter,
  spikeSetHostGate,
  spikeSetJoinPrefill,
  spikeShareScreen,
  spikeStop,
  spikeSubscribe,
  spikeToggleSelfMute,
} from "../remote/spike";

/**
 * A persistent bar for a live Remote Guest session — always on the main UI, so
 * the mute controls don't vanish when the setup dialog closes. Renders nothing
 * when no session is active.
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

  const connected = session.gates !== null || session.guestItem !== null;

  return (
    <div className="flex shrink-0 flex-col gap-1.5">
      <div className="flex items-center gap-3 rounded-xl border border-havoc-accent/30 bg-havoc-accent/10 px-4 py-1.5 text-xs">
        <span
          className={`h-2 w-2 shrink-0 rounded-full ${
            session.gates ? "bg-emerald-400" : "bg-amber-300"
          }`}
          aria-hidden
        />
        <span className="font-semibold text-havoc-text">
          {session.role === "host" ? t("remote-hosting-guest") : t("remote-you-are-guest")}
        </span>
        <span className="min-w-0 flex-1 truncate text-havoc-muted">{session.status}</span>
        {session.role === "host" && session.guestItem && (
          <GuestPositionButtons target={session.guestItem} />
        )}
        {session.gates &&
          (session.role === "host" ? (
            <HostMuteButton hostGate={session.gates.hostGate} selfGate={session.gates.selfGate} />
          ) : (
            <SelfMuteButton
              state={guestButton(session.gates)}
              canToggle={guestCanToggle(session.gates)}
            />
          ))}
        {session.role === "host" && connected && (
          <>
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
            <button
              type="button"
              title={t("remote-allow-center-title")}
              onClick={() => spikeSetAllowCenter(!session.allowCenter)}
              aria-pressed={session.allowCenter}
              className={`shrink-0 rounded-md border px-2 py-1 text-[11px] ${
                session.allowCenter
                  ? "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text"
                  : "border-white/10 text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
              }`}
            >
              {t("remote-guest-switching")} {session.allowCenter ? t("toggle-on") : t("toggle-off")}
            </button>
            <HostModerationButtons />
          </>
        )}
        {session.role === "guest" && session.gates && (
          <>
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
                  title={t("remote-center-cam-title")}
                  onClick={() => spikeRequestCenter("guestCam")}
                  className="rounded border border-white/10 px-1.5 py-0.5 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
                >
                  {t("remote-center-my-cam")}
                </button>
                {session.sharingScreen && (
                  <button
                    type="button"
                    title={t("remote-center-screen-title")}
                    onClick={() => spikeRequestCenter("guestScreen")}
                    className="rounded border border-white/10 px-1.5 py-0.5 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
                  >
                    {t("remote-center-my-screen")}
                  </button>
                )}
                <button
                  type="button"
                  title={t("remote-center-host-title")}
                  onClick={() => spikeRequestCenter("hostView")}
                  className="rounded border border-white/10 px-1.5 py-0.5 text-[11px] text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
                >
                  {t("remote-center-host-view")}
                </button>
              </div>
            )}
          </>
        )}
        <button
          type="button"
          onClick={() => spikeStop()}
          className="shrink-0 rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-red-400/50 hover:text-red-300"
        >
          {session.role === "host" ? t("remote-end-session") : t("remote-leave")}
        </button>
      </div>
      {session.role === "guest" && session.hostShare && (
        <HostViewPanel stream={session.hostShare} />
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

/** One-click guest seats: top/middle/bottom × left/right. The engine fits the
 * guest into the slot on their next frame (same path as the ▦ Arrange). */
function GuestPositionButtons({ target }: { target: { sceneId: SceneId; itemId: ItemId } }) {
  const t = useT();
  return (
    <div
      className="flex shrink-0 items-center gap-1"
      role="group"
      aria-label={t("remote-guest-position-label")}
    >
      <span className="text-[10px] uppercase tracking-wide text-havoc-muted">
        {t("remote-guest-label")}
      </span>
      {POSITION_PRESETS.map((preset) => (
        <button
          key={preset.key}
          type="button"
          title={t("remote-put-guest", { position: preset.label })}
          aria-label={t("remote-put-guest", { position: preset.label })}
          onClick={() =>
            studioSetItemSlot(target.sceneId, target.itemId, preset.slot).catch((err) =>
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

/** TASK-R8 host moderation: kick (link stays valid) or ban (denylist + a
 * fresh session id, so the old invite link dies). */
function HostModerationButtons() {
  const t = useT();
  return (
    <div className="flex shrink-0 items-center gap-1">
      <button
        type="button"
        title={t("remote-remove-title")}
        onClick={() => void spikeRemoveGuest(false)}
        className="rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-amber-400/50 hover:text-amber-300"
      >
        {t("remote-remove")}
      </button>
      <button
        type="button"
        title={t("remote-ban-title")}
        onClick={() => void spikeRemoveGuest(true)}
        className="rounded-md border border-white/10 px-2 py-1 text-[11px] text-havoc-muted hover:border-red-400/50 hover:text-red-300"
      >
        {t("remote-ban")}
      </button>
    </div>
  );
}

function HostMuteButton({ hostGate, selfGate }: { hostGate: boolean; selfGate: boolean }) {
  const t = useT();
  return (
    <div className="flex shrink-0 items-center gap-2">
      {!hostGate && selfGate && (
        <span className="text-[11px] text-amber-300">{t("remote-guest-self-muted")}</span>
      )}
      <button
        type="button"
        onClick={() => spikeSetHostGate(!hostGate)}
        aria-pressed={hostGate}
        className={`rounded-md border px-3 py-1 text-xs font-semibold ${
          hostGate
            ? "border-red-400/60 bg-red-500/20 text-red-300"
            : "border-havoc-accent/60 bg-havoc-accent/15 text-havoc-text hover:bg-havoc-accent/25"
        }`}
      >
        {hostGate ? t("remote-unmute-guest") : t("remote-mute-guest")}
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
