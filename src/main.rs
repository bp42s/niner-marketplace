#[path = "forms/account.rs"]
mod account;
#[path = "forms/common.rs"]
mod common;
#[path = "forms/image.rs"]
mod image;
#[path = "forms/listing.rs"]
mod listing;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
#[allow(unused_imports)]
use std::{fs, net::SocketAddr};

// send html/ejs content for: index
async fn handle_page_index() -> impl IntoResponse {
    match fs::read_to_string("views/index.ejs") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Error : {}", e)),
    }
}

// send html/ejs content for: about
async fn handle_page_about() -> impl IntoResponse {
    match fs::read_to_string("views/about.ejs") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Error : {}", e)),
    }
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let router: Router = Router::new()
        .route("/", get(handle_page_index))
        .route("/about", get(handle_page_about));
    // run on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
