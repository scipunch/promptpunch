use crate::{
    llm::{chat_gpt::ChatGpt, LlmProvider},
    prelude::*,
    prompt::InjectableData,
};
use askama_axum::Template;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Clone)]
pub struct AppState {
    pub llm: ChatGpt,
    pub prompt_info: PromptInfo,
}

#[derive(Clone)]
pub struct PromptInfo {
    pub prompt_markdown: String,
    pub placeholder_name: String,
}

impl Default for PromptInfo {
    fn default() -> Self {
        Self {
            prompt_markdown: r#"# System
Some system prompt

# User
Some user input that uses {placheloder_name}

# Assistant"#
                .to_string(),
            placeholder_name: "{placeholder_name}".to_string(),
        }
    }
}

pub fn init_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get::root))
        .route("/", post(post::generate))
        .with_state(state)
}

mod get {
    use super::*;

    #[derive(Template)]
    #[template(path = "root.html")]
    struct Root {
        prompt_info: PromptInfo,
    }

    pub async fn root(State(state): State<AppState>) -> impl IntoResponse {
        Root {
            prompt_info: state.prompt_info,
        }
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

    impl PromptPunchError {
        pub fn new(message: impl Display) -> Self {
            Self {
                message: message.to_string(),
            }
        }
    }

    pub async fn generate(
        State(state): State<AppState>,
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
                PromptPunchError::new(format!(
                    "Placeholder {} not found in the prompt",
                    placeholder_name
                )),
            )
                .into_response();
        }
        let injectable_data = [InjectableData::new(placeholder_name, placeholder_value)];

        let Ok(messages) = crate::prompt::read_markdown_prompt(prompt.lines(), &injectable_data)
        else {
            return (
                StatusCode::BAD_REQUEST,
                PromptPunchError::new("Failed to read prompt"),
            )
                .into_response();
        };

        let prompt = PromptBuilder::default().messages(messages).build().unwrap();

        let completion = match state.llm.complete_chat(prompt).await {
            Ok(r) => r,
            Err(err) => {
                let msg = format!("Failed to get completion from LLM with {err:?}");
                tracing::error!(msg);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    PromptPunchError::new(msg),
                )
                    .into_response();
            }
        };

        completion
            .messages
            .into_iter()
            .filter(|msg| msg.role == Role::Assistant)
            .enumerate()
            .map(|(idx, msg)| format!("{} :::: {}\n\n", idx + 1, msg.content))
            .collect::<Vec<_>>()
            .join("\n")
            .into_response()
    }
}
