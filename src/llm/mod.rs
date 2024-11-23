use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{message, Completion, Prompt, PromptMessage, Role};

pub mod chat_gpt;

pub trait LlmProvider {
    fn complete_chat(
        &self,
        prompt: Prompt,
    ) -> impl std::future::Future<Output = anyhow::Result<Completion>> + Send;
}
