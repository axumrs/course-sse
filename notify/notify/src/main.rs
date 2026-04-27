use notify::{AppState, config::Config, init};
use tokio::net::TcpListener;

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .thread_name("notify-sse")
        .enable_all()
        .build()?;
    rt.block_on(async_main())
}

async fn async_main() -> anyhow::Result<()> {
    init::log();

    let cfg = Config::from_env()?;

    let listener = TcpListener::bind(&cfg.web_addr).await?;

    let state = AppState::new_arc(cfg);

    let app = init::router(state);

    tracing::info!("🚀 Listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}
