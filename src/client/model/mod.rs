mod raw_model;

use crate::block::{Block, BlockId, Blocks};
use crate::client::model::raw_model::RawModel;
use crate::client::texture::{TextureInfo, TextureManager, TextureResolver};
use crate::client::vertex::{Vertex, VertexPosTex};
use crate::math::direction::Direction;
use crate::math::vec3::Vec3;
use crate::Block;
use enum_map::{enum_map, Enum, EnumMap};
use log::warn;
use std::collections::{HashMap, HashSet};

pub struct ModelManager {
    missing_model: BakedModel,
    models: HashMap<BlockId, BakedModel>,
}

pub(super) struct ModelLoader {
    models: HashMap<String, RawModel>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Enum)]
enum Cullface {
    None,
    Some(Direction),
}

pub struct BakedModel {
    geometry: EnumMap<Cullface, (Vec<VertexPosTex>, Vec<u32>)>,
}

impl ModelLoader {
    fn add_model(&mut self, model: RawModel) {
        self.models.insert(model.get_id().into(), model).map(|model| {
            warn!("Overwriting model with id {:?}", model.get_id());
        });
    }

    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    pub fn load_models(&mut self) {
        self.add_model(RawModel::builder("block")
                           .with_textures()
                           .add_texture("up")
                           .add_texture("down")
                           .add_texture("north")
                           .add_texture("south")
                           .add_texture("east")
                           .add_texture("west")
                           .with_elements()
                           .add_element((0.0, 0.0, 0.0), (16.0, 16.0, 16.0), None, |builder, dir| Some(builder.add_face(dir.get_name(), Some(dir))))
                           .build(),
        );
        self.add_model(RawModel::builder("block_all")
                           .with_textures()
                           .add_texture("all")
                           .add_texture_and_reference("up", "all")
                           .add_texture_and_reference("down", "all")
                           .add_texture_and_reference("north", "all")
                           .add_texture_and_reference("south", "all")
                           .add_texture_and_reference("east", "all")
                           .add_texture_and_reference("west", "all")
                           .with_parent("block"),
        );
        self.add_model(RawModel::builder("stone")
            .with_textures()
            .add_texture_and_bind("all", "stone")
            .with_parent("block_all")
        );
        self.add_model(RawModel::builder("cobblestone")
            .with_textures()
            .add_texture_and_bind("all", "cobblestone")
            .with_parent("block_all")
        );
        self.add_model(RawModel::builder("dirt")
            .with_textures()
            .add_texture_and_bind("all", "dirt")
            .with_parent("block_all")
        );
    }

    pub fn gather_textures(&self) -> HashSet<String> {
        let mut textures = HashSet::new();
        self.models.values().for_each(|model| {
            textures.extend(model.get_textures().map(|e| e.1.as_ref().left()).flatten().flatten().cloned());
        });
        textures
    }
}

impl ModelManager {
    pub(super) fn bake(loader: ModelLoader, textures: TextureResolver, texture_manager: &TextureManager) -> Self {
        let missing = texture_manager.get_texture(textures.get_missing_texture());
        let mut models = HashMap::new();
        for block in Blocks::all().filter(|block| block != &Block!(AIR)) {
            if let Some(model) = loader.models.get(block.get_name_id()) {
                if let Some(model) = model.bake(&loader.models, &textures, &texture_manager) {
                    models.insert(block.get_id(), model);
                }
            } //
            else {
                warn!("Could not find model for block {:?}!", block.get_name_id());
                continue;
            }
        }
        Self {
            missing_model: BakedModel {
                geometry: enum_map! {
                    Cullface::None => (vec![], vec![]),
                    Cullface::Some(Direction::North) => Self::get_face((0.0, 0.0, 0.0), (1.0, 1.0, 1.0), Direction::North, missing),
                    Cullface::Some(Direction::East) => Self::get_face((0.0, 0.0, 0.0), (1.0, 1.0, 1.0), Direction::East, missing),
                    Cullface::Some(Direction::South) => Self::get_face((0.0, 0.0, 0.0), (1.0, 1.0, 1.0), Direction::South, missing),
                    Cullface::Some(Direction::West) => Self::get_face((0.0, 0.0, 0.0), (1.0, 1.0, 1.0), Direction::West, missing),
                    Cullface::Some(Direction::Up) => Self::get_face((0.0, 0.0, 0.0), (1.0, 1.0, 1.0), Direction::Up, missing),
                    Cullface::Some(Direction::Down) => Self::get_face((0.0, 0.0, 0.0), (1.0, 1.0, 1.0), Direction::Down, missing),
                },
            },
            models,
        }
    }

    pub fn get_model(&self, block: Block) -> &BakedModel {
        &self.models.get(&block.get_id()).unwrap_or(&self.missing_model)
    }

    fn get_face(from: impl Into<Vec3>, to: impl Into<Vec3>, dir: Direction, texture: TextureInfo) -> (Vec<VertexPosTex>, Vec<u32>) {
        let from = from.into();
        let to = to.into();
        let mut vertices = vec![];
        let indices = vec![0, 1, 2, 1, 3, 2];
        let from = dir.choose(from, to);
        let to = dir.choose(to, from);
        let mut index = 0;
        for horiz in dir.get_horizontal_neighbours() {
            let from = horiz.choose(from, to);
            let to = horiz.choose(to, from);
            for vert in dir.get_vertical_neighbours() {
                vertices.push(Vertex::new().pos(vert.choose(from, to)).uv(texture.get_raw(index.into())));
                index += 1;
            }
        }
        (vertices, indices)
    }
}

impl BakedModel {
    pub fn get_data(&self, cullface: Option<Direction>) -> &(Vec<VertexPosTex>, Vec<u32>) {
        &self.geometry[cullface.into()]
    }
}

impl From<Option<Direction>> for Cullface {
    #[inline(always)]
    fn from(direction: Option<Direction>) -> Self {
        match direction {
            None => Cullface::None,
            Some(d) => Cullface::Some(d),
        }
    }
}