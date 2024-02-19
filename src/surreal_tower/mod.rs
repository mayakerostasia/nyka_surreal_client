use rs_nico_tracing::debug;
mod db_request;
// mod db_functions;
mod db_service;

use surrealdb::sql::{Value, Thing};

pub use db_service::DbService;

pub use db_request::Cmd;
pub use db_request::DbRequest;

// pub use db_functions::DbFunctions;


mod prelude {
    pub use crate::surreal_tower::Record;
    pub use crate::surreal_tower::db_request::Cmd;
    pub use crate::surreal_tower::db_request::DbRequest;
    pub use crate::surreal_tower::db_service::DbService;
    // pub use crate::surreal_tower::db_functions::DbFunctions;
}


#[cfg(test)]

mod tests {
    use crate::Error;
    use super::prelude::*;

    #[tokio::test]
    async fn test_get_database() -> Result<(), Error> {

        // DbService::new().await?;
        Ok(())
    }
}

