use keyval::{KeyVal, Memory, Store};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = KeyVal::<_, String, i32>::new(Memory::new());

    store.insert(String::from("key"), 200).await?;

    Ok(())
}
