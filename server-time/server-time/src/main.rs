use axum::{Router, routing::get};
use tokio::net::TcpListener;

mod handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9527").await?;

    let app = Router::new()
        .route("/", get(handler::index))
        .route("/api/sse", get(handler::sse_handler));

    axum::serve(listener, app).await?;

    Ok(())
}
