pub use builder_macro::Builder;
mod surreal_client;
mod config;
mod error;
mod storable;
mod ident;

pub use error::Error;
pub use storable::Storable;
use surrealdb::sql::{/* Ident */ Value};

use core::fmt::Debug;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::future::IntoFuture;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

// use crate::id::Ident;
// pub mod ident;

pub use ident::Ident;
// use surreal_client::*;

static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);
static CONFIG: Lazy<config::DbConfig> = Lazy::new(config::setup);

pub mod prelude {
    pub use super::connect;
    pub use super::create_record;
    pub use super::get_record;
    // pub use super::update_record;
    pub use super::delete_record;
    pub use super::Record;
    pub use surrealdb::sql::Thing;
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Record<T> {
    RecordIdDataT {
        id: Ident,
        data: T,
    },
    Record(T),
    Records(Vec<T>),
    RecordIdData {
        id: Ident,
        data: BTreeMap<String, Value>,
    },
    RecordId(Ident),
}

impl<T> Record<T> {
    pub fn into_inner(self) -> T {
        match self {
            Record::Record(data) => data,
            _ => unimplemented!(),
        }
    }
}

pub async fn create_record<'a, T>(
    table: &str,
    id: Option<&str>,
    data: Option<T>,
) -> Result<Option<Record<T>>, Error>
where
    T: Serialize + Debug,
    T: DeserializeOwned + Debug,
    T: Storable<'a>,
{
    // println!("Creating record: {} {} \n Data: {:#?}", table, id, data);
    let created: Option<Record<T>>;
    
    if let Some(data) = data {
        created = DB.create((table, id.unwrap())).content(data).await?;
    } else {
        created = DB.create((table, id.unwrap())).await?;
    }
    Ok(created.into())
}

// pub async fn update_record<T>(table: &str, id: &str, data: Option<T>) -> Result<Option<Record<T>>, Error>
// where
//     T: Serialize + Debug,
//     T: DeserializeOwned + Debug,
// {
//     let updated: Option<Record<T>>;
//     if let Some(data) = data {
//         updated = DB.update((table, id)).content(data).await?;
//     } else {
//         updated = DB.update((table, id)).await?;
//     }
//     Ok(updated)
// }

pub async fn get_record<T>(table: &str, id: &str) -> Result<Option<T>, Error>
where
    T: DeserializeOwned + Debug,
{
    // println!("Getting record: {} {}", table, id);
    let this_thing = Ident {
        // id: id.to_string(),
        id: Thing::from((table, id)),
    };
    let value: Option<T> = DB
        .select((&this_thing.id.tb, &this_thing.id.id.to_raw()))
        .into_future()
        .await
        .map_err(|_e| Error::NoRecordFound {
            namespace: CONFIG.ns.to_string(),
            database: CONFIG.db.to_string(),
            table: table.to_string(),
            id: id.to_string(),
            // msg: e
        })?;
    // println!("Got record: {:?}", &value);

    Ok(value.into())
}

pub async fn delete_record<T>(table: &str, id: &str) -> Result<Record<T>, Error>
where
    T: DeserializeOwned + Debug,
{
    let this_thing = Thing::from((table, id));
    let deleted: Option<T> = DB.delete(this_thing).await?;
    if let Some(deleted) = deleted {
        return Ok(Record::Record(deleted));
    } else {
        return Err(Error::NoRecordFound {
            namespace: CONFIG.ns.clone(),
            database: CONFIG.db.clone(),
            table: table.to_string(),
            id: id.to_string(),
            // msg: Err(surrealdb::err::Error::NoRecordFound).expect_err(msg),
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
                None => &CONFIG.path,
            };
            let _connect = DB.connect(addr).await?;
            let _db_ns = DB.use_ns(&CONFIG.ns).use_db(&CONFIG.db).await?;
        }
    };
    Ok(())
}

pub async fn close() -> Result<(), Error> {
    let _close = DB.invalidate().await?;
    Ok(())
}
