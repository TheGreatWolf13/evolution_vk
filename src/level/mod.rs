use crate::level::chunk::Chunk;
use crate::math::chunk_pos::ChunkPos;
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;

pub mod chunk;

pub struct Level<const Y: usize> {
    chunks: HashMap<ChunkPos, Chunk<Y>>,
    min_y_section: i8,
}

impl<const Y: usize> Level<Y> {
    pub fn new(min_y_section: i8) -> Self {
        Self {
            chunks: HashMap::new(),
            min_y_section,
        }
    }

    pub fn generate_terrain(&mut self) {
        for x in -8..8 {
            for z in -8..8 {
                let pos = ChunkPos::new(x, z);
                let chunk = Chunk::<Y>::new(pos);
                self.chunks.insert(pos, chunk);
            }
        }
    }

    pub fn get_chunks(&self) -> Iter<'_, ChunkPos, Chunk<{ Y }>> {
        self.chunks.iter()
    }

    pub fn get_chunks_mut(&mut self) -> IterMut<'_, ChunkPos, Chunk<{ Y }>> {
        self.chunks.iter_mut()
    }

    pub fn get_min_y_section(&self) -> i8 {
        self.min_y_section
    }
}