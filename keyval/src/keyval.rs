use super::error::Error;
use async_trait::async_trait;
use std::marker::PhantomData;
use std::time::Instant;

pub type Ttl = Instant;

#[async_trait]
pub trait Store<K, V>: Send + Sync {
    type Error: Into<Error>;

    async fn insert(&self, key: K, value: V) -> Result<(), Self::Error>;
    async fn get(&self, key: &K) -> Result<V, Self::Error>;
    async fn remove(&self, key: &K) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait TtlStore<K, V>: Store<K, V> {
    async fn insert_ttl(&self, key: K, ttl: Ttl, value: &V) -> Result<(), Self::Error>;
    async fn touch(&self, key: &K, ttl: Ttl) -> Result<(), Self::Error>;
}

pub struct KeyVal<S, K, V>(S, PhantomData<K>, PhantomData<V>);

impl<S, K, V> KeyVal<S, K, V>
where
    S: Store<K, V>,
{
    pub fn new(store: S) -> KeyVal<S, K, V> {
        KeyVal(store, PhantomData, PhantomData)
    }

    pub async fn insert(&self, key: K, value: V) -> Result<(), Error> {
        Ok(self.0.insert(key, value).await.map_err(|err| err.into())?)
    }
    pub async fn get(&self, key: &K) -> Result<V, Error> {
        Ok(self.0.get(key).await.map_err(|err| err.into())?)
    }
    pub async fn remove(&self, key: &K) -> Result<(), Error> {
        Ok(self.0.remove(key).await.map_err(|err| err.into())?)
    }
}

impl<S, K, V> KeyVal<S, K, V>
where
    S: TtlStore<K, V>,
{
    pub async fn insert_ttl(&self, key: K, ttl: Ttl, value: &V) -> Result<(), Error> {
        Ok(self
            .0
            .insert_ttl(key, ttl, value)
            .await
            .map_err(|err| err.into())?)
    }
    pub async fn touch(&self, key: &K, ttl: Ttl) -> Result<(), Error> {
        Ok(self.0.touch(key, ttl).await.map_err(|err| err.into())?)
    }
}
