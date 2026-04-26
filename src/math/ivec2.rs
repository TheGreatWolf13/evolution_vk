use crate::{impl_bin_op, impl_from, impl_un_op};
use glam::IVec2 as V;
use std::fmt::{Debug, Formatter};
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::{Add, Sub};
use std::ops::{Mul, Neg, SubAssign};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IVec2(pub(super) V);

type P = i32;
type S = IVec2;

impl Debug for S {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({:?}, {:?})", self.x(), self.y()))
    }
}

impl S {
    pub const ZERO: Self = Self(V::ZERO);
    pub const ONE: Self = Self(V::ONE);
    pub const X: Self = Self(V::X);
    pub const Y: Self = Self(V::Y);

    #[inline(always)]
    #[must_use]
    pub const fn new(x: P, y: P) -> Self {
        Self(V::new(x, y))
    }

    #[inline(always)]
    #[must_use]
    pub const fn x(&self) -> P {
        self.0.x
    }

    #[inline(always)]
    #[must_use]
    pub const fn y(&self) -> P {
        self.0.y
    }

    #[inline(always)]
    #[must_use]
    pub const fn x_mut(&mut self) -> &mut P {
        &mut self.0.x
    }

    #[inline(always)]
    #[must_use]
    pub const fn y_mut(&mut self) -> &mut P {
        &mut self.0.y
    }
}

//Add
impl_bin_op!(S + S: Add add, (self, rhs) => Self(self.0 + rhs.0));
impl_bin_op!(S + (P, P): Add add, (self, rhs) => Self(self.0 + V::from(rhs)));
impl_bin_op!(S + [P; 2]: Add add, (self, rhs) => Self(self.0 + V::from(rhs)));
//Sub
impl_bin_op!(S - S: Sub sub, (self, rhs) => Self(self.0 - rhs.0));
impl_bin_op!(S - (P, P): Sub sub, (self, rhs) => Self(self.0 - V::from(rhs)));
impl_bin_op!(S - [P; 2]: Sub sub, (self, rhs) => Self(self.0 - V::from(rhs)));
//Mul
impl_bin_op!(S * P: Mul mul, (self, rhs) => Self(self.0 * rhs));
//Neg
impl_un_op!(- S: Neg neg, self => Self(-self.0));
//From
impl_from!((P, P) as S: v => Self(V::from(v)));
impl_from!([P; 2] as S: v => Self(V::from(v)));
impl_from!(S as (P, P): v => (v.x(), v.y()));
impl_from!(S as [P; 2]: v => [v.x(), v.y()]);