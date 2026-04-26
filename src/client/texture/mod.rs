use crate::client::texture::atlas::{Atlas, AtlasBuilder};
use image::{ExtendedColorType, ImageFormat};

mod atlas;
mod placer;

pub struct TextureManager {
    atlas: Atlas,
}

impl TextureManager {
    pub fn new() -> Self {
        let mut builder = AtlasBuilder::new("block".into());
        let _ = builder.add_texture("cobblestone".into());
        let _ = builder.add_texture("stone".into());
        let atlas = builder.build();
        let image = atlas.get_texture();
        image::save_buffer_with_format("block_atlas.png", image, image.width(), image.height(), ExtendedColorType::Rgba8, ImageFormat::Png).unwrap();
        Self {
            atlas
        }
    }
}