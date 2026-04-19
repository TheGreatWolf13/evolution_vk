use crate::math::angle::Angle;
use crate::math::quat::Quat;
use crate::math::vec3::Vec3;
use crate::math::vec4::Vec4;
use crate::{impl_bin_op, impl_from};
use std::ops::MulAssign;
use std::ops::SubAssign;
use std::ops::{Add, Mul};
use std::ops::{AddAssign, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Mat4(glam::Mat4);

impl Mat4 {
    pub const IDENTITY: Self = Self(glam::Mat4::IDENTITY);

    #[inline]
    #[must_use]
    pub const fn from_cols_array(array: &[f32; 16]) -> Self {
        Self(glam::Mat4::from_cols_array(array))
    }

    #[inline(always)]
    #[must_use]
    pub fn from_cols(x: impl Into<Vec4>, y: impl Into<Vec4>, z: impl Into<Vec4>, w: impl Into<Vec4>) -> Self {
        let x = x.into();
        let y = y.into();
        let z = z.into();
        let w = w.into();
        Self(glam::Mat4::from_cols(x.0, y.0, z.0, w.0))
    }

    #[inline]
    #[must_use]
    pub fn from_quat(rotation: Quat) -> Self {
        Self(glam::Mat4::from_quat(rotation.0))
    }

    #[inline]
    #[must_use]
    pub fn from_translation(translation: impl Into<Vec3>) -> Self {
        let translation = translation.into();
        Self(glam::Mat4::from_translation(translation.0))
    }

    #[inline]
    pub fn look_to(eye: impl Into<Vec3>, target: impl Into<Vec3>, up: impl Into<Vec3>) -> Self {
        let eye = eye.into();
        let target = target.into();
        let up = up.into();
        Self(glam::Mat4::look_to_rh(eye.0, target.0, up.0))
    }

    #[inline]
    #[must_use]
    pub fn perspective(fov_y: impl Angle, aspect: f32, near: f32, far: f32) -> Self {
        Self(glam::Mat4::perspective_rh(*fov_y.to_radians(), aspect, near, far))
    }
}

//Add
impl_bin_op!(Mat4 + Mat4: Add add, (self, rhs) => Self(self.0 + rhs.0));
//Sub
impl_bin_op!(Mat4 - Mat4: Sub sub, (self, rhs) => Self(self.0 - rhs.0));
//Mul
impl_bin_op!(Mat4 * Mat4: Mul mul, (self, rhs) => Self(self.0 * rhs.0));
//From
impl_from!(Mat4 as [[f32; 4]; 4]: v => v.0.to_cols_array_2d());
