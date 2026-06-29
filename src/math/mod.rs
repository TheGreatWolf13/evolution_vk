use crate::math::direction::{Axis2, Axis3, Axis4};
use crate::{seq_literal, seq_literal_2};
use std::ops::{Add, Mul, RangeBounds};

pub mod angle;
pub mod bitvec;
pub mod block_pos;
pub mod chunk_pos;
pub mod color;
pub mod direction;
pub mod local_section_pos;
pub mod mat3;
pub mod mat4;
pub mod quat;
pub mod section_pos;
pub mod vec2f;
pub mod vec2i;
pub mod vec2u;
pub mod vec3f;
pub mod vec3i;
pub mod vec3u;
pub mod vec4f;
pub mod vec4i;
pub mod vec4u;

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

pub trait Vector2 {
    type T: MinMax;
    const X: Self;
    const Y: Self;

    #[must_use]
    fn x(&self) -> Self::T;

    #[must_use]
    fn y(&self) -> Self::T;

    #[must_use]
    fn x_mut(&mut self) -> &mut Self::T;

    #[must_use]
    fn y_mut(&mut self) -> &mut Self::T;

    #[inline(always)]
    #[must_use]
    fn get(&self, axis: Axis2) -> Self::T {
        match axis {
            Axis2::X => self.x(),
            Axis2::Y => self.y(),
        }
    }

    #[inline(always)]
    #[must_use]
    fn get_mut(&mut self, axis: Axis2) -> &mut Self::T {
        match axis {
            Axis2::X => self.x_mut(),
            Axis2::Y => self.y_mut(),
        }
    }

    #[inline(always)]
    #[must_use]
    fn map<S, V: From<(S, S)>>(&self, f: impl Fn(Self::T) -> S) -> V {
        (f(self.x()), f(self.y())).into()
    }
}

pub trait Vector3 {
    type T: MinMax;
    const X: Self;
    const Y: Self;
    const Z: Self;

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

    #[inline(always)]
    #[must_use]
    fn get(&self, axis: Axis3) -> Self::T {
        match axis {
            Axis3::X => self.x(),
            Axis3::Y => self.y(),
            Axis3::Z => self.z(),
        }
    }

    #[inline(always)]
    #[must_use]
    fn get_mut(&mut self, axis: Axis3) -> &mut Self::T {
        match axis {
            Axis3::X => self.x_mut(),
            Axis3::Y => self.y_mut(),
            Axis3::Z => self.z_mut(),
        }
    }

    #[inline(always)]
    #[must_use]
    fn map<S, V: From<(S, S, S)>>(&self, f: impl Fn(Self::T) -> S) -> V {
        (f(self.x()), f(self.y()), f(self.z())).into()
    }
}

pub trait Vector4 {
    type T: MinMax;
    const X: Self;
    const Y: Self;
    const Z: Self;
    const W: Self;

    #[must_use]
    fn x(&self) -> Self::T;

    #[must_use]
    fn y(&self) -> Self::T;

    #[must_use]
    fn z(&self) -> Self::T;

    #[must_use]
    fn w(&self) -> Self::T;

    #[must_use]
    fn x_mut(&mut self) -> &mut Self::T;

    #[must_use]
    fn y_mut(&mut self) -> &mut Self::T;

    #[must_use]
    fn z_mut(&mut self) -> &mut Self::T;

    #[must_use]
    fn w_mut(&mut self) -> &mut Self::T;

    #[inline(always)]
    #[must_use]
    fn get(&self, axis: Axis4) -> Self::T {
        match axis {
            Axis4::X => self.x(),
            Axis4::Y => self.y(),
            Axis4::Z => self.z(),
            Axis4::W => self.w(),
        }
    }

    #[inline(always)]
    #[must_use]
    fn get_mut(&mut self, axis: Axis4) -> &mut Self::T {
        match axis {
            Axis4::X => self.x_mut(),
            Axis4::Y => self.y_mut(),
            Axis4::Z => self.z_mut(),
            Axis4::W => self.w_mut(),
        }
    }

    #[inline(always)]
    #[must_use]
    fn map<S, V: From<(S, S, S, S)>>(&self, f: impl Fn(Self::T) -> S) -> V {
        (f(self.x()), f(self.y()), f(self.z()), f(self.w())).into()
    }
}

pub trait MinMax {
    fn min(self, other: Self) -> Self;

    fn max(self, other: Self) -> Self;
}

seq_literal!(N in (32, 64) {
    impl MinMax for f~N {
        fn min(self, other: Self) -> Self {
            self.min(other)
        }

        fn max(self, other: Self) -> Self {
            self.max(other)
        }
    }
});

seq_literal_2!(M in ("i", "u") and N in (8, 16, 32, 64, 128) {
    impl MinMax for [<M N>] {
        fn min(self, other: Self) -> Self {
            Ord::min(self, other)
        }

        fn max(self, other: Self) -> Self {
            Ord::max(self, other)
        }
    }
});

pub trait InRange {
    type T;

    fn in_range(&self, range: impl RangeBounds<Self::T>) -> bool;
}

macro_rules! impl_range {
    ($ty:ty) => {
        impl InRange for $ty {
            type T = Self;

            #[inline]
            fn in_range(&self, range: impl RangeBounds<<Self as InRange>::T>) -> bool {
                range.contains(&self)
            }
        }
    };
}

seq_literal!(N in (8, 16, 32, 64, 128) {
    impl_range!(u~N);
    impl_range!(i~N);
});
impl_range!(f32);
impl_range!(f64);
impl_range!(usize);
impl_range!(isize);