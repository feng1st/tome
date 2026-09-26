//! tome: a Pixel Dungeon-like roguelike in Bevy. This binary only
//! assembles plugins and the cross-side ordering of their set labels.

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use core::game_loop::GameLoop;

mod core;
mod frontend;

fn main() {
    // Assembly only: engine plugins, the two sides, and the game-loop
    // chain. Concrete systems are owned and ordered inside each side's
    // register; mode gating travels with each system as `run_if`.
    App::new()
        .add_plugins(engine_plugins())
        .add_plugins((core::register, frontend::register))
        .configure_sets(
            Update,
            (GameLoop::Input, GameLoop::Core, GameLoop::Display).chain(),
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
