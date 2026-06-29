use crate::level::chunk::Section;
use crate::math::direction::{Axis3, Direction3};
use crate::math::vec3i::Vec3I8;
use crate::math::Vector3;
use crate::util::Utils;
use enum_iterator::all;
use light_ranged_integers::RangedI8;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LocalSectionPos(Vec3I8);

pub type Range = RangedI8<0, { Section::MASK }>;

impl Debug for LocalSectionPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{}({:?}, {:?}, {:?})", Utils::name_of::<Self>(), self.x(), self.y(), self.z()))
    }
}

impl LocalSectionPos {
    #[inline(always)]
    #[must_use]
    pub fn new(x: Range, y: Range, z: Range) -> Self {
        Self(Vec3I8::new(x.inner(), y.inner(), z.inner()))
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

    pub fn offset(&self, dir: Direction3) -> Self {
        Self(self.0 + dir.get_offset::<Vec3I8>())
    }

    pub fn is_out_of_range(&self) -> bool {
        for axis in all::<Axis3>() {
            if self.0.get(axis) < 0 || self.0.get(axis) >= Section::SIZE {
                return true;
            }
        }
        false
    }
}