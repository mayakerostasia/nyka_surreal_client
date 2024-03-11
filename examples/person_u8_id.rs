/// This is PROBLEMATIC !!!
use nico_surreal_client::{DbConfig, setup};
use nico_surreal_client::prelude::*;
use surrealdb::sql::Id;

const TEST_TABLE: &str = "test_table";
const TEST_PERSON: &str = "test_person";

// Definition
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Person {
    id: SurrealID,
    /// surrealdb::sql::Id
    // table: String,
    name: String,
    age: u8,
}
impl Into<Record<Person>> for Person {
    fn into(self) -> Record<Person> {
        Record::from(self.into())
    }
}
impl Storable<Person> for Person {}
// impl SurrealIDIdent for Person {
// }
// impl SurrealIDTable for Person {
// }
impl DBThings for Person {}
impl HasSurrealIdentifier for Person {
    fn id(&self) -> Id {
        self.id.id()
    }
    fn table(&self) -> String {
        self.id.table().to_string()
    }
}

// impl SurrealData for Person {}

impl From<Record<Person>> for Person {
    fn from(record: Record<Person>) -> Self {
        let id = record.id();
        println!("ID: {:?}", &id);
        let data = record.into_inner().unwrap();
        data
    }
}

// API Call or Factory
fn person_factory(table: &str, id: Id, name: &str, age: u8) -> Option<Person> {
    Some(Person {
        id: SurrealID::Thing(Thing::from((table, id))),
        name: name.to_string(),
        age: age,
    })
}

#[tokio::main]
async fn main() -> Result<(), nico_surreal_client::Error> {
    let conf = setup();
    // Record To Database
    let john = person_factory(TEST_TABLE, Id::from(1), "John", 32).unwrap();
    println!("Record John: {:?}", &john);

    let _ = john.delete(&conf).await;
    let saved_john = john.save(&conf).await.await?;
    let selected_john = john.select(&conf).await.await?;
    let deleted_john = john.delete(&conf).await.await?;

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
