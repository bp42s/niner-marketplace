use axum::response::{Html, IntoResponse};

pub async fn handler_index() -> impl IntoResponse {
    match std::fs::read_to_string("views/index.ejs") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Error : {}", e)),
    }
}

pub async fn handler_about() -> impl IntoResponse {
    match std::fs::read_to_string("views/about.ejs") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Error : {}", e)),
    }
}

pub async fn handler_account() -> impl IntoResponse {
    match std::fs::read_to_string("views/account.ejs") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Error : {}", e)),
    }
}

pub async fn handler_chat() -> impl IntoResponse {
    match std::fs::read_to_string("views/chat.ejs") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Error : {}", e)),
    }
}

pub async fn handler_404() -> impl IntoResponse {
    match std::fs::read_to_string("views/404.ejs") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Error : {}", e)),
    }
}
