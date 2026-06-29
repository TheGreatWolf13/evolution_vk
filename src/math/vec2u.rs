use crate::math::Vector2;
use crate::{impl_bin_op, impl_from};
use glam::U16Vec2 as V16;
use glam::U64Vec2 as V64;
use glam::U8Vec2 as V8;
use glam::UVec2 as V32;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Mul, Sub};
use std::ops::{AddAssign, MulAssign, SubAssign};
use super_seq_macro::seq;
use winit::dpi::PhysicalSize;

seq!(N in (3..=6).collect().map(|i| 1 << i) {
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vec2U~N(pub(crate) V~N);

    impl Debug for Vec2U~N {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&format!("{}, {}", self.x(), self.y()))
        }
    }

    impl Vec2U~N {
        pub const ZERO: Self = Self(V~N::ZERO);
        pub const X: Self = Self(V~N::X);
        pub const Y: Self = Self(V~N::Y);

        #[inline(always)]
        #[must_use]
        pub const fn new(x: u~N, y: u~N) -> Self {
            Self(V~N::new(x, y))
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

    impl Vector2 for Vec2U~N {
        type T = u~N;
        const X: Self = Self::X;
        const Y: Self = Self::Y;

        fn x(&self) -> Self::T {
            self.0.x
        }

        fn y(&self) -> Self::T {
            self.0.y
        }

        fn x_mut(&mut self) -> &mut Self::T {
            &mut self.0.x
        }

        fn y_mut(&mut self) -> &mut Self::T {
            &mut self.0.y
        }
    }

    //Add
    impl_bin_op!(Vec2U~N + Vec2U~N: Add add, (self, rhs) => Self(self.0 + rhs.0));
    impl_bin_op!(Vec2U~N + (u~N, u~N): Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    impl_bin_op!(Vec2U~N + [u~N; 2]: Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    //Sub
    impl_bin_op!(Vec2U~N - Vec2U~N: Sub sub, (self, rhs) => Self(self.0 - rhs.0));
    impl_bin_op!(Vec2U~N - (u~N, u~N): Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    impl_bin_op!(Vec2U~N - [u~N; 2]: Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    //Mul
    impl_bin_op!(Vec2U~N * u~N: Mul mul, (self, rhs) => Self(self.0 * rhs));
    //From
    impl_from!((u~N, u~N) as Vec2U~N: v => Self(V~N::from(v)));
    impl_from!([u~N; 2] as Vec2U~N: v => Self(V~N::from(v)));
    impl_from!(PhysicalSize<u~N> as Vec2U~N: v => Self::new(v.width, v.height));
    impl_from!(Vec2U~N as (u~N, u~N): v => (v.x(), v.y()));
    impl_from!(Vec2U~N as [u~N; 2]: v => [v.x(), v.y()]);
    impl_from!(Vec2U~N as PhysicalSize<u~N>: v => PhysicalSize::new(v.x(), v.y()));
    seq!(M in (3..=6).collect().map(|i| 1 << i).filter(|i| i != N) {
        impl_from!(Vec2U~M as Vec2U~N: v => v.map(|x| x as u~N));
    });
});