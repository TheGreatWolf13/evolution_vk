use crate::util::error::Throwable;

pub mod error;

pub type Result<T = ()> = core::result::Result<T, Throwable>;

#[macro_export]
macro_rules! impl_from {
    ($t:ty as $other:ty: $value:ident => $e:expr) => {
        impl From<$t> for $other {

            #[inline]
            fn from($value: $t) -> Self {
                $e
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref {
    ($t:ty as $other:ty: $self:ident => $e:expr) => {
        impl std::ops::Deref for $t {
            type Target = $other;

            #[inline]
            fn deref(&$self) -> &Self::Target {
                $e
            }
        }
    };
}