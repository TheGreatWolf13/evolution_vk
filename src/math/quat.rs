use crate::math::angle::Angle;
use crate::math::vec3f::Vec3F32;
use crate::math::vec3f::Vec3F64;
use crate::math::Lerp;
use crate::{impl_bin_op, impl_bin_op_transform};
use glam::DQuat as Q64;
use glam::Quat as Q32;
use std::ops::Mul;
use std::ops::MulAssign;
use super_seq_macro::seq;

seq!(N in (5..=6).collect().map(|i| 1 << i) {
    #[derive(Copy, Clone, Debug)]
    pub struct Quat~N(pub(super) Q~N);

    impl Quat~N {
        pub const IDENTITY: Self = Self(Q~N::IDENTITY);

        #[inline(always)]
        #[must_use]
        pub fn from_axis_angle(axis: impl Into<Vec3F~N>, angle: impl Angle) -> Self {
            let axis = axis.into();
            #[allow(clippy::unnecessary_cast)]
            Self(Q~N::from_axis_angle(axis.0, *angle.to_radians() as f~N))
        }
    }

    impl Lerp for Quat~N {

        #[inline(always)]
        fn lerp(&self, other: Self, t: f32) -> Self {
            #[allow(clippy::unnecessary_cast)]
            Self(other.0.lerp(self.0, t as f~N))
        }
    }

    //Mul
    impl_bin_op!(Quat~N * Quat~N: Mul mul, (self, rhs) => Self(self.0.mul_quat(rhs.0).normalize()));
    impl_bin_op_transform!(Quat~N * Vec3F~N: Mul mul, (self, rhs) => Vec3F~N(self.0 * rhs.0));
});