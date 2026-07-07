/**
 * A tiny ref-counted "is any modal open?" store.
 *
 * On Windows the program preview is a NATIVE GPU child window layered over the
 * webview, so it paints on top of any webview UI in its region — including a
 * centered modal dialog, which would otherwise be invisible behind it. Every
 * `PickerShell` modal registers here while mounted; the preview panel reads
 * this and hides the native overlay so the dialog is actually visible.
 */

let count = 0;
const listeners = new Set<(open: boolean) => void>();

function emit(): void {
  const open = count > 0;
  for (const listener of listeners) listener(open);
}

/** Register a modal as open; returns a disposer to call when it closes. */
export function pushModal(): () => void {
  count += 1;
  emit();
  let released = false;
  return () => {
    if (released) return;
    released = true;
    count -= 1;
    emit();
  };
}

export function modalSubscribe(listener: (open: boolean) => void): () => void {
  listeners.add(listener);
  return () => {
    listeners.delete(listener);
  };
}

export function isModalOpen(): boolean {
  return count > 0;
}
