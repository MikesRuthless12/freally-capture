import { render, screen } from "@testing-library/react";
import { act } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import type { RecordingStatus, StreamStatus } from "../api/types";
import { StatusAnnouncer } from "../components/StatusAnnouncer";

/**
 * The announcer subscribes through `onStream` / `onRecording`. Capture the
 * handlers so a test can drive the ~1 Hz status stream by hand.
 */
let emitStream: (status: StreamStatus) => void = () => undefined;
let emitRecording: (status: RecordingStatus) => void = () => undefined;

vi.mock("../api/events", () => ({
  onStream: (handler: (status: StreamStatus) => void) => {
    emitStream = handler;
    return Promise.resolve(() => undefined);
  },
  onRecording: (handler: (status: RecordingStatus) => void) => {
    emitRecording = handler;
    return Promise.resolve(() => undefined);
  },
  // CAP-M10: the announcer also listens for safety alarms.
  onAlarm: () => Promise.resolve(() => undefined),
}));

/** English is the fallback catalog, so the real keys resolve to real sentences. */
const stream = (over: Partial<StreamStatus> = {}): StreamStatus => ({
  state: "idle",
  elapsedSec: 0,
  reconnects: 0,
  framesDropped: 0,
  service: "Twitch",
  targets: [],
  ...over,
});

const live = () => screen.getByRole("status").textContent ?? "";

describe("StatusAnnouncer", () => {
  beforeEach(() => {
    render(<StatusAnnouncer />);
  });

  it("announces going live, and says nothing on the 1 Hz heartbeat", () => {
    act(() => emitStream(stream({ state: "live" })));
    expect(live()).toMatch(/live/i);

    const first = live();
    act(() => emitStream(stream({ state: "live", elapsedSec: 1 })));
    expect(live(), "an identical state must not re-announce").toBe(first);
  });

  it("announces a burst of dropped frames, not every frame", () => {
    act(() => emitStream(stream({ state: "live" })));
    act(() => emitStream(stream({ state: "live", framesDropped: 5 })));
    expect(live(), "5 frames is noise").toMatch(/live/i);

    act(() => emitStream(stream({ state: "live", framesDropped: 60 })));
    expect(live(), "60 frames is a lost second").toMatch(/60/);
  });

  /**
   * The regression this test exists for. `framesDropped` is cumulative and
   * climbs through an outage. Without re-baselining on the state change, the
   * first tick after reconnecting compares against the pre-outage count and
   * shouts the whole outage as a fresh burst — on top of the "reconnecting"
   * and "live" announcements the user already got.
   */
  it("does not announce a drop burst just because it reconnected", () => {
    act(() => emitStream(stream({ state: "live", framesDropped: 0 })));

    // The connection drops; frames pile up during the outage.
    act(() => emitStream(stream({ state: "reconnecting", framesDropped: 0 })));
    expect(live()).toMatch(/reconnect/i);
    act(() => emitStream(stream({ state: "reconnecting", framesDropped: 900 })));

    // Back on air. This must announce the reconnection, not 900 dropped frames.
    act(() => emitStream(stream({ state: "live", framesDropped: 900 })));
    expect(live()).toMatch(/live/i);

    // And the next quiet tick must stay quiet.
    act(() => emitStream(stream({ state: "live", framesDropped: 902 })));
    expect(live(), "2 new frames is not a burst").not.toMatch(/902/);
    expect(live()).toMatch(/live/i);

    // A genuinely new burst *after* the reconnect still announces.
    act(() => emitStream(stream({ state: "live", framesDropped: 980 })));
    expect(live()).toMatch(/980/);
  });

  it("re-baselines when a new session resets the counter", () => {
    act(() => emitStream(stream({ state: "live", framesDropped: 500 })));
    act(() => emitStream(stream({ state: "idle", framesDropped: 500 })));
    expect(live()).toMatch(/ended/i);

    act(() => emitStream(stream({ state: "live", framesDropped: 0 })));
    act(() => emitStream(stream({ state: "live", framesDropped: 10 })));
    expect(live(), "10 frames into a fresh session is not a burst").toMatch(/live/i);
  });

  it("announces recording transitions", () => {
    act(() => emitRecording({ state: "recording" } as RecordingStatus));
    expect(live()).toMatch(/recording/i);
    act(() => emitRecording({ state: "paused" } as RecordingStatus));
    expect(live()).toMatch(/paus/i);
  });
});
