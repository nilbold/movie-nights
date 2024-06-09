use std::net::TcpListener;

use sqlx::PgPool;

use movie_nights::config::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::read()?;
    let db_pool = PgPool::connect(&config.database.connection_string()).await?;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port))?;

    movie_nights::run(listener, db_pool)?.await?;

    Ok(())
}
