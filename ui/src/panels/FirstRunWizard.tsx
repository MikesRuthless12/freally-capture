import { useEffect, useState } from "react";

import {
  autoconfigSuggest,
  captureListSources,
  settingsCompleteOnboarding,
  settingsSet,
  studioAddItem,
} from "../api/commands";
import type { AutoConfig, CaptureSource, Settings } from "../api/types";
import { PickerShell } from "../components/PickerShell";
import { useT } from "../i18n/t";

type Step = "welcome" | "hardware" | "template" | "done";

/**
 * A starter scene. `needsDisplay` templates enumerate the real displays and use
 * the first — a template that hardcodes a capture id fails on the first click,
 * and capture ids are per-machine, per-session strings.
 */
type Template = {
  id: string;
  labelKey: string;
  noteKey: string;
  needsDisplay: boolean;
};

const TEMPLATES: Template[] = [
  {
    id: "screen",
    labelKey: "wizard-template-screen",
    noteKey: "wizard-template-screen-note",
    needsDisplay: true,
  },
  {
    id: "empty",
    labelKey: "wizard-template-empty",
    noteKey: "wizard-template-empty-note",
    needsDisplay: false,
  },
];

type FirstRunWizardProps = {
  settings: Settings;
  onSettingsSaved: (next: Settings) => void;
  /** The scene the templates add sources to; null disables the template step. */
  activeSceneId: string | null;
  onClose: () => void;
};

/**
 * The first-run wizard (TASK-905 + TASK-903). Probes the machine, proposes
 * encoder / fps / bitrate with a stated reason, and offers a starter scene.
 *
 * Skipping is a first-class outcome: it still records `completedOnboarding`, so
 * a user who dismissed the wizard is never greeted by it again. The one thing a
 * first-run experience must not do is nag.
 */
