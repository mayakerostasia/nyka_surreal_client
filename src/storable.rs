use std::fmt::Debug;
use std::pin::Pin;

use async_trait::async_trait;
use futures_lite::Future;
use lazy_static::lazy_static;
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::opt::IntoResource;

use crate::prelude::*;
// use crate::record::RecordIdData;
use crate::setup;
use crate::DbConfig;
use crate::Error;

pub trait DBThings: Debug + Serialize + DeserializeOwned + Sized + Clone {}

lazy_static!{
    static ref CFG: DbConfig = setup();
}

#[async_trait]
pub trait Storable<T>
where
    T: DBThings + 'static,
{

    fn id(&self) -> Option<Id>;
    fn table(&self) -> Option<String>; 
    fn thing(&self) -> Thing;
    // fn resource(&self) -> Resource;
    fn data(&self) -> T;

    fn to_record(&self) -> Record<T> {
        // println!("Thing: {:?}", &thing);
        // println!("Thing as_string: {:?}", &thing.to_string());
        // println!("Thing as_raw: {:?}", &thing.to_raw());
        // println!("Thing as_resource: {:?}", &thing.into_resource());
        // println!("Thing: {:?}", &thing);
        let thing = self.thing();
        let data = self.data();
        // let meta = self.meta();
        Record { 
            id: thing,
            meta: None,
            data: Some(Box::new(data)),
        }
    }

    async fn save(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<T>, Error>>>> {
        let _ = connect(&CFG).await.ok();
        let record = self.to_record();
        Box::pin(create_record(record))
    }

    async fn select(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Record<T>>, Error>>>> {
        let _ = connect(&CFG).await.ok();
        let record = self.to_record();
        Box::pin(get_record(record))
    }

    async fn delete(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Option<T>, Error>>>> {
        let _ = connect(&CFG).await.ok();
        let record = self.to_record();
        Box::pin(delete_record(record))
    }
}