use static_init::dynamic;
use static_init::lazy::lesser_locked_lazy::ReadGuard;
use std::fmt::{Debug, Formatter};

pub type Block = &'static BlockInner;

#[dynamic]
static mut REGISTRY: Vec<Block> = vec![];

#[dynamic]
pub static AIR: Block = BlockInner::new("air");
#[dynamic]
pub static STONE: Block = BlockInner::new("stone");

pub struct BlockInner {
    name_id: &'static str,
    id: BlockId,
}

impl Debug for BlockInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Block({:?})", &self.name_id))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlockId(u32);

impl BlockInner {
    fn new(name_id: &'static str) -> Block {
        let mut registry = REGISTRY.write();
        let block = Self {
            id: BlockId(registry.len() as u32),
            name_id,
        };
        let block = Box::leak(Box::new(block));
        registry.push(block);
        block
    }

    pub fn get_name_id(&self) -> &'static str {
        self.name_id
    }

    pub fn get_id(&self) -> BlockId {
        self.id
    }
}

pub struct Blocks;

struct BlockIter {
    guard: ReadGuard<'static, Vec<Block>>,
    index: usize,
}

impl Iterator for BlockIter {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.guard.get(self.index).copied();
        self.index += 1;
        item
    }
}

impl Blocks {
    pub fn all() -> impl Iterator<Item = Block> {
        let guard = REGISTRY.read();
        BlockIter {
            guard,
            index: 0,
        }
    }

    pub fn from_id(id: BlockId) -> Block {
        REGISTRY.read()[id.0 as usize]
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        self.id == other.id
    }
}

impl Eq for Block {}

#[allow(non_snake_case)]
#[macro_export]
macro_rules! Block {
    ($name:ident) => {
        *crate::block::$name
    };
}
