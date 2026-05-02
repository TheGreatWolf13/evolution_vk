use crate::math::direction::Axis;
use bitvec::macros::internal::funty::Floating;
use std::ops::{Add, Mul};

pub mod angle;
pub mod block_pos;
pub mod chunk_pos;
pub mod color;
pub mod direction;
pub mod ivec2;
pub mod ivec3;
pub mod local_chunk_pos;
pub mod mat3;
pub mod mat4;
pub mod quat;
pub mod u8vec3;
pub mod uvec2;
pub mod vec2;
pub mod vec3;
pub mod vec4;

#[macro_export]
macro_rules! impl_assign {
    ($t:tt, $rhs:ty, $trait_name:ident, $trait_method:ident, $sign:tt) => {
        impl $trait_name<$rhs> for $t {

            #[inline]
            fn $trait_method(&mut self, rhs: $rhs) {
                *self = *self $sign rhs;
            }
        }
    };
}

#[macro_export]
macro_rules! impl_bin_op {
    ($t:tt $sign:tt $rhs:ty : $trait_name:ident $trait_method:ident, ($self:ident, $r:ident) => $e:expr) => {
        impl $trait_name<$rhs> for $t {

            type Output = $t;

            #[inline]
            fn $trait_method($self, $r: $rhs) -> Self::Output {
                $e
            }
        }

        paste::paste! {
            crate::impl_assign!($t, $rhs, [<$trait_name Assign>], [<$trait_method _assign>], $sign);
        }
    };
}

#[macro_export]
macro_rules! impl_bin_op_transform {
    ($t:tt $sign:tt $rhs:ty : $trait_name:ident $trait_method:ident, ($self:ident, $r:ident) => $e:expr) => {
        impl $trait_name<$rhs> for $t {

            type Output = $rhs;

            #[inline]
            fn $trait_method($self, $r: $rhs) -> Self::Output {
                $e
            }
        }
    };
}

#[macro_export]
macro_rules! impl_un_op {
    ($sign:tt $rhs:ty: $trait_name:ident $trait_method:ident, $self:ident => $e:expr) => {
        impl $trait_name for $rhs {
            type Output = $rhs;

            #[inline]
            fn $trait_method($self) -> $rhs {
                $e
            }
        }
    };
}

///Stands for "Past and Present", as it holds a past value and a present value.
pub struct PaP<T>(pub T, pub T);

impl<T: Copy> PaP<T> {
    #[inline]
    pub fn new(t: T) -> Self {
        PaP(t, t)
    }
}

impl<T: Lerp + Copy> PaP<T> {
    #[inline]
    pub fn lerp(&self, partial_tick: f32) -> T {
        self.1.lerp(self.0, partial_tick)
    }
}

pub trait Lerp {
    fn lerp(&self, other: Self, t: f32) -> Self;
}

impl<M: Mul<f32, Output = Self> + Add<Output = Self> + Copy> Lerp for M {
    #[inline]
    fn lerp(&self, other: Self, t: f32) -> Self {
        let now = *self * t;
        let prev = other * (1.0 - t);
        now + prev
    }
}

pub trait Vector3 {
    type T;

    #[must_use]
    fn x(&self) -> Self::T;

    #[must_use]
    fn y(&self) -> Self::T;

    #[must_use]
    fn z(&self) -> Self::T;

    #[must_use]
    fn x_mut(&mut self) -> &mut Self::T;

    #[must_use]
    fn y_mut(&mut self) -> &mut Self::T;

    #[must_use]
    fn z_mut(&mut self) -> &mut Self::T;

    fn get(&self, axis: Axis) -> Self::T {
        match axis {
            Axis::X => self.x(),
            Axis::Y => self.y(),
            Axis::Z => self.z(),
        }
    }

    fn get_mut(&mut self, axis: Axis) -> &mut Self::T {
        match axis {
            Axis::X => self.x_mut(),
            Axis::Y => self.y_mut(),
            Axis::Z => self.z_mut(),
        }
    }
}

#[macro_export]
macro_rules! impl_vec3 {
    ($ty:ty: $c:ty => $acc:tt $x:ident $y:ident $z:ident) => {
        impl Vector3 for $ty {
            type T = $c;

            #[inline]
            fn x(&self) -> Self::T {
                self.$acc.$x
            }

            #[inline]
            fn y(&self) -> Self::T {
                self.$acc.$y
            }

            #[inline]
            fn z(&self) -> Self::T {
                self.$acc.$z
            }

            #[inline]
            fn x_mut(&mut self) -> &mut Self::T {
                &mut self.$acc.$x
            }

            #[inline]
            fn y_mut(&mut self) -> &mut Self::T {
                &mut self.$acc.$y
            }

            #[inline]
            fn z_mut(&mut self) -> &mut Self::T {
                &mut self.$acc.$z
            }
        }
    };
}

pub trait MinMax {
    fn min(self, other: Self) -> Self;

    fn max(self, other: Self) -> Self;
}

// impl<T: Ord + Sized> MinMax for T {
//     fn min(self, other: Self) -> Self {
//         self.min(other)
//     }
//
//     fn max(self, other: Self) -> Self {
//         self.max(other)
//     }
// }

impl MinMax for f32 {
    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    fn max(self, other: Self) -> Self {
        self.max(other)
    }
}