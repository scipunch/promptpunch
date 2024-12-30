use crate::llm::chat_gpt::ChatGpt;
use askama_axum::Template;
use axum::{
    extract::{Form, State},
    http::StatusCode,
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

    #[derive(Template)]
    #[template(path = "error.html")]
    struct PromptPunchError {
        message: String,
    }

    pub async fn generate(
        State(_state): State<AppState>,
        Form(req): Form<GenerateRequest>,
    ) -> impl IntoResponse {
        tracing::info!("Got form request {req:?}");
        let GenerateRequest {
            prompt,
            placeholder_name,
            placeholder_value,
        } = req;

        if !prompt.contains(&placeholder_name) {
            return (
                StatusCode::BAD_REQUEST,
                PromptPunchError {
                    message: format!("Placeholder {} not found in the prompt", placeholder_name),
                },
            )
                .into_response();
        }
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        "Generated prompt".into_response()
    }
}
