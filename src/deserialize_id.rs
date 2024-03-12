use serde::{de, Deserializer};
use surrealdb::sql::{Id, Thing, Value};

#[allow(unused_imports)]
use rs_nico_tracing::{ info, error, debug , instrument, Instrument };

use crate::SurrealId;

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
            if let Some((ref key, val)) = map.next_entry::<String, Value>()? {
                if let Value::Object(obj) = &val.first() {
                    println!("Test Ok: {:#?}", obj);
                    let id = obj.get("id").unwrap();
                    match id.first() {
                        Value::Thing(thing) => {
                            println!("Test Ok: {:#?}", thing);
                            Err(serde::de::Error::custom(format!("Failed to parse key: {:#?}", thing)))
                        },
                        Value::Object(obj) => {
                            println!("Test Ok: {:#?}", obj);
                            let id = obj.get("id").unwrap();
                            let tb = obj.get("tb").unwrap();
                            match id.first() {
                                Value::Strand(id) => {
                                    println!("Test Ok: {:#?}", id);
                                    let id = SurrealId(Thing::from((tb.to_string(), id.to_string())));
                                    Ok(id)
                                },
                                _ => Err(serde::de::Error::custom("Failed to parse key"))
                            }
                        },
                        _ => Err(serde::de::Error::custom("Failed to parse key"))
                    }
                } else {
                    Err(serde::de::Error::custom("Failed to parse key"))
                }
            } else {
                Err(serde::de::Error::custom("No entry found"))
            }
        }
    }
    deserializer.deserialize_any(Visitor)
}
