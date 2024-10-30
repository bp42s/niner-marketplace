#[path = "forms/account.rs"]
pub mod nm_account;
#[path = "forms/common.rs"]
pub mod nm_common;
#[path = "forms/item.rs"]
pub mod nm_item;
#[path = "forms/listing.rs"]
pub mod nm_listing;
pub mod handlers;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    std::print!("NinerMarket - Server starting up...");

    // initialize the server and handling
    let router: Router = Router::new()
        .route("/", get(handlers::handler_index))
        .route("/about", get(handlers::handler_about))
        .route("/account", get(handlers::handler_account))
        .route("/chat", get(handlers::handler_chat))
        .fallback(handlers::handler_404);

    // startup - runs on https://localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    std::print!("NinerMarket - Server running on port 3000");
}
