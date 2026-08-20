import { Component, type ErrorInfo, type ReactNode } from "react";

import { useT } from "../i18n/t";

/**
 * Contains a render error to one panel instead of the whole studio.
 *
 * Without this, a single throw unmounts the entire React tree and the operator
 * is left staring at a blank window — while the Rust side keeps encoding,
 * recording and streaming. That is the worst possible failure for a live tool:
 * not off-air, but blind and unable to stop. Wrapping the two shared shells
 * (`PickerShell`, `Panel`) covers essentially every dialog and dock, so a
 * crashed panel degrades to a readable message and the show carries on.
 *
 * A class component because `componentDidCatch` has no hook equivalent.
 */
type Props = {
  children: ReactNode;
  /** Shown instead of the children once a render has thrown. */
  fallback: (error: Error) => ReactNode;
};

type State = { error: Error | null };

export class ErrorBoundary extends Component<Props, State> {
  state: State = { error: null };

  static getDerivedStateFromError(error: Error): State {
    return { error };
  }

  componentDidCatch(error: Error, info: ErrorInfo) {
    // Kept on the console so a bug report can carry the component stack; there
    // is deliberately no IPC here, because the backend may be the thing that
    // just handed us the value we choked on.
    console.error("panel crashed:", error, info.componentStack);
  }

  render() {
    const { error } = this.state;
    if (error) return this.props.fallback(error);
    return this.props.children;
  }
}

/** The standard in-panel failure body: what happened, and what still works. */
export function PanelErrorFallback({ error }: { error: Error }) {
  const t = useT();
  return (
    <div role="alert" className="flex flex-col gap-1.5 p-3">
      <p className="m-0 text-xs font-semibold text-red-300">{t("panel-error-title")}</p>
      <p className="m-0 text-[11px] leading-snug text-havoc-muted">{t("panel-error-hint")}</p>
      <code className="m-0 text-[10px] leading-snug break-words text-havoc-muted">
        {String(error.message || error)}
      </code>
    </div>
  );
}
