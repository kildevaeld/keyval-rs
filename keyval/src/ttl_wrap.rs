use super::error::Error;
use super::types::{Raw, Store, Ttl, TtlStore};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Serialize, Deserialize)]
pub struct TtlWrapItem {
    #[serde(with = "serde_millis")]
    ttl: Option<Instant>,
    data: Vec<u8>,
}

pub struct TtlWrap<S> {
    store: S,
}

impl<S> TtlWrap<S> {
    pub fn new(store: S) -> TtlWrap<S> {
        TtlWrap { store }
    }
}

#[async_trait]
impl<S> Store for TtlWrap<S>
where
    S: Store,
{
    async fn insert(&self, key: Raw, value: Raw) -> Result<(), Error> {
        let item = serde_cbor::to_vec(&TtlWrapItem {
            ttl: None,
            data: value,
        })?;
        self.store.insert(key, item).await
    }

    async fn get(&self, key: &Raw) -> Result<Raw, Error> {
        let ret = match self.store.get(key).await {
            Ok(v) => {
                let v: TtlWrapItem = serde_cbor::from_slice(&v)?;
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
    async fn remove(&self, key: &Raw) -> Result<(), Error> {
        self.store.remove(key).await
    }
}

#[async_trait]
impl<S> TtlStore for TtlWrap<S>
where
    S: Store,
{
    async fn insert_ttl(&self, key: Raw, ttl: Ttl, value: Raw) -> Result<(), Error> {
        let item = serde_cbor::to_vec(&TtlWrapItem {
            ttl: Some(ttl),
            data: value,
        })?;
        self.store.insert(key, item).await
    }
    async fn touch(&self, key: &Raw, ttl: Ttl) -> Result<(), Error> {
        let item = self.store.get(key).await?;
        let mut item: TtlWrapItem = serde_cbor::from_slice(&item)?;
        item.ttl = Some(ttl);
        let item = serde_cbor::to_vec(&item)?;
        self.store.insert(key.clone(), item).await?;
        Ok(())
    }
}
