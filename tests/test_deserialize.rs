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
use surrealdb::opt::Resource;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Dummy {
    id: SurrealId,
}

impl DBThings for Dummy {}
impl Storable<Dummy> for Dummy {}

impl From<Record<Dummy>> for Dummy {
    fn from(record: Record<Dummy>) -> Self {
        record.data().unwrap().clone()
    }
}

impl Into<Record<Dummy>> for Dummy {
    fn into(self) -> Record<Dummy> {
        Record::new(Some(self.id.0.tb.clone()), Some(self.id.0.id.clone()), Some(Box::new(self)))
    }
}

use std::collections::BTreeMap;
lazy_static! { 
    static ref TEST_ID: [Value;2] = [ 
        build_object("test", "1"),
        build_object("test", 2),
        // json!({"tb": {"Strand": "test"}, "id": {"Number": {"Strand": "1"}}}),
        // json!({"tb": {"Strand": "test"}, "id": {"Strand": {"Strand": "⟨006367d3-1e51-47c5-8b56-43492cec95ee⟩"}}}),
        // json!({"tb": {"Strand": "test"}, "id": {"Strand": {"Strand": "006367d3-1e51-47c5-8b56-43492cec95ee"}}}),
    ];

}

fn build_object<T>(tb: &str, id: T) -> Value where T: Into<Id> {
    Value::Object(
        Object::from(
            {
                let mut map = BTreeMap::new();
                map.insert("id".to_string(), Value::Thing(Thing::from((tb, Id::from(id.into())))));
                map
            }
        )
    )
}

#[tokio::test]
async fn main() -> Result<(), serde_json::Error> {
    // let otel = rs_nico_tracing::initialize().expect("Failed to start Telemetry");
    let cfg = setup();
    let _conn = connect(&cfg).await.expect("Failed to connect to db");

    let mut query_resp = query("SELECT * from test:`1`").await.expect("Failed to query");
    let resp: Result<Value, surrealdb::Error> = query_resp.take(0);
    match resp {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {:#?}", e);
            return Err(serde::de::Error::custom("Failed to get response"));
        }
        
    }
    // println!("Test: {:#?}", &resp.clone().unwrap());

    if let Value::Object(obj) = &resp.unwrap().first() {
        println!("Test Ok: {:#?}", obj);
    } else {
        // println!("Test else: {:#?}", &obj);
        panic!("Failed to get Object");
    }



    let mut test_iter = TEST_ID.iter();

    while let Some(id) = test_iter.next() {
        // As Object
        let obj_deser: Value = id.clone();
        println!("Input as Object: {:#?}", obj_deser);

        let obj_as_str: String = serde_json::to_string(&obj_deser).expect("whoops");
        println!("Input as String: {:#?}", obj_as_str);
        let obj_resource: Value = serde_json::from_str(&obj_as_str).expect("Failed to deserialize");

        let id: Result<Option<SurrealId>, Error> = match obj_resource {
            Value::Object(obj) => {
                println!("Test Ok: {:#?}", obj);
                // let (_, tb) = obj.0.get_key_value("tb");
                // let tb = tb.clone().as_string();
                let btree = obj.0;

                // let val: Thing = serde_json::from_value(value)
                let id = if let Some((key, val)) = btree.iter().next() {
                    println!("Key: {:#?}", key);
                    println!("Val: {:#?}", val);
                    match val {
                        Value::Thing(thing) => {
                            println!("Test Ok: {:#?}", thing);
                            Some(SurrealId(Thing::from(thing.clone())))
                        },
                        Value::Object(obj) => {
                            println!("Test Ok: {:#?}", obj);
                            let id = obj.get("id").unwrap();
                            let tb = obj.get("tb").unwrap();
                            Some(SurrealId(Thing::from((tb.to_string(), id.to_string()))))
                        },
                        Value::Strand(id) => {
                            println!("Test Ok: {:#?}", id);
                            let id = SurrealId(Thing::from(("_".to_string(), id.to_string())));
                            println!("Test Ok: {:#?}", id);
                            Some(SurrealId(Thing::from(("_".to_string(), id.to_string()))))
                            
                        },
                        _ => None
                    }
                } else {
                    None
                };
                Ok(id)
            },
            _ => {
                println!("Test else: {:#?}", &obj_resource);
                Err(Error::SerdeError(serde::de::Error::custom("Failed to parse key 3")))
            }
        };
        // println!("Input as Value: {:#?}", obj_resource);
        // let obj_deser_thing: Thing = serde_json::from_str(&obj_as_str).expect("Failed to deserialize to Thing");
        // let obj_deser_surid: SurrealId = serde_json::from_str(&obj_as_str).expect("Failed to deserialize To SurId");

        // println!("Input as Thing: {:#?}", obj_deser_thing);
        // println!("Input as SurId: {:#?}", obj_deser_surid);

    };

    Ok(())
}