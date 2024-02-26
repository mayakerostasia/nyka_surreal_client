use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::Deref;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::prelude::*;
use crate::Error;

#[async_trait]
pub trait Storable<'a, T>
where 
    Self: Sized + Deref<Target = T>,
    T: StorableId + Send + Clone
{
    // type Item: StorableId;

    async fn save(self) -> Result<Option<T>, Error> {
        let _ = connect(None).await.ok();

        let record = Record::new(self.table().as_str(), self.id().as_str(), Some(self.clone()));
        let ret: Option<T> = create_record(record).await.expect("Whoops");
        Ok(ret)
    }

    async fn select(self) -> Result<Option<Record<T>>, Error> {
        let _ = connect(None).await.ok();

        let record: Record<T> = self.as_record::<Record<T>>();
        let rec: Option<Record<T>> = get_record(record).await?;
        Ok(rec)
    }

    async fn delete(self) -> Result<Record<T>, Error> {
        let _ = connect(None).await.ok();
        delete_record(self.as_record::<Record<T>>()).await
    }
}

pub trait StorableId: Debug + Serialize + DeserializeOwned + Sized + Clone {
    fn table(&self) -> String;
    fn id(&self) -> String;

    fn as_record<T>(&self) -> Record<Self>
    where
    {
        Record::new(self.table().as_str(), self.id().as_str(), Some(self.clone()))
    }

    fn as_thing(&self) -> Thing
    where
    {
        Thing::from((self.table(), self.id()))
    }
}