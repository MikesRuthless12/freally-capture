import { useEffect, useRef, useState } from "react";

import { onRecording, onStream } from "../api/events";
import { useT } from "../i18n/t";

/**
 * The screen-reader announcer for broadcast state (TASK-901).
 *
 * Going live, starting a recording, losing the connection, and dropping frames
 * are signalled visually — a red dot, an amber badge, a colour change. A blind
 * streamer would have no idea any of it happened. This renders those transitions
 * into an `aria-live` region so assistive tech speaks them.
 *
 * `polite`, not `assertive`: these interrupt nothing the user is doing, and an
 * assertive region would talk over them mid-sentence every time the reconnect
 * counter ticked.
 *
 * Only *transitions* are announced. The status events arrive ~1 Hz, and
 * re-writing an identical message would make some screen readers repeat it once
 * a second, which is worse than silence.
 */
export function StatusAnnouncer() {
  const t = useT();
  const [message, setMessage] = useState("");

  // The last announced state per channel, so a 1 Hz heartbeat announces nothing.
  const lastRecording = useRef<string | null>(null);
  const lastStream = useRef<string | null>(null);
  // Dropped frames climb monotonically; announce a *burst*, not every frame.
  const lastDropped = useRef(0);

  useEffect(() => {
    let alive = true;
    // Every call here is a state *transition*, and a repeated drop burst carries
    // a rising count — so two consecutive announcements can never be the same
    // string. Nothing has to be done to force a live region to re-fire.
    const say = (next: string) => {
      if (alive) setMessage(next);
    };

    const unlistenRecording = onRecording((status) => {
      if (status.state === lastRecording.current) return;
      lastRecording.current = status.state;
      if (status.state === "recording") say(t("announce-recording-started"));
      else if (status.state === "paused") say(t("announce-recording-paused"));
      else if (status.state === "idle") say(t("announce-recording-stopped"));
    }).catch(() => undefined);

    const unlistenStream = onStream((status) => {
      if (status.state !== lastStream.current) {
        lastStream.current = status.state;
        // Re-baseline on every transition. `framesDropped` is cumulative and
        // keeps climbing through an outage, so a live → reconnecting → live
        // round trip would otherwise leave a stale baseline and the first tick
        // back on air would announce the whole outage as a fresh burst — an
        // interruption the user already heard as "reconnecting" and "live".
        lastDropped.current = status.framesDropped;
        if (status.state === "live") say(t("announce-live-started"));
        else if (status.state === "reconnecting") say(t("announce-reconnecting"));
        else if (status.state === "failed") say(t("announce-stream-failed"));
        else if (status.state === "ended" || status.state === "idle") {
          say(t("announce-live-ended"));
        }
        return;
      }
      // A burst of drops while live is the thing a streamer needs to hear.
      const dropped = status.framesDropped;
      if (status.state === "live" && dropped - lastDropped.current >= DROP_BURST) {
        lastDropped.current = dropped;
        say(t("announce-frames-dropped", { count: dropped }));
      } else if (dropped < lastDropped.current) {
        // A new session reset the counter.
        lastDropped.current = dropped;
      }
    }).catch(() => undefined);

    return () => {
      alive = false;
      void unlistenRecording.then((fn) => fn?.());
      void unlistenStream.then((fn) => fn?.());
    };
  }, [t]);

  return (
    <div role="status" aria-live="polite" aria-atomic="true" className="sr-only">
      {message}
    </div>
  );
}

/**
 * How many new dropped frames constitute a burst worth interrupting for. One
 * dropped frame is noise; sixty is a second of video gone.
 */
const DROP_BURST = 60;
