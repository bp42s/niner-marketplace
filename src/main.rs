#[path = "forms/account.rs"]
pub mod account;
#[path = "forms/common.rs"]
pub mod common;
#[path = "forms/item.rs"]
pub mod item;
#[path = "forms/listing.rs"]
pub mod listing;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

// handler functions - sends html/ejs content for requests
async fn handler_index() -> impl IntoResponse {
    match std::fs::read_to_string("views/index.ejs") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Error : {}", e)),
    }
}

async fn handler_about() -> impl IntoResponse {
    match std::fs::read_to_string("views/about.ejs") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Error : {}", e)),
    }
}

async fn handler_account() -> impl IntoResponse {
    match std::fs::read_to_string("views/account.ejs") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Error : {}", e)),
    }
}

async fn handler_404() -> impl IntoResponse {
    match std::fs::read_to_string("views/404.ejs") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Error : {}", e)),
    }
}

#[tokio::main]
async fn main() {
    // initialize the server and handling
    let router: Router = Router::new()
        .route("/", get(handler_index))
        .route("/about", get(handler_about))
        .route("/account", get(handler_account))
        .fallback(handler_404);

    // startup - runs on https://localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    std::print!("NinerMarket - Running on port 3000!");
}
