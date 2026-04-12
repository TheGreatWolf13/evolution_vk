pub mod angle;
pub mod color;
pub mod mat3;
pub mod mat4;
pub mod quat;
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