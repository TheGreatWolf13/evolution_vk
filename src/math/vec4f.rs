use crate::math::Vector4;
use crate::{impl_bin_op, impl_from, impl_un_op};
use core::fmt::{Debug, Formatter};
use glam::DVec4 as V64;
use glam::Vec4 as V32;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use super_seq_macro::seq;

seq!(N in (5..=6).collect().map(|i| 1 << i) {
    #[derive(Copy, Clone, PartialEq)]
    pub struct Vec4F~N(pub(super) V~N);

    impl Debug for Vec4F~N {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
           f.write_str(&format!("{}, {}, {}, {}", self.x(), self.y(), self.z(), self.w()))
        }
    }

    impl Vec4F~N {
        pub const ZERO: Self = Self(V~N::ZERO);
        pub const X: Self = Self(V~N::X);
        pub const Y: Self = Self(V~N::Y);
        pub const Z: Self = Self(V~N::Z);
        pub const W: Self = Self(V~N::W);
        
        #[inline(always)]
        #[must_use]
        pub const fn new(x: f~N, y: f~N, z: f~N, w: f~N) -> Self {
            Self(V~N::new(x, y, z, w))
        }

        #[inline(always)]
        #[must_use]
        pub const fn splat(v: f~N) -> Self {
            Self(V~N::splat(v))
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

    impl Vector4 for Vec4F~N {
        type T = f~N;
        const X: Self = Self::X;
        const Y: Self = Self::Y;
        const Z: Self = Self::Z;
        const W: Self = Self::W;

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
        fn w(&self) -> Self::T {
            self.0.w
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
        
        #[inline(always)]
        fn w_mut(&mut self) -> &mut Self::T {
            &mut self.0.w
        }
    }

    //Add
    impl_bin_op!(Vec4F~N + Vec4F~N: Add add, (self, rhs) => Self(self.0 + rhs.0));
    impl_bin_op!(Vec4F~N + (f~N, f~N, f~N, f~N): Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    impl_bin_op!(Vec4F~N + [f~N; 4]: Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    //Sub
    impl_bin_op!(Vec4F~N - Vec4F~N: Sub sub, (self, rhs) => Self(self.0 - rhs.0));
    impl_bin_op!(Vec4F~N - (f~N, f~N, f~N, f~N): Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    impl_bin_op!(Vec4F~N - [f~N; 4]: Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    //Mul
    impl_bin_op!(Vec4F~N * f~N: Mul mul, (self, rhs) => Self(self.0 * rhs));
    //Neg
    impl_un_op!(-Vec4F~N: Neg neg, self => Self(-self.0));
    //From
    impl_from!((f~N, f~N, f~N, f~N) as Vec4F~N: v => Self(V~N::from(v)));
    impl_from!([f~N; 4] as Vec4F~N: v => Self(V~N::from(v)));
    impl_from!(Vec4F~N as (f~N, f~N, f~N, f~N): v => (v.x(), v.y(), v.z(), v.w()));
    impl_from!(Vec4F~N as [f~N; 4]: v => [v.x(), v.y(), v.z(), v.w()]);
    seq!(M in (5..=6).collect().map(|i| 1 << i).filter(|i| i != N) {
        impl_from!(Vec4F~M as Vec4F~N: v => v.map(|x| x as f~N));
    });
});
