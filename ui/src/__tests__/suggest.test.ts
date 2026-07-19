import { describe, expect, it } from "vitest";

import { indexDict, suggest } from "../lib/suggest";

// Words are frequency-ordered, so the first prefix hit is the "most common" one.
const dict = indexDict({
  words: ["the", "thanks", "thank", "thing", "hello", "help"],
  phrases: ["thank you", "thank you very much", "welcome back"],
});

describe("suggest (ghost-text autocomplete)", () => {
  it("completes the current partial word with the most common match", () => {
    expect(suggest(dict, "th")).toBe("e"); // the
    expect(suggest(dict, "tha")).toBe("nks"); // thanks (before thank in freq order)
    expect(suggest(dict, "hel")).toBe("lo"); // hello (before help)
  });

  it("returns null with too little typed, no match, or an exact word", () => {
    expect(suggest(dict, "t")).toBeNull(); // < 2 chars
    expect(suggest(dict, "xyz")).toBeNull(); // no bucket
    expect(suggest(dict, "start of a sentence zzz")).toBeNull();
  });

  it("continues a common phrase after a space", () => {
    expect(suggest(dict, "thank ")).toBe("you");
    expect(suggest(dict, "welcome ")).toBe("back");
  });

  it("never treats a caesura's dashes as a word to complete", () => {
    expect(suggest(dict, "hello -- ")).toBeNull();
  });

  it("returns null when no dictionary is loaded", () => {
    expect(suggest(null, "th")).toBeNull();
  });
});
