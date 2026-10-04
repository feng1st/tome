//! Snaps every presented sprite onto the screen grid.

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;

use crate::frontend::display::camera::components::main_camera::MainCamera;
use crate::frontend::display::camera::components::sprite_size::SpriteSize;
use crate::frontend::display::camera::utils::screen_grid::{
    min_corner_offset, snap_translation, world_scale_factor,
};
use crate::frontend::display::constants::layout::ZOOM;
use crate::frontend::display::map::utils::coords::cell_to_world;
use crate::frontend::display::motion::components::curr_position::CurrPosition;

/// Snap every entity with a presentation (`CurrPosition` + sprite parts) to
/// the screen grid, in camera-relative space. Runs in the Snap phase,
/// after the camera snapped: the camera defines the lattice sprites
/// snap to. This system is the single enforcement point of the snap
/// discipline — a presented sprite that bypasses it presents fractional
/// and risks frame-boundary misreads (stray lines). z is presentation
/// layering owned by the display side and preserved here.
pub fn snap_sprites(
    camera: Query<&Transform, With<MainCamera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&CurrPosition, &Anchor, &SpriteSize, &mut Transform), Without<MainCamera>>,
) {
    let Ok(camera_transform) = camera.single() else {
        return;
    };
    let world_scale_factor = windows.single().map(world_scale_factor).unwrap_or(ZOOM);
    let camera_position = camera_transform.translation.truncate();
    for (curr_position, anchor, sprite_size, mut transform) in &mut query {
        let world = cell_to_world(*curr_position);
        let snapped = snap_translation(
            world,
            camera_position,
            min_corner_offset(anchor, sprite_size.size()),
            world_scale_factor,
        );
        transform.translation.x = snapped.x;
        transform.translation.y = snapped.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::map::components::cell_coord::CellCoord;

    fn app() -> App {
        let mut app = App::new();
        app.add_systems(Update, snap_sprites);
        app.world_mut().spawn((Transform::default(), MainCamera));
        app
    }

    /// The presented sprite's min corner in camera-relative screen pixels.
    fn min_in_screen_pixels(
        transform: &Transform,
        camera: Vec2,
        offset: Vec2,
        world_scale_factor: f32,
    ) -> Vec2 {
        (transform.translation.truncate() - camera - offset) * world_scale_factor
    }

    #[test]
    fn presented_quads_land_on_the_lattice() {
        let mut app = app();
        // Camera snapped to the grid (no window: the factor falls back
        // to `ZOOM`).
        let world_scale_factor = ZOOM;
        let offset = Vec2::new(6.0, 7.5);
        let actor = app
            .world_mut()
            .spawn((
                CurrPosition::from(CellCoord::new(3, 4)),
                Anchor(Vec2::ZERO),
                SpriteSize::new(Vec2::new(12.0, 15.0)),
                Transform::default(),
            ))
            .id();
        app.update();
        let transform = app.world().get::<Transform>(actor).unwrap();
        let min = min_in_screen_pixels(transform, Vec2::ZERO, offset, world_scale_factor);
        assert_eq!(min, min.round());
        // The world position the transform encodes is still the cell's.
        let world = cell_to_world(CurrPosition::from(CellCoord::new(3, 4)));
        assert!((transform.translation.truncate() - world).length() < 0.5);
    }

    #[test]
    fn fractional_positions_snap_without_drifting_off_cell() {
        let mut app = app();
        let world_scale_factor = ZOOM;
        let offset = Vec2::new(6.0, 7.5);
        let actor = app
            .world_mut()
            .spawn((
                CurrPosition {
                    x: 3.4567,
                    y: 4.8910,
                },
                Anchor(Vec2::ZERO),
                SpriteSize::new(Vec2::new(12.0, 15.0)),
                Transform::default(),
            ))
            .id();
        app.update();
        let transform = app.world().get::<Transform>(actor).unwrap();
        let min = min_in_screen_pixels(transform, Vec2::ZERO, offset, world_scale_factor);
        assert_eq!(min, min.round());
        // The snap error never exceeds half a screen pixel.
        let world = cell_to_world(CurrPosition {
            x: 3.4567,
            y: 4.8910,
        });
        let error = (transform.translation.truncate() - world).abs();
        assert!(error.max_element() <= 0.5 / world_scale_factor + 1e-4);
    }

    #[test]
    fn grounded_frames_snap_with_their_anchor_offset() {
        let mut app = app();
        let world_scale_factor = ZOOM;
        // The production anchor: odd-height frames nudge half a world
        // pixel down so the feet sit flush on the cell's bottom edge.
        let anchor = Anchor(Vec2::new(0.0, 0.5 / 15.0));
        let actor = app
            .world_mut()
            .spawn((
                CurrPosition::from(CellCoord::new(3, 4)),
                anchor,
                SpriteSize::new(Vec2::new(16.0, 15.0)),
                Transform::default(),
            ))
            .id();
        app.update();
        let transform = app.world().get::<Transform>(actor).unwrap();
        let offset = min_corner_offset(&anchor, Vec2::new(16.0, 15.0));
        let min = min_in_screen_pixels(transform, Vec2::ZERO, offset, world_scale_factor);
        assert_eq!(min, min.round());
        // Feet flush on the cell's bottom edge: cell (3,4) spans world
        // y (-80,-64], and the sprite's min y lands exactly on -80.
        assert_eq!(transform.translation.y - offset.y, -80.0);
    }

    #[test]
    fn z_layer_is_preserved() {
        let mut app = app();
        let actor = app
            .world_mut()
            .spawn((
                CurrPosition::from(CellCoord::new(0, 0)),
                Anchor(Vec2::ZERO),
                SpriteSize::new(Vec2::new(12.0, 15.0)),
                Transform::from_xyz(0.0, 0.0, 2.0),
            ))
            .id();
        app.update();
        assert_eq!(
            app.world().get::<Transform>(actor).unwrap().translation.z,
            2.0
        );
    }
}
