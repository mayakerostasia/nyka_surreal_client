use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;
use surrealdb::sql::{Id, Thing};

use crate::{storable::DBThings, Storable};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record<T> {
    pub id: RecordId,
    pub meta: Option<Box<T>>,
    pub data: Option<Box<T>>,
}

impl<T> Record<T> {
    pub fn new(tb: Option<String>, id: Option<Id>, data: Option<Box<T>>) -> Self {
        Self {
            id: Thing::from((tb.clone().expect("No table"), id.expect("No Id!"))),
            meta: None,
            data,
        }
    }

    pub fn id(&self) -> Id {
        self.id.id.clone()
    }

    pub fn table(&self) -> String {
        self.id.tb.clone()
    }

    pub fn data(&self) -> Option<&T> {
        self.data.as_ref().map(|d| &**d)
    }

    pub fn into_inner(self) -> Option<T> {
        todo!("Record::into_inner()");
    }

    pub fn into_inner_mut(&mut self) -> &mut Option<T> {
        todo!("Record::into_inner_mut()");
    }
}

impl<T: DBThings> DBThings for Record<T> {}

impl<T> Storable<Record<T>> for Record<T>
where
    T: DBThings + Storable<T> + 'static,
{
    fn thing(&self) -> Thing {
        Thing::from((self.table(), self.id()))
    }

    fn id(&self) -> Option<Id> {
        Some(self.id.id.clone())
    }

    fn table(&self) -> Option<String> {
        Some(self.id.tb.clone())
    }

    fn data(&self) -> Record<T> {
        unimplemented!()
    }
}
