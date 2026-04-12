use crate::impl_bin_op_transform;
use crate::math::angle::Angle;
use crate::math::vec3::Vec3;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Quat(pub(super) glam::Quat);

impl Quat {
    #[inline]
    #[must_use]
    pub fn from_axis_angle(axis: impl Into<Vec3>, angle: impl Angle) -> Self {
        let axis = axis.into();
        Self(glam::Quat::from_axis_angle(axis.0, *angle.to_radians()))
    }
}

//Mul
impl_bin_op_transform!(Quat * Vec3: Mul mul, (self, rhs) => Vec3(self.0 * rhs.0));