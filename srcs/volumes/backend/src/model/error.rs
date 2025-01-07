use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::crypt;

use super::store;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    Store(store::Error),
    Crypt(crypt::Error),

    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),

    ItemNotFound { entity: &'static str, id: i64 },
    PublicUserNotFound { owner_id: i64 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// Froms

impl From<store::Error> for Error {
    fn from(value: store::Error) -> Self {
        Self::Store(value)
    }
}

impl From<crypt::Error> for Error {
    fn from(value: crypt::Error) -> Self {
        Self::Crypt(value)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}
