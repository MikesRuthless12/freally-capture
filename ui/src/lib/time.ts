/** `HH:MM:SS` (always three fields, rolls past an hour cleanly — `00:00:00`,
 * `01:02:03`) — the one clock format the LIVE and REC timers share. */
export function formatHms(totalSeconds: number): string {
  const whole = Math.max(0, Math.floor(totalSeconds));
  const hours = Math.floor(whole / 3600);
  const minutes = Math.floor((whole % 3600) / 60);
  const seconds = whole % 60;
  const pad = (value: number) => String(value).padStart(2, "0");
  return `${pad(hours)}:${pad(minutes)}:${pad(seconds)}`;
}
