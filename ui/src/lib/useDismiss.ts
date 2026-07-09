import { useEffect, useRef, type RefObject } from "react";

/**
 * Close an open popover when the user clicks outside it or presses Escape.
 *
 * `ref` must wrap **both** the trigger and the popover. If it wrapped only the
 * popover, a click on the trigger would dismiss (pointerdown, outside) and then
 * immediately re-open (click, toggle) — the menu would look stuck.
 *
 * Listens on `pointerdown` rather than `click` so the menu is gone before the
 * click lands on whatever sits underneath it, and in the capture phase so a
 * child calling `stopPropagation` cannot strand the menu open.
 */
export function useDismiss(
  open: boolean,
  ref: RefObject<HTMLElement | null>,
  onDismiss: () => void,
): void {
  // Hold the latest callback so the listeners below don't re-subscribe on every
  // render of a large parent. Synced in an effect, never written during render.
  const dismiss = useRef(onDismiss);
  useEffect(() => {
    dismiss.current = onDismiss;
  }, [onDismiss]);

  useEffect(() => {
    if (!open) return;

    const onPointerDown = (event: PointerEvent) => {
      const el = ref.current;
      if (el && event.target instanceof Node && !el.contains(event.target)) {
        dismiss.current();
      }
    };
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Escape") return;
      // Escape closes the innermost thing only. These menus live inside a
      // `PickerShell`, which also closes on Escape (a bubble-phase listener on
      // `window`). Stopping propagation here — in the capture phase, before the
      // event reaches it — means one Escape closes the menu, and a second
      // closes the dialog. Without this, both would close at once.
      event.stopPropagation();
      dismiss.current();
    };

    document.addEventListener("pointerdown", onPointerDown, true);
    document.addEventListener("keydown", onKeyDown, true);
    return () => {
      document.removeEventListener("pointerdown", onPointerDown, true);
      document.removeEventListener("keydown", onKeyDown, true);
    };
  }, [open, ref]);
}
