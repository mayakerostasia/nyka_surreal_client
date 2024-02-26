// pub use builder_macro::Builder;
mod config;
mod error;
mod ident;
mod storable;
// mod surreal_client;

use core::fmt::Debug;


use std::{borrow::Borrow, ops::{ Deref, DerefMut }};
use surrealdb::sql::Value;



pub use error::Error;
pub use ident::Ident;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
pub use storable::{ Storable, StorableId };
use surrealdb::opt::IntoResource;
pub use surrealdb::{ Response, sql::Id };
use surrealdb::{
    engine::any::Any,
    sql::{ Thing, Object },
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

impl<T: StorableId<T>> From<Object> for Record<T> {
    fn from(object: Object) -> Record<T> {
        match object {
            Object(map) => {
                let id = map.get("id");
                println!("{:?}", id);
                todo!();
                // let tb = map.get("tb").unwrap();
                // let data = map.get("data").unwrap();
                // let data = serde_json::from_value(Value::Object(data.clone())).unwrap();
                // Ok(Record::RecordIdData(RecordIdData::new(, Some(id.into()), data)))
            },
            _ => unimplemented!(),
        }
    }
}
impl<T> Record<T> {
    pub fn new(tb: &str, id: &str, data: Option<T>) -> Self {
        match data {
            Some(data) => Record::RecordIdData(RecordIdData::new(tb, Some(id.into()), data)),
            None => Record::RecordId(Ident::from(Thing::from((tb, id)))),
        }
    }
}
impl<T: StorableId<T>> StorableId<T> for Record<T> {
    type Item = T;
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
    fn data(&self) -> Self::Item {
        match &self {
            Record::RecordIdData(data) => data.data.clone().unwrap(),
            Record::RecordId(id) => panic!("RecordId: {:?}", id),
        }
    }
}

// impl<T: StorableId<T>> From<T> for Record<T> {
//     fn from(data: T) -> Self {
//         Record::new(data.table().as_str(), data.id().as_str(), Some(data))
//     }
// }
impl<T: StorableId<T>> Deref for Record<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match &self {
            Record::RecordIdData(data) => data.data.as_ref().unwrap(),
            Record::RecordId(id) => panic!("RecordId: {:?}", id),
        }
    }
}

// impl<T: StorableId<T>> DerefMut for Record<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         match self.data() {
//             data => data.clone().unwrap(),
//             _ => panic!("RecordId: {:?}", id),
//         }
//         // match &mut self {
//         //     Record::RecordIdData(_data) => &mut self,
//         //     Record::RecordId(id) => panic!("RecordId: {:?}", id),
//         // }
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

impl<T: StorableId<T>> StorableId<T> for RecordIdData<T> {
    type Item = Record<T>;
    fn table(&self) -> String {
        self.id.id.tb.clone()
    }
    fn id(&self) -> String {
        self.id.id.id.to_string()
    }
    fn data(&self) -> T {
        self.data.clone().unwrap()
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

impl<T: StorableId<T>> From<Resource> for Record<T> {
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

impl<T: StorableId<T>> IntoResource<Record<T>> for Record<T> {
    fn into_resource(self) -> Result<Resource, surrealdb::Error> {
        let thinggy = self.as_thing();
        Ok(thinggy.into())
    }
}

impl<T: StorableId<T>> From<Thing> for Record<T> {
    fn from(thing: Thing) -> Self {
        Record::new(&thing.tb, &thing.id.to_raw(), None)
    }
}

pub async fn create_record<T>(
    record: Record<T>,
) -> Result<Vec<T>, Error>
where
    T: StorableId<T>,
{
    // println!("Creating record: {} {} \n Data: {:#?}", table, id, data);
    let created: Vec<T> = DB.create(record.table()).content(record.data()).await?;
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

pub async fn get_record<T>(table: &str, id: &str) -> Result<Option<Record<T>>, Error>
where
    T: StorableId<T>,
{
    println!("Getting record: {:?}:{:?}", &table, &id);
    let value: Result<Option<Record<T>>, Error> = DB
        .select((table, id)) // Implement the IntoResource<T> trait for surrealdb::sql::Thing
        .await
        .map_err(|_e| Error::NoRecordFound {
            namespace: CONFIG.ns.to_string(),
            database: CONFIG.db.to_string(),
            table: table.to_string(),
            id: id.to_string(),
            // msg: e
        });
    println!("Got record: {:?}", &value);

    value
    // todo!();
}

pub async fn delete_record<T: StorableId<T>>(table: &str, id: &str) -> Result<T, Error>
{
    let deleted: Option<T> = DB.delete((table, id)).await?;
    if let Some(deleted) = deleted {
        return Ok(deleted);
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
