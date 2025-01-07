use crate::crypt::token::{verify_web_token_signature, Token};

use crate::ctx::Ctx;
use crate::model::user::user::{UserBmc, UserForAuth};
use crate::model::ModelManager;
use crate::web::AUTH_TOKEN;
use crate::web::{set_auth_token_cookie, Error, Result};
use axum::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

//Middleware boilerplate code
// use axum::body::Body; --> Body must be this type
//pub async fn mv_require_auth(req: Request<Body>, next: Next) -> Result<Response> {
//    Ok(next.run(req).await)

//}

pub async fn mw_ctx_resolver(
    mm: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - Ctx resolver middleware", "MIDDLEWARE",);

    let ctx_res = _ctx_resolver(mm, &cookies).await;

    if ctx_res.is_err() && !matches!(ctx_res, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    req.extensions_mut().insert(ctx_res);

    Ok(next.run(req).await)
}

async fn _ctx_resolver(State(mm): State<ModelManager>, cookies: &Cookies) -> CtxExtResult {
    let auth_token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)?;

    let auth_token: Token = auth_token
        .parse()
        .map_err(|_| CtxExtError::TokenParsingFail)?;

    let root_ctx = Ctx::root_ctx();

    let user: UserForAuth = UserBmc::first_by_username(
        &root_ctx,
        &mm,
        &auth_token.ident,
        "id, username, token_salt",
    )
    .await
    .map_err(|e| CtxExtError::ModelAccessError(e.to_string()))?
    .ok_or(CtxExtError::UserNotFound)?;

    verify_web_token_signature(&auth_token, &user.token_salt.to_string())
        .map_err(|_| CtxExtError::TokenInvalidVerification)?;

    set_auth_token_cookie(
        cookies,
        &user.username.to_string(),
        &user.token_salt.to_string(),
    )
    .map_err(|_| CtxExtError::TokenUpdateFailed)?;

    let ctx_res = Ctx::new(user.id).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()));

    ctx_res
}

pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    debug!("{:<12} - Auth Middleware", "MIDDLEWARE",);

    ctx?;

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequest))?
            .clone()
            .map_err(Error::CtxExt)
    }
}

type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    TokenNotInCookie,
    TokenParsingFail,

    UserNotFound,
    TokenInvalidVerification,
    TokenUpdateFailed,

    ModelAccessError(String),

    CtxNotInRequest,
    CtxCreateFail(String),
}
