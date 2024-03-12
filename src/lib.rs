mod config;
mod creds;
mod deserialize_id;
mod error;
mod ident;
mod record;
mod storable;

pub use config::{setup, DbConfig};
// pub use deserialize_id::deserialize_id;
pub use error::Error;
pub use ident::SurrealId;
use once_cell::sync::Lazy;
pub use record::Record;
pub use serde::{Deserialize, Serialize};
pub use storable::{DBThings, Storable};
use surrealdb::opt::auth::Root;
use surrealdb::{engine::any::Any, opt::auth::Jwt, Response, Surreal};

static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);

// static CONFIG: Lazy<config::DbConfig> = Lazy::new(config::setup);

pub mod prelude {
    pub use surrealdb::sql::Id;
    pub use surrealdb::sql::Thing;
    pub use surrealdb::sql::Value;
    pub use surrealdb::Error as SDBError;

    pub use super::{
        connect,
        create_record,
        // update_record,
        delete_record,
        // deserialize_id,
        get_record,
        query,
        DBThings,
        Deserialize,
        Error,
        // HasSurrealIdentifier,
        Record,
        Serialize,
        Storable,
        // SurrealData,
        SurrealId,
        // SurrealIDIdent,
        // SurrealIDTable,
    };
}

pub async fn create_record<T>(record: Record<T>) -> Result<Vec<T>, Error>
where
    T: DBThings
{
    let _id = &record.id;
    let data = record.data();

    match data {
        Some(data) => {
            let created: Vec<T> = DB.create(_id.tb.clone()).content(data).await?;
            return Ok(created);
        }
        None => {
            // id create
            let created: Option<T> = DB.create(_id).await?;
            return Ok(vec![created.unwrap()]);
        }
        
    };
}

pub async fn updata_record<'a, T>(
    record: Record<T>,
) -> Result<Option<Record<T>>, Error>
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

pub async fn get_record<T>(record: Record<T>) -> Result<Option<Record<T>>, Error>
// pub async fn get_record<T>(table: &str, id: Id) -> Result<Option<Record<T>>, Error>
where
    T: DBThings,
{
    let ret: Option<Record<T>>;
    let _id = &record.id;

    DB.select(_id) // Implement the IntoResource<T> trait for surrealdb::sql::Thing
        .await
        .map_err(|_e| Error::NoRecordFound {
            id: _id.to_string(),
            id_raw: _id.to_raw(), // msg: e
            table: record.table(),
        })
}

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

pub async fn query(query: &str) -> Result<Response, Error> {
    let results: Response = DB.query(query).await?;
    Ok(results)
}

pub async fn connect<'a>(config: &'a config::DbConfig) -> Result<(), Error> {
    DB.connect(&config.path).await?;
    let _result = DB
        .signin(Root {
            username: &config.user,
            password: &config.secret,
        })
        .await?;

    DB.use_ns(&config.ns).use_db(&config.db).await?;
    Ok(())
}

struct DBGuard {
    _token: Jwt,
}
impl DBGuard {
    fn new(token: Jwt) -> Self {
        Self { _token: token }
    }

    fn token(self) -> Jwt {
        self._token
    }
}

// impl Drop for DBGuard {
//     fn drop(&mut self) {
//         // let _ = DB.invalidate();
//     }
// }

pub async fn close() -> Result<(), Error> {
    DB.invalidate().await?;
    Ok(())
}
