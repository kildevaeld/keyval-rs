use super::error::Error;
use async_trait::async_trait;
use std::time::Instant;

pub type Ttl = Instant;

// pub struct Raw {}
pub type Raw = Vec<u8>;

pub trait Key: Sized + AsRef<[u8]> {
    // fn from_raw(raw: &Raw) -> Result<Self, Error>;
    fn to_raw(&self) -> Result<Raw, Error> {
        Ok(self.as_ref().into())
    }
}

pub trait Value: Sized {
    fn to_raw(self) -> Result<Raw, Error>;
    fn from_raw(r: Raw) -> Result<Self, Error>;
}

#[async_trait]
pub trait Store: Send + Sync {
    async fn insert(&self, key: Raw, value: Raw) -> Result<(), Error>;
    async fn get(&self, key: &Raw) -> Result<Raw, Error>;
    async fn remove(&self, key: &Raw) -> Result<(), Error>;
}

#[async_trait]
pub trait TtlStore: Store {
    async fn insert_ttl(&self, key: Raw, ttl: Ttl, value: Raw) -> Result<(), Error>;
    async fn touch(&self, key: &Raw, ttl: Ttl) -> Result<(), Error>;
}

#[async_trait]
impl Store for Box<dyn Store> {
    async fn insert(&self, key: Raw, value: Raw) -> Result<(), Error> {
        self.as_ref().insert(key, value).await
    }
    async fn get(&self, key: &Raw) -> Result<Raw, Error> {
        self.as_ref().get(key).await
    }
    async fn remove(&self, key: &Raw) -> Result<(), Error> {
        self.as_ref().remove(key).await
    }
}

#[async_trait]
impl Store for Box<dyn TtlStore> {
    async fn insert(&self, key: Raw, value: Raw) -> Result<(), Error> {
        self.as_ref().insert(key, value).await
    }
    async fn get(&self, key: &Raw) -> Result<Raw, Error> {
        self.as_ref().get(key).await
    }
    async fn remove(&self, key: &Raw) -> Result<(), Error> {
        self.as_ref().remove(key).await
    }
}

#[async_trait]
impl TtlStore for Box<dyn TtlStore> {
    async fn insert_ttl(&self, key: Raw, ttl: Ttl, value: Raw) -> Result<(), Error> {
        self.as_ref().insert_ttl(key, ttl, value).await
    }
    async fn touch(&self, key: &Raw, ttl: Ttl) -> Result<(), Error> {
        self.as_ref().touch(key, ttl).await
    }
}

impl<'a> Key for &'a [u8] {}

impl<'a> Key for &'a str {}

impl Key for Vec<u8> {}

impl Key for String {}

impl Key for Integer {}
// Integer

/// Integer key type
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Integer(pub(crate) [u8; 16]);

impl From<u128> for Integer {
    fn from(i: u128) -> Integer {
        unsafe { Integer(std::mem::transmute(i.to_be())) }
    }
}

// impl From<u64> for Integer {
//     fn from(i: u64) -> Integer {
//         let i = i as u128;
//         i.into()
//     }
// }

// impl From<u32> for Integer {
//     fn from(i: u32) -> Integer {
//         let i = i as u128;
//         i.into()
//     }
// }

// impl From<i32> for Integer {
//     fn from(i: i32) -> Integer {
//         let i = i as u128;
//         i.into()
//     }
// }

// impl From<usize> for Integer {
//     fn from(i: usize) -> Integer {
//         let i = i as u128;
//         i.into()
//     }
// }

impl From<Integer> for u128 {
    #[cfg(target_endian = "big")]
    fn from(i: Integer) -> u128 {
        unsafe { mem::transmute(i.0) }
    }

    #[cfg(target_endian = "little")]
    fn from(i: Integer) -> u128 {
        u128::from_be(unsafe { std::mem::transmute(i.0) })
    }
}

impl AsRef<[u8]> for Integer {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<'a> From<&'a [u8]> for Integer {
    fn from(buf: &'a [u8]) -> Integer {
        let mut dst = Integer::from(0u128);
        dst.0[..16].clone_from_slice(&buf[..16]);
        dst
    }
}

impl Value for String {
    fn to_raw(self) -> Result<Raw, Error> {
        Ok(self.into())
    }

    fn from_raw(r: Raw) -> Result<Self, Error> {
        let x = r.to_vec();
        Ok(String::from_utf8(x)?)
    }
}

impl Value for Vec<u8> {
    fn to_raw(self) -> Result<Raw, Error> {
        Ok(self)
    }

    fn from_raw(r: Raw) -> Result<Self, Error> {
        Ok(r)
    }
}

macro_rules! impl_ints {
    ($ty: ty) => {
        impl From<$ty> for Integer {
            fn from(i: $ty) -> Integer {
                Integer::from(i as u128)
            }
        }

        impl From<Integer> for $ty {
            fn from(i: Integer) -> $ty {
                u128::from(i) as $ty
            }
        }

        impl Value for $ty {
            fn to_raw(self) -> Result<Raw, Error> {
                let i = Integer::from(self);
                Ok(i.as_ref().to_vec())
            }
            fn from_raw(r: Raw) -> Result<Self, Error> {
                let i = Integer::from(r.as_ref());
                Ok(i.into())
            }
        }
    };
}

impl_ints!(usize);
impl_ints!(i64);
impl_ints!(u64);
impl_ints!(i32);
impl_ints!(u32);
impl_ints!(i16);
impl_ints!(u16);
impl_ints!(u8);
impl_ints!(i8);
