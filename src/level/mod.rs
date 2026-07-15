use crate::level::chunk::Chunk;
use crate::math::chunk_pos::ChunkPos;
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;

pub mod chunk;

pub struct Level {
    chunks: HashMap<ChunkPos, Chunk>,
    min_y_section: i8,
    num_sections: usize,
}

impl Level {
    pub fn new(num_sections: usize, min_y_section: i8) -> Self {
        Self {
            chunks: HashMap::new(),
            min_y_section,
            num_sections,
        }
    }

    pub fn generate_terrain(&mut self) {
        for x in -8..8 {
            for z in -8..8 {
                let pos = ChunkPos::new(x, z);
                let chunk = Chunk::new(pos, self.num_sections);
                self.chunks.insert(pos, chunk);
            }
        }
    }

    pub fn get_chunks(&self) -> Iter<'_, ChunkPos, Chunk> {
        self.chunks.iter()
    }

    pub fn get_chunks_mut(&mut self) -> IterMut<'_, ChunkPos, Chunk> {
        self.chunks.iter_mut()
    }

    pub fn get_min_y_section(&self) -> i8 {
        self.min_y_section
    }
}