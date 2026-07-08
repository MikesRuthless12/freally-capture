//! # fcap-plugin — the plugin SDK (Phase 7, TASK-704)
//!
//! The **documented contracts** a plugin implements to add a **source** or a
//! **filter** without touching core, and the **registry** that is the single
//! seam between plugins and the studio.
//!
//! A plugin is a Rust crate that implements [`PluginSource`] and/or
//! [`PluginFilter`] and registers into the app's [`PluginRegistry`] — one
//! registration line in the app's plugin manifest, **zero changes to the
//! core crates**. See `plugins/checkerboard` for the complete sample and
//! `design/plugin-sdk.md` for the guide.
//!
//! ## The contracts
//!
//! - **Source** ([`PluginSource`]): `kind()` names it (`"vendor.thing"`),
//!   `start(params)` returns a running [`PluginFeed`] that yields RGBA
//!   [`PluginFrame`]s on demand (pull-based; the studio pulls at its own
//!   cadence, latest-wins — a slow plugin can never stall the render loop).
//! - **Filter** ([`PluginFilter`]): `kind()` + `apply(params, frame)` — a
//!   CPU RGBA transform applied where a source's frames enter the engine.
//!   (The built-in filter chain is GPU; plugin filters run at the frame
//!   boundary by design — honest about the cost, simple to write.)
//!
//! Params are plain JSON (`serde_json::Value`) so plugins version their own
//! settings without core schema changes.

#![forbid(unsafe_code)]

use std::collections::HashMap;

use serde_json::Value;

/// One RGBA frame crossing the plugin boundary (8-bit, tightly packed).
pub struct PluginFrame {
    pub width: u32,
    pub height: u32,
    /// RGBA, `width * height * 4` bytes, row-major, no padding.
    pub rgba: Vec<u8>,
}

impl PluginFrame {
    /// A validated frame; `Err` when the buffer doesn't match the size.
    pub fn new(width: u32, height: u32, rgba: Vec<u8>) -> Result<Self, String> {
        let expected = (width as usize) * (height as usize) * 4;
        if width == 0 || height == 0 || rgba.len() != expected {
            return Err(format!(
                "plugin frame must be width*height*4 bytes (got {} for {width}x{height})",
                rgba.len()
            ));
        }
        Ok(Self {
            width,
            height,
            rgba,
        })
    }
}

/// A running source instance the studio pulls frames from.
pub trait PluginFeed: Send {
    /// The newest frame, or `None` when nothing new is ready (the studio
    /// keeps showing the last frame — latest-wins, never blocking).
    fn next_frame(&mut self) -> Option<PluginFrame>;
}

/// A source a plugin contributes. Implementations must be cheap to construct
/// (registration happens at startup); the work starts in [`Self::start`].
pub trait PluginSource: Send + Sync {
    /// The unique source kind, namespaced (`"vendor.checkerboard"`).
    fn kind(&self) -> &'static str;
    /// Human label for pickers.
    fn label(&self) -> &'static str;
    /// Start a feed with the given params (plugin-defined JSON shape).
    /// Validate params here and error honestly — never panic.
    fn start(&self, params: &Value) -> Result<Box<dyn PluginFeed>, String>;
}

/// A filter a plugin contributes: a CPU RGBA transform at the frame boundary.
pub trait PluginFilter: Send + Sync {
    /// The unique filter kind, namespaced (`"vendor.invert"`).
    fn kind(&self) -> &'static str;
    /// Human label for pickers.
    fn label(&self) -> &'static str;
    /// Transform the frame in place. Must be total — bad params degrade to
    /// a no-op, never a panic (a plugin can't take the render loop down).
    fn apply(&self, params: &Value, frame: &mut PluginFrame);
}

/// The registry — the single seam a plugin registers through. A plugin crate
/// exposes a `register(&mut PluginRegistry)` (see `plugins/checkerboard`);
/// duplicate kinds are refused (first one wins, the duplicate is reported) so
/// two plugins can't shadow each other. Wiring the registry into the studio's
/// source/filter pickers (a `SourceSettings::Plugin { kind, params }` variant +
/// registry-backed sessions) is the named follow-on these contracts are stable
/// for — see `design/plugin-sdk.md`; the contracts + sample ship now.
#[derive(Default)]
pub struct PluginRegistry {
    sources: HashMap<&'static str, Box<dyn PluginSource>>,
    filters: HashMap<&'static str, Box<dyn PluginFilter>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a source; `Err` on a duplicate kind.
    pub fn register_source(&mut self, source: Box<dyn PluginSource>) -> Result<(), String> {
        let kind = source.kind();
        if self.sources.contains_key(kind) {
            return Err(format!("duplicate plugin source kind: {kind}"));
        }
        self.sources.insert(kind, source);
        Ok(())
    }

    /// Register a filter; `Err` on a duplicate kind.
    pub fn register_filter(&mut self, filter: Box<dyn PluginFilter>) -> Result<(), String> {
        let kind = filter.kind();
        if self.filters.contains_key(kind) {
            return Err(format!("duplicate plugin filter kind: {kind}"));
        }
        self.filters.insert(kind, filter);
        Ok(())
    }

    pub fn source(&self, kind: &str) -> Option<&dyn PluginSource> {
        self.sources.get(kind).map(Box::as_ref)
    }

    pub fn filter(&self, kind: &str) -> Option<&dyn PluginFilter> {
        self.filters.get(kind).map(Box::as_ref)
    }

    /// Every registered source kind + label (for pickers), sorted.
    pub fn source_kinds(&self) -> Vec<(&'static str, &'static str)> {
        let mut kinds: Vec<_> = self
            .sources
            .values()
            .map(|source| (source.kind(), source.label()))
            .collect();
        kinds.sort();
        kinds
    }

    /// Every registered filter kind + label (for pickers), sorted.
    pub fn filter_kinds(&self) -> Vec<(&'static str, &'static str)> {
        let mut kinds: Vec<_> = self
            .filters
            .values()
            .map(|filter| (filter.kind(), filter.label()))
            .collect();
        kinds.sort();
        kinds
    }
}

/// This crate's version (inherited from the workspace).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    struct NullSource;
    impl PluginSource for NullSource {
        fn kind(&self) -> &'static str {
            "test.null"
        }
        fn label(&self) -> &'static str {
            "Null"
        }
        fn start(&self, _params: &Value) -> Result<Box<dyn PluginFeed>, String> {
            struct Empty;
            impl PluginFeed for Empty {
                fn next_frame(&mut self) -> Option<PluginFrame> {
                    None
                }
            }
            Ok(Box::new(Empty))
        }
    }

    #[test]
    fn duplicate_kinds_are_refused() {
        let mut registry = PluginRegistry::new();
        registry
            .register_source(Box::new(NullSource))
            .expect("first");
        assert!(registry.register_source(Box::new(NullSource)).is_err());
        assert_eq!(registry.source_kinds(), vec![("test.null", "Null")]);
    }

    #[test]
    fn frames_validate_their_dimensions() {
        assert!(PluginFrame::new(2, 2, vec![0; 16]).is_ok());
        assert!(PluginFrame::new(2, 2, vec![0; 15]).is_err());
        assert!(PluginFrame::new(0, 2, vec![]).is_err());
    }
}
