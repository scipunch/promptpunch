use async_trait::async_trait;
use std::borrow::Borrow;

use crate::{Completion, Prompt};

pub mod chat_gpt;

#[async_trait]
pub trait LlmProvider {
    async fn complete_chat(
        &self,
        prompt: impl Borrow<Prompt> + std::marker::Send,
    ) -> anyhow::Result<Completion>;
}
