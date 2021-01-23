use super::{Error, Key, Store, Ttl, TtlStore, Value};
use std::sync::Arc;

pub struct KeyVal<S>(Arc<S>);

impl<S> Clone for KeyVal<S> {
    fn clone(&self) -> KeyVal<S> {
        KeyVal(self.0.clone())
    }
}

impl<S> KeyVal<S> {
    pub fn new(store: S) -> KeyVal<S> {
        KeyVal(Arc::new(store))
    }
}

impl<S> KeyVal<S>
where
    S: Store,
{
    pub async fn insert<K: Key, V: Value>(&self, key: K, value: V) -> Result<(), Error> {
        self.0.insert(key.to_raw()?, value.to_raw()?).await
    }

    pub async fn get<K: Key, V: Value>(&self, key: K) -> Result<V, Error> {
        let raw = self.0.get(&key.to_raw()?).await?;
        Ok(V::from_raw(raw)?)
    }

    pub async fn remove<K: Key>(&self, key: &K) -> Result<(), Error> {
        self.0.remove(&key.to_raw()?).await
    }
}

impl<S> KeyVal<S>
where
    S: TtlStore,
{
    pub async fn insert_ttl<K: Key, V: Value>(
        &self,
        key: K,
        value: V,
        ttl: Ttl,
    ) -> Result<(), Error> {
        self.0.insert_ttl(key.to_raw()?, ttl, value.to_raw()?).await
    }
    pub async fn touch<K: Key, V: Value>(&self, key: K, ttl: Ttl) -> Result<(), Error> {
        self.0.touch(&key.to_raw()?, ttl).await
    }
}
