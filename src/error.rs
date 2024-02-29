// use std::sync::{PoisonError, RwLockReadGuard};
use thiserror::Error;

// use crate::client::DBClient;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Please set the \n DB_PATH, \n DB_NS, DB_DB, \n DB_USER, and \n DB_SECRET \n environment variables")]
    EnvError,

    #[error("DB No Record Found: \n NS: {namespace:#?} \n DB: {database:#?} \n Table: {table:#?} \n ID: {id:#?} \n")]
    NoRecordFound {
        namespace: String,
        database: String,
        table: String,
        id: String,
        id_raw: String,
        // msg: surrealdb::Error
    },

    #[error("Surreal Error: {0}")]
    DbError(#[from] surrealdb::Error),

    #[error("DB Error: {0:?}")]
    SurrealAPIError(#[from] surrealdb::error::Api),

    #[error("Surreal Db Error: {0}")]
    SurrealError(#[from] surrealdb::error::Db),

    #[error("Service Error: {0}")]
    ServiceError(String),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Serde Error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Poison Error: {0}")]
    PoisonError(String),

    #[error("No Data stored in Record! {0}")]
    NoDataStored(String),
}

// impl From<PoisonError<RwLockReadGuard<'_, DBClient>>> for Error {
//     fn from(e: PoisonError<RwLockReadGuard<'_, DBClient>>) -> Self {
//         Error::PoisonError(e.to_string())
//     }
// }
// `fn from(_: PoisonError<RwLockReadGuard<'_, DBClient>>) -> Self { todo!() }`:
// `fn from(_: PoisonError<RwLockReadGuard<'_, DBClient>>) -> Self { todo!() }
// `
