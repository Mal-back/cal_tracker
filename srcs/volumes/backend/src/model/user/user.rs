use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::prelude::FromRow;
use sqlx::Transaction;
use uuid::Uuid;

use crate::crypt::pwd::encrypt_pwd;
use crate::crypt::EncryptContent;
use crate::ctx::Ctx;
use crate::model::{Error, ModelManager, Result};


#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub password_clear: String,
}

#[derive(Deserialize)]
pub struct UserForNewPwd {
    pub old_pwd_clear: String,
    pub password_clear: String,
}

pub struct UserForInsert {
    pub username: String,
}

#[derive(Clone, FromRow, Debug)]
pub struct UserForLogin {
    pub id: i64,
    pub username: String,

    pub password: Option<String>,
    pub password_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Debug)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,

    pub token_salt: Uuid,
}

pub struct UserBmc {}

pub trait UserBy: for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForAuth {}
impl UserBy for UserForLogin {}

impl UserBmc {
    pub async fn create(
        _ctx: &Ctx,
        transaction_manager: &mut Transaction<'_, sqlx::Postgres>,
        user_c: UserForInsert, 
        ) -> Result<i64> {
        let (id, ) = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO \"user\" (username) VALUES ($1) RETURNING id"
            )
            .bind(user_c.username)
            .fetch_one(transaction_manager)
            .await?;
        Ok(id)
    }

    pub async fn get<U: UserBy>(
        _ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        fields: &str,
        entity: &'static str,
    ) -> Result<U> {
        let db = mm.db();

        sqlx::query_as::<_, U>(&format!(
            "SELECT {fields}
            FROM \"user\" WHERE id = $1"
        ))
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::ItemNotFound { entity, id })
    }

    pub async fn first_by_username<U: UserBy>(
        _ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
        fields: &str,
    ) -> Result<Option<U>> {
        let db = mm.db();

        let user = sqlx::query_as::<_, U>(&format!(
            "SELECT {fields}
            FROM \"user\" WHERE username = $1"
        ))
        .bind(username)
        .fetch_optional(db)
        .await?;

        Ok(user)
    }

    pub async fn update_password(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        password_clear: &str,
    ) -> Result<()> {
        let db = mm.db();

        let user: UserForLogin = UserBmc::get(
            ctx,
            mm,
            id,
            "id, username, password, password_salt, token_salt",
            "user for login",
        )
        .await?;

        let encrypted_password = encrypt_pwd(&EncryptContent {
            content: password_clear.to_string(),
            salt: user.password_salt.to_string(),
        })?;

        sqlx::query(
            "UPDATE \"user\" SET password = $1
            WHERE id = $2",
        )
        .bind(encrypted_password)
        .bind(id)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        let db = mm.db();

        let count = sqlx::query("DELETE FROM \"user\" WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            Err(Error::ItemNotFound { entity: "user", id })
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::_dev_utils;
    use crate::ctx;

    use super::*;
    use anyhow::Context;
    use anyhow::Ok;
    use anyhow::Result;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_first_ok_demo1() -> Result<()> {
        let mm = _dev_utils::dev_init_tests().await;
        let ctx = Ctx::root_ctx();
        let fx_username = "demo1";
        let fx_password = "welcome";

        let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username, "id, username")
            .await?
            .context("Should have demo1")?;

        UserBmc::update_password(&ctx, &mm, user.id, fx_password).await?;

        assert_eq!(user.username, fx_username);

        Ok(())
    }

    //#[serial]
    //#[tokio::test]
    //async fn test_create_ok() -> Result<()> {
    //    let mm = _dev_utils::dev_init_tests().await;
    //    let ctx = Ctx::root_ctx();
    //    let fx_username = "test_user_1";
    //    let fx_user_for_insert = UserForInsert {
    //        username: fx_username.to_string(),
    //    };
    //
    //    let id = UserBmc::create(&ctx, &mm, fx_user_for_insert).await?;
    //    let user: User = UserBmc::get(&ctx, &mm, id, &"id, username", &"user").await?;
    //
    //    assert_eq!(user.username, fx_username);
    //
    //    UserBmc::delete(&ctx, &mm, id).await?;
    //
    //    Ok(())
    //}
}
