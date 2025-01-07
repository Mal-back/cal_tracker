use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // Config
    ConfigMissingEnv(&'static str),
    ConfigWrongFormat(&'static str),

    Model(model::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// Froms
//
impl From<model::Error> for Error {
    fn from(value: model::Error) -> Self {
        Self::Model(value)
    }
}
