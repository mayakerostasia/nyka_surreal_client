use nico_surreal_client::prelude::*;
use surrealdb::sql::Thing;

const TEST_TABLE: &str = "test_table";
const TEST_PERSON: &str = "test_person"; 

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Person {
    // id: String,
    name: String,
    age: u8,
}

#[tokio::test]
async fn create_get_delete() -> Result<(), Error> {
    connect(None).await?;
    // create_table("test_table").await?;
    let _: Option<Record<Person>> = delete_record(TEST_TABLE, TEST_PERSON).await.ok();

    let created: Option<Record<Person>> = create_record::<Person>(
        TEST_TABLE, TEST_PERSON,
        Some(Person {
            // id: format!("{}:{}", TEST_TABLE, TEST_PERSON),
            name: "John".to_string(),
            age: 32,
        }),
    )
    .await?;

    println!("Created -> {:?}", created);

    let record: Option<Record<Person>> = get_record(TEST_TABLE, TEST_PERSON).await?;
    println!("Record -> {:?}", record);

    Ok(())
}
