use nico_surreal_client::{Record, Storable, StorableId};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};
// use builder_macro::Builder;

const TEST_TABLE: &str = "test_table";
const TEST_PERSON: &str = "test_person";

// TODO: To de or not to de
// fn deserialize_id<'de, D>(deserializer: D) -> Result<Thing, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let id = String::deserialize(deserializer)?;
//     println!("Deserialized ID: {:?}", id);
//     let id: Thing = id.parse().unwrap();
//     Ok(id)
// }

// Definition
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Person {
    // #[serde(skip_serializing)]
    id: u8,
    name: String,
    age: u8,
}

impl StorableId for Person {
    fn table(&self) -> String {
        TEST_TABLE.to_string()
        // self.id.tb.to_string()
    }

    fn id(&self) -> String {
        TEST_PERSON.to_string()
        // self.id.id.to_raw()
    }
}

impl<'a> Storable<'_, Person> for Record<Person> {}

// API Call or Factory
fn person_factory(id: u8, name: &str, age: u8) -> Option<Person> {
    Some(Person {
        id: id,
        name: name.to_string(),
        age: age,
    })
}

#[tokio::main]
async fn main() -> Result<(), nico_surreal_client::Error> {
    // Record To Database
    let john = person_factory( 1, "John", 32).unwrap();
    // let deleted_john = john.clone().delete().await?;
    let record_john = Record::from(john.clone());
    let saved_john = Record::from(john.clone()).save().await?;
    let selected_john = Record::from(john.clone()).select().await?;
    let deleted_john = Record::from(john.clone()).delete().await?;

    // Some Logging
    println!("Created -> Yes");
    println!(
        "SavedJohn : ({}:{}) -> {:?}",
        TEST_TABLE, TEST_PERSON, saved_john
    );
    println!(
        "SelectedJohn : ({}:{}) -> {:?}",
        TEST_TABLE, TEST_PERSON, selected_john
    );
    println!(
        "DeletedJohn : ({}:{}) -> {:?}",
        TEST_TABLE, TEST_PERSON, deleted_john
    );

    // We Succeeded so Ret 0
    Ok(())
}
