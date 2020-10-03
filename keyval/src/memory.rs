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

struct MemoryItem<V> {
    ttl: Option<Instant>,
    data: V,
}

// #[derive(Debug)]
// pub enum MemoryError {
//     NotFound,
//     Expired,
// }

// impl From<MemoryError> for Error {
//     fn from(error: MemoryError) -> Error {
//         match error {
//             MemoryError::NotFound => Error::NotFound,
//             MemoryError::Expired => Error::Expired,
//         }
//     }
// }

// impl fmt::Display for MemoryError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             MemoryError::NotFound => write!(f, "Not found"),
//             MemoryError::Expired => write!(f, "Expired"),
//         }
//     }
// }

// impl StdError for MemoryError {}

pub struct Memory<K, V> {
    db: Arc<Mutex<HashMap<K, MemoryItem<V>>>>,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

unsafe impl<K, V> Sync for Memory<K, V> {}

unsafe impl<K, V> Send for Memory<K, V> {}

impl<K, V> Memory<K, V> {
    pub fn new() -> Memory<K, V> {
        Memory {
            db: Arc::new(Mutex::new(HashMap::new())),
            _k: PhantomData,
            _v: PhantomData,
        }
    }
}

#[async_trait]
impl<K, V> Store<K, V> for Memory<K, V>
where
    K: Eq + std::hash::Hash + Send + Sync,
    V: Clone + Send + Sync,
{
    async fn insert(&self, key: K, value: V) -> Result<(), Error> {
        let data = value.clone();
        let mut lock = self.db.lock().await;
        lock.insert(key, MemoryItem { ttl: None, data });
        Ok(())
    }
    async fn get(&self, key: &K) -> Result<V, Error> {
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
    async fn remove(&self, key: &K) -> Result<(), Error> {
        let mut lock = self.db.lock().await;
        lock.remove(key);
        Ok(())
    }
}

#[async_trait]
impl<K, V> TtlStore<K, V> for Memory<K, V>
where
    K: Eq + std::hash::Hash + Send + Sync,
    V: Clone + Send + Sync,
{
    async fn insert_ttl(&self, key: K, ttl: Ttl, value: &V) -> Result<(), Error> {
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
    async fn touch(&self, key: &K, ttl: Ttl) -> Result<(), Error> {
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
