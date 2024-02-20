use crate::prelude::*;
use crate::Error;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

#[async_trait]
pub trait Storable<'a>
where
    Self: Serialize + DeserializeOwned + Debug + Sized,
{
    type Item: Serialize;

    fn table(&self) -> &'a str;
    fn id(&self) -> &'a str;
    // fn as_thing(&self) -> Thing;
    // fn as_record(&self) -> Record<Self::Item>;
    // fn for_db(&self) -> ToValue;

    async fn save(self) -> Result<Option<Record<Self>>, Error> {
        connect(None).await?;
        let ret: Result<Option<Record<Self>>, Error> =
            create_record(self.table(), self.id(), Some(self)).await;
        ret
    }

    async fn select(&self) -> Result<Option<Self>, Error> {
        connect(None).await?;
        get_record::<Self>(self.table(), self.id()).await
    }

    async fn delete(self) -> Result<Record<Self>, Error> {
        connect(None).await?;
        delete_record::<Self>(self.table(), self.id()).await
    }
}
