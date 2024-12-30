use crate::llm::chat_gpt::ChatGpt;
use askama_axum::Template;
use axum::{
    extract::{Form, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;

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

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GenerateRequest {
        prompt: String,
        placeholder_name: String,
        placeholder_value: String,
    }

    pub async fn generate(
        State(_state): State<AppState>,
        Form(req): Form<GenerateRequest>,
    ) -> impl IntoResponse {
        tracing::info!("Got form request {req:?}");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        "Generated prompt"
    }
}
