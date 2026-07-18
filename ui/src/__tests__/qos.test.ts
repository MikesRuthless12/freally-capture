import { describe, expect, it } from "vitest";

import { classifyQos, qualityScore, sampleQos } from "../remote/qos";

/** Build a fake RTCStatsReport (a Map) from a candidate-pair + inbound video. */
function fakeStats(opts: {
  rttSec?: number;
  jitter?: number;
  fps?: number;
  width?: number;
  height?: number;
  received?: number;
  lost?: number;
}): RTCStatsReport {
  const map = new Map<string, unknown>();
  map.set("cp", {
    type: "candidate-pair",
    nominated: true,
    currentRoundTripTime: opts.rttSec ?? 0.05,
  });
  map.set("in", {
    type: "inbound-rtp",
    kind: "video",
    jitter: opts.jitter ?? 0.01,
    framesPerSecond: opts.fps ?? 30,
    frameWidth: opts.width ?? 1280,
    frameHeight: opts.height ?? 720,
    packetsReceived: opts.received ?? 1000,
    packetsLost: opts.lost ?? 0,
  });
  return map as unknown as RTCStatsReport;
}

describe("classifyQos", () => {
  it("is good on a clean link", () => {
    expect(classifyQos({ rttMs: 40, lossPct: 0, fps: 30 })).toBe("good");
  });
  it("is fair on moderate loss or latency", () => {
    expect(classifyQos({ rttMs: 250, lossPct: 0, fps: 30 })).toBe("fair");
    expect(classifyQos({ rttMs: 40, lossPct: 2, fps: 30 })).toBe("fair");
    expect(classifyQos({ rttMs: 40, lossPct: 0, fps: 16 })).toBe("fair");
  });
  it("is poor on heavy loss, high latency, or a frozen feed", () => {
    expect(classifyQos({ rttMs: 500, lossPct: 0, fps: 30 })).toBe("poor");
    expect(classifyQos({ rttMs: 40, lossPct: 8, fps: 30 })).toBe("poor");
    expect(classifyQos({ rttMs: 40, lossPct: 0, fps: 5 })).toBe("poor");
  });
});

describe("qualityScore", () => {
  it("is 100 on a pristine link and drops with loss + latency", () => {
    expect(qualityScore({ rttMs: 50, lossPct: 0 })).toBe(100);
    expect(qualityScore({ rttMs: 50, lossPct: 5 })).toBeLessThan(100);
    expect(qualityScore({ rttMs: 400, lossPct: 0 })).toBeLessThan(100);
    expect(qualityScore({ rttMs: 5000, lossPct: 50 })).toBe(0);
  });
});

describe("sampleQos", () => {
  it("parses RTT, jitter, fps, and resolution", () => {
    const { qos } = sampleQos(fakeStats({ rttSec: 0.08, jitter: 0.02, fps: 24 }));
    expect(qos.rttMs).toBeCloseTo(80);
    expect(qos.jitterMs).toBeCloseTo(20);
    expect(qos.fps).toBe(24);
    expect(qos.width).toBe(1280);
    expect(qos.height).toBe(720);
    // First sample has no previous counters → loss is null.
    expect(qos.lossPct).toBeNull();
    expect(qos.history).toHaveLength(1);
  });

  it("computes interval packet loss from the delta and grows history", () => {
    const first = sampleQos(fakeStats({ received: 1000, lost: 0 }));
    // Next interval: +100 received, +10 lost → ~9.1% loss.
    const second = sampleQos(fakeStats({ received: 1100, lost: 10 }), first.prev);
    expect(second.qos.lossPct).toBeCloseTo((10 / 110) * 100, 1);
    expect(second.qos.level).toBe("poor");
    expect(second.qos.history).toHaveLength(2);
  });

  it("caps history to the sparkline window", () => {
    let prev = sampleQos(fakeStats({})).prev;
    for (let i = 0; i < 50; i += 1) {
      prev = sampleQos(fakeStats({ received: 1000 + i, lost: 0 }), prev).prev;
    }
    expect(prev.history.length).toBeLessThanOrEqual(30);
  });
});
