//! Alpha blending for chunk rendering.

use bevy::sprite_render::AlphaMode2d;
use serde::Deserialize;

/// How a chunk's pixels blend. `Opaque` is the default; `Blend` is for
/// tiles with semi-transparent pixels (shoreline edges).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlphaMode {
    #[default]
    Opaque,
    Blend,
}

impl From<AlphaMode> for AlphaMode2d {
    fn from(mode: AlphaMode) -> Self {
        match mode {
            AlphaMode::Opaque => AlphaMode2d::Opaque,
            AlphaMode::Blend => AlphaMode2d::Blend,
        }
    }
}
