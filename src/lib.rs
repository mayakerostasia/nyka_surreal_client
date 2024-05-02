//! # SurrealDB Client for BlueBastion
//!
//! This crate is a client for the SurrealDB database.
//! If you use this crait to implement the [Storable] trait for your struct,
//!     you can store and retrieve your struct from the SurrealDB database.
//!
//! ## Example
//! ```rust
//! // Define your object that you want to store in the database
//! // We recommend avoiding fields called "id" and "table" in your struct
//! // You can use serde::skip() to skip these fields in serialization
//! #[derive(Debug, Deserialize, Serialize, Clone)]
//! struct Person {
//!     name: String,
//!     age: u8,
//! }
//!
//! impl DBThings for Person {}
//!
//! impl Storable<Person> for Person {
//!     fn thing(&self) -> Thing {
//!         Thing::from((self.table().unwrap(), self.id().unwrap()))
//!     }
//!
//!     fn id(&self) -> Option<Id> {
//!         // We recommend using `Id::from([your_id])`
//!         // in your implementation
//!         Some( Id::from(1) )
//!     }
//!     
//!     fn table(&self) -> Option<String> {
//!         Some( "some_table".to_string() )
//!     }
//!     
//!     fn data(&self) -> Person {
//!         self.clone()
//!     }
//! }
//! ```
mod config;
mod creds;
mod deserialize_id;
mod error;
mod ident;
mod live;
mod record;
mod storable;

pub use config::{setup, DbConfig};
pub use error::Error;
pub use ident::SurrealId;
use once_cell::sync::Lazy;
pub use record::Record;
pub use storable::{DBThings, Storable};

use surrealdb::{
    engine::any::Any,
    opt::{auth::Root, PatchOp},
    sql::Thing,
    Response, Surreal,
};

static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);

// static CONFIG: Lazy<config::DbConfig> = Lazy::new(config::setup);

pub mod prelude {
    pub use surrealdb::sql::Id;
    pub use surrealdb::sql::Thing;
    pub use surrealdb::sql::Value;
    pub use surrealdb::Error as SDBError;

    pub use super::live::subscribe;
    // live_select
    pub use super::live_select;
    pub use super::{
        connect,
        create_record,
        delete_record,
        update_record,
        // deserialize_id,
        get_record,
        query,
        DBThings,
        Error,
        Record,
        Storable,
        SurrealId,
    };
}

/// needs `connect` to be called first
///
///
/// # Examples
///
pub async fn create_record<T>(record: Record<T>) -> Result<Option<T>, Error>
where
    T: DBThings,
{
    let _id = &record.id;
    let data = record.data();

    match data {
        Some(data) => {
            let created: Option<T> = DB
                .create((_id.tb.clone(), _id.id.clone()))
                .content(data)
                .await?;
            Ok(created)
        }
        None => {
            // id create
            let created: Option<T> = DB.create(_id).await?;
            Ok(created)
        }
    }
}

/// Static function to update a record
/// This function requires you to call the `connect` function before calling
pub async fn update_record<'a, T>(record: Record<T>) -> Result<Option<Record<T>>, Error>
where
    T: DBThings,
{
    let updated: Option<Record<T>>;
    let _id = &record.id;

    if let Some(data) = record.data() {
        updated = DB.update(_id).content(data).await?;
    } else {
        updated = DB.update(_id).await?;
    }
    Ok(updated)
}

/// Static function to get a record
/// This function requires you to call the `connect` function before calling
pub async fn get_record<T>(record: Record<T>) -> Result<Option<Record<T>>, Error>
// pub async fn get_record<T>(table: &str, id: Id) -> Result<Option<Record<T>>, Error>
where
    T: DBThings,
{
    let _ret: Option<Record<T>>;
    let _id = &record.id;

    DB.select(_id) // Implement the IntoResource<T> trait for surrealdb::sql::Thing
        .await
        .map_err(|_e| Error::NoRecordFound {
            id: _id.to_string(),
            id_raw: _id.to_raw(), // msg: e
            table: record.table(),
        })
}

/// Static function to delete a record
/// This function requires you to call the `connect` function before calling
pub async fn delete_record<T>(record: Record<T>) -> Result<Option<T>, Error>
// pub async fn delete_record<T>(table: &str, id: Id) -> Result<Option<T>, Error>
where
    T: DBThings,
{
    let table = record.table();

    if table == "_" {
        return Err(Error::TableNameUnset);
    }

    let id = record.id();

    Ok(DB.delete((table, id)).await?)
}

/// Static function to update a record
/// This function is used automatically in the `Storable` trait
pub async fn patch_record<T>(record: Record<T>, patch: PatchOp) -> Result<Option<T>, Error> 
where T: DBThings 
{
    let table = record.table();
    if table == "_" {
        return Err(Error::TableNameUnset);
    }

    let id = record.id();

    let ret = DB.update((table.clone(), id.clone())).patch(patch).await
        .map_err(|_e| Error::UpdateFailed {
            id: id.to_string(),
            id_raw: id.to_raw(),
            table,
        })?;

    Ok(ret)
}

/// Static function to query the database
/// This function takes in a SurrealQL query string
/// e.g. `SELECT * FROM table;`
/// This function requires you to call the `connect` function before calling
pub async fn query(query: &str) -> Result<Response, Error> {
    let results: Response = DB.query(query).await?;
    Ok(results)
}


/// Static function to connect to the database
/// This function is used automatically in the `Storable` trait
pub async fn connect<'a>(config: &'a config::DbConfig) -> Result<(), Error> {
    DB.connect(&config.path).await?;
    let _result = DB
        .signin(Root {
            username: &config.user,
            password: &config.pass,
        })
        .await?;

    DB.use_ns(&config.ns).use_db(&config.db).await?;
    Ok(())
}

/// Static function to start a live select stream
/// This function requires you to call the `connect` function before calling
/// Unimplemented
pub async fn live_select<'a, T>(
    table: &str,
    id: Option<Thing>,
) -> Result<surrealdb::method::Stream<'a, Any, Vec<T>>, Error>
where
    T: DBThings,
{
    match id {
        Some(_) => {
            unimplemented!()
        }
        None => {
            let stream: surrealdb::method::Stream<'_, Any, Vec<T>> =
                DB.select(table).live().await?;
            Ok(stream)
        }
    }
}

// DBGuard Implementation
// /// Currently Unimplemented
// struct DBGuard(Jwt);
// impl DBGuard {
//     fn new(token: Jwt) -> Self {
//         Self(token)
//     }

//     fn token(self) -> Jwt {
//         self.0
//     }
// }

// impl Drop for DBGuard {
//     fn drop(&mut self) {
//         // let _ = DB.invalidate();
//     }
// }

pub async fn close() -> Result<(), Error> {
    DB.invalidate().await?;
    Ok(())
}
