use derive_builder::Builder;

pub mod llm;

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Prompt {
    messages: Vec<PromptMessageRequest>,
    temperature: f32
}

#[derive(Debug, Clone)]
pub enum PromptMessageRequest {
    Message { body: PromptMessage },
    WaitCompletion,
}

#[derive(Debug, Clone)]
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
}

pub mod prelude {
    pub use crate::{
        complete, llm::chat_gpt::ChatGpt, llm::LlmProvider, message, Prompt, PromptBuilder,
        PromptMessage, PromptMessageRequest, Role,
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
