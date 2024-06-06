#![allow(unused)]

use anyhow::Result;
//Lib for tests

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name-Jen").await?.print().await?;

    Ok(())
}