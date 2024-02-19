use std::env;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub surreal_path: String,
    pub namespace: String,
    pub database: String,
    pub user: String,
    pub secret: String,
}

impl DbConfig {
    pub fn new(
        surreal_path: String,
        namespace: String,
        database: String,
        user: String,
        secret: String,
    ) -> Self {
        Self {
            surreal_path,
            namespace,
            database,
            user,
            secret,
        }
    }
}

pub fn setup() -> DbConfig {
    DbConfig {
        surreal_path: env::var("DB_PATH").expect("DB_PATH must be set"),
        namespace: env::var("DB_NS").expect("DB_NS ( a Namespace Name ) must be set"),
        database: env::var("DB_DB").expect("DB_DB ( a Database Name ) must be set"),
        user: env::var("DB_USER").expect("USER must be set"),
        secret: env::var("DB_SECRET").expect("SECRET must be set"),
    }
}
