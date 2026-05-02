use crate::math::ivec3::IVec3;
use crate::math::vec4::Vec4;
use crate::math::Vector3;
use crate::{impl_bin_op, impl_from, impl_un_op, impl_vec3};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, PartialEq)]
pub struct Vec3(pub(super) glam::Vec3);

impl Vec3 {
    pub const ZERO: Self = Self(glam::Vec3::ZERO);
    pub const ONE: Self = Self(glam::Vec3::ONE);
    pub const X: Self = Self(glam::Vec3::X);
    pub const Y: Self = Self(glam::Vec3::Y);
    pub const Z: Self = Self(glam::Vec3::Z);

    #[inline]
    #[must_use]
    pub fn cross(self, other: Self) -> Self {
        Self(self.0.cross(other.0))
    }

    #[inline]
    #[must_use]
    pub fn normalize_and_len(self) -> (Self, f32) {
        let (vec, len) = self.0.normalize_and_length();
        (Self(vec), len)
    }

    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        Self(self.0.normalize())
    }

    #[doc(alias = "magnitude")]
    #[inline]
    #[must_use]
    pub fn len(self) -> f32 {
        self.0.length()
    }

    #[doc(alias = "magnitude2")]
    #[inline]
    #[must_use]
    pub fn len_sqr(self) -> f32 {
        self.0.length_squared()
    }

    #[inline]
    #[must_use]
    pub fn horiz_len_sqr(self) -> f32 {
        self.x() * self.x() + self.z() * self.z()
    }

    #[inline]
    #[must_use]
    pub fn horiz_len(self) -> f32 {
        self.horiz_len_sqr().sqrt()
    }

    #[inline(always)]
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(glam::Vec3::new(x, y, z))
    }

    #[inline]
    #[must_use]
    pub const fn splat(a: f32) -> Self {
        Self(glam::Vec3::splat(a))
    }

    #[inline]
    #[must_use]
    pub fn to_homogeneous(&self) -> Vec4 {
        Vec4(self.0.to_homogeneous())
    }
}

//Vec
impl_vec3!(Vec3: f32 => 0 x y z);
//Add
impl_bin_op!(Vec3 + Vec3: Add add, (self, rhs) => Self(self.0 + rhs.0));
impl_bin_op!(Vec3 + (f32, f32, f32): Add add, (self, rhs) => Self(self.0 + glam::Vec3::from(rhs)));
impl_bin_op!(Vec3 + [f32; 3]: Add add, (self, rhs) => Self(self.0 + glam::Vec3::from(rhs)));
impl_bin_op!([f32; 3] + Vec3: Add add, (self, rhs) => [self[0] + rhs.x(), self[1] + rhs.y(), self[2] + rhs.z()]);
//Sub
impl_bin_op!(Vec3 - Vec3: Sub sub, (self, rhs) => Self(self.0 - rhs.0));
impl_bin_op!(Vec3 - (f32, f32, f32): Sub sub, (self, rhs) => Self(self.0 - glam::Vec3::from(rhs)));
impl_bin_op!(Vec3 - [f32; 3]: Sub sub, (self, rhs) => Self(self.0 - glam::Vec3::from(rhs)));
impl_bin_op!([f32; 3] - Vec3: Sub sub, (self, rhs) => [self[0] - rhs.x(), self[1] - rhs.y(), self[2] - rhs.z()]);
//Mul
impl_bin_op!(Vec3 * Vec3: Mul mul, (self, rhs) => Self(self.0 * rhs.0));
impl_bin_op!(Vec3 * (f32, f32, f32): Mul mul, (self, rhs) => Self(self.0 * glam::Vec3::from(rhs)));
impl_bin_op!(Vec3 * [f32; 3]: Mul mul, (self, rhs) => Self(self.0 * glam::Vec3::from(rhs)));
impl_bin_op!(Vec3 * f32: Mul mul, (self, rhs) => Self(self.0 * rhs));
//Neg
impl_un_op!(-Vec3: Neg neg, self => Self(-self.0));
//From 
impl_from!((f32, f32, f32) as Vec3: v => Self(glam::Vec3::from(v)));
impl_from!([f32; 3] as Vec3: v => Self(glam::Vec3::from(v)));
impl_from!(Vec3 as (f32, f32, f32): v => (v.x(), v.y(), v.z()));
impl_from!(Vec3 as [f32; 3]: v => [v.x(), v.y(), v.z()]);
impl_from!(IVec3 as Vec3: v => Self(v.0.as_vec3()));