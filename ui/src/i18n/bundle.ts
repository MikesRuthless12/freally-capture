import { FluentBundle, FluentResource } from "@fluent/bundle";

import { LOCALES, SOURCE_LOCALE, type LocaleCode } from "./locales";

/**
 * All eighteen catalogs, inlined at build time. `eager` because the studio must
 * paint in the right language on its first frame — a lazily-fetched catalog
 * would flash English first.
 */
const SOURCES = import.meta.glob("./locales/*.ftl", {
  query: "?raw",
  import: "default",
  eager: true,
}) as Record<string, string>;

/**
 * The "Central inside" panel's fcp-* catalogs, vendored from the
 * freally-central submodule and loaded into OUR bundles — the panel never owns
 * an i18n runtime, it renders through this app's `t`.
 */
const PANEL_SOURCES = import.meta.glob(
  "../../../vendor/freally-central/ui/src/panel/locales/*.ftl",
  {
    query: "?raw",
    import: "default",
    eager: true,
  },
) as Record<string, string>;

// A missing catalog is a build-time mistake (or an un-initialized submodule),
// not a runtime one — fail loudly with the actionable hint.
function ftlSource(sources: Record<string, string>, path: string, hint: string): string {
  const text = sources[path];
  if (text === undefined) throw new Error(hint);
  return text;
}

function sourceFor(code: LocaleCode): string {
  return ftlSource(
    SOURCES,
    `./locales/${code}.ftl`,
    `i18n: no catalog for "${code}" — expected ui/src/i18n/locales/${code}.ftl`,
  );
}

function panelSourceFor(code: LocaleCode): string {
  return ftlSource(
    PANEL_SOURCES,
    `../../../vendor/freally-central/ui/src/panel/locales/${code}.ftl`,
    `i18n: no Central-panel catalog for "${code}" — run \`git submodule update --init\``,
  );
}

/**
 * English is added to every bundle *first*, then the target locale overwrites
 * it. Fluent keeps the first definition of a key unless told otherwise, so
 * `allowOverrides` is what makes the target win — without it every string would
 * silently render in English. The layering is what lets a half-translated
 * catalog ship: a missing key falls back to English instead of showing its id.
 */
function buildBundle(code: LocaleCode): FluentBundle {
  // `useIsolating` wraps placeables in Unicode bidi marks. Right for prose in a
  // mixed-direction paragraph, wrong for a UI where the invisible characters
  // leak into `title` attributes and test assertions.
  const bundle = new FluentBundle(code, { useIsolating: false });

  if (code !== SOURCE_LOCALE) {
    bundle.addResource(new FluentResource(sourceFor(SOURCE_LOCALE)));
    bundle.addResource(new FluentResource(panelSourceFor(SOURCE_LOCALE)));
  }
  bundle.addResource(new FluentResource(sourceFor(code)), { allowOverrides: true });
  bundle.addResource(new FluentResource(panelSourceFor(code)), { allowOverrides: true });
  return bundle;
}

const CACHE = new Map<LocaleCode, FluentBundle>();

export function bundleFor(code: LocaleCode): FluentBundle {
  let bundle = CACHE.get(code);
  if (!bundle) {
    bundle = buildBundle(code);
    CACHE.set(code, bundle);
  }
  return bundle;
}

/** Every locale we ship a catalog for. Used by the parity test. */
export function loadedLocales(): LocaleCode[] {
  return LOCALES.map((l) => l.code);
}

/** The raw catalog text — for the parity test and the lint script. */
export function catalogSource(code: LocaleCode): string {
  return sourceFor(code);
}
