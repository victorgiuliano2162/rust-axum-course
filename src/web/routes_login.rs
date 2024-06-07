use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{Error, Result};

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("-> {:<12} - api_login", "HANDLER");
    todo!()
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
