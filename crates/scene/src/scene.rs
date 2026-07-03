//! Scenes and the items they compose.
//!
//! A [`Scene`] is an ordered list of [`SceneItem`]s — index order **is** the
//! z-order, `items[0]` painted first (bottom-most). Items reference shared
//! [`crate::Source`]s by id and carry everything per-placement: transform,
//! blend mode, visibility, lock, and the ordered filter chain.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::filter::Filter;
use crate::source::SourceId;

/// Stable identity of a [`Scene`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SceneId(pub Uuid);

impl SceneId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SceneId {
    fn default() -> Self {
        Self::new()
    }
}

/// Stable identity of a [`SceneItem`] (unique within its collection).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ItemId(pub Uuid);

impl ItemId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ItemId {
    fn default() -> Self {
        Self::new()
    }
}

/// How an item's pixels combine with what's already on the canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BlendMode {
    /// Alpha-over (the ordinary case).
    #[default]
    Normal,
    Additive,
    Subtract,
    Screen,
    Multiply,
    Lighten,
    Darken,
}

impl BlendMode {
    /// Every mode, in UI order.
    pub const ALL: [BlendMode; 7] = [
        BlendMode::Normal,
        BlendMode::Additive,
        BlendMode::Subtract,
        BlendMode::Screen,
        BlendMode::Multiply,
        BlendMode::Lighten,
        BlendMode::Darken,
    ];
}

/// Pixels cut from each edge of the *source* (pre-scale, source px).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Crop {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

impl Crop {
    pub fn is_zero(&self) -> bool {
        *self == Crop::default()
    }
}

fn default_scale() -> f32 {
    1.0
}

/// Where and how an item sits on the canvas.
///
/// `x`/`y` are the canvas-pixel position of the item's **center** (of the
/// cropped content); `scale_*` are relative to the cropped source size;
/// `rotation` is degrees clockwise about that center. Center-based transforms
/// keep the on-canvas handle math exact under rotation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    /// Degrees, clockwise.
    pub rotation: f32,
    pub crop: Crop,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale_x: default_scale(),
            scale_y: default_scale(),
            rotation: 0.0,
            crop: Crop::default(),
        }
    }
}

/// Corner-slot geometry as fractions of the canvas. A slot spans [`SLOT_SIZE`]
/// of *each* axis (so on a 16:9 canvas it is itself 16:9 — a 16:9 camera fills
/// it edge to edge), inset from the edges by [`SLOT_MARGIN`]. Four such slots
/// never overlap each other or crowd the centered screen.
pub const SLOT_MARGIN: f32 = 0.02;
/// See [`SLOT_MARGIN`].
pub const SLOT_SIZE: f32 = 0.30;

/// A rectangle in normalized canvas coordinates — `0.0..=1.0` on each axis,
/// origin top-left. A layout stores one on a corner item as its
/// [`SceneItem::pending_slot`]; the engine turns it into pixels against the
/// live canvas size when the source's first frame arrives.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

/// One of the four corners the screen-plus-corners layout can drop a camera
/// into — the host and up to three guests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Corner {
    /// The corners in a natural host-first fill order (top-right, then the
    /// others) — how auto-assignment seats people.
    pub const FILL_ORDER: [Corner; 4] = [
        Corner::TopRight,
        Corner::TopLeft,
        Corner::BottomRight,
        Corner::BottomLeft,
    ];

    /// The normalized slot this corner fills (see [`SLOT_SIZE`]/[`SLOT_MARGIN`]).
    pub fn slot(self) -> NormRect {
        let far = 1.0 - SLOT_MARGIN - SLOT_SIZE;
        let (x, y) = match self {
            Corner::TopLeft => (SLOT_MARGIN, SLOT_MARGIN),
            Corner::TopRight => (far, SLOT_MARGIN),
            Corner::BottomLeft => (SLOT_MARGIN, far),
            Corner::BottomRight => (far, far),
        };
        NormRect {
            x,
            y,
            w: SLOT_SIZE,
            h: SLOT_SIZE,
        }
    }
}

fn default_visible() -> bool {
    true
}

/// One placement of a source in a scene.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneItem {
    #[serde(default)]
    pub id: ItemId,
    pub source: SourceId,
    #[serde(default = "default_visible")]
    pub visible: bool,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub blend: BlendMode,
    #[serde(default)]
    pub transform: Transform,
    /// One-shot: on the first frame whose size is known, fit-if-larger and
    /// center this item on the canvas, then clear the flag. Set for newly
    /// added items so a 4K display lands fitted instead of overflowing.
    #[serde(default)]
    pub pending_fit: bool,
    /// When set (and `pending_fit` is still true), the first-frame placement
    /// fits the source into this normalized canvas slot instead of the whole
    /// canvas — the corner-cam slots of a screen-plus-corners layout.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_slot: Option<NormRect>,
    #[serde(default)]
    pub filters: Vec<Filter>,
}

impl SceneItem {
    /// A new visible, unlocked item awaiting its first-frame fit.
    pub fn new(source: SourceId) -> Self {
        Self {
            id: ItemId::new(),
            source,
            visible: true,
            locked: false,
            blend: BlendMode::Normal,
            transform: Transform::default(),
            pending_fit: true,
            pending_slot: None,
            filters: Vec::new(),
        }
    }
}

/// One scene: a name + its ordered items (index = z-order, bottom first).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    #[serde(default)]
    pub id: SceneId,
    pub name: String,
    #[serde(default)]
    pub items: Vec<SceneItem>,
}

impl Scene {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: SceneId::new(),
            name: name.into(),
            items: Vec::new(),
        }
    }

    pub fn item(&self, id: ItemId) -> Option<&SceneItem> {
        self.items.iter().find(|item| item.id == id)
    }

    pub fn item_mut(&mut self, id: ItemId) -> Option<&mut SceneItem> {
        self.items.iter_mut().find(|item| item.id == id)
    }
}
