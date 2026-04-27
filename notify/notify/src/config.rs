use dotenv::dotenv;

pub struct Config {
    pub database_url: String,
    pub database_max_conns: u32,
    pub sse_keep_alive_interval: u32,
    pub web_addr: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")?;
        let database_max_conns = std::env::var("DATABASE_MAX_CONNS")?.parse()?;
        let sse_keep_alive_interval = std::env::var("SSE_KEEP_ALIVE_INTERVAL")?.parse()?;
        let web_addr = std::env::var("WEB_ADDR")?;

        Ok(Self {
            database_url,
            database_max_conns,
            sse_keep_alive_interval,
            web_addr,
        })
    }
}
