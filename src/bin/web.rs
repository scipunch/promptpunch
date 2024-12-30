use promptpunch::{llm::chat_gpt::ChatGpt, web::AppState};

#[tokio::main]
async fn main() {
    let state = AppState {
        llm: ChatGpt::from_env(),
    };
    let router = promptpunch::web::init_router().with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap()
}
