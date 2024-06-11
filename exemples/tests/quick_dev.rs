#![allow(unused)]
pub mod quick_dev;

use anyhow::Result;
//Lib for tests


#[tokio::main]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello2/Cargo").await?.print().await?;
    //hc.do_get("/src/main.rs").await?.print().await?;



    Ok(())
}