use rs_nico_tracing::info;
use serde::Deserializer;
use serde_json::{Map, Value as JValue};
use surrealdb::sql::Id;

use crate::SurrealID;

pub fn deserialize_id<'de, D>(deserializer: D) -> Result<SurrealID, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = SurrealID;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("well shit bitch... someone went and shit in my oven")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            println!("Here at string");
            let str_val = value.parse::<String>().expect("In string");
            // Check for `:` in the string
            // TODO:
            Ok(SurrealID::from(("default".to_string(), str_val)))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            println!("Here at i64");
            let sid: SurrealID = SurrealID::new("default", Some(Id::from(value)));
            Ok(sid)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            println!("Here at u64");
            let sid: SurrealID = SurrealID::new("default", Some(Id::from(value)));
            Ok(sid)
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let _table: Option<(String, String)> = map.next_entry()?;
            info!("Table from de {:?}", _table);

            let id: Option<(String, Map<String, JValue>)> = map.next_entry()?;

            let mut _id: Option<Id> = None;
            match id {
                Some((key, value)) => {
                    info!("Key: {:#?}", key);
                    loop {
                        let entry = value.get("id");
                        if let Some(entry) = entry {
                            match entry {
                                JValue::Array(arr) => {
                                    info!("Array: {:#?}", arr);
                                    unimplemented!();
                                    // _id = Some(Id::Array(arr));
                                }
                                JValue::Bool(boole) => {
                                    info!("Bool: {:#?}", boole);
                                    unimplemented!();
                                }
                                JValue::Number(num) => {
                                    info!("Number: {:#?}", num);
                                    _id = Some(Id::Number(num.as_i64().expect("Here")));
                                }
                                JValue::Object(obj) => {
                                    info!("Object: {:#?}", obj);
                                    unimplemented!()
                                }
                                JValue::String(str) => {
                                    info!("String: {:#?}", str);
                                    _id = Some(Id::String(str.as_str().to_string()));
                                }
                                JValue::Null => {
                                    info!("Null: {:#?}", "Null");
                                    unimplemented!();
                                }
                            }
                        } else {
                            info!("No id");
                            break;
                        }
                    }
                    let sid: SurrealID = SurrealID::new("default", _id);
                    Ok(sid)
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
        let json = "1";
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
