# Env Required
- DB_PATH
- DB_NS
- DB_DB
- DB_USER
- DB_SECRET

DB_PATH is the path to the database. It can be a file path or a URL.  
&nbsp;&nbsp;&nbsp;&nbsp;*note* : If using file path... prepend the path with some file descriptor like, but perhaps not exactly, `file://`  [More Info](https://docs.rs/surrealdb/latest/surrealdb/engine/any/index.html)  
DB_NS is the namespace of the database.  
DB_DB is the name of the database.  
DB_USER is the username of the database.  
DB_SECRET is the password of the database.  

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