use crate::{web, Error, Result};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};



pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}


async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("-> {:<12} - api_login", "HANDLER");
    //TODO: implement real db/auth logic

    if payload.username != "demo1" ||  payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    } //it's just for the test
    
    //FIXME: implement real auth-token generation/signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
    //TODO: Set cookies

    //Create the sucess body
    let body = Json(json!({
    "result": {
        "sucess": true
    }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
