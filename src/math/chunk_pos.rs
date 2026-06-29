use crate::math::section_pos::SectionPos;
use crate::math::vec2i::Vec2I32;
use crate::math::Vector2;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ChunkPos(Vec2I32);

impl Debug for ChunkPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("ChunkPos({:?}, {:?})", self.x(), self.y()))
    }
}

impl ChunkPos {
    #[inline(always)]
    #[must_use]
    pub fn new(x: i32, z: i32) -> Self {
        Self(Vec2I32::new(x, z))
    }

    #[inline(always)]
    #[must_use]
    pub fn x(&self) -> i32 {
        self.0.x()
    }

    #[inline(always)]
    #[must_use]
    pub fn y(&self) -> i32 {
        self.0.y()
    }

    #[inline(always)]
    #[must_use]
    pub fn x_mut(&mut self) -> &mut i32 {
        self.0.x_mut()
    }

    #[inline(always)]
    #[must_use]
    pub fn y_mut(&mut self) -> &mut i32 {
        self.0.y_mut()
    }

    pub fn with_section_z(self, z: i32) -> SectionPos {
        SectionPos::new(self.x(), self.y(), z)
    }
}