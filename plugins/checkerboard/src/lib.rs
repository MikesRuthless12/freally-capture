//! The sample plugin (TASK-704): an animated **checkerboard source** and an
//! **invert filter**, implemented entirely against the `fcap-plugin`
//! contracts — no core crate is touched. The app's plugin manifest calls
//! [`register`]; that one line is the plugin's entire integration.

#![forbid(unsafe_code)]

use fcap_plugin::{PluginFeed, PluginFilter, PluginFrame, PluginRegistry, PluginSource};
use serde_json::Value;

/// Register everything this plugin provides.
pub fn register(registry: &mut PluginRegistry) -> Result<(), String> {
    registry.register_source(Box::new(Checkerboard))?;
    registry.register_filter(Box::new(Invert))?;
    Ok(())
}

/// An animated checkerboard. Params (all optional):
/// `{ "width": 640, "height": 360, "cell": 40 }`.
struct Checkerboard;

impl PluginSource for Checkerboard {
    fn kind(&self) -> &'static str {
        "sample.checkerboard"
    }
    fn label(&self) -> &'static str {
        "Checkerboard (sample plugin)"
    }
    fn start(&self, params: &Value) -> Result<Box<dyn PluginFeed>, String> {
        let dim = |key: &str, default: u32, max: u32| -> Result<u32, String> {
            match params.get(key) {
                None => Ok(default),
                Some(value) => match value.as_u64() {
                    Some(n) if (1..=max as u64).contains(&n) => Ok(n as u32),
                    _ => Err(format!("bad {key}: expected 1..={max}")),
                },
            }
        };
        Ok(Box::new(CheckerFeed {
            width: dim("width", 640, 4096)?,
            height: dim("height", 360, 4096)?,
            cell: dim("cell", 40, 512)?,
            tick: 0,
        }))
    }
}

struct CheckerFeed {
    width: u32,
    height: u32,
    cell: u32,
    tick: u64,
}

impl PluginFeed for CheckerFeed {
    fn next_frame(&mut self) -> Option<PluginFrame> {
        self.tick = self.tick.wrapping_add(1);
        let phase = (self.tick % 2) as u32; // the animation: parity flips
        let mut rgba = Vec::with_capacity((self.width * self.height * 4) as usize);
        for y in 0..self.height {
            for x in 0..self.width {
                let on = ((x / self.cell) + (y / self.cell) + phase) % 2 == 0;
                let v = if on { 230 } else { 25 };
                rgba.extend_from_slice(&[v, v, v, 255]);
            }
        }
        PluginFrame::new(self.width, self.height, rgba).ok()
    }
}

/// Inverts RGB (alpha kept). Params: none (anything is ignored).
struct Invert;

impl PluginFilter for Invert {
    fn kind(&self) -> &'static str {
        "sample.invert"
    }
    fn label(&self) -> &'static str {
        "Invert (sample plugin)"
    }
    fn apply(&self, _params: &Value, frame: &mut PluginFrame) {
        for px in frame.rgba.chunks_exact_mut(4) {
            px[0] = 255 - px[0];
            px[1] = 255 - px[1];
            px[2] = 255 - px[2];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// The P7.4 accept case: the plugin registers a source + a filter through
    /// the SDK seam and frames flow — with zero changes to any core crate.
    #[test]
    fn the_sample_plugin_adds_a_source_and_filter_without_touching_core() {
        let mut registry = PluginRegistry::new();
        register(&mut registry).expect("registers");

        let source = registry.source("sample.checkerboard").expect("listed");
        let mut feed = source
            .start(&json!({ "width": 8, "height": 8, "cell": 4 }))
            .expect("starts");
        let mut frame = feed.next_frame().expect("a frame");
        assert_eq!((frame.width, frame.height), (8, 8));
        let before = frame.rgba[0];

        let filter = registry.filter("sample.invert").expect("listed");
        filter.apply(&Value::Null, &mut frame);
        assert_eq!(frame.rgba[0], 255 - before, "the filter transformed pixels");
        assert_eq!(frame.rgba[3], 255, "alpha untouched");
    }

    #[test]
    fn hostile_params_error_honestly_instead_of_panicking() {
        let mut registry = PluginRegistry::new();
        register(&mut registry).expect("registers");
        let source = registry.source("sample.checkerboard").expect("listed");
        assert!(source.start(&json!({ "width": 0 })).is_err());
        assert!(source.start(&json!({ "width": 1_000_000 })).is_err());
        assert!(source.start(&json!({ "cell": "huge" })).is_err());
        // Absent params = defaults.
        assert!(source.start(&Value::Null).is_ok());
    }

    #[test]
    fn the_animation_actually_animates() {
        let mut registry = PluginRegistry::new();
        register(&mut registry).expect("registers");
        let source = registry.source("sample.checkerboard").expect("listed");
        let mut feed = source
            .start(&json!({ "width": 8, "height": 8, "cell": 4 }))
            .expect("starts");
        let first = feed.next_frame().expect("frame 1");
        let second = feed.next_frame().expect("frame 2");
        assert_ne!(first.rgba, second.rgba, "parity flips per frame");
    }
}
