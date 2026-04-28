use axum::{
    Router,
    routing::{get, post},
};

use crate::{ArcAppState, handler};

pub fn init(state: ArcAppState) -> Router {
    Router::new()
        .nest("/sse", sse_init(state.clone()))
        .nest("/api", api_init(state))
}

fn sse_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/unread-notify-counter", get(handler::sse_handler))
        .with_state(state)
}

fn api_init(state: ArcAppState) -> Router {
    Router::new()
        .route(
            "/notify",
            post(handler::api::list).put(handler::api::make_all_read),
        )
        .route(
            "/notify/{id}",
            get(handler::api::find)
                .put(handler::api::make_read)
                .delete(handler::api::del),
        )
        .with_state(state)
}
