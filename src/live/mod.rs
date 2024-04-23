pub mod subscribe {
    use futures_lite::StreamExt;
    use serde::{Deserialize, Serialize};
    use surrealdb::sql::Value;
    use tokio::task::JoinHandle;

    use crate::prelude::Record;
    use crate::{live_select, DBThings };

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct SomeData {
        // pub id: SurrealId,
        pub data: Value,
    }

    impl DBThings for SomeData {}

    pub async fn subscribe<'a>(table: &str) -> Result<JoinHandle<()>, surrealdb::Error> {
        println!("Subscribing to the database");

        let mut stream = live_select::<Record<SomeData>>(table, None).await.unwrap();

        Ok(tokio::spawn(async move {
            while let Some(record) = stream.next().await {
                println!("Record: {:?}", record);
            }
        }))
    }
}
