use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    KeyFailHmac,

    PasswordNotMatching,

    TokenInvalidFormat,
    TokentCannotDecodeIdent,
    TokenCannotDecodeExp,
    TokenSignatureNotMatching,
    TokenTimeNotIso,
    TokenExpired,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
