import { describe, expect, it, vi } from "vitest";

import { deviceOption, micConstraints, routeToOutput } from "../remote/devices";

describe("deviceOption", () => {
  it("keeps a real label", () => {
    expect(deviceOption("audioinput", { deviceId: "abc", label: "USB Mic" }, 0)).toEqual({
      deviceId: "abc",
      label: "USB Mic",
    });
  });

  it("names label-less (pre-permission) devices honestly, 1-based", () => {
    expect(deviceOption("audioinput", { deviceId: "a", label: "" }, 1).label).toBe("Microphone 2");
    expect(deviceOption("audiooutput", { deviceId: "b", label: "" }, 0).label).toBe("Output 1");
  });
});

describe("micConstraints", () => {
  it("defaults to echo-safe processing with no device pin", () => {
    expect(micConstraints(null)).toEqual({
      echoCancellation: true,
      noiseSuppression: true,
      autoGainControl: true,
    });
  });

  it("pins a chosen mic with ideal (degrades instead of failing)", () => {
    expect(micConstraints("mic-1").deviceId).toEqual({ ideal: "mic-1" });
  });
});

describe("routeToOutput", () => {
  it("is ok on the default device even without setSinkId support", async () => {
    const element = document.createElement("audio");
    expect(await routeToOutput(element, null)).toBe("ok");
  });

  it("reports unsupported when a device is requested but setSinkId is missing", async () => {
    const element = document.createElement("audio");
    expect(await routeToOutput(element, "spk-1")).toBe("unsupported");
  });

  it("routes via setSinkId when available and reports failures", async () => {
    const element = document.createElement("audio") as HTMLAudioElement & {
      setSinkId: (id: string) => Promise<void>;
    };
    const setSinkId = vi.fn().mockResolvedValue(undefined);
    element.setSinkId = setSinkId;
    expect(await routeToOutput(element, "spk-1")).toBe("ok");
    expect(setSinkId).toHaveBeenCalledWith("spk-1");

    element.setSinkId = vi.fn().mockRejectedValue(new Error("no such device"));
    expect(await routeToOutput(element, "spk-2")).toBe("failed");
  });
});
