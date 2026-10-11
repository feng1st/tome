use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::core::game_loop::GameLoop;

mod core;
mod frontend;

fn main() {
    App::new()
        .add_plugins(engine_plugins())
        .add_plugins((core::register, frontend::register))
        .configure_sets(
            Update,
            (GameLoop::Input, GameLoop::Core, GameLoop::Display).chain(),
        )
        .run();
}

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
