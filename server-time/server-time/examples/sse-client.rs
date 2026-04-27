use eventsource_stream::Eventsource;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut cli = reqwest::Client::new()
        .get("http://127.0.0.1:9527/api/sse")
        .send()
        .await?
        .bytes_stream()
        .eventsource();

    while let Some(event) = cli.next().await {
        match event {
            Ok(event) => println!("收到事件：{:?}", event),
            Err(err) => eprintln!("发生错误：{:?}", err),
        }
    }

    Ok(())
}
