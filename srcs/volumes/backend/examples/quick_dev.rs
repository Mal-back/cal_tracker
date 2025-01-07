use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello/").await?.print().await?;

    let auth_request = hc.do_post(
        "/api/login/",
        json!({
            "username": "val",
            "password": "aaa"
        }),
    );
    auth_request.await?.print().await?;

    let create_req = hc.do_post(
        "/api/tickets/",
        json!({
            "title": "My first Ticket"
        }),
    );
    create_req.await?.print().await?;

    let create_req = hc.do_post(
        "/api/tickets/",
        json!({
            "title": "My second Ticket"
        }),
    );
    create_req.await?.print().await?;
    hc.do_get("/api/tickets/").await?.print().await?;

    hc.do_delete("/api/tickets/1").await?.print().await?;
    hc.do_delete("/api/tickets/1").await?.print().await?;
    hc.do_get("/api/tickets/").await?.print().await?;
    Ok(())
}
