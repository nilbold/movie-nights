use std::net::TcpListener;

use sqlx::PgPool;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

use movie_nights::config::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let config = AppConfig::read()?;
    let db_pool = PgPool::connect(&config.database.connection_string()).await?;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port))?;

    tracing::info!(
        "movie nights api starting on {}",
        listener.local_addr().unwrap()
    );

    movie_nights::run(listener, db_pool)?.await?;

    Ok(())
}
