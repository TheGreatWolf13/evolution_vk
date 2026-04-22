use crate::util::error::Throwable;

pub mod error;
pub mod timer;
mod random;

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

#[macro_export]
macro_rules! if_else {
    ($e:expr => $if_true:expr ; $if_false:expr) => {
        if $e {
            $if_true
        } //
        else {
            $if_false
        }
    }
}