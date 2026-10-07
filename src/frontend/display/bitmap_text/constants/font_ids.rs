//! The font id vocabulary.

/// The font ids in registry order: ascending `1x, 15x, 2x, 25x, 3x`.
/// The registry validates this order at load, and the zoom-matched
/// choice addresses fonts positionally against it.
pub const FONT_IDS: [&str; 5] = ["1x", "15x", "2x", "25x", "3x"];
