#[allow(unused_imports)]
use crate::{Error, Raw, Value};

/// Base trait for values that can be encoded using serde
pub trait Codec<T: serde::Serialize + serde::de::DeserializeOwned>:
    Value + AsRef<T> + AsMut<T>
{
    /// Convert back into inner value
    fn into_inner(self) -> T;
}

#[macro_export]
/// Define a codec type and implement the Codec trait
macro_rules! codec {
    ($x:ident) => {
        /// Codec implementation
        pub struct $x<T: serde::Serialize + serde::de::DeserializeOwned>(pub T);

        impl<T: serde::Serialize + serde::de::DeserializeOwned> AsRef<T> for $x<T> {
            fn as_ref(&self) -> &T {
                &self.0
            }
        }

        impl<T: serde::Serialize + serde::de::DeserializeOwned> AsMut<T> for $x<T> {
            fn as_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }

        impl<T: serde::Serialize + serde::de::DeserializeOwned> Codec<T> for $x<T> {
            fn into_inner(self) -> T {
                self.0
            }
        }

        impl<T: Clone + serde::Serialize + serde::de::DeserializeOwned> Clone for $x<T> {
            fn clone(&self) -> Self {
                $x(self.0.clone())
            }
        }
    };

    ($x:ident, {$ser:expr, $de:expr}) => {
        codec!($x);

        impl<T: serde::Serialize + serde::de::DeserializeOwned> Value for $x<T> {
            fn to_raw(self) -> Result<Raw, Error> {
                let x = $ser(&self.0)?;
                Ok(x.into())
            }

            fn from_raw(r: Raw) -> Result<Self, Error> {
                let x = $de(&r)?;
                Ok($x(x))
            }
        }
    };
}

#[cfg(feature = "cbor")]
mod cbor_value {
    use super::*;
    codec!(Cbor, { serde_cbor::to_vec, serde_cbor::from_slice});
}

#[cfg(feature = "cbor")]
pub use cbor_value::*;
