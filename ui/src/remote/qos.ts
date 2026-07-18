/**
 * Per-guest connection quality (CAP-N56): parse WebRTC `getStats()` into a
 * small, honest QoS snapshot — RTT, jitter, packet loss, received resolution
 * and fps — with a green/amber/red roll-up and a short history for a sparkline.
 *
 * Pure functions so the classification + history are unit-tested without a live
 * peer connection.
 */

export type QosLevel = "good" | "fair" | "poor";

export type GuestQos = {
  /** Round-trip time, ms (from the nominated candidate pair). */
  rttMs: number | null;
  /** Inbound video jitter, ms. */
  jitterMs: number | null;
  /** Packet loss over the last interval, percent. */
  lossPct: number | null;
  /** Received frames per second. */
  fps: number | null;
  /** Received resolution. */
  width: number | null;
  height: number | null;
  level: QosLevel;
  /** Recent quality scores (0–100), oldest first, for the sparkline. */
  history: number[];
};

/** Cumulative counters carried between samples (packet-loss deltas + history). */
export type QosPrev = { received: number; lost: number; history: number[] };

/** How many quality scores the sparkline keeps. */
const HISTORY = 30;

/** Roll RTT / loss / fps into a single good/amber/red verdict. */
export function classifyQos(qos: {
  rttMs: number | null;
  lossPct: number | null;
  fps: number | null;
}): QosLevel {
  const loss = qos.lossPct ?? 0;
  const rtt = qos.rttMs ?? 0;
  const fps = qos.fps;
  if (loss > 5 || rtt > 400 || (fps !== null && fps > 0 && fps < 12)) return "poor";
  if (loss > 1.5 || rtt > 200 || (fps !== null && fps > 0 && fps < 20)) return "fair";
  return "good";
}

/** A 0–100 quality score for the sparkline (100 = pristine). */
export function qualityScore(qos: { rttMs: number | null; lossPct: number | null }): number {
  const loss = qos.lossPct ?? 0;
  const rtt = qos.rttMs ?? 0;
  const lossPenalty = Math.min(60, loss * 8);
  const rttPenalty = Math.min(40, Math.max(0, rtt - 60) / 10);
  return Math.max(0, Math.round(100 - lossPenalty - rttPenalty));
}

/** A number field from a WebRTC stats report, or null. */
function num(report: Record<string, unknown>, key: string): number | null {
  const value = report[key];
  return typeof value === "number" && Number.isFinite(value) ? value : null;
}

/**
 * Turn one `getStats()` report into a QoS snapshot. `prev` carries the previous
 * cumulative packet counters (for interval loss) and the score history.
 */
export function sampleQos(stats: RTCStatsReport, prev?: QosPrev): { qos: GuestQos; prev: QosPrev } {
  let rttMs: number | null = null;
  let jitterMs: number | null = null;
  let fps: number | null = null;
  let width: number | null = null;
  let height: number | null = null;
  let received = 0;
  let lost = 0;

  stats.forEach((raw) => {
    const report = raw as unknown as Record<string, unknown>;
    const type = report.type;
    if (
      type === "candidate-pair" &&
      (report.nominated === true || report.selected === true) &&
      typeof report.currentRoundTripTime === "number"
    ) {
      rttMs = report.currentRoundTripTime * 1000;
    }
    if (type === "inbound-rtp" && report.kind === "video") {
      const jitter = num(report, "jitter");
      if (jitter !== null) jitterMs = jitter * 1000;
      fps = num(report, "framesPerSecond");
      width = num(report, "frameWidth");
      height = num(report, "frameHeight");
      received += num(report, "packetsReceived") ?? 0;
      lost += Math.max(0, num(report, "packetsLost") ?? 0);
    }
  });

  // Interval packet loss from the delta since the last sample.
  let lossPct: number | null = null;
  if (prev) {
    const dReceived = Math.max(0, received - prev.received);
    const dLost = Math.max(0, lost - prev.lost);
    const total = dReceived + dLost;
    lossPct = total > 0 ? (dLost / total) * 100 : 0;
  }

  const level = classifyQos({ rttMs, lossPct, fps });
  const score = qualityScore({ rttMs, lossPct });
  const history = [...(prev?.history ?? []), score].slice(-HISTORY);

  return {
    qos: { rttMs, jitterMs, lossPct, fps, width, height, level, history },
    prev: { received, lost, history },
  };
}
