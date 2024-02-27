// use rs_nico_tracing::span::Record;
use serde::{Deserialize, Serialize};
use core::ops::Deref;
// use surrealdb::sql::{Id, Thing};
// use std::collections::BTreeMap;
use std::fmt::Debug;
use crate::ident::SurrealData;
use crate::ident::HasSurrealIdentifier;
use crate::ident::SurrealIDFactory;
use crate::ident::SurrealIDIdent;
use crate::ident::SurrealIDTable;
use crate::storable::DBThings;
use crate::RecordIdData;
use crate::SurrealID;
use crate::StorableId;

use surrealdb::sql::{ Thing, Id };

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
// impl<T: DBThings> SurrealIDFactory for Record<T> {}
impl<T: DBThings> SurrealIDIdent for Record<T> {
    fn id(&self) -> String {
        match &self {
            Record::RecordIdData(data) => data.id.id(),
            Record::RecordId(id) => id.0.id.to_string(),
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

// impl<T> DBThings for Record<T> {}

// impl<T> Record<T> {
//     pub fn new(tb: &str, id: &str, data: Option<T>) -> Self {
//         match data {
//             Some(data) => Record::RecordIdData(RecordIdData::new(tb, Some(id.into()), data)),
//             None => Record::RecordId(SurrealID::from(Thing::from((tb, id)))),
//         }
//     }
// }

// trait<T: 
// impl<T: DBThings> StorableId<T> for Record<T> {
//     // type Id = String;
//     type Item = T;
//     fn id(&self) -> String {
//         self.id()
//     }
//     fn table<k>(&self) -> String {
//         self.table()
//     }
//     fn data(&self) -> Self::Item {
//         match &self {
//             Record::RecordIdData(data) => data.data.clone().unwrap(),
//             Record::RecordId(id) => panic!("RecordId: {:?}", id),
//         }
//     }
// }


// impl<T: StorableId<T>> Deref for Record<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         match &self {
//             Record::RecordIdData(data) => data.data.as_ref().unwrap(),
//             Record::RecordId(id) => panic!("RecordId: {:?}", id),
//         }
//     }
// }

// 
// impl<T: StorableId<T>> From<Resource> for Record<T> {
// 	fn from(resource: Resource) -> Self {
// 		match resource {
// 			// Resource::Table(resource) => resource.into(),
// 			Resource::RecordId(resource) => resource.into(),
// 			// Resource::Object(resource) => resource.into(),
// 			// Resource::Array(resource) => resource.into(),
// 			// Resource::Edges(resource) => resource.into(),
//             _ => unimplemented!(),
// 		}
// 	}
// }
// 
// impl<T: StorableId<T>> IntoResource<Record<T>> for Record<T> {
//     fn into_resource(self) -> Result<Resource, surrealdb::Error> {
//         let thinggy = self.as_thing();
//         Ok(thinggy.into())
//     }
// }
// 
// impl<T: StorableId<T>> From<Thing> for Record<T> {
//     fn from(thing: Thing) -> Self {
//         Record::new(&thing.tb, &thing.id.to_raw(), None)
//     }
// }
