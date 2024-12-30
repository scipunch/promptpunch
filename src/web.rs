use crate::llm::chat_gpt::ChatGpt;
use askama_axum::Template;
use axum::{
    extract::{Form, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct AppState {
    pub llm: ChatGpt,
}

pub fn init_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get::root))
        .route("/", post(post::generate))
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

mod post {
    use super::*;

    pub async fn generate(
        State(_state): State<AppState>,
        Form(form): Form<HashMap<String, String>>,
    ) -> impl IntoResponse {
        tracing::info!("Got form request {form:?}");
        "Generated prompt"
    }
}
