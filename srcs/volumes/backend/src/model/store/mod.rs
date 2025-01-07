mod error;

use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub use self::error::{Error, Result};

use crate::config::config;

pub type Db = Pool<Postgres>;

pub async fn init_db_bool() -> Result<Db> {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(500))
        .max_connections(5)
        .connect(&config().DB_URL)
        .await
        .map_err(|e| Error::FailToCreatePool(e.to_string()))
}
