use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Transaction};

use crate::{ctx::Ctx, model::{Error, ModelManager, Result}};


pub struct PublicUserForCreate {
    pub owner: i64,
    pub age: i32,
    pub size_cm: i32,
    pub weight: f32,
}

#[derive(Debug, FromRow, Serialize)]
pub struct PublicUser {
    pub id: i64,
    pub owner: i64,
    pub age: i32,
    pub size_cm: i32,
    pub weight: f32,
}

#[derive(Deserialize)]
pub struct PublicUserForUpdate {
    pub age: Option<i32>,
    pub size_cm: Option<i32>,
    pub weight: Option<f32>,
}

pub struct PublicUserBmc {}

impl PublicUserBmc {
    pub async fn create(_ctx: &Ctx, transaction_manager: &mut Transaction<'_, sqlx::Postgres>, pub_user_c: PublicUserForCreate) -> Result<i64> {

        let (id, ) = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO public_user (owner, age, size_cm, weight) VALUES ($1, $2, $3, $4) RETURNING id"
            )
            .bind(pub_user_c.owner)
            .bind(pub_user_c.age)
            .bind(pub_user_c.size_cm)
            .bind(pub_user_c.weight)
            .fetch_one(transaction_manager)
            .await?;
        Ok(id)
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<PublicUser> {
        let db = mm.db();


        sqlx::query_as::<_, PublicUser>("SELECT id, owner, age, size_cm, weight FROM public_user WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::ItemNotFound { entity: "public_user", id })
    }

    pub async fn first_by_owner(ctx: &Ctx, mm: &ModelManager) -> Result<PublicUser> {
        let db = mm.db();

        let id = ctx.user_id();


        sqlx::query_as::<_, PublicUser>("SELECT id, owner, age, size_cm, weight FROM public_user WHERE owner = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::PublicUserNotFound { owner_id: id })

    } 

    pub async fn update(ctx: &Ctx, mm: &ModelManager, pub_user_u: PublicUserForUpdate) -> Result<()> {
        let db = mm.db();

        let id = ctx.user_id();

        let current_pub_user_data = PublicUserBmc::first_by_owner(ctx, mm)
            .await?;

        let count = sqlx::query(
            "UPDATE public_user SET age = $1, size_cm = $2, weight = $3
            WHERE id = $6",
        )
        .bind(pub_user_u.age.unwrap_or(current_pub_user_data.age))
        .bind(pub_user_u.size_cm.unwrap_or(current_pub_user_data.size_cm))
        .bind(pub_user_u.weight.unwrap_or(current_pub_user_data.weight))
        .bind(current_pub_user_data.id)
        .execute(db)
        .await?
        .rows_affected();

        if count == 0 {
            Err(Error::ItemNotFound { entity: "public_user", id })
        } else {
            Ok(())
        }
    }
}
