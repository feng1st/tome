//! Sprite anchor policy: how a creature frame sits on its cell.

use bevy::prelude::*;
use bevy::sprite::Anchor;

/// The sprite's anchor: an odd-height frame centered on a cell center
/// would float its feet half a world pixel above the cell's bottom
/// edge, so odd axes get a half-pixel nudge (down on y, putting the
/// feet flush on the edge). Grid alignment itself is not the anchor's
/// business — the snap system lands every presented sprite's edges on the
/// screen grid whatever the offset's fractional part is.
pub fn sprite_anchor(frame_size: UVec2) -> Anchor {
    let size = frame_size.as_vec2();
    let center_shift = Vec2::new(
        if frame_size.x % 2 == 1 { 0.5 } else { 0.0 },
        if frame_size.y % 2 == 1 { -0.5 } else { 0.0 },
    );
    // Sprite centers sit at `transform − anchor × size`.
    Anchor(-center_shift / size)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The sprite center shift implied by an anchor.
    fn center_shift(frame_size: UVec2, anchor: Anchor) -> Vec2 {
        -anchor.0 * frame_size.as_vec2()
    }

    #[test]
    fn even_frames_stay_centered() {
        let shift = center_shift(UVec2::new(16, 16), sprite_anchor(UVec2::new(16, 16)));
        assert_eq!(shift, Vec2::ZERO);
    }

    #[test]
    fn odd_height_nudges_down_half_a_world_pixel() {
        // The warrior frame is 12x15.
        let shift = center_shift(UVec2::new(12, 15), sprite_anchor(UVec2::new(12, 15)));
        assert!((shift - Vec2::new(0.0, -0.5)).length() < 1e-6);
    }

    #[test]
    fn odd_width_nudges_half_a_world_pixel() {
        let shift = center_shift(UVec2::new(13, 16), sprite_anchor(UVec2::new(13, 16)));
        assert!((shift - Vec2::new(0.5, 0.0)).length() < 1e-6);
    }
}
