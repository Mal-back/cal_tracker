mod error;
pub mod meal;
mod store;
pub mod user;

use store::{init_db_bool, Db};

pub use self::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            db: init_db_bool().await?,
        })
    }
    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}
