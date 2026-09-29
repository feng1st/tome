//! The appearance registry: every figure resolved to how it renders.
//! One theme, one file: the resource, its construction from the
//! appearance table file, and the table's parsing and validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::figure::components::figure_index::FigureIndex;
use crate::core::figure::resources::figure_registry::FigureRegistry;
use crate::frontend::display::appearance::types::anim_clip_entry::AnimClipEntry;
use crate::frontend::display::appearance::types::appearance::Appearance;
use crate::frontend::display::appearance::types::appearance_entry::AppearanceEntry;
use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::types::anim_clip::AnimClip;

/// The appearance table loaded at startup.
pub const APPEARANCES_PATH: &str = "data/graphic/appearances.ron";

/// Registry of all appearances, built once at startup from the
/// appearance table file. Every declared figure is guaranteed an
/// appearance (checked at load), so a missing key at runtime is a
/// load-time bug, not a runtime case. Instances never own this data —
/// they carry only the `FigureIndex` handle and cloned sprite handles;
/// systems look appearances up through this resource.
#[derive(Resource)]
pub struct AppearanceRegistry {
    appearances: HashMap<FigureIndex, Appearance>,
}

impl AppearanceRegistry {
    pub(crate) fn new(appearances: HashMap<FigureIndex, Appearance>) -> Self {
        AppearanceRegistry { appearances }
    }

    /// The appearance for a figure handle.
    pub fn appearance(&self, figure_index: FigureIndex) -> &Appearance {
        self.appearances
            .get(&figure_index)
            .expect("every declared figure has an appearance (checked at load)")
    }
}

impl FromWorld for AppearanceRegistry {
    /// Build from the appearance table file, resolving ids against the
    /// core figure registry — pulling it into existence if it is not
    /// built yet, so registration order never matters. Fire-and-forget
    /// texture loads: handles and layout metadata suffice for attaching
    /// an appearance, the renderer waits for the pixels. `Assets::add`
    /// does not deduplicate, so atlas layouts are built once here, not
    /// per entity.
    fn from_world(world: &mut World) -> Self {
        let text = fs::read_to_string(APPEARANCES_PATH)
            .unwrap_or_else(|e| panic!("cannot read appearance table '{APPEARANCES_PATH}': {e}"));
        world.get_resource_or_init::<FigureRegistry>();
        // The dependency exists now (a pull builds it if missing);
        // a shared read suffices.
        let figure_registry = world.resource::<FigureRegistry>();
        let entries = parse_appearance_entries(APPEARANCES_PATH, &text, figure_registry);
        // Id resolution ends with the parse; the registry borrow ends
        // here so asset resources can be taken.
        let asset_server = world.resource::<AssetServer>().clone();
        let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let mut appearances = HashMap::with_capacity(entries.len());
        for (figure_index, entry) in entries {
            let image = asset_server.load(entry.texture);
            let layout = layouts.add(TextureAtlasLayout::from_grid(
                entry.frame_size,
                entry.columns,
                entry.rows,
                None,
                None,
            ));
            let clips = entry
                .clips
                .into_iter()
                .map(|clip_entry| {
                    (
                        clip_entry.anim,
                        AnimClip {
                            frames: clip_entry.frames,
                            fps: clip_entry.fps,
                        },
                    )
                })
                .collect();
            appearances.insert(
                figure_index,
                Appearance::new(image, layout, entry.frame_size, clips),
            );
        }
        AppearanceRegistry::new(appearances)
    }
}

/// Parse and validate appearance table text against the figure
/// vocabulary, resolving each entry's id to its handle. Split from file
/// IO and asset assembly (`FromWorld`) so tests can exercise it with
/// inline documents; the figure registry is pure data and keeps the
/// seam engine-free.
pub(crate) fn parse_appearance_entries(
    path: &str,
    text: &str,
    figure_registry: &FigureRegistry,
) -> Vec<(FigureIndex, AppearanceEntry)> {
    let entries: Vec<AppearanceEntry> = ron::from_str(text)
        .unwrap_or_else(|e| panic!("appearance table '{path}' is not valid RON: {e}"));
    for (i, entry) in entries.iter().enumerate() {
        assert!(
            figure_registry.get_index(&entry.figure).is_some(),
            "appearance table '{path}': unknown figure '{}'",
            entry.figure
        );
        assert!(
            !entries[..i].iter().any(|e| e.figure == entry.figure),
            "appearance table '{path}': duplicate appearance for figure '{}'",
            entry.figure
        );
        assert!(
            entry.columns > 0 && entry.rows > 0,
            "appearance table '{path}': figure '{}' has an empty grid",
            entry.figure
        );
        let frame_count = u64::from(entry.columns) * u64::from(entry.rows);
        for (j, clip_entry) in entry.clips.iter().enumerate() {
            check_clip_entry(path, entry, clip_entry, frame_count);
            assert!(
                !entry.clips[..j].iter().any(|c| c.anim == clip_entry.anim),
                "appearance table '{path}': figure '{}' has duplicate {:?} clip",
                entry.figure,
                clip_entry.anim
            );
        }
        assert!(
            entry.clips.iter().any(|c| c.anim == AnimKind::Idle),
            "appearance table '{path}': figure '{}' has no idle clip",
            entry.figure
        );
    }
    for (_, id) in figure_registry.iter() {
        assert!(
            entries.iter().any(|e| e.figure == id),
            "appearance table '{path}': figure '{id}' has no appearance"
        );
    }
    entries
        .into_iter()
        .map(|entry| {
            let figure_index = figure_registry
                .get_index(&entry.figure)
                .expect("validated above");
            (figure_index, entry)
        })
        .collect()
}

