use crate::chunk::Section;
use crate::math::i8vec3::I8Vec3;
use crate::math::Vector3;
use light_ranged_integers::RangedI8;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LocalSectionPos(I8Vec3);

pub type Range = RangedI8<0, { Section::MASK }>;

impl Debug for LocalSectionPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("LocalChunkPos({:?}, {:?}, {:?})", self.x(), self.y(), self.z()))
    }
}

impl LocalSectionPos {
    #[inline(always)]
    #[must_use]
    pub fn new(x: Range, y: Range, z: Range) -> Self {
        Self(I8Vec3::new(x.inner(), y.inner(), z.inner()))
    }

    #[inline(always)]
    #[must_use]
    pub fn x(&self) -> i8 {
        self.0.x()
    }

    #[inline(always)]
    #[must_use]
    pub fn y(&self) -> i8 {
        self.0.y()
    }

    #[inline(always)]
    #[must_use]
    pub fn z(&self) -> i8 {
        self.0.z()
    }
}