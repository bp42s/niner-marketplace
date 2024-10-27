#[path = "forms/account.rs"]
mod account;
#[path = "forms/common.rs"]
mod common;
#[path = "forms/image.rs"]
mod image;
#[path = "forms/listing.rs"]
mod listing;

use axum::{response::Html, routing::get, Router};

async fn handle_page_home() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn handle_page_about() -> Html<&'static str> {
    Html("<p>about page</p>")
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let _app: Router = Router::new()
        .route("/", get(handle_page_home))
        .route("/about", get(handle_page_about));

    // run it with hyper on localhost:3000
    let router = Router::new().route("/", get(handle_page_home));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
