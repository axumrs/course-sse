use std::sync::Arc;

use sqlx::PgPool;

use crate::config::Config;

pub struct AppState {
    pub cfg: Arc<Config>,
    pub pool: PgPool,
}

impl AppState {
    pub fn new_arc(cfg: Config, pool: PgPool) -> ArcAppState {
        Arc::new(AppState {
            cfg: Arc::new(cfg),
            pool,
        })
    }
}

pub type ArcAppState = Arc<AppState>;
