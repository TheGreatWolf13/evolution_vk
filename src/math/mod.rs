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