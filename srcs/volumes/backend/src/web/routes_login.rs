use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;

use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::debug;

use crate::{
    crypt,
    ctx::Ctx,
    model::{
        user::user::{UserBmc, UserForLogin},
        ModelManager,
    },
    web::{remove_auth_token_cookie, set_auth_token_cookie, Error, Result},
};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LogoutPayload {
    should_log_out: bool,
}

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login/", post(api_login_handler))
        .route("/api/logout/", post(api_logout_handler))
        .with_state(mm)
}

async fn api_login_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_login", "HANDLER");

    let LoginPayload {
        username,
        password: pwd_clear,
    } = payload;

    let ctx = Ctx::root_ctx();

    let user: UserForLogin = UserBmc::first_by_username(
        &ctx,
        &mm,
        &username,
        "id, username, password, password_salt, token_salt",
    )
    .await?
    .ok_or(Error::LoginFailUsernameNotFound)?;

    let user_id = user.id;

    let Some(password) = user.password else {
        return Err(Error::LoginFailUserHasNoPassword { user_id });
    };

    crypt::pwd::validate_password(
        &crypt::EncryptContent {
            content: pwd_clear,
            salt: user.password_salt.to_string(),
        },
        &password,
    )
    .map_err(|_| Error::LoginFailPasswordNotMatching { user_id })?;

    set_auth_token_cookie(&cookies, &username, &user.token_salt.to_string())?;

    let body = Json(json!(
    {"result": {
        "success": true
               }
    }));

    Ok(body)
}
async fn api_logout_handler(
    cookies: Cookies,
    Json(payload): Json<LogoutPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_logout", "HANDLER");

    if payload.should_log_out == true {
        remove_auth_token_cookie(&cookies);
    }

    let body = Json(json!(
    {"result": {
        "logout": payload.should_log_out
               }
    }));

    Ok(body)
}
