import { useEffect, useState } from "react";

import {
  collectionCreate,
  collectionSwitch,
  collectionsList,
  profileCreate,
  profileSwitch,
  profilesList,
  type NamedList,
} from "../api/commands";
import type { Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

const inputClass =
  "min-w-0 flex-1 rounded-md border border-white/10 bg-havoc-panel px-2 py-1 text-xs text-havoc-text outline-none focus:border-havoc-accent/60";

/** One switchable-names column (profiles / scene collections). */
function NamedColumn({
  title,
  hint,
  list,
  onSwitch,
  onCreate,
}: {
  title: string;
  hint: string;
  list: NamedList | null;
  onSwitch: (name: string) => void;
  onCreate: (name: string) => void;
}) {
  const t = useT();
  const [draft, setDraft] = useState("");
  return (
    <div className="flex min-w-0 flex-1 flex-col gap-2">
      <p className="m-0 text-[11px] font-semibold tracking-wide text-havoc-muted uppercase">
        {title}
      </p>
      <ul className="m-0 flex max-h-48 list-none flex-col gap-1 overflow-auto p-0">
        {(list?.names ?? []).map((name) => {
          const active = name === list?.active;
          return (
            <li key={name}>
              <button
                type="button"
                onClick={() => !active && onSwitch(name)}
                title={active ? t("workspace-active") : t("workspace-switch-to", { name })}
                className={`w-full truncate rounded-md border px-2 py-1 text-left text-xs ${
                  active
                    ? "border-havoc-accent/50 bg-havoc-accent/10 text-havoc-text"
                    : "border-white/10 bg-white/[0.02] text-havoc-muted hover:text-havoc-text"
                }`}
              >
                {name}
                {active && (
                  <span className="ml-1.5 text-[10px] text-havoc-accent">
                    {t("workspace-active-marker")}
                  </span>
                )}
              </button>
            </li>
          );
        })}
      </ul>
      <div className="flex gap-2">
        <input
          value={draft}
          onChange={(event) => setDraft(event.target.value)}
          placeholder={t("workspace-new-name-placeholder")}
          aria-label={t("workspace-new-name-label", { title: title.toLowerCase() })}
          className={inputClass}
        />
        <button
          type="button"
          disabled={!draft.trim()}
          onClick={() => {
            onCreate(draft.trim());
            setDraft("");
          }}
          className="shrink-0 rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-2.5 py-1 text-xs font-semibold text-havoc-text enabled:hover:bg-havoc-accent/25 disabled:opacity-50"
        >
          {t("workspace-create")}
        </button>
      </div>
      <p className="m-0 text-[10px] leading-snug text-havoc-muted">{hint}</p>
    </div>
  );
}

/**
 * Profiles + scene collections (TASK-506). Switching saves the outgoing one
 * first — nothing is ever lost. A profile carries the settings; a collection
 * carries the scenes.
 */
export function WorkspaceDialog({
  onClose,
  onSettingsSaved,
}: {
  onClose: () => void;
  /** A profile switch replaces the live settings — the app re-renders them. */
  onSettingsSaved: (next: Settings) => void;
}) {
  const t = useT();
  const [profiles, setProfiles] = useState<NamedList | null>(null);
  const [collections, setCollections] = useState<NamedList | null>(null);
  const [error, setError] = useState<string | null>(null);

  const refresh = () => {
    profilesList()
      .then(setProfiles)
      .catch(() => {});
    collectionsList()
      .then(setCollections)
      .catch(() => {});
  };
  useEffect(refresh, []);

  const run = (work: Promise<unknown>) => {
    setError(null);
    work.then(refresh).catch((err) => setError(String(err)));
  };

  return (
    <PickerShell title={t("workspace-title")} onClose={onClose} wide>
      <div className="flex gap-4">
        <NamedColumn
          title={t("workspace-profiles")}
          hint={t("workspace-profiles-hint")}
          list={profiles}
          onSwitch={(name) =>
            run(profileSwitch(name).then((settings) => onSettingsSaved(settings)))
          }
          onCreate={(name) => run(profileCreate(name))}
        />
        <NamedColumn
          title={t("workspace-collections")}
          hint={t("workspace-collections-hint")}
          list={collections}
          onSwitch={(name) => run(collectionSwitch(name))}
          onCreate={(name) => run(collectionCreate(name))}
        />
      </div>
      {error && (
        <p role="alert" className="mt-2 mb-0 text-[11px] text-red-300">
          {error}
        </p>
      )}
    </PickerShell>
  );
}
