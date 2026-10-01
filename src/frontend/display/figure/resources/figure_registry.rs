//! The figure registry: every figure id resolved to its runtime handle
//! and its render data. One theme, one file: the resource, its
//! construction from the figure table file, and the table's parsing and
//! validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::frontend::display::figure::components::figure_index::FigureIndex;
use crate::frontend::display::figure::types::anim_entry::AnimEntry;
use crate::frontend::display::figure::types::appearance::Appearance;
use crate::frontend::display::figure::types::figure_entry::FigureEntry;
use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::types::anim::Anim;

/// The figure table loaded at startup.
pub const FIGURE_TABLE_PATH: &str = "data/graphic/figures.ron";

/// Registry of all figures, built once at startup from the figure table
/// file. `FigureIndex` is the index into the internal table, assigned in
/// entry order — an unstable runtime handle, never an identity: the same
/// figure's index changes when the file's entry order changes. Ids serve
/// file references, error messages, and save serialization only; binding
/// files resolve ids to handles here, and the render side resolves
/// handles to appearances. Entries sit in file order in a dense table:
/// row storage keeps one lookup per figure. Instances never own
/// appearance data — they carry only the handle and cloned sprite
/// handles; systems look appearances up through this resource.
#[derive(Resource)]
pub struct FigureRegistry {
    appearances: Vec<Appearance>,
    by_id: HashMap<String, FigureIndex>,
}

impl FigureRegistry {
    /// The handle for a figure id, if the id is declared. Ids resolve
    /// only at content boundaries (display bindings).
    pub fn get_index(&self, id: &str) -> Option<FigureIndex> {
        self.by_id.get(id).copied()
    }

    /// The appearance for a figure handle. Callers hold valid handles by
    /// construction (checked at load), so a missing entry at runtime is
    /// a load-time bug, not a runtime case.
    pub fn appearance(&self, figure_index: FigureIndex) -> &Appearance {
        &self.appearances[figure_index.index()]
    }
}

#[cfg(test)]
impl FigureRegistry {
    /// A registry for tests: the given ids resolve to handles in list
    /// order, each backed by a placeholder appearance (default handles,
    /// idle-only). Binding and playback tests never touch pixels.
    pub(crate) fn for_test(ids: &[&str]) -> Self {
        let mut appearances = Vec::with_capacity(ids.len());
        let mut by_id = HashMap::with_capacity(ids.len());
        for (index, id) in ids.iter().enumerate() {
            by_id.insert(id.to_string(), FigureIndex::from_index(index));
            appearances.push(Appearance::new(
                Handle::default(),
                Handle::default(),
                UVec2::ONE,
                HashMap::from([(
                    AnimKind::Idle,
                    Anim {
                        frames: vec![0],
                        fps: 1.0,
                    },
                )]),
            ));
        }
        FigureRegistry { appearances, by_id }
    }
}

impl FromWorld for FigureRegistry {
    /// Build from the figure table file. Fire-and-forget texture loads:
    /// handles and layout metadata suffice for attaching an appearance,
    /// the renderer waits for the pixels. `Assets::add` does not
    /// deduplicate, so atlas layouts are built once here, not per
    /// entity.
    fn from_world(world: &mut World) -> Self {
        let text = fs::read_to_string(FIGURE_TABLE_PATH)
            .unwrap_or_else(|e| panic!("cannot read figure table '{FIGURE_TABLE_PATH}': {e}"));
        let entries = parse_figure_entries(FIGURE_TABLE_PATH, &text);
        // Id resolution ends with the parse; asset resources are taken
        // afterwards.
        let asset_server = world.resource::<AssetServer>().clone();
        let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let mut appearances = Vec::with_capacity(entries.len());
        let mut by_id = HashMap::with_capacity(entries.len());
        for (index, entry) in entries.into_iter().enumerate() {
            let figure_index = FigureIndex::from_index(index);
            let image = asset_server.load(entry.texture);
            let layout = layouts.add(TextureAtlasLayout::from_grid(
                entry.frame_size,
                entry.columns,
                entry.rows,
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
                        },
                    )
                })
                .collect();
            appearances.push(Appearance::new(image, layout, entry.frame_size, anims));
            by_id.insert(entry.figure.clone(), figure_index);
        }
        FigureRegistry { appearances, by_id }
    }
}

