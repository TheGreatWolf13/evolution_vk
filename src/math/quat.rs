use crate::math::angle::Angle;
use crate::math::vec3::Vec3;
use crate::math::Lerp;
use crate::{impl_bin_op, impl_bin_op_transform};
use std::ops::Mul;
use std::ops::MulAssign;

#[derive(Debug, Clone, Copy)]
pub struct Quat(pub(super) glam::Quat);

impl Quat {
    pub const IDENTITY: Self = Quat(glam::Quat::IDENTITY);

    #[inline]
    #[must_use]
    pub fn from_axis_angle(axis: impl Into<Vec3>, angle: impl Angle) -> Self {
        let axis = axis.into();
        Self(glam::Quat::from_axis_angle(axis.0, *angle.to_radians()))
    }
}

impl Lerp for Quat {
    fn lerp(&self, other: Self, t: f32) -> Self {
        Quat(other.0.lerp(self.0, t))
    }
}

//Mul
impl_bin_op!(Quat * Quat: Mul mul, (self, rhs) => Quat(self.0.mul_quat(rhs.0).normalize()));
impl_bin_op_transform!(Quat * Vec3: Mul mul, (self, rhs) => Vec3(self.0 * rhs.0));