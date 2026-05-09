use crate::client::mesh::{Mesh, MeshBuilder};
use crate::client::model::ModelManager;
use crate::client::vertex::VertexPosTex;
use crate::level::chunk::palette::BlockPallet;
use crate::math::chunk_pos::ChunkPos;
use crate::math::direction::Direction;
use crate::math::local_section_pos::LocalSectionPos;
use crate::math::mat4::Mat4;
use crate::Block;
use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use enum_iterator::all;
use itertools::Itertools;
use std::sync::Arc;
use vulkano::memory::allocator::StandardMemoryAllocator;

mod palette;

pub struct Chunk<const Y: usize> {
    pos: ChunkPos,
    sections: [Section; Y],
}

pub struct Section {
    index: u8,
    blocks: BlockPallet,
    mesh: Option<Mesh<VertexPosTex>>,
    dirty: bool,
}

impl Section {
    pub const SIZE: i8 = 32;
    pub const MASK: i8 = Self::SIZE - 1;

    pub fn get_mesh(&self) -> Option<&Mesh<VertexPosTex>> {
        self.mesh.as_ref()
    }

    pub fn remesh(&mut self, pos: ChunkPos, min_y_section: i8, model_manager: &ModelManager, allocator: Arc<StandardMemoryAllocator>) {
        if self.dirty {
            let mut builder = MeshBuilder::new(Mat4::from_translation((pos.x() as f32 * Section::SIZE as f32, (self.index as i32 + min_y_section as i32) as f32 * Section::SIZE as f32, pos.z() as f32 * Section::SIZE as f32)));
            for x in 0..Self::SIZE {
                for y in 0..Self::SIZE {
                    for z in 0..Self::SIZE {
                        let pos = LocalSectionPos::new(x.into(), y.into(), z.into());
                        let block = self.blocks.get_block_at(pos);
                        if block != Block!(AIR) {
                            let mut faces = BitVec::<usize, Lsb0>::new();
                            for dir in all::<Direction>() {
                                let neighbour_pos = pos.offset(dir);
                                if neighbour_pos.is_out_of_range() || self.blocks.get_block_at(neighbour_pos) == Block!(AIR) {
                                    faces.push(true);
                                } //
                                else {
                                    faces.push(false);
                                }
                            }
                            builder = builder.local_transform(Mat4::from_translation((x as f32, y as f32, z as f32))).model(model_manager.get_model(block), faces);
                        }
                    }
                }
            }
            self.mesh = builder.build(allocator);
            self.dirty = false;
        }
    }
}

impl<const Y: usize> Chunk<Y> {
    pub fn new(pos: ChunkPos) -> Self {
        Self {
            pos,
            sections: (0..Y).map(|i| {
                Section {
                    index: i as u8,
                    blocks: BlockPallet::from_single(match i {
                        0 | 1 => Block!(COBBLESTONE),
                        2 => Block!(STONE),
                        3 => Block!(DIRT),
                        _ => Block!(AIR),
                    }),
                    mesh: None,
                    dirty: true,
                }
            }).next_array().unwrap(),
        }
    }

    pub fn get_pos(&self) -> ChunkPos {
        self.pos
    }

    pub fn get_sections(&self) -> &[Section; Y] {
        &self.sections
    }

    pub fn get_sections_mut(&mut self) -> &mut [Section; Y] {
        &mut self.sections
    }
}