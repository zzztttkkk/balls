use bevy::asset::RenderAssetUsages;
use bevy::image::Image;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub(crate) fn create_circle_image(radius: u32) -> Image {
    let size = radius * 2;
    let mut data = vec![0; (size * size * 4) as usize];
    let center = radius as f32 - 0.5;
    
    for y in 0..size {
        for x in 0..size {
            let offset = ((y * size + x) * 4) as usize;
            let dist = ((x as f32 - center).powi(2) + (y as f32 - center).powi(2)).sqrt();

            let strength = 1.0 - (dist - (radius as f32 - 1.0)).clamp(0.0, 1.0);

            data[offset] = 255;
            data[offset + 1] = 255;
            data[offset + 2] = 255;
            data[offset + 3] = (strength * 255.0) as u8;
        }
    }

    return Image::new(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
}
