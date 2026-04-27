pub async fn init(dsn: &str, max_conns: u32) -> sqlx::Result<sqlx::PgPool> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_conns)
        .connect(dsn)
        .await
}
