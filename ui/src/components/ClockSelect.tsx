import { useT } from "../i18n/t";

/**
 * A 12-hour wall-clock picker over a 24-hour `"HH:MM"` value — hours 1–12,
 * minutes in 5-minute steps, AM/PM. Shared by the Starting Soon form and the
 * Timer properties so the tricky 12↔24-hour conversion (12 AM = 00, 12 PM =
 * 12) lives in exactly one place.
 */
export function ClockSelect({
  value,
  onChange,
  selectClass,
}: {
  /** The 24-hour "HH:MM" target ("" and malformed parse as 12:00 PM). */
  value: string;
  onChange: (next: string) => void;
  /** The host surface's select styling (form vs properties). */
  selectClass: string;
}) {
  const t = useT();
  const match = /^(\d{1,2}):(\d{2})$/.exec(value.trim());
  const h24 = match ? Math.min(23, Number(match[1])) : 12;
  // The stored minute is shown EXACTLY (a legacy "19:37" target must never be
  // silently rounded); the dropdown offers 5-minute steps plus, when needed,
  // the stored off-step minute itself.
  const minute = match ? Math.min(59, Number(match[2])) : 0;
  const minuteOptions = Array.from({ length: 12 }, (_, i) => i * 5);
  if (!minuteOptions.includes(minute)) {
    minuteOptions.push(minute);
    minuteOptions.sort((a, b) => a - b);
  }
  const h12 = h24 % 12 === 0 ? 12 : h24 % 12;
  const ap: "AM" | "PM" = h24 >= 12 ? "PM" : "AM";
  const write = (nextH12: number, nextMin: number, nextAp: "AM" | "PM") => {
    const next24 =
      nextAp === "PM" ? (nextH12 === 12 ? 12 : nextH12 + 12) : nextH12 === 12 ? 0 : nextH12;
    onChange(`${String(next24).padStart(2, "0")}:${String(nextMin).padStart(2, "0")}`);
  };
  return (
    <div className="flex items-center gap-1.5">
      <select
        value={h12}
        onChange={(event) => write(Number(event.target.value), minute, ap)}
        aria-label={t("sources-starting-soon-hours")}
        className={selectClass}
      >
        {Array.from({ length: 12 }, (_, i) => i + 1).map((h) => (
          <option key={h} value={h}>
            {h}
          </option>
        ))}
      </select>
      <span className="text-havoc-muted">:</span>
      <select
        value={minute}
        onChange={(event) => write(h12, Number(event.target.value), ap)}
        aria-label={t("sources-starting-soon-minutes")}
        className={selectClass}
      >
        {minuteOptions.map((m) => (
          <option key={m} value={m}>
            {String(m).padStart(2, "0")}
          </option>
        ))}
      </select>
      <select
        value={ap}
        onChange={(event) => write(h12, minute, event.target.value as "AM" | "PM")}
        aria-label="AM/PM"
        className={selectClass}
      >
        <option value="AM">AM</option>
        <option value="PM">PM</option>
      </select>
    </div>
  );
}
