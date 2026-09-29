//! Canvas geometry: the fixed offscreen resolution the world renders
//! into and the fixed upscale that presents it. One world pixel maps to
//! exactly one canvas pixel, so rasterization itself snaps the world to
//! the canvas's texel grid; window size and compositor scaling only
//! affect the nearest-neighbor upscale afterwards.

use bevy::camera::visibility::RenderLayers;

/// The view size presented on screen, in canvas texels (world pixels).
pub const CANVAS_VIEW_WIDTH: u32 = 640;
pub const CANVAS_VIEW_HEIGHT: u32 = 360;

/// Margin per side, in texels. The canvas camera snaps to integer
/// pixels and the presentation pans by the sub-pixel remainder (at most
/// half a texel), so two texels of margin cover the pan with room to
/// spare; the ring holds valid world content that never enters the view.
pub const CANVAS_MARGIN: u32 = 2;

/// Canvas size in pixels: the view plus the margin ring.
pub const CANVAS_WIDTH: u32 = CANVAS_VIEW_WIDTH + 2 * CANVAS_MARGIN;
pub const CANVAS_HEIGHT: u32 = CANVAS_VIEW_HEIGHT + 2 * CANVAS_MARGIN;

/// Fixed upscale from canvas to window. The canvas presents at this
/// factor, centered: larger windows show borders, smaller windows crop.
pub const CANVAS_UPSCALE: f32 = 2.0;

/// The canvas layer: the whole world and the canvas camera. Same as the
/// engine default, declared for symmetry with `SCREEN_LAYERS` — world
/// entities stay unmarked and inherit it.
pub const CANVAS_LAYERS: RenderLayers = RenderLayers::layer(0);

/// The screen layer: the canvas sprite and the screen camera, the only
/// content the screen camera sees.
pub const SCREEN_LAYERS: RenderLayers = RenderLayers::layer(1);
