use crate::math::Vector2;
use crate::{impl_bin_op, impl_from, impl_un_op};
use glam::DVec2 as V64;
use glam::Vec2 as V32;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use super_seq_macro::seq;
use winit::dpi::PhysicalSize;

seq!(N in (5..=6).collect().map(|i| 1 << i) {
    #[derive(Copy, Clone)]
    pub struct Vec2F~N(pub(super) V~N);

    impl Debug for Vec2F~N {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.write_str(&format!("{}, {}", self.x(), self.y()))
        }
    }

    impl Vec2F~N {
        pub const ZERO: Self = Self(V~N::ZERO);
        pub const X: Self = Self(V~N::X);
        pub const Y: Self = Self(V~N::Y);

        #[inline(always)]
        #[must_use]
        pub const fn new(x: f~N, y: f~N) -> Self {
            Self(V~N::new(x, y))
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

    impl Vector2 for Vec2F~N {
        type T = f~N;
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
    impl_bin_op!(Vec2F~N + Vec2F~N: Add add, (self, rhs) => Self(self.0 + rhs.0));
    impl_bin_op!(Vec2F~N + (f~N, f~N): Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    impl_bin_op!(Vec2F~N + [f~N; 2]: Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    //Sub
    impl_bin_op!(Vec2F~N - Vec2F~N: Sub sub, (self, rhs) => Self(self.0 - rhs.0));
    impl_bin_op!(Vec2F~N - (f~N, f~N): Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    impl_bin_op!(Vec2F~N - [f~N; 2]: Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    //Mul
    impl_bin_op!(Vec2F~N * f~N: Mul mul, (self, rhs) => Self(self.0 * rhs));
    //Neg
    impl_un_op!(- Vec2F~N: Neg neg, self => Self(-self.0));
    //From
    impl_from!((f~N, f~N) as Vec2F~N: v => Self(V~N::from(v)));
    impl_from!([f~N; 2] as Vec2F~N: v => Self(V~N::from(v)));
    impl_from!(PhysicalSize<f~N> as Vec2F~N: v => Self(V~N::new(v.width, v.height)));
    impl_from!(Vec2F~N as (f~N, f~N): v => (v.x(), v.y()));
    impl_from!(Vec2F~N as [f~N; 2]: v => [v.x(), v.y()]);
    impl_from!(Vec2F~N as PhysicalSize<f~N>: v => PhysicalSize::new(v.x(), v.y()));
    seq!(M in (5..=6).collect().map(|i| 1 << i).filter(|i| i != N) {
        impl_from!(Vec2F~M as Vec2F~N: v => v.map(|x| x as f~N));
    });
});
