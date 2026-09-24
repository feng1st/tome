//! Repacking grid sprite sheets into vertically-stacked array textures,
//! so a whole tileset or flipbook renders as one texture with layer indices.

use bevy::asset::RenderAssetUsages;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// Repack a grid of `tile`-sized tiles into vertically-stacked array layers
/// (row-major tile order). Input/output are RGBA8 pixel buffers.
pub fn grid_to_array(image: &Image, tile: u32) -> Vec<u8> {
    let img_w = image.width();
    let img_h = image.height();
    let cols = img_w / tile;
    let rows = img_h / tile;
    let src = image.data.as_deref().expect("image pixel data");
    let mut out = vec![0u8; (tile * tile * 4 * cols * rows) as usize];
    for ty in 0..rows {
        for tx in 0..cols {
            let layer = ty * cols + tx;
            for py in 0..tile {
                let src_start = (((ty * tile + py) * img_w + tx * tile) * 4) as usize;
                let dst_start = ((layer * tile * tile + py * tile) * 4) as usize;
                let len = (tile * 4) as usize;
                out[dst_start..dst_start + len].copy_from_slice(&src[src_start..src_start + len]);
            }
        }
    }
    out
}

/// Extract animation frames from a repeating texture by sampling tile-sized
/// windows at the given horizontal offsets (wrapping like PD's REPEAT mode).
pub fn offset_frames(image: &Image, tile: u32, offsets: &[u32]) -> Vec<u8> {
    let w = image.width();
    let h = image.height();
    let src = image.data.as_deref().expect("image pixel data");
    let mut out = vec![0u8; (tile * tile * 4 * offsets.len() as u32) as usize];
    for (frame, &offset) in offsets.iter().enumerate() {
        for py in 0..tile {
            for px in 0..tile {
                let sx = (px + offset) % w;
                let sy = py % h;
                let s = ((sy * w + sx) * 4) as usize;
                let d = (((frame as u32) * tile * tile + py * tile + px) * 4) as usize;
                out[d..d + 4].copy_from_slice(&src[s..s + 4]);
            }
        }
    }
    out
}

pub fn array_image(data: Vec<u8>, tile: u32, layers: u32) -> Image {
    let mut image = Image::new(
        Extent3d {
            width: tile,
            height: tile,
            depth_or_array_layers: layers,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::nearest();
    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_repack_matches_source() {
        // 2x2 tiles of 2x2 pixels: tile index == fill color channel value.
        let tile = 2u32;
        let img_w = 4u32;
        let img_h = 4u32;
        let mut data = vec![0u8; (img_w * img_h * 4) as usize];
        for ty in 0..2u32 {
            for tx in 0..2u32 {
                let value = (ty * 2 + tx) as u8;
                for py in 0..tile {
                    for px in 0..tile {
                        let i = (((ty * tile + py) * img_w + tx * tile + px) * 4) as usize;
                        data[i] = value;
                    }
                }
            }
        }
        let mut image = Image::default();
        image.texture_descriptor.size = Extent3d {
            width: img_w,
            height: img_h,
            depth_or_array_layers: 1,
        };
        image.data = Some(data);

        let out = grid_to_array(&image, tile);
        // Each layer must be uniformly filled with its tile index.
        for layer in 0..4u32 {
            let base = (layer * tile * tile * 4) as usize;
            for i in 0..(tile * tile) as usize {
                assert_eq!(out[base + i * 4], layer as u8, "layer {layer} pixel {i}");
            }
        }
    }
}
