use std::fmt::Debug;
use std::pin::Pin;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

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
    async fn save(self) -> Result<Vec<Self>, Error> {
        let _ = check_connect().await.ok();

        let ret: Result<Vec<Self>, Error> = create_record(Record::new(
            self.table().as_str(),
            self.id(),
            Some(Box::new(self)),
        ))
        .await;
        ret
    }

    async fn select(&self) -> Result<Option<Record<Self>>, Error> {
        let _ = check_connect().await.ok();
        let rec: Option<Record<Self>> = get_record(self.table().as_str(), self.id()).await?;
        Ok(rec)
    }

    async fn delete(&self) -> Result<Pin<Box<Self>>, Error> {
        let _ = check_connect().await.ok();
        let rec: Result<Self, Error> = delete_record(self.table().as_str(), self.id()).await;
        match rec {
            Ok(rec) => Ok(Box::pin(rec)),
            Err(e) => Err(e),
        }
    }
}

impl<T: Storable> From<T> for Record<T> {
    fn from(storable: T) -> Self {
        let id = storable.id();
        let table = storable.table();
        let data = storable.data();
        let record: Record<T> = Record::new(table.as_str(), id, Some(Box::new(data)));
        record
    }
}
