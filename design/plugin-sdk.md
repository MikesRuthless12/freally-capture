# The Plugin SDK (Phase 7, TASK-704)

Freally Capture plugins are **Rust crates** built against the documented
contracts in `crates/plugin` (`fcap-plugin`) and registered through **one
seam** — the `PluginRegistry`. Adding a plugin never touches a core crate:
the plugin implements the traits, exposes `register(&mut PluginRegistry)`,
and the app's plugin manifest calls it (one line + the Cargo dependency).

## The contracts (v1)

| Contract | What you implement | Where it runs |
|----------|--------------------|---------------|
| `PluginSource` | `kind()` (namespaced, e.g. `"vendor.thing"`), `label()`, `start(params) -> PluginFeed` | your feed is **pulled** at the studio's cadence — latest-wins, a slow plugin can never stall the render loop |
| `PluginFeed` | `next_frame() -> Option<PluginFrame>` (RGBA8, tightly packed) | the studio's source-upload stage |
| `PluginFilter` | `kind()`, `label()`, `apply(params, &mut frame)` | a **CPU RGBA transform at the frame boundary** — the built-in filter chain is GPU; plugin filters trade raw speed for zero-boilerplate authoring, said honestly |

Params are plain JSON (`serde_json::Value`), so a plugin versions its own
settings without core schema changes. Contract rules:

- **Validate `params` in `start` and error honestly — never panic.** A
  plugin must not be able to take the render loop down.
- `apply` must be total: bad params degrade to a no-op.
- Kinds are namespaced (`vendor.name`); duplicate kinds are refused at
  registration (first wins, the duplicate is reported).

## The sample

`plugins/checkerboard` is the complete, tested reference: an animated
checkerboard **source** + an **invert filter**, registered via its
`register()` — the crate's tests prove frames flow through the registry and
the filter transforms them, with zero core changes (the P7.4 accept case).

## Transitions + studio wiring — honest status

- **Transition contracts are not in v1.** Transitions are GPU passes inside
  the owned compositor (`shaders/transition.wgsl`); a CPU contract would
  fake it. They join the SDK when a GPU-plugin story exists.
- **Scene-model wiring is the named follow-on:** exposing registered plugin
  sources/filters in the pickers rides a single `SourceSettings::Plugin
  { kind, params }` variant + a registry-backed capture session. The
  contracts above are stable to build against now; the wiring is tracked in
  the roadmap (Phase 8 polish).
