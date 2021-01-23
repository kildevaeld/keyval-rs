use super::error::Error;
use super::types::{Raw, Store, Ttl, TtlStore};
use async_mutex::Mutex;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

struct MemoryItem {
    ttl: Option<Instant>,
    data: Vec<u8>,
}

pub struct Memory {
    db: Arc<Mutex<HashMap<Raw, MemoryItem>>>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            db: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Store for Memory {
    async fn insert(&self, key: Raw, value: Raw) -> Result<(), Error> {
        let data = value.clone();
        let mut lock = self.db.lock().await;
        lock.insert(key, MemoryItem { ttl: None, data });
        Ok(())
    }

    async fn get(&self, key: &Raw) -> Result<Raw, Error> {
        let mut lock = self.db.lock().await;
        let ret = match lock.get(key) {
            Some(v) => {
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
            None => return Err(Error::NotFound),
        };

        match ret {
            Some(s) => Ok(s),
            None => {
                lock.remove(key);
                Err(Error::Expired)
            }
        }
    }
    async fn remove(&self, key: &Raw) -> Result<(), Error> {
        let mut lock = self.db.lock().await;
        lock.remove(key);
        Ok(())
    }
}

#[async_trait]
impl TtlStore for Memory {
    async fn insert_ttl(&self, key: Raw, ttl: Ttl, value: Raw) -> Result<(), Error> {
        let mut lock = self.db.lock().await;
        lock.insert(
            key,
            MemoryItem {
                ttl: Some(ttl),
                data: value.clone(),
            },
        );
        Ok(())
    }
    async fn touch(&self, key: &Raw, ttl: Ttl) -> Result<(), Error> {
        let mut lock = self.db.lock().await;
        match lock.get_mut(key) {
            Some(m) => {
                m.ttl = Some(ttl);
                Ok(())
            }
            None => Err(Error::NotFound),
        }
    }
}
