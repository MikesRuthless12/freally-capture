//! # fcap-script
//!
//! Sandboxed **Lua** scripting (Phase 7, TASK-703): a script reacts to studio
//! events (go-live, scene change, recording state) and drives the **same
//! command surface** the remote API exposes — the host hands the engine one
//! dispatch closure, so a script can do exactly what a controller can, and
//! nothing else.
//!
//! **Sandbox:** the Lua state loads only the safe stdlib slices (`math`,
//! `string`, `table`) — no `io`, no `os`, no `package`/`require`, no `debug`
//! — and the only doors out are the `fcap.*` functions below. Scripts run on
//! the host's scripting thread; a script error is contained (logged, the
//! script keeps running for later events unless load itself failed).
//!
//! ## The script API
//!
//! ```lua
//! fcap.log("hello")                       -- into the app log
//! fcap.on("streamStarted", function(data) -- react to an event
//!   fcap.command("setProgramScene", { scene = "Live" })
//! end)
//! local status = fcap.command("getStatus", {})
//! ```
//!
//! Events: `streamStarted`, `streamEnded`, `recordingStarted`,
//! `recordingStopped`, `sceneChanged` (data: `{ scene = "<id>" }`), plus the
//! raw `state` snapshot on every change.

#![forbid(unsafe_code)]

use std::sync::Arc;

use mlua::{Function, Lua, LuaSerdeExt, RegistryKey, StdLib, Value as LuaValue, Variadic};
use serde_json::Value;

/// The host's command dispatcher — the same allowlist the remote API serves.
pub type CommandFn = Arc<dyn Fn(&str, &Value) -> Result<Value, String> + Send + Sync>;

/// The host's log sink (script name is prepended by the host).
pub type LogFn = Arc<dyn Fn(&str) + Send + Sync>;

/// One loaded script: its sandboxed Lua state + registered event handlers.
pub struct Script {
    lua: Lua,
    /// (event name, handler) in registration order.
    handlers: Vec<(String, RegistryKey)>,
}

