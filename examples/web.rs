use promptpunch::{
    llm::chat_gpt::ChatGpt,
    web::{AppState, PromptInfo},
};
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let state = AppState {
        llm: ChatGpt::from_env(),
        prompt_info: PromptInfo::default(),
    };
    let router = promptpunch::web::init_router().with_state(state);
    let host = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    tracing::info!("Starting app on http://{host}");
    axum::serve(listener, router).await.unwrap()
}
