use axum::{Router, routing::get};

use crate::{ArcAppState, handler};

pub fn init(state: ArcAppState) -> Router {
    Router::new().nest("/sse", sse_init(state))
}

fn sse_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/unread-notify-counter", get(handler::sse_handler))
        .with_state(state)
}
