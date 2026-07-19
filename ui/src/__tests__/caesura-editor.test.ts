import { describe, expect, it } from "vitest";

import { isChip, normalizePaste, tokenize } from "../lib/caesuraChips";

/** Re-join tokens; must reproduce the input byte-for-byte (the round-trip that
 * lets the chip editor stay a faithful controlled view of the script string). */
const roundTrip = (s: string) =>
  tokenize(s)
    .map((t) => (isChip(t) ? t.chip : t.text))
    .join("");
const chips = (s: string) =>
  tokenize(s)
    .filter(isChip)
    .map((t) => t.chip);

describe("caesura tokenizer", () => {
  it("round-trips arbitrary scripts exactly", () => {
    for (const s of [
      "",
      "plain text no caesura",
      "aaaaa -- bbbbb",
      "go --2 stop",
      "go --0.5 stop",
      "-- first\nlast --",
      "line one\n\nline three",
      "a -- b -- c",
      "trailing space -- ",
    ]) {
      expect(roundTrip(s)).toBe(s);
    }
  });

  it("splits caesura chips with the same fence rules as the Rust parser", () => {
    // The chip token is the dashes(+digits) core only; fence spaces stay as text.
    expect(chips("aaaaa -- bbbbb")).toEqual(["--"]);
    expect(chips("go --2 stop")).toEqual(["--2"]);
    expect(chips("go --0.5 stop")).toEqual(["--0.5"]);
    // Line edges count as a fence.
    expect(chips("-- first\nlast --")).toEqual(["--", "--"]);
  });

  it("does NOT treat bullets, hyphenated words, or triple dashes as caesuras", () => {
    expect(chips("- a bullet line")).toEqual([]);
    expect(chips("well-known term")).toEqual([]);
    expect(chips("a --- b")).toEqual([]);
    expect(chips("a--b")).toEqual([]);
  });
});

describe("paste normalization", () => {
  it("collapses spacing around a fenced caesura to canonical single spaces", () => {
    expect(normalizePaste("a  --  b")).toBe("a -- b");
    expect(normalizePaste("x --2 y")).toBe("x --2 y");
  });

  it("leaves unfenced double-dashes literal", () => {
    expect(normalizePaste("word--word")).toBe("word--word");
    expect(normalizePaste("--start")).toBe("--start");
  });

  it("normalized paste then tokenizes into chips", () => {
    expect(chips(normalizePaste("a  --  b"))).toEqual(["--"]);
  });
});
