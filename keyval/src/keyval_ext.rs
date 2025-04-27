use std::sync::Arc;

use async_trait::async_trait;

use crate::{Error, Raw, Ttl, TtlStore};

use super::ttl_wrap::TtlWrap;
use super::Store;

pub trait StoreExt: Store + Sized {
    fn into_ttl(self) -> TtlWrap<Self> {
        TtlWrap::new(self)
    }

    fn into_shared(self) -> SharedStore<Self> {
        SharedStore { inner: self.into() }
    }
}

impl<S> StoreExt for S where S: Store {}

pub struct SharedStore<T> {
    inner: Arc<T>,
}

#[async_trait]
impl<T> Store for SharedStore<T>
where
    T: Store,
{
    async fn insert(&self, key: Raw, value: Raw) -> Result<(), Error> {
        self.inner.insert(key, value).await
    }
    async fn get(&self, key: &Raw) -> Result<Raw, Error> {
        self.inner.get(key).await
    }
    async fn remove(&self, key: &Raw) -> Result<(), Error> {
        self.inner.remove(key).await
    }
}

#[async_trait]
impl<T> TtlStore for SharedStore<T>
where
    T: TtlStore,
{
    async fn insert_ttl(&self, key: Raw, ttl: Ttl, value: Raw) -> Result<(), Error> {
        self.inner.insert_ttl(key, ttl, value).await
    }
    async fn touch(&self, key: &Raw, ttl: Ttl) -> Result<(), Error> {
        self.inner.touch(key, ttl).await
    }
}
