use super::LlmProvider;
use std::{borrow::Borrow, fmt::Display};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Completion, Prompt, PromptMessage, Role};

pub struct ChatGpt {
    api_token: String,
    model: ChatGptModel,
    client: reqwest::Client,
}

impl ChatGpt {
    pub fn from_env() -> Self {
        let api_token = std::env::var("OPENAI_API_KEY").expect("Set OPENAI_API_TOKEN");
        Self {
            api_token,
            model: ChatGptModel::default(),
            client: reqwest::Client::new(),
        }
    }

    async fn make_completion(&self, request: &mut ChatGptCompletionRequest) -> anyhow::Result<()> {
        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_token))
            .body(dbg!(serde_json::to_string(&request)?))
            .send()
            .await?
            .json::<ChatGptCompletionResponse>()
            .await?;

        if let Some(choice) = response.choices.into_iter().next() {
            if choice.message.role == "assistant" {
                request.messages.push(choice.message);
            }
        }
        
        Ok(())
    }
}

impl LlmProvider for ChatGpt {
    async fn complete_chat(
        &self,
        prompt: impl Borrow<Prompt> + std::marker::Send,
    ) -> anyhow::Result<Completion> {
        let mut request = ChatGptCompletionRequest {
            model: self.model.to_string(),
            messages: vec![],
            temperature: prompt.borrow().temperature,
        };

        for message_request in &prompt.borrow().messages {
            match message_request {
                crate::PromptMessageRequest::Message { body } => {
                    request.messages.push(body.clone().into())
                }
                crate::PromptMessageRequest::WaitCompletion => {
                    self.make_completion(&mut request).await?;
                }
            }
        }
        self.make_completion(&mut request).await?;

        Ok(Completion {
            messages: request
                .messages
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatGptMessage {
    role: String,
    content: String,
}

impl From<PromptMessage> for ChatGptMessage {
    fn from(value: PromptMessage) -> Self {
        let role = match value.role {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        }
        .to_string();

        ChatGptMessage {
            role,
            content: value.content,
        }
    }
}

impl From<ChatGptMessage> for PromptMessage {
    fn from(value: ChatGptMessage) -> Self {
        let role = match value.role.as_str() {
            "assistant" => Role::Assistant,
            "user" => Role::User,
            "system" => Role::System,
            unknown => panic!("Got unknown={} message role", unknown),
        };
        PromptMessage {
            role,
            content: value.content,
        }
    }
}

#[derive(Debug, Serialize)]
struct ChatGptCompletionRequest {
    model: String,
    messages: Vec<ChatGptMessage>,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChatGptCompletionResponse {
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    object: String,
    #[allow(dead_code)]
    created: i64,
    #[allow(dead_code)]
    model: String,
    #[allow(dead_code)]
    usage: Usage,
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Usage {
    #[allow(dead_code)]
    prompt_tokens: Option<i64>,
    #[allow(dead_code)]
    completion_tokens: Option<i64>,
    #[allow(dead_code)]
    total_tokens: Option<i64>,
    #[allow(dead_code)]
    completion_tokens_details: Option<CompletionTokensDetails>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CompletionTokensDetails {
    #[allow(dead_code)]
    reasoning_tokens: i64,
    #[allow(dead_code)]
    accepted_prediction_tokens: i64,
    #[allow(dead_code)]
    rejected_prediction_tokens: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Choice {
    message: ChatGptMessage,
    #[allow(dead_code)]
    logprobs: Value,
    #[allow(dead_code)]
    finish_reason: Option<String>,
    #[allow(dead_code)]
    index: i64,
}

#[derive(Debug, Default)]
pub enum ChatGptModel {
    Latest4o,
    #[default]
    Mini4o,
}

impl Display for ChatGptModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ChatGptModel::Latest4o => "chatgpt-4o-latest ",
            ChatGptModel::Mini4o => "gpt-4o-mini",
        };
        write!(f, "{}", str)
    }
}
