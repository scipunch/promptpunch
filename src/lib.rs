use std::borrow::Borrow;

pub trait LlmProvider {
    fn complete_chat(prompt: Prompt) -> impl std::future::Future<Output = anyhow::Result<Completion>> + Send;
}

pub struct Prompt {
    messages: Vec<PromptMessage>
}

pub struct PromptMessage {
    role: Role,
    content: String
}

pub enum Role {
    System,
    User,
    Assistant
}

pub struct Completion {
    messages: Vec<PromptMessage>
}

async fn complete(
    llm_provider: impl LlmProvider,
    prompt: impl Borrow<Prompt>,
) -> anyhow::Result<Completion> {
    todo!()
}
