use crate::client::engine::buffer::MappedRegion;
use crate::client::mesh::{MeshBuilder, SectionMesh};
use crate::client::model::ModelManager;
use crate::client::vertex::VertexPosTex;
use crate::level::chunk::palette::BlockPallet;
use crate::math::chunk_pos::ChunkPos;
use crate::math::direction::Direction;
use crate::math::local_section_pos::LocalSectionPos;
use crate::math::mat4::Mat4;
use crate::math::section_pos::SectionPos;
use crate::Block;
use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use enum_iterator::all;
use itertools::Itertools;

mod palette;

pub struct Chunk<const Y: usize> {
    pos: ChunkPos,
    sections: [Section; Y],
}

pub struct Section {
    index: u8,
    blocks: BlockPallet,
    mesh_region: Option<MappedRegion>,
    dirty: bool,
}

impl Section {
    pub const SIZE: i8 = 32;
    pub const MASK: i8 = Self::SIZE - 1;

    pub fn get_transform(&self, pos: SectionPos) -> Mat4 {
        Mat4::from_translation((pos.x() as f32 * Self::SIZE as f32, pos.y() as f32, pos.z() as f32 * Self::SIZE as f32))
    }

    pub fn get_pos(&self, chunk_pos: ChunkPos, min_y_section: i8) -> SectionPos {
        chunk_pos.with_section_y(self.index as i32 + min_y_section as i32)
    }

    pub fn get_mesh_region(&self) -> Option<&MappedRegion> {
        self.mesh_region.as_ref()
    }

    pub fn remesh(&'_ mut self, pos: ChunkPos, min_y_section: i8, model_manager: &ModelManager) -> Option<SectionMesh<'_, VertexPosTex>> {
        if self.dirty {
            let mut builder = MeshBuilder::new();
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
            self.dirty = false;
            Some(builder.build_section(pos.with_section_y(self.index as i32 + min_y_section as i32), &mut self.mesh_region))
        } //
        else {
            None
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
                    mesh_region: None,
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
