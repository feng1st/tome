use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;
use crate::frontend::display::camera::components::sprite_size::SpriteSize;
use crate::frontend::display::constants::layout::LAYER_ACTOR;
use crate::frontend::display::figure::components::figure_index::FigureIndex;
use crate::frontend::display::figure::resources::figure_registry::FigureRegistry;
use crate::frontend::display::map::utils::coords::cell_to_world_position;
use crate::frontend::display::motion::components::curr_position::CellPosition;
use crate::frontend::display::sprite_animation::components::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;

pub fn attach_figure(
    mut commands: Commands,
    figure_registry: Res<FigureRegistry>,
    targets: Query<(Entity, &FigureIndex, &CellCoord), Added<FigureIndex>>,
) {
    for (entity, &figure_index, &cell_coord) in &targets {
        let figure = figure_registry.figure(figure_index);
        let cell_position = CellPosition::from(cell_coord);
        let world_position = cell_to_world_position(cell_position);
        let anim_kind = AnimKind::Idle;
        let anim = figure.anim(anim_kind);
        commands.entity(entity).insert((
            cell_position,
            Sprite {
                image: figure.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: figure.layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            SpriteSize::new(figure.frame_size.as_vec2()),
            Transform::from_xyz(world_position.x, world_position.y, LAYER_ACTOR),
            anim_kind,
            AnimState {
                anim_kind,
                looped: anim.looped,
                frame_offset: (cell_coord.x + cell_coord.y) as usize,
                started_at: 0.0,
                finished: false,
            },
        ));
    }
}
