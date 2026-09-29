//! Canvas geometry: the offscreen resolution the world renders into
//! and the fixed upscale that presents it. The canvas tracks the
//! window (`canvas_size_for_window`): one world pixel maps to exactly
//! one canvas pixel, so rasterization itself snaps the world to the
//! canvas's texel grid; the fixed nearest upscale then puts one canvas
//! texel on two window pixels.

use bevy::camera::visibility::RenderLayers;

/// Margin per side, in texels. The canvas camera snaps to integer
/// pixels and the presentation pans by the sub-pixel remainder (at most
/// half a texel), so two texels of margin cover the pan with room to
/// spare; the ring holds valid world content that never enters the
/// presented view (it is cropped by the window edge).
pub const CANVAS_MARGIN: u32 = 2;

/// Fixed upscale from canvas to window: one canvas texel presents as a
/// 2x2 window-pixel block, always — a resized window resizes the view,
/// never the texel size.
pub const CANVAS_UPSCALE: f32 = 2.0;

/// The canvas layer: the whole world and the canvas camera. Same as the
/// engine default, declared for symmetry with `SCREEN_LAYERS` — world
/// entities stay unmarked and inherit it.
pub const CANVAS_LAYERS: RenderLayers = RenderLayers::layer(0);

/// The screen layer: the canvas sprite, the terrain layers beneath
/// it, and the screen camera — the only content the screen camera
/// sees.
pub const SCREEN_LAYERS: RenderLayers = RenderLayers::layer(1);
