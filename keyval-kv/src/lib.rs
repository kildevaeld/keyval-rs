use async_trait::async_trait;
use keyval::{Error as KeyValError, Raw, Store};
use kv::Store as StoreBackend;
use std::error::Error as StdError;
use std::fmt;

pub use kv;

#[derive(Debug)]
pub enum Error {
    NotFound,
    Kv(kv::Error),
    SpawnError(tokio::task::JoinError),
}

impl From<Error> for KeyValError {
    fn from(_error: Error) -> KeyValError {
        KeyValError::NotFound
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotFound => write!(f, "not found"),
            Error::Kv(err) => write!(f, "kv error {}", err),
            Error::SpawnError(err) => write!(f, "spawn error {}", err),
        }
    }
}

impl StdError for Error {}

impl From<tokio::task::JoinError> for Error {
    fn from(error: tokio::task::JoinError) -> Self {
        Error::SpawnError(error)
    }
}

impl From<kv::Error> for Error {
    fn from(error: kv::Error) -> Self {
        Error::Kv(error)
    }
}

pub struct KvStore {
    store: StoreBackend,
}

impl KvStore {
    pub fn new(store: StoreBackend) -> KvStore {
        KvStore { store }
    }
}

#[async_trait]
impl Store for KvStore {
    async fn insert(&self, key: Raw, value: Raw) -> Result<(), KeyValError> {
        let store = self.store.clone();
        tokio::task::spawn_blocking(move || {
            let bucket = store.bucket::<Raw, Raw>(None)?;
            bucket.set(&key, &value)
        })
        .await
        .map_err(Error::SpawnError)?
        .map_err(Error::Kv)?;
        Ok(())
    }

    async fn get(&self, key: &Raw) -> Result<Raw, KeyValError> {
        let key = key.clone();
        let store = self.store.clone();
        let ret = tokio::task::spawn_blocking(move || {
            let bucket = store.bucket::<Raw, Raw>(None)?;
            bucket.get(&key)
        })
        .await
        .map_err(Error::SpawnError)?
        .map_err(Error::Kv)?;

        match ret {
            Some(ret) => Ok(ret),
            None => Err(KeyValError::NotFound),
        }
    }

    async fn remove(&self, key: &Raw) -> Result<(), KeyValError> {
        let key = key.clone();
        let store = self.store.clone();
        tokio::task::spawn_blocking(move || {
            let bucket = store.bucket::<Raw, Raw>(None)?;
            bucket.remove(&key)
        })
        .await
        .map_err(Error::SpawnError)?
        .map_err(Error::Kv)?;

        Ok(())
    }
}
