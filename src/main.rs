use bevy::prelude::*;

mod animation;
mod camera;
mod hero;
mod map;
mod movement;

fn main() {
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
        .add_plugins((map::register, hero::register, camera::register))
        // Assembly-level: cross-domain system ordering lives here.
        .add_systems(
            Update,
            (
                hero::systems::click::handle_click,
                movement::systems::follow_path::follow_path,
                animation::systems::animate::animate,
                camera::systems::follow_target::follow_target,
            )
                .chain(),
        )
        .run();
}
