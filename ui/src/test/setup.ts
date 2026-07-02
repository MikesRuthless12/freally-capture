import "@testing-library/jest-dom/vitest";

import { cleanup } from "@testing-library/react";
import { afterEach } from "vitest";

// Vitest runs without injected globals, so Testing Library cannot register
// its automatic cleanup hook — do it explicitly.
afterEach(() => {
  cleanup();
});
