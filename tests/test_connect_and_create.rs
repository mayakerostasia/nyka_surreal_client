// use surrealdb::sql::Thing;
use nico_surreal_client::{create_record_data, delete_record, prelude::*, Record};
use surrealdb::sql::Thing;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Person {
    name: String,
    age: u8,
}

#[tokio::test]
async fn test_connect_and_create() -> Result<(), Error> {
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
