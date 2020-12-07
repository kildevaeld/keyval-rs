use keyval::{Memory, Store};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = Memory::new();

    store.insert(String::from("key"), 200).await?;

    store.get(&String::from("key")).await?;

    Ok(())
}
