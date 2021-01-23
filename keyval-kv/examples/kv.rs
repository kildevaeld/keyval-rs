use keyval::Store;
use keyval_kv::KvStore;
use kv::{Config, Store as STORE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = STORE::new(Config::new("./example.db"))?;

    let store = KvStore::new(store);

    store.insert(b"key".to_vec(), b"Hellow".to_vec()).await?;

    Ok(())
}
