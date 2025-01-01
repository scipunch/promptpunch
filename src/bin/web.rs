use promptpunch::{
    llm::chat_gpt::ChatGpt,
    web::{AppState, PromptInfo},
};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let state = AppState {
        llm: ChatGpt::from_env(),
        prompt_info: PromptInfo::default(),
    };
    let router = promptpunch::web::init_router().with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Starting app");
    axum::serve(listener, router).await.unwrap()
}
