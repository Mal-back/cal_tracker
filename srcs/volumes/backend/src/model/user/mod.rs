use public_user::{PublicUserBmc, PublicUserForCreate};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Transaction};
use user::{User, UserBmc, UserForCreate, UserForInsert};


use crate::{ctx::Ctx, model::Error};

use super::{ModelManager, Result};

pub mod user;
pub mod public_user;

#[derive(Deserialize)]
pub struct FullUserForCreate {
    pub username: String,
    pub password_clear: String,
    pub age: i32,
    pub size_cm: i32,
    pub weight: f32,
}

#[derive(Serialize, FromRow)]
pub struct FullUser {
    pub id: i64,
    pub username: String,
    pub age: i32,
    pub size_cm: i32,
    pub weight: f32,
}

pub struct FullUserBmc {}

impl FullUserBmc {
    pub async fn create_new_user(ctx: &Ctx, mm: &ModelManager, full_user_c: &FullUserForCreate) -> Result<i64> {

        let mut transaction_manager = mm.db().begin().await?;
        let user_c = UserForInsert {
            username: full_user_c.username.clone()
        };

        let auth_user_id = UserBmc::create(&ctx, &mut transaction_manager, user_c).await?;

        let pub_user_c = PublicUserForCreate {
            owner: auth_user_id,
            age: full_user_c.age,
            size_cm : full_user_c.size_cm,
            weight: full_user_c.weight
        };

        let _public_user_id = PublicUserBmc::create(ctx, & mut transaction_manager, pub_user_c).await?;

        transaction_manager.commit().await?; 
        Ok(auth_user_id)
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<FullUser> {
        let db = mm.db();

        sqlx::query_as::<_, FullUser>("SELECT \"user\".id, username, age, size_cm, weight FROM \"user\"
             JOIN public_user ON public_user.owner = \"user\".id WHERE \"user\".id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::ItemNotFound { entity: "full user", id})
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use serial_test::serial;

    use crate::{_dev_utils::dev_init_tests, ctx::Ctx};

    use super::{user::UserBmc, FullUserBmc, FullUserForCreate};

    #[serial]
    #[tokio::test]
    async fn test_create_public_user_ok() -> Result<()> {
        let mm = dev_init_tests().await;
        let ctx = Ctx::root_ctx();

        let fixture_user = FullUserForCreate {
            username: "val".to_string(),
            password_clear: "Welcome".to_string(),
            age: 44,
            size_cm: 182,
            weight: 51.5
        };

        let id = FullUserBmc::create_new_user(&ctx, &mm, &fixture_user).await?;

        let check_user = FullUserBmc::get(&ctx, &mm, id).await?;

        assert_eq!(fixture_user.username, check_user.username);
        assert_eq!(fixture_user.age, check_user.age);
        assert_eq!(fixture_user.size_cm, check_user.size_cm);
        assert_eq!(fixture_user.weight, check_user.weight);

        UserBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }
}
