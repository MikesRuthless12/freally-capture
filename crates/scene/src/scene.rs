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

/// The six one-click seats — the four corners plus vertically centered
/// left/right. Same size and margins as the corner slots, so no two seats
/// ever overlap. Order = the bump-to-free fill order.
pub fn preset_seats() -> [NormRect; 6] {
    let far = 1.0 - SLOT_MARGIN - SLOT_SIZE;
    let mid = 0.5 - SLOT_SIZE / 2.0;
    [
        Corner::TopLeft.slot(),
        Corner::TopRight.slot(),
        NormRect {
            x: SLOT_MARGIN,
            y: mid,
            w: SLOT_SIZE,
            h: SLOT_SIZE,
        },
        NormRect {
            x: far,
            y: mid,
            w: SLOT_SIZE,
            h: SLOT_SIZE,
        },
        Corner::BottomLeft.slot(),
        Corner::BottomRight.slot(),
    ]
}

/// The centered shared view (a Desktop/Window capture or a promoted cam).
/// Sized so it never overlaps the right-hand cam rail: cams sit beside the
/// shared view, never on top of it.
pub fn center_slot() -> NormRect {
    NormRect {
        x: 0.02,
        y: 0.02,
        w: 0.74,
        h: 0.96,
    }
}

/// The right-hand cam rail — four 16:9 seats (host + up to three guests)
/// stacked beside the centered view. None overlap the center or each other.
pub fn rail_seats() -> [NormRect; 4] {
    let seat = |y: f32| NormRect {
        x: 0.78,
        y,
        w: 0.20,
        h: 0.20,
    };
    [seat(0.02), seat(0.27), seat(0.52), seat(0.77)]
}

/// Whether two normalized rects overlap (strictly — touching edges do not).
pub fn rects_overlap(a: NormRect, b: NormRect) -> bool {
    a.x + a.w > b.x && b.x + b.w > a.x && a.y + a.h > b.y && b.y + b.h > a.y
}

/// How Studio Mode commits Preview → Program. `Cut` is instant; the rest
/// blend the two composed scenes on the GPU over a set duration. The Phase 6
/// pack adds more built-in luma patterns, custom luma-wipe images, and the
/// stinger (a video played over the cut).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransitionKind {
    #[default]
    Cut,
    Fade,
    SlideLeft,
    SlideRight,
    SlideUp,
    SlideDown,
    SwipeLeft,
    SwipeRight,
    /// Built-in luma patterns.
    LumaLinear,
    LumaRadial,
    /// Phase 6 pack: more built-in wipe patterns.
    LumaHorizontal,
    LumaDiamond,
    LumaClock,
    /// A custom grayscale wipe image (Settings → transition's luma image).
    LumaImage,
    /// A video played over the cut (the scene swap lands at the configured
    /// cut point). Files with straight alpha (e.g. ProRes 4444) composite
    /// transparently; others draw opaque while they play.
    Stinger,
}

/// One item's pre-focus placement, restored exactly when focus toggles off.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusRestore {
    pub item: ItemId,
    pub transform: Transform,
    pub visible: bool,
    /// The item's remembered seat, restored with it (seat-swap reads seats).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_slot: Option<NormRect>,
}

/// Highlight Speaker (Focus/Spotlight): `item` is promoted to fill the whole
/// canvas while the other video items hide; `prior` holds the exact layout to
/// restore when focus toggles off.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusState {
    pub item: ItemId,
    pub prior: Vec<FocusRestore>,
}

fn default_visible() -> bool {
    true
}

/// Stable identity of a [`SourceGroup`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GroupId(pub Uuid);

impl GroupId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for GroupId {
    fn default() -> Self {
        Self::new()
    }
}

/// A named set of a scene's items that move / show / hide **together**
/// (Phase 6, TASK-605). Grouping is metadata over ordinary items — z-order
/// and per-item settings stay untouched; an item belongs to at most one
/// group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceGroup {
    #[serde(default)]
    pub id: GroupId,
    pub name: String,
    #[serde(default)]
    pub items: Vec<ItemId>,
    /// Group visibility ANDs with each member's own eye toggle.
    #[serde(default = "default_visible")]
    pub visible: bool,
}

/// One source's per-scene mixer override (Phase 6, TASK-605): while this
/// scene is the program, these replace the source's global fader/mute — a
/// scene can sound different from the global mix.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneAudioOverride {
    pub source: SourceId,
    pub volume_db: f32,
    pub muted: bool,
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
    /// Highlight Speaker: present while one item is promoted to fill the
    /// canvas (Focus/Spotlight); absent in a normal layout.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus: Option<FocusState>,
    /// Source groups (Phase 6): items that move/show/hide together.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<SourceGroup>,
    /// Per-scene mixer overrides (Phase 6): applied while this scene is the
    /// program.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audio_overrides: Vec<SceneAudioOverride>,
}

impl Scene {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: SceneId::new(),
            name: name.into(),
            items: Vec::new(),
            focus: None,
            groups: Vec::new(),
            audio_overrides: Vec::new(),
        }
    }

    pub fn item(&self, id: ItemId) -> Option<&SceneItem> {
        self.items.iter().find(|item| item.id == id)
    }

    pub fn item_mut(&mut self, id: ItemId) -> Option<&mut SceneItem> {
        self.items.iter_mut().find(|item| item.id == id)
    }

    pub fn group(&self, id: GroupId) -> Option<&SourceGroup> {
        self.groups.iter().find(|group| group.id == id)
    }

    /// The group `item` belongs to, if any (an item is in at most one).
    pub fn group_of(&self, item: ItemId) -> Option<&SourceGroup> {
        self.groups.iter().find(|group| group.items.contains(&item))
    }

    /// Whether a group's eye toggle hides `item` (ANDs with the item's own).
    pub fn group_hides(&self, item: ItemId) -> bool {
        self.group_of(item).is_some_and(|group| !group.visible)
    }

    /// Drop dangling member ids and empty groups (after item removals).
    pub fn prune_groups(&mut self) {
        let live: Vec<ItemId> = self.items.iter().map(|item| item.id).collect();
        for group in &mut self.groups {
            group.items.retain(|id| live.contains(id));
        }
        self.groups.retain(|group| !group.items.is_empty());
    }
}
