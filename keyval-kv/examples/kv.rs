use keyval::Store;
use keyval_kv::KvStore;
use kv::{Config, Store as STORE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = STORE::new(Config::new("./example.db"))?;

    let store = KvStore::<String, String>::new(store);

    store
        .insert(String::from("key"), "Hellow".to_owned())
        .await?;

    Ok(())
}
