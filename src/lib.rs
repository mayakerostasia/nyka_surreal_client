mod config;
mod error;

use error::Error;

use core::fmt::Debug;
use once_cell::sync::Lazy;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use surrealdb::engine::any::Any;
use surrealdb::sql::{Table, Thing};
use surrealdb::Surreal;

static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);
static CONFIG: Lazy<config::DbConfig> = Lazy::new(config::setup);

type DBResult<T> = Result<T, Error>;

pub mod prelude {
    pub use super::connect;
    pub use super::create_record;
    pub use super::create_record_data;
    pub use super::create_table;
    pub use super::get_record;
    pub use super::delete_record;
    pub use super::Record;
    pub use super::error::Error;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ident {
    pub id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Record<T> {
    RecordId(Ident),
    Data(T),
    RecordIdData { id: Ident, value: T },
}

pub async fn create_table(name: &str) -> DBResult<Vec<Record<Table>>>
// where T: DeserializeOwned + Debug
{
    Ok(DB.create(name).await?)
}

pub async fn create_record<T>(resource: Thing, data: Option<T>) -> Result<Option<Record<T>>, Error>
where
    T: Serialize + Debug,
    T: DeserializeOwned + Debug,
{
    let mut creatable: Record<T>;
    if let Some(data) = data {
        creatable = Record::RecordIdData {
            id: Ident {
                id: resource.clone(),
            },
            value: data,
        };
    } else {
        creatable = Record::RecordId(Ident {
            id: resource.clone(),
        });
        // Ok(created)
    };
    let created: Option<Record<T>> = DB.create(resource).content(creatable).await?;
    // todo!();
    Ok(created)
}

pub async fn create_record_data<T>(resource: Thing, data: T) -> Result<Option<Record<T>>, Error>
where
    T: Serialize + Debug,
    T: DeserializeOwned + Debug,
{
    let created: Option<Record<T>> = DB.create(resource).content(data).await?;
    Ok(created)
}

pub async fn get_record<T>(table: &str, id: &str) -> Result<Vec<Record<T>>, Error>
where
    T: DeserializeOwned + Debug,
{
    println!("Getting record: {} {}", table, id);
    let this_thing = Ident {
        id: Thing::from((table, id)),
    };
    let value: Option<Record<T>> = DB.select(&this_thing.id).await?;
    match value {
        Some(Record::RecordIdData { id, value }) => {
            println!("Got record: {:?}", value);
            Ok(vec![Record::RecordIdData { id, value }])
        }
        Some(Record::Data(value)) => {
            println!("Got record: {:?}", value);
            Ok(vec![Record::Data(value)])
        }
        Some(Record::RecordId(id)) => {
            println!("Got record: {:?}", id);
            Ok(vec![Record::RecordId(id)])
        }
        None => {
            println!("No record found");
            Err(Error::NoRecordFound {
                namespace: CONFIG.namespace.clone(),
                database: CONFIG.database.clone(),
                table: table.to_string(),
                id: id.to_string(),
            })
        }
    }
    // Ok(vec![value.unwrap()])
}

pub async fn delete_record<T>(table: &str, id: &str) -> Result<Record<T>, Error>
where
    T: DeserializeOwned + Debug,
{
    let this_thing = Thing::from((table, id));
    let deleted: Option<T> = DB.delete(this_thing).await?;
    if let Some(deleted) = deleted {
        return Ok(Record::Data(deleted));
    } else {
        return Err(Error::NoRecordFound {
            namespace: CONFIG.namespace.clone(),
            database: CONFIG.database.clone(),
            table: table.to_string(),
            id: id.to_string(),
        });
    }
}

pub async fn connect(address: Option<&str>) -> Result<(), Error> {
    let health = DB.health().await;
    match health {
        Ok(_) => (),
        Err(_) => {
            let addr = match address {
                Some(addr) => addr,
                None => &CONFIG.surreal_path,
            };
            let _ = DB.connect(addr).await?;
            let _ = DB
                .use_ns(&CONFIG.namespace)
                .use_db(&CONFIG.database)
                .await?;
        }
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use surrealdb::sql::json;

    // #[tokio::test]
    // async fn test_create_table() -> Result<(), Error> {
    //     connect(None).await?;
    //     let table = "test_table";
    //     let created = create_table(table).await?;
    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_create_record() -> Result<(), Error> {
    //     connect(None).await?;
    //     let table = "test_table";
    //     let id = "test_id";
    //     let data = json(r#"{"data": "test"}"#).unwrap();
    //     let _created = create_record(Thing::from((table, id)), Some(data)).await?;
    //     let _deleted = delete_record(table, id).await?;
    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_get_record() -> Result<(), Error> {
    //     connect(None).await?;
    //     let table = "test_table";
    //     let id = "test_id";
    //     let record = get_record(table, id).await?;

    //     assert_eq!(record.unwrap().len(), 1);
    //     Ok(())
    // }
}