impl Script {
    /// Load + run `source` in a fresh sandboxed Lua state. `fcap.on`
    /// registrations made at load time become the script's event handlers.
    pub fn load(source: &str, command: CommandFn, log: LogFn) -> Result<Self, String> {
        // Safe stdlib slices only: math/string/table. No io/os/package/debug.
        let lua = Lua::new_with(
            StdLib::MATH | StdLib::STRING | StdLib::TABLE,
            mlua::LuaOptions::default(),
        )
        .map_err(|err| format!("lua init: {err}"))?;

        // Handlers land here during load; moved out after.
        let registered: Arc<std::sync::Mutex<Vec<(String, RegistryKey)>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));

        {
            let fcap = lua.create_table().map_err(err_str)?;

            let log_fn = {
                let log = Arc::clone(&log);
                lua.create_function(move |_, args: Variadic<String>| {
                    log(&args
                        .iter()
                        .map(String::as_str)
                        .collect::<Vec<_>>()
                        .join(" "));
                    Ok(())
                })
                .map_err(err_str)?
            };
            fcap.set("log", log_fn).map_err(err_str)?;

            let command_fn = {
                let command = Arc::clone(&command);
                lua.create_function(move |lua, (name, params): (String, Option<LuaValue>)| {
                    let params_json: Value = match params {
                        Some(value) => lua.from_value(value)?,
                        None => Value::Null,
                    };
                    match command(&name, &params_json) {
                        Ok(data) => lua.to_value(&data),
                        Err(error) => Err(mlua::Error::RuntimeError(error)),
                    }
                })
                .map_err(err_str)?
            };
            fcap.set("command", command_fn).map_err(err_str)?;

            let on_fn = {
                let registered = Arc::clone(&registered);
                lua.create_function(move |lua, (event, handler): (String, Function)| {
                    let key = lua.create_registry_value(handler)?;
                    registered
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .push((event, key));
                    Ok(())
                })
                .map_err(err_str)?
            };
            fcap.set("on", on_fn).map_err(err_str)?;

            let globals = lua.globals();
            // The Lua base library ships file loaders even without
            // `StdLib::IO` — close them; the sandbox test proves they're gone.
            globals.set("dofile", LuaValue::Nil).map_err(err_str)?;
            globals.set("loadfile", LuaValue::Nil).map_err(err_str)?;
            // `print` routes to the host log instead of raw stdout.
            globals
                .set("print", fcap.get::<Function>("log").map_err(err_str)?)
                .map_err(err_str)?;
            globals.set("fcap", fcap).map_err(err_str)?;
        }

        lua.load(source)
            .exec()
            .map_err(|err| format!("script error: {err}"))?;

        let handlers = std::mem::take(
            &mut *registered
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
        );
        Ok(Self { lua, handlers })
    }

    /// Deliver an event to every handler registered for it. A handler error
    /// is returned (for the host to log) but never unloads the script.
    pub fn emit(&self, event: &str, data: &Value) -> Result<(), String> {
        let mut first_error = None;
        for (name, key) in &self.handlers {
            if name != event {
                continue;
            }
            let handler: Function = self.lua.registry_value(key).map_err(err_str)?;
            let lua_data = self.lua.to_value(data).map_err(err_str)?;
            if let Err(err) = handler.call::<()>(lua_data) {
                first_error.get_or_insert(format!("handler for {event}: {err}"));
            }
        }
        match first_error {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    /// How many handlers listen for `event` (host diagnostics).
    pub fn handler_count(&self, event: &str) -> usize {
        self.handlers
            .iter()
            .filter(|(name, _)| name == event)
            .count()
    }
}

fn err_str(err: mlua::Error) -> String {
    err.to_string()
}

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    fn no_log() -> LogFn {
        Arc::new(|_| {})
    }

    #[test]
    fn a_script_reacts_to_go_live_and_switches_a_scene() {
        // The P7.3 accept criterion, in miniature: the sample script shape
        // registers for streamStarted and calls setProgramScene.
        let calls: Arc<Mutex<Vec<(String, Value)>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&calls);
        let command: CommandFn = Arc::new(move |name, params| {
            sink.lock().unwrap().push((name.to_owned(), params.clone()));
            Ok(Value::Null)
        });
        let script = Script::load(
            r#"
            fcap.on("streamStarted", function()
              fcap.command("setProgramScene", { scene = "Live" })
            end)
            "#,
            command,
            no_log(),
        )
        .expect("loads");
        assert_eq!(script.handler_count("streamStarted"), 1);
        script.emit("streamStarted", &Value::Null).expect("emits");
        let seen = calls.lock().unwrap();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, "setProgramScene");
        assert_eq!(seen[0].1["scene"], "Live");
    }

    #[test]
    fn event_data_round_trips_into_lua_and_back() {
        let calls: Arc<Mutex<Vec<Value>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&calls);
        let command: CommandFn = Arc::new(move |_name, params| {
            sink.lock().unwrap().push(params.clone());
            Ok(serde_json::json!({ "ok": true }))
        });
        let script = Script::load(
            r#"
            fcap.on("sceneChanged", function(data)
              local result = fcap.command("echo", { got = data.scene })
              assert(result.ok == true)
            end)
            "#,
            command,
            no_log(),
        )
        .expect("loads");
        script
            .emit("sceneChanged", &serde_json::json!({ "scene": "abc-123" }))
            .expect("emits");
        assert_eq!(calls.lock().unwrap()[0]["got"], "abc-123");
    }

    #[test]
    fn the_sandbox_has_no_io_os_or_require() {
        let command: CommandFn = Arc::new(|_, _| Ok(Value::Null));
        let script = Script::load(
            r#"
            assert(io == nil, "io must be absent")
            assert(os == nil, "os must be absent")
            assert(require == nil, "require must be absent")
            assert(dofile == nil, "dofile must be absent")
            assert(loadfile == nil, "loadfile must be absent")
            "#,
            command,
            no_log(),
        );
        assert!(script.is_ok(), "sandbox asserts failed: {:?}", script.err());
    }

    #[test]
    fn a_handler_error_is_reported_but_contained() {
        let command: CommandFn = Arc::new(|_, _| Err("refused".into()));
        let script = Script::load(
            r#"
            fcap.on("streamStarted", function()
              fcap.command("anything", {})
            end)
            fcap.on("streamStarted", function()
              -- the second handler still runs after the first errored
              fcap.log("still alive")
            end)
            "#,
            command,
            no_log(),
        )
        .expect("loads");
        let result = script.emit("streamStarted", &Value::Null);
        assert!(result.is_err(), "the dispatch error must surface");
        // The script object survives — later events still deliver.
        assert_eq!(script.handler_count("streamStarted"), 2);
    }

    #[test]
    fn a_broken_script_fails_to_load_with_the_error() {
        let command: CommandFn = Arc::new(|_, _| Ok(Value::Null));
        let result = Script::load("this is not lua(", command, no_log());
        assert!(result.is_err());
    }
}
