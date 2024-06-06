#![allow(unused)]

use std::net::SocketAddr;

use axum::{extract::Query, response::{Html, IntoResponse}, routing::get, Router, Server};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(handler_hello),
    );

    //Start serve

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-> Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();

    //Finish server
}
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("-> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("Carlito");
    Html("Hello <strong>{name}</strong>")
}


