use keyval::{KeyVal, Memory, Store};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = KeyVal::new(Memory::new());

    store.insert("test", String::from("Hello, World")).await?;

    let value: String = store.get("test").await?;

    Ok(())
}
