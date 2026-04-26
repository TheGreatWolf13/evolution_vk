use crate::math::ivec2::IVec2;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChunkPos(IVec2);

impl Debug for ChunkPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("ChunkPos({:?}, {:?})", self.x(), self.z()))
    }
}

impl ChunkPos {
    #[inline(always)]
    #[must_use]
    pub fn new(x: i32, z: i32) -> Self {
        Self(IVec2::new(x, z))
    }

    #[inline(always)]
    #[must_use]
    pub const fn x(&self) -> i32 {
        self.0.x()
    }

    #[inline(always)]
    #[must_use]
    pub const fn z(&self) -> i32 {
        self.0.y()
    }

    #[inline(always)]
    #[must_use]
    pub const fn x_mut(&mut self) -> &mut i32 {
        self.0.x_mut()
    }

    #[inline(always)]
    #[must_use]
    pub const fn z_mut(&mut self) -> &mut i32 {
        self.0.y_mut()
    }
}