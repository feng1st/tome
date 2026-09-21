//! new-tome2: a Pixel Dungeon-like roguelike in Bevy. This binary only
//! assembles plugins and the cross-side ordering of their set labels.

use bevy::prelude::*;

use core::sets::{CoreSet, GraphicSet, InputSet};

mod core;
mod graphic;
mod input;

fn main() {
    // Assembly only: engine plugins, the three sides, and the cross-side
    // ordering by abstract set labels. Concrete systems are owned and
    // ordered inside each side's register.
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
        .add_plugins((core::register, graphic::register, input::register))
        .configure_sets(Update, (InputSet, CoreSet, GraphicSet).chain())
        .run();
}
