use nico_surreal_client::{Ident, Storable};
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
    #[serde(skip_serializing)]
    id: Ident,
    name: String,
    age: u8,
}

impl<'a> Storable<'a> for Person {
    type Item = Self;

    fn table(&self) -> String {
        TEST_TABLE.to_string()
        // self.id.tb.to_string()
    }

    fn id(&self) -> String {
        TEST_PERSON.to_string()
        // self.id.id.to_raw()
    }
}

// API Call or Factory
fn person_factory(id: Option<Ident>, name: &str, age: u8) -> Option<Person> {
    if id.is_some() {
        Some(Person {
            id: id.unwrap(),
            name: name.to_string(),
            age: age,
        })
    } else {
        Some(Person {
            id: Ident::from((TEST_TABLE, TEST_PERSON)),
            name: name.to_string(),
            age: age,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), nico_surreal_client::Error> {
    // Record To Database
    let john = person_factory(Some(Ident::from(("test_table", Id::rand()))), "John", 32).unwrap();
    // let deleted_john = john.clone().delete().await?;
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
