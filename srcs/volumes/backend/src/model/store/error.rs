use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize)]
pub enum Error {
    FailToCreatePool(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
