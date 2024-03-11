// "⟩"

#[allow(unused_imports)]
use rs_nico_tracing::{ info, error, debug , instrument, Instrument };
use serde::Deserializer;
use serde_json::{Map, Value as JValue};
use surrealdb::sql::{Id, Thing};

use crate::SurrealId;

#[instrument(skip(deserializer))]
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

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let _table: Option<(String, String)> = map.next_entry()?;

            let table = if let Some((key, value)) = _table {
                Some(value)
            } else {
                None
            };

            let id: Option<(String, Map<String, JValue>)> = map.next_entry()?;

            match id {
                Some((_, value)) => {
                    let entry = value.get("id");
                    if let Some(entry) = entry {
                        debug!("Attempting to deserialize: {:#?}", entry);
                        let id = match entry {
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

                        let thing = Thing::from((table.expect(format!("What! Table wasn't set? Here's the id : ({})", &id).as_str()), id));
                        let sid = SurrealId(thing);
                        Ok(sid)

                    } else {
                        Err(serde::de::Error::custom("No id"))
                    }

                }
                None => Err(serde::de::Error::custom("No id")),
            }
        }
    }
    deserializer.deserialize_any(Visitor)
}

#[cfg(test)]
mod tests {
    // use super::*;
    use std::collections::{BTreeMap, HashMap};
    use std::iter;

    use serde_json;
    use surrealdb::sql::{Object, Value};

    #[test]
    fn test_deserialize_id() -> Result<(), serde_json::Error> {
        let hm: HashMap<String, Value> =
            HashMap::from([("id".to_string(), Value::from("1".to_string()))]);
        let obj = Object::from(hm);
        let mut btree: BTreeMap<String, Value> = BTreeMap::new();
        btree.extend(iter::once(("id".to_string(), Value::Object(obj))));
        println!("BTree Is {:?}", btree);
        let thingy: Value = Value::Object(Object(btree));
        println!("Thingy is {:?}", thingy);
        // let as_sdb_value: SurrealID = Thing::from(btree);
        // println!("SDB_val = {:?}", as_sdb_value);
        // let sid = SurrealID(Thing::from(as_sdb_value));
        // let _sid = match sid {

        // }
        // let id: SurrealID = serde_json::from_value(thingy);
        // println!("{:?}", sid);
        // let id: SurrealID = serde_json::from_value(thingy).unwrap();
        // println!("{:?}", id);
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
        Ok(())
    }
}
