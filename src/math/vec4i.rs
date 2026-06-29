use crate::math::Vector4;
use crate::{impl_bin_op, impl_from, impl_un_op};
use glam::I16Vec4 as V16;
use glam::I64Vec4 as V64;
use glam::I8Vec4 as V8;
use glam::IVec4 as V32;
use std::fmt::{Debug, Formatter};
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::{Add, Sub};
use std::ops::{Mul, Neg, SubAssign};
use super_seq_macro::seq;

seq!(N in (3..=6).collect().map(|i| 1 << i) {
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vec4I~N(pub(super) V~N);
    
    impl Debug for Vec4I~N {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&format!("{}, {}, {}, {}", self.x(), self.y(), self.z(), self.w()))
        }
    }
    
    impl Vec4I~N {
        pub const ZERO: Self = Self(V~N::ZERO);
        pub const X: Self = Self(V~N::X);
        pub const Y: Self = Self(V~N::Y);
        pub const Z: Self = Self(V~N::Z);
        pub const W: Self = Self(V~N::W);
        
        #[inline(always)]
        #[must_use]
        pub const fn new(x: i~N, y: i~N, z: i~N, w: i~N) -> Self {
            Self(V~N::new(x, y, z, w))
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
    
    impl Vector4 for Vec4I~N {
        type T = i~N;
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
    impl_bin_op!(Vec4I~N + Vec4I~N: Add add, (self, rhs) => Self(self.0 + rhs.0));
    impl_bin_op!(Vec4I~N + (i~N, i~N, i~N, i~N): Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    impl_bin_op!(Vec4I~N + [i~N; 4]: Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    //Sub
    impl_bin_op!(Vec4I~N - Vec4I~N: Sub sub, (self, rhs) => Self(self.0 - rhs.0));
    impl_bin_op!(Vec4I~N - (i~N, i~N, i~N, i~N): Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    impl_bin_op!(Vec4I~N - [i~N; 4]: Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    //Mul
    impl_bin_op!(Vec4I~N * i~N: Mul mul, (self, rhs) => Self(self.0 * rhs));
    //Neg
    impl_un_op!(- Vec4I~N: Neg neg, self => Self(-self.0));
    //From
    impl_from!((i~N, i~N, i~N, i~N) as Vec4I~N: v => Self(V~N::from(v)));
    impl_from!([i~N; 4] as Vec4I~N: v => Self(V~N::from(v)));
    impl_from!(Vec4I~N as (i~N, i~N, i~N, i~N): v => (v.x(), v.y(), v.z(), v.w()));
    impl_from!(Vec4I~N as [i~N; 4]: v => [v.x(), v.y(), v.z(), v.w()]);
    seq!(M in (3..=6).collect().map(|i| 1 << i).filter(|i| i != N) {
        impl_from!(Vec4I~M as Vec4I~N: v => v.map(|x| x as i~N));
    });
});