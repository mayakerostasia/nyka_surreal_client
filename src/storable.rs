use std::fmt::Debug;
use std::pin::Pin;

use async_trait::async_trait;
use futures_lite::Future;
use serde::{de::DeserializeOwned, Serialize};

use crate::prelude::*;
use crate::DbConfig;
use crate::Error;

pub trait DBThings: Debug + Serialize + DeserializeOwned + Sized + Clone {}

#[async_trait]
pub trait Storable<T>
where
    Self: Into<Record<T>> + DBThings,
    T: DBThings + From<Record<T>> + 'static,
{
    async fn save(
        &self,
        config: &DbConfig,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<T>, Error>>>> {
        // todo!();
        let _ = connect(config).await.ok();
        let record: Record<T> = <Self as Into<Record<T>>>::into(self.clone());
        Box::pin(create_record(record))
    }

    async fn select(
        &self,
        config: &DbConfig,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Record<T>>, Error>>>> {
        let _ = connect(config).await.ok();
        let record: Record<T> = <Self as Into<Record<T>>>::into(self.clone());
        Box::pin(get_record(record))
    }

    async fn delete(
        &self,
        config: &DbConfig,
    ) -> Pin<Box<dyn Future<Output = Result<Option<T>, Error>>>> {
        let _ = connect(config).await.ok();
        let record: Record<T> = <Self as Into<Record<T>>>::into(self.clone());
        Box::pin(delete_record(record))
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
