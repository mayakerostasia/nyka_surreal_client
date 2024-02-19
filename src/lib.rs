mod config;
mod error;

use error::Error;
use surrealdb::method::Select;
use surrealdb::opt::Resource;

use core::fmt::Debug;
use std::collections::BTreeMap;
use std::future::IntoFuture;
use once_cell::sync::Lazy;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use surrealdb::engine::any::Any;
use surrealdb::sql::{Object, Table, Thing};
use surrealdb::Surreal;

static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);
static CONFIG: Lazy<config::DbConfig> = Lazy::new(config::setup);

// use serde_json::Value;

type DBResult<T> = Result<T, Error>;

pub mod prelude {
    pub use super::connect;
    pub use super::create_record;
    // pub use super::create_record_data;
    // pub use super::create_table;
    pub use super::delete_record;
    pub use super::error::Error;
    pub use super::get_record;
    pub use super::Record;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ident {
    pub id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Record<T> {
    Data(T),
    Records(Vec<T>),
    RecordIdData { id: Ident, data: surrealdb::sql::Value },
    RecordId(Ident),
}

impl<T> Into<Record<T>> for Resource {
    fn into(self) -> Record<T> {
        match self {
            Resource::Object(object) => Record::RecordIdData{ 
                id : Ident { id: Thing::from((
                    object.0.get("id").unwrap().to_string(), 
                    object.0.get("tb").unwrap().to_string()
                ))},
                data: object.0.clone().into()
            },
            _ => unimplemented!()
        }
    }
}


// pub async fn create_table(name: &str) -> DBResult<Vec<Record<Table>>>
// // where T: DeserializeOwned + Debug
// {
//     Ok(DB.create(Table(name.to_string())).await?)
// }

pub async fn create_record<T>(table: &str, id: &str, data: Option<T>) -> Result<Option<Record<T>>, Error>
where
    T: Serialize + Debug,
    T: DeserializeOwned + Debug,
{
    println!("Creating record: {} {} \n Data: {:#?}", table, id, data);
    let created: Option<Record<T>>;
    if let Some(data) = data {
        println!("Creating record with data: {:#?}", data);
        created = DB.create((table, id)).content(data).await?;
        println!("{:?}", &created.as_ref().unwrap());
    } else {
        created = DB.create((table, id)).await?;
    }
    Ok(created)
}

// pub async fn create_record_data<T>(resource: Thing, data: T) -> Result<Option<Record<T>>, Error>
// where
//     T: Serialize + Debug,
//     T: DeserializeOwned + Debug,
// {
//     let created: Option<Record<T>> = DB.create(resource).content(data).await?;
//     Ok(created)
// }

pub async fn get_record<'a, T>(table: &str, id: &str) -> Result<Option<T>, Error>
where
    T: DeserializeOwned + Debug,
{
    println!("Getting record: {} {}", table, id);
    let this_thing = Ident {
        id: Thing::from((table, id)),
    };
    let value: Option<T> = DB.select((&this_thing.id.tb, &this_thing.id.id.to_raw()))
        .into_future()
        .await
        .map_err(|e| 
            Error::NoRecordFound { 
                namespace: CONFIG.ns.to_string(), 
                database: CONFIG.db.to_string(), 
                table: table.to_string(), 
                id: id.to_string() ,
                // msg: e
        })?;
    println!("Got record: {:?}", &value);

    Ok(value.into())
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
            let _db_ns = DB
                .use_ns(&CONFIG.ns)
                .use_db(&CONFIG.db)
                .await?;
        }
    };
    Ok(())
}