use crate::math::lerp::Lerp;
use crate::math::quat::Quat32;
use crate::math::vec3f::Vec3F32;
use crate::{impl_bin_op, impl_deref, seq_literal_1};
use std::f32::consts::PI;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Angle: Sized + Copy + PartialOrd + PartialEq {
    const ZERO: Self;
    const FULL: Self;

    fn to_degrees(self) -> AngleDeg;

    fn to_radians(self) -> AngleRad;

    fn to_revolutions(self) -> AngleRev;

    fn sin(self) -> f32;

    fn cos(self) -> f32;

    fn sin_cos(self) -> (f32, f32);

    fn to_rot(self, dir: RotDirection) -> Rot<Self> {
        Rot {
            angle: self,
            dir,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RotDirection {
    CW,
    CCW,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct AngleDeg(f32);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct AngleRad(f32);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct AngleRev(f32);

impl Angle for AngleDeg {
    const ZERO: Self = Self(0.0);
    const FULL: Self = Self(360.0);

    #[inline]
    fn to_degrees(self) -> Self {
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
    const ZERO: AngleRad = AngleRad(0.0);
    const FULL: Self = AngleRad(2.0 * PI);

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
    const ZERO: AngleRev = AngleRev(0.0);
    const FULL: Self = AngleRev(1.0);

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
    #[inline]
    pub const fn new(deg: f32) -> Self {
        Self(deg)
    }
}

impl AngleRad {
    #[inline]
    pub fn new(radians: f32) -> Self {
        Self(radians)
    }
}

impl AngleRev {
    #[inline]
    pub const fn new(revolutions: f32) -> Self {
        Self(revolutions)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rot<T: Angle + Copy> {
    angle: T,
    dir: RotDirection,
}

impl<T: Angle + Copy> Rot<T> {
    pub const fn get_angle(&self) -> T {
        self.angle
    }

    pub const fn get_direction(&self) -> RotDirection {
        self.dir
    }
}

impl<T: Angle + Copy + Sub<Output = T> + Mul<f32, Output = T> + Add<Output = T>> Lerp for Rot<T> {
    fn lerp(self, other: Self, t: f32) -> Self {
        //Self is now
        let delta;
        match self.dir {
            RotDirection::CW => delta = other.angle - self.angle,
            RotDirection::CCW => delta = self.angle - other.angle,
        }
        let angle = other.angle + delta * t;
        Self {
            angle,
            dir: self.dir,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rot3<T: Angle + Copy>(Rot<T>, Rot<T>, Rot<T>);

impl<T: Angle + Copy> Rot3<T> {
    #[inline]
    pub fn zero() -> Self {
        Self(T::ZERO.to_rot(RotDirection::CCW), T::ZERO.to_rot(RotDirection::CCW), T::ZERO.to_rot(RotDirection::CCW))
    }

    #[inline]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x.to_rot(RotDirection::CCW), y.to_rot(RotDirection::CCW), z.to_rot(RotDirection::CCW))
    }

    #[inline]
    pub const fn x(&self) -> Rot<T> {
        self.0
    }

    #[inline]
    pub const fn y(&self) -> Rot<T> {
        self.1
    }

    #[inline]
    pub const fn z(&self) -> Rot<T> {
        self.2
    }

    #[inline]
    pub const fn x_mut(&mut self) -> &mut Rot<T> {
        &mut self.0
    }

    #[inline]
    pub const fn y_mut(&mut self) -> &mut Rot<T> {
        &mut self.1
    }

    #[inline]
    pub const fn z_mut(&mut self) -> &mut Rot<T> {
        &mut self.2
    }

    pub fn to_quat(&self) -> Quat32 {
        let mut quat = Quat32::from_axis_angle(Vec3F32::Z, self.z().get_angle());
        quat *= Quat32::from_axis_angle(Vec3F32::Y, self.y().get_angle());
        quat *= Quat32::from_axis_angle(Vec3F32::X, self.x().get_angle());
        quat
    }
}

impl<T: Angle + Copy + Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T>> Lerp for Rot3<T> {
    fn lerp(self, other: Self, t: f32) -> Self {
        let x = self.0.lerp(other.0, t);
        let y = self.1.lerp(other.1, t);
        let z = self.2.lerp(other.2, t);
        Self(x, y, z)
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
seq_literal_1!(N in ("AngleDeg", "AngleRad", "AngleRev") {
    impl Add<[< N >]> for Rot<[< N >]> {
        type Output = Self;
        
        fn add(self, rhs: [< N >]) -> Self::Output {
            let angle = self.angle + rhs;
            let dir = if rhs >= [< N >]::ZERO {
                RotDirection::CCW
            } //
            else {
                RotDirection::CW
            };
            Self {
                angle,
                dir,
            }
        }
    }
    
    impl AddAssign<[< N >]> for Rot<[< N >]> {
        fn add_assign(&mut self, rhs: [< N >]) {
            self.angle += rhs;
            self.dir = if rhs >= [< N >]::ZERO {
                RotDirection::CCW
            } //
            else {
                RotDirection::CW
            };
        }
    }
});
//Sub
impl_rot!(AngleDeg - (AngleDeg, AngleRad, AngleRev): Sub sub, (self, rhs) => Self(self.0 - rhs.to_degrees().0));
impl_rot!(AngleRad - (AngleDeg, AngleRad, AngleRev): Sub sub, (self, rhs) => Self(self.0 - rhs.to_radians().0));
impl_rot!(AngleRev - (AngleDeg, AngleRad, AngleRev): Sub sub, (self, rhs) => Self(self.0 - rhs.to_revolutions().0));
//Mul
impl_bin_op!(AngleDeg * f32: Mul mul, (self, rhs) => Self(self.0 * rhs));
impl_bin_op!(AngleRad * f32: Mul mul, (self, rhs) => Self(self.0 * rhs));
impl_bin_op!(AngleRev * f32: Mul mul, (self, rhs) => Self(self.0 * rhs));
//Div
impl_bin_op!(AngleDeg / f32: Div div, (self, rhs) => Self(self.0 / rhs));
impl_bin_op!(AngleRad / f32: Div div, (self, rhs) => Self(self.0 / rhs));
impl_bin_op!(AngleRev / f32: Div div, (self, rhs) => Self(self.0 / rhs));