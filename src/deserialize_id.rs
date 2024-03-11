// "⟩"

#[allow(unused_imports)]
use rs_nico_tracing::{ info, error, debug , instrument, Instrument };
// use log::{ info, error, debug , instrument, Instrument };
use serde::{de, Deserializer};
use std::collections::HashMap;
// use serde_json::{Value as JValue};
use surrealdb::sql::{Id, Object, Table, Thing, Value};

use crate::SurrealId;

struct DeserMap<K,V>(K, V);

// #[instrument(skip(deserializer, )]
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

        // #[instrument(skip(map))]
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let sid: SurrealId = if let Some((str, j_value)) = map.next_entry::<String, Value>()? {
                error!("Key: {:#?}, Value: {:#?}", str, j_value);
                SurrealId(serde_json::from_value::<Thing>(j_value.into_json()).expect("Couldn't reconstruct Thing"))
            } else {
                panic!("Failed to get next entry");
            };

            Ok(sid)
        }
    }
    deserializer.deserialize_any(Visitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;
    use serde::Serialize;
    use serde::Deserialize;
    use crate::prelude::{SurrealId, Thing, Id, DBThings};
    use crate::{setup, Record, Storable, create_record, delete_record, connect};

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

    use std::collections::HashMap;
    lazy_static! { 
        static ref TEST_ID: [Thing;14] = [ 
            Thing::from(("test", Id::from(1))),
            Thing::from(("test", Id::from("1"))),
            Thing::from(("test", Id::from(13030))),
            Thing::from(("test", Id::from("1"))),
            Thing::from(("test", Id::from("1"))),
            Thing::from(("test", Id::from("1"))),
            Thing::from(("test", Id::from("1"))),
            Thing::from(("test", Id::from("1"))),
            Thing::from(("test", Id::from("1"))),
            Thing::from(("test", Id::from("1"))),
            Thing::from(("test", Id::from("1"))),
            Thing::from(("test", Id::from("⟨006367d3-1e51-47c5-8b56-43492cec95ee⟩"))),
            Thing::from(("test", Id::from("006367d3-1e51-47c5-8b56-43492cec95ee"))),
            Thing::from(("test", Id::from("006367d3-1e51-47c5-8b56-43492cec95ee"))),
        ];
    }

    #[tokio::test]
    async fn test_deserialize_id() -> Result<(), serde_json::Error> {
        // let otel = rs_nico_tracing::initialize().expect("Failed to start tracing");
        let cfg = setup();
        let conn = connect(&cfg).await.expect("Failed to connect to db");

        let mut test_iter = TEST_ID.iter();

        while let Some(id) = test_iter.next() {
            let obj_as_str = serde_json::to_string(id).expect("Failed to serialize");
            // let obj_deser: Object = serde_json::from_str(&obj_as_str).expect("Failed to deserialize");
            let obj_deser_value: Thing = serde_json::from_str(&obj_as_str).expect("Failed to deserialize");
            println!("Input as Object: {:#?}", id);
            println!("Input as SerObj: {:#?}", obj_as_str);
            // println!("Input as DeserObj: {:#?}", obj_deser);
            println!("Input as DeserObj: {:#?}", obj_deser_value);
            let obj_deser_surid: SurrealId = serde_json::from_str(&obj_as_str).expect("Failed to deserialize");
            println!("Input as DeserObj: {:#?}", obj_deser_surid);

            assert_eq!(obj_as_str, serde_json::to_string(&obj_deser_surid.0).expect("whoops"));
        };

        Ok(())
    }
}
