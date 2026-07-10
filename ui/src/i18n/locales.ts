/**
 * The eighteen shipped locales, in the order the Havoc apps use them (English
 * first, then alphabetical by code). Freally File Manager and Freally Sourcerer
 * ship the same list — keep them in step.
 *
 * `native` is what the language picker shows: a speaker should recognise their
 * own language without reading English. `dir` drives `<html dir>`; Arabic is the
 * only right-to-left locale in the set.
 */
export type LocaleCode =
  | "en"
  | "ar"
  | "zh-CN"
  | "nl"
  | "fr"
  | "de"
  | "hi"
  | "id"
  | "it"
  | "ja"
  | "ko"
  | "pl"
  | "pt-BR"
  | "ru"
  | "es"
  | "tr"
  | "uk"
  | "vi";

export type Locale = {
  code: LocaleCode;
  /** The language's own name for itself. */
  native: string;
  dir: "ltr" | "rtl";
};

export const LOCALES: readonly Locale[] = [
  { code: "en", native: "English", dir: "ltr" },
  { code: "ar", native: "العربية", dir: "rtl" },
  { code: "zh-CN", native: "简体中文", dir: "ltr" },
  { code: "nl", native: "Nederlands", dir: "ltr" },
  { code: "fr", native: "Français", dir: "ltr" },
  { code: "de", native: "Deutsch", dir: "ltr" },
  { code: "hi", native: "हिन्दी", dir: "ltr" },
  { code: "id", native: "Bahasa Indonesia", dir: "ltr" },
  { code: "it", native: "Italiano", dir: "ltr" },
  { code: "ja", native: "日本語", dir: "ltr" },
  { code: "ko", native: "한국어", dir: "ltr" },
  { code: "pl", native: "Polski", dir: "ltr" },
  { code: "pt-BR", native: "Português (Brasil)", dir: "ltr" },
  { code: "ru", native: "Русский", dir: "ltr" },
  { code: "es", native: "Español", dir: "ltr" },
  { code: "tr", native: "Türkçe", dir: "ltr" },
  { code: "uk", native: "Українська", dir: "ltr" },
  { code: "vi", native: "Tiếng Việt", dir: "ltr" },
] as const;

/** The catalog every other locale falls back to, key by key. */
export const SOURCE_LOCALE: LocaleCode = "en";

/**
 * Persisted in `Settings.language` to mean "follow the operating system".
 * A real BCP-47 tag there means the user chose it explicitly and we honour it.
 * The Rust `validate()` rejects an empty tag, so the sentinel is a word.
 */
export const AUTO_LOCALE = "auto";

const BY_CODE = new Map(LOCALES.map((l) => [l.code.toLowerCase(), l]));

export function isLocaleCode(value: string): value is LocaleCode {
  return BY_CODE.has(value.toLowerCase());
}

export function localeDir(code: LocaleCode): "ltr" | "rtl" {
  return BY_CODE.get(code.toLowerCase())?.dir ?? "ltr";
}

/**
 * Map an OS/browser language tag onto one of the eighteen.
 *
 * Region matters for exactly two of them: `pt` collapses to `pt-BR` and any
 * Chinese to `zh-CN`, because those are the only variants we ship. Everything
 * else drops its region (`fr-CA` → `fr`). Unknown tags fall back to English
 * rather than throwing — a user with an unshipped locale still gets an app.
 */
export function normalizeLocale(tag: string): LocaleCode {
  const lower = tag.trim().toLowerCase();
  if (!lower) return SOURCE_LOCALE;
  if (isLocaleCode(lower)) return BY_CODE.get(lower)!.code;

  const base = lower.split("-")[0];
  if (base === "pt") return "pt-BR";
  if (base === "zh") return "zh-CN";
  if (isLocaleCode(base)) return BY_CODE.get(base)!.code;
  return SOURCE_LOCALE;
}

/** The first of the user's preferred languages that we actually ship. */
export function detectLocale(preferred: readonly string[]): LocaleCode {
  for (const tag of preferred) {
    const lower = tag.trim().toLowerCase();
    if (!lower) continue;
    // An exact or region-collapsed hit means the user really asked for it.
    if (isLocaleCode(lower)) return BY_CODE.get(lower)!.code;
    const base = lower.split("-")[0];
    if (base === "pt") return "pt-BR";
    if (base === "zh") return "zh-CN";
    if (isLocaleCode(base)) return BY_CODE.get(base)!.code;
  }
  return SOURCE_LOCALE;
}

/**
 * Resolve what `Settings.language` means. `"auto"` (or anything unshipped)
 * defers to the OS; an explicit tag wins.
 */
export function resolveLocale(setting: string, preferred: readonly string[]): LocaleCode {
  if (!setting || setting === AUTO_LOCALE) return detectLocale(preferred);
  return normalizeLocale(setting);
}
