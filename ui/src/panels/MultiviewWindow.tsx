import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

import { studioGet } from "../api/commands";
import { onStudio } from "../api/events";
import type { StudioDto } from "../api/types";
import { MultiviewDialog } from "./MultiviewDialog";

/**
 * The multiview grid as its own window on a display (CAP-M07 extension). Unlike
 * the chrome-free projector windows, this one drives IPC — it loads the studio
 * model (`studioGet` + the `studio` event) and clicking a cell cuts/stages a
 * scene — so its window label (`multiview`) is granted the `multiview`
 * capability. Esc or the card's close button closes the window.
 */
export function MultiviewWindow() {
  const [studio, setStudio] = useState<StudioDto | null>(null);

  useEffect(() => {
    let cancelled = false;
    studioGet()
      .then((dto) => {
        if (!cancelled) setStudio(dto);
      })
      .catch(() => undefined);
    const unlisten = onStudio((dto) => {
      if (!cancelled) setStudio(dto);
    }).catch(() => undefined);
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape") void getCurrentWindow().close();
    };
    window.addEventListener("keydown", onKey);
    return () => {
      cancelled = true;
      window.removeEventListener("keydown", onKey);
      void unlisten.then((fn) => fn?.());
    };
  }, []);

  return (
    <div className="fixed inset-0 overflow-auto bg-black p-4">
      <MultiviewDialog studio={studio} onClose={() => void getCurrentWindow().close()} />
    </div>
  );
}
