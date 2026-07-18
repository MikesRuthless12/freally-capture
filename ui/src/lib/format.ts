/**
 * Shared human-readable formatters for byte sizes, rates, and durations, so the
 * Stats dock, Recordings, Models, and anywhere else render the same way (one
 * source of truth — no drifting per-panel copies).
 */

/** Bytes → a compact size (binary units): B / KB below a megabyte, then MB, then
 * GB at or above a gigabyte. */
export function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${Math.round(bytes)} B`;
  if (bytes < 1024 ** 2) return `${(bytes / 1024).toFixed(0)} KB`;
  const gb = bytes / 1024 ** 3;
  if (gb >= 1) return `${gb.toFixed(gb >= 100 ? 0 : gb >= 10 ? 1 : 2)} GB`;
  return `${(bytes / 1024 ** 2).toFixed(1)} MB`;
}

/** Bytes/second → a compact rate (binary units, MB/s). */
export function formatRate(bytesPerSec: number): string {
  return `${(bytesPerSec / (1024 * 1024)).toFixed(1)} MB/s`;
}

/** Seconds → a compact duration ("5h 12m" / "45m" / "30s"). */
export function formatDuration(secs: number): string {
  if (secs < 60) return `${Math.round(secs)}s`;
  const minutes = Math.floor(secs / 60);
  if (minutes < 60) return `${minutes}m`;
  return `${Math.floor(minutes / 60)}h ${minutes % 60}m`;
}
