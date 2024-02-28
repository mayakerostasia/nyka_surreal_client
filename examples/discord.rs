use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::sql::Id;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[tokio::main]
async fn main() {
    let db = Surreal::new::<Mem>(()).await.expect("expectin db");
    db.use_ns("test").use_db("test").await.expect("expect use");
    if let Err(e) = populate(db).await {
        dbg!(e);
    }
}

async fn populate(db: Surreal<Db>) -> Result<(), surrealdb::Error> {
    let settlement = Settlement {
        id: SurrealID(Thing::from(("settlements", Id::rand()))),
        region: String::from("region"),
        area: String::from("area"),
        name: None,
        location: Geom(Poin(Coor { x: 0.0, y: 0.0 })),
    };
    let settle: Vec<Settlement> = db.create("settlements").content(settlement).await?;
    println!("Settlement: {:#?}", settle);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settlement {
    pub id: SurrealID,
    pub region: String,
    pub area: String,
    pub name: Option<String>,
    pub location: Geom,
}

/// These are the "Maps" with a single Key
#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealID(pub Thing);

#[derive(Debug, Serialize, Deserialize)]
pub struct Geom(Poin);

#[derive(Debug, Serialize, Deserialize)]
pub struct Poin(Coor);

/// A "Map" with more than one key ( In this instance )
#[derive(Debug, Serialize, Deserialize)]
pub struct Coor {
    pub x: f64,
    pub y: f64,
}
