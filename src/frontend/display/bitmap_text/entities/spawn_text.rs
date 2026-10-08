//! Spawning a text as glyph sprites: one root entity, one sprite per
//! glyph, laid out by the font.

use bevy::prelude::*;

use crate::frontend::display::bitmap_text::types::font::Font;

/// Spawn a one-line text: a root entity at `position` (layer `z`),
/// with one child sprite per glyph — the font's rectangle, tinted
/// `color`, scaled by `scale`, laid side by side centered on the root.
/// Returns the root; the caller layers its own behavior components on
/// it. Glyph sprites render the texture's own pixels (white fill,
/// dark outline) multiplied by the tint, so the tint colors the fill
/// and the outline stays dark. The root carries `Visibility` for the
/// children's sake: each glyph sprite's `InheritedVisibility` inherits
/// down the tree, and a parent outside the propagation leaves its
/// glyphs undrawn.
pub fn spawn_text(
    commands: &mut Commands,
    font: &Font,
    text: &str,
    color: Color,
    scale: f32,
    position: Vec2,
    z: f32,
) -> Entity {
    let laid = font.layout(text);
    let total = font.measure(text);
    let mut root = commands.spawn((
        Transform::from_xyz(position.x, position.y, z),
        Visibility::default(),
    ));
    root.with_children(|children| {
        for (rect, left) in laid {
            let glyph_center_x = (left + rect.width() / 2.0 - total / 2.0) * scale;
            children.spawn((
                Sprite {
                    image: font.texture.clone(),
                    color,
                    rect: Some(rect),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(glyph_center_x, 0.0, 0.0),
                    scale: Vec3::splat(scale),
                    ..default()
                },
            ));
        }
    });
    root.id()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    /// A font whose '1' is 3px and '0' is 4px wide, tracking -1: the
    /// text "10" measures 4px wide (3 + 2, minus trailing -1).
    fn font() -> Font {
        let glyphs = HashMap::from([
            ('0', Rect::new(0.0, 0.0, 4.0, 8.0)),
            ('1', Rect::new(5.0, 0.0, 8.0, 8.0)),
        ]);
        Font {
            texture: Handle::default(),
            glyphs,
            space_advance: 2.0,
            line_height: 8.0,
            baseline: 6.0,
            tracking: -1.0,
        }
    }

    #[test]
    fn two_digits_lay_side_by_side_centered() {
        let mut world = World::new();
        let font = font();
        let root = {
            let mut commands = world.commands();
            spawn_text(
                &mut commands,
                &font,
                "10",
                Color::WHITE,
                1.0,
                Vec2::ZERO,
                5.0,
            )
        };
        world.flush();

        let transform = world.get::<Transform>(root).unwrap();
        assert_eq!(transform.translation.z, 5.0);
        let children: Vec<Entity> = world
            .entity(root)
            .get::<Children>()
            .unwrap()
            .iter()
            .collect();
        assert_eq!(children.len(), 2, "one sprite per glyph");
        // Lefts: '1' at advance 0, '0' at advance 2; total 6 — the
        // glyph centers land at -1.5 and +1, each side of the center.
        let first = world.get::<Transform>(children[0]).unwrap().translation;
        let second = world.get::<Transform>(children[1]).unwrap().translation;
        assert_eq!(first.x, -1.5);
        assert_eq!(second.x, 1.0);
        let sprite = world.get::<Sprite>(children[0]).unwrap();
        assert_eq!(sprite.rect, Some(Rect::new(5.0, 0.0, 8.0, 8.0)));
    }

    #[test]
    fn scale_multiplies_the_advance_and_the_sprite() {
        let mut world = World::new();
        let font = font();
        let root = {
            let mut commands = world.commands();
            spawn_text(
                &mut commands,
                &font,
                "10",
                Color::WHITE,
                0.5,
                Vec2::ZERO,
                0.0,
            )
        };
        world.flush();
        let children: Vec<Entity> = world
            .entity(root)
            .get::<Children>()
            .unwrap()
            .iter()
            .collect();
        let first = world.get::<Transform>(children[0]).unwrap();
        assert_eq!(first.translation.x, -0.75);
        assert_eq!(first.scale.x, 0.5);
    }
}
