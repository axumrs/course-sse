use notify::{AppState, ArcAppState, config::Config, init, model};
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

    let pool = init::datbase(&cfg.database_url, cfg.database_max_conns).await?;

    let state = AppState::new_arc(cfg, pool);

    let mut create_notify_handler = tokio::spawn(create_notify(state.clone()));
    let mut clean_notify_handler = tokio::spawn(clean_notify(state.clone()));

    let app = init::router(state);

    let mut web_server = tokio::spawn(async move {
        tracing::info!("🚀 Listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await
    });

    tokio::select! {
        _ = &mut create_notify_handler => {
            tracing::info!("创建通知任务退出");
            clean_notify_handler.abort();
            web_server.abort();
        }
        _ = &mut clean_notify_handler => {
            tracing::info!("清理通知任务退出");
            create_notify_handler.abort();
            web_server.abort();
        }
        _ = &mut web_server => {
            tracing::info!("WEB 服务退出");
            create_notify_handler.abort();
            clean_notify_handler.abort();
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("CTRL+C 退出");
            create_notify_handler.abort();
            clean_notify_handler.abort();
            web_server.abort();
        }
    }

    Ok(())
}

async fn create_notify(state: ArcAppState) {
    loop {
        let title = format!("通知-{}", rand::random::<u32>());
        let content = format!("通知的内容-{}", rand::random::<u32>());
        let kind = match rand::random_range(0..7) {
            0 => model::NotificationKind::Uncategorized,
            1 => model::NotificationKind::Instance,
            2 => model::NotificationKind::Snapshot,
            3 => model::NotificationKind::Transfer,
            4 => model::NotificationKind::Security,
            5 => model::NotificationKind::Ticket,
            6 => model::NotificationKind::System,
            _ => model::NotificationKind::Uncategorized,
        };
        let m = model::Notification::new(title, content, kind);
        let m = match m.create(&state.pool).await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("创建通知失败:{}", e);
                continue;
            }
        };
        tracing::info!("创建通知:{:?}", m);
        tokio::time::sleep(tokio::time::Duration::from_mins(5)).await;
    }
}

async fn clean_notify(state: ArcAppState) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_hours(24));

    loop {
        interval.tick().await;

        if let Err(e) = model::Notification::turncate(&state.pool).await {
            tracing::error!("清理通知失败:{}", e);
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            continue;
        }

        tracing::info!("通知清理完成");
    }
}
