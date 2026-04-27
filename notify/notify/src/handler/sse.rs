use std::{convert::Infallible, time::Duration};

use axum::{
    extract::State,
    response::{Sse, sse::Event},
};
use futures_util::{Stream, stream};
use tokio_stream::StreamExt;

use crate::{ArcAppState, model};

pub async fn handler(
    State(state): State<ArcAppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let state_clone = state.clone();

    let stream = stream::once(async move {
        let data = count_unread_notify(state_clone).await;
        Event::default().json_data(data).unwrap()
    })
    .map(Ok);

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(
                state.cfg.sse_keep_alive_interval as u64,
            ))
            .text("keep-alive-text"),
    )
}

async fn count_unread_notify(state: ArcAppState) -> i64 {
    model::Notification::count_unread(&state.pool)
        .await
        .unwrap_or_default()
}
