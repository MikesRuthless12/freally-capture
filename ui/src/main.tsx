import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";

import App from "./App";
import { Projector } from "./panels/Projector";
import { TeleprompterProjector } from "./panels/Teleprompter";
import { MultiviewWindow } from "./panels/MultiviewWindow";
import { initLocale } from "./i18n/t";
import "./styles/global.css";

const root = document.getElementById("root");
if (!root) {
  throw new Error("index.html is missing #root");
}

// A projector window (CAP-M07) shares this bundle but renders only its clean
// feed. Its target is its window label; it has no settings, so it follows the OS
// language. Any non-Tauri context (tests import App directly) falls back to main.
const label = (() => {
  try {
    return getCurrentWindow().label;
  } catch {
    return "main";
  }
})();
const isProjector = label.startsWith("projector-");
// CAP-N58: the teleprompter projector renders the scrolling script (not a
// compositor feed), so it takes its own branch.
const isTeleprompter = label === "projector-teleprompter";
// The multiview-on-display window (CAP-M07 extension) shares this bundle too.
const isMultiview = label === "multiview";
if (isProjector || isMultiview) initLocale("auto");

createRoot(root).render(
  <StrictMode>
    {isTeleprompter ? (
      <TeleprompterProjector />
    ) : isProjector ? (
      <Projector label={label} />
    ) : isMultiview ? (
      <MultiviewWindow />
    ) : (
      <App />
    )}
  </StrictMode>,
);
