// use rs_autotask_api::debug;
use serde::Deserializer;
use serde_json::Map;
use serde_json::Value;

pub fn deserialize_id<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = i64;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i64 or a map")
        }
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.parse().unwrap())
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value as i64)
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let _table: Option<(String, String)> = map.next_entry()?;
            let id: (String, Map<String, Value>) = map.next_entry()?.unwrap();

            let _id = id.1.get("Number").unwrap();
            Ok(_id.as_i64().unwrap())
        }
    }

    deserializer.deserialize_any(Visitor)
}
