// use std::collections::BTreeMap;

// // use color_eyre::eyre::{Error as CError, Ok};
// use serde::{de, Deserializer};
// use surrealdb::sql::{Id, Thing, Value};

// #[allow(unused_imports)]
// use rs_nico_tracing::{ info, error, debug , instrument, Instrument };

// use surrealdb::sql::Object;
// use crate::{Error, SurrealId};

// // #[instrument(skip(deserializer, )]
// pub fn deserialize_id<'de, D>(deserializer: D) -> Result<SurrealId, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     struct Visitor;

//     impl<'de> serde::de::Visitor<'de> for Visitor {
//         type Value = SurrealId;

//         fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//             debug!("Here at expecting");
//             formatter.write_str("well shit bitch... someone went and shit in my oven")
//         }

//         // #[instrument(skip(map))]
//         // fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
//         // where
//         //     A: serde::de::MapAccess<'de>,
//         //     // A::Error: CError,
//         // {
//         //     let map = map.next_entry::<String, Value>()?;
//         //     match map {
//         //         Some((_, val)) => {
//         //             if let Value::Object(obj) = val.first() {
//         //                 println!("Deserd: {:#?}", obj);
//         //                 let id = obj.0.get("id").expect("Missing ID key");
//         //                 println!("Deserd in ID: {:#?}", id);
//         //                 if let thing_value = id.first() {
//         //                     println!("Deserd in ID ThingValue : {:#?}", thing_value);


//         //                     // let thing = Thing::try_from(id.to_string());
//         //                     // println!("Deserd in Thing: {:#?}", thing);
//         //                     // let id = SurrealId(thing.unwrap());
//         //                     println!("Deserd in ID 3 : {:#?}", id);
//         //                     todo!();
//         //                     // return Ok(id)
//         //                 } else {
//         //                     return Err(serde::de::Error::custom("Failed to get ID"))
//         //                 }
                        
//         //             } else {
//         //                 return Err(serde::de::Error::custom("Failed to get Object"))
//         //             }
//         //         },
//         //         None => {
//         //             return Err(serde::de::Error::custom("Got None!"))
//         //         }
//         //     }
//         // }
        
//         // fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
//         // where
//         //     E: serde::de::Error,
//         // {
//         //     let str_val = value.parse::<String>().expect("In string");
//         //     let thing = Thing::from(("_", str_val.as_str()));
//         //     Ok(SurrealId(thing))
//         // }

//         // fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
//         // where
//         //     E: serde::de::Error,
//         // {
//         //     debug!("Here at i64");
//         //     let sid: SurrealId = SurrealId(Thing::from(("_", Id::from(value))));
//         //     Ok(sid)
//         // }

//         // fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
//         // where
//         //     E: serde::de::Error,
//         // {
//         //     debug!("Here at u64");
//         //     let sid: SurrealId = SurrealId(Thing::from(("_", Id::from(value))));
//         //     Ok(sid)
//         // }

//     }
//     deserializer.deserialize_any(Visitor)
// }
