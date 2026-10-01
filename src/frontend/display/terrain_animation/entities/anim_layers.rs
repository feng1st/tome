//! The animated terrain layers: map-sized layers beneath the terrain
//! mesh, showing through its per-kind alpha-0 cells.
//! Hardwired to water for now; multi-kind support (a table plus
//! per-cell union meshes) is future work.

use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;

use crate::core::map::resources::current_map::CurrentMap;
use crate::frontend::display::constants::layout::{LAYER_GROUND, TILE_SIZE};
use crate::frontend::display::terrain_animation::components::terrain_anim_state::TerrainAnimState;
use crate::frontend::display::terrain_animation::constants::terrain_anims::WATER_ANIM;

/// Z in the world: beneath the ground plane the terrain mesh draws on.
const LAYER_TERRAIN_ANIM: f32 = -1.0;

// The layer shows through the terrain mesh's holes; drawing above the
// ground plane would flood it.
const _: () = assert!(LAYER_TERRAIN_ANIM < LAYER_GROUND);

/// Spawn the animated terrain layers (today: the one water layer) on
/// entering `Game` — the map's spawn path. The layer matches the map
/// rectangle and sits at the map's center like any world content:
/// world-locked by construction, never re-based. The time-driven
/// scroll rides on top via `uv_transform`. Repeat-space UVs are baked
/// into the mesh, so playback only writes the translation.
///
/// The terrain mesh draws nothing on open-water cells, leaving alpha-0
/// holes; premultiplied alpha compositing shows this layer exactly
/// through them — no mask pass, no shader. The chunk tiles'
/// semi-transparent shoreline pixels composite over it the same way.
/// The layer is map-sized and map-anchored, so it lands on the screen
/// grid with the terrain (integer world positions).
pub fn spawn_anim_layers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    current_map: Res<CurrentMap>,
) {
    let local_map = current_map.map();
    let size = Vec2::new(
        local_map.width as f32 * TILE_SIZE,
        local_map.height as f32 * TILE_SIZE,
    );
    let terrain_anim = WATER_ANIM;
    // REPEAT addressing lets the scroll wrap.
    let texture = asset_server
        .load_builder()
        .with_settings(|s: &mut ImageLoaderSettings| {
            s.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..default()
            });
        })
        .load(terrain_anim.texture);
    // Bake repeat-space UVs (one repeat per texture-size world units).
    let mut mesh = Mesh::from(Rectangle::from_size(size));
    let uv_scale = size / terrain_anim.texture_size.as_vec2();
    if let Some(VertexAttributeValues::Float32x2(uvs)) = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        for uv in uvs.iter_mut() {
            uv[0] *= uv_scale.x;
            uv[1] *= uv_scale.y;
        }
    }
    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from(texture))),
        Transform::from_xyz(size.x / 2.0, -size.y / 2.0, LAYER_TERRAIN_ANIM),
        TerrainAnimState {
            velocity: terrain_anim.velocity / terrain_anim.texture_size.as_vec2(),
        },
    ));
}
