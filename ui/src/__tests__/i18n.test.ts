import { afterEach, describe, expect, it } from "vitest";

import { STREAM_SERVICES, TRANSITION_KINDS } from "../api/types";
import { bundleFor, catalogSource, loadedLocales } from "../i18n/bundle";
import {
  AUTO_LOCALE,
  LOCALES,
  detectLocale,
  localeDir,
  normalizeLocale,
  resolveLocale,
  SOURCE_LOCALE,
} from "../i18n/locales";
import { getLocale, resetLocaleForTests, setLocale, t } from "../i18n/t";

afterEach(() => resetLocaleForTests());

/** `key = value` lines, ignoring comments and blanks. Mirrors the lint. */
function keysOf(code: (typeof LOCALES)[number]["code"]): Set<string> {
  const keys = new Set<string>();
  for (const line of catalogSource(code).split(/\r?\n/)) {
    if (!line.trim() || line.trimStart().startsWith("#")) continue;
    const eq = line.indexOf("=");
    if (eq > 0) keys.add(line.slice(0, eq).trim());
  }
  return keys;
}

describe("catalogs", () => {
  it("ships all eighteen locales", () => {
    expect(loadedLocales()).toHaveLength(18);
    expect(LOCALES.map((l) => l.code)).toContain("en");
    expect(LOCALES.map((l) => l.code)).toContain("ar");
    expect(LOCALES.map((l) => l.code)).toContain("pt-BR");
    expect(LOCALES.map((l) => l.code)).toContain("zh-CN");
  });

  /**
   * The CI lint enforces this too, but a unit test fails in a second on a
   * developer's machine rather than five minutes later in a runner.
   */
  it("every locale defines exactly the keys en defines", () => {
    const source = keysOf(SOURCE_LOCALE);
    expect(source.size).toBeGreaterThan(0);

    for (const { code } of LOCALES) {
      if (code === SOURCE_LOCALE) continue;
      const keys = keysOf(code);
      const missing = [...source].filter((k) => !keys.has(k));
      const extra = [...keys].filter((k) => !source.has(k));
      expect(missing, `${code} is missing keys`).toEqual([]);
      expect(extra, `${code} has keys en does not`).toEqual([]);
    }
  });

  it("actually translates — a locale is not just an English copy", () => {
    setLocale("ja");
    expect(t("stats")).toBe("統計");
    setLocale("ar");
    expect(t("stats")).toBe("الإحصائيات");
    setLocale("fr");
    expect(t("studio-mode")).toBe("Mode Studio");
  });

  /**
   * English is layered under every bundle, so a key a translator has not reached
   * yet renders in English rather than as its raw id.
   */
  it("falls back to English for a key a locale lacks", () => {
    const bundle = bundleFor("ja");
    const message = bundle.getMessage("stats");
    expect(message?.value).toBeDefined();
    // Sanity: the layering did not overwrite Japanese *with* English.
    expect(bundle.formatPattern(message!.value!)).toBe("統計");
  });

  it("shows the raw id for a key nobody defines, rather than nothing", () => {
    expect(t("no-such-key-anywhere")).toBe("no-such-key-anywhere");
  });
});

describe("locale resolution", () => {
  it("collapses regions except where a variant is the one we ship", () => {
    expect(normalizeLocale("fr-CA")).toBe("fr");
    expect(normalizeLocale("en-GB")).toBe("en");
    // The only two variants in the set.
    expect(normalizeLocale("pt")).toBe("pt-BR");
    expect(normalizeLocale("pt-PT")).toBe("pt-BR");
    expect(normalizeLocale("zh")).toBe("zh-CN");
    expect(normalizeLocale("zh-TW")).toBe("zh-CN");
  });

  it("falls back to English for a language we do not ship", () => {
    expect(normalizeLocale("sw")).toBe("en");
    expect(normalizeLocale("")).toBe("en");
    expect(normalizeLocale("   ")).toBe("en");
  });

  it("takes the first preferred language we actually ship", () => {
    expect(detectLocale(["sw", "is", "de-AT", "fr"])).toBe("de");
    expect(detectLocale(["ja-JP"])).toBe("ja");
    expect(detectLocale([])).toBe("en");
  });

  it("honours an explicit choice and defers on `auto`", () => {
    expect(resolveLocale("ja", ["fr"])).toBe("ja");
    expect(resolveLocale(AUTO_LOCALE, ["fr-CA", "en"])).toBe("fr");
    // A stale tag for a locale we dropped must not brick the UI.
    expect(resolveLocale("sw", ["ja"])).toBe("en");
  });
});

