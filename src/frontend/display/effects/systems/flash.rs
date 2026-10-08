//! Hit flash presentation and expiry.

use bevy::prelude::*;

use crate::core::health::messages::damaged::Damaged;
use crate::frontend::display::effects::components::flash::Flash;
use crate::frontend::display::effects::constants::feedback::{FLASH_INTERVAL, FLASH_TINT};

/// Present every landed wound: a target still in the world flashes —
/// its sprite gains a flash that will overexpose it briefly. A target
/// already removed (the killing blow) flashes nothing; the death
/// presentation carries the moment.
pub fn flash(
    time: Res<Time>,
    mut damaged_reader: MessageReader<Damaged>,
    sprites: Query<&Sprite>,
    mut commands: Commands,
) {
    let elapsed = time.elapsed_secs();
    for damage in damaged_reader.read() {
        if sprites.get(damage.target).is_ok() {
            commands.entity(damage.target).insert(Flash {
                until: elapsed + FLASH_INTERVAL,
            });
        }
    }
}

/// Advance every running flash: while it lasts the sprite renders
/// overexposed; at its end the tint restores and the flash leaves.
/// The tint write assumes no other presentation tints creature
/// sprites — the white tint is the sprite's resting color.
pub fn update_flash(
    time: Res<Time>,
    mut flashes: Query<(Entity, &Flash, &mut Sprite)>,
    mut commands: Commands,
) {
    let elapsed = time.elapsed_secs();
    for (entity, flash, mut sprite) in &mut flashes {
        if elapsed < flash.until {
            sprite.color = FLASH_TINT;
        } else {
            sprite.color = Color::WHITE;
            commands.entity(entity).remove::<Flash>();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::map::components::cell_coord::CellCoord;

    fn app() -> App {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .init_resource::<Messages<Damaged>>()
            .add_systems(Update, (flash, update_flash).chain());
        app
    }

    fn spawn_target(app: &mut App) -> Entity {
        app.world_mut().spawn(Sprite::default()).id()
    }

    #[test]
    fn a_wounded_creature_flashes_briefly_then_restores() {
        let mut app = app();
        let target = spawn_target(&mut app);
        app.world_mut().write_message(Damaged {
            target,
            cell: CellCoord::new(0, 0),
            source_cell: None,
            amount: 3,
            max: 10,
        });
        app.update();
        assert_eq!(
            app.world().get::<Sprite>(target).unwrap().color,
            FLASH_TINT,
            "the flash overexposes"
        );
        assert!(app.world().get::<Flash>(target).is_some());
        // A quarter second later the flash has ended.
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(std::time::Duration::from_secs_f32(0.25));
        app.update();
        assert_eq!(
            app.world().get::<Sprite>(target).unwrap().color,
            Color::WHITE,
            "the tint restores"
        );
        assert!(app.world().get::<Flash>(target).is_none());
    }

    #[test]
    fn a_removed_target_flashes_nothing() {
        let mut app = app();
        let ghost = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(Damaged {
            target: ghost,
            cell: CellCoord::new(0, 0),
            source_cell: None,
            amount: 3,
            max: 10,
        });
        app.update();
        assert!(app.world().get::<Flash>(ghost).is_none());
    }
}
