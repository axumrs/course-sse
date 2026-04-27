use axum::{Router, routing::get};

use crate::{ArcAppState, handler};

pub fn init(_state: ArcAppState) -> Router {
    Router::new().nest("/sse", sse_init())
}

fn sse_init() -> Router {
    Router::new().route("/unread-notify-counter", get(handler::sse_handler))
}
