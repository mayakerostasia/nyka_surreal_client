
use std::fmt::Display;
use std::pin::Pin;
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
    T: StorableId<T> + Send + Clone 
{
    // type Item: StorableId;

    async fn save(self) -> Result<Vec<T>, Error> {
        let _ = connect(None).await.ok();

        let record = Record::new(self.table().as_str(), self.id().as_str(), Some(self.clone()));
        let ret: Vec<T> = create_record(record).await.expect("Whoops");
        Ok(ret)
    }

    async fn select(&self) -> Result<Option<Record<T>>, Error> {
        let _ = connect(None).await.ok();
        let record: Record<T> = Record::new(self.table().as_str(), self.id().as_str(), None);
        let rec: Option<Record<T>> = get_record(self.table().as_str(), self.id().as_str()).await?;
        Ok(rec)
    }

    async fn delete(&self) -> Result<Pin<Box<T>>, Error> {
        let _ = connect(None).await.ok();
        let rec: Result<T, Error> = delete_record(self.table().as_str(), self.id().as_str()).await;
        match rec {
            Ok(rec) => Ok(Box::pin(rec)),
            Err(e) => Err(e),
        }
    }
}

pub trait StorableId<T>: Debug + Serialize + DeserializeOwned + Sized + Clone 
// where T: StorableId<T>
{
    type Item: StorableId<T>;
    // type Id; 

    fn table(&self) -> String;
    fn id(&self) -> String;
    fn data(&self) -> T;

    fn to_record(self) -> Record<T>
    where
    {
        let record: Record<T> = Record::new(self.table().as_str(), self.id().as_str(), Some(self.data()));
        record
    }

    fn as_thing(&self) -> Thing
    where
    {
        Thing::from((self.table(), self.id()))
    }
}