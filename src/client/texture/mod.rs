pub(crate) use crate::client::texture::atlas::{Atlas, AtlasBuilder, TextureId, TextureInfo};
use image::RgbaImage;
use std::collections::{HashMap, HashSet};

mod atlas;
mod placer;

pub struct TextureManager {
    atlas: Atlas,
}

pub struct TextureResolver(HashMap<String, TextureId>);

impl TextureManager {
    pub fn new(textures_ids: HashSet<String>) -> (Self, TextureResolver) {
        let mut builder = AtlasBuilder::new("block".into());
        textures_ids.into_iter().for_each(|id| {
            builder.add_texture(id);
        });
        let (atlas, textures) = builder.build();
        (
            Self {
                atlas
            },
            TextureResolver(textures),
        )
    }

    pub fn get_texture(&self, id: TextureId) -> TextureInfo {
        self.atlas.get_sprite(id)
    }

    pub fn get_atlas_image(&self) -> &RgbaImage {
        self.atlas.get_texture()
    }
}

impl TextureResolver {
    pub fn get_missing_texture(&self) -> TextureId {
        TextureId(0)
    }

    pub fn get_texture(&self, id: &String) -> TextureId {
        self.0.get(id).copied().unwrap_or(self.get_missing_texture())
    }
}