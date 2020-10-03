use super::error::Error;
use async_trait::async_trait;
use std::marker::PhantomData;
use std::time::Instant;

pub type Ttl = Instant;

#[async_trait]
pub trait Store<K, V>: Send + Sync {
    async fn insert(&self, key: K, value: V) -> Result<(), Error>;
    async fn get(&self, key: &K) -> Result<V, Error>;
    async fn remove(&self, key: &K) -> Result<(), Error>;
}

#[async_trait]
pub trait TtlStore<K, V>: Store<K, V> {
    async fn insert_ttl(&self, key: K, ttl: Ttl, value: V) -> Result<(), Error>;
    async fn touch(&self, key: &K, ttl: Ttl) -> Result<(), Error>;
}
