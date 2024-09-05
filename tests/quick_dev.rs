use anyhow::{Ok, Result};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/hello?name=Bob").await?.print().await?;
    hc.do_get("/hello2/Mike").await?.print().await?;
    hc.do_get("/src/index.html").await?.print().await?;

    Ok(())
}
