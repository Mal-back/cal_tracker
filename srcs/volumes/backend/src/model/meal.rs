use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::ctx::Ctx;

use crate::model::{Error, Result};

use super::ModelManager;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Meal {
    pub id: i64,
    pub owner: i64,
    pub name: String,
    pub kcal: i32,
    pub carbs: i32,
    pub proteins: i32,
    pub lipids: i32,
}

#[derive(Deserialize)]
pub struct MealForCreate {
    pub name: String,
    pub kcal: i32,
    pub carbs: i32,
    pub proteins: i32,
    pub lipids: i32,
}

#[derive(Deserialize)]
pub struct MealForUpdate {
    pub name: Option<String>,
    pub kcal: Option<i32>,
    pub carbs: Option<i32>,
    pub lipids: Option<i32>,
    pub proteins: Option<i32>,
}

pub struct MealBmc {}

impl MealBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, meal_c: MealForCreate) -> Result<i64> {
        let db = mm.db();

        let (id, ) = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO meal (name, kcal, carbs, lipids, proteins, owner) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"
            )
            .bind(meal_c.name)
            .bind(meal_c.kcal)
            .bind(meal_c.carbs)
            .bind(meal_c.lipids)
            .bind(meal_c.proteins)
            .bind(ctx.user_id())
            .fetch_one(db)
            .await?;
        Ok(id)
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Meal> {
        let db = mm.db();

        sqlx::query_as::<_, Meal>(
            "SELECT id, name, kcal, carbs, lipids, proteins, owner
            FROM meal WHERE id = $1 and owner = $2",
        )
        .bind(id)
        .bind(ctx.user_id())
        .fetch_optional(db)
        .await?
        .ok_or(Error::ItemNotFound { entity: "meal", id })
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Meal>> {
        let db = mm.db();

        let meals = sqlx::query_as(
            "SELECT id, name, kcal, carbs, lipids, proteins, owner
            FROM meal WHERE owner = $1 ORDER BY id",
        )
        .bind(ctx.user_id())
        .fetch_all(db)
        .await?;

        Ok(meals)
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        meal_u: MealForUpdate,
    ) -> Result<()> {
        let meal_to_update = MealBmc::get(ctx, mm, id).await?;

        let db = mm.db();

        let count = sqlx::query(
            "UPDATE meal SET name = $1, kcal = $2, carbs = $3, lipids = $4, proteins = $5
            WHERE id = $6 AND owner = $7",
        )
        .bind(meal_u.name.unwrap_or(meal_to_update.name))
        .bind(meal_u.kcal.unwrap_or(meal_to_update.kcal))
        .bind(meal_u.carbs.unwrap_or(meal_to_update.carbs))
        .bind(meal_u.lipids.unwrap_or(meal_to_update.lipids))
        .bind(meal_u.proteins.unwrap_or(meal_to_update.proteins))
        .bind(id)
        .bind(ctx.user_id())
        .execute(db)
        .await?
        .rows_affected();

        if count == 0 {
            Err(Error::ItemNotFound { entity: "meal", id })
        } else {
            Ok(())
        }
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        let db = mm.db();

        let count = sqlx::query("DELETE FROM meal WHERE id = $1 AND owner = $2")
            .bind(id)
            .bind(ctx.user_id())
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            Err(Error::ItemNotFound { entity: "meal", id })
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use std::{thread::sleep, time::Duration};

    use crate::{_dev_utils::dev_init_tests, ctx};

    use super::*;
    use anyhow::{Ok, Result};
    use serial_test::serial;
    use sqlx::Postgres;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = dev_init_tests().await;
        let ctx = Ctx::demo1_ctx();

        let fixture_name = "test_create_ok title";
        let fixture_kcal = 155;
        let fixture_carbs = 15;
        let fixture_lipids = 28;
        let fixture_proteins = 42;

        let meal_c = MealForCreate {
            name: fixture_name.to_string(),
            kcal: fixture_kcal,
            carbs: fixture_carbs,
            lipids: fixture_lipids,
            proteins: fixture_proteins,
        };

        let id = MealBmc::create(&ctx, &mm, meal_c).await?;

        let meal = MealBmc::get(&ctx, &mm, id).await?;

        assert_eq!(fixture_name, meal.name);
        assert_eq!(fixture_kcal, meal.kcal);
        assert_eq!(fixture_carbs, meal.carbs);
        assert_eq!(fixture_lipids, meal.lipids);
        assert_eq!(fixture_proteins, meal.proteins);

        MealBmc::delete(&ctx, &mm, id);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_not_found() -> Result<()> {
        let mm = dev_init_tests().await;
        let ctx = Ctx::root_ctx();

        let res = MealBmc::get(&ctx, &mm, 100).await;

        //match res {
        //    Err(e) => println!("->> {:?}", e),
        //    _ => println!("->> OK"),
        //};

        assert!(
            matches!(
                res,
                Err(Error::ItemNotFound {
                    entity: "meal",
                    id: 100
                })
            ),
            "Not found info desn't match"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_not_found() -> Result<()> {
        let mm = dev_init_tests().await;
        let ctx = Ctx::root_ctx();

        let res = MealBmc::delete(&ctx, &mm, 100).await;

        //match res {
        //    Err(e) => println!("->> {:?}", e),
        //    _ => println!("->> OK"),
        //};

        assert!(
            matches!(
                res,
                Err(Error::ItemNotFound {
                    entity: "meal",
                    id: 100
                })
            ),
            "Not found info desn't match"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        // TODO : Refactor me
        let mm = dev_init_tests().await;
        let ctx = Ctx::demo1_ctx();

        let fixture_kcal = 155;
        let fixture_carbs = 15;
        let fixture_lipids = 28;
        let fixture_proteins = 42;

        let meal_c_first = MealForCreate {
            name: "test_list_ok title - Task1".to_string(),
            kcal: fixture_kcal,
            carbs: fixture_carbs,
            lipids: fixture_lipids,
            proteins: fixture_proteins,
        };

        let meal_c_second = MealForCreate {
            name: "test_list_ok title - Task2".to_string(),
            kcal: fixture_kcal,
            carbs: fixture_carbs,
            lipids: fixture_lipids,
            proteins: fixture_proteins,
        };

        let meal_c_third = MealForCreate {
            name: "test_list_ok title - Task3".to_string(),
            kcal: fixture_kcal,
            carbs: fixture_carbs,
            lipids: fixture_lipids,
            proteins: fixture_proteins,
        };

        let id = MealBmc::create(&ctx, &mm, meal_c_first).await?;
        let id = MealBmc::create(&ctx, &mm, meal_c_second).await?;
        let id = MealBmc::create(&ctx, &mm, meal_c_third).await?;

        let tasks = MealBmc::list(&ctx, &mm).await?;
        let filtered_tasks: Vec<Meal> = tasks
            .into_iter()
            .filter(|t| t.name.starts_with("test_list_ok title"))
            .collect();

        assert_eq!(filtered_tasks.len(), 3);

        Ok(())
    }
}
