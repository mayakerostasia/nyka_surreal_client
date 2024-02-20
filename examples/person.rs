use nico_surreal_client::Storable;
use serde::{Deserialize, Serialize};

const TEST_TABLE: &str = "test_table";
const TEST_PERSON: &str = "test_person";

// Definition
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Person<'a> {
    #[serde(skip_serializing, skip_deserializing)]
    id: (&'a str, &'a str),
    name: String,
    age: u8,
}

impl<'a> Storable<'a> for Person<'a> {
    type Item = Self;

    fn table(&self) -> &'a str {
        self.id.0
    }

    fn id(&self) -> &'a str {
        self.id.1
    }
}

// API Call or Factory
fn person_factory(name: &str, age: u8) -> Option<Person> {
    Some(Person {
        id: (TEST_TABLE, TEST_PERSON),    
        name: name.to_string(),
        age,
    })
}

#[tokio::main]
async fn main() -> Result<(), nico_surreal_client::Error> {
    // Record To Database
    let john = person_factory("John", 32).unwrap();
    let saved_john = john.clone().save().await?;
    let selected_john = john.clone().select().await?;
    let deleted_john = john.clone().delete().await?;

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
