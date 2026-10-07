//! Floating damage text: the amount rises from the wounded sprite and
//! fades; same-cell texts stack.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::core::health::components::hit_points::HitPoints;
use crate::core::health::messages::damage_applied::DamageApplied;
use crate::frontend::display::bitmap_text::entities::spawn_text::spawn_text;
use crate::frontend::display::bitmap_text::resources::font_registry::FontRegistry;
use crate::frontend::display::bitmap_text::utils::choose_font::choose_font;
use crate::frontend::display::camera::utils::screen_grid::world_scale_factor;
use crate::frontend::display::constants::layout::{LAYER_FLOATING_TEXT, TILE_SIZE, ZOOM};
use crate::frontend::display::effects::components::floating_text::FloatingText;
use crate::frontend::display::effects::constants::feedback::{
    LIFESPAN, NEGATIVE, TEXT_TARGET_GLYPH_HEIGHT, WARNING,
};
use crate::frontend::display::effects::resources::text_stacks::TextStacks;
use crate::frontend::display::map::utils::coords::cell_to_world;
use crate::frontend::display::motion::components::curr_position::CurrPosition;

/// Present every landed wound as its floating amount: the text spawns
/// at the wounded cell's top edge, world-space, in the font the zoom
/// picks for a 9px target glyph. The color reports the target's
/// remaining condition — orange while above half the maximum, red at
/// or below it, red for a target already gone. Texts alive on the same
/// cell stack: the newcomer pushes them up one line.
pub fn show_status(
    font_registry: Res<FontRegistry>,
    windows: Query<&Window, With<PrimaryWindow>>,
    hit_points: Query<&HitPoints>,
    mut text_stacks: ResMut<TextStacks>,
    mut stacked_texts: Query<&mut Transform, With<FloatingText>>,
    mut damages_applied: MessageReader<DamageApplied>,
    mut commands: Commands,
) {
    let zoom = windows.single().map(world_scale_factor).unwrap_or(ZOOM);
    for damage in damages_applied.read() {
        let (font, scale) = choose_font(font_registry.fonts(), TEXT_TARGET_GLYPH_HEIGHT, zoom);
        // Doubled comparison: the threshold is half the maximum,
        // kept in integers.
        let color = match hit_points.get(damage.target) {
            Ok(hit_points) if hit_points.current * 2 > damage.max => WARNING,
            _ => NEGATIVE,
        };
        let line = font.line_height * scale;
        let world = cell_to_world(CurrPosition::from(damage.cell));
        let position = Vec2::new(world.x, world.y + TILE_SIZE / 2.0 + line / 2.0);
        // The stack push: the cell's living texts rise one line, the
        // newcomer takes the base.
        for stacked in text_stacks.texts_on(damage.cell).to_vec() {
            if let Ok(mut transform) = stacked_texts.get_mut(stacked) {
                transform.translation.y += line;
            }
        }
        let root = spawn_text(
            &mut commands,
            font,
            &damage.amount.to_string(),
            color,
            scale,
            position,
            LAYER_FLOATING_TEXT,
        );
        commands.entity(root).insert(FloatingText {
            cell: damage.cell,
            lifespan: LIFESPAN,
            left: LIFESPAN,
        });
        text_stacks.push(damage.cell, root);
    }
}