/// Parse and validate figure table text. Split from file IO and asset
/// assembly (`FromWorld`) so tests can exercise it with inline
/// documents.
pub(crate) fn parse_figure_entries(path: &str, text: &str) -> Vec<FigureEntry> {
    let entries: Vec<FigureEntry> = ron::from_str(text)
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
        let frame_count = u64::from(entry.columns) * u64::from(entry.rows);
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

/// Validate one anim table entry: a playable anim needs frames, a
/// positive rate, and indices inside the sheet grid.
fn check_anim_entry(path: &str, entry: &FigureEntry, anim_entry: &AnimEntry, frame_count: u64) {
    assert!(
        !anim_entry.frames.is_empty(),
        "figure table '{path}': figure '{}' has an empty {:?} anim",
        entry.figure,
        anim_entry.anim
    );
    // `> 0.0` also rejects NaN.
    assert!(
        anim_entry.fps > 0.0,
        "figure table '{path}': figure '{}' has a non-positive {:?} fps",
        entry.figure,
        anim_entry.anim
    );
    for frame in &anim_entry.frames {
        assert!(
            (*frame as u64) < frame_count,
            "figure table '{path}': figure '{}' {:?} frame {frame} is outside the grid",
            entry.figure,
            anim_entry.anim
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = r#"[
        (
            figure: "warrior",
            texture: "warrior.png",
            frame_size: (12, 15),
            columns: 21,
            rows: 8,
            anims: [
                ( anim: idle, frames: [0, 0, 0, 1, 0, 0, 1, 1], fps: 8.0 ),
                ( anim: run,  frames: [2, 3, 4, 5, 6, 7],         fps: 20.0 ),
            ],
        ),
        (
            figure: "rat",
            texture: "rat.png",
            frame_size: (16, 15),
            columns: 4,
            rows: 1,
            anims: [ ( anim: idle, frames: [0, 1], fps: 6.0 ) ],
        ),
    ]"#;

    #[test]
    fn entries_parse_with_ids_intact() {
        let entries = parse_figure_entries("test", DOC);
        assert_eq!(entries.len(), 2);
        let warrior = &entries[0];
        assert_eq!(warrior.figure, "warrior");
        assert_eq!(warrior.texture, "warrior.png");
        assert_eq!(warrior.frame_size, UVec2::new(12, 15));
        assert_eq!((warrior.columns, warrior.rows), (21, 8));
        assert_eq!(warrior.anims.len(), 2);
        assert_eq!(warrior.anims[0].anim, AnimKind::Idle);
        assert_eq!(warrior.anims[0].frames, [0, 0, 0, 1, 0, 0, 1, 1]);
        assert_eq!(warrior.anims[1].anim, AnimKind::Run);
        assert_eq!(warrior.anims[1].fps, 20.0);
    }

    #[test]
    #[should_panic(expected = "duplicate figure id 'warrior'")]
    fn duplicate_id_panics() {
        let doc = DOC.replace("figure: \"rat\"", "figure: \"warrior\"");
        parse_figure_entries("test", &doc);
    }

    #[test]
    #[should_panic(expected = "empty figure id")]
    fn empty_id_panics() {
        let doc = DOC.replace("figure: \"warrior\"", "figure: \"\"");
        parse_figure_entries("test", &doc);
    }

    #[test]
    #[should_panic(expected = "has an empty grid")]
    fn empty_grid_panics() {
        let doc = DOC.replace("columns: 21", "columns: 0");
        parse_figure_entries("test", &doc);
    }

    #[test]
    #[should_panic(expected = "has an empty Idle anim")]
    fn empty_anim_panics() {
        let doc = DOC.replace("frames: [0, 1]", "frames: []");
        parse_figure_entries("test", &doc);
    }

    #[test]
    #[should_panic(expected = "non-positive Run fps")]
    fn non_positive_fps_panics() {
        let doc = DOC.replace("fps: 20.0", "fps: 0.0");
        parse_figure_entries("test", &doc);
    }

    #[test]
    #[should_panic(expected = "frame 168 is outside the grid")]
    fn frame_outside_grid_panics() {
        // The warrior grid holds 21 * 8 = 168 frames; index 168 is out.
        let doc = DOC.replace("frames: [0, 1]", "frames: [168]");
        parse_figure_entries("test", &doc);
    }

    #[test]
    #[should_panic(expected = "duplicate Run anim")]
    fn duplicate_anim_panics() {
        let doc = DOC.replace(
            "( anim: idle, frames: [0, 0, 0, 1, 0, 0, 1, 1], fps: 8.0 )",
            "( anim: run,  frames: [0, 0, 0, 1, 0, 0, 1, 1], fps: 8.0 )",
        );
        parse_figure_entries("test", &doc);
    }

    #[test]
    #[should_panic(expected = "has no idle anim")]
    fn missing_idle_panics() {
        let doc = DOC.replace("anim: idle", "anim: walk");
        parse_figure_entries("test", &doc);
    }

    #[test]
    #[should_panic(expected = "not valid RON")]
    fn malformed_document_panics() {
        parse_figure_entries("test", "this is not ron");
    }

    /// Spec-alignment test: the real figure table on disk reproduces the
    /// established warrior presentation.
    #[test]
    fn warrior_entry_matches_the_reference_presentation() {
        let text = std::fs::read_to_string(FIGURE_TABLE_PATH).unwrap();
        let entries = parse_figure_entries(FIGURE_TABLE_PATH, &text);
        let entry = entries
            .iter()
            .find(|e| e.figure == "warrior")
            .expect("warrior is a declared figure");
        assert_eq!(entry.texture, "warrior.png");
        assert_eq!(entry.frame_size, UVec2::new(12, 15));
        assert_eq!((entry.columns, entry.rows), (21, 8));
        let idle = entry
            .anims
            .iter()
            .find(|a| a.anim == AnimKind::Idle)
            .unwrap();
        assert_eq!(idle.frames, [0, 0, 0, 1, 0, 0, 1, 1]);
        assert_eq!(idle.fps, 8.0);
        let run = entry
            .anims
            .iter()
            .find(|a| a.anim == AnimKind::Run)
            .unwrap();
        assert_eq!(run.frames, [2, 3, 4, 5, 6, 7]);
        assert_eq!(run.fps, 20.0);
    }

    /// Spec-alignment test: the real figure table on disk reproduces the
    /// established giant white rat presentation.
    #[test]
    fn giant_white_rat_entry_matches_the_reference_presentation() {
        let text = std::fs::read_to_string(FIGURE_TABLE_PATH).unwrap();
        let entries = parse_figure_entries(FIGURE_TABLE_PATH, &text);
        let entry = entries
            .iter()
            .find(|e| e.figure == "giant_white_rat")
            .expect("giant_white_rat is a declared figure");
        assert_eq!(entry.texture, "rat.png");
        assert_eq!(entry.frame_size, UVec2::new(16, 15));
        assert_eq!((entry.columns, entry.rows), (16, 2));
        // Only an idle anim: the rat neither walks nor fights yet.
        assert_eq!(entry.anims.len(), 1);
        let idle = entry
            .anims
            .iter()
            .find(|a| a.anim == AnimKind::Idle)
            .unwrap();
        assert_eq!(idle.frames, [16, 16, 16, 17]);
        assert_eq!(idle.fps, 2.0);
    }
}
