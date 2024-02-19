# Env Required
- DB_PATH
- DB_NS
- DB_DB
- DB_USER
- DB_SECRET

# Example
```rust
use nico_surreal_client::prelude::*;
use surrealdb::sql::Thing;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Person {
    name: String,
    age: u8,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    connect(None).await?;
    create_table("test_table").await?;
    let _: Option<Record<Person>> = delete_record("test_table", "test_person").await.ok();

    let created: Option<Record<Person>> = create_record_data::<Person>(
        Thing::from(("test_table", "test_person")),
        Person {
            name: "John".to_string(),
            age: 32,
        },
    )
    .await
    .ok()
    .flatten();

    println!("Created -> {:?}", created);

    let record: Vec<Record<Thing>> = get_record("test_table", "test_person").await?;
    println!("Record -> {:?}", record);

    Ok(())
}
```