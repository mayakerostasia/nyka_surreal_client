mod config;
mod creds;
mod deserialize_id;
mod error;
mod ident;
mod record;
mod storable;

// use creds::Credentials;
pub use deserialize_id::deserialize_id;
pub use error::Error;
pub use ident::SurrealID;
pub use ident::{HasSurrealIdentifier, SurrealData, SurrealIDIdent, SurrealIDTable};
use once_cell::sync::Lazy;
pub use record::Record;
pub use serde::{Deserialize, Serialize};
pub use storable::{DBThings, Storable};
use surrealdb::opt::auth::Root;
use surrealdb::{
    engine::any::Any,
    opt::auth::{Database, Jwt},
    sql::Id,
    Response, Surreal,
};

static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);

static CONFIG: Lazy<config::DbConfig> = Lazy::new(config::setup);

pub mod prelude {
    pub use surrealdb::sql::Id;
    pub use surrealdb::sql::Thing;
    pub use surrealdb::sql::Value;
    pub use surrealdb::Error as SDBError;

    pub use super::{
        config::{ setup, DbConfig },
        check_connect,
        connect,
        create_record,
        // update_record,
        delete_record,
        deserialize_id,
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

pub async fn get_record<T>(table: &str, id: Id) -> Result<Option<Record<T>>, Error>
where
    T: HasSurrealIdentifier + DBThings,
{
    let _id = id.clone();
    println!("Getting record: {:?}:{:?}", &table, &_id);
    println!("Getting record: {:?}:{:?}", &table, &_id.to_raw());
    println!("Getting record: {:?}:{:?}", &table, &_id.to_string());
    println!(
        "Getting record: {:?}:{:?}",
        &table,
        &_id.to_raw().to_string()
    );

    let value: Result<Option<Record<T>>, Error> = DB
        .select((table, _id.clone())) // Implement the IntoResource<T> trait for surrealdb::sql::Thing
        .await
        .map_err(|_e| Error::NoRecordFound {
            namespace: CONFIG.ns.to_string(),
            database: CONFIG.db.to_string(),
            table: table.to_string(),
            id: id.to_string(),
            id_raw: id.to_raw(), // msg: e
        });
    println!("Got record: {:?}", &value);

    value
    // todo!();
}

pub async fn delete_record<T>(table: &str, id: Id) -> Result<T, Error>
where
    T: HasSurrealIdentifier + DBThings,
{
    let deleted: Option<T> = DB.delete((table, id.to_raw())).await?;
    if let Some(deleted) = deleted {
        Ok(deleted)
    } else {
        Err(Error::NoRecordFound {
            namespace: CONFIG.ns.clone(),
            database: CONFIG.db.clone(),
            table: table.to_string(),
            id: id.to_string(),
            id_raw: id.to_raw(),
            // msg: Err(surrealdb::err::Error::NoRecordFound).expect_err(msg),
        })
    }
}

pub async fn query(query: &str) -> Result<Response, Error> {
    let results: Response = DB.query(query).await?;
    Ok(results)
}

pub async fn connect<'a>(config: config::DbConfig) -> Result<(), Error> {
    DB.connect(&config.path).await?;
    // let guard = DBGuard::new(
    //     DB.signin(Database {
    //         namespace: &config.ns,
    //         database: &config.db,
    //         username: &config.user,
    //         password: &config.secret,
    //     })
    //     .await?,
    // );

    // DB.authenticate(guard.token()).await?;

    DB.use_ns(&CONFIG.ns).use_db(&CONFIG.db).await?;
    Ok(())
}

pub async fn check_connect<'a>() -> Result<(), Error> {
    let health = DB.health().await;
    match health {
        // Ok(_) => (),
        // Err(_) => {
        _ => {
            DB.connect(&CONFIG.path).await?;
            DB.signin(Root {
                username: &CONFIG.user,
                password: &CONFIG.secret,
            })
            .await?;
            DB.use_ns(&CONFIG.ns).use_db(&CONFIG.db).await?;
        }
    };
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
