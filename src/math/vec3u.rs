use crate::math::Vector3;
use crate::{impl_bin_op, impl_from};
use core::fmt::{Debug, Formatter};
use glam::U16Vec3 as V16;
use glam::U64Vec3 as V64;
use glam::U8Vec3 as V8;
use glam::UVec3 as V32;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;
use std::ops::{Add, Mul, Sub};
use super_seq_macro::seq;

seq!(N in (3..=6).collect().map(|i| 1 << i) {
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vec3U~N(pub(crate) V~N);

    impl Debug for Vec3U~N {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&format!("{}, {}, {}", self.x(), self.y(), self.z()))
        }
    }

    impl Vec3U~N {
        pub const ZERO: Self = Self(V~N::ZERO);
        pub const X: Self = Self(V~N::X);
        pub const Y: Self = Self(V~N::Y);
        pub const Z: Self = Self(V~N::Z);

        #[inline(always)]
        #[must_use]
        pub const fn new(x: u~N, y: u~N, z: u~N) -> Self {
            Self(V~N::new(x, y, z))
        }
        
        #[inline(always)]
        #[must_use]
        pub const fn splat(v: u~N) -> Self {
            Self(V~N::splat(v))
        }
        
        #[inline(always)]
        #[must_use]
        pub fn cross(self, rhs: Self) -> Self {
            Self(self.0.cross(rhs.0))
        }
        
        #[inline(always)]
        #[must_use]
        pub fn len_sqr(self) -> u~N {
            self.0.length_squared()
        }
    }

    impl Vector3 for Vec3U~N {
        type T = u~N;
        const X: Self = Self::X;
        const Y: Self = Self::Y;
        const Z: Self = Self::Z;

        fn x(&self) -> Self::T {
            self.0.x
        }

        fn y(&self) -> Self::T {
            self.0.y
        }

        fn z(&self) -> Self::T {
            self.0.z
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
    }

    //Add
    impl_bin_op!(Vec3U~N + Vec3U~N: Add add, (self, rhs) => Self(self.0 + rhs.0));
    impl_bin_op!(Vec3U~N + (u~N, u~N, u~N): Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    impl_bin_op!(Vec3U~N + [u~N; 3]: Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    //Sub
    impl_bin_op!(Vec3U~N - Vec3U~N: Sub sub, (self, rhs) => Self(self.0 - rhs.0));
    impl_bin_op!(Vec3U~N - (u~N, u~N, u~N): Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    impl_bin_op!(Vec3U~N - [u~N; 3]: Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    //Mul
    impl_bin_op!(Vec3U~N * u~N: Mul mul, (self, rhs) => Self(self.0 * rhs));
    //From
    impl_from!((u~N, u~N, u~N) as Vec3U~N: v => Self(V~N::from(v)));
    impl_from!([u~N; 3] as Vec3U~N: v => Self(V~N::from(v)));
    impl_from!(Vec3U~N as (u~N, u~N, u~N): v => (v.x(), v.y(), v.z()));
    impl_from!(Vec3U~N as [u~N; 3]: v => [v.x(), v.y(), v.z()]);
    seq!(M in (3..=6).collect().map(|i| 1 << i).filter(|i| i != N) {
        impl_from!(Vec3U~M as Vec3U~N: v => v.map(|x| x as u~N));
    });
});