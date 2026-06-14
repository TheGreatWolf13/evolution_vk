use std::fmt;
use std::fmt::{Debug, Formatter};
use crate::math::ivec3::IVec3;
use crate::math::Vector3;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SectionPos(IVec3);

impl Debug for SectionPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("SectionPos({:?}, {:?}, {:?})", self.x(), self.y(), self.z()))
    }
}

impl SectionPos {
    #[inline(always)]
    #[must_use]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(IVec3::new(x, y, z))
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
}