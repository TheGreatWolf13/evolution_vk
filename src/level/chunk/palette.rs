use crate::block::Block;
use crate::math::local_section_pos::LocalSectionPos;
use bitvec::field::BitField;
use bitvec::vec::BitVec;
use std::collections::HashMap;
use std::ops::Index;

pub struct BlockPallet(Palette<Block>);

enum Palette<T> {
    Single(T),
    Multi(IndexedMultiMap<T>, BitVec),
}

struct IndexedMultiMap<T> {
    bits_per_block: u8,
    index_to_t: Vec<T>,
    t_to_index: HashMap<T, u16>,
}

impl<T> IndexedMultiMap<T> {
    fn new() -> Self {
        Self {
            bits_per_block: 2,
            index_to_t: vec![],
            t_to_index: HashMap::new(),
        }
    }

    fn get_indexed(&self, data: &BitVec, pos: LocalSectionPos) -> u16 {
        let mut index = pos.z() as usize + pos.y() as usize * 16 + pos.x() as usize * (16 * 16);
        index *= self.bits_per_block as usize;
        data[index..(index + self.bits_per_block as usize)].load()
    }
}

impl<T> Index<u16> for IndexedMultiMap<T> {
    type Output = T;

    fn index(&self, index: u16) -> &Self::Output {
        &self.index_to_t[index as usize]
    }
}

impl BlockPallet {
    pub fn from_single(block: Block) -> Self {
        BlockPallet(Palette::Single(block))
    }

    pub fn get_block_at(&self, pos: LocalSectionPos) -> Block {
        match &self.0 {
            Palette::Single(b) => b,
            Palette::Multi(palette, data) => {
                let local_id = palette.get_indexed(&data, pos);
                palette[local_id]
            }
        }
    }
}