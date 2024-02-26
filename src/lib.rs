// pub use builder_macro::Builder;
mod config;
mod error;
mod ident;
mod storable;
// mod surreal_client;

use core::fmt::Debug;
use std::f32::consts::E;
use std::ops::DerefMut;
use std::{collections::BTreeMap, ops::Deref};
use std::future::IntoFuture;
use std::fmt::Display;

pub use error::Error;
pub use ident::Ident;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use storable::{ Storable, StorableId };
use surrealdb::opt::IntoResource;
pub use surrealdb::{ Response, sql::Id };
use surrealdb::{
    engine::any::Any,
    sql::{Thing, Value },
    Surreal,
    opt::Resource,
};

static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);
static CONFIG: Lazy<config::DbConfig> = Lazy::new(config::setup);

pub mod prelude {
    pub use surrealdb::sql::Thing;

    pub use super::{
        connect,
        create_record,
        //    update_record,
        delete_record,
        get_record,
        query,
        Error,
        Ident,
        Record,
        Storable,
    };
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Record<T> {
    RecordIdData(RecordIdData<T>),
    RecordId(Ident),
    // Records(Vec<T>),
    // RecordIdData {
    //     id: Ident,
    //     data: BTreeMap<String, Value>,
    // },
    // RecordId(Ident),
}

impl<T> Record<T> {
    pub fn new(tb: &str, id: &str, data: Option<T>) -> Self {
        match data {
            Some(data) => Record::RecordIdData(RecordIdData::new(tb, Some(id.into()), data)),
            None => Record::RecordId(Ident::from(Thing::from((tb, id)))),
        }
    }
}
impl<T: StorableId> StorableId for Record<T> {
    fn table(&self) -> String {
        match &self {
            Record::RecordIdData(data) => data.table(),
            Record::RecordId(id) => id.id.tb.clone(),
        }
    }
    fn id(&self) -> String {
        match &self {
            Record::RecordIdData(data) => data.id(),
            Record::RecordId(id) => id.id.id.to_string(),
        }
    }
    fn as_record<A>(&self) -> Record<Record<T>>
    where
        T: StorableId,
    {
        let clone = self.clone();
        match &self {
            Record::RecordIdData(data) => Record::from(clone),
            Record::RecordId(id) => Record::RecordId(id.clone()),
        }
    }
    fn as_thing(&self) -> Thing {
        match &self {
            Record::RecordIdData(data) => data.as_thing(),
            Record::RecordId(id) => unimplemented!(),
        }
    }
}
impl<T: StorableId> From<T> for Record<T> {
    fn from(data: T) -> Self {
        Record::new(data.table().as_str(), data.id().as_str(), Some(data))
    }
}
impl<T: StorableId> Deref for Record<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match &self {
            Record::RecordIdData(data) => self,
            Record::RecordId(id) => panic!("RecordId: {:?}", id),
        }
    }
}

// impl<T: StorableId> DerefMut for Record<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         match &self {
//             Record::RecordIdData(data) => (&mut self.data.as_mut()).unwrap(),
//             Record::RecordId(id) => panic!("RecordId: {:?}", id),
//         }
//     }
// }


#[derive(Debug, Serialize, Deserialize, Clone)] 
pub struct RecordIdData<T> {
    pub id: Ident,
    pub data: Option<T>,
}

impl<T> RecordIdData<T> {
    fn _new(id: Ident, data: T) -> Self {
        RecordIdData { id, data: Some(data) }
    }

    pub fn new(tb: &str, id: Option<Id>, data: T) -> Self {
        match id {
            Some(id) => RecordIdData {
                id: Ident::from(Thing::from((tb, id.to_raw().as_str()))),
                data: Some(data),
            },
            None => RecordIdData {
                id: Ident::from(Thing::from((tb, Id::rand().to_raw().as_str()))),
                data: Some(data),
            },
        }
    }

