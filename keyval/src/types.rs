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

impl<'a> Key for &'a [u8] {}

impl<'a> Key for &'a str {}

impl Key for Vec<u8> {}

impl Key for String {}

impl Key for Integer {}
// Integer

/// Integer key type
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Integer([u8; 16]);

impl From<u128> for Integer {
    fn from(i: u128) -> Integer {
        unsafe { Integer(std::mem::transmute(i.to_be())) }
    }
}

impl From<u64> for Integer {
    fn from(i: u64) -> Integer {
        let i = i as u128;
        i.into()
    }
}

impl From<u32> for Integer {
    fn from(i: u32) -> Integer {
        let i = i as u128;
        i.into()
    }
}

impl From<i32> for Integer {
    fn from(i: i32) -> Integer {
        let i = i as u128;
        i.into()
    }
}

impl From<usize> for Integer {
    fn from(i: usize) -> Integer {
        let i = i as u128;
        i.into()
    }
}

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

impl From<Integer> for u64 {
    fn from(i: Integer) -> u64 {
        let i: u128 = i.into();
        i as u64
    }
}

impl From<Integer> for usize {
    fn from(i: Integer) -> usize {
        let i: u128 = i.into();
        i as usize
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

// impl Integer {
//     /// Current timestamp in seconds from the Unix epoch
//     pub fn timestamp() -> Result<Integer, Error> {
//         let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
//         Ok(Integer::from(ts.as_secs() as u128))
//     }

//     /// Current timestamp in milliseconds from the Unix epoch
//     pub fn timestamp_ms() -> Result<Integer, Error> {
//         let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
//         Ok(Integer::from(ts.as_millis()))
//     }
// }

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
        // let x = r.to_vec();
        Ok(r)
    }
}
