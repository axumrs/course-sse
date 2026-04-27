use std::sync::Arc;

use crate::config::Config;

pub struct AppState {
    pub cfg: Arc<Config>,
}

impl AppState {
    pub fn new_arc(cfg: Config) -> ArcAppState {
        Arc::new(AppState { cfg: Arc::new(cfg) })
    }
}

pub type ArcAppState = Arc<AppState>;
