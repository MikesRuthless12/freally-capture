#!/usr/bin/env node
// Headless smoke for the CEF OSR backend: run the built host against a data: URL
// for a moment, close its stdin, and assert it emits the FBH1 header + at least
// one full frame with the geometry we asked for. Exits non-zero on any failure.
import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";

const exe = process.platform === "win32" ? "freally-browser-host.exe" : "freally-browser-host";
const bin = join(process.cwd(), "target", "release", exe);
if (!existsSync(bin)) {
  console.error(`cef-smoke: built host not found at ${bin}`);
  process.exit(1);
}

const W = 200,
  H = 120,
  FPS = 10;

// The `is_allowed_url` gate only permits http/https, so serve a trivial local
// page over http — deterministic and offline, and it exercises the real gate.
import { createServer } from "node:http";
const server = createServer((_req, res) => {
  res.writeHead(200, { "content-type": "text/html" });
  res.end("<body style='background:#336699'>freally cef smoke</body>");
});

server.listen(0, "127.0.0.1", () => {
  const port = server.address().port;
  const child = spawn(
    bin,
    [
      "--url",
      `http://127.0.0.1:${port}/`,
      "--width",
      String(W),
      "--height",
      String(H),
      "--fps",
      String(FPS),
      "--cef",
      process.env.CEF_ROOT || join(process.cwd(), ".cef-sdk"),
    ],
    { stdio: ["pipe", "pipe", "inherit"] },
  );

  const chunks = [];
  child.stdout.on("data", (d) => chunks.push(d));

  // Give CEF a few seconds to init + paint, then close stdin for a clean exit.
  setTimeout(() => child.stdin.end(), 6000);

  child.on("close", (code) => {
    server.close();
    const buf = Buffer.concat(chunks);
    const frameBytes = W * H * 4;
    if (buf.length < 16) {
      console.error(`cef-smoke: no header (got ${buf.length} bytes)`);
      process.exit(1);
    }
    const magic = buf.subarray(0, 4).toString("latin1");
    const w = buf.readUInt32LE(4);
    const h = buf.readUInt32LE(8);
    const fps = buf.readUInt32LE(12);
    const frames = Math.floor((buf.length - 16) / frameBytes);
    console.log(`cef-smoke: magic=${magic} ${w}x${h}@${fps}, ${frames} frame(s), exit ${code}`);
    if (magic !== "FBH1" || w !== W || h !== H || fps !== FPS) {
      console.error("cef-smoke: header did not match the request");
      process.exit(1);
    }
    if (frames < 1) {
      console.error("cef-smoke: no frame rendered");
      process.exit(1);
    }
    if (code !== 0) {
      console.error(`cef-smoke: host exited ${code}, expected 0 (clean stdin close)`);
      process.exit(1);
    }
    console.log("cef-smoke: OK");
  });
});
