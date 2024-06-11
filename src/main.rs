#![allow(unused)]
pub use self::error::{Error, Result};

mod error;
mod web;
mod model;


//use crate::ctx::Ctx;
//use crate::log::log_request;
use crate::model::ModelController;

use axum::extract::{Path, Query};
use axum::http::{Method, Uri};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Json, Router};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
//use uuid::Uuid;J

#[tokio::main]
async fn main() -> Result <()> {
    //Initialize ModelController
    let mc = ModelController::new().await?;

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc.clone()) )
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    //Layers are executed from bottom to top

    //Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-> Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    //Finish server

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();

    res
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    //Trabalha com parâmetros e estes estão definidos no código, o que não é essencial
    println!("-> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello, <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    //Recebe os dados via http/json
    println!("-> {:<12} - handler_hello - {name:?}", "HANDLER");

    Html(format!("Hello, <strong>{name}</strong>"))
}
