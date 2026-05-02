use crate::math::Vector3;
use crate::{impl_bin_op, impl_from, impl_un_op, impl_vec3};
use glam::IVec3 as V;
use std::ops::SubAssign;
use std::ops::{Add, Mul};
use std::ops::{AddAssign, Sub};
use std::ops::{MulAssign, Neg};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IVec3(pub(super) V);

type P = i32;
type S = IVec3;

impl S {
    pub const ZERO: Self = Self(V::ZERO);
    pub const ONE: Self = Self(V::ONE);
    pub const X: Self = Self(V::X);
    pub const Y: Self = Self(V::Y);
    pub const Z: Self = Self(V::Z);

    #[inline(always)]
    #[must_use]
    pub const fn new(x: P, y: P, z: P) -> Self {
        Self(V::new(x, y, z))
    }
}

//Vec
impl_vec3!(S: P => 0 x y z);
//Add
impl_bin_op!(S + S: Add add, (self, rhs) => Self(self.0 + rhs.0));
impl_bin_op!(S + (P, P, P): Add add, (self, rhs) => Self(self.0 + V::from(rhs)));
impl_bin_op!(S + [P; 3]: Add add, (self, rhs) => Self(self.0 + V::from(rhs)));
//Sub
impl_bin_op!(S - S: Sub sub, (self, rhs) => Self(self.0 - rhs.0));
impl_bin_op!(S - (P, P, P): Sub sub, (self, rhs) => Self(self.0 - V::from(rhs)));
impl_bin_op!(S - [P; 3]: Sub sub, (self, rhs) => Self(self.0 - V::from(rhs)));
//Mul
impl_bin_op!(S * P: Mul mul, (self, rhs) => Self(self.0 * rhs));
//Neg
impl_un_op!(- S: Neg neg, self => Self(-self.0));
//From
impl_from!((P, P, P) as S: v => Self(V::from(v)));
impl_from!([P; 3] as S: v => Self(V::from(v)));
impl_from!(S as (P, P, P): v => (v.x(), v.y(), v.z()));
impl_from!(S as [P; 3]: v => [v.x(), v.y(), v.z()]);