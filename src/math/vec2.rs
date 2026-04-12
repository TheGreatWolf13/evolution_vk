use crate::{impl_bin_op, impl_from};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct Vec2(glam::Vec2);

impl Debug for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("({:?}, {:?})", self.x(), self.y()))
    }
}

impl Vec2 {
    pub const ZERO: Self = Self(glam::Vec2::ZERO);
    pub const ONE: Self = Self(glam::Vec2::ONE);
    pub const X: Self = Self(glam::Vec2::X);
    pub const Y: Self = Self(glam::Vec2::Y);

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self(glam::Vec2::new(x, y))
    }

    #[inline]
    pub const fn splat(v: f32) -> Self {
        Self(glam::Vec2::splat(v))
    }

    #[inline]
    pub const fn x(&self) -> f32 {
        self.0.x
    }

    #[inline]
    pub const fn y(&self) -> f32 {
        self.0.y
    }

    #[inline]
    pub const fn x_mut(&mut self) -> &mut f32 {
        &mut self.0.x
    }

    #[inline]
    pub const fn y_mut(&mut self) -> &mut f32 {
        &mut self.0.y
    }
}

//Add
impl_bin_op!(Vec2 + Vec2: Add add, (self, rhs) => Self(self.0 + rhs.0));
impl_bin_op!(Vec2 + [f32; 2]: Add add, (self, rhs) => Self(self.0 + glam::Vec2::from(rhs)));
impl_bin_op!(Vec2 + (f32, f32): Add add, (self, rhs) => Self(self.0 + glam::Vec2::from(rhs)));
//Sub
impl_bin_op!(Vec2 - Vec2: Sub sub, (self, rhs) => Self(self.0 - rhs.0));
impl_bin_op!(Vec2 - [f32; 2]: Sub sub, (self, rhs) => Self(self.0 - glam::Vec2::from(rhs)));
impl_bin_op!(Vec2 - (f32, f32): Sub sub, (self, rhs) => Self(self.0 - glam::Vec2::from(rhs)));
//Mul
impl_bin_op!(Vec2 * f32: Mul mul, (self, rhs) => Self(self.0 * rhs));
//From
impl_from!((f32, f32) as Vec2: v => Self(glam::Vec2::from(v)));
impl_from!([f32; 2] as Vec2: v => Self(glam::Vec2::from(v)));
impl_from!(Vec2 as (f32, f32): v => (v.x(), v.y()));
impl_from!(Vec2 as [f32; 2]: v => [v.x(), v.y()]);