import { useMemo } from "react";
import qrcode from "qrcode-generator";

import { useT } from "../i18n/t";

/** A link as a QR code (TASK-R3) — vendored zero-dep encoder, drawn as a
 * plain SVG path (no innerHTML, CSP-safe). `ariaKey` names the payload for
 * screen readers (invite links by default; LAN ingest passes its own). */
export function QrSvg({
  link,
  ariaKey = "sources-invite-qr-aria",
}: {
  link: string;
  ariaKey?: string;
}) {
  const t = useT();
  const rendered = useMemo(() => {
    try {
      const qr = qrcode(0, "M"); // type 0 = auto-size for the payload
      qr.addData(link);
      qr.make();
      const count = qr.getModuleCount();
      let path = "";
      for (let row = 0; row < count; row += 1) {
        for (let col = 0; col < count; col += 1) {
          if (qr.isDark(row, col)) path += `M${col} ${row}h1v1h-1z`;
        }
      }
      return { count, path };
    } catch {
      return null; // an over-long payload — the copyable link still works
    }
  }, [link]);
  if (!rendered) return null;
  return (
    <svg
      viewBox={`0 0 ${rendered.count} ${rendered.count}`}
      role="img"
      aria-label={t(ariaKey)}
      className="h-28 w-28 shrink-0 rounded bg-white p-1.5"
    >
      <path d={rendered.path} fill="#000" />
    </svg>
  );
}