export function FirstRunWizard({
  settings,
  onSettingsSaved,
  activeSceneId,
  onClose,
}: FirstRunWizardProps) {
  const t = useT();
  const [step, setStep] = useState<Step>("welcome");
  const [config, setConfig] = useState<AutoConfig | null>(null);
  const [probeError, setProbeError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    if (step !== "hardware" || config || probeError) return;
    let alive = true;
    autoconfigSuggest()
      .then((next) => {
        if (alive) setConfig(next);
      })
      .catch((err) => {
        if (alive) setProbeError(String(err));
      });
    return () => {
      alive = false;
    };
  }, [step, config, probeError]);

  /** Finishing and skipping both record it — the wizard never returns. */
  const finish = () => {
    settingsCompleteOnboarding().catch((err) => console.error("onboarding record failed:", err));
    onClose();
  };

  const applyConfig = () => {
    if (!config) return;
    setBusy(true);
    const next: Settings = {
      ...settings,
      recording: {
        ...settings.recording,
        encoderId: config.encoderId,
        fps: config.fps,
      },
      stream: {
        ...settings.stream,
        targets: settings.stream.targets.map((target) => ({
          ...target,
          encoderId: config.encoderId,
          fps: config.fps,
          bitrateKbps: config.bitrateKbps,
        })),
      },
    };
    onSettingsSaved(next);
    settingsSet(next)
      .catch((err) => {
        onSettingsSaved(settings);
        setProbeError(String(err));
      })
      .finally(() => {
        setBusy(false);
        setStep("template");
      });
  };

  const applyTemplate = (template: Template) => {
    if (!activeSceneId || !template.needsDisplay) {
      setStep("done");
      return;
    }
    setBusy(true);
    captureListSources()
      .then((sources: CaptureSource[]) => {
        const display = sources.find((source) => source.kind === "display");
        // No display the OS will admit to (a headless CI box, a locked-down
        // Wayland session): finish rather than add a source that cannot start.
        if (!display) return undefined;
        return studioAddItem(activeSceneId, {
          kind: "display",
          captureId: display.id,
          label: display.label,
        }).then(() => undefined);
      })
      .catch((err) => setProbeError(String(err)))
      .finally(() => {
        setBusy(false);
        setStep("done");
      });
  };

  return (
    <PickerShell title={t("wizard-title")} onClose={finish} wide>
      <div className="flex flex-col gap-3 text-xs text-havoc-text">
        {step === "welcome" && (
          <>
            <p className="m-0 leading-snug">{t("wizard-welcome")}</p>
            <p className="m-0 text-[11px] leading-snug text-havoc-muted">
              {t("wizard-local-first")}
            </p>
            <Actions>
              <Secondary onClick={finish}>{t("wizard-skip")}</Secondary>
              <Primary onClick={() => setStep("hardware")}>{t("wizard-start")}</Primary>
            </Actions>
          </>
        )}

        {step === "hardware" && (
          <>
            <h3 className="m-0 text-[11px] tracking-wide text-havoc-muted uppercase">
              {t("wizard-hardware-title")}
            </h3>
            {probeError && (
              <p role="alert" className="m-0 text-[11px] text-red-300">
                {probeError}
              </p>
            )}
            {!config && !probeError && (
              <p className="m-0 text-havoc-muted">{t("wizard-probing")}</p>
            )}
            {config && (
              <>
                <dl className="m-0 grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-[11px]">
                  <dt className="text-havoc-muted">{t("wizard-encoder")}</dt>
                  <dd className="m-0 font-mono">{config.encoderLabel}</dd>
                  <dt className="text-havoc-muted">{t("wizard-canvas")}</dt>
                  <dd className="m-0 font-mono">
                    {config.width}×{config.height} @ {config.fps} fps
                  </dd>
                  <dt className="text-havoc-muted">{t("wizard-bitrate")}</dt>
                  <dd className="m-0 font-mono">{config.bitrateKbps} kbps</dd>
                </dl>
                <p className="m-0 text-[11px] leading-snug text-havoc-muted">
                  {t(config.encoderReason)} {t(config.qualityReason)}
                </p>
                <p className="m-0 text-[10px] leading-snug text-havoc-muted">
                  {t("wizard-probe-found", {
                    gpus: config.gpus.length ? config.gpus.join(", ") : t("wizard-no-gpu"),
                    cores: config.physicalCores,
                  })}
                </p>
                <Actions>
                  <Secondary onClick={() => setStep("template")}>
                    {t("wizard-keep-current")}
                  </Secondary>
                  <Primary onClick={applyConfig} disabled={busy}>
                    {t("wizard-apply")}
                  </Primary>
                </Actions>
              </>
            )}
          </>
        )}

        {step === "template" && (
          <>
            <h3 className="m-0 text-[11px] tracking-wide text-havoc-muted uppercase">
              {t("wizard-template-title")}
            </h3>
            <div className="flex flex-col gap-2">
              {TEMPLATES.map((template) => (
                <button
                  key={template.id}
                  type="button"
                  disabled={busy}
                  onClick={() => applyTemplate(template)}
                  className="rounded-md border border-white/10 bg-white/[0.03] px-3 py-2 text-left hover:border-havoc-accent/50 disabled:opacity-50"
                >
                  <span className="block text-xs font-semibold text-havoc-text">
                    {t(template.labelKey)}
                  </span>
                  <span className="block text-[10px] leading-snug text-havoc-muted">
                    {t(template.noteKey)}
                  </span>
                </button>
              ))}
            </div>
          </>
        )}

        {step === "done" && (
          <>
            <p className="m-0 leading-snug">{t("wizard-done")}</p>
            <p className="m-0 text-[11px] leading-snug text-havoc-muted">{t("wizard-done-hint")}</p>
            <Actions>
              <Primary onClick={finish}>{t("wizard-close")}</Primary>
            </Actions>
          </>
        )}
      </div>
    </PickerShell>
  );
}

function Actions({ children }: { children: React.ReactNode }) {
  return <div className="flex justify-end gap-2 border-t border-white/5 pt-2">{children}</div>;
}

function Primary({
  onClick,
  disabled,
  children,
}: {
  onClick: () => void;
  disabled?: boolean;
  children: React.ReactNode;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      disabled={disabled}
      className="rounded-md border border-havoc-accent/60 bg-havoc-accent/15 px-3 py-1.5 text-xs font-semibold text-havoc-text hover:bg-havoc-accent/25 disabled:opacity-50"
    >
      {children}
    </button>
  );
}

function Secondary({ onClick, children }: { onClick: () => void; children: React.ReactNode }) {
  return (
    <button
      type="button"
      onClick={onClick}
      className="rounded-md border border-white/10 px-3 py-1.5 text-xs text-havoc-muted hover:border-havoc-accent/50 hover:text-havoc-text"
    >
      {children}
    </button>
  );
}
