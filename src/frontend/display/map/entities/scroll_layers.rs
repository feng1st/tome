// TODO: pending cleanup review — remove once stabilized
//! Scrolling terrain layers: one map-sized REPEAT quad per animated
//! terrain, beneath the chunks — PD's `SkinnedBlock` approach (the chunks
//! render open water as a fully transparent tile, so the layer shows
//! through).

use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::prelude::*;
use bevy::sprite_render::AlphaMode2d;

use crate::core::app_state::AppState;
use crate::core::map::resources::current_map::CurrentMap;
use crate::frontend::display::map::components::terrain_scroll::TerrainScroll;
use crate::frontend::display::map::constants::layout::{LAYER_SCROLL, TILE_SIZE};
use crate::frontend::display::map::constants::terrain_anims::TERRAIN_ANIMS;

/// Spawn one scrolling layer per animated terrain the map contains. The
/// quad is map-sized and centered on the map; which cells reveal it is
/// the chunks' business (transparent open-water tiles and semi-transparent
/// shoreline pixels).
pub fn spawn_scroll_layers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid_map: Res<CurrentMap>,
) {
    let map = grid_map.map();
    let w = map.width as f32 * TILE_SIZE as f32;
    let h = map.height as f32 * TILE_SIZE as f32;
    for spec in TERRAIN_ANIMS {
        if !map.tiles.contains(&spec.kind) {
            continue;
        }
        // REPEAT addressing lets the UV scroll wrap, like PD's
        // `texture.wrap(Texture.REPEAT, Texture.REPEAT)`.
        let texture = asset_server
            .load_builder()
            .with_settings(|s: &mut ImageLoaderSettings| {
                s.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    ..default()
                });
            })
            .load(spec.texture);
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(w, h)))),
            MeshMaterial2d(materials.add(ColorMaterial {
                texture: Some(texture),
                alpha_mode: AlphaMode2d::Blend,
                ..default()
            })),
            Transform::from_xyz(w / 2.0, -h / 2.0, LAYER_SCROLL),
            TerrainScroll {
                scale: Vec2::new(w / spec.texture_size as f32, h / spec.texture_size as f32),
                scroll: spec.scroll,
            },
            DespawnOnExit(AppState::Game),
        ));
    }
}
