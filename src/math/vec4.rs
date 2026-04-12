use crate::impl_from;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone)]
pub struct Vec4(pub(super) glam::Vec4);

impl Debug for Vec4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("({:?}, {:?}, {:?}, {:?})", self.x(), self.y(), self.z(), self.w()))
    }
}

impl Vec4 {
    #[inline(always)]
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Self(glam::Vec4::new(x, y, z, w))
    }

    #[inline(always)]
    #[must_use]
    pub fn x(&self) -> f32 {
        self.0.x
    }

    #[inline(always)]
    #[must_use]
    pub fn y(&self) -> f32 {
        self.0.y
    }

    #[inline(always)]
    #[must_use]
    pub fn z(&self) -> f32 {
        self.0.z
    }

    #[inline(always)]
    #[must_use]
    pub fn w(&self) -> f32 {
        self.0.w
    }
}

//From
impl_from!(Vec4 as [f32; 4]: v => v.0.to_array());
impl_from!(Vec4 as (f32, f32, f32, f32): v => v.0.into());
impl_from!([f32; 4] as Vec4: v => Self(glam::Vec4::from(v)));
impl_from!((f32, f32, f32, f32) as Vec4: v => Self(glam::Vec4::from(v)));