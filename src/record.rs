use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

use crate::ident::SurrealId;
// use crate::ident::SurrealData;
// use crate::ident::SurrealIDFactory;
// use crate::ident::SurrealIDIdent;
// use crate::ident::SurrealIDTable;
use crate::storable::DBThings;
// use crate::SurrealID;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Record<T> {
    RecordIdData(RecordIdData<T>),
    RecordId(SurrealId),
}

impl<T> Record<T> {
    pub fn new(tb: Option<String>, id: Option<Id>, data: Option<Box<T>>) -> Self {
        match data {
            Some(data) => Record::RecordIdData(RecordIdData::new(tb, id, *data)),
            None => Record::RecordIdData(RecordIdData::new_dataless(tb.as_ref().unwrap().as_str(), id)),
            _ => unimplemented!(    "Record::new( {:?}, {:?} )", tb, id ) ,
        }
    }
    
    pub fn id(&self) -> Id {
        match &self {
            Record::RecordIdData(data) => data.id.0.id.clone(),
            Record::RecordId(id) => id.0.id.clone(),
        }
    }
    
    pub fn table(&self) -> String {
        match &self {
            Record::RecordIdData(data) => data.id.0.tb.clone(),
            Record::RecordId(id) => id.0.tb.clone(),
        }
    }

    pub fn data(&self) -> Option<&T> {
        match self {
            Record::RecordIdData(data) => data.data.as_ref(),
            Record::RecordId(_id) => None,
        }
    }

    pub fn into_inner(self) -> Option<T> {
        match self {
            Record::RecordIdData(data) => data.data,
            Record::RecordId(_id) => None,
        }
    }

    pub fn into_inner_mut(&mut self) -> &mut Option<T> {
        match self {
            Record::RecordIdData(data) => &mut data.data,
            Record::RecordId(_id) => panic!("Cannot mutate a RecordId"),
        }
    }
}

// impl<T: SurrealIdentifier> SurrealIDFactory for Record<T> {
//     fn create(tb: &str, id: &str) -> SurrealID {
//         todo!("Record::create()");
//         // SurrealID::new(tb, Id::from(id))
//     }
//     fn random(tb: &str) -> SurrealID {
//         todo!("Record::random()");
//         // SurrealID::new(tb, Id::rand())
//     }
// }

// impl<T: DBThings> SurrealData for Record<T> {}
// impl<T: DBThings> SurrealIdentifier for Record<T> {}

impl<T: DBThings> DBThings for Record<T> {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecordIdData<T> {
    pub id: SurrealId,
    pub data: Option<T>,
}

impl<T> RecordIdData<T> {
    fn _new(id: SurrealId, data: T) -> Self {
        RecordIdData {
            id,
            data: Some(data),
        }
    }

    pub fn new(tb: Option<String>, id: Option<Id>, data: T) -> Self {
        match (tb, id) {
            (Some(tb), Some(id)) => RecordIdData {
                id: SurrealId(Thing::from((tb.as_str(), id.to_string().as_str()))),
                data: Some(data),
            },
            (Some(tb), None) => RecordIdData {
                id: SurrealId(Thing::from((tb.as_str(), Id::rand().to_string().as_str()))),
                data: Some(data),
            },
            (None, Some(id)) => unimplemented!(
                " Didn't provide Table info - Provided Table:None and {:?} ",
                id
            ),
            (None, None) => unimplemented!(
                " Didn't provide Table info - Provided Table:None and Id:None "
            ),
        }
    }

    pub fn new_dataless(tb: &str, id: Option<Id>) -> Self {
        match id {
            Some(id) => RecordIdData {
                id: SurrealId(Thing::from((tb, id.to_raw().as_str()))),
                data: None,
            },
            None => RecordIdData {
                id: SurrealId(Thing::from((tb, Id::rand().to_raw().as_str()))),
                data: None,
            },
        }
    }

    pub fn into_inner(self) -> Option<T> {
        self.data
    }

    pub fn into_inner_mut(self) -> Option<T> {
        self.data
    }
}
