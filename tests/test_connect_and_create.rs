use nico_surreal_client::close;
use nico_surreal_client::Error;
use nico_surreal_client::Storable;
use serde::{Deserialize, Serialize};
// use surrealdb::sql::Thing;

const TEST_TABLE: &str = "test_table";
const TEST_PERSON: &str = "test_person";
const TEST_PERSON_TWO: &str = "test_person_2";

// Definition
#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
}

impl<'a> Storable<'a> for Person {
    type Item = Self;

    fn table(&self) -> &'a str {
        TEST_TABLE
    }

    fn id(&self) -> &'a str {
        TEST_PERSON
    }
}

// API Call or Factory
fn person_factory(name: &str, age: u8) -> Option<Person> {
    Some(Person {
        name: name.to_string(),
        age,
    })
}

#[tokio::test]
async fn test_storable() -> Result<(), Error> {
    // Some Test Setup
    use nico_surreal_client::connect;
    use nico_surreal_client::prelude::{delete_record, Record};
    connect(None).await.ok();
    let _: Option<Record<Person>> = delete_record(TEST_TABLE, TEST_PERSON).await.ok();
    close().await.ok();
    // End Test Setup

    // Initiate the connection using ENV Vars
    let john = person_factory("John", 32).unwrap();
    // Create the Record
    let saved_john = john.save().await?;

    // Some Logging
    println!("Created -> Yes");
    println!("({}:{}) -> {:?}", TEST_TABLE, TEST_PERSON, saved_john);

    // Delete the Record
    let _old_john = saved_john.unwrap().into_inner().delete().await?;

    Ok(())
}

#[tokio::test]
async fn test_basic_store() -> Result<(), Error> {
    use nico_surreal_client::prelude::*;
    // Initiate the connection using ENV Vars
    connect(None).await?;
    let _: Option<Record<Person>> = delete_record(TEST_TABLE, TEST_PERSON_TWO).await.ok();

    // Create the Record
    let created: Option<Record<Person>> =
        create_record::<Person>(TEST_TABLE, TEST_PERSON_TWO, person_factory("John", 32)).await?;

    // Check if we got a record
    let record: Option<Record<Thing>> = get_record(TEST_TABLE, TEST_PERSON_TWO).await?;

    // Update the Record
    // let updated: Option<Record<Person>> = update_record(TEST_TABLE, TEST_PERSON_TWO, person_factory("John", 33)).await?;

    // Cleaning up
    let _: Option<Record<Person>> = delete_record(TEST_TABLE, TEST_PERSON_TWO).await.ok();

    // Some Logging
    println!("Created -> {:?}", created);
    println!("Record -> {:?}", record);

    // We Succeeded so Ret 0
    close().await.ok();
    Ok(())
}
