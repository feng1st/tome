//! new-tome2: a Pixel Dungeon-like roguelike in Bevy. This binary only
//! assembles plugins and the cross-domain system chain.

use bevy::prelude::*;

mod core;
mod graphic;
mod input;

fn main() {
    // Assembly only: engine plugins, core domains, the graphic plugin, and
    // the cross-domain per-frame chain. Everything else is registered by
    // the domains and plugins themselves.
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "new-tome2".into(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((core::register, graphic::register))
        // Assembly-level: cross-domain system ordering lives here. Input
        // intents are resolved by the core, the core moves positions, and
        // the graphic plugin derives all render state from them.
        .add_systems(
            Update,
            (
                input::systems::click::handle_click,
                core::hero::systems::resolve_goal::resolve_goal,
                core::movement::systems::follow_path::follow_path,
                graphic::sync::systems::sync_position::sync_position,
                graphic::animation::systems::animate::animate,
                graphic::camera::systems::follow_target::follow_target,
            )
                .chain(),
        )
        .run();
}