    pub fn new_dataless(tb: &str, id: Option<Id>) -> Self {
        match id {
            Some(id) => RecordIdData {
                id: Ident::from(Thing::from((tb, id.to_raw().as_str()))),
                data: None
            },
            None => RecordIdData {
                id: Ident::from(Thing::from((tb, Id::rand().to_raw().as_str()))),
                data: None,
            },
        }
    }

    pub fn into_inner(self) -> Option<T> {
        self.data
    }

    pub fn into_inner_mut(&mut self) -> &mut Option<T> {
        &mut self.data
    }
}

impl<T: StorableId> StorableId for RecordIdData<T> {
    fn table(&self) -> String {
        self.id.id.tb.clone()
    }
    fn id(&self) -> String {
        self.id.id.id.to_string()
    }
}


// #[async_trait]
// impl<T> Storable<'_, Record<T>> for Record<T>
// where T: StorableId + Debug + Serialize + DeserializeOwned + Sized + Clone
// {
//     type Item = Record<T>;
// }

impl<T> Record<T> {
    pub fn into_inner(self) -> Option<T> {
        match self {
            Record::RecordIdData(data) => data.data,
            _ => unimplemented!(),
        }
    }

    pub fn into_inner_mut(&mut self) -> &mut Option<T> {
        match self {
            Record::RecordIdData(data) => &mut data.data,
            _ => unimplemented!(),
        }
    }
}

impl<T: StorableId> From<Resource> for Record<T> {
	fn from(resource: Resource) -> Self {
		match resource {
			// Resource::Table(resource) => resource.into(),
			Resource::RecordId(resource) => resource.into(),
			// Resource::Object(resource) => resource.into(),
			// Resource::Array(resource) => resource.into(),
			// Resource::Edges(resource) => resource.into(),
            _ => unimplemented!(),
		}
	}
}

impl<T: StorableId> IntoResource<Record<T>> for Record<T> {
    fn into_resource(self) -> Result<Resource, surrealdb::Error> {
        let thinggy = self.as_thing();
        Ok(thinggy.into())
    }
}

impl<T: StorableId> From<Thing> for Record<T> {
    fn from(thing: Thing) -> Self {
        Record::new(&thing.tb, &thing.id.to_raw(), None)
    }
}

pub async fn create_record<T>(
    record: Record<T>,
) -> Result<Option<T>, Error>
where
    T: StorableId,
{
    // println!("Creating record: {} {} \n Data: {:#?}", table, id, data);
    let created: Option<T> = DB.create((record.table(), record.id())).content(&record).await?;
    Ok(created)
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

pub async fn get_record<T>(record: Record<T>) -> Result<Option<Record<T>>, Error>
where
    T: StorableId,
{
    let value: Result<Option<Record<T>>, Error> = DB
        .select(record.as_thing()) // Implement the IntoResource<T> trait for surrealdb::sql::Thing
        .await
        .map_err(|_e| Error::NoRecordFound {
            namespace: CONFIG.ns.to_string(),
            database: CONFIG.db.to_string(),
            table: record.table(),
            id: record.id(),
            // msg: e
        });
    println!("Got record: {:?}", &value);

    // Ok(value)
    todo!();
}

pub async fn delete_record<T: StorableId>(record: Record<T>) -> Result<Record<T>, Error>
{
    let deleted: Option<T> = DB.delete(record.as_thing()).await?;
    if let Some(deleted) = deleted {
        return Ok(deleted.into());
    } else {
        return Err(Error::NoRecordFound {
            namespace: CONFIG.ns.clone(),
            database: CONFIG.db.clone(),
            table: record.table(),
            id: record.id(),
            // msg: Err(surrealdb::err::Error::NoRecordFound).expect_err(msg),
        });
    }
}

pub async fn query(query: &str) -> Result<Response, Error> {
    let results: Response = DB.query(query).await?;
    Ok(results)
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
