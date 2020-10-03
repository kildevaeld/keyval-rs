use async_trait::async_trait;
use keyval::{Error as KeyValError, Store};
use kv::Store as StoreBackend;
use std::error::Error as StdError;
use std::fmt;
use std::marker::PhantomData;

#[derive(Debug)]
pub enum Error {
    NotFound,
    Kv(kv::Error),
    SpawnError(runtime::SpawnError),
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

impl From<runtime::SpawnError> for Error {
    fn from(error: runtime::SpawnError) -> Self {
        Error::SpawnError(error)
    }
}

impl From<kv::Error> for Error {
    fn from(error: kv::Error) -> Self {
        Error::Kv(error)
    }
}

pub struct KvStore<K, V> {
    store: StoreBackend,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

impl<K, V> KvStore<K, V> {
    pub fn new(store: StoreBackend) -> KvStore<K, V> {
        KvStore {
            store,
            _k: PhantomData,
            _v: PhantomData,
        }
    }
}

#[async_trait]
impl<K, V> Store<K, V> for KvStore<K, V>
where
    K: Clone + 'static + Sync + Send + for<'a> kv::Key<'a>,
    V: 'static + Sync + Send + kv::Value,
{
    async fn insert(&self, key: K, value: V) -> Result<(), KeyValError> {
        let store = self.store.clone();
        runtime::spawn_blocking(move || {
            let bucket = store.bucket::<K, V>(None)?;
            bucket.set(key, value)
        })
        .await
        .map_err(Error::SpawnError)?
        .map_err(Error::Kv)?;
        Ok(())
    }

    async fn get(&self, key: &K) -> Result<V, KeyValError> {
        let key = key.clone();
        let store = self.store.clone();
        let ret = runtime::spawn_blocking(move || {
            let bucket = store.bucket::<K, V>(None)?;
            bucket.get(key)
        })
        .await
        .map_err(Error::SpawnError)?
        .map_err(Error::Kv)?;

        match ret {
            Some(ret) => Ok(ret),
            None => Err(KeyValError::NotFound),
        }
    }

    async fn remove(&self, key: &K) -> Result<(), KeyValError> {
        let key = key.clone();
        let store = self.store.clone();
        runtime::spawn_blocking(move || {
            let bucket = store.bucket::<K, V>(None)?;
            bucket.remove(key)
        })
        .await
        .map_err(Error::SpawnError)?
        .map_err(Error::Kv)?;

        Ok(())
    }
}
