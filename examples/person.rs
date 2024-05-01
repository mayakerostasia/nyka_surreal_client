use nico_surreal_client::prelude::*;
use serde::{Deserialize, Serialize};
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

impl Storable<Person> for Person {
    fn thing(&self) -> Thing {
        Thing::from((self.table().unwrap(), self.id().unwrap()))
    }
    fn id(&self) -> Option<Id> {
        Some(Id::Number(1))
    }

    fn table(&self) -> Option<String> {
        Some(TEST_TABLE.to_string())
    }

    fn data(&self) -> Person {
        self.clone()
    }
}

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
    // Record To Database
    let john = person_factory(TEST_TABLE, Id::from(1), "John", 32).unwrap();
    println!("Record John: {:?}", &john);

    let _ = john.delete().await.await?;
    let saved_john = john.save().await.await?;
    let mut selected_john = john.select().await.await?;
    let john: Record<Person> = selected_john.take().expect("Couldn't select john");
    let mut _john = john.data();
    println!("{:#?}", _john);
    let updated_john = john.update().await.await?;
    let deleted_john = john.delete().await.await?;

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
    println!{
        "UpdatedJohn : ({}:{}) -> {:?}",
        TEST_TABLE, TEST_PERSON, updated_john
    };
    println!(
        "DeletedJohn : ({}:{}) -> {:?}",
        TEST_TABLE, TEST_PERSON, deleted_john
    );

    // We Succeeded so Ret 0
    Ok(())
}
