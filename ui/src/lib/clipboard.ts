/**
 * The transform/filter clipboard (CAP-M05). An in-app store — a copied
 * transform or filter chain survives across scenes and scene-collection
 * switches within a session (it is not the OS clipboard). A tiny hand-rolled
 * store matching `lib/modal.ts`; the app has no state library.
 */

import { useSyncExternalStore } from "react";

import type { Filter, Transform } from "../api/types";

type Clipboard = {
  /** A copied item transform, or `null`. */
  transform: Transform | null;
  /** A copied filter chain, or `null`. */
  filters: Filter[] | null;
};

let clipboard: Clipboard = { transform: null, filters: null };
const listeners = new Set<() => void>();

function emit() {
  for (const listener of listeners) listener();
}

export function copyTransform(transform: Transform): void {
  clipboard = { ...clipboard, transform };
  emit();
}

export function copyFilters(filters: Filter[]): void {
  // Snapshot a copy so later edits to the source item don't mutate the clip.
  clipboard = { ...clipboard, filters: filters.map((filter) => ({ ...filter })) };
  emit();
}

export function clipboardSnapshot(): Clipboard {
  return clipboard;
}

function subscribe(listener: () => void): () => void {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

/** Subscribe a component to clipboard changes (so Paste buttons enable/disable). */
export function useClipboard(): Clipboard {
  return useSyncExternalStore(subscribe, clipboardSnapshot, clipboardSnapshot);
}
