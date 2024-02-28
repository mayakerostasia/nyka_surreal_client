use core::fmt::Formatter;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SurrealID(pub Thing);

impl SurrealID {
    pub fn new(tb: &str, id: Option<&str>) -> Self {
        match id {
            Some(id) => SurrealID(Thing::from((tb, id))),
            None => SurrealID(Thing::from((tb, Id::rand().to_raw().as_str()))),
        }
    }
}

pub trait SurrealIDFactory {
    fn new(tb: &str, id: &str) -> SurrealID {
        SurrealID(Thing::from((tb, id)))
    }
    fn random(tb: &str) -> SurrealID {
        SurrealID(Thing::from((tb, Id::rand().to_raw().as_str())))
    }
}

pub trait HasSurrealIdentifier: SurrealIDTable + SurrealIDIdent {}

pub trait SurrealData
where
    Self: Sized + Clone,
{
    fn data<T: std::convert::From<Self>>(self) -> T {
        self.clone().try_into().unwrap()
    }
}
pub trait SurrealIDTable {
    fn table(&self) -> String;
}
pub trait SurrealIDIdent {
    fn id(&self) -> String;
}

impl SurrealIDFactory for SurrealID {}
impl SurrealIDIdent for SurrealID {
    fn id(&self) -> String {
        self.0.id.to_string()
    }
}
impl SurrealIDTable for SurrealID {
    fn table(&self) -> String {
        self.0.tb.clone()
    }
}
// impl SurrealData for SurrealID {
//     fn data(&self) -> String {
//         self.clone().to_string()
//     }
// }

impl Default for SurrealID {
    fn default() -> Self {
        SurrealID(Thing::from(("_", Id::rand().to_raw().as_str())))
    }
}

impl Display for SurrealID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl From<Thing> for SurrealID {
    fn from(thing: Thing) -> Self {
        SurrealID(thing)
    }
}

impl From<SurrealID> for Thing {
    fn from(ident: SurrealID) -> Self {
        ident.0
    }
}

impl From<(String, String)> for SurrealID {
    fn from((tb, id): (String, String)) -> Self {
        SurrealID(Thing::from((tb, id)))
    }
}

impl From<(&str, &str)> for SurrealID {
    fn from((tb, id): (&str, &str)) -> Self {
        SurrealID(Thing::from((tb, id)))
    }
}

impl From<(String, Id)> for SurrealID {
    fn from((tb, id): (String, Id)) -> Self {
        SurrealID(Thing::from((tb, id)))
    }
}

impl From<(&str, Id)> for SurrealID {
    fn from((tb, id): (&str, Id)) -> Self {
        SurrealID(Thing::from((tb, id)))
    }
}

impl FromStr for SurrealID {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id: Thing = s.parse().unwrap();
        Ok(SurrealID(id))
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
