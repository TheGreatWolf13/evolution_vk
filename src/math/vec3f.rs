use crate::math::Vector3;
use crate::{impl_bin_op, impl_from, impl_un_op};
use core::fmt::{Debug, Formatter};
use glam::DVec3 as V64;
use glam::Vec3 as V32;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use super_seq_macro::seq;

seq!(N in (5..=6).collect().map(|i| 1 << i) {
    #[derive(Copy, Clone, PartialEq)]
    pub struct Vec3F~N(pub(super) V~N);

    impl Debug for Vec3F~N {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
           f.write_str(&format!("{}, {}, {}", self.x(), self.y(), self.z()))
        }
    }

    impl Vec3F~N {
        pub const ZERO: Self = Self(V~N::ZERO);
        pub const X: Self = Self(V~N::X);
        pub const Y: Self = Self(V~N::Y);
        pub const Z: Self = Self(V~N::Z);

        #[inline(always)]
        #[must_use]
        pub const fn new(x: f~N, y: f~N, z: f~N) -> Self {
            Self(V~N::new(x, y, z))
        }

        #[inline(always)]
        #[must_use]
        pub const fn splat(v: f~N) -> Self {
            Self(V~N::splat(v))
        }

        #[inline(always)]
        #[must_use]
        pub fn cross(self, rhs: Self) -> Self {
            Self(self.0.cross(rhs.0))
        }

        #[inline(always)]
        #[must_use]
        pub fn len_sqr(self) -> f~N {
            self.0.length_squared()
        }

        #[inline(always)]
        #[must_use]
        pub fn len(self) -> f~N {
            self.0.length()
        }

        #[inline(always)]
        #[must_use]
        pub fn normalize(self) -> Self {
            Self(self.0.normalize())
        }

        #[inline(always)]
        #[must_use]
        pub fn normalize_and_len(self) -> (Self, f~N) {
            let (vec, len) = self.0.normalize_and_length();
            (Self(vec), len)
        }
    }

    impl Vector3 for Vec3F~N {
        type T = f~N;
        const X: Self = Self::X;
        const Y: Self = Self::Y;
        const Z: Self = Self::Z;

        #[inline(always)]
        fn x(&self) -> Self::T {
            self.0.x
        }

        #[inline(always)]
        fn y(&self) -> Self::T {
            self.0.y
        }

        #[inline(always)]
        fn z(&self) -> Self::T {
            self.0.z
        }

        #[inline(always)]
        fn x_mut(&mut self) -> &mut Self::T {
            &mut self.0.x
        }

        #[inline(always)]
        fn y_mut(&mut self) -> &mut Self::T {
            &mut self.0.y
        }

        #[inline(always)]
        fn z_mut(&mut self) -> &mut Self::T {
            &mut self.0.z
        }
    }

    //Add
    impl_bin_op!(Vec3F~N + Vec3F~N: Add add, (self, rhs) => Self(self.0 + rhs.0));
    impl_bin_op!(Vec3F~N + (f~N, f~N, f~N): Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    impl_bin_op!(Vec3F~N + [f~N; 3]: Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    //Sub
    impl_bin_op!(Vec3F~N - Vec3F~N: Sub sub, (self, rhs) => Self(self.0 - rhs.0));
    impl_bin_op!(Vec3F~N - (f~N, f~N, f~N): Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    impl_bin_op!(Vec3F~N - [f~N; 3]: Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    //Mul
    impl_bin_op!(Vec3F~N * f~N: Mul mul, (self, rhs) => Self(self.0 * rhs));
    //Neg
    impl_un_op!(-Vec3F~N: Neg neg, self => Self(-self.0));
    //From
    impl_from!((f~N, f~N, f~N) as Vec3F~N: v => Self(V~N::from(v)));
    impl_from!([f~N; 3] as Vec3F~N: v => Self(V~N::from(v)));
    impl_from!(Vec3F~N as (f~N, f~N, f~N): v => (v.x(), v.y(), v.z()));
    impl_from!(Vec3F~N as [f~N; 3]: v => [v.x(), v.y(), v.z()]);
    seq!(M in (5..=6).collect().map(|i| 1 << i).filter(|i| i != N) {
        impl_from!(Vec3F~M as Vec3F~N: v => v.map(|x| x as f~N));
    });
});
