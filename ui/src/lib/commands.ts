/**
 * The command-palette model and its matcher (TASK-904).
 *
 * Lives outside the component so Fast Refresh keeps working (a module that
 * exports both a component and helpers loses it), and so the matcher can be
 * tested without rendering anything.
 */

/** One thing the palette can do. `keywords` widen the match beyond the label. */
export type Command = {
  id: string;
  /** Already translated — the palette never calls `t` on caller strings. */
  label: string;
  group: string;
  keywords?: string;
  run: () => void;
};

/**
 * Subsequence match: every character of the query appears in the haystack, in
 * order, not necessarily adjacent. So `srcw` finds "Source: Window Capture".
 *
 * Cheap and forgiving. Spaces in the query are ignored so "add win" behaves like
 * "addwin" rather than demanding a literal space in the target.
 */
export function matches(query: string, haystack: string): boolean {
  if (!query) return true;
  const needle = query.toLowerCase();
  const hay = haystack.toLowerCase();
  let at = 0;
  for (const char of needle) {
    if (char === " ") continue;
    at = hay.indexOf(char, at);
    if (at === -1) return false;
    at += 1;
  }
  return true;
}

export function filterCommands(commands: readonly Command[], query: string): Command[] {
  if (!query.trim()) return [...commands];
  return commands.filter((command) =>
    matches(query, `${command.group} ${command.label} ${command.keywords ?? ""}`),
  );
}
