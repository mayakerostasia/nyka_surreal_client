// "⟩"

#[allow(unused_imports)]
use rs_nico_tracing::{ info, error, debug , instrument, Instrument };
use serde::{de, Deserializer};
use serde_json::{Map, Value as JValue};
use surrealdb::sql::{Id, Thing};

use crate::SurrealId;

pub fn deserialize_id<'de, D>(deserializer: D) -> Result<SurrealId, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = SurrealId;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            debug!("Here at expecting");
            formatter.write_str("well shit bitch... someone went and shit in my oven")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let str_val = value.parse::<String>().expect("In string");
            let thing = Thing::from(("_", str_val.as_str()));
            Ok(SurrealId(thing))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            debug!("Here at i64");
            let sid: SurrealId = SurrealId(Thing::from(("_", Id::from(value))));
            Ok(sid)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            debug!("Here at u64");
            let sid: SurrealId = SurrealId(Thing::from(("_", Id::from(value))));
            Ok(sid)
        }

        #[instrument(skip(self, map))]
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut table: Option<String> = None;
            let mut id: Option<Id> = None;

            while let Some((str, j_value)) = map.next_entry::<String, Map<String, JValue>>()? {
                debug!("Key: {:#?}, Value: {:#?}", str, j_value);
                let done = match str.as_ref() {
                    "tb" => {
                        debug!("TB -> Key: {:#?}, Value: {:#?}", str, j_value);
                        table = None;
                    }
                    "id" => {
                        debug!("ID -> Key: {:#?}, Value: {:#?}", str, j_value);
                        let entry = j_value.get("id");
                        if let Some(entry) = entry {
                            debug!("Attempting to deserialize: {:#?}", entry);
                            let _id = match entry {
                                JValue::Array(arr) => {
                                    debug!("Array: {:#?}", arr);
                                    // _id = Some(Id::Array(arr));
                                    unimplemented!("Array: {:#?}", arr);
                                }
                                JValue::Bool(boole) => {
                                    debug!("Bool: {:#?}", boole);
                                    unimplemented!("Bool: {:#?}", boole);
                                }
                                JValue::Number(num) => {
                                    // info!("Number: {:#?}", num);
                                    Id::Number(num.as_i64().expect("Failed to get i64 from number"))
                                }
                                JValue::Object(obj) => {
                                    debug!("Object: {:#?}", obj);
                                    // _id = Some(Id::Object(surrealdb::sql::Object(obj)))
                                    unimplemented!( "Object: {:#?}", obj );
                                }
                                JValue::String(str) => {
                                    // info!("String: {:#?}", str);
                                    Id::String(str.as_str().to_string())
                                }
                                JValue::Null => {
                                    debug!("Null: {:#?}", "Null");
                                    unimplemented!("Null: {:#?}", "Null");
                                }
                            };

                            id = Some(_id);
                        }
                    }
                    _ => {
                        debug!("Other -> Key: {:#?}, Value: {:#?}", str, j_value);
                        return Err(de::Error::custom("Unexpected key"));
                    }
                };
                println!("Done: {:#?}", done);
            };

            let thing = Thing {
                tb: table.expect("Failed to get table"),
                id: id.expect("Failed to get id"),
            };

            Ok(SurrealId(thing))

        }
    }
    deserializer.deserialize_any(Visitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    use once_cell::sync::Lazy;
    use serde_json;
    use serde::Serialize;
    use serde::Deserialize; 
    use crate::prelude::{SurrealId, Thing, Id, DBThings};
    use crate::{setup, Record, Storable};

    use lazy_static::lazy_static;
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

    lazy_static! { 
        static ref TEST_ID: [Thing;13] = [ 
            Thing {tb: "test".to_string(), id: Id::from("1".to_string())},
            Thing {tb: "test".to_string(), id: Id::from("2".to_string())},
            Thing {tb: "test".to_string(), id: Id::from("3".to_string())},
            Thing {tb: "test".to_string(), id: Id::from("4".to_string())},
            Thing {tb: "test".to_string(), id: Id::from("5".to_string())},
            Thing {tb: "test".to_string(), id: Id::from("6".to_string())},
            Thing {tb: "test".to_string(), id: Id::from("7".to_string())},
            Thing {tb: "test".to_string(), id: Id::from("8".to_string())},
            Thing {tb: "test".to_string(), id: Id::from("9".to_string())},
            Thing {tb: "test".to_string(), id: Id::from("10".to_string())},
            Thing {tb: "test".to_string(), id: Id::from("006367d3-1e51-47c5-8b56-43492cec95ee")},
            Thing {tb: "test".to_string(), id: Id::from("006367d3-1e51-47c5-8b56-43492cec95ee")},
            Thing {tb: "test".to_string(), id: Id::from("⟨006367d3-1e51-47c5-8b56-43492cec95ee⟩")},
        ];
    }

    #[tokio::test]
    async fn test_deserialize_id() -> Result<(), serde_json::Error> {
        // let otel = rs_nico_tracing::initialize();
        let cfg = setup();

        let mut test_iter = TEST_ID.iter();

        while let Some(id) = test_iter.next() {
            println!("Input as Thing: {:#?}", id);
            let thing_as_str = serde_json::to_string(id).unwrap();
            println!("Input as Str: {:#?}", thing_as_str);
            let old_id_str: Result<JValue, serde_json::Error> = serde_json::from_str(&thing_as_str);
            let deser_id: JValue = match old_id_str {
                Ok(id) => {
                    id
                },
                Err(e) => {
                    panic!("Failed to deserialize: {:#?}", e);
                },
                
            };
            // assert_eq!(*id, deser_id.0);
        };

        Ok(())
    }
}
