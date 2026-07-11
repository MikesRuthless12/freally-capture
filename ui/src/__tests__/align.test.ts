import { describe, expect, it } from "vitest";

import type { SceneItem, Transform } from "../api/types";
import { alignItems, alignToCanvas, distributeItems, type Measured } from "../lib/align";
import type { Box } from "../lib/constrain";

const canvas = { w: 1920, h: 1080 };

function plain(x: number, y: number): Transform {
  return {
    x,
    y,
    scaleX: 1,
    scaleY: 1,
    rotation: 0,
    crop: { left: 0, top: 0, right: 0, bottom: 0 },
  };
}
/** A measured item whose transform center sits at (cx, cy) over `box`. */
function measured(id: string, cx: number, cy: number, box: Box): Measured {
  return { id, transform: plain(cx, cy), box };
}
const hbox = (minX: number, maxX: number): Box => ({ minX, maxX, minY: 0, maxY: 100 });

/** A 200×100 source item centered at (x, y), no crop/rotation/scale. */
function item(x: number, y: number, extra: Partial<Transform> = {}): SceneItem {
  return {
    id: "i",
    source: "s",
    visible: true,
    locked: false,
    blend: "normal",
    transform: {
      x,
      y,
      scaleX: 1,
      scaleY: 1,
      rotation: 0,
      crop: { left: 0, top: 0, right: 0, bottom: 0 },
      ...extra,
    },
    pendingFit: false,
    filters: [],
  };
}

describe("align to canvas (CAP-M04)", () => {
  // box of item(500,300): x∈[400,600], y∈[250,350].
  it("aligns the left edge to the canvas left", () => {
    expect(alignToCanvas(item(500, 300), 200, 100, canvas, "left")?.x).toBe(100);
  });
  it("centers horizontally", () => {
    expect(alignToCanvas(item(500, 300), 200, 100, canvas, "hcenter")?.x).toBe(960);
  });
  it("aligns the right edge to the canvas right", () => {
    expect(alignToCanvas(item(500, 300), 200, 100, canvas, "right")?.x).toBe(1820);
  });
  it("aligns the top edge to the canvas top", () => {
    expect(alignToCanvas(item(500, 300), 200, 100, canvas, "top")?.y).toBe(50);
  });
  it("centers vertically", () => {
    expect(alignToCanvas(item(500, 300), 200, 100, canvas, "vcenter")?.y).toBe(540);
  });
  it("aligns the bottom edge to the canvas bottom", () => {
    expect(alignToCanvas(item(500, 300), 200, 100, canvas, "bottom")?.y).toBe(1030);
  });
  it("leaves the untouched axis alone", () => {
    const result = alignToCanvas(item(500, 300), 200, 100, canvas, "left");
    expect(result?.y).toBe(300);
  });
  it("returns null when fully cropped away", () => {
    const cropped = item(500, 300, { crop: { left: 200, top: 0, right: 0, bottom: 0 } });
    expect(alignToCanvas(cropped, 200, 100, canvas, "left")).toBeNull();
  });
});

describe("align items to each other (CAP-M04 follow-on)", () => {
  // A: box [0,100] center 50; B: [200,260] center 230; C: [400,500] center 450.
  const items: Measured[] = [
    measured("a", 50, 0, hbox(0, 100)),
    measured("b", 230, 0, hbox(200, 260)),
    measured("c", 450, 0, hbox(400, 500)),
  ];

  it("aligns left edges to the group's leftmost", () => {
    const out = alignItems(items, "left");
    expect(out.get("a")).toBeUndefined(); // already the leftmost — unmoved
    expect(out.get("b")?.x).toBe(30); // 230 + (0 - 200)
    expect(out.get("c")?.x).toBe(50); // 450 + (0 - 400)
  });

  it("aligns right edges to the group's rightmost", () => {
    const out = alignItems(items, "right");
    expect(out.get("c")).toBeUndefined();
    expect(out.get("a")?.x).toBe(450); // 50 + (500 - 100)
    expect(out.get("b")?.x).toBe(470); // 230 + (500 - 260)
  });

  it("centers items on the group's shared center line", () => {
    // group box x∈[0,500] → center 250.
    const out = alignItems(items, "hcenter");
    expect(out.get("a")?.x).toBe(250);
    expect(out.get("c")?.x).toBe(250);
  });

  it("is a no-op for fewer than two items", () => {
    expect(alignItems([items[0]], "left").size).toBe(0);
  });
});

describe("distribute items (CAP-M04 follow-on)", () => {
  it("equalizes the gaps between boxes, extremes fixed", () => {
    // A [0,100], B [200,260], C [500,600]. span 600, sizes 260, gap = 170.
    const items: Measured[] = [
      measured("a", 50, 0, hbox(0, 100)),
      measured("b", 230, 0, hbox(200, 260)),
      measured("c", 550, 0, hbox(500, 600)),
    ];
    const out = distributeItems(items, "h");
    expect(out.get("a")).toBeUndefined();
    expect(out.get("c")).toBeUndefined();
    // B's low edge → 100 + 170 = 270, so its center shifts by (270 - 200) = 70.
    expect(out.get("b")?.x).toBe(300);
  });

  it("distributes along the vertical axis", () => {
    const vbox = (minY: number, maxY: number): Box => ({ minX: 0, maxX: 100, minY, maxY });
    const items: Measured[] = [
      measured("a", 0, 50, vbox(0, 100)),
      measured("b", 0, 230, vbox(200, 260)),
      measured("c", 0, 550, vbox(500, 600)),
    ];
    expect(distributeItems(items, "v").get("b")?.y).toBe(300);
  });

  it("needs at least three items", () => {
    const items: Measured[] = [
      measured("a", 50, 0, hbox(0, 100)),
      measured("b", 230, 0, hbox(200, 260)),
    ];
    expect(distributeItems(items, "h").size).toBe(0);
  });
});
