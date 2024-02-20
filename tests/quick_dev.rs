use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let http_client = httpc_test::new_client("http://localhost:5050")?;

    /*
        http_client.do_get("/hello?name=pawel").await?.print().await?;
        http_client.do_get("/hello2/ola").await?.print().await?;
        http_client.do_get("/src/main.rs").await?.print().await?;
    */

    let req_login = http_client.do_post(
        "/api/login", 
        json!({
            "username": "demo",
            "pwd": "pass"
        })
    );
    // req_login.await?.print().await?;

    let req_create_ticket = http_client.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket Test 1"
        })
    );

    req_create_ticket.await?.print().await?;

    // http_client.do_delete("/api/tickets/1").await?.print().await?;

    http_client.do_get("/api/tickets").await?.print().await?;

    Ok(())
}