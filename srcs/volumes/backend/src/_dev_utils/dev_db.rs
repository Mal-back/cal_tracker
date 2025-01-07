use std::{fs, path::PathBuf, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use tracing::{debug, info};

use crate::{
    ctx::Ctx,
    model::{
        user::user::{User, UserBmc},
        ModelManager,
    },
};

type Db = Pool<Postgres>;

const PG_ROOT_DEV_URL: &str = "postgres://postgres:welcome@db:5432/postgres";
const PG_USER_DEV_URL: &str = "postgres://app_user:dev_only@db:5432/app_db";

const SQL_RECREATE: &str = "migrations/dev_initial/0000-recreate.sql";
const SQL_DIR: &str = "migrations/dev_initial/";

const DEMO_PWD: &str = "Welcome";

pub async fn exec_reset() -> Result<(), Box<dyn std::error::Error>> {
    {
        let root_db = new_db_pool(PG_ROOT_DEV_URL).await?;
        pexec(&root_db, SQL_RECREATE).await?;
    }

    let user_db = new_db_pool(PG_USER_DEV_URL).await?;

    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    paths.sort();

    for path in paths {
        if let Some(path) = path.to_str() {
            let path = path.replace('\\', "/");
            if path != SQL_RECREATE && path.ends_with(".sql") {
                pexec(&user_db, &path).await?;
            }
        }
    }

    // Set demo1 pwd
    let mm = ModelManager::new().await?;
    let ctx = Ctx::root_ctx();

    let demo1_user: User = UserBmc::first_by_username(&ctx, &mm, "demo1", "id, username")
        .await?
        .unwrap();

    UserBmc::update_password(&ctx, &mm, demo1_user.id, DEMO_PWD)
        .await
        .unwrap();
    debug!("{:<12} - setting demo1 password", "DEV_ONLY");

    Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    info!("{:<12} - pexec file {file}", "DEV_ONLY");

    let content = fs::read_to_string(file)?;

    let queries = content.split_inclusive(";");

    for query in queries {
        debug!("{:<12} - executing query {query}", "DEV_ONLY");

        // Way I did it myself
        //db.execute(query).await?;

        // The good way, allowing for latter prepared statement
        sqlx::query(query).execute(db).await?;
    }

    Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(1000))
        .max_connections(5)
        .connect(db_con_url)
        .await
}
