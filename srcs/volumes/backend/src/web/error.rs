use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
use tracing::debug;

use crate::{crypt, model, web};

use super::mw_auth::CtxExtError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    Model(model::Error),
    Crypt(crypt::Error),
    CtxExt(web::mw_auth::CtxExtError),

    // Login
    LoginFailUsernameNotFound,
    LoginFailUserHasNoPassword { user_id: i64 },
    LoginFailPasswordNotMatching { user_id: i64 },

    // User
    AccountCreationFailedPassowrdToWeak,
    AccountCreationFailUsernameAlreadyTaken,
    UpdateFailedPasswordNotMatching,
    UpdateFailedPasswordTooWeak,

}

impl From<model::Error> for Error {
    fn from(value: model::Error) -> Self {
        Self::Model(value)
    }
}

impl From<crypt::Error> for Error {
    fn from(value: crypt::Error) -> Self {
        Self::Crypt(value)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        debug!("{:<12} - web:Error {self:?}", "INTO_RESPONSE");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(Arc::new(self));

        response
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Error::CtxExt(CtxExtError::CtxCreateFail(_))
            | Error::CtxExt(CtxExtError::UserNotFound)
            | Error::CtxExt(CtxExtError::CtxNotInRequest)
            | Error::CtxExt(CtxExtError::TokenParsingFail)
            | Error::CtxExt(CtxExtError::TokenInvalidVerification)
            | Error::CtxExt(CtxExtError::TokenNotInCookie) => {
                (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH)
            },
            Error::LoginFailUsernameNotFound
            | Error::LoginFailPasswordNotMatching { .. }
            | Error::LoginFailUserHasNoPassword { .. } => {
                (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL)
            },
            Error::AccountCreationFailedPassowrdToWeak
            | Error::UpdateFailedPasswordTooWeak => {
                (StatusCode::BAD_REQUEST, ClientError::WEAK_PASSWORD)
            },
            Error::UpdateFailedPasswordNotMatching => {
                (StatusCode::BAD_REQUEST, ClientError::WRONG_PASSWORD)
            }
            Error::AccountCreationFailUsernameAlreadyTaken => {
                (StatusCode::BAD_REQUEST, ClientError::USERNAME_ALREADY_TAKEN)
            },
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    WEAK_PASSWORD,
    USERNAME_ALREADY_TAKEN,
    WRONG_PASSWORD,
    SERVICE_ERROR,
}
