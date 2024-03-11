use std::fmt::Debug;
use std::pin::Pin;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::DbConfig;
use crate::ident::HasSurrealIdentifier;
use crate::ident::SurrealData;
use crate::prelude::*;
use futures_lite::Future;
use crate::Error;

pub trait DBThings: Debug + Serialize + DeserializeOwned + Sized + Clone {}

#[async_trait]
pub trait Storable<T>
where
    Self: Into<Record<T>> + DBThings,
    T: DBThings + From<Record<T>> + 'static,
{
    async fn save(&self, config: &DbConfig) -> Pin<Box< dyn Future< Output = Result<Vec<T>, Error>> >> {
        // todo!();
        let _ = connect(config).await.ok();
        let record: Record<T> = <Self as Into<Record<T>>>::into(self.clone());
        Box::pin(async move { create_record(record).await })
    }

    async fn select(&self, config: &DbConfig) -> Pin<Box< dyn Future< Output = Result<Option<Record<T>>, Error>>  >> {
        let _ = connect(config).await.ok();
        let record: Record<T> = <Self as Into<Record<T>>>::into(self.clone());
        Box::pin(async move { get_record(&record.table(), record.id()).await } )
    }

    async fn delete(&self, config: &DbConfig) -> Pin<Box<dyn Future<Output = Result< Option<Record<T>>, Error>> >> {
        let _ = connect(config).await.ok();
        let record: Record<T> = <Self as Into<Record<T>>>::into(self.clone());
        // let rec =  
        Box::pin(async move { delete_record(&record.table(), record.id()).await })
    }
}

// impl<T: Storable<T>> From<T> for Record<T> {
//     fn from(storable: T) -> Self {
//         todo!("Storable::From<T> for Record<T>");
//         // // let id = storable.id(false);
//         // // let table = storable.table(false);
//         // // let data = storable.data();
//         // let record: Record<T> = Record::new(table.as_str(), Some(id), Some(Box::new(data)));
//         // record
//     }
// }
