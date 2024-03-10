use std::fmt::Debug;
use std::pin::Pin;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::DbConfig;
use crate::ident::HasSurrealIdentifier;
use crate::ident::SurrealData;
use crate::prelude::*;
use crate::Error;

pub trait DBThings: Debug + Serialize + DeserializeOwned + Sized + Clone {}

#[async_trait]
pub trait Storable
where
    Self: DBThings + HasSurrealIdentifier + SurrealData + From<Record<Self>>,
{
    async fn save(self, config: &DbConfig) -> Result<Vec<Self>, Error> {
        let _ = connect(config).await.ok();

        let ret: Result<Vec<Self>, Error> = create_record(Record::new(
            self.table(false).as_str(),
            Some(self.id(true)),
            Some(Box::new(self)),
        ))
        .await;
        ret
    }

    async fn select(&self, config: &DbConfig) -> Result<Option<Record<Self>>, Error> {
        let _ = connect(config).await.ok();
        let rec: Option<Record<Self>> = get_record(self.table(false).as_str(), self.id(false)).await?;
        Ok(rec)
    }

    async fn delete(&self, config: &DbConfig) -> Result<Pin<Box<Self>>, Error> {
        let _ = connect(config).await.ok();
        let rec: Result<Self, Error> = delete_record(self.table(false).as_str(), self.id(false)).await;
        match rec {
            Ok(rec) => Ok(Box::pin(rec)),
            Err(e) => Err(e),
        }
    }
}

impl<T: Storable> From<T> for Record<T> {
    fn from(storable: T) -> Self {
        let id = storable.id(false);
        let table = storable.table(false);
        let data = storable.data();
        let record: Record<T> = Record::new(table.as_str(), Some(id), Some(Box::new(data)));
        record
    }
}
