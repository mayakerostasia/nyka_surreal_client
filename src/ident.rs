use std::{fmt::Debug, str::FromStr};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ident {
    pub id: Thing,
}

impl From<Thing> for Ident {
    fn from(thing: Thing) -> Self {
        Ident { id: thing }
    }
}

impl From<Ident> for Thing {
    fn from(ident: Ident) -> Self {
        ident.id
    }
}

impl From<(String, String)> for Ident {
    fn from((tb, id): (String, String)) -> Self {
        Ident {
            id: Thing::from((tb, id)),
        }
    }
}

impl From<(&str, &str)> for Ident {
    fn from((tb, id): (&str, &str)) -> Self {
        Ident {
            id: Thing::from((tb, id)),
        }
    }
}

impl From<(String, Id)> for Ident {
    fn from((tb, id): (String, Id)) -> Self {
        Ident {
            id: Thing::from((tb, id)),
        }
    }
}

impl From<(&str, Id)> for Ident {
    fn from((tb, id): (&str, Id)) -> Self {
        Ident {
            id: Thing::from((tb, id)),
        }
    }
}

impl FromStr for Ident {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id: Thing = s.parse().unwrap();
        Ok(Ident { id })
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident() {
        let random = Thing::from(("random", "1"));
        let ident: Ident = random.into();
        println!("{:#?}", ident);
    }
}