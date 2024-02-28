mod config;
mod error;
mod ident;
mod record;
mod storable;

use once_cell::sync::Lazy;
use surrealdb::{engine::any::Any, Response, Surreal};

pub use error::Error;
pub use ident::SurrealID;
pub use ident::{HasSurrealIdentifier, SurrealData, SurrealIDIdent, SurrealIDTable};
pub use record::Record;
pub use serde::{Deserialize, Serialize};
pub use storable::{DBThings, Storable};

static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);

static CONFIG: Lazy<config::DbConfig> = Lazy::new(config::setup);

pub mod prelude {
    pub use surrealdb::sql::Thing;

    pub use super::{
        connect,
        create_record,
        // update_record,
        delete_record,
        get_record,
        query,
        DBThings,
        Deserialize,
        Error,
        HasSurrealIdentifier,
        Record,
        Serialize,
        Storable,
        SurrealData,
        SurrealID,
        SurrealIDIdent,
        SurrealIDTable,
    };
}

pub async fn create_record<T>(record: Record<T>) -> Result<Vec<T>, Error>
where
    T: HasSurrealIdentifier + SurrealData + DBThings + From<Record<T>>,
{
    let created: Vec<T> = DB
        .create(record.table())
        .content(record.data::<T>())
        .await?;
    Ok(created)
}

pub async fn updata_record<'a, T>(
    table: &str,
    id: &str,
    data: Option<T>,
) -> Result<Option<Record<T>>, Error>
where
    T: HasSurrealIdentifier + SurrealData + DBThings,
{
    let updated: Option<Record<T>>;
    if let Some(data) = data {
        updated = DB.update((table, id)).content(data).await?;
    } else {
        updated = DB.update((table, id)).await?;
    }
    Ok(updated)
}

pub async fn get_record<T>(table: &str, id: &str) -> Result<Option<Record<T>>, Error>
where
    T: HasSurrealIdentifier + DBThings,
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

pub async fn delete_record<T>(table: &str, id: &str) -> Result<T, Error>
where
    T: HasSurrealIdentifier + DBThings,
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
