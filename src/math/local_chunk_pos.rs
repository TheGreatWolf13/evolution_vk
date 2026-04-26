use crate::chunk::Section;
use crate::math::u8vec3::U8Vec3;
use light_ranged_integers::RangedU8;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LocalChunkPos(U8Vec3);

pub type Range = RangedU8<0, { Section::MASK }>;

impl Debug for LocalChunkPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("LocalChunkPos({:?}, {:?}, {:?})", self.x(), self.y(), self.z()))
    }
}

impl LocalChunkPos {
    #[inline(always)]
    #[must_use]
    pub fn new(x: Range, y: Range, z: Range) -> Self {
        Self(U8Vec3::new(x.inner(), y.inner(), z.inner()))
    }

    #[inline(always)]
    #[must_use]
    pub const fn x(&self) -> u8 {
        self.0.x()
    }

    #[inline(always)]
    #[must_use]
    pub const fn y(&self) -> u8 {
        self.0.y()
    }

    #[inline(always)]
    #[must_use]
    pub const fn z(&self) -> u8 {
        self.0.z()
    }
}