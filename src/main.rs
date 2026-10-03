//! tome: a tile-based roguelike in Bevy. This binary only
//! assembles plugins and the cross-side ordering of their set labels.

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::core::game_loop::GameLoop;

mod core;
mod diag;
mod frontend;

fn main() {
    // Assembly only: engine plugins, the two sides, and the game-loop
    // chain. Concrete systems are owned and ordered inside each side's
    // register; mode gating travels with each system as `run_if`.
    App::new()
        .add_plugins(engine_plugins())
        .add_plugins((core::register, frontend::register))
        // TEMP(诊断)
        .init_resource::<diag::DiagFrame>()
        .add_systems(PreUpdate, diag::diag_frame_tick)
        .add_systems(Update, diag::autodrive.in_set(GameLoop::Input))
        .add_systems(
            Update,
            diag::creature_probe
                .in_set(crate::frontend::display::display_phase::DisplayPhase::Snap),
        )
        .configure_sets(
            Update,
            (GameLoop::Input, GameLoop::Core, GameLoop::Display).chain(),
        )
        .run();
}

/// Engine plugins with pixel-art settings: nearest-neighbor sampling and a
/// 1280x720 window (resizable).
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
