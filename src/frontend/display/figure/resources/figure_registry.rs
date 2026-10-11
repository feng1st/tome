use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::frontend::display::figure::components::figure_index::FigureIndex;
use crate::frontend::display::figure::types::anim_entry::AnimEntry;
use crate::frontend::display::figure::types::figure::Figure;
use crate::frontend::display::figure::types::figure_entry::FigureEntry;
use crate::frontend::display::sprite_animation::components::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::types::anim::Anim;

pub const FIGURE_TABLE_PATH: &str = "data/graphic/figures.ron";

#[derive(Resource)]
pub struct FigureRegistry {
    figures: Vec<Figure>,
    by_id: HashMap<String, FigureIndex>,
}

impl FigureRegistry {
    pub fn get_index(&self, id: &str) -> Option<FigureIndex> {
        self.by_id.get(id).copied()
    }

    pub fn figure(&self, figure_index: FigureIndex) -> &Figure {
        &self.figures[figure_index.index()]
    }
}

impl FromWorld for FigureRegistry {
    fn from_world(world: &mut World) -> Self {
        let figure_table_content = fs::read_to_string(FIGURE_TABLE_PATH)
            .unwrap_or_else(|e| panic!("cannot read figure table '{FIGURE_TABLE_PATH}': {e}"));
        let entries = parse_figure_table(FIGURE_TABLE_PATH, &figure_table_content);
        let asset_server = world.resource::<AssetServer>().clone();
        let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let mut figures = Vec::with_capacity(entries.len());
        let mut by_id = HashMap::with_capacity(entries.len());
        for (index, entry) in entries.into_iter().enumerate() {
            let figure_index = FigureIndex::from_index(index);
            let image = asset_server.load(entry.texture);
            let layout = layouts.add(TextureAtlasLayout::from_grid(
                entry.frame_size,
                entry.columns as u32,
                entry.rows as u32,
                None,
                None,
            ));
            let anims = entry
                .anims
                .into_iter()
                .map(|anim_entry| {
                    (
                        anim_entry.anim,
                        Anim {
                            frames: anim_entry.frames,
                            fps: anim_entry.fps,
                            looped: anim_entry.looped,
                        },
                    )
                })
                .collect();
            figures.push(Figure::new(image, layout, entry.frame_size, anims));
            by_id.insert(entry.figure.clone(), figure_index);
        }
        FigureRegistry { figures, by_id }
    }
}

pub(crate) fn parse_figure_table(path: &str, figure_table_content: &str) -> Vec<FigureEntry> {
    let entries: Vec<FigureEntry> = ron::from_str(figure_table_content)
        .unwrap_or_else(|e| panic!("figure table '{path}' is not valid RON: {e}"));
    for (i, entry) in entries.iter().enumerate() {
        assert!(
            !entry.figure.is_empty(),
            "figure table '{path}': empty figure id"
        );
        assert!(
            !entries[..i].iter().any(|e| e.figure == entry.figure),
            "figure table '{path}': duplicate figure id '{}'",
            entry.figure
        );
        assert!(
            entry.columns > 0 && entry.rows > 0,
            "figure table '{path}': figure '{}' has an empty grid",
            entry.figure
        );
        let frame_count = entry.columns * entry.rows;
        for (j, anim_entry) in entry.anims.iter().enumerate() {
            check_anim_entry(path, entry, anim_entry, frame_count);
            assert!(
                !entry.anims[..j].iter().any(|a| a.anim == anim_entry.anim),
                "figure table '{path}': figure '{}' has duplicate {:?} anim",
                entry.figure,
                anim_entry.anim
            );
        }
        assert!(
            entry.anims.iter().any(|a| a.anim == AnimKind::Idle),
            "figure table '{path}': figure '{}' has no idle anim",
            entry.figure
        );
    }
    entries
}

fn check_anim_entry(
    path: &str,
    figure_entry: &FigureEntry,
    anim_entry: &AnimEntry,
    frame_count: usize,
) {
    assert!(
        !anim_entry.frames.is_empty(),
        "figure table '{path}': figure '{}' has an empty {:?} anim",
        figure_entry.figure,
        anim_entry.anim
    );
    assert!(
        anim_entry.fps > 0.0,
        "figure table '{path}': figure '{}' has a non-positive {:?} fps",
        figure_entry.figure,
        anim_entry.anim
    );
    for frame in &anim_entry.frames {
        assert!(
            *frame < frame_count,
            "figure table '{path}': figure '{}' {:?} frame {frame} is outside the grid",
            figure_entry.figure,
            anim_entry.anim
        );
    }
}
