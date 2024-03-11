use nico_surreal_client::prelude::*;
/// This is PROBLEMATIC !!!
use nico_surreal_client::{setup, DbConfig};
use surrealdb::sql::Id;

const TEST_TABLE: &str = "test_table";
const TEST_PERSON: &str = "test_person";

// Definition
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Person {
    id: SurrealId,
    name: String,
    age: u8,
}
impl DBThings for Person {}

impl From<Record<Person>> for Person {
    fn from(record: Record<Person>) -> Self {
        let id = record.id();
        println!("ID: {:?}", &id);
        record.into_inner().unwrap()
    }
}

impl Into<Record<Person>> for Person {
    fn into(self) -> Record<Person> {
        Record::new(
            Some(TEST_TABLE.to_string()), 
            Some(Id::from(1)), 
            Some(Box::new(self.clone())))
    }
}

impl Storable<Person> for Person {}

// API Call or Factory
fn person_factory(table: &str, id: Id, name: &str, age: u8) -> Option<Person> {
    Some(Person {
        id: SurrealId(Thing::from((table, id))),
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

    // let _ = john.delete(&conf).await.await?;
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
