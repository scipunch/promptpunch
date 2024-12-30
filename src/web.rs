use askama_axum::Template;
use axum::{response::IntoResponse, routing::get, Router};

pub async fn init_router() -> Router {
    Router::new().route("/", get(get::root))
}

mod get {
    use super::*;

    #[derive(Template)]
    #[template(path = "root.html")]
    struct Root {}

    pub async fn root() -> impl IntoResponse {
        Root {}
    }
}
