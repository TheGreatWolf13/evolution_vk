use crate::impl_from;
use crate::level::chunk::Section;
use crate::math::local_section_pos::{LocalSectionPos, Range};
use crate::math::vec3i::Vec3I32;
use crate::math::Vector3;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlockPos(Vec3I32);

impl Debug for BlockPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("BlockPos({:?}, {:?}, {:?})", self.x(), self.y(), self.z()))
    }
}

impl_from!(BlockPos as Vec3I32: pos => pos.0);

impl BlockPos {
    #[inline(always)]
    #[must_use]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(Vec3I32::new(x, y, z))
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
    pub fn z(&self) -> i32 {
        self.0.z()
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

    #[inline(always)]
    #[must_use]
    pub fn z_mut(&mut self) -> &mut i32 {
        self.0.z_mut()
    }

    #[inline(always)]
    #[must_use]
    pub fn get_local_pos(&self) -> LocalSectionPos {
        LocalSectionPos::new(Range::new((self.x() & Section::MASK as i32) as i8), Range::new((self.y() & Section::MASK as i32) as i8), Range::new((self.z() & Section::MASK as i32) as i8))
    }
}