/// Advance every floating text: rise one tile per second of life, fade
/// through the latter half, and leave the world (and its cell's stack)
/// at the end.
pub fn update_floating_text(
    time: Res<Time>,
    mut text_stacks: ResMut<TextStacks>,
    mut texts: Query<(Entity, &mut FloatingText, &mut Transform, &Children)>,
    mut glyphs: Query<&mut Sprite, Without<FloatingText>>,
    mut commands: Commands,
) {
    let delta = time.delta_secs();
    for (entity, mut text, mut transform, children) in &mut texts {
        text.left -= delta;
        if text.left <= 0.0 {
            text_stacks.remove(text.cell, entity);
            // Despawning the root takes its glyph children along.
            commands.entity(entity).despawn();
            continue;
        }
        transform.translation.y += TILE_SIZE * delta;
        // Opaque while more than half the life remains, then fading
        // with the remaining fraction — the fade lives in the latter
        // half only.
        let fraction = text.left / text.lifespan;
        let alpha = if fraction > 0.5 { 1.0 } else { fraction * 2.0 };
        for child in children.iter() {
            if let Ok(mut sprite) = glyphs.get_mut(child) {
                sprite.color = sprite.color.with_alpha(alpha);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::Duration;

    use super::*;
    use crate::core::map::components::cell_coord::CellCoord;
    use crate::frontend::display::bitmap_text::types::font::Font;

    /// A five-font registry (the choice indexes fonts positionally)
    /// whose 25x slot holds digit glyphs, so the text spawns real glyph
    /// sprites.
    fn app() -> App {
        let glyphs = ('0'..='9')
            .map(|digit| (digit, Rect::new(0.0, 0.0, 4.0, 8.0)))
            .collect::<HashMap<_, _>>();
        let digit_font = Font {
            texture: Handle::default(),
            glyphs,
            space_advance: 2.0,
            line_height: 8.0,
            baseline: 6.0,
            tracking: -1.0,
        };
        let blank_font = || Font {
            texture: Handle::default(),
            glyphs: HashMap::new(),
            space_advance: 2.0,
            line_height: 8.0,
            baseline: 6.0,
            tracking: -1.0,
        };
        let fonts = vec![
            blank_font(),
            blank_font(),
            blank_font(),
            digit_font,
            blank_font(),
        ];
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .insert_resource(FontRegistry::for_test_with_fonts(fonts))
            .init_resource::<TextStacks>()
            .init_resource::<Messages<DamageApplied>>()
            .add_systems(Update, (show_status, update_floating_text).chain());
        app
    }

    fn advance(app: &mut App, secs: f32) {
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f32(secs));
        app.update();
    }

    fn applied(target: Entity, amount: i32) -> DamageApplied {
        DamageApplied {
            target,
            cell: CellCoord::new(2, 3),
            source_cell: None,
            amount,
            max: 20,
        }
    }

    #[test]
    fn a_hit_floats_its_amount_at_the_cells_top() {
        let mut app = app();
        let rat = app
            .world_mut()
            .spawn(HitPoints {
                current: 20,
                max: 20,
            })
            .id();
        app.world_mut().write_message(applied(rat, 4));
        app.update();
        let mut query = app
            .world_mut()
            .query_filtered::<(&FloatingText, &Transform), With<Children>>();
        let (text, transform) = query.single(app.world()).unwrap();
        assert_eq!(text.cell, CellCoord::new(2, 3));
        assert_eq!(text.lifespan, 1.0);
        // Cell (2,3) center y = -(3.5)*16 = -56; the top edge is 8
        // above it, the half line (8 * 0.5 scale / 2) 2 more.
        assert_eq!(transform.translation.y, -56.0 + 8.0 + 2.0);
        let stacks = app.world().resource::<TextStacks>();
        assert_eq!(stacks.texts_on(CellCoord::new(2, 3)).len(), 1);
    }

    #[test]
    fn below_half_the_maximum_turns_red_and_a_gone_target_stays_red() {
        let mut app = app();
        let healthy = app
            .world_mut()
            .spawn(HitPoints {
                current: 20,
                max: 20,
            })
            .id();
        let hurt = app
            .world_mut()
            .spawn(HitPoints {
                current: 5,
                max: 20,
            })
            .id();
        let ghost = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(applied(healthy, 4));
        app.world_mut().write_message(applied(hurt, 4));
        app.world_mut().write_message(applied(ghost, 4));
        app.update();
        // Three roots; colors live on the glyph children.
        let mut roots = app
            .world_mut()
            .query_filtered::<&Children, With<FloatingText>>();
        let colors: Vec<Color> = roots
            .iter(app.world())
            .map(|children| {
                let glyph = children.iter().next().unwrap();
                app.world().get::<Sprite>(glyph).unwrap().color
            })
            .collect();
        // The presented order follows the messages; the healthy hit is
        // orange, the hurt and the gone are red.
        assert_eq!(colors[0], WARNING);
        assert_eq!(colors[1], NEGATIVE);
        assert_eq!(colors[2], NEGATIVE);
    }

    #[test]
    fn same_cell_texts_stack_upward() {
        let mut app = app();
        let rat = app
            .world_mut()
            .spawn(HitPoints {
                current: 20,
                max: 20,
            })
            .id();
        app.world_mut().write_message(applied(rat, 4));
        app.update();
        let mut roots = app
            .world_mut()
            .query_filtered::<&Transform, With<FloatingText>>();
        let first_y = roots.single(app.world()).unwrap().translation.y;
        app.world_mut().write_message(applied(rat, 4));
        app.update();
        let mut roots = app
            .world_mut()
            .query_filtered::<&Transform, With<FloatingText>>();
        let positions: Vec<f32> = roots
            .iter(app.world())
            .map(|transform| transform.translation.y)
            .collect();
        assert_eq!(positions.len(), 2);
        assert!(
            positions.iter().any(|y| *y > first_y),
            "the newcomer pushed the first text up one line"
        );
    }

    #[test]
    fn the_text_rises_and_leaves() {
        let mut app = app();
        let rat = app
            .world_mut()
            .spawn(HitPoints {
                current: 20,
                max: 20,
            })
            .id();
        app.world_mut().write_message(applied(rat, 4));
        app.update();
        let mut roots = app
            .world_mut()
            .query_filtered::<&Transform, With<FloatingText>>();
        let first_y = roots.single(app.world()).unwrap().translation.y;
        advance(&mut app, 0.5);
        let mut roots = app
            .world_mut()
            .query_filtered::<&Transform, With<FloatingText>>();
        let risen_y = roots.single(app.world()).unwrap().translation.y;
        assert!(
            (risen_y - first_y - 8.0).abs() < 1e-4,
            "half a life rises half a tile"
        );
        advance(&mut app, 0.6);
        let mut roots = app
            .world_mut()
            .query_filtered::<&Transform, With<FloatingText>>();
        assert_eq!(
            roots.iter(app.world()).count(),
            0,
            "the text left the world"
        );
        assert!(
            app.world()
                .resource::<TextStacks>()
                .texts_on(CellCoord::new(2, 3))
                .is_empty(),
            "the stack entry left with it"
        );
    }
}
