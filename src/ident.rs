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
    pub fn new(tb: &str, id: Option<Id>) -> Self {
        match id {
            Some(id) => SurrealID(Thing::from((tb, id))),
            None => SurrealID(Thing::from((tb, Id::rand().to_raw().as_str()))),
        }
    }
}

pub trait SurrealIDFactory {
    fn create(tb: &str, id: &str) -> SurrealID {
        SurrealID(Thing::from((tb, Id::from(id))))
    }
    fn random(tb: &str) -> SurrealID {
        SurrealID(Thing::from((tb, Id::rand().to_raw().as_str())))
    }
}

// impl From<SurrealID> for Object {
//     fn from(ident: SurrealID) -> Self {
//         let mut object = Object(BTreeMap::<String, Value>::new());
//         object.insert("tb".to_string(), Value::Strand(Strand(ident.0.tb.clone()));
//         // object.insert("id".to_string(), Id::(ident.0.id.clone()));
//         object
//     }
// }

// impl From<Object> for SurrealID {
//     fn from(object: Object) -> Self {
//         println!("{:?}", object.0);
//         Self::from((object.0["tb"].clone(), object.0["id"].clone()))
//         // object.0.into()
//     }
// }

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
        self.0.id.clone()
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