/// Validate one anim table entry: a playable clip needs frames, a
/// positive rate, and indices inside the sheet grid.
fn check_clip_entry(
    path: &str,
    entry: &AppearanceEntry,
    clip_entry: &AnimClipEntry,
    frame_count: u64,
) {
    assert!(
        !clip_entry.frames.is_empty(),
        "appearance table '{path}': figure '{}' has an empty {:?} clip",
        entry.figure,
        clip_entry.anim
    );
    // `> 0.0` also rejects NaN.
    assert!(
        clip_entry.fps > 0.0,
        "appearance table '{path}': figure '{}' has a non-positive {:?} fps",
        entry.figure,
        clip_entry.anim
    );
    for frame in &clip_entry.frames {
        assert!(
            (*frame as u64) < frame_count,
            "appearance table '{path}': figure '{}' {:?} frame {frame} is outside the grid",
            entry.figure,
            clip_entry.anim
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::figure::resources::figure_registry::parse_figure_registry;

    fn figure_registry() -> FigureRegistry {
        parse_figure_registry("test", r#"[ "warrior", "rat" ]"#)
    }

    const DOC: &str = r#"[
        (
            figure: "warrior",
            texture: "warrior.png",
            frame_size: (12, 15),
            columns: 21,
            rows: 8,
            clips: [
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
            clips: [ ( anim: idle, frames: [0, 1], fps: 6.0 ) ],
        ),
    ]"#;

    #[test]
    fn entries_parse_and_resolve() {
        let figure_registry = figure_registry();
        let entries = parse_appearance_entries("test", DOC, &figure_registry);
        assert_eq!(entries.len(), 2);
        let (figure_index, warrior) = &entries[0];
        assert_eq!(*figure_index, figure_registry.get_index("warrior").unwrap());
        assert_eq!(warrior.texture, "warrior.png");
        assert_eq!(warrior.frame_size, UVec2::new(12, 15));
        assert_eq!((warrior.columns, warrior.rows), (21, 8));
        assert_eq!(warrior.clips.len(), 2);
        assert_eq!(warrior.clips[0].anim, AnimKind::Idle);
        assert_eq!(warrior.clips[0].frames, [0, 0, 0, 1, 0, 0, 1, 1]);
        assert_eq!(warrior.clips[1].anim, AnimKind::Run);
        assert_eq!(warrior.clips[1].fps, 20.0);
    }

    #[test]
    #[should_panic(expected = "unknown figure 'wolf'")]
    fn unknown_figure_panics() {
        let doc = DOC.replace("figure: \"warrior\"", "figure: \"wolf\"");
        parse_appearance_entries("test", &doc, &figure_registry());
    }

    #[test]
    #[should_panic(expected = "duplicate appearance for figure 'warrior'")]
    fn duplicate_appearance_panics() {
        // Point the rat entry at warrior: two appearances for one figure.
        let doc = DOC.replace("figure: \"rat\"", "figure: \"warrior\"");
        parse_appearance_entries("test", &doc, &figure_registry());
    }

    #[test]
    #[should_panic(expected = "figure 'rat' has no appearance")]
    fn missing_appearance_panics() {
        // Drop the rat entry: the declared figure loses its appearance.
        let doc = DOC.replace(
            r#",
        (
            figure: "rat",
            texture: "rat.png",
            frame_size: (16, 15),
            columns: 4,
            rows: 1,
            clips: [ ( anim: idle, frames: [0, 1], fps: 6.0 ) ],
        )"#,
            "",
        );
        parse_appearance_entries("test", &doc, &figure_registry());
    }

    #[test]
    #[should_panic(expected = "has an empty grid")]
    fn empty_grid_panics() {
        let doc = DOC.replace("columns: 21", "columns: 0");
        parse_appearance_entries("test", &doc, &figure_registry());
    }

    #[test]
    #[should_panic(expected = "has an empty Idle clip")]
    fn empty_clip_panics() {
        let doc = DOC.replace("frames: [0, 1]", "frames: []");
        parse_appearance_entries("test", &doc, &figure_registry());
    }

    #[test]
    #[should_panic(expected = "non-positive Run fps")]
    fn non_positive_fps_panics() {
        let doc = DOC.replace("fps: 20.0", "fps: 0.0");
        parse_appearance_entries("test", &doc, &figure_registry());
    }

    #[test]
    #[should_panic(expected = "frame 168 is outside the grid")]
    fn frame_outside_grid_panics() {
        // The warrior grid holds 21 * 8 = 168 frames; index 168 is out.
        let doc = DOC.replace("frames: [0, 1]", "frames: [168]");
        parse_appearance_entries("test", &doc, &figure_registry());
    }

    #[test]
    #[should_panic(expected = "duplicate Run clip")]
    fn duplicate_anim_panics() {
        let doc = DOC.replace(
            "( anim: idle, frames: [0, 0, 0, 1, 0, 0, 1, 1], fps: 8.0 )",
            "( anim: run,  frames: [0, 0, 0, 1, 0, 0, 1, 1], fps: 8.0 )",
        );
        parse_appearance_entries("test", &doc, &figure_registry());
    }

    #[test]
    #[should_panic(expected = "has no idle clip")]
    fn missing_idle_panics() {
        let doc = DOC.replace("anim: idle", "anim: walk");
        parse_appearance_entries("test", &doc, &figure_registry());
    }

    #[test]
    #[should_panic(expected = "not valid RON")]
    fn malformed_document_panics() {
        parse_appearance_entries("test", "this is not ron", &figure_registry());
    }

    /// Spec-alignment test: the real data files on disk reproduce the
    /// established warrior presentation.
    #[test]
    fn warrior_appearance_matches_the_reference_presentation() {
        use crate::core::figure::resources::figure_registry::FIGURE_TABLE_PATH;

        let text = std::fs::read_to_string(FIGURE_TABLE_PATH).unwrap();
        let figure_registry = parse_figure_registry(FIGURE_TABLE_PATH, &text);
        figure_registry
            .get_index("warrior")
            .expect("warrior is a declared figure");

        let text = std::fs::read_to_string(APPEARANCES_PATH).unwrap();
        let entries = parse_appearance_entries(APPEARANCES_PATH, &text, &figure_registry);
        let entry = &entries
            .iter()
            .find(|(_, e)| e.figure == "warrior")
            .expect("warrior has an appearance entry")
            .1;
        assert_eq!(entry.texture, "warrior.png");
        assert_eq!(entry.frame_size, UVec2::new(12, 15));
        assert_eq!((entry.columns, entry.rows), (21, 8));
        let idle = entry
            .clips
            .iter()
            .find(|c| c.anim == AnimKind::Idle)
            .unwrap();
        assert_eq!(idle.frames, [0, 0, 0, 1, 0, 0, 1, 1]);
        assert_eq!(idle.fps, 8.0);
        let run = entry
            .clips
            .iter()
            .find(|c| c.anim == AnimKind::Run)
            .unwrap();
        assert_eq!(run.frames, [2, 3, 4, 5, 6, 7]);
        assert_eq!(run.fps, 20.0);
    }

    /// Spec-alignment test: the real data files on disk reproduce the
    /// established giant white rat presentation.
    #[test]
    fn giant_white_rat_appearance_matches_the_reference_presentation() {
        use crate::core::figure::resources::figure_registry::FIGURE_TABLE_PATH;

        let text = std::fs::read_to_string(FIGURE_TABLE_PATH).unwrap();
        let figure_registry = parse_figure_registry(FIGURE_TABLE_PATH, &text);
        figure_registry
            .get_index("giant_white_rat")
            .expect("giant_white_rat is a declared figure");

        let text = std::fs::read_to_string(APPEARANCES_PATH).unwrap();
        let entries = parse_appearance_entries(APPEARANCES_PATH, &text, &figure_registry);
        let entry = &entries
            .iter()
            .find(|(_, e)| e.figure == "giant_white_rat")
            .expect("giant_white_rat has an appearance entry")
            .1;
        assert_eq!(entry.texture, "rat.png");
        assert_eq!(entry.frame_size, UVec2::new(16, 15));
        assert_eq!((entry.columns, entry.rows), (16, 2));
        // Only an idle clip: the rat neither walks nor fights yet.
        assert_eq!(entry.clips.len(), 1);
        let idle = entry
            .clips
            .iter()
            .find(|c| c.anim == AnimKind::Idle)
            .unwrap();
        assert_eq!(idle.frames, [16, 16, 16, 17]);
        assert_eq!(idle.fps, 2.0);
    }
}
