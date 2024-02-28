use crate::ident::HasSurrealIdentifier;
use crate::ident::SurrealData;
use crate::ident::SurrealIDFactory;
use crate::ident::SurrealIDIdent;
use crate::ident::SurrealIDTable;
use crate::storable::DBThings;
use crate::SurrealID;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use surrealdb::sql::{Id, Thing};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Record<T> {
    RecordIdData(RecordIdData<T>),
    RecordId(SurrealID),
}

impl<T: HasSurrealIdentifier> Record<T> {
    pub fn new(tb: &str, id: &str, data: Option<Box<T>>) -> Self {
        match data {
            Some(data) => Record::RecordIdData(RecordIdData::new(tb, Some(id.into()), *data)),
            None => Record::RecordId(SurrealID::new(tb, Some(id))),
        }
    }

    pub fn into_inner(self) -> Option<T> {
        match self {
            Record::RecordIdData(data) => data.data,
            _ => unimplemented!(),
        }
    }

    pub fn into_inner_mut(&mut self) -> &mut Option<T> {
        match self {
            Record::RecordIdData(data) => &mut data.data,
            _ => unimplemented!(),
        }
    }
}

impl<T: HasSurrealIdentifier> SurrealIDFactory for Record<T> {
    fn new(tb: &str, id: &str) -> SurrealID {
        SurrealID::new(tb, Some(id))
    }
    fn random(tb: &str) -> SurrealID {
        SurrealID::new(tb, None)
    }
}

impl<T: DBThings> SurrealData for Record<T> {}
impl<T: DBThings> HasSurrealIdentifier for Record<T> {}
impl<T: DBThings> DBThings for Record<T> {}
impl<T: DBThings> SurrealIDIdent for Record<T> {
    fn id(&self) -> Id {
        match &self {
            Record::RecordIdData(data) => data.id.id(),
            Record::RecordId(id) => id.0.id,
        }
    }
}
impl<T: DBThings> SurrealIDTable for Record<T> {
    fn table(&self) -> String {
        match &self {
            Record::RecordIdData(data) => data.id.table(),
            Record::RecordId(id) => id.0.tb.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecordIdData<T> {
    pub id: SurrealID,
    pub data: Option<T>,
}

impl<T> RecordIdData<T> {
    fn _new(id: SurrealID, data: T) -> Self {
        RecordIdData {
            id,
            data: Some(data),
        }
    }

    pub fn new(tb: &str, id: Option<Id>, data: T) -> Self {
        match id {
            Some(id) => RecordIdData {
                id: SurrealID::from(Thing::from((tb, id.to_raw().as_str()))),
                data: Some(data),
            },
            None => RecordIdData {
                id: SurrealID::from(Thing::from((tb, Id::rand().to_raw().as_str()))),
                data: Some(data),
            },
        }
    }

    pub fn new_dataless(tb: &str, id: Option<Id>) -> Self {
        match id {
            Some(id) => RecordIdData {
                id: SurrealID::from(Thing::from((tb, id.to_raw().as_str()))),
                data: None,
            },
            None => RecordIdData {
                id: SurrealID::from(Thing::from((tb, Id::rand().to_raw().as_str()))),
                data: None,
            },
        }
    }

    pub fn into_inner(self) -> Option<T> {
        self.data
    }

    pub fn into_inner_mut(&mut self) -> &mut Option<T> {
        &mut self.data
    }
}