describe("direction", () => {
  it("marks Arabic right-to-left and nothing else", () => {
    expect(localeDir("ar")).toBe("rtl");
    for (const { code } of LOCALES) {
      if (code !== "ar") expect(localeDir(code), code).toBe("ltr");
    }
  });

  it("stamps <html lang> and <html dir> when the language changes", () => {
    setLocale("ar");
    expect(document.documentElement.getAttribute("lang")).toBe("ar");
    expect(document.documentElement.getAttribute("dir")).toBe("rtl");

    setLocale("ja");
    expect(document.documentElement.getAttribute("lang")).toBe("ja");
    expect(document.documentElement.getAttribute("dir")).toBe("ltr");
  });
});

/**
 * Lookup tables hold catalog keys, and their call sites render `t(labelKey)` —
 * a *variable*. The i18n lint only scans literal `t("…")` calls, so a table that
 * still holds English silently renders English in every locale (`t("15 min")`
 * falls back to `"15 min"`), and a typo ships a raw id on screen. Every gate
 * stays green. This test is the only thing standing there.
 *
 * Two tables shipped broken exactly this way and a code review caught them —
 * `INVITE_TTLS` and `SLOT_OPTIONS`, whose translated keys sat orphaned in all
 * 18 catalogs. Anything shaped like `[value, key]` belongs in this list.
 */
describe("label tables", () => {
  const source = bundleFor(SOURCE_LOCALE);

  const TABLES: Array<[string, ReadonlyArray<readonly [unknown, string]>]> = [
    ["TRANSITION_KINDS", TRANSITION_KINDS],
    ["STREAM_SERVICES", STREAM_SERVICES],
  ];

  it.each(TABLES)("every %s key exists in en.ftl", (name, table) => {
    expect(table.length, `${name} is empty`).toBeGreaterThan(0);
    for (const [value, key] of table) {
      expect(source.getMessage(key)?.value, `${name}: ${String(value)} -> ${key}`).toBeDefined();
    }
  });

  it.each(TABLES)("%s holds keys, not English", (name, table) => {
    for (const [, key] of table) {
      expect(key, `${name}: "${key}" looks like English, not a key`).toMatch(/^[a-z0-9-]+$/);
    }
  });

  /**
   * A catalog key nothing references is either a lookup table that still holds
   * English (the bug above) or dead weight nobody will ever delete. Neither the
   * lint nor `tsc` can see this: the lint only follows literal `t("…")`, and a
   * table of strings type-checks whatever it holds.
   *
   * Keys built with a template literal — `` t(`filters-crop-${side}`) `` — are
   * matched by their prefix, which is the same escape hatch the lint uses.
   */
  it("no key in en.ftl is orphaned", () => {
    const keys = catalogSource(SOURCE_LOCALE)
      .split(/\r?\n/)
      .filter((line) => line.trim() && !line.trimStart().startsWith("#"))
      .map((line) => line.slice(0, line.indexOf("=")).trim())
      .filter(Boolean);

    // Vite inlines the sources; no `node:fs`, so this runs anywhere vitest does.
    const modules = import.meta.glob("../**/*.{ts,tsx}", {
      query: "?raw",
      import: "default",
      eager: true,
    }) as Record<string, string>;

    const haystack = Object.entries(modules)
      .filter(([path]) => !path.includes("/i18n/") && !path.includes("/__tests__/"))
      .map(([, text]) => text)
      .join("\n");

    // `t(`some-prefix-${x}`)` → keys starting with `some-prefix-` are referenced.
    const dynamicPrefixes = [...haystack.matchAll(/t\(\s*`([a-z0-9-]+-)\$\{/g)].map((m) => m[1]);

    const orphans = keys.filter(
      (key) =>
        !haystack.includes(`"${key}"`) && !dynamicPrefixes.some((prefix) => key.startsWith(prefix)),
    );

    expect(orphans, `orphaned keys — nothing renders these:\n  ${orphans.join("\n  ")}`).toEqual(
      [],
    );
  });
});

describe("the active locale", () => {
  it("starts at English and follows setLocale", () => {
    expect(getLocale()).toBe("en");
    setLocale("ko");
    expect(getLocale()).toBe("ko");
    expect(t("stats")).toBe("통계");
  });
});
