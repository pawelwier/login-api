use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let http_client = httpc_test::new_client("http://localhost:8080")?;

    http_client.do_get("/hello?name=pawel").await?.print().await?;
    http_client.do_get("/hello2/ola").await?.print().await?;
    http_client.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}