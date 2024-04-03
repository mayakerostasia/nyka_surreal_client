// use async_nats::Connection;

// const NATS_URL: &str = "nats://127.0.0.1:4222";


// DbService {
//     conn: Connection,
//     db: String,
// }


// impl DbService {
//     pub async fn new(conn: Connection, db: String) -> Self {
//         Self { conn, db }
//     }
// }


// pub async fn connect() -> Result<Connection, Error> {
//     let conn = Connection::new(NATS_URL).await?;