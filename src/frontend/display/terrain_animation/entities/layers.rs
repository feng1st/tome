//! The animated terrain layer: one map-sized REPEAT quad beneath the
//! chunks — PD's `SkinnedBlock` approach (the chunks render open water
//! as a fully transparent tile, so the layer shows through). Hardwired
//! to water for now; multi-kind support (presence mask + per-cell
//! meshes) is future work.

use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::prelude::*;
use bevy::sprite_render::AlphaMode2d;

use crate::core::map::resources::current_map::CurrentMap;
use crate::frontend::display::map::constants::layout::{LAYER_SCROLL, TILE_SIZE};
use crate::frontend::display::terrain_animation::components::terrain_anim::TerrainAnim;
use crate::frontend::display::terrain_animation::constants::terrain_anims::WATER_ANIM_SPEC;

/// Spawn the water layer. The quad is map-sized and centered on the map;
/// which cells reveal it is the chunks' business (transparent open-water
/// tiles and semi-transparent shoreline pixels). Spawns unconditionally —
/// a "skip kinds the map lacks" check returns with the multi-kind rework
/// (as an O(1) presence mask built at map creation, not a per-spawn
/// scan).
pub fn spawn_layers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid_map: Res<CurrentMap>,
) {
    let map = grid_map.map();
    let w = map.width as f32 * TILE_SIZE;
    let h = map.height as f32 * TILE_SIZE;

    let spec = WATER_ANIM_SPEC;
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
        TerrainAnim {
            scale: Vec2::new(
                w / spec.texture_size.x as f32,
                h / spec.texture_size.y as f32,
            ),
            velocity: spec.velocity,
        },
    ));
}
