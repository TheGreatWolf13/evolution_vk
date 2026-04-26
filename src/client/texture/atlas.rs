use crate::client::texture::placer::TexturePlacer;
use crate::math::uvec2::UVec2;
use crate::math::vec2::Vec2;
use image::{Rgba, RgbaImage};
use log::warn;
use std::collections::HashMap;
use std::env::current_dir;

#[derive(Copy, Clone)]
pub struct TextureInfo {
    pub id: TextureId,
    pub uv0: Vec2,
    pub uv1: Vec2,
}

#[derive(Copy, Clone)]
pub struct RawTextureInfo(pub UVec2, pub UVec2);

impl PartialEq for TextureInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for TextureInfo {}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TextureId(usize);

pub struct Atlas {
    placed_map: Vec<TextureInfo>,
    texture: RgbaImage,
}

impl Atlas {
    pub fn get_texture(&self) -> &RgbaImage {
        &self.texture
    }
}

pub struct AtlasBuilder {
    base_path: String,
    used_textures: HashMap<String, TextureId>,
    textures: Vec<RgbaImage>,
}

impl AtlasBuilder {
    pub fn new(base_path: String) -> Self {
        const BASE_SIZE: u32 = 16;
        let missing_texture = RgbaImage::from_fn(BASE_SIZE, BASE_SIZE, |x, y| {
            if x < BASE_SIZE / 2 {
                if y < BASE_SIZE / 2 {
                    Rgba([0, 0, 0, 0xFF])
                } //
                else {
                    Rgba([0xF8, 0, 0xF8, 0xFF])
                }
            } //
            else {
                if y < BASE_SIZE / 2 {
                    Rgba([0xF8, 0, 0xF8, 0xFF])
                } //
                else {
                    Rgba([0, 0, 0, 0xFF])
                }
            }
        });
        Self {
            base_path,
            used_textures: HashMap::new(),
            textures: vec![missing_texture],
        }
    }

    pub fn add_texture(&mut self, path: String) -> TextureId {
        if self.used_textures.contains_key(&path) {
            return self.used_textures[&path];
        }
        let texture = match image::open(current_dir().unwrap().join(format!("res/assets/textures/{}/{}.png", self.base_path, path))) {
            Ok(i) => {
                if i.width() % 1 != 0 || i.height() % 1 != 0 {
                    warn!("Texture {}:{} has non power of two dimensions which may cause rendering artifacts!", self.base_path, path);
                }
                i.to_rgba8()
            }
            Err(e) => {
                warn!("Could not load texture {}:{} => Error: {}", self.base_path, path, e);
                return TextureId(0);
            }
        };
        let id = TextureId(self.textures.len());
        self.used_textures.insert(path, id);
        self.textures.push(texture);
        id
    }

    pub fn build(self) -> Atlas {
        let mut placer = TexturePlacer::new((256, 256));
        let atlas = self.textures.iter().map(|texture| {
            placer.place((texture.width(), texture.height()))
        }).collect::<Vec<_>>();
        let uv = placer.get_uv();
        let mut image = RgbaImage::new(uv.get_size().x(), uv.get_size().y());
        self.textures.iter().enumerate().for_each(|(i, texture)| {
            let offset = atlas[i];
            texture.enumerate_pixels().for_each(|(x, y, pixel)| {
                image.put_pixel(x + offset.0.x(), y + offset.0.y(), *pixel);
            })
        });
        Atlas {
            placed_map: atlas.into_iter().enumerate().map(|(id, info)| uv.get_uv(TextureId(id), info)).collect(),
            texture: image,
        }
    }
}