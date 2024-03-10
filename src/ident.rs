use core::fmt::Formatter;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SurrealID { 
    Thing(Thing),
    TableId(String, Id),
    Id(Id),
    Table(String),
}

impl SurrealID {
    pub fn new(tb: Option<&str>, id: Option<Id>) -> Self {
        match (tb, id) {
            (Some(tb), Some(id)) => SurrealID::Thing(Thing::from((tb, id))),
            (Some(tb), None) => SurrealID::Table(tb.to_string()),
            (None, Some(id)) => SurrealID::Id(id),
            (None, None) => SurrealID::Id(Id::rand()),
        }
    }
}

pub trait SurrealIDFactory {
    fn create(tb: &str, id: &str) -> SurrealID {
        todo!("SurrealIDFactory::create()");
        // SurrealID { tb: tb.to_string(), id: Some(Id::from(id)) }
    }
    fn random(tb: &str) -> SurrealID {
        todo!(  "SurrealIDFactory::random()" );
        // SurrealID { tb: tb.to_string(), id: Some(Id::rand()) }
    }
}

pub trait HasSurrealIdentifier: SurrealIDTable + SurrealIDIdent {}

pub trait SurrealData
where
    Self: Sized + Clone,
{
    fn data<T: std::convert::From<Self>>(self) -> T {
        <Self as std::convert::Into<T>>::into(self.clone())
    }
}
pub trait SurrealIDTable {
    fn table(&self) -> String;
}
pub trait SurrealIDIdent {
    fn id(&self) -> Id;
}

impl SurrealIDFactory for SurrealID {}
impl SurrealIDIdent for SurrealID {
    fn id(&self) -> Id {
        // todo!( "SurrealID::id()" );
        match self {
            SurrealID::Thing(thing) => thing.id.clone(),
            SurrealID::TableId(_, id) => id.clone(),
            SurrealID::Id(id) => id.clone(),
            _ => unimplemented!("SurrealID::id( Didn't have an ID! )"),
        }
    }
}
impl SurrealIDTable for SurrealID {
    fn table(&self) -> String {
        // todo!(  "SurrealID::table()" );
        match self {
            SurrealID::Thing(thing) => thing.tb.clone(),
            SurrealID::Table(tb) => tb.clone(),
            SurrealID::TableId(tb, _) => tb.clone(),
            _ => unimplemented!(),
        }
        // self.tb.clone()
    }
}
// impl SurrealData for SurrealID {
//     fn data(&self) -> String {
//         self.clone().to_string()
//     }
// }

impl Default for SurrealID {
    fn default() -> Self {
        todo!("SurrealID::default()");
        // SurrealID::Thing(Thing::random())
        // SurrealID { tb: "_".to_string(), id: Some(Id::rand()) }
    }
}

impl Display for SurrealID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!("SurrealID::fmt");
        // write!(f, "{}:{:?}", self.tb, self.id.clone())
    }
}

impl From<Thing> for SurrealID {
    fn from(thing: Thing) -> Self {
        SurrealID::Thing(thing)
    }
}

impl From<SurrealID> for Thing {
    fn from(ident: SurrealID) -> Self {
        todo!("SurrealID::from(SurrealID)");

    }
}

impl From<(String, String)> for SurrealID {
    fn from((tb, id): (String, String)) -> Self {
        SurrealID::Thing(Thing::from((tb, id)))
    }
}

impl From<(&str, &str)> for SurrealID {
    fn from((tb, id): (&str, &str)) -> Self {
        SurrealID::Thing(Thing::from((tb, id)))
    }
}

impl From<(String, Id)> for SurrealID {
    fn from((tb, id): (String, Id)) -> Self {
        SurrealID::Thing(Thing::from((tb, id)))
    }
}

impl From<(&str, Id)> for SurrealID {
    fn from((tb, id): (&str, Id)) -> Self {
        SurrealID::Thing(Thing::from((tb, id)))
    }
}

impl FromStr for SurrealID {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("SurrealID::from_str");
        // let id: Thing = s.parse().unwrap();
        // Ok(SurrealID(id))
    }
}

// impl &Ident {
//     pub fn as_thing(&self) -> &Thing {
//         &self.id
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident() {
        let random = Thing::from(("random", "1"));
        let ident: SurrealID = random.into();
        println!("{:#?}", ident);
    }
}
