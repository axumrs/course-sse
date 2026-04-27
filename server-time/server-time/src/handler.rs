use std::{convert::Infallible, time::Duration};

use axum::response::{Html, Sse, sse::Event};
use futures_util::{Stream, stream};
use tokio_stream::StreamExt;

pub async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| {
        let now = chrono::Utc::now().to_rfc3339();
        Event::default().data(now)
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(3600))
            .text("keep-alive-text"),
    )
}

pub async fn index() -> Html<String> {
    let html = include_str!("../asset/index.html");

    Html(html.to_string())
}
