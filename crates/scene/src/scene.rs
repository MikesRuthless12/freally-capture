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

/// How an item's pixels scale onto the canvas (CAP-N70) — the pixel-perfect
/// modes for retro/pixel content. `Auto` is the smooth bilinear every item
/// used before the setting existed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScaleMode {
    /// Smooth bilinear (the default).
    #[default]
    Auto,
    /// Nearest-neighbor: hard texel edges at any size.
    Nearest,
    /// Nearest **and** the drawn scale snapped to whole multiples ("3×") —
    /// every source pixel maps to an exact square of canvas pixels. The
    /// selection handles show the logical size; the pixels draw snapped.
    Integer,
    /// Scale up nearest-crisp, then a soft half-texel edge — crisp pixels
    /// without the shimmer of raw nearest under motion.
    SharpBilinear,
}

impl ScaleMode {
    /// Every mode, in UI order.
    pub const ALL: [ScaleMode; 4] = [
        ScaleMode::Auto,
        ScaleMode::Nearest,
        ScaleMode::Integer,
        ScaleMode::SharpBilinear,
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
    /// Degrees, clockwise (2D rotation in the screen plane).
    pub rotation: f32,
    pub crop: Crop,
    /// 3D tilt about the horizontal axis, degrees (CAP-N23). 0 = flat.
    pub rotation_x: f32,
    /// 3D tilt about the vertical axis, degrees (CAP-N23). 0 = flat.
    pub rotation_y: f32,
    /// Perspective strength for the 3D tilt, 0..=1 (0 = orthographic).
    pub perspective: f32,
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
            rotation_x: 0.0,
            rotation_y: 0.0,
            perspective: 0.0,
        }
    }
}

impl Transform {
    /// True when a 3D tilt is applied. The compositor then uses the projective
    /// matrix; a plain 2D transform keeps the exact affine path (so existing
    /// scenes render pixel-identically).
    pub fn has_3d(&self) -> bool {
        self.rotation_x != 0.0 || self.rotation_y != 0.0
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

/// Where a scene's backdrop wallpaper sits: the whole canvas (cover-fit,
/// overflow cropped — seamless edge-to-edge) or one half (the media
/// fit-contained in that half so the whole picture stays visible — the
/// critique layouts), leaving the other half to the capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackdropSplit {
    #[default]
    Full,
    Left,
    Right,
    Top,
    Bottom,
}

impl BackdropSplit {
    /// The normalized canvas region this mode fills.
    pub fn region(self) -> NormRect {
        let (x, y, w, h) = match self {
            BackdropSplit::Full => (0.0, 0.0, 1.0, 1.0),
            BackdropSplit::Left => (0.0, 0.0, 0.5, 1.0),
            BackdropSplit::Right => (0.5, 0.0, 0.5, 1.0),
            BackdropSplit::Top => (0.0, 0.0, 1.0, 0.5),
            BackdropSplit::Bottom => (0.0, 0.5, 1.0, 0.5),
        };
        NormRect { x, y, w, h }
    }

