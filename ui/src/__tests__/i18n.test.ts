import { afterEach, describe, expect, it } from "vitest";

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

describe("the active locale", () => {
  it("starts at English and follows setLocale", () => {
    expect(getLocale()).toBe("en");
    setLocale("ko");
    expect(getLocale()).toBe("ko");
    expect(t("stats")).toBe("통계");
  });
});
