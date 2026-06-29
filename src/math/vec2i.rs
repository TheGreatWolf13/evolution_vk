use crate::math::Vector2;
use crate::{impl_bin_op, impl_from, impl_un_op};
use glam::I16Vec2 as V16;
use glam::I64Vec2 as V64;
use glam::I8Vec2 as V8;
use glam::IVec2 as V32;
use std::fmt::{Debug, Formatter};
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::{Add, Sub};
use std::ops::{Mul, Neg, SubAssign};
use super_seq_macro::seq;
use winit::dpi::PhysicalSize;

seq!(N in (3..=6).collect().map(|i| 1 << i) {
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Vec2I~N(pub(super) V~N);

    impl Debug for Vec2I~N {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&format!("{}, {}", self.x(), self.y()))
        }
    }

    impl Vec2I~N {
        pub const ZERO: Self = Self(V~N::ZERO);
        pub const X: Self = Self(V~N::X);
        pub const Y: Self = Self(V~N::Y);

        #[inline(always)]
        #[must_use]
        pub const fn new(x: i~N, y: i~N) -> Self {
            Self(V~N::new(x, y))
        }

        #[inline(always)]
        #[must_use]
        pub const fn splat(v: i~N) -> Self {
            Self(V~N::splat(v))
        }

        #[inline(always)]
        #[must_use]
        pub fn len_sqr(self) -> i~N {
            self.0.length_squared()
        }
    }

    impl Vector2 for Vec2I~N {
        type T = i~N;
        const X: Self = Self::X;
        const Y: Self = Self::Y;

        #[inline(always)]
        fn x(&self) -> Self::T {
           self.0.x
        }

        #[inline(always)]
        fn y(&self) -> Self::T {
            self.0.y
        }

        #[inline(always)]
        fn x_mut(&mut self) -> &mut Self::T {
            &mut self.0.x
        }

        #[inline(always)]
        fn y_mut(&mut self) -> &mut Self::T {
            &mut self.0.y
        }
    }

    //Add
    impl_bin_op!(Vec2I~N + Vec2I~N: Add add, (self, rhs) => Self(self.0 + rhs.0));
    impl_bin_op!(Vec2I~N + (i~N, i~N): Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    impl_bin_op!(Vec2I~N + [i~N; 2]: Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    //Sub
    impl_bin_op!(Vec2I~N - Vec2I~N: Sub sub, (self, rhs) => Self(self.0 - rhs.0));
    impl_bin_op!(Vec2I~N - (i~N, i~N): Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    impl_bin_op!(Vec2I~N - [i~N; 2]: Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    //Mul
    impl_bin_op!(Vec2I~N * i~N: Mul mul, (self, rhs) => Self(self.0 * rhs));
    //Neg
    impl_un_op!(- Vec2I~N: Neg neg, self => Self(-self.0));
    //From
    impl_from!((i~N, i~N) as Vec2I~N: v => Self(V~N::from(v)));
    impl_from!([i~N; 2] as Vec2I~N: v => Self(V~N::from(v)));
    impl_from!(PhysicalSize<i~N> as Vec2I~N: v => Self::new(v.width, v.height));
    impl_from!(Vec2I~N as (i~N, i~N): v => (v.x(), v.y()));
    impl_from!(Vec2I~N as [i~N; 2]: v => [v.x(), v.y()]);
    impl_from!(Vec2I~N as PhysicalSize<i~N>: v => PhysicalSize::new(v.x(), v.y()));
    seq!(M in (3..=6).collect().map(|i| 1 << i).filter(|i| i != N) {
        impl_from!(Vec2I~M as Vec2I~N: v => v.map(|x| x as i~N));
    });
});