    /// The other half — where a split seats the capture. `None` for Full
    /// (the capture re-fits to the whole canvas instead).
    pub fn opposite(self) -> Option<NormRect> {
        match self {
            BackdropSplit::Full => None,
            BackdropSplit::Left => Some(BackdropSplit::Right.region()),
            BackdropSplit::Right => Some(BackdropSplit::Left.region()),
            BackdropSplit::Top => Some(BackdropSplit::Bottom.region()),
            BackdropSplit::Bottom => Some(BackdropSplit::Top.region()),
        }
    }
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

/// Margin around the auto-grid (CAP-N59) and the gap between its cells.
const GRID_MARGIN: f32 = 0.02;
const GRID_GAP: f32 = 0.015;
/// The most participants an auto-grid arranges (CAP-N59's 1–9 gallery).
pub const MAX_GRID: usize = 9;

/// A reflowing 1–9 participant grid (CAP-N59): `n` equal, non-overlapping cells
/// filling the canvas (inset by [`GRID_MARGIN`], separated by [`GRID_GAP`]).
/// Columns = ⌈√n⌉; a short final row is centered. Deterministic and pure — the
/// non-overlap invariant is unit-tested.
pub fn grid_seats(n: usize) -> Vec<NormRect> {
    let n = n.clamp(1, MAX_GRID);
    let cols = (n as f32).sqrt().ceil() as usize;
    let rows = n.div_ceil(cols);
    let cell_w = (1.0 - 2.0 * GRID_MARGIN - (cols as f32 - 1.0) * GRID_GAP) / cols as f32;
    let cell_h = (1.0 - 2.0 * GRID_MARGIN - (rows as f32 - 1.0) * GRID_GAP) / rows as f32;
    let step_x = cell_w + GRID_GAP;
    let step_y = cell_h + GRID_GAP;
    let last_row_start = (rows - 1) * cols;
    let last_row_count = n - last_row_start;
    let mut seats = Vec::with_capacity(n);
    for i in 0..n {
        let row = i / cols;
        let col = i % cols;
        // Center a short final row so it sits under the middle of the grid.
        let row_count = if row == rows - 1 {
            last_row_count
        } else {
            cols
        };
        let row_offset = (cols - row_count) as f32 * step_x / 2.0;
        seats.push(NormRect {
            x: GRID_MARGIN + row_offset + col as f32 * step_x,
            y: GRID_MARGIN + row as f32 * step_y,
            w: cell_w.max(0.01),
            h: cell_h.max(0.01),
        });
    }
    seats
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
    /// Morph (CAP-N20): items present in BOTH scenes animate their transform
    /// from the outgoing to the incoming layout instead of cutting; items in
    /// only one scene fade in/out. Composited item-by-item (`render_move`),
    /// not a two-texture blend.
    Move,
}

/// How a stinger video carries its transparency (CAP-N29 track matte). Most
/// stinger files are opaque or straight-alpha; a **track matte** packs the fill
/// (color) and its matte (a grayscale alpha channel) into one frame — the fill
/// in the first half, the matte's luminance in the second — so per-pixel
/// transparency survives codecs that drop alpha (H.264/HEVC). The fill/matte
/// split is the After-Effects convention: fill first, matte second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StingerMatte {
    /// The frame is used as-is (opaque, or its own straight alpha).
    #[default]
    None,
    /// Fill in the LEFT half, matte (luma → alpha) in the RIGHT half.
    Horizontal,
    /// Fill in the TOP half, matte (luma → alpha) in the BOTTOM half.
    Vertical,
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

/// A custom alignment guide line the user dragged out (CAP-M04 follow-on):
/// a straight line at a fixed canvas-pixel position that dragged items snap to.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GuideOrientation {
    /// A vertical line at a constant x.
    V,
    /// A horizontal line at a constant y.
    H,
}

/// One custom guide line, in canvas pixels (CAP-M04 follow-on).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuideLine {
    pub orientation: GuideOrientation,
    /// x for a vertical line, y for a horizontal one.
    pub position: f32,
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
    /// Per-output visibility (CAP-N53): whether this item is composed into
    /// the live outputs (stream lanes, incl. the virtual camera and Freally
    /// Link). Master `visible` still gates everything; this only *further*
    /// hides the item from that output. The operator surfaces (preview,
    /// projectors, multiview) always show the full program.
    #[serde(default = "default_visible")]
    pub on_stream: bool,
    /// Per-output visibility (CAP-N53): whether this item is composed into
    /// the local-disk outputs (the recorder — program/vertical/alpha lanes —
    /// and the replay buffer). See `on_stream`.
    #[serde(default = "default_visible")]
    pub on_record: bool,
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
    /// Pixel-perfect scaling (CAP-N70): how this item's pixels reach the
    /// canvas. `Auto` = the ordinary smooth bilinear.
    #[serde(default)]
    pub scaling: ScaleMode,
    /// Present on a scene's backdrop wallpaper (at most one per scene, always
    /// `items[0]`): which canvas region it fills. The compositor lays it out
    /// itself every frame — cover-fit for [`BackdropSplit::Full`], fit-contain
    /// for a half — reading this item's `transform` only as zoom (`scale_x`)
    /// and pan (`x`/`y`) *within* that region, clamped so blank canvas never
    /// shows. Pinned: reorder is a no-op and nothing moves below it, so it can
    /// never sit above (or otherwise interfere with) a capture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backdrop: Option<BackdropSplit>,
    /// Show/hide fade-in (CAP-N21): when this item is made visible (an eye
    /// toggle or a rule), its opacity ramps 0→1 over this many ms. `0` (the
    /// default) appears instantly, exactly as before.
    #[serde(default)]
    pub reveal_ms: u32,
}

impl SceneItem {
    /// A new visible, unlocked item awaiting its first-frame fit.
    pub fn new(source: SourceId) -> Self {
        Self {
            id: ItemId::new(),
            source,
            visible: true,
            on_stream: true,
            on_record: true,
            locked: false,
            blend: BlendMode::Normal,
            transform: Transform::default(),
            pending_fit: true,
            pending_slot: None,
            filters: Vec::new(),
            scaling: ScaleMode::default(),
            backdrop: None,
            reveal_ms: 0,
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
    /// Custom alignment guides the user dragged out (CAP-M04 follow-on).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guides: Vec<GuideLine>,
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
            guides: Vec::new(),
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
