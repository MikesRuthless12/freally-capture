// Ghost-text autocomplete engine for the teleprompter script editor. Each of the
// 18 app languages has its own dictionary (frequency-ordered common words +
// common phrases) under ../dict/<locale>.json, lazy-loaded for the ACTIVE locale
// only so the base bundle stays lean. Dictionaries can be very large (Mike wants
// the biggest free lists we can gather — up to hundreds of thousands of entries,
// reused by the standalone Freally Teleprompt), so on load we bucket entries by
// their first two characters: a lookup then scans only one small bucket, in
// frequency order, and the first prefix hit is the most common completion. Kept
// DOM-free so it's unit-testable and liftable into the clone.

/** The on-disk dictionary shape (what ../dict/<locale>.json contains). */
export type Dict = {
  /** Common words, most-frequent first (lowercase). */
  words: string[];
  /** Common phrases, most-frequent first (lowercase, space-separated). */
  phrases: string[];
};

/** An in-memory dictionary with first-two-char buckets for fast prefix lookup. */
export type LoadedDict = {
  wordIdx: Map<string, string[]>;
  phraseIdx: Map<string, string[]>;
};

function buildIndex(list: string[]): Map<string, string[]> {
  const idx = new Map<string, string[]>();
  for (const s of list) {
    if (s.length < 2) continue;
    const key = s.slice(0, 2); // entries are already lowercase in the data files
    const bucket = idx.get(key);
    if (bucket) bucket.push(s);
    else idx.set(key, [s]);
  }
  return idx;
}

/** Build the in-memory index for a dictionary (exposed for tests + reuse). */
export function indexDict(dict: Dict): LoadedDict {
  return { wordIdx: buildIndex(dict.words ?? []), phraseIdx: buildIndex(dict.phrases ?? []) };
}

// Vite bundles each dictionary as its own async chunk; we only fetch the one for
// the active locale.
const loaders = import.meta.glob<{ default: Dict }>("../dict/*.json");
const cache = new Map<string, LoadedDict | null>();
const pending = new Map<string, Promise<LoadedDict | null>>();

/** Lazy-load + index a locale's dictionary (cached). Returns null when the app
 * has no dictionary for that locale (the caller then shows no suggestions). */
export function loadDict(locale: string): Promise<LoadedDict | null> {
  const hit = cache.get(locale);
  if (hit !== undefined) return Promise.resolve(hit);
  const existing = pending.get(locale);
  if (existing) return existing;
  const loader = loaders[`../dict/${locale}.json`];
  if (!loader) {
    cache.set(locale, null);
    return Promise.resolve(null);
  }
  const p = loader()
    .then((m) => {
      const loaded = indexDict(m.default);
      cache.set(locale, loaded);
      pending.delete(locale);
      return loaded;
    })
    .catch(() => {
      cache.set(locale, null);
      pending.delete(locale);
      return null;
    });
  pending.set(locale, p);
  return p;
}

// A "word" is a run of Unicode letters/marks/apostrophes — deliberately NOT
// hyphens, so a caesura `--` token never looks like a partial word to complete.
const WORD_TAIL = /[\p{L}\p{M}']+$/u;
const PREV_WORD = /([\p{L}\p{M}']+)\s+$/u;

/** The ghost-text completion for the text immediately before the caret, or null.
 * Completes the current partial word; failing that, continues a common phrase
 * from the previous word. The returned string is what would be appended. */
export function suggest(dict: LoadedDict | null, before: string): string | null {
  if (!dict) return null;
  const tail = before.match(WORD_TAIL);
  if (tail) {
    const partial = tail[0];
    if (partial.length < 2) return null; // too little typed to guess usefully
    const lower = partial.toLowerCase();
    const bucket = dict.wordIdx.get(lower.slice(0, 2));
    if (!bucket) return null;
    for (const w of bucket) {
      if (w.length > lower.length && w.startsWith(lower)) return w.slice(lower.length);
    }
    return null;
  }
  // Caret sits just after a space: offer to continue a common phrase.
  const prev = before.match(PREV_WORD);
  if (prev) {
    const lead = prev[1].toLowerCase();
    const bucket = dict.phraseIdx.get(lead.slice(0, 2));
    if (!bucket) return null;
    const needle = `${lead} `;
    for (const p of bucket) {
      if (p.length > needle.length && p.startsWith(needle)) return p.slice(needle.length);
    }
  }
  return null;
}
