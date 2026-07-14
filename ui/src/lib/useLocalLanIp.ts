import { useEffect, useState } from "react";

import { localLanIp } from "../api/commands";

/** CAP-N11: this machine's LAN IP for the connect URL — null until resolved
 * (or if the probe fails; the caller shows a placeholder). */
export function useLocalLanIp(): string | null {
  const [host, setHost] = useState<string | null>(null);
  useEffect(() => {
    let cancelled = false;
    localLanIp()
      .then((ip) => {
        if (!cancelled) setHost(ip);
      })
      .catch(() => {});
    return () => {
      cancelled = true;
    };
  }, []);
  return host;
}
