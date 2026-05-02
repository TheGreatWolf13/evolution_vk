use crate::client::texture::placer::TexturePlacer;
use crate::math::uvec2::UVec2;
use crate::math::vec2::Vec2;
use crate::math::PaP;
use image::{ExtendedColorType, ImageFormat, Rgba, RgbaImage};
use light_ranged_integers::RangedU8;
use log::warn;
use std::collections::HashMap;
use std::env::current_dir;

#[derive(Copy, Clone, Debug)]
pub struct TextureInfo {
    id: TextureId,
    uv0: Vec2,
    uv1: Vec2,
}

#[derive(Copy, Clone)]
pub struct RawTextureInfo(pub UVec2, pub UVec2);

impl PartialEq for TextureInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for TextureInfo {}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TextureId(pub(super) usize);

#[derive(Debug)]
pub struct Atlas {
    sprites: Vec<TextureInfo>,
    texture: RgbaImage,
}

impl Atlas {
    pub fn get_texture(&self) -> &RgbaImage {
        &self.texture
    }

    pub fn get_sprite(&self, id: TextureId) -> TextureInfo {
        self.sprites[id.0]
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

    pub fn add_texture(&mut self, path: String) {
        if self.used_textures.contains_key(&path) {
            return;
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
                return;
            }
        };
        let id = TextureId(self.textures.len());
        self.used_textures.insert(path, id);
        self.textures.push(texture);
    }

    pub fn build(self) -> (Atlas, HashMap<String, TextureId>) {
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
        image::save_buffer_with_format("block_atlas.png", &image, image.width(), image.height(), ExtendedColorType::Rgba8, ImageFormat::Png).unwrap();
        (
            Atlas {
                sprites: atlas.into_iter().enumerate().map(|(id, info)| uv.get_uv(TextureId(id), info)).collect(),
                texture: image,
            },
            self.used_textures
        )
    }
}

impl TextureInfo {
    pub fn new(id: TextureId, uv0: Vec2, uv1: Vec2) -> Self {
        Self {
            id,
            uv0,
            uv1,
        }
    }

    pub fn get_00(&self) -> Vec2 {
        self.uv0
    }

    pub fn get_01(&self) -> Vec2 {
        Vec2::new(self.uv0.x(), self.uv1.y())
    }

    pub fn get_10(&self) -> Vec2 {
        Vec2::new(self.uv1.x(), self.uv0.y())
    }

    pub fn get_11(&self) -> Vec2 {
        self.uv1
    }

    pub fn get_raw(&self, index: RangedU8<0, 3>) -> Vec2 {
        match index.inner() {
            0 => self.get_00(),
            1 => self.get_01(),
            2 => self.get_10(),
            3 => self.get_11(),
            _ => unreachable!(),
        }
    }

    pub fn get_mapped(&self, index: RangedU8<0, 3>, uv: (Vec2, Vec2)) -> Vec2 {
        let u = PaP(self.uv0.x(), self.uv1.x());
        let v = PaP(self.uv0.y(), self.uv1.y());
        match index.inner() {
            0 => Vec2::new(u.lerp(uv.0.x()), v.lerp(uv.0.y())),
            1 => Vec2::new(u.lerp(uv.0.x()), v.lerp(uv.1.y())),
            2 => Vec2::new(u.lerp(uv.1.x()), v.lerp(uv.0.y())),
            3 => Vec2::new(u.lerp(uv.1.x()), v.lerp(uv.1.y())),
            _ => unreachable!(),
        }
    }
}