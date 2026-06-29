use crate::math::Vector3;
use crate::{impl_bin_op, impl_from, impl_un_op};
use glam::I16Vec3 as V16;
use glam::I64Vec3 as V64;
use glam::I8Vec3 as V8;
use glam::IVec3 as V32;
use std::fmt::{Debug, Formatter};
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::{Add, Sub};
use std::ops::{Mul, Neg, SubAssign};
use super_seq_macro::seq;

seq!(N in (3..=6).collect().map(|i| 1 << i) {
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Vec3I~N(pub(super) V~N);
    
    impl Debug for Vec3I~N {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&format!("{}, {}, {}", self.x(), self.y(), self.z()))
        }
    }
    
    impl Vec3I~N {
        pub const ZERO: Self = Self(V~N::ZERO);
        pub const X: Self = Self(V~N::X);
        pub const Y: Self = Self(V~N::Y);
        pub const Z: Self = Self(V~N::Z);
        
        #[inline(always)]
        #[must_use]
        pub const fn new(x: i~N, y: i~N, z: i~N) -> Self {
            Self(V~N::new(x, y, z))
        }

        #[inline(always)]
        #[must_use]
        pub const fn splat(v: i~N) -> Self {
            Self(V~N::splat(v))
        }
        
        #[inline(always)]
        #[must_use]
        pub fn cross(self, rhs: Self) -> Self {
            Self(self.0.cross(rhs.0))
        }
        
        #[inline(always)]
        #[must_use]
        pub fn len_sqr(self) -> i~N {
            self.0.length_squared()
        }
    }
    
    impl Vector3 for Vec3I~N {
        type T = i~N;
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
    impl_bin_op!(Vec3I~N + Vec3I~N: Add add, (self, rhs) => Self(self.0 + rhs.0));
    impl_bin_op!(Vec3I~N + (i~N, i~N, i~N): Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    impl_bin_op!(Vec3I~N + [i~N; 3]: Add add, (self, rhs) => Self(self.0 + V~N::from(rhs)));
    //Sub
    impl_bin_op!(Vec3I~N - Vec3I~N: Sub sub, (self, rhs) => Self(self.0 - rhs.0));
    impl_bin_op!(Vec3I~N - (i~N, i~N, i~N): Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    impl_bin_op!(Vec3I~N - [i~N; 3]: Sub sub, (self, rhs) => Self(self.0 - V~N::from(rhs)));
    //Mul
    impl_bin_op!(Vec3I~N * i~N: Mul mul, (self, rhs) => Self(self.0 * rhs));
    //Neg
    impl_un_op!(- Vec3I~N: Neg neg, self => Self(-self.0));
    //From
    impl_from!((i~N, i~N, i~N) as Vec3I~N: v => Self(V~N::from(v)));
    impl_from!([i~N; 3] as Vec3I~N: v => Self(V~N::from(v)));
    impl_from!(Vec3I~N as (i~N, i~N, i~N): v => (v.x(), v.y(), v.z()));
    impl_from!(Vec3I~N as [i~N; 3]: v => [v.x(), v.y(), v.z()]);
    seq!(M in (3..=6).collect().map(|i| 1 << i).filter(|i| i != N) {
        impl_from!(Vec3I~M as Vec3I~N: v => v.map(|x| x as i~N));
    });
});