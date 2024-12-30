#[tokio::main]
async fn main() {
    let router = promptpunch::web::init_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap()
}
