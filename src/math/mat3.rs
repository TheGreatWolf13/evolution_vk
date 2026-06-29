use crate::impl_from;
use crate::math::quat::Quat32;
use crate::math::quat::Quat64;
use glam::DMat3 as M64;
use glam::Mat3 as M32;
use super_seq_macro::seq;

seq!(N in (5..=6).collect().map(|i| 1 << i) {
    #[derive(Debug, Copy, Clone)]
    pub struct Mat3F~N(pub(super) M~N);

    impl Mat3F~N {

        #[inline(always)]
        #[must_use]
        pub fn from_quat(rotation: Quat~N) -> Self {
            Self(M~N::from_quat(rotation.0))
        }
    }

    //From
    impl_from!(Mat3F~N as [[f~N; 3]; 3]: v => v.0.to_cols_array_2d());
    impl_from!([[f~N; 3]; 3] as Mat3F~N: v => Self(M~N::from_cols_array_2d(&v)));
});