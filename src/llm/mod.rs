use std::borrow::Borrow;

use crate::{Completion, Prompt};

pub mod chat_gpt;

pub trait LlmProvider {
    fn complete_chat(
        &self,
        prompt: impl Borrow<Prompt> + std::marker::Send,
    ) -> impl std::future::Future<Output = anyhow::Result<Completion>> + Send;
}
