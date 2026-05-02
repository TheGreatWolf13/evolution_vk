use crate::client::model::BakedModel;
use crate::client::texture::{TextureInfo, TextureManager, TextureResolver};
use crate::client::vertex::{Vertex, VertexPosTex};
use crate::math::direction::Direction;
use crate::math::quat::Quat;
use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;
use enum_map::EnumMap;
use itertools::Either;
use log::warn;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use tuple_map::TupleMap2;

pub(super) struct RawModel {
    id: String,
    parent_or_elements: Either<String, Vec<Element>>,
    textures: TextureHolder,
}

#[derive(Debug)]
struct Element {
    from: Vec3,
    to: Vec3,
    rot: Option<Rotation>,
    faces: EnumMap<Direction, Option<Face>>,
}

struct TextureHolder(HashMap<TextureVar, Either<Option<String>, TextureVar>>);

#[derive(Debug)]
pub(super) struct Face {
    uv: (Vec2, Vec2),
    cullface: Option<Direction>,
    texture: TextureVar,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub(super) struct TextureVar(String);

#[derive(Debug)]
pub(super) struct Rotation(Quat);

pub(super) struct RawModelBuilder {
    id: String,
}

pub(super) struct RawModelBuilderTexture {
    id: String,
    textures: TextureHolder,
}

pub(super) struct RawModelBuilderTextureElement {
    id: String,
    textures: TextureHolder,
    elements: Vec<Element>,
}

impl RawModel {
    fn _bake_face(vertices: &mut Vec<VertexPosTex>, indices: &mut Vec<u32>, from: Vec3, to: Vec3, dir: Direction, uv: (Vec2, Vec2), texture: TextureInfo) {
        let uv = uv.map(|v| v * (1.0 / 16.0));
        let index = vertices.len() as u32;
        indices.extend([index, index + 1, index + 2, index + 1, index + 3, index + 2]);
        let from = dir.choose(from, to);
        let to = dir.choose(to, from);
        let mut index = 0;
        for horiz in dir.get_horizontal_neighbours() {
            let from = horiz.choose(from, to);
            let to = horiz.choose(to, from);
            for vert in dir.get_vertical_neighbours() {
                vertices.push(Vertex::new().pos(vert.choose(from, to)).uv(texture.get_mapped(index.into(), uv)));
                index += 1;
            }
        }
    }

    fn _get_texture(&self, var: &TextureVar, model_hierarchy: &Vec<&RawModel>, resolver: &TextureResolver, textures: &TextureManager) -> TextureInfo {
        for model in model_hierarchy {
            if let Some(e) = model.textures.0.get(var) {
                match e {
                    Either::Left(tex) => {
                        if let Some(tex) = tex {
                            return textures.get_texture(resolver.get_texture(tex));
                        }
                    }
                    Either::Right(var) => {
                        return self._get_texture(var, model_hierarchy, resolver, textures);
                    }
                }
            }
        }
        warn!("No texture defined for TextureVar {:?} on model {:?}", var.0, self.id);
        textures.get_texture(resolver.get_missing_texture())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_textures(&self) -> Iter<'_, TextureVar, Either<Option<String>, TextureVar>> {
        self.textures.0.iter()
    }

    pub fn builder(id: impl Into<String>) -> RawModelBuilder {
        RawModelBuilder {
            id: id.into(),
        }
    }

    pub fn bake(&self, models: &HashMap<String, RawModel>, resolver: &TextureResolver, textures: &&TextureManager) -> Option<BakedModel> {
        let mut geometry = EnumMap::default();
        let mut model_hierarchy = vec![];
        let mut model = self;
        model_hierarchy.push(model);
        let elements = loop {
            match &model.parent_or_elements {
                Either::Right(elements) => {
                    break elements;
                }
                Either::Left(parent_name) => {
                    if let Some(parent_model) = models.get(parent_name) {
                        model = parent_model;
                        model_hierarchy.push(model);
                    } //
                    else {
                        warn!("Could not find model {:?} referenced by model {:?}", parent_name, model.id);
                        return None;
                    }
                }
            }
        };
        for element in elements {
            let from = element.from * (1.0 / 16.0);
            let to = element.to * (1.0 / 16.0);
            for (dir, face) in &element.faces {
                if let Some(face) = face {
                    let texture = self._get_texture(&face.texture, &model_hierarchy, resolver, textures);
                    let (vertices, indices) = &mut geometry[face.cullface.into()];
                    Self::_bake_face(vertices, indices, from, to, dir, face.uv, texture);
                }
            }
        }
        Some(
            BakedModel {
                geometry
            }
        )
    }
}

impl RawModelBuilder {
    pub fn with_textures(self) -> RawModelBuilderTexture {
        RawModelBuilderTexture {
            id: self.id,
            textures: TextureHolder(HashMap::new()),
        }
    }
}

impl RawModelBuilderTexture {
    fn _add_texture(mut self, var: impl Into<String>, texture: Either<Option<String>, TextureVar>) -> Self {
        let var = TextureVar(var.into());
        if self.textures.0.contains_key(&var) {
            panic!("TextureVar {:?} already exists on model {:?}!", var.0, self.id);
        }
        self.textures.0.insert(var, texture);
        self
    }

    pub fn add_texture(self, var: impl Into<String>) -> Self {
        self._add_texture(var, Either::Left(None))
    }

    pub fn add_texture_and_bind(self, var: impl Into<String>, texture: impl Into<String>) -> Self {
        self._add_texture(var, Either::Left(Some(texture.into())))
    }

    pub fn add_texture_and_reference(self, var: impl Into<String>, texture: impl Into<String>) -> Self {
        let reference = TextureVar(texture.into());
        if !self.textures.0.contains_key(&reference) {
            panic!("TextureVar {:?} not found on model {:?}!", reference.0, self.id);
        }
        self._add_texture(var, Either::Right(reference))
    }

    pub fn with_parent(self, parent: impl Into<String>) -> RawModel {
        RawModel {
            id: self.id,
            textures: self.textures,
            parent_or_elements: Either::Left(parent.into()),
        }
    }

    pub fn with_elements(self) -> RawModelBuilderTextureElement {
        RawModelBuilderTextureElement {
            id: self.id,
            textures: self.textures,
            elements: Vec::new(),
        }
    }
}

impl RawModelBuilderTextureElement {
    pub fn add_element(mut self, from: impl Into<Vec3>, to: impl Into<Vec3>, rot: Option<Rotation>, f: impl Fn(&mut Self, Direction) -> Option<Face>) -> Self {
        let element = Element {
            from: from.into(),
            to: to.into(),
            rot,
            faces: EnumMap::from_fn(|d| f(&mut self, d)),
        };
        self.elements.push(element);
        self
    }

    pub fn add_face(&self, texture: impl Into<String>, cullface: Option<Direction>) -> Face {
        let var = TextureVar(texture.into());
        if !self.textures.0.contains_key(&var) {
            panic!("TextureVar {:?} not found on model {:?}!", var.0, self.id);
        }
        Face {
            uv: (Vec2::new(0.0, 0.0), Vec2::new(16.0, 16.0)),
            texture: var,
            cullface,
        }
    }

    pub fn build(self) -> RawModel {
        RawModel {
            id: self.id,
            textures: self.textures,
            parent_or_elements: Either::Right(self.elements),
        }
    }
}