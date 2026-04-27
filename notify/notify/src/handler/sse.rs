use std::{convert::Infallible, time::Duration};

use axum::response::{Sse, sse::Event};
use futures_util::{Stream, stream};
use tokio_stream::StreamExt;

pub async fn handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::once(async move { Event::default().json_data(12345).unwrap() }).map(Ok);

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
