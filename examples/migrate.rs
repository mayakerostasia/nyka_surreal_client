use std::fs;
use std::io::Read;
use std::path::Path;

use nico_surreal_client::Error;
use surrealdb::engine::any::connect;
use surrealdb::opt::auth::Root;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db = connect("ws://localhost:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("nico").use_db("nico").await?;

    let p = include_str!("./migrations.surql");
    db.query(p).await?; // Import all migrations from a file

    Ok(())
}
