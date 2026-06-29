use crate::math::angle::Angle;
use crate::math::quat::Quat32;
use crate::math::quat::Quat64;
use crate::math::vec3f::Vec3F32;
use crate::math::vec3f::Vec3F64;
use crate::math::vec4f::Vec4F32;
use crate::math::vec4f::Vec4F64;
use crate::{impl_bin_op, impl_from};
use glam::DMat4 as M64;
use glam::Mat4 as M32;
use std::ops::MulAssign;
use std::ops::SubAssign;
use std::ops::{Add, Mul};
use std::ops::{AddAssign, Sub};
use super_seq_macro::seq;

seq!(N in (5..=6).collect().map(|i| 1 << i) {
    #[derive(Debug, Copy, Clone)]
    pub struct Mat4F~N(M~N);

    impl Mat4F~N {
        pub const IDENTITY: Self = Self(M~N::IDENTITY);

        #[inline(always)]
        #[must_use]
        pub const fn from_cols_array(array: &[f~N; 16]) -> Self {
            Self(M~N::from_cols_array(array))
        }

        #[inline(always)]
        #[must_use]
        pub fn from_cols(x: impl Into<Vec4F~N>, y: impl Into<Vec4F~N>, z: impl Into<Vec4F~N>, w: impl Into<Vec4F~N>) -> Self {
            let x = x.into();
            let y = y.into();
            let z = z.into();
            let w = w.into();
            Self(M~N::from_cols(x.0, y.0, z.0, w.0))
        }

        #[inline(always)]
        #[must_use]
        pub fn from_quat(rotation: Quat~N) -> Self {
            Self(M~N::from_quat(rotation.0))
        }

        #[inline(always)]
        #[must_use]
        pub fn from_translation(translation: impl Into<Vec3F~N>) -> Self {
            let translation = translation.into();
            Self(M~N::from_translation(translation.0))
        }

        #[inline(always)]
        #[must_use]
        pub fn look_to(eye: impl Into<Vec3F~N>, target: impl Into<Vec3F~N>, up: impl Into<Vec3F~N>) -> Self {
            let eye = eye.into();
            let target = target.into();
            let up = up.into();
            Self(M~N::look_to_rh(eye.0, target.0, up.0))
        }

        #[inline(always)]
        #[must_use]
        pub fn perspective(fov_y: impl Angle, aspect: f~N, near: f~N, far: f~N) -> Self {
            #[allow(clippy::unnecessary_cast)]
            Self(M~N::perspective_rh(*fov_y.to_radians() as f~N, aspect, near, far))
        }

        #[inline(always)]
        #[must_use]
        pub fn transform(&self, vec: Vec3F~N) -> Vec3F~N {
            Vec3F~N(self.0.transform_point3(vec.0))
        }
    }

    //Add
    impl_bin_op!(Mat4F~N + Mat4F~N: Add add, (self, rhs) => Self(self.0 + rhs.0));
    //Sub
    impl_bin_op!(Mat4F~N - Mat4F~N: Sub sub, (self, rhs) => Self(self.0 + rhs.0));
    //Mul
    impl_bin_op!(Mat4F~N * Mat4F~N: Mul mul, (self, rhs) => Self(self.0 * rhs.0));
    //From
    impl_from!(Mat4F~N as [[f~N; 4]; 4]: v => v.0.to_cols_array_2d());
});