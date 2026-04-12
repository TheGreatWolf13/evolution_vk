use crate::impl_from;
use crate::math::quat::Quat;

#[derive(Debug, Copy, Clone)]
pub struct Mat3(glam::Mat3);

impl Mat3 {
    #[inline]
    #[must_use]
    pub fn from_quat(rotation: Quat) -> Self {
        Self(glam::Mat3::from_quat(rotation.0))
    }
}

//From
impl_from!(Mat3 as [[f32; 3]; 3]: v => v.0.to_cols_array_2d());