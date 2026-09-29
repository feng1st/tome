//! The animated terrain layers: map-sized quads beneath the canvas
//! sprite, showing through the canvas's holes for their cells.
//! Hardwired to water for now; multi-kind support (a table plus
//! per-cell union meshes) is future work.

use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;

use crate::core::map::resources::current_map::CurrentMap;
use crate::frontend::display::canvas::constants::geometry::SCREEN_LAYERS;
use crate::frontend::display::constants::layout::TILE_SIZE;
use crate::frontend::display::terrain_animation::components::terrain_anim_state::TerrainAnimState;
use crate::frontend::display::terrain_animation::constants::terrain_anims::WATER_ANIM;

/// Z in the screen pass: beneath the canvas sprite (which sits at 0).
const LAYER_TERRAIN_ANIM: f32 = -1.0;

/// Spawn the animated terrain layers (today: the one water layer) on
/// entering `Game` — the map's spawn path. The quad matches the map
/// rectangle: world geometry never leaves it, so the void beyond the
/// map shows the window background through the canvas's transparent
/// pixels, not water. The follow rig positions it at
/// `world_anchor − target` (world-locked); the time-driven scroll
/// rides on top via `uv_transform`. Repeat-space UVs are baked into
/// the mesh, so playback only writes the translation.
///
/// The world pass draws nothing on water cells, leaving alpha-0 holes
/// in the canvas; premultiplied alpha compositing shows this layer
/// exactly through them — no mask pass, no shader. The chunk tiles'
/// semi-transparent shoreline pixels composite over it the same way.
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
    // Bake repeat-space UVs (one repeat per texture-size screen units).
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
        Transform::from_xyz(0.0, 0.0, LAYER_TERRAIN_ANIM),
        TerrainAnimState {
            // Anchored at the map center; the follow rig re-bases it
            // against the target on its first pass.
            world_anchor: Vec2::new(size.x / 2.0, -size.y / 2.0),
            velocity: terrain_anim.velocity / terrain_anim.texture_size.as_vec2(),
        },
        SCREEN_LAYERS,
    ));
}
