//! tome: a Pixel Dungeon-like roguelike in Bevy. This binary only
//! assembles plugins and the cross-side ordering of their set labels.

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use core::frame_phase::FramePhase;

mod core;
mod frontend;

fn main() {
    // Assembly only: engine plugins, the two sides, and the frame-phase
    // ordering by abstract set labels. Concrete systems are owned and
    // ordered inside each side's register.
    App::new()
        .add_plugins(engine_plugins())
        .add_plugins((core::register, frontend::register))
        .configure_sets(
            Update,
            (FramePhase::Input, FramePhase::Game, FramePhase::Render).chain(),
        )
        .run();
}

/// Engine plugins with pixel-art settings: nearest-neighbor sampling and a
/// fixed 1280x720 window.
fn engine_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "tome".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        })
}
