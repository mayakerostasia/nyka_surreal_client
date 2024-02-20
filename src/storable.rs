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

    fn table(&self) -> String;
    fn id(&self) -> String;
    // fn as_thing(&self) -> Thing;
    // fn as_record(&self) -> Record<Self::Item>;
    // fn for_db(&self) -> ToValue;

    async fn save(self) -> Result<Option<Record<Self>>, Error> {
        connect(None).await?;
        let ret: Result<Option<Record<Self>>, Error> =
            create_record(self.table().as_str(), self.id().as_str(), Some(self)).await;
        ret
    }

    async fn select(&self) -> Result<Option<Self>, Error> {
        connect(None).await?;
        get_record::<Self>(self.table().as_str(), self.id().as_str()).await
    }

    async fn delete(self) -> Result<Record<Self>, Error> {
        connect(None).await?;
        delete_record::<Self>(self.table().as_str(), self.id().as_str()).await
    }
}
