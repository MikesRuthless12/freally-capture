import { useEffect, type RefObject } from "react";

/**
 * The elements a browser will Tab to, in DOM order. `:not([disabled])` and the
 * `tabindex="-1"` exclusion matter: a disabled button and a programmatically
 * focusable div are both focusable in some sense, and neither is Tab-reachable.
 */
const FOCUSABLE = [
  "a[href]",
  "button:not([disabled])",
  "input:not([disabled]):not([type='hidden'])",
  "select:not([disabled])",
  "textarea:not([disabled])",
  "[tabindex]:not([tabindex='-1'])",
].join(",");

function focusableIn(container: HTMLElement): HTMLElement[] {
  return Array.from(container.querySelectorAll<HTMLElement>(FOCUSABLE)).filter(
    // `offsetParent === null` catches `display:none` ancestors. A hidden control
    // must not swallow a Tab, or the trap appears to skip a stop at random.
    (element) => element.offsetParent !== null || element === document.activeElement,
  );
}

/**
 * Trap Tab inside `ref` while `active`, and give focus back where it came from
 * on unmount (TASK-901).
 *
 * Without this a screen-reader or keyboard user Tabs straight out of an open
 * dialog and into the studio behind it — still visible, still clickable, but
 * `aria-modal` has told the assistive tech that everything back there is inert.
 * The user is then interacting with controls their screen reader refuses to
 * announce. `aria-modal` without a focus trap is worse than neither.
 *
 * Restoring focus matters just as much: a dialog closed with Escape must not
 * dump focus on `<body>`, or the next Tab starts from the top of the page.
 */
export function useFocusTrap(active: boolean, ref: RefObject<HTMLElement | null>): void {
  useEffect(() => {
    if (!active) return;
    const container = ref.current;
    if (!container) return;

    // Where focus was before we stole it. Captured on mount, not on close: by
    // then the element may be gone, and `document.activeElement` is the dialog.
    const previous = document.activeElement as HTMLElement | null;

    // Move focus in. Prefer the first real control; fall back to the container
    // itself so the dialog is at least announced.
    const initial = focusableIn(container)[0];
    if (initial) initial.focus();
    else {
      container.setAttribute("tabindex", "-1");
      container.focus();
    }

    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Tab") return;
      const focusable = focusableIn(container);
      if (focusable.length === 0) {
        event.preventDefault();
        return;
      }
      const first = focusable[0];
      const last = focusable[focusable.length - 1];
      const activeElement = document.activeElement;

      // Focus escaped the dialog entirely (a click on the backdrop, a stale
      // ref): pull it back rather than letting Tab wander into the studio.
      if (!container.contains(activeElement)) {
        event.preventDefault();
        first.focus();
        return;
      }
      if (event.shiftKey && activeElement === first) {
        event.preventDefault();
        last.focus();
      } else if (!event.shiftKey && activeElement === last) {
        event.preventDefault();
        first.focus();
      }
    };

    document.addEventListener("keydown", onKeyDown, true);
    return () => {
      document.removeEventListener("keydown", onKeyDown, true);
      // `isConnected` guards the common case: the thing that opened the dialog
      // was itself removed by the action that opened it.
      if (previous?.isConnected) previous.focus();
    };
  }, [active, ref]);
}
