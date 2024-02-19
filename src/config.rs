use std::env;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub path: String,
    pub ns: String,
    pub db: String,
    pub user: String,
    pub secret: String,
}

pub fn setup() -> DbConfig {
    DbConfig {
        path: env::var("DB_PATH").expect("DB_PATH must be set"),
        ns: env::var("DB_NS").expect("DB_NS ( a Namespace Name ) must be set"),
        db: env::var("DB_DB").expect("DB_DB ( a Database Name ) must be set"),
        user: env::var("DB_USER").expect("DB_USER must be set"),
        secret: env::var("DB_SECRET").expect("DB_SECRET must be set"),
    }
}
