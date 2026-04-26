use crate::chunk::Section;
use crate::math::ivec3::IVec3;
use crate::math::local_chunk_pos::{LocalChunkPos, Range};
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlockPos(IVec3);

impl Debug for BlockPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("BlockPos({:?}, {:?}, {:?})", self.x(), self.y(), self.z()))
    }
}

impl BlockPos {
    #[inline(always)]
    #[must_use]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(IVec3::new(x, y, z))
    }

    #[inline(always)]
    #[must_use]
    pub const fn x(&self) -> i32 {
        self.0.x()
    }

    #[inline(always)]
    #[must_use]
    pub const fn y(&self) -> i32 {
        self.0.y()
    }

    #[inline(always)]
    #[must_use]
    pub const fn z(&self) -> i32 {
        self.0.z()
    }

    #[inline(always)]
    #[must_use]
    pub const fn x_mut(&mut self) -> &mut i32 {
        self.0.x_mut()
    }

    #[inline(always)]
    #[must_use]
    pub const fn y_mut(&mut self) -> &mut i32 {
        self.0.y_mut()
    }

    #[inline(always)]
    #[must_use]
    pub const fn z_mut(&mut self) -> &mut i32 {
        self.0.z_mut()
    }

    #[inline(always)]
    #[must_use]
    pub fn get_local_pos(&self) -> LocalChunkPos {
        LocalChunkPos::new(Range::new((self.x() & Section::MASK as i32) as u8), Range::new((self.y() & Section::MASK as i32) as u8), Range::new((self.z() & Section::MASK as i32) as u8))
    }
}