use super::error::Error;
use super::keyval::{Store, Ttl, TtlStore};
use async_mutex::Mutex;
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Instant;

struct TtlWrapItem<V> {
    ttl: Option<Instant>,
    data: V,
}

pub struct TtlWrap<S, K, V> {
    store: S,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

unsafe impl<S, K, V> Sync for TtlWrap<S, K, V> where S: Sync {}

unsafe impl<S, K, V> Send for TtlWrap<S, K, V> where S: Send {}

impl<S, K, V> TtlWrap<S, K, V> {
    pub fn new(store: S) -> TtlWrap<S, K, V> {
        TtlWrap {
            store,
            _k: PhantomData,
            _v: PhantomData,
        }
    }
}

#[async_trait]
impl<S, K, V> Store<K, V> for TtlWrap<S, K, V>
where
    S: Store<K, TtlWrapItem<V>>,
    K: Send + Sync,
    V: Send + Clone,
{
    async fn insert(&self, key: K, value: V) -> Result<(), Error> {
        self.store
            .insert(
                key,
                TtlWrapItem {
                    ttl: None,
                    data: value,
                },
            )
            .await
    }

    async fn get(&self, key: &K) -> Result<V, Error> {
        let ret = match self.store.get(key).await {
            Ok(v) => {
                if let Some(ttl) = v.ttl {
                    let now = Instant::now();
                    if now > ttl {
                        None
                    } else {
                        Some(v.data.clone())
                    }
                } else {
                    Some(v.data.clone())
                }
            }
            Err(err) => return Err(err),
        };

        match ret {
            Some(s) => Ok(s),
            None => {
                self.store.remove(key).await?;
                Err(Error::Expired)
            }
        }
    }
    async fn remove(&self, key: &K) -> Result<(), Error> {
        self.store.remove(key).await
    }
}

#[async_trait]
impl<S, K, V> TtlStore<K, V> for TtlWrap<S, K, V>
where
    S: Store<K, TtlWrapItem<V>>,
    K: Send + Sync + Clone,
    V: Send + Clone,
{
    async fn insert_ttl(&self, key: K, ttl: Ttl, value: V) -> Result<(), Error> {
        self.store
            .insert(
                key,
                TtlWrapItem {
                    ttl: Some(ttl),
                    data: value,
                },
            )
            .await
    }
    async fn touch(&self, key: &K, ttl: Ttl) -> Result<(), Error> {
        let mut item = self.store.get(key).await?;
        item.ttl = Some(ttl);
        self.store.insert(key.clone(), item).await?;
        Ok(())
    }
}
