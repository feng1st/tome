//! The frontend: everything player-facing — world rendering, input
//! modalities, and later the HUD. Replaceable as a whole (e.g. with a text
//! frontend) without touching the core; replaceable units inside it
//! (tilesets, animations, HUD, modalities) are directories, not plugins.

pub mod display;
pub mod input;

use bevy::prelude::*;

/// Register the frontend's sub-areas. Systems land in the game-loop stages
/// owned by the core (`GameLoop::Input` at frame start, `GameLoop::Display` at frame end);
/// the orchestration layer only chains the labels.
pub fn register(app: &mut App) {
    display::register(app);
    input::register(app);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The display assembly builds headless: engine plugins minus the
    /// window and renderer, both registers, and the FromWorld builders
    /// the registers trigger — the figure table parses, every font
    /// font's texture decodes and splits, validation passes. Running
    /// frames headless stops at the render-integrated tile chunks
    /// (their insert hooks need the renderer), so the animated boot is
    /// the manual visual pass; this test pins the load path.
    #[test]
    fn the_display_assembly_builds_headless() {
        use bevy::app::TaskPoolPlugin;
        use bevy::asset::AssetPlugin;
        use bevy::image::TextureAtlasLayout;
        use bevy::state::app::StatesPlugin;
        use bevy::time::TimePlugin;

        let mut app = App::new();
        app.add_plugins((
            TaskPoolPlugin::default(),
            TimePlugin,
            StatesPlugin,
            AssetPlugin::default(),
        ))
        // The asset stores the registries write into, minus the
        // renderer and its pipelines: the font registry registers its
        // self-decoded fonts as images, the figure registry builds
        // atlas layouts. Nothing here needs the pixels to render.
        .init_asset::<Image>()
        .init_resource::<Assets<TextureAtlasLayout>>()
        .add_plugins((crate::core::register, register));
        let figure_registry = app
            .world()
            .get_resource::<display::figure::resources::figure_registry::FigureRegistry>()
            .expect("the figure registry built");
        assert!(figure_registry.get_index("warrior").is_some());
        let font_registry = app
            .world()
            .get_resource::<display::bitmap_text::resources::font_registry::FontRegistry>()
            .expect("the font registry built");
        assert_eq!(font_registry.fonts().len(), 5);
    }
}
