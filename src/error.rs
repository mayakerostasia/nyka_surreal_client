// use std::sync::{PoisonError, RwLockReadGuard};
use thiserror::Error;

// use crate::client::DBClient;

/// Error type for the SurrealDB Client
#[derive(Error, Debug)]
pub enum Error {
    #[error("Please set the \n DB_PATH, \n DB_NS, DB_DB, \n DB_USER, and \n DB_SECRET \n environment variables")]
    EnvError,

    #[error("DB No Record Found: \n Table: {table:#?} \n ID: {id:#?} \n")]
    NoRecordFound {
        table: String,
        id: String,
        id_raw: String,
        // msg: surrealdb::Error
    },

    #[error("Deserialization Error: {0}")]
    DeserializationError(String),

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

    #[error("Table name cannot be unset")]
    TableNameUnset,
}

// impl From<PoisonError<RwLockReadGuard<'_, DBClient>>> for Error {
//     fn from(e: PoisonError<RwLockReadGuard<'_, DBClient>>) -> Self {
//         Error::PoisonError(e.to_string())
//     }
// }
// `fn from(_: PoisonError<RwLockReadGuard<'_, DBClient>>) -> Self { todo!() }`:
// `fn from(_: PoisonError<RwLockReadGuard<'_, DBClient>>) -> Self { todo!() }
// `
