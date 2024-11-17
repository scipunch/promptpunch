use std::fmt::Display;

use serde::Serialize;

use crate::{Completion, Prompt};

pub trait LlmProvider {
    fn complete_chat(
        &self,
        prompt: Prompt,
    ) -> impl std::future::Future<Output = anyhow::Result<Completion>> + Send;
}

pub struct ChatGpt {
    api_token: String,
    model: ChatGptModel,
    client: reqwest::Client,
}

impl ChatGpt {
    pub fn from_env() -> Self {
        let api_token = std::env::var("OPENAI_API_TOKEN").expect("Set OPENAI_API_TOKEN");
        Self {
            api_token,
            model: ChatGptModel::default(),
            client: reqwest::Client::new(),
        }
    }
}

impl LlmProvider for ChatGpt {
    async fn complete_chat(&self, prompt: Prompt) -> anyhow::Result<Completion> {
        let request: ChatGptCompletionRequest = prompt.into();
        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_token))
            .body(serde_json::to_string(&request)?)
            .send()
            .await?
            .error_for_status()?;
        todo!()
    }
}

#[derive(Serialize)]
struct ChatGptMessageRequest {
    role: String,
    content: String
}

#[derive(Serialize)]
struct ChatGptCompletionRequest {
    model: String,
    messages: Vec<ChatGptMessageRequest>,
}

impl From<Prompt> for ChatGptCompletionRequest {
    fn from(value: Prompt) -> Self {
        todo!()
    }
}

#[derive(Debug, Default)]
pub enum ChatGptModel {
    #[default]
    _4oLatest,
    _4oMini,
}

impl Display for ChatGptModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ChatGptModel::_4oLatest => "gpt-4o-latest",
            ChatGptModel::_4oMini => "gpt-4o-mini",
        };
        write!(f, "{}", str)
    }
}
