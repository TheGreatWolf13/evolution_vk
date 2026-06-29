use crate::math::Vector4;
use crate::{impl_bin_op, impl_from};
use core::fmt::{Debug, Formatter};
use glam::U16Vec4 as V16;
use glam::U64Vec4 as V64;
use glam::U8Vec4 as V8;
use glam::UVec4 as V32;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;
use std::ops::{Add, Mul, Sub};
use super_seq_macro::seq;

seq!(N in (3..=6).collect().map(|i| 1 << i) {
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vec4U~N(pub(crate) V~N);

    impl Debug for Vec4U~N {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&format!("{}, {}, {}, {}", self.x(), self.y(), self.z(), self.w()))
        }
    }

    impl Vec4U~N {
        pub const ZERO: Self = Self(V~N::ZERO);
        pub const X: Self = Self(V~N::X);
        pub const Y: Self = Self(V~N::Y);
        pub const Z: Self = Self(V~N::Z);
        pub const W: Self = Self(V~N::W);

        #[inline(always)]
        #[must_use]
        pub const fn new(x: u~N, y: u~N, z: u~N, w: u~N) -> Self {
            Self(V~N::new(x, y, z, w))
        }

        #[inline(always)]
        #[must_use]
        pub const fn splat(v: u~N) -> Self {
            Self(V~N::splat(v))
        }
        
        #[inline(always)]
        #[must_use]
        pub fn len_sqr(self) -> u~N {
            self.0.length_squared()
        }
    }

    impl Vector4 for Vec4U~N {
        type T = u~N;
        const X: Self = Self::X;
        const Y: Self = Self::Y;
        const Z: Self = Self::Z;
        const W: Self = Self::W;

        fn x(&self) -> Self::T {
            self.0.x
        }

        fn y(&self) -> Self::T {
            self.0.y
        }

        fn z(&self) -> Self::T {
            self.0.z
        }

        fn w(&self) -> Self::T {
            self.0.w
        }

        fn x_mut(&mut self) -> &mut Self::T {
            &mut self.0.x
        }

        fn y_mut(&mut self) -> &mut Self::T {
            &mut self.0.y
        }

        fn z_mut(&mut self) -> &mut Self::T {
            &mut self.0.z
        }

        fn w_mut(&mut self) -> &mut Self::T {
            &mut self.0.w
        }
    }

    //Add
    impl_bin_op!(Vec4U~N + Vec4U~N: Add add, (self, rhs) => Self(self.0 + rhs.0));
    impl_bin_op!(Vec4U~N + (u~N, u~N, u~N, u~N): Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    impl_bin_op!(Vec4U~N + [u~N; 4]: Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    //Sub
    impl_bin_op!(Vec4U~N - Vec4U~N: Sub sub, (self, rhs) => Self(self.0 - rhs.0));
    impl_bin_op!(Vec4U~N - (u~N, u~N, u~N, u~N): Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    impl_bin_op!(Vec4U~N - [u~N; 4]: Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    //Mul
    impl_bin_op!(Vec4U~N * u~N: Mul mul, (self, rhs) => Self(self.0 * rhs));
    //From
    impl_from!((u~N, u~N, u~N, u~N) as Vec4U~N: v => Self(V~N::from(v)));
    impl_from!([u~N; 4] as Vec4U~N: v => Self(V~N::from(v)));
    impl_from!(Vec4U~N as (u~N, u~N, u~N, u~N): v => (v.x(), v.y(), v.z(), v.w()));
    impl_from!(Vec4U~N as [u~N; 4]: v => [v.x(), v.y(), v.z(), v.w()]);
    seq!(M in (3..=6).collect().map(|i| 1 << i).filter(|i| i != N) {
        impl_from!(Vec4U~M as Vec4U~N: v => v.map(|x| x as u~N));
    });
});