use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Completion, Prompt, PromptMessage};

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
    async fn complete_chat_gpt(
        &self,
        request: ChatGptCompletionRequest,
    ) -> anyhow::Result<ChatGptCompletionResponse> {
        let response = self
            .client
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

impl LlmProvider for ChatGpt {
    async fn complete_chat(&self, prompt: Prompt) -> anyhow::Result<Completion> {
        let mut messages = vec![];
        for message_request in prompt.messages {
            match message_request {
                crate::PromptMessageRequest::Message { body } => messages.push(body),
                crate::PromptMessageRequest::WaitCompletion => {
                    let request = ChatGptCompletionRequest {
                        model: self.model.to_string(),
                        messages: messages
                            .clone()
                            .into_iter()
                            .map(Into::into)
                            .collect::<Vec<_>>(),
                    };
                    let response = self.complete_chat_gpt(request).await?;
                }
            }
        }

        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatGptMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatGptCompletionRequest {
    model: String,
    messages: Vec<ChatGptCompletionRequest>,
}

impl From<PromptMessage> for ChatGptCompletionRequest {
    fn from(value: PromptMessage) -> Self {
        todo!()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChatGptCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
    pub completion_tokens_details: CompletionTokensDetails,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CompletionTokensDetails {
    pub reasoning_tokens: i64,
    pub accepted_prediction_tokens: i64,
    pub rejected_prediction_tokens: i64,
}

#[derive(Debug,  Deserialize)]
#[serde(rename_all = "camelCase")]
struct Choice {
    pub message: ChatGptMessage,
    pub logprobs: Value,
    pub finish_reason: String,
    pub index: i64,
}


#[derive(Debug, Default)]
pub enum ChatGptModel {
    #[default]
    Latest4o,
    Mini4o,
}

impl Display for ChatGptModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ChatGptModel::Latest4o => "gpt-4o-latest",
            ChatGptModel::Mini4o => "gpt-4o-mini",
        };
        write!(f, "{}", str)
    }
}
