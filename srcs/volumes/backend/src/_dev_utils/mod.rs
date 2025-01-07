use dev_db::exec_reset;
use tokio::sync::OnceCell;
use tracing::info;

use crate::model::ModelManager;

mod dev_db;

pub async fn dev_init_db() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - calling exec reset", "DEV_ONLY");
        exec_reset().await.unwrap();
    })
    .await;
}

pub async fn dev_init_tests() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT
        .get_or_init(|| async {
            dev_init_db().await;
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}
