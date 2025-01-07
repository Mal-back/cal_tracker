use axum::{
    extract::State,
    middleware,
    routing::{delete, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::{debug, info};

use crate::{
    crypt,
    ctx::Ctx,
    model::{
        user::{public_user::{PublicUser, PublicUserBmc, PublicUserForUpdate}, user::{User, UserBmc, UserForLogin, UserForNewPwd}, FullUser, FullUserBmc, FullUserForCreate},
        ModelManager,
    },
    utils::password::check_password_safety,
    web::{mw_auth::mw_require_auth, remove_auth_token_cookie},
};

use super::{Error, Result};

pub fn routes(mm: ModelManager) -> Router {
    let no_middleware_routes = Router::new()
        .route("/users/", post(create_user_handler))
        .with_state(mm.clone());

    let protected_routes = Router::new()
        .route("/users/password/", post(update_user_password_handler))
        .route("/users/", delete(delete_user_handler).get(get_user_handler))
        .route("/public_users/", patch(update_public_user_handler))
        .route_layer(middleware::from_fn(mw_require_auth))
        .with_state(mm.clone());

    no_middleware_routes.merge(protected_routes)
}

async fn create_user_handler(
    State(mm): State<ModelManager>,
    Json(payload): Json<FullUserForCreate>,
) -> Result<Json<FullUser>> {
    debug!("{:<12} - account creation", "HANDLER");
    //let (username, password_clear) = (payload.username, payload.password_clear);
    check_password_safety(&payload.password_clear)
        .map_err(|_| Error::AccountCreationFailedPassowrdToWeak)?;

    let ctx = Ctx::root_ctx();

    let existing: Option<User> =
        UserBmc::first_by_username(&ctx, &mm, &payload.username, "id, username").await?;

    if existing.is_some() {
        return Err(Error::AccountCreationFailUsernameAlreadyTaken);
    }

    let id = FullUserBmc::create_new_user(&ctx, &mm, &payload).await?;

    UserBmc::update_password(&ctx, &mm, id, &payload.password_clear).await?;

    let created = FullUserBmc::get(&ctx, &mm, id).await?;

    info!("Account {} was created", id);

    Ok(Json(created))
}

async fn get_user_handler(
    State(mm): State<ModelManager>,
    ctx: Ctx) -> Result<Json<FullUser>> {
    let full_user = FullUserBmc::get(&ctx, &mm, ctx.user_id()).await?;
    Ok(Json(full_user))
}

async fn update_user_password_handler(
    State(mm): State<ModelManager>,
    ctx: Ctx,
    Json(payload): Json<UserForNewPwd>,
) -> Result<Json<Value>> {
    debug!("{:<12} - Password update", "HANDLER");

    let user_id = ctx.user_id();
    let user: UserForLogin = UserBmc::get(
        &ctx,
        &mm,
        user_id,
        "id, username, password, password_salt, token_salt",
        "user for auth",
    )
    .await?;

    let Some(password) = user.password else {
        return Err(Error::LoginFailUserHasNoPassword { user_id });
    };

    crypt::pwd::validate_password(
        &crypt::EncryptContent {
            content: payload.old_pwd_clear,
            salt: user.password_salt.to_string(),
        },
        &password,
    )
    .map_err(|_| Error::UpdateFailedPasswordNotMatching)?;

    check_password_safety(&payload.password_clear)
        .map_err(|_| Error::UpdateFailedPasswordTooWeak)?;
    UserBmc::update_password(&ctx, &mm, user_id, &payload.password_clear).await?;

    Ok(Json(json!({
        "Ok": "Password Modified"
    })))
}

async fn update_public_user_handler(
    State(mm): State<ModelManager>,
    ctx: Ctx,
    Json(payload): Json<PublicUserForUpdate>
    ) -> Result<Json<PublicUser>> {
    PublicUserBmc::update(&ctx, &mm, payload).await?;
    let updated_user = PublicUserBmc::get(&ctx, &mm, ctx.user_id())
        .await?;

    Ok(Json(updated_user))
}

async fn delete_user_handler(
    State(mm): State<ModelManager>,
    ctx: Ctx,
    cookies: Cookies,
) -> Result<Json<Value>> {
    debug!("{:<12} - Account deletion", "HANDLER");
    UserBmc::delete(&ctx, &mm, ctx.user_id()).await?;

    remove_auth_token_cookie(&cookies);

    info!("Account {} was deleted", ctx.user_id());
    Ok(Json(json!({
        "Ok": "Account deleted"
    })))
}
