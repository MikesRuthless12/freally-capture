import { useEffect, useState } from "react";

import { teleprompterGet } from "../api/commands";
import { onTeleprompter } from "../api/events";
import type { TeleprompterState } from "../api/types";

/** Subscribe to the shared teleprompter state (CAP-N58): the initial read plus
 * live updates on every control change. Kept in its own module so the surface
 * components (dock, projector) can share it without a component file exporting
 * a hook. */
export function useTeleprompter(): TeleprompterState | null {
  const [state, setState] = useState<TeleprompterState | null>(null);
  useEffect(() => {
    let alive = true;
    void teleprompterGet()
      .then((s) => alive && setState(s))
      .catch(() => undefined);
    const unlisten = onTeleprompter((s) => alive && setState(s));
    return () => {
      alive = false;
      void unlisten.then((fn) => fn());
    };
  }, []);
  return state;
}
