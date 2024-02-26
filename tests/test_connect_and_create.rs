use nico_surreal_client::{Error, Id, Record, RecordIdData, Storable, StorableId};
use serde::{Deserialize, Serialize}; 

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
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Person {
    id: String,
    name: String,
    age: u8,
}

impl StorableId for Person {
    fn table(&self) -> String {
        TEST_TABLE.to_string()
    }

    fn id(&self) -> String {
        self.id.to_string()
    }
}

impl Storable<'_, Person> for Record<Person> {}

// API Call or Factory
fn person_factory(id: &str, name: &str, age: u8) -> Person {
    Person {
        id: id.to_string(),
        name: name.to_string(),
        age: age,
    }
}

fn recordify<T>(id: &str, data: T) -> Record<T> {
    Record::RecordIdData( RecordIdData::new(TEST_TABLE, Some(id.into()), data))
}

#[tokio::test]
async fn test_storable() -> Result<(), Error> {
    // Some Test Setup
    use nico_surreal_client::connect;
    use nico_surreal_client::prelude::{delete_record, Record};
    let _ = connect(None).await.ok();

    let rec = Record::RecordIdData(RecordIdData::new(
        TEST_TABLE, 
        Some(Id::from(TEST_PERSON)), 
        person_factory(TEST_PERSON, "John", 32)
    ));
    let _: Option<Record<Person>> = delete_record(rec).await.ok();
    // End Test Setup

    // Initiate the connection using ENV Vars
    let john = recordify::<Person>(
            TEST_TABLE,
            person_factory(
                TEST_PERSON, 
                "John", 
                32
            ));

    // Create the Record
    let saved_john = john.clone().save().await?;

    // Some Logging
    println!("Created -> Yes");
    println!("({}:{}) -> {:?}", TEST_TABLE, TEST_PERSON, saved_john);

    // // Delete the Record
    let _old_john = Record::from(saved_john.expect(format!("Couldn't find {:?}", john).as_str())).delete().await?;

    // self::close().await.ok();
    Ok(())
}

#[tokio::test]
async fn test_basic_store() -> Result<(), Error> {
    // use nico_surreal_client::prelude::*;
    // // Initiate the connection using ENV Vars
    // let _ = connect(None).await.ok();
    // let _: Option<Record<Person>> = delete_record(TEST_TABLE, TEST_PERSON_TWO).await.ok();

    // // Create the Record
    // let created: Option<Record<Person>> = create_record::<Person>(
    //     TEST_TABLE,
    //     Some(TEST_PERSON_TWO),
    //     person_factory(None, "John", 32),
    // )
    // .await?;

    // // Check if we got a record
    // let record: Option<Record<Thing>> = get_record(TEST_TABLE, TEST_PERSON_TWO).await?;

    // // Update the Record
    // // let updated: Option<Record<Person>> = update_record(TEST_TABLE, TEST_PERSON_TWO, person_factory("John", 33)).await?;

    // // Cleaning up
    // let _: Option<Record<Person>> = delete_record(TEST_TABLE, TEST_PERSON_TWO).await.ok();

    // // Some Logging
    // println!("Created -> {:?}", created);
    // println!("Record -> {:?}", record);

    // // We Succeeded so Ret 0
    // close().await.ok();
    Ok(())
}
