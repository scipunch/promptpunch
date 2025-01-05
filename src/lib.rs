use anyhow::Context;
use derive_builder::Builder;

pub mod llm;
pub mod prompt;

#[cfg(feature = "web")]
pub mod web;

#[derive(Debug, Builder, Clone)]
#[builder(setter(into))]
pub struct Prompt {
    pub messages: Vec<PromptMessageRequest>,
    #[builder(default = "0.3")]
    pub temperature: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromptMessageRequest {
    Message { body: PromptMessage },
    WaitCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug)]
pub struct Completion {
    pub messages: Vec<PromptMessage>,
    pub user_tokens: usize,
    pub assistant_tokens: usize,
}
impl Completion {
    pub fn last_assistant_response(&self) -> anyhow::Result<String> {
        let last_message = self
            .messages
            .iter()
            .filter(|msg| msg.role == Role::Assistant)
            .last()
            .context("There is no assistant message")?;
        Ok(last_message.content.clone())
    }
}

pub mod prelude {
    pub use crate::{
        llm::chat_gpt::ChatGpt, llm::LlmProvider, message, prompt::InjectableData, Prompt,
        PromptBuilder, PromptMessage, PromptMessageRequest, Role,
    };
}

pub mod message {
    pub use crate::PromptMessageRequest;

    #[macro_export]
    macro_rules! system {
        ($content:expr) => {
            PromptMessageRequest::Message {
                body: PromptMessage {
                    role: Role::System,
                    content: $content.to_string(),
                },
            }
        };
    }

    #[macro_export]
    macro_rules! user {
        ($content:expr) => {
            PromptMessageRequest::Message {
                body: PromptMessage {
                    role: Role::User,
                    content: $content.to_string(),
                },
            }
        };
    }

    #[macro_export]
    macro_rules! complete {
        () => {
            PromptMessageRequest::WaitCompletion
        };
    }

    pub use complete;
    pub use system;
    pub use user;
}
