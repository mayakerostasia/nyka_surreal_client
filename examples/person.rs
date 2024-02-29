// use nico_surreal_client::ident::{HasSurrealIdentifier, SurrealData};
// use nico_surreal_client::{DBThings, SurrealID};
// use nico_surreal_client::{Record, Storable, prelude::Thing,
//     ident::{
//         SurrealIDTable,
//         SurrealIDIdent,
//         SurrealIDFactory,
// }};
// use serde::{Deserialize, Serialize};

use nico_surreal_client::prelude::*;

const TEST_TABLE: &str = "test_table";
const TEST_PERSON: &str = "test_person";

// Definition
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Person {
    id: SurrealID,
    name: String,
    age: u8,
}
impl Storable for Person {}
impl SurrealIDIdent for Person {
    fn id(&self) -> Id {
        self.id.id()
    }
}
impl SurrealIDTable for Person {
    fn table(&self) -> String {
        self.id.table()
    }
}
impl DBThings for Person {}
impl HasSurrealIdentifier for Person {}
impl SurrealData for Person {}
impl From<Record<Person>> for Person {
    fn from(record: Record<Person>) -> Self {
        let data = record.into_inner().unwrap();
        data
    }
}

// API Call or Factory
fn person_factory(table: &str, id: &str, name: &str, age: u8) -> Option<Person> {
    Some(Person {
        id: SurrealID(Thing::from((table, id))),
        name: name.to_string(),
        age: age,
    })
}

#[tokio::main]
async fn main() -> Result<(), nico_surreal_client::Error> {
    // Record To Database
    let john = person_factory(TEST_TABLE, "one", "John", 32).unwrap();
    println!("Record John: {:?}", &john);
    let _ = &john.delete().await;

    // let save_john = Record::from(john.clone());
    let saved_john = (&john).clone().save().await;
    let selected_john = &john.select().await?;
    let deleted_john = &john.delete().await?;

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
