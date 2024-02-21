use nico_surreal_client::close;
use nico_surreal_client::Error;
use nico_surreal_client::Storable;
use nico_surreal_client::Ident;
use nico_surreal_client::Builder;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

const TEST_TABLE: &str = "test_table";
const TEST_PERSON: &str = "test_person";
const TEST_PERSON_TWO: &str = "test_person_2";

// Definition
// #[derive(Debug, Deserialize, Serialize)]
// struct Person {
//     name: String,
//     age: u8,
// }

// Definition
#[derive(Builder, Debug, Deserialize, Serialize, Clone)]
struct Person {
    #[serde(skip_serializing)]
    id: Ident,
    name: String,
    age: u8,
}
impl<'a> Storable<'a> for Person {
    type Item = Self;

    fn table(&self) -> String {
        TEST_TABLE.to_string()
    }

    fn id(&self) -> String {
        TEST_PERSON.to_string()
    }
}

// API Call or Factory
// use nico_surreal_client::
// fn person_factory(name: &str, age: u8) -> Option<Person> {
//     Some(Person {
//         name: name.to_string(),
//         age,
//     })
// }

// API Call or Factory
fn person_factory(id: Option<Ident>, name: &str, age: u8) -> Option<Person> {
    let mut person = Person::builder();
    person.name(name.to_string());
    person.age(age);
    if id.is_some() {
        println!("ID: {:?}", id);
        Some(person.id(id.unwrap()).build().ok()?)
    } else {
        Some(person.build().ok()?)
    }
}

#[tokio::test]
async fn test_storable() -> Result<(), Error> {
    // Some Test Setup
    use nico_surreal_client::connect;
    use nico_surreal_client::prelude::{delete_record, Record};
    let _ = connect(None).await.ok();
    let _: Option<Record<Person>> = delete_record(TEST_TABLE, TEST_PERSON).await.ok();
    close().await.ok();
    // End Test Setup

    // Initiate the connection using ENV Vars
    let john = person_factory(None, "John", 32).ok_or("Failed to create John");
    let john = match john {
        Ok(john) => john,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };
    // Create the Record
    let saved_john = john.save().await?;

    // Some Logging
    println!("Created -> Yes");
    println!("({}:{}) -> {:?}", TEST_TABLE, TEST_PERSON, saved_john);

    // Delete the Record
    let _old_john = saved_john.unwrap().into_inner().delete().await?;

    self::close().await.ok();
    Ok(())
}

#[tokio::test]
async fn test_basic_store() -> Result<(), Error> {
    use nico_surreal_client::prelude::*;
    // Initiate the connection using ENV Vars
    let _ = connect(None).await.ok();
    let _: Option<Record<Person>> = delete_record(TEST_TABLE, TEST_PERSON_TWO).await.ok();

    // Create the Record
    let created: Option<Record<Person>> =
        create_record::<Person>(TEST_TABLE, Some( TEST_PERSON_TWO ), person_factory(None, "John", 32)).await?;

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
