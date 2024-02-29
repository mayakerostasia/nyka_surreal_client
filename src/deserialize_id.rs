// use rs_autotask_api::debug;
use rs_nico_tracing::info;
use crate::SurrealID;
use serde::Deserializer;
use serde_json::{Map, Value as JValue};
use surrealdb::sql::{Id, Value};
// use std::iter;
// use crate::Id::Number;

pub fn deserialize_id<'de, D>(deserializer: D) -> Result<SurrealID, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = SurrealID;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i64 or a map")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let str_val = value.parse::<String>().expect("In string");
            // Check for `:` in the string
            // TODO:
            Ok(SurrealID::from(("default".to_string(), str_val)))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let sid: SurrealID = SurrealID::new("default", Some(Id::from(value as i64)));
            Ok(sid)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let sid: SurrealID = SurrealID::new("default", Some(Id::from(value as u64)));
            Ok(sid)
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let _table: Option<(String, String)> = map.next_entry()?;
            info!("Table from de {:?}", _table);

            let id: Option<(String, Map<String, JValue>)> = map.next_entry()?;
            info!("Id from de: {:?}", id);

            let mut _id: Option<Id> = None;
            match id {
                Some(opt) => { 
                    match opt {
                        (key, mut val) => {
                            info!("Key: {:#?}", key);
                            loop {
                                let entry = val.get("id");
                                if let Some(entry) = entry {
                                    match entry {
                                        JValue::Number(num) => {
                                            info!("Num: {:#?}", num);
                                            let _num = num.as_i64();
                                            if let Some(num) = _num {
                                                _id = Some(Id::from(num));
                                                break;
                                            }
                                        },
                                        JValue::String(s) => {
                                            info!("String: {:#?}", s);
                                            let _s = s.as_str();
                                            if let s = _s {
                                                _id = Some(Id::from(s));
                                                break;
                                            }
                                        },
                                        _ => info!("No id"),
                                    }
                                }
                            };
                            let sid: SurrealID = SurrealID::new(key.as_str(), Some(_id.expect("Heree")));
                            return Ok(sid)
                        },
                        None => info!("No id"),
                    }
                    // info!("Key: {:#?} \n Val: {:#?}" , key, map);
                    // panic!()
                },
                Err(e) => info!("No id"),
            }

            // todo!();

            let sid: SurrealID = SurrealID::new(_table.expect("Now down here").1.as_str(), _id);
            Ok(sid)
        }

    }

    deserializer.deserialize_any(Visitor)
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use serde_json::json;
    use surrealdb::sql::Object;
    use std::collections::BTreeMap;

    #[test]
    fn test_deserialize_id() {
        let json = "1";
        // Object::from(("id", "1"));
        let mut btree: BTreeMap<String, Value> = BTreeMap::new();
        btree.extend(iter::once(("id".to_string(), Value::from(Number(1)))));
        println!("{:?}", btree);
        let thingy: Value = serde_json::to_value(btree).unwrap();
        let id: SurrealID = serde_json::from_value(thingy).unwrap();
        println!("{:?}", id);
        // let id: SurrealID = serde_json::from_(json).unwrap();
        // assert_eq!(id.0.id, Id::String("1".to_string()));

        // let json = r#"1"#;
        // let id: SurrealID = serde_json::from_str(json).unwrap();
        // assert_eq!(id.0.id, Id::String("1".to_string()));

        // let json = r#"{"table": "default", "id": 1}"#;
        // let id: SurrealID = serde_json::from_str(json).unwrap();
        // assert_eq!(id.0.id, Id::String("1".to_string()));

        // let json = r#"{"table": "default", "id": "1"}"#;
        // let id: SurrealID = serde_json::from_str(json).unwrap();
        // assert_eq!(id.0.id, Id::String("1".to_string()));

        // let json = r#"{"table": "default", "id": 1, "name": "test"}"#;
        // let id: SurrealID = serde_json::from_str(json).unwrap();
        // assert_eq!(id.0.id, Id::String("1".to_string()));

        // let json = r#"{"table": "default", "id": "1", "name": "test"}"#;
        // let id: SurrealID = serde_json::from_str(json).unwrap();
        // assert_eq!(id.0.id, Id::String("1".to_string()));
    }
}
//     }
// }
// Compare this snippet from src/record_id_data.rs:
// use crate::ident::HasSurrealIdentifier;
// use crate::ident::SurrealData;
// use crate::prelude::*;
// use crate::Error;
// use async_trait::async_trait;
// use serde::{de::DeserializeOwned, Serialize};
// use std::fmt::Debug;
// use std::pin::Pin;
// 
// pub struct RecordIdData<T> {
//     pub id: SurrealID,
//     pub table: String,
//     pub data: Option<Box<T>>,
// }
// 
// impl<T: HasSurrealIdentifier> RecordIdData<T> {
//     pub fn new(tb: &str, id: Option<Id>, data: Box<T>) -> Self {
//         let id = SurrealID::new(tb, id);
//         Self