use bb_config::{srql_config, SurrealCfg};
// use std::env;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub path: String,
    pub ns: String,
    pub db: String,
    pub user: String,
    pub pass: String,
}

pub fn setup() -> DbConfig {
    let cfg: SurrealCfg = srql_config().expect("Failed to setup surreal from ENV");
    DbConfig {
        path: cfg.path,
        ns: cfg.ns,
        db: cfg.db,
        user: cfg.user,
        pass: cfg.pass,
    }
}
