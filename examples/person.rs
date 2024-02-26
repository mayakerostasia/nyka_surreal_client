// use std::ops::Deref;

// use serde::de::value::MapDeserializer;
// use serde::de::MapAccess;
use std::collections::BTreeMap;
use std::ops::Deref;
use serde_with::serde_as;
use nico_surreal_client::{Record, Storable, StorableId, prelude::Thing};
use serde::{Deserialize, Serialize};
use surrealdb::opt::Resource;
use surrealdb::sql::Value;

// use builder_macro::Builder;

const TEST_TABLE: &str = "test_table";
const TEST_PERSON: &str = "test_person";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonId(Thing);

impl Deref for PersonId {
    type Target = Thing;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Definition
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Person {
    id: PersonId,
    name: String,
    age: u8,
}

impl StorableId<Person> for Person {
    type Item = Person;

    
    fn table(&self) -> String {
        (*(self.id)).tb.clone()
    }

    fn id(&self) -> String {
        (*(self.id)).id.clone().to_raw()
    }

    fn data(&self) -> Self::Item {
        self.clone()
    }
}

impl From<Person> for Record<Person> {
    fn from(person: Person) -> Record<Person> {
        Record::new(TEST_TABLE, TEST_PERSON, Some(person))
    }
}

impl<'a> Storable<'_, Person> for Record<Person> {}


// API Call or Factory
fn person_factory(table: &str, id: &str, name: &str, age: u8) -> Option<Person> {
    Some(Person {
        id: PersonId(Thing::from((table, id))),
        name: name.to_string(),
        age: age,
    })
}

#[tokio::main]
async fn main() -> Result<(), nico_surreal_client::Error> {
    // Record To Database
    let john = person_factory(TEST_TABLE, "one", "John", 32).unwrap();
    let _record_john = Record::from(john.clone());
    println!("Record John: {:?}", _record_john);
    _record_john.delete().await;

    let save_john = Record::from(john.clone());
    let saved_john = save_john.save().await;
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
