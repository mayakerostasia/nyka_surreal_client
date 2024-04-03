use color_eyre::owo_colors::OwoColorize;
#[allow(unused_imports)]
use nico_surreal_client::{
    Error,
    setup, Record, Storable, 
    query, create_record, delete_record, connect, 
    SurrealId, DBThings,
    prelude::{ Value, Id, Thing }
};

use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use surrealdb::sql::Object;
// use surrealdb::{opt::IntoResource, sql::Object};
// use surrealdb::opt::Resource;

struct SurIdent(Thing);
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Dummy {
    id: SurrealId,
    something: String
}

impl DBThings for Dummy {}
impl Storable<Dummy> for Dummy {
    fn thing(&self) -> Thing {
        Thing::from((self.id.0.tb.clone(), self.id.0.id.clone()))
    }

    fn id(&self) -> Option<Id> {
        Some(self.id.0.id.clone())
    }

    fn table(&self) -> Option<String> {
        let tb = self.id.0.tb.clone();
        if let "_" = tb.as_str() {
            return None
        } else {
            return Some(tb)
        }
    }

    fn data(&self) -> Dummy {
        self.clone()
    }
}


use std::collections::BTreeMap;
lazy_static! { 
    static ref TEST_ID_VALUE: [Value;8] = [ 
        wrap_value("test", "1"),
        wrap_value("test", 2),
        wrap_value("test", "two"),
        wrap_value("test", "one"),
        wrap_value("test", "`2`"),
        wrap_value("test", "2"),
        wrap_value("test", "006367d3-1e51-47c5-8b56-43492cec95ee"),
        wrap_value("test", "⟨006367d3-1e51-47c5-8b56-43492cec95ee⟩"),
    ];

    static ref TEST_ID_OBJ: [Object;8] = [ 
        build_object("test", "1"),
        build_object("test", 2),
        build_object("test", "two"),
        build_object("test", "one"),
        build_object("test", "`2`"),
        build_object("test", "2"),
        build_object("test", "006367d3-1e51-47c5-8b56-43492cec95ee"),
        build_object("test", "⟨006367d3-1e51-47c5-8b56-43492cec95ee⟩"),
    ];
    static ref TEST_ID_SIMPLE: [Thing;10] = [ 
        simple("test", Id::Number(1)),
        simple("test", Id::Number(2)),
        simple("test", Id::String("two".into())),
        simple("test", Id::String("one".to_string())),
        simple("test", Id::String("1".to_string())),
        simple("test", Id::String("2".to_string())),
        simple("test", Id::from("006367d3-1e51-47c5-8b56-43492cec95ee")),
        simple("test", Id::from(vec!["006367d3-1e51-47c5-8b56-43492cec95ee", "1"])),
        simple("test", Id::from("`006367d3-1e51-47c5-8b56-43492cec95ee`")),
        simple("test", Id::from("⟨006367d3-1e51-47c5-8b56-43492cec95ee⟩")),
        // simple("test", "⟨006367d3-1e51-47c5-8b56-43492cec95ee⟩"),
    ];
}

fn build_object<T>(tb: &str, id: T) -> Object where T: Into<Id> {
        Object::from(
            {
                let mut map = BTreeMap::new();
                map.insert("id".to_string(), Value::Thing(Thing::from((tb, Id::from(id.into())))));
                map
            }
        )
}

fn wrap_value<T>(tb: &str, id: T) -> Value where T: Into<Id> {
    Value::Object(build_object::<T>(tb, id))
}

fn simple<T>(tb: &str, id: T) -> Thing where T: Into<Id> {
    Thing::from((tb, Id::from(id.into())))
}

// #[tokio::test]
// async fn test_deser_from_value() -> Result<(), serde_json::Error> {
//     // let mut test_iter = TEST_ID_VALUE.iter();
//     // while let Some(id) = test_iter.next() {
//     //     // As Object
//     //     let obj_deser: Value = id.clone();
//     //     println!("Input as Object: {:#?}", obj_deser);

//     //     let obj_as_str: String = serde_json::to_string(&obj_deser).expect("whoops");
//     //     let obj_deser_surid: SurrealId = serde_json::from_str(&obj_as_str.clone()).expect("Failed to deserialize To SurId");
//     //     println!("Input as SurId: {:#?}", obj_deser_surid);
//     // };

//     Ok(())
// }

// #[tokio::test]
// async fn test_deser_from_object() -> Result<(), serde_json::Error> {
//     // let mut test_iter = TEST_ID_OBJ.iter();
//     // while let Some(id) = test_iter.next() {
//     //     // As Object
//     //     let obj_deser: Object = id.clone();
//     //     println!("Input as Object: {:#?}", obj_deser);
//     //     let obj_as_str: String = serde_json::to_string(&obj_deser).expect("whoops");
//     //     let obj_deser_surid: SurrealId = serde_json::from_str(&obj_as_str.clone()).expect("Failed to deserialize To SurId");
//     //     println!("Input as SurId: {:#?}", obj_deser_surid);
//     // };

//     Ok(())
// }


#[tokio::test]
async fn test_from_query() -> Result<(), serde_json::Error> {
    // let cfg = setup();
    // let _conn = connect(&cfg).await.expect("Failed to connect to db");
    // let mut resp = query("SELECT * from test:`1`").await.expect("Failed to query");
    // let deserd: Value = resp.take(0).expect("Couldn't take 0");
    // if let Value::Object(obj) = deserd.first() {
    //                     println!("Deserd: {:#?}", obj);
    //                     let id = obj.0.get("id").expect("Missing ID key");
    //                     println!("Deserd in ID: {:#?}", id);
    //                     if let thing_value = id.first() {
    //                         println!("Deserd in ID ThingValue : {:#?}", thing_value);
                            

    //                         // let thing = Thing::try_from(id.to_string());
    //                         // println!("Deserd in Thing: {:#?}", thing);
    //                         // let id = SurrealId(thing.unwrap());
    //                         println!("Deserd in ID 3 : {:#?}", id);
    //                         todo!();
    //                         // return Ok(id)
    //                     } else {
    //                         return Err(serde::de::Error::custom("Failed to get ID"))
    //                     }
                        
    //     // let id = obj.0.get("id");
    //     // println!("Deserd in ID: {:#?}", id);
    // //     if let Some(id) = id {
    // //         println!("Deserd in ID 2 : {:#?}", id);
    // //         let thing = Thing::try_from(id.to_string());
    // //         println!("Deserd in Thing: {:#?}", thing);
    // //         let id = SurrealId(thing.unwrap());

    // //         println!("Deserd in ID 3 : {:#?}", id);
    // //     }
    // } else {
    //     panic!("Failed to get Object");
    // }
    // println!("Deserd: {:#?}", deserd.first().is_object());


    Ok(())
}


#[tokio::test]
async fn test_from_storable() -> Result<(), serde_json::Error> {
    let iter_ids = TEST_ID_SIMPLE.iter();
    for thing in iter_ids {
        let dum = Dummy {
            id: SurrealId(thing.clone()),
            something: "Some data".to_string()
        };

        println!("Dummy: {:#?}", dum.bold());

        let saved: Result<Option<Dummy>, Error> = dum.save().await.await;
        println!("Saved: {:#?}", saved.green());
        let saved: Result<Option<Dummy>, Error> = dum.delete().await.await;
        println!("Deleted: {:#?}", saved.red());
    };

    Ok(())

}