use keyval::{Cbor, KeyVal, Memory, Store};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    test: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = KeyVal::new(Memory::new());

    store
        .insert(
            "test",
            Cbor(Test {
                test: "Test".to_string(),
            }),
        )
        .await?;

    let value: Cbor<Test> = store.get(b"test".as_ref()).await?;

    println!("Value {:?}", value);

    Ok(())
}
