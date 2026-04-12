use crate::{impl_bin_op, impl_deref};
use std::f32::consts::PI;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Angle {
    fn to_degrees(self) -> AngleDeg;

    fn to_radians(self) -> AngleRad;

    fn to_revolutions(self) -> AngleRev;

    fn sin(self) -> f32;

    fn cos(self) -> f32;

    fn sin_cos(self) -> (f32, f32);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleDeg(f32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleRad(f32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AngleRev(f32);

impl Angle for AngleDeg {
    #[inline]
    fn to_degrees(self) -> AngleDeg {
        self
    }

    #[inline]
    fn to_radians(self) -> AngleRad {
        AngleRad(self.0.to_radians())
    }

    #[inline]
    fn to_revolutions(self) -> AngleRev {
        AngleRev(self.0 / 360.0)
    }

    #[inline]
    fn sin(self) -> f32 {
        self.0.to_radians().sin()
    }

    #[inline]
    fn cos(self) -> f32 {
        self.0.to_radians().cos()
    }

    #[inline]
    fn sin_cos(self) -> (f32, f32) {
        let rad = self.0.to_radians();
        (rad.sin(), rad.cos())
    }
}

impl Angle for AngleRad {
    #[inline]
    fn to_degrees(self) -> AngleDeg {
        AngleDeg(self.0.to_degrees())
    }

    #[inline]
    fn to_radians(self) -> AngleRad {
        self
    }

    #[inline]
    fn to_revolutions(self) -> AngleRev {
        AngleRev(self.0 / (2.0 * PI))
    }

    #[inline]
    fn sin(self) -> f32 {
        self.0.sin()
    }

    #[inline]
    fn cos(self) -> f32 {
        self.0.cos()
    }

    #[inline]
    fn sin_cos(self) -> (f32, f32) {
        (self.0.sin(), self.0.cos())
    }
}

impl Angle for AngleRev {
    #[inline]
    fn to_degrees(self) -> AngleDeg {
        AngleDeg(self.0 * 360.0)
    }

    #[inline]
    fn to_radians(self) -> AngleRad {
        AngleRad(self.0 * (2.0 * PI))
    }

    #[inline]
    fn to_revolutions(self) -> AngleRev {
        self
    }

    #[inline]
    fn sin(self) -> f32 {
        (self.0 * (2.0 * PI)).sin()
    }

    #[inline]
    fn cos(self) -> f32 {
        (self.0 * (2.0 * PI)).cos()
    }

    #[inline]
    fn sin_cos(self) -> (f32, f32) {
        let rad = self.0 * (2.0 * PI);
        (rad.sin(), rad.cos())
    }
}

impl AngleDeg {
    pub const ZERO: AngleDeg = Self::new(0.0);

    #[inline]
    pub const fn new(deg: f32) -> Self {
        Self(deg)
    }
}

impl AngleRad {
    pub const ZERO: AngleRad = AngleRad(0.0);

    #[inline]
    pub fn new(radians: f32) -> Self {
        Self(radians)
    }
}

impl AngleRev {
    pub const ZERO: AngleRev = Self::new(0.0);

    #[inline]
    pub const fn new(revolutions: f32) -> Self {
        Self(revolutions)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rot3Deg(AngleDeg, AngleDeg, AngleDeg);

impl Rot3Deg {
    pub const ZERO: Rot3Deg = Self::new(AngleDeg::ZERO, AngleDeg::ZERO, AngleDeg::ZERO);

    #[inline]
    pub const fn new(x: AngleDeg, y: AngleDeg, z: AngleDeg) -> Self {
        Self(x, y, z)
    }

    #[inline]
    pub const fn x(&self) -> AngleDeg {
        self.0
    }

    #[inline]
    pub const fn y(&self) -> AngleDeg {
        self.1
    }

    #[inline]
    pub const fn z(&self) -> AngleDeg {
        self.2
    }

    #[inline]
    pub const fn x_mut(&mut self) -> &mut AngleDeg {
        &mut self.0
    }

    #[inline]
    pub const fn y_mut(&mut self) -> &mut AngleDeg {
        &mut self.1
    }

    #[inline]
    pub const fn z_mut(&mut self) -> &mut AngleDeg {
        &mut self.2
    }
}

macro_rules! impl_rot {
    ($main:tt $sign:tt ($($other: ty),+): $trait_name:tt $trait_method:tt, ($self:ident, $rhs:ident) => $e:expr) => {
        $(
            impl_bin_op!($main $sign $other: $trait_name $trait_method, ($self, $rhs) => $e);
        )+
    };
}

//Deref
impl_deref!(AngleDeg as f32: self => &self.0);
impl_deref!(AngleRad as f32: self => &self.0);
impl_deref!(AngleRev as f32: self => &self.0);
//Add
impl_rot!(AngleDeg + (AngleDeg, AngleRad, AngleRev): Add add, (self, rhs) => Self(self.0 + rhs.to_degrees().0));
impl_rot!(AngleRad + (AngleDeg, AngleRad, AngleRev): Add add, (self, rhs) => Self(self.0 + rhs.to_radians().0));
impl_rot!(AngleRev + (AngleDeg, AngleRad, AngleRev): Add add, (self, rhs) => Self(self.0 + rhs.to_revolutions().0));
impl_bin_op!(Rot3Deg + Rot3Deg: Add add, (self, rhs) => Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2));
//Sub
impl_rot!(AngleDeg - (AngleDeg, AngleRad, AngleRev): Sub sub, (self, rhs) => Self(self.0 - rhs.to_degrees().0));
impl_rot!(AngleRad - (AngleDeg, AngleRad, AngleRev): Sub sub, (self, rhs) => Self(self.0 - rhs.to_radians().0));
impl_rot!(AngleRev - (AngleDeg, AngleRad, AngleRev): Sub sub, (self, rhs) => Self(self.0 - rhs.to_revolutions().0));
impl_bin_op!(Rot3Deg - Rot3Deg: Sub sub, (self, rhs) => Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2));
//Mul
impl_bin_op!(AngleDeg * f32: Mul mul, (self, rhs) => Self(self.0 * rhs));
impl_bin_op!(AngleRad * f32: Mul mul, (self, rhs) => Self(self.0 * rhs));
impl_bin_op!(AngleRev * f32: Mul mul, (self, rhs) => Self(self.0 * rhs));
impl_bin_op!(Rot3Deg * f32: Mul mul, (self, rhs) => Self(self.0 * rhs, self.1 * rhs, self.2 * rhs));
//Div
impl_bin_op!(AngleDeg / f32: Div div, (self, rhs) => Self(self.0 / rhs));
impl_bin_op!(AngleRad / f32: Div div, (self, rhs) => Self(self.0 / rhs));
impl_bin_op!(AngleRev / f32: Div div, (self, rhs) => Self(self.0 / rhs));
impl_bin_op!(Rot3Deg / f32: Div div, (self, rhs) => Self(self.0 / rhs, self.1 / rhs, self.2 / rhs));