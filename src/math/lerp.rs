use crate::math::vec2f::Vec2F32;
use crate::math::vec2f::Vec2F64;
use crate::math::vec3f::Vec3F32;
use crate::math::vec3f::Vec3F64;
use crate::math::vec4f::Vec4F32;
use crate::math::vec4f::Vec4F64;
use crate::{seq_floats, seq_literal_2};

pub enum LerpMode {
    /// Performs linear interpolation between the current present value (which becomes the past value) and the new one.
    Interpolate,
    /// Moves the value immediately without interpolation.
    Immediate,
    /// Overrides the last present value, without altering the past one.
    /// Linear interpolation will take place between this new present value and the past one.
    Override,
}

///Stands for "Past and Present", as it holds a past value and a present value.
pub struct PaP<T: Copy + Lerp>(T, T);

impl<T: Copy + Lerp> PaP<T> {
    #[inline]
    pub fn new(t: T) -> Self {
        PaP(t, t)
    }

    pub fn past_and_present(past: T, present: T) -> Self {
        Self(past, present)
    }

    #[inline]
    pub fn get(&self, partial_tick: f32) -> T {
        self.1.lerp(self.0, partial_tick)
    }

    #[inline]
    pub fn get_past(&self) -> T {
        self.0
    }

    #[inline]
    pub fn get_present(&self) -> T {
        self.1
    }

    pub fn update(&mut self, new_val: T, lerp_mode: LerpMode) {
        match lerp_mode {
            LerpMode::Interpolate => {
                self.0 = self.1;
                self.1 = new_val;
            }
            LerpMode::Immediate => {
                self.0 = new_val;
                self.1 = new_val;
            }
            LerpMode::Override => {
                self.1 = new_val;
            }
        }
    }
}

pub trait Lerp {
    fn lerp(self, other: Self, t: f32) -> Self;
}

seq_floats!(N {
    impl Lerp for [< N >] {
        fn lerp(self, other: Self, t: f32) -> Self {
            #[allow(clippy::unnecessary_cast)]
            let now = self * t as Self;
            #[allow(clippy::unnecessary_cast)]
            let prev = other * (1.0 - t) as Self;
            now + prev
        }
    }
});

seq_literal_2!(D in (2, 3, 4) and N in (32, 64) {
    impl Lerp for [< Vec D F N >] {
        fn lerp(self, other: Self, t: f32) -> Self {
            #[allow(clippy::unnecessary_cast)]
            let now = self * t as [<f N>];
            #[allow(clippy::unnecessary_cast)]
            let prev = other * (1.0 - t) as [<f N>];
            now + prev
        }
    }
});