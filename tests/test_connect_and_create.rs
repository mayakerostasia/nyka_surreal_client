// use nico_surreal_client::prelude::*;
// use nico_surreal_client::connect;
// use nico_surreal_client::{Error, Storable};
use nico_surreal_client::Error;
// use rs_nico_tracing::info;
// use serde::{Deserialize, Serialize}; 

// const TEST_TABLE: &str = "test_table";
// const TEST_PERSON: &str = "test_person";
// const TEST_PERSON_TWO: &str = "test_person_2";

// // Definition
// // #[derive(Debug, Deserialize, Serialize)]
// // struct Person {
// //     name: String,
// //     age: u8,
// // }

// // Definition
// #[allow(unused_attributes)]
// #[derive(Debug, Deserialize, Serialize, Clone)]
// struct Person {
//     #[serde(skip_serializing)]
//     id: String,
//     name: String,
//     age: u8,
// }

// impl StorableId for Person {
//     fn table(&self) -> String {
//         TEST_TABLE.to_string()
//     }

//     fn id(&self) -> String {
//         self.id.to_string()
//     }
// }

// impl Storable<'_, Person> for Record<Person> {}

// // API Call or Factory
// fn person_factory(id: &str, name: &str, age: u8) -> Person {
//     Person {
//         id: id.to_string(),
//         name: name.to_string(),
//         age: age,
//     }
// }

// // fn recordify<T>(id: &str, data: T) -> Record<T> {
// //     Record::RecordIdData( RecordIdData::new(TEST_TABLE, Some(id.into()), data))
// // }


#[tokio::main]
async fn main() -> Result<(), Error> {
//     // Some Test Setup
//     // let _otel = rs_nico_tracing::initialize().expect("Failed to start Telemetry");

//     info!("Starting Test");
//     let _ = connect(None).await.ok();

//     let rec = Record::new(
//         TEST_TABLE, 
//         TEST_PERSON, 
//         Some(person_factory(TEST_PERSON, "John", 32))
//     );
    
//     info!("Deleting Record");
//     let _: Option<Record<Person>> = delete_record(rec.clone()).await.ok();
//     // End Test Setup

//     info!("Creating Record");
//     // Create the Record
//     let saved_john = rec.clone().save().await?;

//     // Some Logging
//     println!("Created -> Yes");
//     println!("({}:{}) -> {:?}", TEST_TABLE, TEST_PERSON, &saved_john);

//     // // Delete the Record
//     let _old_john = Record::from(saved_john.expect(format!("Couldn't find {:?}", rec.clone()).as_str())).delete().await?;

//     nico_surreal_client::close().await.ok();
    Ok(())
}